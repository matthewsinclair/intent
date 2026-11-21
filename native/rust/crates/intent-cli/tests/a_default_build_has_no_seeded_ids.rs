//! ST0095 AC-03.3: **THE SEED IS ABSENT FROM WHAT SHIPS** (intent-cc's design).
//!
//! The explorer shots take their event ids from a sequence file, through a
//! `seeded-ids` build the shots script makes for itself. A binary a person can
//! install has no seed at all: no env name, no code path, nothing to refuse.
//! This drives the default build the suite tests with `INTENT_SEEDED_IDS`
//! pointing at a sequence file. Two fresh projects each mint a thread, each with
//! its own file holding the same starting ULID, and the two threads' origins
//! differ, which a seeded mint from those files could not give. Neither file is
//! read, so each holds what it was given.
//!
//! The arm is compiled out of a `seeded-ids` build, which is the one build that
//! does read the name. That build refuses a release profile at compile time
//! (`event.rs`), and no arm drives that refusal: it is a compile-time fact, and
//! a test of it would be a planted fault.

#![cfg(not(feature = "seeded-ids"))]

use std::path::Path;

/// The env name a `seeded-ids` build reads, written here as a literal because
/// the default build this arm tests has no such name to import.
const SEEDED_IDS_ENV: &str = "INTENT_SEEDED_IDS";

/// A ULID a seeded run could start from.
const START: &str = "01JD00000000000000000000AA\n";

/// `intent` in `cwd` with the seeded-ids name set, answering what it printed
/// and its exit code.
fn run_seeded(args: &[&str], cwd: &Path, file: &Path) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env(SEEDED_IDS_ENV, file)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

// ST0095 AC-03.3: the seed is absent from the default build.
#[test]
fn a_default_build_reads_no_seeded_ids_file() {
  let seed = tempfile::tempdir().expect("tempdir");
  // Each project names its own sequence file, both holding the same start: a
  // build that read them would mint the same origin twice.
  let runs: Vec<(String, std::path::PathBuf)> = ["seeded-a", "seeded-b"]
    .iter()
    .map(|name| {
      let file = seed.path().join(format!("{name}.ids"));
      std::fs::write(&file, START).expect("write the sequence file");
      let dir = tempfile::tempdir().expect("tempdir");
      let root = dir.path();
      let (out, code) = run_seeded(&["init", name], root, &file);
      assert_eq!(code, 0, "init: {out}");
      let (out, code) = run_seeded(&["st", "new", "A thread"], root, &file);
      assert_eq!(code, 0, "st new: {out}");
      let id = crate::common::created(&out);
      let canon = crate::common::canon_json(root, &intentsvcs::project::canon_thread_rel(&id));
      let origin = canon["origin"]
        .as_str()
        .unwrap_or_else(|| panic!("the new thread {id} names no origin: {canon}"))
        .to_string();
      (origin, file)
    })
    .collect();

  assert_ne!(
    runs[0].0, runs[1].0,
    "two projects minted one origin from one starting ULID, so the default build read its sequence file"
  );
  for (_, file) in &runs {
    assert_eq!(
      std::fs::read_to_string(file).expect("read the sequence file"),
      START,
      "the default build wrote the sequence file {}",
      file.display()
    );
  }
}
