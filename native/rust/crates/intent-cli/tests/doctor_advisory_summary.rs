//! The summary line the operator reads: advisories are set aside BY NAME on it,
//! and `N finding(s)` is the count of what is actionable -- hv, 2026-08-26, on
//! Baize reporting "66 finding(s)" at exit 1 for 66 rows its own text called
//! "ADVISORY, not a refusal". This is the only arm that reads the rendered
//! line, so it is the only arm a mutation of the renderer can red.
use std::path::Path;
use std::process::Command;

fn intent(root: &Path, args: &[&str]) -> (bool, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent {args:?}`: {e}"));
  (
    out.status.success(),
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr),
  )
}

#[test]
fn the_summary_counts_advisories_apart_and_exits_zero_on_them_alone() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (ok, out) = intent(root, &["init", "probe"]);
  assert!(ok, "init: {out}");
  let (ok, out) = intent(root, &["st", "new", "a live thread"]);
  assert!(ok, "st new: {out}");

  // Plant one v2-grammar citation on the live thread, the shape the migration
  // carries whole: no `file`, a `legacy.raw` with `::` in it.
  let canon = root.join("intent/.canon/st/ST0001.json");
  let mut thread: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&canon).expect("read canon"))
      .expect("canon is json");
  thread["criteria"] = serde_json::json!([
    {"id": "AC-01.1", "text": "the thing works", "kind": "test", "state": {"is": "computed"}}
  ]);
  thread["tests"] = serde_json::json!([
    {"id": "AT-01.1", "kind": "test", "covers": ["AC-01.1"], "status": "green",
     "legacy": {"raw": "apps/x/test/y_test.exs::a name with spaces"}}
  ]);
  std::fs::write(
    &canon,
    serde_json::to_string_pretty(&thread).expect("render"),
  )
  .expect("write canon");

  // A canon edited by hand leaves the store and the views behind it, and a
  // fresh store has no snapshot: bring all three into step so the ONLY thing
  // left for doctor to say is the advisory. Each of these is its own finding
  // otherwise, and the arm would be asserting over a project that really
  // does have findings.
  for step in [
    &["sync", "--to-store", "ST0001"][..],
    &["sync", "--to-disk", "ST0001"][..],
    &["backup"][..],
  ] {
    let (ok, out) = intent(root, step);
    assert!(ok, "{step:?}: {out}");
  }

  let (ok, out) = intent(root, &["doctor"]);
  assert!(
    !out.contains("residue: "),
    "nothing is printed as residue: {out}"
  );
  assert!(
    out.contains("doctor: 0 finding(s) across"),
    "the summary counts what is actionable: {out}"
  );
  assert!(
    out.contains("1 advisory(ies), not counted"),
    "the advisories are named on the summary line: {out}"
  );
  assert!(ok, "advisories alone exit 0: {out}");

  // **THE BODY IS NOT IN THE DEFAULT OUTPUT.** Baize carries 66 of these, and
  // printing each with its remedy buried the report under four-line blocks
  // that ask for nothing. hv, reading exactly that: "How is this an
  // improvement?"
  assert!(
    !out.contains("carries a legacy reference"),
    "an advisory body must not be printed by default: {out}"
  );
  assert!(
    out.contains("advisory: 1 note(s) not shown and not counted"),
    "the default run says how many there are and how to read them: {out}"
  );

  // `--verbose` is where they live, in full, with the remedy.
  let (ok, verbose) = intent(root, &["doctor", "--verbose"]);
  assert!(ok, "still exit 0 under --verbose: {verbose}");
  assert!(
    verbose.contains(
      "advisory: intent/.canon/st/ST0001.json -- advisory -- AT-01.1 carries a legacy reference"
    ),
    "--verbose prints the row under `advisory:`: {verbose}"
  );
  assert!(
    verbose.contains("remedy: nothing is owed now"),
    "--verbose keeps the remedy: {verbose}"
  );
  assert!(
    !verbose.contains("not shown and not counted"),
    "the pointer is pointless once the bodies are shown: {verbose}"
  );

  // `--quiet` drops them entirely and keeps the verdict.
  let (ok, quiet) = intent(root, &["doctor", "--quiet"]);
  assert!(ok, "still exit 0 under --quiet: {quiet}");
  assert!(
    quiet.contains("doctor: 0 finding(s) across"),
    "the verdict survives --quiet: {quiet}"
  );
  // The COUNT stays on the verdict line, which is the one thing `--quiet`
  // keeps: "0 finding(s)" with 66 notes silently dropped would be a quieter
  // report and a less honest one. What goes is every line that is not the
  // verdict -- the bodies and the pointer.
  assert!(
    !quiet.contains("carries a legacy reference") && !quiet.contains("not shown and not counted"),
    "--quiet drops the bodies and the pointer: {quiet}"
  );
  assert!(
    quiet.contains("1 advisory(ies), not counted"),
    "the count is part of the verdict and survives --quiet: {quiet}"
  );
}
