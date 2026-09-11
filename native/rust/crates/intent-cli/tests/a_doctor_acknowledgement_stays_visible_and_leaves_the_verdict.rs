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
//! A fresh project has exactly the finding this needs: no backup has ever been
//! taken, so `doctor` reports `backup-stale` and exits non-zero.

use std::path::Path;
use std::process::Command;

fn intent(root: &Path, args: &[&str]) -> (bool, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
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
  // A fresh project's generated views are not on disk yet; render them so the
  // backup is the ONLY counted finding and the exit code below has one cause.
  let (ok, out) = intent(root, &["sync", "--to-disk"]);
  assert!(ok, "sync --to-disk failed: {out}");

  // The fixture must exhibit the finding, or everything below passes vacuously.
  let (ok, before) = intent(root, &["doctor"]);
  assert!(
    !ok && before.contains("backup-stale") && before.contains("doctor: 1 finding(s)"),
    "a fresh project should report backup-stale as its one finding and fail:\n{before}"
  );

  let reason = "this probe keeps no backups, by decision";
  acknowledge(root, serde_json::json!({ "backup-stale": reason }));

  let (ok, text) = intent(root, &["doctor"]);
  assert!(
    ok,
    "an acknowledged class must leave the exit code:\n{text}"
  );
  assert!(
    text.contains(&format!(
      "acknowledged: backup-stale -- {reason} (1 finding(s))"
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
  assert_eq!(acked["class"], "backup-stale", "{doc:#}");
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
    serde_json::json!({ "backup-stale": reason, "backup-stael": "a typo" }),
  );
  let (ok, typo) = intent(root, &["doctor"]);
  assert!(
    !ok,
    "an acknowledgement naming no class must not pass:\n{typo}"
  );
  assert!(
    typo.contains("unhonourable-setting") && typo.contains("backup-stael"),
    "the misspelt acknowledgement must be reported by name:\n{typo}"
  );
}
