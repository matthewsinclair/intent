//! **`intent issues` -- the READ half, driven through the real binary.**
//!
//! The three read verbs ship. `add`, `close` and `open` do NOT, and they are
//! blocked on a ratification rather than on effort: `transitions.rs` declares
//! `Issue.status` as `Disposition::Unbuilt`, `data-model.md` ratifies three
//! machines and no issue machine, and AC-04.6 requires the implemented graph to
//! match the ratified ones EXACTLY with no undeclared edge.
//!
//! **The edges look obvious, which is exactly when the discipline earns its
//! keep**: the point of a ratified machine is that whoever implements it does
//! not get to add to it. So the last case here asserts that the three
//! mutations still report themselves unbuilt -- **a guard against building them
//! by reflex** as much as a record of where the family stands.

use std::path::Path;
use std::process::{Command, Output};

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent/.config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"I\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");

  let issues = dir.path().join("intent/issues");
  std::fs::create_dir_all(&issues).expect("mkdir issues");
  // **An OPEN one with a severity and a CLOSED one WITHOUT**, because the two
  // differences are what every case below discriminates on, and a fixture whose
  // rows differ in only one way cannot show a filter working.
  std::fs::write(
    issues.join("0021.json"),
    "{\"schema\":\"intent/issue@3.0\",\"number\":21,\"slug\":\"a-thing\",\"title\":\"A thing went wrong\",\"status\":\"open\",\"severity\":\"high\",\"created\":\"2026-08-01\"}\n",
  )
  .expect("write issue");
  std::fs::write(
    issues.join("0007.json"),
    "{\"schema\":\"intent/issue@3.0\",\"number\":7,\"slug\":\"older\",\"title\":\"An older thing\",\"status\":\"closed\",\"created\":\"2026-07-01\",\"closed\":\"2026-07-09\"}\n",
  )
  .expect("write issue");
  dir
}

fn run(root: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).to_string()
}

/// **The default bucket is OPEN, and the bare command is `list`.**
///
/// Both halves are v2's, and the second is the table's declared default verb
/// for this family rather than a convenience added here.
#[test]
fn the_bare_command_lists_the_open_bucket() {
  let dir = project();
  let bare = run(dir.path(), &["issues"]);
  assert_eq!(bare.status.code(), Some(0), "{}", stdout(&bare));

  let out = stdout(&bare);
  assert!(out.contains("0021"), "the open issue is listed:\n{out}");
  assert!(
    !out.contains("0007"),
    "and the CLOSED one is not -- a default that showed everything would make `--kind` decorative:\n{out}"
  );
  assert_eq!(
    out,
    stdout(&run(dir.path(), &["issues", "list"])),
    "`intent issues` and `intent issues list` are the same request, which is what the table's declared default verb means"
  );
}

/// **The bucket filter discriminates in both directions**, which one case
/// cannot show: a filter that returns everything passes an `all` assertion, and
/// one that returns nothing passes a `closed` assertion about an open issue.
#[test]
fn the_bucket_filter_selects_and_excludes() {
  let dir = project();

  let all = stdout(&run(dir.path(), &["issues", "list", "--kind", "all"]));
  assert!(
    all.contains("0007") && all.contains("0021"),
    "`all` carries both buckets:\n{all}"
  );

  let closed = stdout(&run(dir.path(), &["issues", "list", "--kind", "closed"]));
  assert!(
    closed.contains("0007") && !closed.contains("0021"),
    "`closed` carries the closed one and NOT the open one:\n{closed}"
  );
}

/// **A severity nobody recorded prints `?`, not a blank.**
///
/// v2's token, and it is kept for the reason v2 had it: a blank cell in a
/// padded table reads as a rendering fault, where `?` reads as "nobody said".
/// The same distinction `or_unknown` holds for `intent info`.
#[test]
fn an_unrecorded_severity_is_marked_rather_than_left_blank() {
  let dir = project();
  let all = stdout(&run(dir.path(), &["issues", "list", "--kind", "all"]));
  let row = all
    .lines()
    .find(|l| l.contains("0007"))
    .unwrap_or_else(|| panic!("the closed issue is listed:\n{all}"));
  assert!(
    row.contains('?'),
    "the row for an issue with no severity marks it:\n{row}"
  );
}

/// **`21`, `0021` and `0021.json` are one issue.**
///
/// v2 normalises the same way, and an operator who copied a padded id out of a
/// filename must not be told it does not exist.
#[test]
fn an_issue_id_is_read_the_way_an_operator_spells_it() {
  let dir = project();
  let canonical = stdout(&run(dir.path(), &["issues", "show", "0021"]));
  assert!(canonical.contains("A thing went wrong"), "{canonical}");

  for spelling in ["21", "0021", "0021.json"] {
    assert_eq!(
      stdout(&run(dir.path(), &["issues", "show", spelling])),
      canonical,
      "`{spelling}` names the same issue as `0021`"
    );
  }
}

/// An issue that is not there is a refusal, and a spelling that is not an id is
/// a DIFFERENT refusal -- because the actions differ.
#[test]
fn an_absent_issue_and_an_unreadable_id_are_told_apart() {
  let dir = project();

  let absent = run(dir.path(), &["issues", "show", "999"]);
  let absent_err = String::from_utf8_lossy(&absent.stderr).to_string();
  assert_eq!(absent.status.code(), Some(1), "{absent_err}");
  assert!(
    absent_err.contains("no issue 0999"),
    "the refusal names the issue as the operator will see it listed -- padded:\n{absent_err}"
  );
  assert!(
    absent_err.contains("--kind all"),
    "and the remedy names the flag WITHOUT which a closed issue cannot appear. A bare `issues list` would send them looking in a bucket that cannot hold \
     it:\n{absent_err}"
  );

  let unreadable = run(dir.path(), &["issues", "show", "notanumber"]);
  let unreadable_err = String::from_utf8_lossy(&unreadable.stderr).to_string();
  assert_eq!(unreadable.status.code(), Some(1), "{unreadable_err}");
  assert!(
    !unreadable_err.contains("no issue"),
    "an unreadable id must not be reported as a MISSING issue -- one is the operator's spelling and the other is the estate's contents, and telling someone to \
     go and look is the wrong action for the first:\n{unreadable_err}"
  );
}

/// `--json` emits the issue, not a rendering of it.
#[test]
fn the_json_form_is_the_issue_itself() {
  let dir = project();
  let out = stdout(&run(dir.path(), &["issues", "show", "21", "--json"]));
  let parsed: serde_json::Value =
    serde_json::from_str(&out).unwrap_or_else(|e| panic!("`--json` emits JSON ({e}):\n{out}"));
  assert_eq!(parsed["number"], 21);
  assert_eq!(parsed["status"], "open");
  assert_eq!(parsed["title"], "A thing went wrong");
}

/// **THE MUTATIONS ARE STILL UNBUILT, AND THIS GUARDS AGAINST BUILDING THEM BY
/// REFLEX.**
///
/// They are blocked on a ratification, not on effort. `Issue.status` is
/// declared `Disposition::Unbuilt`; `data-model.md` ratifies a thread, a work
/// package and a criterion machine, and no issue machine. AC-04.6 forbids an
/// undeclared edge, so wiring `close` and `open` means declaring `open <->
/// closed` on nobody's authority.
///
/// **When the machine is ratified, this test is what should be deleted first**
/// -- deliberately, by whoever wires them, rather than found failing afterwards.
#[test]
fn the_mutating_verbs_report_themselves_unbuilt_until_a_machine_is_ratified() {
  let dir = project();
  for args in [
    vec!["issues", "add", "A new thing"],
    vec!["issues", "close", "21"],
    vec!["issues", "open", "7"],
  ] {
    let out = run(dir.path(), &args);
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
      err.contains("is a known command that is not implemented yet"),
      "`intent {}` is wired, and no issue machine is ratified in data-model.md -- so its edges were declared by whoever wrote it rather than by hv. If the \
       machine HAS since been ratified, delete this test in the change that wires them:\n{err}",
      args.join(" ")
    );
    assert_eq!(
      out.status.code(),
      Some(2),
      "and an unbuilt command answers in the deliberate code, not the one that means a real refusal:\n{err}"
    );
  }
}
