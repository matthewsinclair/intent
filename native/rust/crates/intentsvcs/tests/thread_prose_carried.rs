//! AT-00.2 / ST0057 AC-00.2: **`THREAD_PROSE` names no filename in the
//! classifier, and every `design.md`, `impl.md` and `tasks.md` under
//! `intent/st/` is carried as an attachment whose bytes round-trip** (D57-6).
//!
//! # Two halves, and only one of them is about the files
//!
//! The carry is the visible half. The other is that the classifier reaches it
//! WITHOUT KNOWING THOSE NAMES -- because a name list is a roster somebody
//! must extend on the day they add a fourth prose file, and `project.rs`
//! already says so at the seam: _a name list here would be `THREAD_PROSE`
//! wearing a different constant_.
//!
//! # The name half is tested BEHAVIOURALLY, not by grepping the source
//!
//! Asserting that the string `design.md` does not appear in `project.rs`
//! answers a question about the file rather than about the classifier -- it
//! passes the moment someone spells the list differently, and fails the moment
//! someone writes the name in a comment. **So the discriminator is a name
//! nobody has ever used**: an invented filename must classify IDENTICALLY to
//! the three real ones. A classifier keyed on names cannot do that, and a
//! classifier keyed on shape cannot fail to.
//!
//! **With a control, because "everything is an Attachment" satisfies that
//! completely.** `info.md` at a thread root is a generated view, so it must
//! come back DIFFERENT -- which is what proves the classifier discriminates at
//! all before its agreement on the three names means anything.
//!
//! # Property, not count
//!
//! The row says so and the reason is the failure it excludes: a carry covering
//! SOME files is indistinguishable from one covering ALL under any assertion
//! that only counts. So every file found on disk is checked individually and
//! the enumerated total is printed as the denominator.
//!
//! # And presence is not the property either
//!
//! An attachment present with the wrong bytes is the shape this thread keeps
//! meeting -- the artefact is there, the check that looks for it passes, and
//! the content is somebody else's. So the assertion is byte equality against
//! the file on disk, not membership.

mod common;

use intentsvcs::ingest;
use intentsvcs::project::{Project, ThreadFile};
use std::path::{Path, PathBuf};
use testkit::repo_root;

/// The three names D57-6 names. Used to FIND files on disk, never to decide
/// what they are -- that is the classifier's job and the first test is that it
/// does it without this list.
const PROSE: &[&str] = &["design.md", "impl.md", "tasks.md"];

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      walk(&path, out);
    } else if path
      .file_name()
      .and_then(|n| n.to_str())
      .is_some_and(|n| PROSE.contains(&n))
    {
      out.push(path);
    }
  }
}

#[test]
fn the_classifier_answers_by_shape_and_never_by_a_prose_filename() {
  let invented = Path::new("zzz-no-classifier-has-ever-heard-of-this.md");
  let baseline = Project::classify(invented);

  for name in PROSE {
    assert_eq!(
      Project::classify(Path::new(name)),
      baseline,
      "`{name}` classifies differently from an invented filename at the same shape, so the \
       classifier is keyed on the NAME. That is `THREAD_PROSE` wearing a different constant: it \
       carries three files today and silently drops the fourth one anybody adds"
    );
  }

  // **The control.** A classifier that answered `Attachment` for everything
  // would satisfy every assertion above, so something at the same depth must
  // come back DIFFERENT before their agreement means anything.
  assert_ne!(
    Project::classify(Path::new("info.md")),
    baseline,
    "the classifier gives one answer for a generated view and an invented file alike, so it is \
     not discriminating at all and the agreement asserted above is vacuous"
  );
  assert_eq!(
    baseline,
    ThreadFile::Attachment,
    "an ordinary `.md` under a thread is carried, and if that ever stops being true the \
     comparisons above would agree on the wrong answer"
  );
}

#[test]
fn every_thread_prose_file_is_carried_and_its_bytes_round_trip() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads from the real estate");

  let st_dir = root.join("intent/st");
  let mut found = Vec::new();
  walk(&st_dir, &mut found);

  assert!(
    !found.is_empty(),
    "no prose file was found under {}, so this probe's population cannot contain the failure it \
     tests for",
    st_dir.display()
  );

  let mut checked = 0usize;
  let mut problems: Vec<String> = Vec::new();

  for file in &found {
    let rel = file
      .strip_prefix(&st_dir)
      .expect("found under intent/st by construction");
    let mut parts = rel.components();
    let Some(thread_id) = parts.next().map(|c| c.as_os_str().to_string_lossy().to_string()) else {
      problems.push(format!("{}: no thread directory above it", rel.display()));
      continue;
    };
    // The attachment path is relative to the THREAD's own directory.
    let within: PathBuf = parts.collect();
    let within = within.to_string_lossy().replace('\\', "/");

    let Some(thread) = canon.threads.iter().find(|t| t.id == thread_id) else {
      problems.push(format!(
        "{thread_id}/{within}: the file is on disk and its thread is not in canon"
      ));
      continue;
    };
    let Some(carried) = thread.attachments.iter().find(|a| a.path == within) else {
      problems.push(format!(
        "{thread_id}/{within}: on disk and NOT carried -- dehydrating this thread would remove a \
         file canon has no copy of"
      ));
      continue;
    };
    checked += 1;

    let on_disk = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("read {rel:?}: {e}"));
    if carried.text != on_disk {
      problems.push(format!(
        "{thread_id}/{within}: carried, but the bytes differ -- canon holds {} byte(s), disk holds \
         {}. Presence is not the property; an attachment carried with the wrong content passes \
         every check that looks for it",
        carried.text.len(),
        on_disk.len()
      ));
    }
  }

  eprintln!(
    "AT-00.2: {checked} of {} prose file(s) carried and byte-compared, across {} thread(s)",
    found.len(),
    canon.threads.len()
  );

  assert!(
    problems.is_empty(),
    "{} of {} prose file(s) are not carried as canon holds them:\n  {}",
    problems.len(),
    found.len(),
    problems.join("\n  ")
  );
  assert_eq!(
    checked,
    found.len(),
    "the byte comparison ran over {checked} file(s) of {} found -- the two must be equal or some \
     file was skipped without being reported",
    found.len()
  );
}
