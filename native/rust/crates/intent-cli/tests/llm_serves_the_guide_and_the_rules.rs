//! **ST0067 AC-00.1 and AC-00.3: the two doors of `intent llm` that refused.**
//!
//! Bare `intent llm` exited 2 -- "this build cannot answer the question at all"
//! -- while `intent llm guide`, one word away, answered it. `llm usage_rules`
//! exited 2 for real: v2 printed the project's root `usage-rules.md` and v3
//! shipped a stub.
//!
//! # AC-00.1 is asserted as an IDENTITY, not as two exit codes
//!
//! Checking that both doors exit 0 would pass on a build where bare `llm`
//! printed something else entirely, which is the failure worth naming: two
//! doors onto one document drift when they are two code paths. The property is
//! that the bytes are the same. The implementation collapses them onto one
//! match arm, so this test is what stops a future author splitting them again
//! for a reason that looks good at the time.
//!
//! # AC-00.3's refusal arm is worthless without its control
//!
//! "absent file exits nonzero" is satisfied by a binary that refuses whatever
//! it is handed -- including one where the verb was never wired at all, which
//! is the exact state this AC exists to leave. **Measured while writing this:
//! the first version of the refusal drive passed for precisely that reason**,
//! because the fixture had no project in it and every invocation exited 1. So
//! the present-file arm runs on the same fixture and must exit 0 and print the
//! file, or the absent-file arm proves nothing.

use std::path::Path;
use std::process::Command;

fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (_, err, code) = run(dir.path(), &["init", "llmproj"]);
  assert_eq!(code, 0, "the fixture must be a real project: {err}");
  dir
}

/// **`usage-rules.md` is written directly rather than seeded through
/// `claude upgrade --apply`.** The verb under test reads a file at the project
/// root; how it arrived is another command's contract, and routing this fixture
/// through it would make these assertions fail whenever THAT verb broke.
fn with_rules(dir: &Path, body: &str) {
  std::fs::write(dir.join("usage-rules.md"), body).expect("write the rules file");
}

#[test]
fn bare_llm_and_llm_guide_serve_the_same_document() {
  let dir = project();
  let (bare, bare_err, bare_code) = run(dir.path(), &["llm"]);
  let (named, _, named_code) = run(dir.path(), &["llm", "guide"]);

  assert_eq!(bare_code, 0, "bare `llm` used to exit 2: {bare_err}");
  assert_eq!(named_code, 0, "`llm guide` answers under its own name");
  assert_eq!(
    bare, named,
    "bare `intent llm` and `intent llm guide` are one document -- if these ever \
     differ, the two doors have become two code paths"
  );
  // Without this the assertion above is satisfied by two empty strings.
  assert!(
    bare.len() > 1000 && bare.contains("#### intent "),
    "the document must actually be the guide, not an empty match: {} bytes",
    bare.len()
  );
}

#[test]
fn usage_rules_prints_the_projects_own_file_verbatim() {
  let dir = project();
  // Trailing newline included deliberately: the verb uses `print!`, so a body
  // that does not end in one must not gain one.
  let body = "# Rules\n\nDO drive it.\nNEVER assume it.";
  with_rules(dir.path(), body);

  let (out, err, code) = run(dir.path(), &["llm", "usage_rules"]);
  assert_eq!(code, 0, "a present rules file is not an error: {err}");
  assert_eq!(
    out, body,
    "the file is printed verbatim, with nothing added"
  );
  assert!(err.is_empty(), "nothing belongs on stderr here: {err}");
}

#[test]
fn usage_rules_refuses_plainly_when_the_file_is_absent() {
  let dir = project();

  // **THE CONTROL, AND IT RUNS FIRST.** If this arm does not pass, the refusal
  // below is not evidence of anything -- see the module note.
  with_rules(dir.path(), "# Rules\n");
  let (_, _, present) = run(dir.path(), &["llm", "usage_rules"]);
  assert_eq!(
    present, 0,
    "control: with the file present this verb must succeed, or the refusal \
     arm below passes for the wrong reason"
  );

  std::fs::remove_file(dir.path().join("usage-rules.md")).expect("remove the rules file");
  let (out, err, code) = run(dir.path(), &["llm", "usage_rules"]);

  assert_ne!(code, 0, "an absent rules file is a refusal");
  assert_eq!(
    code, 1,
    "exit 1 -- the build CAN answer this; this project has nothing to answer with. \
     Exit 2 would tell a v2 consumer the tool is unavailable and invite it to fail open"
  );
  assert!(
    out.is_empty(),
    "a refusal puts nothing on stdout, where a caller would read it as rules: {out:?}"
  );
  assert!(
    err.contains("usage-rules.md"),
    "the refusal names the file it could not find: {err:?}"
  );
}
