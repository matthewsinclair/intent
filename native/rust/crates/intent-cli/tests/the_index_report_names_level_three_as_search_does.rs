//! AT-01.1 / AC-01.1 to AC-01.3 (ST0090 WP-01): **the index report names a
//! language's level 3 the way every search answer does, from one function in
//! intentsvcs.** A run that stored reads `stale` once a file it resolved has
//! changed, and `current` only with nothing stale; a run that failed or found
//! no tool keeps its own word whatever is stale. `intent index status` and
//! `intent index rebuild` say it on the language's first line and in their
//! JSON, with the last run's own word kept as `outcome`, and a search answer
//! names the same state for the same run. `intent index resolve` prints the
//! same lines, and `render.rs`'s unit arm pins its first line, because driving
//! it here would run a real toolchain.
//!
//! **RED ON THE COMMIT BEFORE THIS ONE** (issue 0664): the report printed the
//! run's own word, so a run that stored and then went stale read `current`
//! above its own list of stale files while every search answer named it
//! `stale`, and the JSON said `"state": "current"` beside a non-empty `stale`.

use crate::common::level_three::{TWO, both, envelope, estate, resolve, run};
use intentsvcs::index::resolved::{CURRENT, FAILED, MISSING, STALE, Trace, Unresolved};
use serde_json::json;
use std::path::Path;

/// Rust's first line in what `intent index <verb>` prints.
fn first_line(root: &Path, verb: &str) -> String {
  let (out, err, code) = run(&["index", verb], root);
  assert_eq!(code, 0, "index {verb} failed: {err}");
  out
    .lines()
    .find(|line| line.starts_with("resolution: rust"))
    .unwrap_or_else(|| panic!("index {verb} names no rust line: {out}"))
    .to_string()
}

/// Rust's level 3 in what `intent index <verb> --json` prints.
fn reported(root: &Path, verb: &str) -> serde_json::Value {
  let (out, err, code) = run(&["index", verb, "--json"], root);
  assert_eq!(code, 0, "index {verb} --json failed: {err}");
  let report: serde_json::Value = serde_json::from_str(&out).expect("the report is JSON");
  report["resolution"]["rust"].clone()
}

/// What a search answer names for rust's level 3: `null` when it names
/// nothing, which is how an answer says a language is current.
fn answered(root: &Path) -> serde_json::Value {
  envelope(root, &["--context", "helper"])["index"]["resolution"]["rust"].clone()
}

/// `src/two.rs` changes after the run that resolved it, and the index takes
/// the change in.
fn edit_two(root: &Path) {
  std::fs::write(
    root.join("src/two.rs"),
    format!("{TWO}pub fn later() {{}}\n"),
  )
  .expect("edit");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "{err}");
}

#[test]
fn a_stored_run_with_a_file_changed_since_reads_stale_on_every_face() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));
  edit_two(root);

  for verb in ["status", "rebuild"] {
    assert_eq!(
      first_line(root, verb),
      format!("resolution: rust  {STALE}  fixture-analyzer"),
      "index {verb} names the state a search answer names, not the run's own word"
    );
    let rust = reported(root, verb);
    assert_eq!(
      (&rust["state"], &rust["outcome"], &rust["stale"]),
      (&json!(STALE), &json!(CURRENT), &json!(["src/two.rs"])),
      "index {verb} --json names the same state, with the last run's own word beside it"
    );
  }
  assert_eq!(
    answered(root)["state"],
    STALE,
    "a search answer names the same state for the same run"
  );
}

#[test]
fn a_stored_run_with_nothing_stale_reads_current_and_an_answer_names_nothing() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));

  for verb in ["status", "rebuild"] {
    assert_eq!(
      first_line(root, verb),
      format!("resolution: rust  {CURRENT}  fixture-analyzer")
    );
    let rust = reported(root, verb);
    assert_eq!(
      (&rust["state"], &rust["outcome"], &rust["stale"]),
      (&json!(CURRENT), &json!(CURRENT), &json!([])),
      "index {verb} --json: {rust}"
    );
  }
  assert!(
    answered(root).is_null(),
    "an answer names no language that is current"
  );
}

#[test]
fn a_run_that_did_not_store_keeps_its_own_word_whatever_is_stale() {
  let dir = estate();
  let root = dir.path();
  resolve(root, Ok(both()));
  edit_two(root);

  let runs: Vec<(Result<Trace, Unresolved>, &str)> = vec![
    (
      Err(Unresolved::Failed {
        path: Some("src/lib.rs".to_string()),
        line: Some(3),
        detail: "the build script panicked".to_string(),
      }),
      FAILED,
    ),
    (
      Err(Unresolved::Missing {
        detail: "fixture-analyzer is not on PATH".to_string(),
      }),
      MISSING,
    ),
  ];
  for (outcome, state) in runs {
    resolve(root, outcome);
    assert_eq!(
      first_line(root, "status"),
      format!("resolution: rust  {state}  fixture-analyzer")
    );
    let rust = reported(root, "status");
    assert_eq!(
      (&rust["state"], &rust["outcome"], &rust["stale"]),
      (&json!(state), &json!(state), &json!(["src/two.rs"])),
      "a run that did not store is named by its own word, with the stale paths beside it"
    );
    assert_eq!(
      answered(root)["state"],
      state,
      "and so does a search answer"
    );
  }
}
