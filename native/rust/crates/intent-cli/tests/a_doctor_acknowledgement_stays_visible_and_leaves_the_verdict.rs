//! **AN ACKNOWLEDGED DOCTOR CLASS STILL RUNS AND STILL PRINTS, AND LEAVES THE
//! COUNT AND THE EXIT CODE** (issue `0065`, hv decision 14).
//!
//! A project that has decided to keep a warned-about state had no way to say
//! so, so `doctor` warned about it on every run and trained its operator to
//! stop reading it. The acknowledgement lives in project config under
//! `doctor.acknowledged`, keyed by the class name `doctor` prints, with a
//! reason. The close condition is the half that is easy to get wrong: the
//! check must not DISAPPEAR, because a check that vanishes is worse than one
//! that shouts -- and an acknowledgement naming no real class must not silence
//! anything either, so it is reported rather than ignored.
//!
//! **THE VEHICLE IS `view-skew`, AND IT USED TO BE `backup-stale`** (vc,
//! 2026-09-12, issue `0308`). The subject here is the ACKNOWLEDGEMENT
//! mechanism, not any one class, and it needs a class that is COUNTED and
//! BLOCKING or every assertion below passes vacuously. `backup-stale` became
//! advisory when `doctor` joined the pre-commit gate -- a protection being
//! behind is not a reason to refuse someone's work -- so it stopped being able
//! to carry this test. A hand-edited generated view is the cheapest blocking
//! finding a fresh project can be given, and it is one the operator really does
//! meet.

use std::path::Path;

fn intent(root: &Path, args: &[&str]) -> (bool, String) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(root)
    .stdin(testkit::lifeline_for(args))
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent {args:?}`: {e}"));
  (
    out.status.success(),
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr),
  )
}

/// Put `acknowledged` into the project's config as its `doctor` block.
fn acknowledge(root: &Path, acknowledged: serde_json::Value) {
  let path = root.join("intent/.config/config.json");
  let mut config: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&path).expect("config")).expect("config json");
  config["doctor"] = serde_json::json!({ "acknowledged": acknowledged });
  std::fs::write(
    &path,
    serde_json::to_string_pretty(&config).expect("encode"),
  )
  .expect("write");
}

#[test]
fn an_acknowledged_class_prints_its_reason_and_leaves_the_count_and_the_exit_code() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (ok, out) = intent(root, &["init", "probe"]);
  assert!(ok, "init failed: {out}");
  // A fresh project's generated views are not on disk yet; render them, then
  // skew exactly one so the exit code below has one cause.
  let (ok, out) = intent(root, &["sync", "--to-disk"]);
  assert!(ok, "sync --to-disk failed: {out}");
  std::fs::write(root.join("intent/todo.md"), "edited by hand\n").expect("skew one view");

  // The fixture must exhibit the finding, or everything below passes vacuously.
  let (ok, before) = intent(root, &["doctor"]);
  assert!(
    !ok && before.contains("view-skew") && before.contains("doctor: 1 finding(s)"),
    "a hand-edited view should be the project's one counted finding and should fail:\n{before}"
  );

  let reason = "this probe edits that view by hand, by decision";
  acknowledge(root, serde_json::json!({ "view-skew": reason }));

  let (ok, text) = intent(root, &["doctor"]);
  assert!(
    ok,
    "an acknowledged class must leave the exit code:\n{text}"
  );
  assert!(
    text.contains(&format!(
      "acknowledged: view-skew -- {reason} (1 finding(s))"
    )),
    "the acknowledged class must still print, with its reason and count:\n{text}"
  );
  assert!(
    text.contains("doctor: 0 finding(s)"),
    "an acknowledged class must leave the count:\n{text}"
  );

  let (ok, json) = intent(root, &["doctor", "--format", "json"]);
  assert!(ok, "json run failed:\n{json}");
  let doc: serde_json::Value = serde_json::from_str(&json).expect("doctor json");
  assert_eq!(doc["healthy"], true, "{doc:#}");
  let acked = &doc["acknowledged"][0];
  assert_eq!(acked["class"], "view-skew", "{doc:#}");
  assert_eq!(acked["acknowledged"], true, "{doc:#}");
  assert_eq!(acked["reason"], reason, "{doc:#}");
  assert_eq!(
    acked["findings"].as_array().map(Vec::len),
    Some(1),
    "{doc:#}"
  );

  // A misspelt class acknowledges nothing, and says so rather than silencing.
  acknowledge(
    root,
    serde_json::json!({ "view-skew": reason, "view-skwe": "a typo" }),
  );
  let (ok, typo) = intent(root, &["doctor"]);
  assert!(
    !ok,
    "an acknowledgement naming no class must not pass:\n{typo}"
  );
  assert!(
    typo.contains("unhonourable-setting") && typo.contains("view-skwe"),
    "the misspelt acknowledgement must be reported by name:\n{typo}"
  );
}
