//! AT-07.1, covering ST0056 AC-07.1 -- **a fresh `intent init` works offline
//! from the binary alone (embedded canon); the 0022 broken-install class is
//! unconstructible.**
//!
//! **RUNNING `CARGO_BIN_EXE_intent` IN PLACE WOULD PROVE NOTHING, AND THAT IS
//! THE WHOLE DESIGN OF THIS FILE.** That binary lives in `target/`, which is
//! inside the repository, which contains `lib/templates` a few directories up.
//! A test driven there passes whether the templates are embedded or read off
//! the disk beside it -- it cannot tell the two apart, which is the only
//! question the criterion asks. **So the binary is COPIED OUT** to a temporary
//! directory with no Intent install anywhere above it, and run there.
//!
//! **AND THE FIXTURE IS ASSERTED BEFORE IT IS USED.** `parity.md:338` records a
//! fixture built to have "no git" that sat inside a repository, so a
//! measurement of *what happens outside git* measured something else and
//! reported it into the contract as fact. The same trap is one directory over
//! here: a fixture built to have "no install" that turns out to have one makes
//! every assertion below vacuous, in the passing direction. Checked, not
//! assumed.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The binary, copied somewhere it has no install tree, with the fixture's
/// isolation asserted rather than assumed.
fn isolated_binary() -> (tempfile::TempDir, PathBuf) {
  let dir = tempfile::tempdir().expect("tempdir");
  let dest = dir.path().join("intent");
  std::fs::copy(env!("CARGO_BIN_EXE_intent"), &dest).expect("copy the binary out of the repo");

  // THE CONTROL. Walk every ancestor of the copied binary and require that
  // none of them holds an Intent install. Without this the test passes on a
  // machine where the tempdir happens to sit under one, and reports the
  // embedding as proven when nothing was proven.
  for ancestor in dest.ancestors() {
    for marker in ["lib/templates", "intent/.config/config.json"] {
      let candidate = ancestor.join(marker);
      assert!(
        !candidate.exists(),
        "the isolation fixture is not isolated: {} exists, so `init` could read templates from disk and this test could not tell",
        candidate.display()
      );
    }
  }

  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(&dest).expect("stat").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&dest, perms).expect("chmod the copied binary");
  }
  (dir, dest)
}

/// Run the ISOLATED binary with `INTENT_HOME` removed from the environment.
///
/// Clearing it is not belt-and-braces: this suite runs inside a shell where it
/// is set to the development checkout, so leaving it would hand the binary a
/// path to a full install and re-create exactly the condition the copy exists
/// to remove.
fn run_isolated(bin: &Path, args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(bin)
    .args(args)
    .current_dir(cwd)
    .env_remove("INTENT_HOME")
    .output()
    .expect("run the isolated binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

#[test]
fn init_works_from_the_binary_alone() {
  let (_bin_dir, bin) = isolated_binary();
  let proj = tempfile::tempdir().expect("tempdir");

  let (out, err, code) = run_isolated(&bin, &["init", "offline"], proj.path());
  assert_eq!(
    code, 0,
    "init failed with no install tree, so the templates are not embedded: {err}"
  );

  assert!(
    proj.path().join("intent/.config/config.json").is_file(),
    "no project was created: {out}"
  );
}

/// **THE TEMPLATES ARRIVED, AND THEY CAME OUT OF THE BINARY.** The test above
/// would pass on an `init` that wrote a config and no content at all -- which
/// is a working project, and is not what AC-07.1 claims. This is the half that
/// distinguishes embedded canon from no canon.
#[test]
fn the_templates_come_out_of_the_binary() {
  let (_bin_dir, bin) = isolated_binary();
  let proj = tempfile::tempdir().expect("tempdir");

  let (_, err, code) = run_isolated(&bin, &["init", "offline"], proj.path());
  assert_eq!(code, 0, "init failed: {err}");

  for template in ["CLAUDE.md", "intent/wip.md", "intent/llm/MODULES.md"] {
    let path = proj.path().join(template);
    assert!(
      path.is_file(),
      "`{template}` was not written, so the embed did not reach the operator"
    );
    let body = std::fs::read_to_string(&path).expect("read the written template");
    assert!(
      !body.is_empty(),
      "`{template}` was written empty, which an absent embed would also produce"
    );
    // **THE SUBSTITUTION RAN.** An unsubstituted `[[PROJECT_NAME]]` reaching a
    // user's file is the visible end of a template that was copied but not
    // filled -- and it would pass every existence check above.
    assert!(
      !body.contains("[[PROJECT_NAME]]") && !body.contains("[[DATE]]"),
      "`{template}` still carries an unsubstituted placeholder"
    );
  }
}

/// **THE PROJECT IT MAKES OFFLINE IS A REAL ONE.** AC-07.1 says a fresh init
/// WORKS, not that it writes files; a directory the next command refuses is not
/// a working project however complete it looks.
#[test]
fn the_offline_project_accepts_the_first_command() {
  let (_bin_dir, bin) = isolated_binary();
  let proj = tempfile::tempdir().expect("tempdir");

  let (_, err, code) = run_isolated(&bin, &["init", "offline"], proj.path());
  assert_eq!(code, 0, "init failed: {err}");

  let (out, err, code) = run_isolated(&bin, &["st", "new", "offline thread"], proj.path());
  assert_eq!(code, 0, "the offline project refused `st new`: {err}");
  assert_eq!(out.trim(), "created: ST0001");
}

/// **THE 0022 CLASS IS UNCONSTRUCTIBLE, WHICH IS A STRONGER CLAIM THAN
/// "HANDLED".** v2's only recourse for a missing template is
/// `bin/intent_init:225`, `error "Template not found"`. The criterion requires
/// that there be no arrangement of the filesystem that produces it -- so this
/// asserts the absence of the failure MODE rather than the presence of a
/// remedy, on the most hostile input available: no install, no `INTENT_HOME`,
/// and an empty directory.
#[test]
fn no_arrangement_of_the_disk_produces_a_missing_template_error() {
  let (_bin_dir, bin) = isolated_binary();
  let proj = tempfile::tempdir().expect("tempdir");

  let (out, err, code) = run_isolated(&bin, &["init", "offline"], proj.path());
  assert_eq!(code, 0, "init failed: {err}");
  let both = format!("{out}{err}");
  for phrase in ["Template not found", "template not found", "INTENT_HOME"] {
    assert!(
      !both.contains(phrase),
      "the output mentions `{phrase}`, so this build still reaches for an install tree: {both}"
    );
  }
}
