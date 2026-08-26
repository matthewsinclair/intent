//! **`Thread::attachments`: the authored files beside a thread that no typed
//! document holds.**
//!
//! The report that NAMES what is not carried lives in `doctor` and is tested
//! there, because an uncarried file is a live condition rather than a record
//! of what a migration once did.
//!
//! It exists because hv ruled disk optional -- an index plus render-on-demand
//! -- and the moment disk is optional, anything the store does not hold is
//! destroyed by the first render. Measured on this project the day the hoist
//! landed: of 485 `.md` under the thread estate, 380 were in the store and 52
//! were not, and the 52 were found by counting rather than by any surface
//! saying so.
//!
//! **The report is the property, not the carry.** Which files qualify is a
//! list of extensions and could reasonably be argued either way; that a file
//! outside the list is NAMED cannot, because silence and full coverage are
//! indistinguishable to a reader.
//!
//! What this file pins is the other half: that the carry is byte-exact, that
//! the derived fields cannot drift from the content, and that the partition
//! puts every file in exactly one place.

mod common;

use common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::Attachment;
use intentsvcs::project::{Project, ThreadFile};

fn project(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n",
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// The carry, and the two shapes that make it more than a file copy: a nested
/// path, and content that is NOT trimmed.
#[test]
fn an_authored_file_is_carried_whole_and_addressed_from_the_thread_root() {
  let fixture = Fixture::new();
  project(&fixture);
  fixture.write_file(
    "intent/st/ST0001/reference.md",
    "# Reference\n\nA quokka.\n",
  );
  fixture.write_file("intent/st/ST0001/notes/day-one.txt", "\n  ragged  \n\n");

  let thread = &scan(&fixture).threads[0];
  let paths: Vec<&str> = thread.attachments.iter().map(|a| a.path.as_str()).collect();
  assert_eq!(
    paths,
    vec!["notes/day-one.txt", "reference.md"],
    "both carried, addressed relative to the thread's own directory, in path order"
  );

  let nested = &thread.attachments[0];
  assert_eq!(
    nested.text.as_deref(),
    Some("\n  ragged  \n\n"),
    "**NOT trimmed, and not even the trailing newline.** An attachment round \
     trips to a file on disk, so byte-equality is the property and a trim \
     would cost one byte per file on every trip, forever. `Issue::body` holds \
     the same line for the same reason -- it was trimmed until a renderer for \
     it was scheduled, at which point the normalisation became a defect with a \
     date on it rather than a tidy-up"
  );
  assert_eq!(nested.bytes, 13, "bytes is the length of what was read");
}

/// **The derived fields cannot drift from the content, because there is one
/// way to make one.**
#[test]
fn bytes_and_sha256_describe_the_text_they_were_built_from() {
  let a = Attachment::new("reference.md", "# Reference\n");
  let text = a
    .text
    .as_deref()
    .expect("a text attachment carries its text");
  assert_eq!(a.bytes as usize, text.len());
  assert_eq!(a.sha256, intentsvcs::model::sha256_hex(text.as_bytes()));
  assert_ne!(
    a.sha256,
    intentsvcs::model::sha256_hex(b""),
    "and it is not the empty-string hash, which is what a field left unset \
     would carry and what 112 ratified template drops already look like"
  );
}
/// **A file that is not carried is NOT filed as a migration disposition
/// either -- and on 2026-08-26 its subject changed while the property did
/// not.**
///
/// It used to be "a file outside the declared extensions". `ATTACHMENT_EXTENSIONS`
/// is gone, so nothing is uncarried for having the wrong NAME; what is still
/// uncarried is what will not FIT, and the population this guards moved with
/// it. The property is untouched and is the reason the test survives the
/// rewrite rather than being deleted with the list.
///
/// `dropped` means content existed, was deliberately not brought across, and
/// canon is verified empty for it -- safe because nobody wanted it. These
/// files are STILL ON DISK and still the only copy. **The disposition record
/// is a licence, not an account**: `conservation_check.sh` reads a declared
/// drop as "removed on purpose, not loss" and stops reporting it, so filing
/// them there would silence the exact population that check exists to find.
///
/// Reporting the live condition is `doctor`'s job, tested there.
#[test]
fn a_file_that_does_not_attach_is_not_carried_and_not_licensed_as_a_drop() {
  let fixture = Fixture::new();
  project(&fixture);
  fixture.write_file("intent/st/ST0001/reference.md", "# Reference\n");
  fixture.write_file("intent/st/ST0001/baseline.tap", "ok 1 - a test\n");
  fixture.write_file("intent/st/ST0001/tools/run.sh", "#!/bin/bash\necho hi\n");
  std::fs::write(
    fixture.path("intent/st/ST0001/huge.png"),
    vec![b'x'; intentsvcs::project::ATTACHMENT_CAP_BYTES as usize + 1],
  )
  .expect("write a file over the cap");

  let out = scan(&fixture);
  assert_eq!(
    out.threads[0]
      .attachments
      .iter()
      .map(|a| a.path.as_str())
      .collect::<Vec<_>>(),
    vec!["baseline.tap", "reference.md", "tools/run.sh"],
    "the shell, the markdown AND the baseline are all carried now -- an \
     extension decides nothing. The baseline was excluded by the authorship \
     principle (no tool can make this again, versus a tool made this and can \
     again), which is NOT retired: it moves to `.intentfiles`, per project, \
     because a global list answered `.tap` right here and `.json` wrong \
     elsewhere (vc, 3.0.2). `huge.png` is absent because it will not FIT"
  );
  assert!(
    !out.dispositions.iter().any(|d| d.owner.ends_with(".png")),
    "a file still sitting on disk has not been dropped, and saying it was would \
     license the check that watches it to stop looking: {:?}",
    out
      .dispositions
      .iter()
      .map(|d| &d.owner)
      .collect::<Vec<_>>()
  );
}

/// **The typed documents and the generated views are consumed, not attached.**
///
/// A file is one thing or the other and never both: `design.md` is parsed into
/// the model, so carrying it here as well would put one file's content in two
/// places and let them disagree.
#[test]
fn the_generated_views_are_not_attached() {
  let fixture = Fixture::new();
  project(&fixture);
  fixture.write_file("intent/st/ST0001/design.md", "# Design\n\n## Shape\n\nA.\n");
  fixture.write_file("intent/st/ST0001/acceptance.md", "# Acceptance\n");
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    "---\nstatus: Done\n---\n\n# WP01\n",
  );

  let thread = &scan(&fixture).threads[0];
  let carried: Vec<&String> = thread.attachments.iter().map(|a| &a.path).collect();

  // **`design.md` MOVED SIDES on 2026-08-18 and the test says which side it is
  // on now** (D57-6). It was a typed document with a home in the parser and no
  // carriage; it is an attachment, carried verbatim, like any other authored
  // markdown under a thread.
  assert_eq!(
    carried,
    vec!["design.md"],
    "design.md is carried now; the generated views still are not: {carried:?}"
  );

  // The views are the unchanged half, and they are the reason this test exists:
  // each is rendered FROM the model, so carrying one would give a single fact
  // two homes and let a stale copy answer for the live one.
  for view in ["info.md", "acceptance.md", "WP/01/info.md"] {
    assert!(
      !carried.iter().any(|p| p.as_str() == view),
      "{view} is generated from the model and has a home already: {carried:?}"
    );
  }
}

/// **The view test keys on SHAPE, not on a bare filename**, and this is the
/// arm that proves the difference.
///
/// `info.md` at a thread's root is the generated cover. `parity/info.md` is
/// somebody's document that happens to share a name -- matching on the name
/// alone would take an author's file, call it ours, and overwrite it on the
/// next render.
#[test]
fn a_file_named_like_a_view_but_nested_is_an_authored_file() {
  assert_eq!(
    Project::classify(std::path::Path::new("info.md")),
    ThreadFile::GeneratedView
  );
  assert_eq!(
    Project::classify(std::path::Path::new("WP/01/info.md")),
    ThreadFile::GeneratedView
  );
  assert_eq!(
    Project::classify(std::path::Path::new("parity/info.md")),
    ThreadFile::Attachment,
    "three levels down under someone's own directory, this is not our cover"
  );
  assert_eq!(
    Project::classify(std::path::Path::new("notes/design.md")),
    ThreadFile::Attachment,
    "and a typed document's name only means that at the thread's root"
  );
  assert_eq!(
    Project::classify(std::path::Path::new("design.md")),
    ThreadFile::Attachment,
    "D57-6: THREAD_PROSE is deleted, so a thread's own design.md is carried \
     verbatim like any other authored markdown rather than skipped"
  );
  // **THIS ROW CHANGED ITS ANSWER ON 2026-08-26, AND THE CHANGE IS A RULING
  // RATHER THAN A CONSEQUENCE.**
  //
  // It used to assert `Unattached`, because `ATTACHMENT_EXTENSIONS` was
  // `["md", "txt", "sh"]` and a `.tap` was not on the list. That list encoded a
  // real principle of vc's -- *no tool can make this again, versus a tool made
  // this and can again* -- under which a generated baseline stays out because
  // carrying regenerable output buys nothing.
  //
  // **The principle survives; the list did not, because a global extension list
  // cannot answer a per-project question.** `.tap` is tool output HERE and
  // Conflab's `.json` may well be authored THERE, so the same list answered one
  // right and the other wrong. Its new home is `.intentfiles`, a per-project
  // declaration of what a project does not want carried (vc, 3.0.2).
  //
  // Until that lands, the baseline IS carried, under the cap. The measured cost
  // is 328 KB in one thread on this repo, taken deliberately in preference to
  // inventing a second classifier under release pressure.
  assert_eq!(
    Project::classify(std::path::Path::new("parity/baseline.tap")),
    ThreadFile::Attachment,
    "an extension no longer decides what is carried -- size does, and \
     authorship will, per project, once `.intentfiles` carries it"
  );
}

/// **A gitignored file is not canon and is not reported either** (D29).
///
/// It is excluded by RULE rather than by naming `.DS_Store`, which is the only
/// version of this that keeps working for the next stray file.
///
/// **REAL git, because the rule IS git's ignore semantics** -- and because the
/// walker's `require_git` default is a ratified decision rather than an
/// accident: a project with no repository has no ignore rules, so the corpus
/// degrades to everything-in-scope. A fixture that merely writes a
/// `.gitignore` into a bare tempdir tests the no-repository case while
/// claiming to test the ignore case, which is how this test failed first.
#[test]
fn a_gitignored_file_is_neither_carried_nor_reported() {
  let fixture = Fixture::new();
  project(&fixture);
  assert!(
    std::process::Command::new("git")
      .args(["init", "-q"])
      .current_dir(fixture.root())
      .status()
      .expect("run git")
      .success(),
    "git init failed"
  );
  fixture.write_file(".gitignore", "*.local.md\n");
  fixture.write_file("intent/st/ST0001/scratch.local.md", "not mine\n");
  fixture.write_file("intent/st/ST0001/reference.md", "# Reference\n");

  let out = scan(&fixture);
  assert_eq!(
    out.threads[0]
      .attachments
      .iter()
      .map(|a| a.path.as_str())
      .collect::<Vec<_>>(),
    vec!["reference.md"],
    "a path git does not carry cannot be canon"
  );
  assert!(
    !out
      .dispositions
      .iter()
      .any(|d| d.owner.contains("scratch.local")),
    "and it is not reported as uncarried either -- it was never a candidate"
  );
}
