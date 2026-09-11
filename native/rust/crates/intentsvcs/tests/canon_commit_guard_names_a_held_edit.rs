//! **0210: the canon commit guard says when the file it refuses on is somebody
//! else's uncommitted edit.**
//!
//! With intentd ingesting an attachment edit from disk, the canon extract a
//! node regenerates names a peer's uncommitted bytes, and
//! `canon_commit_check.sh --staged` refuses that node's commit on a file it is
//! not editing. The refusal named the file and prescribed `st attach` -- the
//! route for YOUR file, which here would commit the peer's work in progress.
//! Reaching "wait for the holder" took three inferences the guard did not
//! supply.
//!
//! The fixture is the issue's shape in a scratch repository: an attachment
//! committed as `old`, the working tree holding `new` uncommitted, and canon
//! naming `new` staged on its own.

use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Command;
use testkit::repo_root;

/// A child process that cannot be pointed at the real repository by a git
/// environment inherited from a hook.
fn command(program: &str, cwd: &Path) -> Command {
  let mut c = Command::new(program);
  c.current_dir(cwd)
    .env_remove("GIT_DIR")
    .env_remove("GIT_INDEX_FILE")
    .env_remove("GIT_WORK_TREE");
  c
}

fn git(cwd: &Path, args: &[&str]) {
  let status = command("git", cwd)
    .args(["-c", "user.name=cc", "-c", "user.email=cc@example.invalid"])
    .args(args)
    .status()
    .expect("run git");
  assert!(status.success(), "git {args:?} failed");
}

fn canon(bytes: &[u8]) -> String {
  let sha: String = Sha256::digest(bytes)
    .iter()
    .map(|b| format!("{b:02x}"))
    .collect();
  format!("{{\"id\":\"ST0001\",\"attachments\":[{{\"path\":\"f.sh\",\"sha256\":\"{sha}\"}}]}}\n")
}

#[test]
fn the_refusal_names_an_attachment_held_uncommitted_outside_this_commit() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let file = root.join("intent/st/ST0001/f.sh");
  let extract = root.join("intent/.canon/st/ST0001.json");
  std::fs::create_dir_all(file.parent().expect("parent")).expect("mkdir");
  std::fs::create_dir_all(extract.parent().expect("parent")).expect("mkdir");

  std::fs::write(&file, b"old\n").expect("write");
  std::fs::write(&extract, canon(b"old\n")).expect("write");
  git(root, &["init", "-q"]);
  git(root, &["add", "-A"]);
  git(root, &["commit", "-qm", "base"]);

  // A peer's edit, uncommitted, and canon regenerated from a store that took it.
  std::fs::write(&file, b"new\n").expect("write");
  std::fs::write(&extract, canon(b"new\n")).expect("write");
  git(root, &["add", "intent/.canon/st/ST0001.json"]);

  let out = command("bash", root)
    .arg(repo_root().join("intent/st/ST0056/parity/tools/canon_commit_check.sh"))
    .arg("--staged")
    .env("ROOT", root)
    .output()
    .expect("run the guard");
  let said = String::from_utf8_lossy(&out.stderr);

  assert_eq!(
    out.status.code(),
    Some(1),
    "canon naming bytes the commit does not carry is refused: {said}"
  );
  assert!(
    said.contains("NOT IN THIS COMMIT, AND MODIFIED IN THE WORKING TREE")
      && said.contains("      ST0001/f.sh"),
    "the refusal does not say the file is an uncommitted edit this commit does not carry: {said}"
  );
}
