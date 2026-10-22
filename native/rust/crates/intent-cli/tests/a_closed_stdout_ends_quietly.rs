//! Issue 0613: a verb whose standard output is closed early ends quietly, at
//! the status a filter has when its reader goes away, instead of panicking.
//!
//! Rust ignores SIGPIPE, so a write to a pipe with no reader returns an error,
//! and `println!` panics on it: `intent at list | grep -q ...` ended at 101
//! with a `failed printing to stdout: Broken pipe` panic whenever Intent wrote
//! after `grep` had gone. The reader here is dropped BEFORE the child starts,
//! so the first write meets a closed pipe every time rather than by race.

use std::path::Path;
use std::process::Stdio;

/// The status a shell reports for a process its reader's going away ended:
/// 128 plus SIGPIPE's 13.
const READER_GONE: i32 = 141;

/// Run `args` from `cwd` with stdout a pipe whose reader is already gone.
fn into_a_closed_pipe(args: &[&str], cwd: &Path) -> (Option<i32>, String) {
  let (reader, writer) = std::io::pipe().expect("a pipe");
  drop(reader);
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .stdout(writer)
    .stderr(Stdio::piped())
    .output()
    .expect("run the binary");
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
  )
}

/// Two verbs, one that needs a project and one that does not, so the fix is
/// shown to live where every verb passes rather than in either of them.
#[test]
fn a_verb_whose_reader_has_gone_ends_quietly_at_the_filters_status() {
  let dir = tempfile::tempdir().expect("tempdir");
  let init = crate::common::intent()
    .args(["init", "piped"])
    .current_dir(dir.path())
    .stdin(Stdio::null())
    .output()
    .expect("init");
  assert!(
    init.status.success(),
    "the fixture's init: {}",
    String::from_utf8_lossy(&init.stderr)
  );

  for args in [
    &["llm", "guide"][..],
    &["st", "list", "--status", "all"][..],
  ] {
    let (code, stderr) = into_a_closed_pipe(args, dir.path());
    let verb = args.join(" ");
    assert_eq!(
      code,
      Some(READER_GONE),
      "`intent {verb}` into a closed pipe did not end at the filter's status: {stderr}"
    );
    assert!(
      !stderr.contains("panicked") && !stderr.contains("Broken pipe"),
      "`intent {verb}` into a closed pipe said so on stderr: {stderr}"
    );
  }
}
