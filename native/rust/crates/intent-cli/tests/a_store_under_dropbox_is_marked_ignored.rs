//! ST0083 WP-07, AC-07.1: a store under a Dropbox folder is marked ignored for
//! the Dropbox client, said once, and `doctor` names an unmarked one.
//!
//! **DRIVEN THROUGH THE BINARY, UNDER A HOME OF ITS OWN** whose
//! `.dropbox/info.json` names `<tmp>/CloudStorage/Dropbox`, while the project
//! is made through the symlink `<tmp>/Dropbox`, the way `~/Dropbox` reaches the
//! File Provider's folder. An answer that compared paths as written would miss
//! every project opened that way.

use intentsvcs::dropbox;
use std::path::{Path, PathBuf};

struct Fixture {
  _tmp: tempfile::TempDir,
  home: PathBuf,
  link: PathBuf,
}

fn fixture() -> Fixture {
  let tmp = tempfile::tempdir().expect("tempdir");
  let real = tmp.path().join("CloudStorage/Dropbox");
  std::fs::create_dir_all(&real).expect("the Dropbox folder");
  let link = tmp.path().join("Dropbox");
  std::os::unix::fs::symlink(&real, &link).expect("the symlink");
  let home = tmp.path().join("home");
  std::fs::create_dir_all(home.join(".dropbox")).expect("the client's directory");
  let info = serde_json::json!({ "personal": { "path": real, "is_team": false } });
  std::fs::write(home.join(".dropbox/info.json"), info.to_string()).expect("info.json");
  Fixture {
    _tmp: tmp,
    home,
    link,
  }
}

fn run(args: &[&str], cwd: &Path, home: &Path) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A project made by `init` at `dir`, which is created first.
fn initialised(dir: &Path, home: &Path) -> String {
  std::fs::create_dir_all(dir).expect("mkdir");
  let (out, err, code) = run(&["init", "Book"], dir, home);
  assert_eq!(code, 0, "init failed: {out}{err}");
  out
}

const MARKED: &str = "marked: intent/.cache ignored by Dropbox";

#[test]
fn init_under_dropbox_marks_the_cache_and_says_so_once() {
  let fx = fixture();
  let book = fx.link.join("book");
  let out = initialised(&book, &fx.home);
  assert_eq!(
    out.matches(MARKED).count(),
    1,
    "init under Dropbox did not say once that it marked intent/.cache:\n{out}"
  );
  assert!(dropbox::is_marked(&book.join("intent/.cache")).expect("read"));
}

#[test]
fn init_outside_dropbox_marks_nothing_and_says_nothing() {
  let fx = fixture();
  let outside = tempfile::tempdir().expect("tempdir");
  let out = initialised(&outside.path().join("book"), &fx.home);
  assert!(
    !out.contains("Dropbox"),
    "init outside Dropbox spoke of it:\n{out}"
  );
  assert!(!dropbox::is_marked(&outside.path().join("book/intent/.cache")).expect("read"));
}

#[test]
fn the_first_open_of_a_fresh_clone_marks_the_cache_and_only_that_open_says_so() {
  let fx = fixture();
  let book = fx.link.join("book");
  initialised(&book, &fx.home);
  // A fresh clone has no `intent/.cache`: it is ignored by git.
  std::fs::remove_dir_all(book.join("intent/.cache")).expect("drop the cache");

  let (out, err, code) = run(&["st", "list"], &book, &fx.home);
  assert_eq!(code, 0, "st list failed: {out}{err}");
  assert_eq!(
    err.matches(MARKED).count(),
    1,
    "the open that created intent/.cache did not say once that it marked it\nstdout:\n{out}\nstderr:\n{err}"
  );
  assert!(
    !out.contains(MARKED),
    "the line reached stdout, which a caller may parse:\n{out}"
  );
  assert!(dropbox::is_marked(&book.join("intent/.cache")).expect("read"));

  let (out, err, code) = run(&["st", "list"], &book, &fx.home);
  assert_eq!(code, 0, "st list failed: {out}{err}");
  assert!(!err.contains(MARKED), "a second open said it again:\n{err}");
}

#[test]
fn doctor_names_an_unmarked_cache_under_dropbox_with_the_command_and_does_not_count_it() {
  let fx = fixture();
  let book = fx.link.join("book");
  initialised(&book, &fx.home);
  let (out, err, _) = run(&["doctor"], &book, &fx.home);
  assert!(
    !out.contains("dropbox-cache-unmarked"),
    "doctor reported a marked cache:\n{out}{err}"
  );

  // A cache made by hand, as a store from before this build has one. The
  // baseline is the same tree read by a HOME with no Dropbox client, so the
  // client is the only difference between the two readings.
  std::fs::remove_dir_all(book.join("intent/.cache")).expect("drop the cache");
  std::fs::create_dir(book.join("intent/.cache")).expect("an unmarked cache");
  let no_client = tempfile::tempdir().expect("tempdir");
  let (before, _, before_code) = run(&["doctor"], &book, no_client.path());
  let (out, err, code) = run(&["doctor"], &book, &fx.home);
  assert!(
    out.contains("dropbox-cache-unmarked") && out.contains(&dropbox::command("intent/.cache")),
    "doctor did not name an unmarked cache under Dropbox with the command that marks it:\n{out}{err}"
  );
  // The counted half of the verdict line, up to its first ` -- `: the
  // advisory tally after it rightly grows by this one.
  let verdict = |report: &str| {
    report
      .lines()
      .find(|line| line.starts_with("doctor:"))
      .and_then(|line| line.split(" -- ").next())
      .map(str::to_string)
  };
  assert_eq!(
    (verdict(&out), code),
    (verdict(&before), before_code),
    "the advisory moved doctor's count or exit:\nbefore:\n{before}\nafter:\n{out}"
  );
}

#[test]
fn doctor_says_nothing_of_dropbox_for_a_cache_outside_it() {
  let fx = fixture();
  let outside = tempfile::tempdir().expect("tempdir");
  let book = outside.path().join("book");
  initialised(&book, &fx.home);
  let (out, err, _) = run(&["doctor"], &book, &fx.home);
  assert!(
    !out.contains("dropbox"),
    "doctor spoke of Dropbox outside it:\n{out}{err}"
  );
}
