//! Issue 0599: **`intent --version` moved a legacy `~/.intent/` into the XDG
//! layout and removed it.**
//!
//! `main` ran `userstate::migrate_legacy` before any argument was parsed, so a
//! build step that read the binary's version migrated the operator's home. The
//! migration is wanted; its trigger was not. A read of the binary (`--version`,
//! `--help`, `info`) now touches nothing, and a verb that does work still
//! migrates first and prints the note.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A fixture `HOME` carrying a 3.0.1-layout `~/.intent/` with two entries
/// Intent owns.
fn legacy_home(name: &str) -> PathBuf {
  let home = std::env::temp_dir().join(format!("intent-legacy-home-{name}"));
  let _ = std::fs::remove_dir_all(&home);
  for (rel, body) in [
    (
      ".intent/skills/installed-skills.v3.json",
      "{\"skills\":{}}\n",
    ),
    (".intent/agents/installed-agents.json", "{\"agents\":{}}\n"),
  ] {
    let path = home.join(rel);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("legacy dir");
    std::fs::write(&path, body).expect("legacy file");
  }
  home
}

/// Every file under `dir`, by path relative to it, with its bytes.
fn tree(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
  fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
    for entry in std::fs::read_dir(dir).expect("read the dir") {
      let path = entry.expect("an entry").path();
      if path.is_dir() {
        walk(root, &path, out);
      } else {
        let rel = path
          .strip_prefix(root)
          .expect("under the root")
          .to_path_buf();
        out.insert(rel, std::fs::read(&path).expect("read the file"));
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(dir, dir, &mut out);
  out
}

/// One run under `home`: its stdout and its stderr.
fn run(home: &Path, args: &[&str]) -> (String, String) {
  let out = crate::common::intent()
    .args(args)
    .env("HOME", home)
    .current_dir(home)
    .output()
    .expect("run intent");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
  )
}

/// `--version`, `--help`, `info`, a family's `help`, `claude hook` and
/// `bootstrap --check` leave `~/.intent/` byte for byte and write nothing into
/// the layout it would move to. `claude hook` runs in every Claude session, and
/// its stdout reaches it.
#[test]
fn a_read_of_the_binary_leaves_a_legacy_home_byte_for_byte() {
  for args in [
    &["--version"][..],
    &["--help"],
    &["info"],
    &["st", "help"],
    &["claude", "hook", "session-finish"],
    &["bootstrap", "--check"],
  ] {
    let shown = args.join(" ");
    let home = legacy_home(&format!("read-{}", shown.replace(['-', ' '], "")));
    let before = tree(&home.join(".intent"));
    let (_, stderr) = run(&home, args);
    assert!(
      !stderr.contains("into the XDG layout"),
      "`intent {shown}` printed the migration note: {stderr}"
    );
    assert!(
      home.join(".intent").is_dir(),
      "`intent {shown}` removed ~/.intent"
    );
    assert_eq!(
      tree(&home.join(".intent")),
      before,
      "`intent {shown}` changed ~/.intent"
    );
    assert!(
      !home.join(".local/share/intent/skills").exists(),
      "`intent {shown}` wrote into the XDG layout"
    );
  }
}

/// `bootstrap` still migrates first: the entries reach the layout byte for
/// byte, `~/.intent/` goes, and the note says so.
#[test]
fn a_verb_that_does_work_still_migrates_and_says_so() {
  let home = legacy_home("bootstrap");
  let before = tree(&home.join(".intent"));
  let (_, stderr) = run(&home, &["bootstrap", "--quiet"]);
  assert!(
    stderr.contains("into the XDG layout, and removed it"),
    "no migration note: {stderr}"
  );
  assert!(!home.join(".intent").exists(), "~/.intent is still there");
  let data = home.join(".local/share/intent");
  for (rel, bytes) in before {
    assert_eq!(
      std::fs::read(data.join(&rel)).ok().as_ref(),
      Some(&bytes),
      "{} did not reach the layout byte for byte",
      rel.display()
    );
  }
}

/// `bootstrap --check` names the legacy home it leaves, so the check still
/// reports what the next verb that does work will move.
#[test]
fn a_check_names_the_legacy_home_it_leaves() {
  let home = legacy_home("check-names");
  let (stdout, stderr) = run(&home, &["bootstrap", "--check"]);
  let named = format!("{} is a legacy home", home.join(".intent").display());
  assert!(
    stdout.contains(&named),
    "the check did not name ~/.intent: stdout={stdout}\nstderr={stderr}"
  );
}
