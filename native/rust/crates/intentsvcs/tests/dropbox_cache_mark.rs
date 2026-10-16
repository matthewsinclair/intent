//! ST0083 WP-07, AC-07.1: the library half of the Dropbox mark on
//! `intent/.cache`.
//!
//! **THE FIXTURE ROOT IS REACHED THROUGH A SYMLINK**, as `~/Dropbox` reaches
//! `~/Library/CloudStorage/Dropbox` on a File Provider Mac, so an answer that
//! compared the paths as written would miss every project opened the usual
//! way. Each arm passes its own `home`, so none reads the operator's.

use intentsvcs::dropbox::{self, DropboxError, Marking};
use std::path::{Path, PathBuf};

/// A home whose `info.json` names `<tmp>/CloudStorage/Dropbox`, and the
/// symlink `<tmp>/Dropbox` pointing at it.
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

fn made(dir: &Path) -> PathBuf {
  std::fs::create_dir_all(dir).expect("mkdir");
  dir.to_path_buf()
}

#[test]
fn a_directory_carries_every_attribute_once_marked_and_none_before() {
  let tmp = tempfile::tempdir().expect("tempdir");
  assert!(
    !dropbox::is_marked(tmp.path()).expect("read"),
    "a fresh directory read as marked"
  );
  dropbox::mark(tmp.path()).expect("mark");
  assert!(dropbox::is_marked(tmp.path()).expect("read"));
  for attribute in dropbox::ATTRIBUTES {
    assert_eq!(
      xattr::get(tmp.path(), attribute).expect("read").as_deref(),
      Some(dropbox::VALUE),
      "{attribute} is not set to 1"
    );
  }
}

#[test]
fn a_directory_under_the_folder_through_its_symlink_is_marked() {
  let fx = fixture();
  let cache = made(&fx.link.join("book/intent/.cache"));
  let marking = dropbox::mark_if_under_dropbox(&cache, &fx.home);
  assert!(
    matches!(marking, Some(Marking::Marked { .. })),
    "a directory under the Dropbox folder by its symlink was not marked: {marking:?}"
  );
  assert!(dropbox::is_marked(&cache).expect("read"));
}

#[test]
fn a_directory_outside_every_folder_is_left_alone() {
  let fx = fixture();
  let outside = tempfile::tempdir().expect("tempdir");
  let cache = made(&outside.path().join("book/intent/.cache"));
  assert!(dropbox::mark_if_under_dropbox(&cache, &fx.home).is_none());
  assert!(!dropbox::is_marked(&cache).expect("read"));
}

#[test]
fn no_info_json_is_no_client_and_not_an_error() {
  let home = tempfile::tempdir().expect("tempdir");
  let cache = tempfile::tempdir().expect("tempdir");
  assert_eq!(
    dropbox::client_roots(home.path()).expect("read"),
    Vec::<PathBuf>::new()
  );
  assert!(dropbox::mark_if_under_dropbox(cache.path(), home.path()).is_none());
}

#[test]
fn an_unreadable_info_json_is_said_and_names_its_file() {
  let fx = fixture();
  std::fs::write(fx.home.join(".dropbox/info.json"), "not json").expect("write");
  let cache = made(&fx.link.join("book/intent/.cache"));
  match dropbox::mark_if_under_dropbox(&cache, &fx.home) {
    Some(Marking::Unmarked {
      cause: DropboxError::InfoMalformed { path, .. },
      ..
    }) => assert!(path.ends_with(".dropbox/info.json"), "{}", path.display()),
    other => panic!("a malformed info.json was not said: {other:?}"),
  }
}

#[test]
fn the_birth_of_the_cache_marks_only_a_directory_it_created() {
  let tmp = tempfile::tempdir().expect("tempdir");
  let cache = made(&tmp.path().join("intent/.cache"));
  assert!(
    dropbox::make_cache_dir(&cache).expect("exists").is_none(),
    "a directory that already existed was treated as born"
  );
}
