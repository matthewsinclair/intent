//! **A remint refuses an id it cannot read, by the text it was given, and
//! writes nothing** (ST0095 WP-04's critic answer W15, owed to WP-06). `st
//! remint` and `issues remint` take the id they move through the id waist's
//! one parse, so a spelling of no id family never reaches the mint: the
//! refusal names what was typed, and the canon is byte for byte as it was.

use crate::common::{canon_files, ok, run};

#[test]
fn st_remint_and_issues_remint_refuse_a_malformed_id_and_write_nothing() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "remints"]);
  ok(root, &["st", "new", "a thread the refusal must not touch"]);
  ok(
    root,
    &["issues", "add", "an issue the refusal must not touch"],
  );

  for (verb, malformed) in [
    (["st", "remint"], "ST-not-an-id"),
    (["issues", "remint"], "not-an-issue"),
  ] {
    let before = canon_files(root);
    let (out, err, code) = run(root, &[verb[0], verb[1], malformed]);
    assert_ne!(
      code,
      0,
      "`intent {} {malformed}` was not refused:\n{out}",
      verb.join(" ")
    );
    assert!(
      err.contains(malformed),
      "`intent {} {malformed}` names what was typed:\n{err}",
      verb.join(" ")
    );
    assert_eq!(
      canon_files(root),
      before,
      "`intent {} {malformed}` changed canon before refusing",
      verb.join(" ")
    );
  }
}
