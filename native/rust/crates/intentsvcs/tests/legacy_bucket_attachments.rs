//! **A BUCKETED THREAD MIGRATED WITH ZERO ATTACHMENTS, AT rc 0.**
//!
//! `thread_dirs` walks the top level AND v2's three status buckets, because
//! `intent st done` MOVES a thread into `COMPLETED/` -- on this estate 54 of
//! 56. It hands each thread back as `(id, dir)`. The attachment walk then threw
//! the `dir` away and re-derived the FLAT `intent/st/<ID>/`, which for a
//! bucketed thread does not exist. `thread_files` walks an absent directory,
//! gets nothing, and every downstream count reconciles perfectly against zero.
//!
//! **NOTHING REPORTED IT, AND THAT IS THE WHOLE DEFECT.** No refusal, because
//! refusals are produced per file and there were no files. No shortfall,
//! because the row accounting reconciles AC/AT rows and an attachment is not a
//! row. The thread converted, the migration printed `ok`, and the authored
//! prose beside it -- `design.md`, `impl.md`, `tasks.md` -- was simply not
//! there. Measured on Devbin: 34 of 54 bucket files carry content the store
//! does not hold, and the estate's own restart brief routes two of its five
//! opening questions to bucket-only paths.
//!
//! # Why every arm here probes CONTENT, per file
//!
//! Every natural check is satisfiable by an ingest that did nothing.
//! "attachments is non-empty" passes if paths were carried without content.
//! "the thread has attachments" passes on a HALF-migrated thread -- which is
//! the observed shape, not a hypothetical: ST0001's `acceptance.md` migrated
//! and its `design.md` did not. So each file is probed for a phrase only it
//! carries, and [`a_phrase_never_written_is_carried_by_nothing`] pins that the
//! probe can still return zero -- a probe that finds everything proves nothing.

mod common;

use common::{Fixture, facade_ctx};
use intentsvcs::{legacy, migrate};

/// `(bucket, id, file, phrase)` -- the phrase appears in exactly one file, so
/// a hit names which file survived rather than which thread did.
const ESTATE: &[(&str, &str, &str, &str)] = &[
  ("", "ST0001", "reference.md", "phrase-flat-reference"),
  (
    "COMPLETED/",
    "ST0002",
    "design.md",
    "phrase-completed-design",
  ),
  ("COMPLETED/", "ST0002", "impl.md", "phrase-completed-impl"),
  ("COMPLETED/", "ST0002", "tasks.md", "phrase-completed-tasks"),
  (
    "NOT-STARTED/",
    "ST0003",
    "plan.md",
    "phrase-notstarted-plan",
  ),
  ("CANCELLED/", "ST0004", "why.md", "phrase-cancelled-why"),
];

/// Written to no file anywhere in the fixture.
const NEVER_WRITTEN: &str = "phrase-never-written-to-disk";

fn status_for(bucket: &str) -> &'static str {
  match bucket {
    "COMPLETED/" => "Completed",
    "NOT-STARTED/" => "Not Started",
    "CANCELLED/" => "Cancelled",
    _ => "WIP",
  }
}

fn v2_estate(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  for (bucket, id, file, phrase) in ESTATE {
    fixture.write_file(
      &format!("intent/st/{bucket}{id}/info.md"),
      &format!(
        "---\nstatus: {}\ncreated: 20260816\n---\n\n# {id}: A thread\n\n## Objective\n\nShip it.\n",
        status_for(bucket)
      ),
    );
    fixture.write_file(
      &format!("intent/st/{bucket}{id}/{file}"),
      &format!("# {file}\n\nThis file alone says {phrase}.\n"),
    );
  }
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// The text carried for one thread's one attachment path, if it was carried.
fn carried_text(scan: &legacy::Scan, id: &str, file: &str) -> Option<String> {
  scan
    .threads
    .iter()
    .find(|t| t.id == id)?
    .attachments
    .iter()
    .find(|a| a.path == file)?
    .text
    .clone()
}

/// **THE CONTROL ON THE INSTRUMENT, and it comes first because every arm below
/// is conditional on it.** A flat thread takes the identical fixture shape and
/// the identical probe. If this is red, the probe is broken and the bucket arms
/// below are measuring the probe rather than the subject.
#[test]
fn a_flat_threads_attachment_is_carried_with_its_content() {
  let fixture = Fixture::new();
  v2_estate(&fixture);

  let text = carried_text(&scan(&fixture), "ST0001", "reference.md")
    .expect("the flat thread's attachment is carried");
  assert!(
    text.contains("phrase-flat-reference"),
    "the probe finds a phrase in a flat thread's carried attachment: {text:?}"
  );
}

/// **The subject, probed PER FILE.** A thread-level check passes on a
/// half-migrated thread, so each file is asserted on its own phrase and the
/// failure names the file that went missing.
#[test]
fn every_bucketed_file_is_carried_with_its_own_content() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let scanned = scan(&fixture);

  for (bucket, id, file, phrase) in ESTATE.iter().filter(|(b, ..)| !b.is_empty()) {
    let text = carried_text(&scanned, id, file).unwrap_or_else(|| {
      panic!("intent/st/{bucket}{id}/{file} was not carried at all -- the bucket walk lost it")
    });
    assert!(
      text.contains(phrase),
      "intent/st/{bucket}{id}/{file} was carried without its content: {text:?}"
    );
  }
}

/// **THE NEGATIVE ARM: the probe can still return zero.** It must pass BEFORE
/// the fix as well as after -- a probe that matches a phrase nobody wrote would
/// have reported every arm above green against an empty migration.
#[test]
fn a_phrase_never_written_is_carried_by_nothing() {
  let fixture = Fixture::new();
  v2_estate(&fixture);

  let scanned = scan(&fixture);
  let hits: Vec<&str> = scanned
    .threads
    .iter()
    .flat_map(|t| t.attachments.iter())
    .filter(|a| {
      a.text
        .as_deref()
        .is_some_and(|text| text.contains(NEVER_WRITTEN))
    })
    .map(|a| a.path.as_str())
    .collect();

  assert!(
    hits.is_empty(),
    "a phrase written to no file was found in {hits:?} -- the probe matches more than its subject"
  );
}

/// **The store-level probe: what the migration would COMMIT holds the content.**
/// The arms above read the scan; this one reads the planned canon bytes, which
/// is the artefact a reader greps a year later.
#[test]
fn the_planned_canon_holds_every_bucketed_files_content() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let project = fixture.project();

  let planned = migrate::plan(&project, &facade_ctx(), scan(&fixture)).expect("plan the migration");
  let canon: String = planned
    .writes
    .writes()
    .filter(|(path, _)| path.to_string_lossy().contains("/.canon/st/"))
    .map(|(_, content)| content)
    .collect::<Vec<_>>()
    .join("\n");

  for (bucket, id, file, phrase) in ESTATE.iter().filter(|(b, ..)| !b.is_empty()) {
    assert!(
      canon.contains(phrase),
      "the planned canon does not hold intent/st/{bucket}{id}/{file}"
    );
  }
  assert!(
    !canon.contains(NEVER_WRITTEN),
    "the canon probe matches a phrase nobody wrote"
  );
}

/// **The whole-population arm.** The per-file arms name what went missing; this
/// one catches a fix that reaches one bucket and not the other two.
#[test]
fn the_carry_covers_every_bucket_the_thread_walk_covers() {
  let fixture = Fixture::new();
  v2_estate(&fixture);

  let carried: usize = scan(&fixture)
    .threads
    .iter()
    .map(|t| t.attachments.len())
    .sum();

  assert_eq!(
    carried,
    ESTATE.len(),
    "one attachment per fixture file, across the flat tree and all three v2 buckets"
  );
}
