//! **NO VERB TAKES A BYTE IT DID NOT NAME FIRST -- REMOVED OR WRITTEN OVER.**
//!
//! hv, 2026-09-12, on finding `organize --apply` listing its removals only
//! after making them: _"silent deletion ... This cannot be released publicly
//! with that kind of bug. THEY NEED IDENTIFYING, TRIAGING, AND FIXING, AS A
//! MATTER OF URGENCY."_ The individual fixes are each their own commit; this
//! file is the thing that keeps the CLASS fixed, so the next verb that learns
//! to remove something cannot quietly join the old shape.
//!
//! # What is asserted, and why it is not "the output mentions the path"
//!
//! A verb that prints `removed: x` AFTER removing `x` mentions the path and is
//! exactly the defect. So the property is about TENSE and ORDER: every path
//! that vanished from the tree must appear in a line that was printed BEFORE
//! the act -- a `to-remove:` or `to-prune:` line -- and those lines must come
//! before the past-tense ones. The run's own stdout carries both, in order, so
//! one capture answers it.
//!
//! # The vacuity control is the point of the file
//!
//! **A fixture where nothing is removed passes every assertion here while
//! proving nothing**, which is this estate's most-repeated failure: a subject
//! that cannot exhibit the defect cannot clear it. So each arm computes what
//! actually vanished by walking the tree before and after, and asserts that the
//! set is NON-EMPTY before asserting anything about it.
//!
//! # Coverage, stated so the gaps are visible rather than implied
//!
//! Covered here: `organize --apply`, `organize --apply --quiet`, `st dehydrate`,
//! and `st hydrate` writing over a view whose bytes differ -- **which is the
//! same class and not a second one**: hv's words are *removes or overwrites*,
//! and a hand edit replaced by a render is as gone as a file deleted.
//! The MCP `organize` tool is covered too, and its property is different in
//! shape and the same in kind: a machine caller has no moment of looking, so
//! `apply: true` must ECHO the digest of a plan a previous call returned.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Output;

fn intent(dir: &Path, args: &[&str]) -> Output {
  crate::common::intent()
    .args(args)
    .current_dir(dir)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent")
}

fn ok(dir: &Path, args: &[&str]) {
  let out = intent(dir, args);
  assert!(
    out.status.success(),
    "fixture step `{args:?}` failed: {}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Every file under the project, relative and `/`-separated -- the population a
/// removal is measured against.
fn tree(root: &Path) -> BTreeSet<String> {
  fn walk(root: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      // `.git` and the store are not this verb's subject and churn on their own.
      if path
        .file_name()
        .is_some_and(|n| n == ".git" || n == ".cache")
      {
        continue;
      }
      if path.is_dir() {
        out.insert(rel(root, &path));
        walk(root, &path, out);
      } else {
        out.insert(rel(root, &path));
      }
    }
  }
  let mut out = BTreeSet::new();
  walk(root, root, &mut out);
  out
}

fn rel(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .components()
    .map(|c| c.as_os_str().to_string_lossy().to_string())
    .collect::<Vec<_>>()
    .join("/")
}

/// A project holding a realised thread the declaration does not name, which is
/// the state every removing verb here reconciles.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "Fixture"]);
  ok(root, &["st", "new", "Gate declaration"]);
  ok(root, &["st", "new", "To be removed"]);
  ok(root, &["st", "hydrate", "ST0001"]);
  ok(root, &["st", "hydrate", "ST0002"]);
  // The estate's own ship gate: exactly one criterion may carry the block, and
  // it must be satisfied or every removal is held and the fixture is vacuous.
  ok(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.1",
      "--text",
      "No dehydration path removes any file while any declared precondition is \
       unmet. <<PRECONDITIONS AC-00.9 PRECONDITIONS>>",
    ],
  );
  ok(
    root,
    &["ac", "new", "ST0001", "AC-00.9", "--text", "a precondition"],
  );
  ok(
    root,
    &[
      "ac",
      "satisfy",
      "ST0001",
      "AC-00.9",
      "--evidence",
      "met by construction in this fixture",
    ],
  );
  ok(root, &["st", "start", "ST0001"]);
  dir
}

/// Take ST0002 out of the declaration while its files stay on disk.
fn undeclare_st0002(root: &Path) {
  let path = root.join("intent/.intentfiles");
  let text = std::fs::read_to_string(&path).expect("the declaration is readable");
  let kept: String = text
    .lines()
    .filter(|l| !l.contains("ST0002"))
    .map(|l| format!("{l}\n"))
    .collect();
  std::fs::write(&path, kept).expect("rewrite the declaration");
}

/// The paths a run took away.
fn vanished(before: &BTreeSet<String>, after: &BTreeSet<String>) -> Vec<String> {
  before.difference(after).cloned().collect()
}

/// The property, applied to one run: everything that went was named in the
/// future tense, and the future tense came first.
fn assert_named_before_it_went(said: &str, gone: &[String], what: &str) {
  assert!(
    !gone.is_empty(),
    "VACUITY CONTROL: `{what}` removed nothing, so this arm proves nothing \\
     about a verb that removes. Output was:\\n{said}"
  );
  for path in gone {
    let announced = said.lines().map(str::trim_start).any(|l| {
      (l.starts_with("to-remove: ") || l.starts_with("to-prune: ")) && l.contains(path.as_str())
    });
    assert!(
      announced,
      "`{what}` removed `{path}` and never named it before doing so. Output \\
       was:\\n{said}"
    );
  }
  let first_future = said
    .lines()
    .position(|l| {
      let t = l.trim_start();
      t.starts_with("to-remove: ") || t.starts_with("to-prune: ")
    })
    .expect("a future-tense line, asserted above");
  if let Some(first_past) = said.lines().position(|l| {
    let t = l.trim_start();
    t.starts_with("removed: ") || t.starts_with("pruned: ")
  }) {
    assert!(
      first_future < first_past,
      "`{what}` printed a past-tense removal line above its plan, so the plan \\
       is not what a reader meets first. Output was:\\n{said}"
    );
  }
}

/// `organize --apply` -- the trigger case.
#[test]
fn organize_apply_names_every_path_before_it_goes() {
  let dir = project();
  let root = dir.path();
  undeclare_st0002(root);

  let before = tree(root);
  let out = intent(root, &["organize", "--apply"]);
  let after = tree(root);

  assert_named_before_it_went(
    &stdout(&out),
    &vanished(&before, &after),
    "organize --apply",
  );
}

/// **AND `--quiet` DOES NOT BUY ITS QUIET WITH THE REMOVALS.** The flag may
/// withhold what a run wrote and the inventory it walked past; the bytes it
/// takes away are not reportage.
#[test]
fn organize_apply_quiet_names_every_path_before_it_goes() {
  let dir = project();
  let root = dir.path();
  undeclare_st0002(root);

  let before = tree(root);
  let out = intent(root, &["organize", "--apply", "--quiet"]);
  let after = tree(root);

  assert_named_before_it_went(
    &stdout(&out),
    &vanished(&before, &after),
    "organize --apply --quiet",
  );
}

/// `st dehydrate` -- the same property in the verb that removes one thread.
#[test]
fn st_dehydrate_names_every_path_before_it_goes() {
  let dir = project();
  let root = dir.path();

  let before = tree(root);
  let out = intent(root, &["st", "dehydrate", "ST0002"]);
  let after = tree(root);

  assert_named_before_it_went(&stdout(&out), &vanished(&before, &after), "st dehydrate");
}

/// **AN OVERWRITE IS A REMOVAL OF THE BYTES THAT WERE THERE.** `st hydrate`
/// put every realised view into its write set unconditionally, so a hand edit
/// -- or any work an unregistered writer had left there -- was replaced by the
/// render, reported afterwards as `wrote:`, at exit 0. Driven before the fix on
/// exactly this fixture.
///
/// **`dehydrate` HAS REFUSED THIS SIGNATURE SINCE IT WAS WRITTEN.** A file whose
/// bytes differ from the render may be a hand edit and nothing on disk says
/// which, so `organize::gate` will not REMOVE it. The two verbs now answer
/// alike.
#[test]
fn st_hydrate_refuses_a_view_it_would_write_over_and_names_it() {
  let dir = project();
  let root = dir.path();
  let view = root.join("intent/st/ST0002/info.md");
  let edited = format!(
    "{}\nA HAND EDIT THE STORE DOES NOT CARRY\n",
    std::fs::read_to_string(&view).expect("the view is realised")
  );
  std::fs::write(&view, &edited).expect("edit the view");

  let refused = intent(root, &["st", "hydrate", "ST0002"]);
  assert!(
    !refused.status.success(),
    "a realisation that would destroy a difference must refuse: {}",
    stdout(&refused)
  );
  let said = format!(
    "{}{}",
    stdout(&refused),
    String::from_utf8_lossy(&refused.stderr)
  );
  assert!(
    said.contains("intent/st/ST0002/info.md"),
    "and it must NAME the view, or the operator diffs a whole thread against a \
     description of it: {said}"
  );
  assert_eq!(
    std::fs::read_to_string(&view).expect("still there"),
    edited,
    "nothing may be written by a refused realisation"
  );

  // **`--overwrite` IS THE ROUTE, AND IT STILL NAMES WHAT IT DISCARDS FIRST.**
  // The flag is the operator accepting the loss; it is not permission to stop
  // reporting which file takes it.
  let forced = intent(root, &["st", "hydrate", "ST0002", "--overwrite"]);
  let told = stdout(&forced);
  assert!(forced.status.success(), "{told}");
  let named = told
    .lines()
    .position(|l| l.trim_start().starts_with("to-overwrite: ") && l.contains("info.md"))
    .expect("the discarded view is named");
  let wrote = told
    .lines()
    .position(|l| l.trim_start().starts_with("wrote: ") && l.contains("info.md"))
    .expect("and the write is reported");
  assert!(
    named < wrote,
    "the naming comes BEFORE the write, or it is a report about something \
     already gone: {told}"
  );
  assert_ne!(
    std::fs::read_to_string(&view).expect("rewritten"),
    edited,
    "and --overwrite really did discard it"
  );
}

/// **A REALISATION VERB REMOVES NOTHING, AND THE ONLY THING THAT STOPPED IT
/// WAS A GATE THAT IS TEMPORARY BY DESIGN** (vc's sweep, item 5).
///
/// `hydrate` builds the estate's plan and narrows it to one artefact's
/// directory, so the narrowed plan can carry `Dehydrate` steps -- and
/// `Plan::run` performs them. `edit` and `st edit` reach the same body, so a
/// verb that prints one path could take a file on its way past.
///
/// **DRIVEN ON THE PARENT: the removal did not happen, and that is the point.**
/// The estate-wide ship gate holds every removal until the last precondition
/// goes green, so the loss is LATENT rather than realised -- the run reported
/// `hydrated ... 0 written by this run` and said nothing about the file at all.
/// A defect whose only guard is a gate designed to open is one that arrives on
/// the day nobody is looking.
///
/// **REFUSED RATHER THAN ANNOUNCED.** The verb that reconciles an estate is
/// `organize`, which now names every removal first and asks on a terminal. A
/// realisation verb performing a removal nobody asked for is the wrong ACT, and
/// announcing it would make it look intended.
#[test]
fn a_realisation_verb_refuses_to_remove_and_names_what_it_would_have_taken() {
  let dir = project();
  let root = dir.path();
  // A view-shaped file under a realised thread that the store does not carry.
  let stray = root.join("intent/st/ST0002/WP/99/info.md");
  std::fs::create_dir_all(stray.parent().expect("parent")).expect("mkdir");
  std::fs::write(&stray, "not anything the store renders\n").expect("plant");

  for args in [
    vec!["st", "hydrate", "ST0002"],
    vec!["edit", "st", "ST0002", "info", "--path"],
  ] {
    let out = intent(root, &args);
    let said = format!("{}{}", stdout(&out), String::from_utf8_lossy(&out.stderr));
    assert!(
      !out.status.success(),
      "`intent {}` must refuse a plan that would remove: {said}",
      args.join(" ")
    );
    assert!(
      said.contains("WP/99/info.md"),
      "and it must NAME the file it would have taken: {said}"
    );
    assert!(stray.is_file(), "and take nothing");
  }
}

/// **THE MCP TOOL REMOVES NOTHING A CALLER WAS NOT SHOWN** (vc's sweep, item 3).
///
/// The terminal face previews, renders the plan, asks a human and pins the act
/// to what it printed. **This surface had none of that**: one call with
/// `apply: true` removed files, and the first and only account of which files
/// was the response that came back after. A machine caller has no moment of
/// looking, so the moment is made into a protocol -- call once to see the plan,
/// then echo its `plan` back.
///
/// **AND THE ECHO IS CHECKED.** A digest that no longer matches means the estate
/// moved between the two calls, so the removals about to run are not the ones
/// that were returned.
#[test]
fn the_mcp_organize_tool_refuses_to_apply_a_plan_the_caller_was_not_shown() {
  let dir = project();
  let root = dir.path();
  undeclare_st0002(root);

  // One tool call, answered. **The payload is a JSON document inside the
  // frame's text**, which is the MCP shape: the frame carries a content block
  // and the block carries the tool's own answer, so a test that grepped the
  // frame would be reading a string of escapes.
  let call = |args: &str| -> (serde_json::Value, String) {
    let frames = [
      r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#.to_string(),
      r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#.to_string(),
      format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"intent_organize","arguments":{args}}}}}"#
      ),
    ];
    let mut child = crate::common::intent()
      .arg("mcp")
      .current_dir(root)
      .env("HOME", testkit::fixture_home())
      .stdin(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .stderr(std::process::Stdio::piped())
      .spawn()
      .expect("spawn intent mcp");
    {
      use std::io::Write as _;
      let stdin = child.stdin.as_mut().expect("stdin");
      for frame in &frames {
        writeln!(stdin, "{frame}").expect("write frame");
      }
    }
    drop(child.stdin.take());
    let out = child.wait_with_output().expect("wait");
    let frame: serde_json::Value = String::from_utf8_lossy(&out.stdout)
      .lines()
      .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
      .find(|v| v.get("id").and_then(|i| i.as_u64()) == Some(2))
      .expect("an answer to the tool call");
    let text = frame
      .pointer("/result/content/0/text")
      .and_then(|t| t.as_str())
      .unwrap_or_default()
      .to_string();
    let payload =
      serde_json::from_str::<serde_json::Value>(&text).unwrap_or(serde_json::Value::Null);
    (payload, frame.to_string())
  };

  // **THE REFUSAL, AND NOTHING IS REMOVED BY IT.**
  let before = tree(root);
  let (_, refused) = call(r#"{"apply": true}"#);
  assert!(
    refused.contains("plan"),
    "`apply: true` with no plan must be refused and must say what to pass: {refused}"
  );
  assert_eq!(
    tree(root),
    before,
    "and it must remove nothing at all: {refused}"
  );

  // **THE PREVIEW HANDS BACK WHAT AN APPLY MUST ECHO**, or the refusal above is
  // a wall with no door.
  let (preview, raw) = call(r#"{"apply": false}"#);
  let digest = preview
    .get("plan")
    .and_then(|p| p.as_str())
    .unwrap_or_default()
    .to_string();
  assert!(
    digest.len() > 16,
    "a preview must return the plan a later apply echoes: {raw}"
  );
  assert_eq!(tree(root), before, "and a preview removes nothing");
  assert_eq!(
    preview.get("applied").and_then(|a| a.as_bool()),
    Some(false)
  );

  // A digest that is not this tree's plan is refused, so the echo is CHECKED.
  let (_, stale) = call(
    r#"{"apply": true, "plan": "0000000000000000000000000000000000000000000000000000000000000000"}"#,
  );
  assert!(
    stale.contains("not the ones that were printed") || stale.contains("error"),
    "a plan digest that does not match this tree must refuse: {stale}"
  );
  assert_eq!(tree(root), before, "and remove nothing");

  // The echo of the plan it was shown performs the removal it named.
  let (applied, raw_applied) = call(&format!(r#"{{"apply": true, "plan": "{digest}"}}"#));
  assert_eq!(
    applied.get("applied").and_then(|a| a.as_bool()),
    Some(true),
    "the echoed plan must be accepted: {raw_applied}"
  );
  assert!(
    !vanished(&before, &tree(root)).is_empty(),
    "and it must really have reconciled: {raw_applied}"
  );
}

/// **THE CONTROL ON THE INSTRUMENT ITSELF.** Everything above rests on `tree`
/// being able to see a removal at all; an arm that walked the wrong root, or
/// skipped the directory the fixture uses, would report an empty `vanished` set
/// and the vacuity guard would fire -- but only if the guard is really reached.
/// This drives the measurement directly, with no verb involved.
#[test]
fn the_measurement_can_see_a_removal() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let file = root.join("intent/st/ST0001/info.md");
  std::fs::create_dir_all(file.parent().expect("parent")).expect("mkdir");
  std::fs::write(&file, "x\n").expect("write");

  let before = tree(root);
  std::fs::remove_file(&file).expect("remove");
  let after = tree(root);

  assert_eq!(
    vanished(&before, &after),
    vec!["intent/st/ST0001/info.md".to_string()],
    "the walk must see a file that went, or every arm above is vacuous"
  );
}

/// A run that removes nothing is not required to announce anything, and must
/// not be made to. Without this the fix could be "print a plan every time",
/// which is noise on the default spelling of a routine verb.
#[test]
fn a_run_that_removes_nothing_announces_no_removal() {
  let dir = project();
  let root = dir.path();

  let before = tree(root);
  let said = stdout(&intent(root, &["organize", "--apply"]));
  let after = tree(root);

  assert!(
    vanished(&before, &after).is_empty(),
    "precondition: with ST0002 still declared this run removes nothing"
  );
  assert!(
    !said
      .lines()
      .any(|l| l.trim_start().starts_with("removed: ")),
    "and it reports no removal. Output was:\\n{said}"
  );
}

fn _unused(_: PathBuf) {}
