//! AT-06.5 / AC-06.5: `intent schema` prints the committed faces, and what it
//! prints is byte-identical to the files under `schema/`.
//!
//! **The command GENERATES; the test compares against the committed files.**
//! That direction is the whole property. A `schema` command implemented as
//! `cat schema/<face>` would satisfy a byte-identity test vacuously and would
//! go on passing after the Rust types and the committed faces had drifted
//! apart -- which is the one failure the schema-as-truth model exists to make
//! impossible. Printing from the compiled-in types makes this a second,
//! independent witness to the drift `schema_faces_drift.rs` guards, reached by
//! a different route: that test regenerates in-process, this one runs the
//! shipped binary.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The REPOSITORY root -- where `schema/`, `surface/` and `bin/` live.
///
/// Searched, never counted. This was `ancestors().nth(2)`, which was correct
/// only while the workspace root and the repository root were the same
/// directory; `native/rust/` made them different and every counting caller
/// broke at once. A depth is a claim about a layout, and it goes stale
/// silently -- a search for the thing actually wanted cannot.
///
/// The marker is `schema/` AND `surface/` together, because either alone
/// could match some unrelated ancestor.
fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .find(|d| d.join("schema").is_dir() && d.join("surface").is_dir())
    .expect("a repository root carrying schema/ and surface/ above this crate")
    .to_path_buf()
}

fn run(cwd: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary")
}

fn committed_faces() -> Vec<(String, String)> {
  let dir = repo_root().join("schema");
  let mut faces: Vec<(String, String)> = std::fs::read_dir(&dir)
    .expect("schema/ exists at the workspace root")
    .filter_map(Result::ok)
    .filter(|e| e.path().is_file())
    .map(|e| {
      let name = e.file_name().to_string_lossy().to_string();
      let body = std::fs::read_to_string(e.path()).expect("read a committed face");
      (name, body)
    })
    .collect();
  faces.sort();
  assert!(
    !faces.is_empty(),
    "precondition: schema/ carries the committed faces"
  );
  faces
}

/// The property AC-06.5 states, per face.
#[test]
fn each_printed_face_is_byte_identical_to_its_committed_file() {
  let root = repo_root();
  for (name, committed) in committed_faces() {
    let out = run(&root, &["schema", &name]);
    assert_eq!(
      out.status.code(),
      Some(0),
      "`intent schema {name}` failed: {}",
      String::from_utf8_lossy(&out.stderr)
    );
    let printed = String::from_utf8(out.stdout).expect("a face is UTF-8");
    assert_eq!(
      printed, committed,
      "`intent schema {name}` differs from schema/{name} -- the model and the committed face have drifted, or the command stopped generating"
    );
  }
}

/// Neither side may carry a face the other does not. A command that printed
/// four of five faces would pass the per-face test above for all four.
#[test]
fn the_command_and_the_directory_carry_the_same_face_set() {
  let root = repo_root();
  let out = run(&root, &["schema"]);
  assert_eq!(out.status.code(), Some(0));
  let printed = String::from_utf8(out.stdout).expect("UTF-8");

  let mut banners: Vec<String> = printed
    .lines()
    .filter_map(|l| l.strip_prefix("== ").and_then(|l| l.strip_suffix(" ==")))
    .map(str::to_string)
    .collect();
  banners.sort();

  let mut committed: Vec<String> = committed_faces().into_iter().map(|(n, _)| n).collect();
  committed.sort();

  assert_eq!(
    banners, committed,
    "the faces the binary prints and the files committed under schema/ are not the same set"
  );
}

/// The bare form carries every face's CONTENT, not just its banner.
#[test]
fn the_bare_form_carries_every_faces_content() {
  let root = repo_root();
  let printed = String::from_utf8(run(&root, &["schema"]).stdout).expect("UTF-8");
  for (name, committed) in committed_faces() {
    assert!(
      printed.contains(committed.trim_end()),
      "the bare `intent schema` output does not contain all of {name}"
    );
  }
}

/// **The load-bearing one.** The faces are compiled in, so the command is the
/// same everywhere -- and it is most useful outside a project, when you are
/// deciding what a project should contain. Requiring a project here would be a
/// gratuitous failure in the command's best use.
#[test]
fn schema_works_outside_a_project() {
  let dir = tempfile::tempdir().expect("tempdir");
  let out = run(dir.path(), &["schema"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    String::from_utf8_lossy(&out.stdout).contains("== ddl.sql =="),
    "the faces do not depend on a project being present"
  );

  // The control: a verb that genuinely needs a project still refuses here, so
  // this test is not passing because the binary ignores its directory.
  let needs_project = run(dir.path(), &["st", "list"]);
  assert_eq!(
    needs_project.status.code(),
    Some(1),
    "control: `st list` must still require a project"
  );
}

/// An unknown face names the real ones rather than printing nothing.
#[test]
fn an_unknown_face_is_refused_and_names_the_alternatives() {
  let root = repo_root();
  let out = run(&root, &["schema", "not-a-face"]);
  assert_eq!(out.status.code(), Some(1));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "v2's voice: {stderr}");
  assert!(stderr.contains("remedy: "), "{stderr}");
  assert!(
    stderr.contains("ddl.sql"),
    "the remedy names the faces that do exist: {stderr}"
  );
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a failure writes nothing to stdout"
  );
}
