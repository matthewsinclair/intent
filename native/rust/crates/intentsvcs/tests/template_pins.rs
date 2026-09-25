//! **THE EMBEDDED v2 TEMPLATES ARE THE ARTEFACT THE DROP RULE CLAIMS IDENTITY
//! AGAINST, AND A PIN THAT SILENTLY DISAGREES WITH ITS SOURCE IS A DROP RULE
//! THAT SILENTLY MATCHES NOTHING.**
//!
//! This exists because it happened. I transcribed the work-package template by
//! copying the file and the steel-thread template FROM MEMORY, and the second
//! one was wrong in seven places -- different placeholder set in the
//! frontmatter, different wording in all four section bodies. **Nothing would
//! have failed.** The migration would have matched zero sections, carried all
//! 35 pieces of scaffolding as authored prose, and reported `0 dropped` --
//! every count reconciling, the estate looking conserved, and the fabricated
//! authorship invisible.
//!
//! **That is the day's own class landing inside the fix for it**: the output of
//! a broken pin and the output of an estate with no scaffolding are identical.
//! The detection question -- what OTHER state would produce this same output?
//! -- answers "a project whose threads were never made from this template",
//! which is indistinguishable by inspection.
//!
//! **So the check is not care during transcription, it is that a pin and its
//! source are the same bytes.** Care is what produced the WP template
//! correctly and the ST template wrongly, in the same hour, by the same person.
//!
//! **A failure here is NOT automatically "re-pin".** The pin is deliberately
//! one version (vc's condition 3): the drop set must not become a function of
//! which Intent is installed, or the same estate migrated twice loses different
//! sections with nothing recording why. v2 is frozen, so these files should
//! never move -- and if one does, the decision is whether the drop set changes,
//! not whether the constant is stale.
//!
//! **THE FILES THEMSELVES ARE GONE FROM `lib/templates/`, AND THE PIN IS NOW TO
//! THE BLOB.** Nothing rendered them: `init` skipped both, and this file was
//! their only reader, so they sat in the compiled-in template tree for a test.
//! The artefact the drop rule cites was never the working copy anyway -- it is
//! the path AT `0b1b3b5b`, which is what `legacy.rs` names -- so each constant
//! is held to the git blob id that path had at that revision. Anyone can check
//! the id with `git rev-parse 0b1b3b5b:<path>`, and no copy has to be kept.

use std::io::Write;
use std::process::{Command, Stdio};
use testkit::repo_root;

/// The git blob id of `text`, from git itself, so the id means what
/// `git rev-parse <rev>:<path>` means.
fn blob_id(text: &str) -> String {
  let mut git = Command::new("git")
    .args(["hash-object", "--stdin"])
    .current_dir(repo_root())
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("git runs");
  git
    .stdin
    .take()
    .expect("git's stdin")
    .write_all(text.as_bytes())
    .expect("the text reaches git");
  let out = git.wait_with_output().expect("git hash-object finishes");
  assert!(out.status.success(), "git hash-object failed: {out:?}");
  String::from_utf8(out.stdout)
    .expect("a blob id is text")
    .trim()
    .to_string()
}

fn pin_matches(source: &str, blob: &str, embedded: &str) {
  assert_eq!(
    blob_id(embedded),
    blob,
    "the embedded pin is not `{source}` at 0b1b3b5b (blob {blob}). The drop rule \
     claims sections are byte-identical to THAT artefact, so a pin that disagrees \
     with it matches nothing and silently carries every piece of scaffolding as \
     authored prose -- with `0 dropped` reported and every count reconciling. \
     If the template genuinely changed, the decision is whether the drop set \
     changes with it, not whether to refresh the constant."
  );
}

#[test]
fn the_steel_thread_template_pin_is_its_source() {
  pin_matches(
    "lib/templates/prj/st/ST####/info.md",
    "c421c4242539d49947d84b3d701d1f5fe831d899",
    intentsvcs::legacy::ST_TEMPLATE_V2,
  );
}

#[test]
fn the_work_package_template_pin_is_its_source() {
  pin_matches(
    "lib/templates/prj/st/WP/info.md",
    "e14911d76b2b248a3e1e97f2b7f075be9db2774d",
    intentsvcs::legacy::WP_TEMPLATE_V2,
  );
}

/// The control: the helper must be able to FAIL. A text that is not the
/// template has to be reported as differing, or the two tests above pass on
/// any input and this whole file is decoration.
#[test]
fn the_pin_check_can_actually_fail() {
  let outcome = std::panic::catch_unwind(|| {
    pin_matches(
      "lib/templates/prj/st/WP/info.md",
      "e14911d76b2b248a3e1e97f2b7f075be9db2774d",
      "definitely not the template",
    )
  });
  assert!(
    outcome.is_err(),
    "a pin that does not match its source must fail the check"
  );
}
