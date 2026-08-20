//! AT-06.3, covering ST0057 AC-06.3 -- **`export --format md` is accepted, and
//! its refusal is removed in the SAME change that makes the refusal's premise
//! false, not before.**
//!
//! **THE PREMISE THAT EXPIRED IS THE POPULATION, NOT THE FACT.** It is not that
//! markdown became readable back -- it did not, and this file asserts that it
//! still is not. It is that markdown was being judged by the INTERCHANGE rule.
//! `RoundTrips` governs canon into another MACHINE format, which is D03's
//! mechanism and the reason v3 can refuse YAML canon without refusing YAML
//! users. The text realisation is not one: `.backup/text/<UTC>/`, no import
//! path, `classify` never sees it, never authoritative. It is a human fallback
//! and was never in that population (hv ruling, 2026-08-20).
//!
//! **SO THE TWO HALVES ARE ASSERTED TOGETHER AND THAT IS THE WHOLE DESIGN OF
//! THIS FILE.** Accepted-and-still-not-readable-back is one claim, not two: a
//! test that only checked acceptance would go green on a build that had
//! quietly grown an import path, which is precisely what AC-06.2 forbids and
//! precisely the change a future contributor would think helpful.

use std::process::Command;

fn run(args: &[&str], cwd: &std::path::Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A project with one real thread, made through the CLI rather than by writing
/// canon by hand -- so the estate being realised is one the product produced.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"P\",\"author\":\"dc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");
  let (_, err, code) = run(&["st", "new", "a thread to realise"], dir.path());
  assert_eq!(code, 0, "fixture setup failed: {err}");
  dir
}

/// **It is accepted, it realises, and it says where and how completely.**
#[test]
fn md_is_accepted_and_realises_with_a_denominator() {
  let dir = project();
  let (out, err, code) = run(&["export", "--format", "md"], dir.path());

  assert_eq!(code, 0, "`--format md` was refused: {err}");

  // THE REFUSAL IS GONE, asserted on its own words rather than on the exit
  // code alone. A future change that made this exit 0 while still printing the
  // refusal would satisfy the code check and fail the operator.
  assert!(
    !err.contains("cannot be read back") && !err.to_lowercase().contains("refused"),
    "the refusal survived its own retirement: {err}"
  );

  // WHERE IT WENT. There is no artefact on stdout for a tree-shaped format, so
  // the destination IS the answer, and an operator who cannot find it has been
  // told nothing.
  assert!(
    out.contains(".backup/text/"),
    "it does not say where the realisation landed: {out}"
  );

  // **THE DENOMINATOR, NOT THE COUNT (AC-06.1).** `n/n` for each population.
  // A bare "wrote 41 files" cannot be wrong out loud; a ratio can.
  for population in ["threads", "wps", "issues", "attachments", "views"] {
    assert!(
      out.contains(population),
      "the denominator omits `{population}`, so a partial realisation reads as complete: {out}"
    );
  }
  assert!(
    out.contains('/'),
    "the counts are printed without their denominators: {out}"
  );
  assert!(
    out.contains("complete"),
    "it does not say out loud whether every artefact arrived: {out}"
  );
}

/// **The tree is really there, and it is under the path AC-06.2 names.**
///
/// Asserted by walking the filesystem rather than by trusting the message,
/// because the message is produced by the same code that would be wrong.
#[test]
fn the_realisation_is_written_under_backup_text() {
  let dir = project();
  let (_, err, code) = run(&["export", "--format", "md"], dir.path());
  assert_eq!(code, 0, "`--format md` was refused: {err}");

  let root = dir.path().join("intent/.backup/text");
  assert!(
    root.is_dir(),
    "nothing was written under {}",
    root.display()
  );

  let stamps: Vec<_> = std::fs::read_dir(&root)
    .expect("read the realisation root")
    .filter_map(|e| e.ok())
    .collect();
  assert_eq!(
    stamps.len(),
    1,
    "expected exactly one stamped realisation directory, found {}",
    stamps.len()
  );

  let files = walk(&stamps[0].path());
  assert!(
    !files.is_empty(),
    "the stamped directory is empty, so acceptance produced nothing"
  );
  assert!(
    files
      .iter()
      .any(|p| p.extension().is_some_and(|e| e == "md")),
    "the realisation contains no markdown at all: {files:?}"
  );
}

/// **STILL NO READ-BACK ROUTE, AND THIS IS THE HALF THAT MUST NOT SILENTLY
/// LAPSE** (AC-06.2, asserted here because AC-06.3's acceptance is what would
/// tempt someone to add one).
///
/// The realisation is a human fallback. If `ingest` or `sync` ever learned to
/// read it, markdown would become an interchange format by accident and
/// `RoundTrips`'s rule -- which is NOT relaxed for interchange formats -- would
/// have been evaded rather than met.
#[test]
fn accepting_md_did_not_create_a_route_back() {
  let dir = project();
  let (_, err, code) = run(&["export", "--format", "md"], dir.path());
  assert_eq!(code, 0, "`--format md` was refused: {err}");

  let root = dir.path().join("intent/.backup/text");
  let stamp = std::fs::read_dir(&root)
    .expect("read the realisation root")
    .filter_map(|e| e.ok())
    .next()
    .expect("one stamped directory")
    .path();
  let some_md = walk(&stamp)
    .into_iter()
    .find(|p| p.extension().is_some_and(|e| e == "md"))
    .expect("at least one markdown file");

  // Named on the command line, which is the strongest form of the ask: not
  // "does it happen to skip this directory" but "does it refuse when pointed
  // straight at it".
  let (_, err, code) = run(
    &["ingest", some_md.to_str().expect("utf-8 path")],
    dir.path(),
  );
  assert_ne!(
    code, 0,
    "`ingest` accepted a realised markdown file, so the realisation has become authoritative: {err}"
  );
}

/// Every file under `root`, recursively. Small by construction -- the fixture
/// is one thread -- so no depth guard is warranted and none is implied.
fn walk(root: &std::path::Path) -> Vec<std::path::PathBuf> {
  let mut found = Vec::new();
  let Ok(entries) = std::fs::read_dir(root) else {
    return found;
  };
  for entry in entries.filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_dir() {
      found.extend(walk(&path));
    } else {
      found.push(path);
    }
  }
  found
}
