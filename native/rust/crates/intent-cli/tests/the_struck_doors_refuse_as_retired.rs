//! **THREE DOORS THAT ANSWERED "NOT IMPLEMENTED" ARE RETIRED** (hv's decision
//! 3, ST0058 AC-00.3).
//!
//! `st bootstrap`, `agents template` and `claude prime` were declared, listed
//! in `--help`, named by canon for agents to run -- and each answered `is a
//! known command that is not implemented yet`. hv struck all three. They are
//! RETIRED rather than deleted, so each still refuses at rc=2 with `was
//! retired` (issue 0044) instead of clap's generic rc=1, and names no
//! replacement because nothing replaces them.
//!
//! The same strike reaches two things that pointed at `st bootstrap`: `init
//! --with-st0000`, which was refused pending it, and `st_zero`, whose
//! refusal named it as the successor.

use std::path::Path;
use std::process::Command;

fn intent(root: &Path, args: &[&str]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(root)
    .stdin(testkit::lifeline_for(args))
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent {args:?}`: {e}"));
  (
    out.status.code().unwrap_or(-1),
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr),
  )
}

#[test]
fn the_struck_doors_refuse_as_retired_and_nothing_points_at_them() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();

  for (family, verb) in [
    ("st", "bootstrap"),
    ("agents", "template"),
    ("claude", "prime"),
  ] {
    let (code, out) = intent(root, &[family, verb]);
    assert_eq!(
      code, 2,
      "`intent {family} {verb}` must refuse at rc=2:\n{out}"
    );
    assert!(
      out.contains(&format!("`intent {family} {verb}` was retired")),
      "`intent {family} {verb}` must answer as retired:\n{out}"
    );
    assert!(
      out.contains("there is no v3 replacement"),
      "nothing replaces `{family} {verb}`, and the refusal must say so:\n{out}"
    );
    assert!(
      !out.contains("not implemented yet"),
      "`{family} {verb}` still answers as unimplemented:\n{out}"
    );

    let (_, help) = intent(root, &[family, "--help"]);
    assert!(
      !help
        .lines()
        .any(|l| l.split_whitespace().next() == Some(verb)),
      "`intent {family} --help` still lists `{verb}`:\n{help}"
    );
  }

  // The second entrance to the ST0000 bootstrap is refused and writes nothing.
  let (code, out) = intent(root, &["init", "--with-st0000"]);
  assert_ne!(code, 0, "`init --with-st0000` must be refused:\n{out}");
  assert!(
    !root.join("intent").exists(),
    "a refused `init --with-st0000` must create nothing:\n{out}"
  );

  // `st_zero`'s refusal named `st bootstrap` as its successor; a refusal must
  // never send anyone to a struck door.
  let (code, out) = intent(root, &["st_zero"]);
  assert_eq!(code, 2, "`intent st_zero` must refuse as retired:\n{out}");
  assert!(
    !out.contains("st bootstrap"),
    "`st_zero`'s refusal still names the struck `st bootstrap`:\n{out}"
  );
}
