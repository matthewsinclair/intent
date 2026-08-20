//! AT-09.3 / AC-09.3 -- **the history is queryable from the one place it lives.**
//!
//! The tracked `intent/events.jsonl` is gone and the store is the log's only
//! home, so this verb is the other half of that ruling rather than polish:
//! without it, dropping the projection would have traded an unread file for an
//! unreadable table. **No `events` verb existed in either binary** before this,
//! which is why the file was the log's only reader-facing surface at all.

use std::path::Path;
use std::process::Command;

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"EV\",\n  \"author\": \"vc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

fn intent(root: &Path, args: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    out.status.code(),
    Some(0),
    "`intent {}` failed\nstdout: {}\nstderr: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).to_string()
}

/// **The verb answers rows the store holds, and the DENOMINATOR is rows.**
#[test]
fn the_history_is_readable_and_the_count_is_of_rows() {
  let fx = project();
  intent(fx.path(), &["st", "new", "A thread"]);

  let out = intent(fx.path(), &["events"]);
  assert!(
    out.contains("st.new"),
    "the act that just happened is in the history: {out}"
  );
  assert!(
    out.contains("event(s)."),
    "and the run states how many rows there are: {out}"
  );
}

/// **A FILTER NARROWS; IT DOES NOT TRUNCATE.**
///
/// Driven as a subset relation plus a present member, which is the pair that
/// separates the two: a filter that returned nothing satisfies "is a subset"
/// on its own, and a filter that returned everything satisfies "contains the
/// match" on its own. **Neither arm alone is worth writing.**
#[test]
fn a_filter_narrows_rather_than_truncating() {
  let fx = project();
  intent(fx.path(), &["st", "new", "One"]);
  intent(fx.path(), &["st", "new", "Two"]);

  let all = intent(fx.path(), &["events"]);
  let filtered = intent(fx.path(), &["events", "--op", "st.new"]);

  assert!(
    filtered.contains("st.new"),
    "the filter keeps what matches: {filtered}"
  );
  let filtered_ids: Vec<&str> = filtered
    .lines()
    .filter_map(|l| l.split_whitespace().nth(1))
    .filter(|t| t.starts_with("01"))
    .collect();
  assert!(
    !filtered_ids.is_empty(),
    "the filtered run returned rows at all, or the subset below is vacuous: {filtered}"
  );
  for id in &filtered_ids {
    assert!(
      all.contains(id),
      "a filtered run returned a row the unfiltered run did not: {id}"
    );
  }
}

/// **AN EMPTY ANSWER MUST SAY WHICH EMPTINESS IT IS.**
///
/// A filter that matched nothing and a store that holds nothing are different
/// facts, and a verb that prints nothing at exit 0 for both is the failure the
/// prose critics have -- silence read as a clean bill of health.
#[test]
fn an_empty_answer_names_which_emptiness_it_is() {
  let fx = project();
  intent(fx.path(), &["st", "new", "A thread"]);

  let no_match = intent(fx.path(), &["events", "--op", "no.such.op"]);
  assert!(
    no_match.contains("no event matches this filter"),
    "a filter that matched nothing says so, and says what it searched: {no_match}"
  );
  assert!(
    !no_match.trim().is_empty(),
    "and it is never silence at exit 0"
  );
}
