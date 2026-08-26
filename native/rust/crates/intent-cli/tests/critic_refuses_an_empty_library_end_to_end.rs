//! The empty-library refusal at the surface the GATE reads -- the process exit
//! code, not a number returned inside the library.
//!
//! # Why this file exists separately from the unit test
//!
//! `Report::exit_code()` is NOT the process's exit code. `render.rs` translates
//! it through `Failure`, and before this change that match had arms for 3 and 1
//! and sent everything else to `Ok(())`. **So the refusal could have been
//! implemented correctly in `intentsvcs`, unit-tested green, and left the binary
//! still exiting 0** -- the fix reported as done, the gate still passing over an
//! empty denominator, and no test anywhere going red about it. That is the exact
//! shape this whole item is about, one layer in, so it gets its own end-to-end
//! arm rather than a comment.
//!
//! # The lever, and the control that proves it reached
//!
//! `install::resolve()` CANONICALISES the exe before walking its ancestors for
//! the `lib/templates` marker, so the binary is **copied** into the fixture
//! install and never symlinked: a symlink resolves back to the real target and
//! the fake root would be silently ignored. `exit_codes.rs:316` symlinks for a
//! different purpose (a PATH shim) and that is correct there; here it would
//! quietly defeat the test.
//!
//! Under `IN-AG-RED-CONTROL-001` a negative control has to be shown capable of
//! going red, and the failure mode this one is exposed to is passing for the
//! wrong reason -- **a fixture install so broken that the binary refuses for
//! some cause other than the empty library would still produce rc=2.** So the
//! control is the SAME copied binary, in the SAME fixture, with the same cwd,
//! differing in exactly one thing: whether `intent/plugins/claude/rules/shell`
//! is present. If the fake root were not being read at all, the control would
//! not change the answer, and it does.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// An install tree carrying the marker and nothing else -- the shipped keg's
/// shape, which is what every estate is running.
fn fixture_install(dir: &Path) -> PathBuf {
  std::fs::create_dir_all(dir.join("lib/templates")).expect("marker");
  std::fs::create_dir_all(dir.join("bin")).expect("bin");
  let exe = dir.join("bin/intent");
  // COPIED, NOT SYMLINKED -- see the module note.
  std::fs::copy(env!("CARGO_BIN_EXE_intent"), &exe).expect("copy the binary");
  exe
}

/// Copy a rule tree wholesale, returning the number of `RULE.md` files placed.
///
/// The count is the control's own control: it is what makes "nothing was copied"
/// an assertion failure rather than a silent green.
fn copy_tree(src: &Path, dest: &Path) -> usize {
  let mut rules = 0usize;
  std::fs::create_dir_all(dest).expect("dest dir");
  for entry in std::fs::read_dir(src).expect("source tree must exist") {
    let entry = entry.expect("dir entry");
    let from = entry.path();
    let to = dest.join(entry.file_name());
    if from.is_dir() {
      rules += copy_tree(&from, &to);
    } else {
      std::fs::copy(&from, &to).expect("copy file");
      if entry.file_name() == "RULE.md" {
        rules += 1;
      }
    }
  }
  rules
}

fn run(exe: &Path) -> (i32, String) {
  let out = Command::new(exe)
    .args(["critic", "shell", "--files", "README.md"])
    .current_dir(repo_root())
    .output()
    .expect("run the fixture binary");
  let mut text = String::from_utf8_lossy(&out.stderr).into_owned();
  text.push_str(&String::from_utf8_lossy(&out.stdout));
  (out.status.code().unwrap_or(-1), text)
}

/// THE DEFECT, at the surface the hook reads. Red before the fix: rc was 0.
#[test]
fn a_binary_whose_install_carries_no_rules_exits_2_and_says_why() {
  let dir = tempfile::tempdir().expect("tempdir");
  let exe = fixture_install(dir.path());

  let (rc, text) = run(&exe);

  assert_eq!(
    rc, 2,
    "an install with no rule library must exit 2 -- 0 seals a clean verdict over \
     an empty denominator and the gate passes.\noutput was:\n{text}"
  );
  assert!(
    text.contains("rule library is EMPTY"),
    "the refusal must NAME the empty library -- a bare rc=2 is a silent refusal \
     and the operator cannot act on it.\noutput was:\n{text}"
  );
}

/// THE CONTROL THAT PROVES THE LEVER REACHED. Same binary, same fixture, same
/// cwd; the only difference is a rules tree. Without this, a fixture broken in
/// any other way would also produce rc=2 and the test above would pass for the
/// wrong reason.
#[test]
fn the_same_fixture_with_a_rules_tree_does_not_refuse() {
  let dir = tempfile::tempdir().expect("tempdir");
  let exe = fixture_install(dir.path());

  // Only `shell` is needed: `Library::files` walks the known languages and skips
  // any whose directory is absent.
  //
  // **COPIED RECURSIVELY AT WHATEVER DEPTH THE RULES SIT.** The first version of
  // this walked one level and joined `RULE.md`, because that is the shape
  // `agnostic/red-control/RULE.md` shows; the real layout carries a CATEGORY
  // level (`shell/code/<name>/RULE.md`) and nothing was copied. The guard below
  // is what reported it -- the control would otherwise have run against an empty
  // tree and tested the empty case twice while reading as a pass.
  let dest = dir.path().join("intent/plugins/claude/rules/shell");
  let src = repo_root().join("intent/plugins/claude/rules/shell");
  let copied = copy_tree(&src, &dest);
  assert!(
    copied > 0,
    "no rules were copied, so this control cannot discriminate -- it would be \
     testing the empty case twice"
  );

  let (rc, text) = run(&exe);

  assert_ne!(
    rc, 2,
    "the same binary with {copied} rule(s) present must NOT report the \
     empty-library refusal -- if it still does, the fixture install root is not \
     being read and the test above proved nothing.\noutput was:\n{text}"
  );
}
