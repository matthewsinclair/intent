//! **`intent issues` -- all six verbs, driven through the real binary.**
//!
//! The three mutations were `Disposition::Unbuilt` for two days, blocked on a
//! ratification rather than on effort, and this file's last case used to assert
//! that they reported themselves so. hv ratified **Machine 4** on 2026-08-17
//! (`Open | Closed`, entry `Open`, no guards) and they are wired.
//!
//! **The fixture below authors both issues directly as JSON, and that is now
//! load-bearing rather than incidental.** vc's mechanism-2 class is an assertion
//! whose reach depends on which states a fixture VISITS -- so a ratification that
//! gives a fixture a new route to a state can defang a test silently, keeping its
//! name and its green. Every read case here reaches `Closed` by authoring `0007`
//! and it still does: the fixture is unchanged by Machine 4, so the reach of
//! those assertions is unchanged. Stated because it was CHECKED, not assumed --
//! and if a later change starts closing `0007` with the verb instead, that
//! sentence is what stops being true.

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

  // Issue canon moved with thread canon: `intent/.canon/issues/`, flat and
  // zero-padded. The whole directory moved -- it held nothing but canon.
  let issues = dir.path().join("intent/.canon/issues");
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

/// What the dispatch table says `issues add --severity` falls back to.
///
/// Read from the table rather than restated, so the assertion that uses it tests
/// the SURFACE against its register instead of testing a literal against itself.
fn declared_severity_default() -> Option<String> {
  intent_cli::dispatch::table()
    .families
    .iter()
    .find(|f| f.name == "issues")
    .and_then(|f| f.entries.iter().find(|e| e.path == "issues add"))
    .and_then(|e| {
      e.flags
        .iter()
        .find(|fl| fl.spellings.iter().any(|s| s == "--severity"))
    })
    .and_then(|fl| fl.default.clone())
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

// ---------------------------------------------------------------------------
// The MUTATING half -- MACHINE 4
//
// **`the_mutating_verbs_report_themselves_unbuilt_until_a_machine_is_ratified`
// STOOD HERE and was deleted deliberately, in the change that wired them.** It
// said the three mutations report themselves unbuilt, and after the ratification
// that is WRONG rather than stale -- so leaving it to be found failing would
// have made the deletion and the wiring separately defensible, which is how a
// guard comes to be removed for the wrong reason. Its own doc comment named
// itself as the thing to delete first; this is that.
//
// The cases below are its replacement, and they are what it was guarding the
// place for.
// ---------------------------------------------------------------------------

/// **The self-loop is v2's `already CLOSED`, and it is told apart from the
/// refusal v2 also has.**
///
/// `move_issue` (`bin/intent_issues:274-291`) looks in the source bucket, and
/// finding nothing looks in the TARGET before erroring -- so an already-closed
/// issue is accepted at 0 and an absent one is refused. **Both arms in one test
/// on purpose**: hv's ruling made the first legal, and a test that only proves
/// acceptance would pass just as well on a `close` that accepted everything,
/// including an issue that does not exist.
#[test]
fn closing_a_closed_issue_is_accepted_and_an_absent_one_is_still_refused() {
  let dir = project();

  let loop_out = run(dir.path(), &["issues", "close", "7"]);
  assert_eq!(
    stdout(&loop_out).trim(),
    "ok: issue 0007 already CLOSED",
    "a self-loop reports the state the issue is in, in v2's own words"
  );
  assert_eq!(loop_out.status.code(), Some(0), "and it is not a failure");

  let absent = run(dir.path(), &["issues", "close", "99"]);
  assert_ne!(
    absent.status.code(),
    Some(0),
    "an issue that does not exist is a refusal, not a no-op -- v2 distinguishes \
     them and so must this:\n{}",
    String::from_utf8_lossy(&absent.stderr)
  );
}

/// **`close` then `open` returns the issue to where it started, and the DATES
/// move with it.**
///
/// The round trip is the test rather than one leg of it, because Machine 4's
/// whole content is that `Closed` has an exit -- that is the AC-04.6 condition
/// the `Unbuilt` row was held by. And `closed` is checked in both directions:
/// **set on the way in and CLEARED on the way out**, because a reopened issue
/// carrying the date it was closed on is the reported-success-with-a-stale-field
/// shape, and it would render into the committed extract.
#[test]
fn an_issue_closes_and_reopens_and_the_close_date_follows() {
  let dir = project();

  assert_eq!(
    stdout(&run(dir.path(), &["issues", "close", "21"])).trim(),
    "ok: issue 0021 -> CLOSED"
  );
  let closed: serde_json::Value = serde_json::from_str(&stdout(&run(
    dir.path(),
    &["issues", "show", "21", "--json"],
  )))
  .expect("json");
  assert_eq!(closed["status"], "closed");
  let stamped = closed["closed"]
    .as_str()
    .unwrap_or_else(|| panic!("closing records a date, and the store is what set it: {closed}"));
  assert!(
    stamped.len() == 10 && stamped.starts_with("20"),
    "the date came from the database as `YYYY-MM-DD` (D42), not from a clock in the facade: `{stamped}`"
  );
  assert_eq!(
    closed["created"], "2026-08-01",
    "and the RAISE date is untouched -- the create door must not re-stamp a \
     domain date that was already authored"
  );

  assert_eq!(
    stdout(&run(dir.path(), &["issues", "open", "21"])).trim(),
    "ok: issue 0021 -> OPEN"
  );
  let reopened: serde_json::Value = serde_json::from_str(&stdout(&run(
    dir.path(),
    &["issues", "show", "21", "--json"],
  )))
  .expect("json");
  assert_eq!(reopened["status"], "open");
  assert_eq!(
    reopened["closed"],
    serde_json::Value::Null,
    "reopening CLEARS the close date -- it described a state that has ended"
  );
}

/// **`add` prints v2's two lines, and the issue it made is readable afterwards.**
///
/// v2 prints the file it wrote and then `<id>:<title>` (`bin/intent_issues:187`).
/// The path is v3's own because v2's `issues/OPEN/<NNNN>/` layout retires under
/// the ratified deviation -- the SHAPE is parity, the path inside it cannot be.
///
/// **The number is `highest + 1`, checked against a fixture whose issues are not
/// contiguous** (0007 and 0021, nothing between). A count-plus-one implementation
/// would answer `0003` here and collide on the next raise; against a contiguous
/// fixture the two are indistinguishable.
#[test]
fn adding_an_issue_numbers_it_past_the_highest_and_reads_back() {
  let dir = project();
  let out = run(dir.path(), &["issues", "add", "A new thing"]);
  let printed = stdout(&out);
  let lines: Vec<&str> = printed.lines().map(str::trim).collect();
  assert_eq!(
    lines.len(),
    2,
    "v2 prints the path it wrote and then `<id>:<title>`:\n{:?}",
    lines
  );
  // **EXACT, and it was `starts_with(..) && ends_with(..)` until issue 0060.**
  // That pair is satisfied by an absolute path just as well as by a relative one,
  // so the assertion could not tell them apart -- and it was written that way
  // BECAUSE the path was absolute and carried a tmpdir the test could not name.
  // **A test that accommodates a defect in order to pass is blinded by the
  // accommodation**, which is the same family as pinning a defect as expected
  // output and harder to see, because nothing here is wrong on its face.
  //
  // The line is now repo-relative, as v2's is, so it can be asserted whole: v2
  // prints a path built from `$INTENT_DIR`, v3 prints one relativised against the
  // project root, and neither embeds the machine. The FLAT layout is the ratified
  // half (`intent/.canon/issues/<NNNN>.json`); the absoluteness was the defect.
  assert_eq!(
    lines[0], "created: intent/.canon/issues/0022.json",
    "the first line names the file this write produced, repo-relative -- an absolute path here \
     leaks $HOME into a line whose whole purpose is to be copied, and cannot be asserted by any \
     literal template (issue 0060)"
  );
  assert_eq!(lines[1], "0022:A new thing");

  let made: serde_json::Value = serde_json::from_str(&stdout(&run(
    dir.path(),
    &["issues", "show", "22", "--json"],
  )))
  .expect("json");
  assert_eq!(made["status"], "open", "a raised issue enters at Open");
  // **THE DEFAULT COMES FROM THE TABLE AND IS READ FROM THE TABLE HERE**, which
  // is the point rather than a convenience. `--severity` declares
  // `"default": "medium"`, the spine builds it into the surface, so an absent
  // flag arrives at the facade as `Some("medium")` -- nothing in the renderer or
  // the facade invents it. Hard-coding `"medium"` in this assertion would let the
  // table and the binary drift apart and still pass, which is the register-versus-
  // code split the whole parity apparatus exists to find.
  assert_eq!(
    made["severity"].as_str(),
    declared_severity_default().as_deref(),
    "an absent `--severity` lands on the value the dispatch table declares, \
     supplied by the surface rather than by the renderer or the facade"
  );
  let created = made["created"]
    .as_str()
    .unwrap_or_else(|| panic!("the store stamps the raise date (D42): {made}"));
  assert!(
    created.len() == 10 && created.starts_with("20"),
    "and it is a date rather than the empty string the facade handed in: `{created}`"
  );
}

/// **The severity the flag DOES carry reaches the issue.**
///
/// Separate from the case above because the two prove opposite things: that an
/// absent severity is recorded as absent, and that a given one is not dropped.
/// One test doing both would pass while `--severity` was ignored entirely.
#[test]
fn a_given_severity_is_recorded() {
  let dir = project();
  run(
    dir.path(),
    &["issues", "add", "Something bad", "--severity", "critical"],
  );
  let made: serde_json::Value = serde_json::from_str(&stdout(&run(
    dir.path(),
    &["issues", "show", "22", "--json"],
  )))
  .expect("json");
  assert_eq!(made["severity"], "critical");
}
