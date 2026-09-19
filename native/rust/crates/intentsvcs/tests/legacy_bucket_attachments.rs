//! AT-02.1 / AC-02.1 -- ST0069 WP-02: the per-file content probe for the
//! bucket carry. The rows were minted on ST0069 when WP-02 built the prune that
//! stands on this carry; the arms predate them and are cited as they are.
//!
//! **A BUCKETED THREAD MIGRATED WITH ZERO ATTACHMENTS, AT rc 0.**
//!
//! `thread_dirs` walks the top level AND v2's status buckets, because `intent
//! st done` MOVES a thread into `COMPLETED/` -- on this estate, nearly every
//! thread. It hands each thread back as `(id, dir)`. The attachment walk then
//! threw the `dir` away and re-derived the FLAT `intent/st/<ID>/`, which for a
//! bucketed thread does not exist. `thread_files` walks an absent directory,
//! gets nothing, and every downstream count reconciles perfectly against zero.
//!
//! **NOTHING REPORTED IT, AND THAT IS THE WHOLE DEFECT.** No refusal, because
//! refusals are produced per file and there were no files. No shortfall,
//! because the row accounting reconciles AC/AT rows and an attachment is not a
//! row. The thread converted, the migration printed `ok`, and the authored
//! prose beside it -- `design.md`, `impl.md`, `tasks.md` -- was simply not
//! there. Measured on Devbin: most bucket files carry content the store
//! does not hold, and the estate's own restart brief routes some of its
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

use crate::common::{Fixture, facade_ctx, sample_thread};
use intentsvcs::facade::Facade;
use intentsvcs::organize::Mode;
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
    .map(|(_, content)| String::from_utf8_lossy(content))
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

/// `(file, phrase)` under `intent/st/COMPLETED/ST0010/` on an estate that is
/// ALREADY v3. Neither name collides with `sample_thread`'s own attachments, so
/// every one of these is content canon has never seen.
const BUCKET_ONLY: &[(&str, &str)] = &[
  ("design.md", "phrase-bucket-only-design"),
  ("notes.md", "phrase-bucket-only-notes"),
];

/// **0319: AN ESTATE THAT IS ALREADY v3 STILL INGESTS WHAT ITS BUCKETS HOLD.**
///
/// `legacy::scan` loads a thread with committed canon and moves on, which is
/// right for the markdown beside it (a generated view) and wrong for an
/// attachment sitting in a v2 bucket: nothing else ever carries it, so the prune
/// probe finds it unheld on every run and the v2 tree can never go. Laksa is
/// that shape, and so is every fleet estate converted before WP-02.
///
/// **THE INGEST AND THE REMOVAL ARE TWO RUNS, AND THE ARM PINS BOTH HALVES.**
/// The upgrade carries the files and deletes nothing (vc, 2026-09-13: a door
/// that ingests and prunes in one run names what it removed only afterwards);
/// the organize preview that follows is where they become removable, by name.
/// A fix that ingested and then pruned in the same run reds the second half,
/// because the preview then has nothing to prune.
#[test]
fn an_already_migrated_estates_bucket_files_are_ingested_and_left_for_organize_to_prune() {
  let fixture = Fixture::new();
  fixture.write_thread(&sample_thread("ST0010"));
  for (file, phrase) in BUCKET_ONLY {
    fixture.write_file(
      &format!("intent/st/COMPLETED/ST0010/{file}"),
      &format!("# {file}\n\nThis file alone says {phrase}.\n"),
    );
  }

  Facade::upgrade(&fixture.project(), &facade_ctx()).expect("an already-migrated estate re-runs");

  let project = fixture.project();
  let mut facade = fixture.facade_on_disk();
  let withheld = legacy::leftovers(&project, facade.canon()).withheld;
  assert!(
    withheld.is_empty(),
    "the upgrade left bucket files the store does not hold: {withheld:?}"
  );

  let preview = facade.organize(Mode::Preview).expect("organize previews");
  assert!(
    preview.refused.is_empty(),
    "the preview refused: {:?}",
    preview.refused
  );
  let bucket = project.st_dir().join("COMPLETED").join("ST0010");
  let mut to_prune: Vec<String> = preview
    .pruned_legacy
    .iter()
    .filter_map(|p| p.strip_prefix(&bucket).ok())
    .map(|p| p.to_string_lossy().into_owned())
    .collect();
  to_prune.sort();
  assert_eq!(
    (to_prune, preview.pruned_legacy.len()),
    (vec!["design.md".to_string(), "notes.md".to_string()], 2),
    "the preview prunes exactly the two ingested bucket files, so the upgrade removed neither"
  );
}

/// **WHAT THE INGEST DECLINES IS NAMED, WITH ITS REASON, AND NEVER HALTS THE RUN**
/// (issue 0319, vc 2026-09-13). Two ways a bucket file is not ingested, both on
/// the fleet's real shape: a name the naming gate refuses (a `?` cannot survive
/// the attachment URL), and a copy of an attachment canon already carries at a
/// different sha, where canon wins. Beside them one ordinary file IS ingested,
/// so a door that refused the whole bucket over one bad name reds here too.
#[test]
fn a_bucket_file_the_ingest_declines_is_named_with_its_reason_and_the_upgrade_still_runs() {
  let fixture = Fixture::new();
  let thread = sample_thread("ST0011");
  let canon_reference = thread
    .attachments
    .iter()
    .find(|a| a.path == "reference.md")
    .and_then(|a| a.text.clone())
    .expect("sample_thread carries reference.md as text");
  fixture.write_thread(&thread);
  let bucket = "intent/st/COMPLETED/ST0011";
  fixture.write_file(&format!("{bucket}/kept.md"), "# kept\n\nIngested.\n");
  fixture.write_file(
    &format!("{bucket}/bad?name.md"),
    "# bad\n\nUnaddressable.\n",
  );
  fixture.write_file(
    &format!("{bucket}/reference.md"),
    "# Reference\n\nAn older copy canon does not carry.\n",
  );

  let done = Facade::upgrade(&fixture.project(), &facade_ctx())
    .expect("a declined bucket file never halts the upgrade");

  let project = fixture.project();
  let root = project.st_dir().join("COMPLETED").join("ST0011");
  let names = |paths: Vec<&std::path::Path>| -> Vec<String> {
    let mut out: Vec<String> = paths
      .into_iter()
      .map(|p| {
        p.strip_prefix(&root)
          .unwrap_or(p)
          .to_string_lossy()
          .into_owned()
      })
      .collect();
    out.sort();
    out
  };
  assert_eq!(
    names(done.ingested.iter().map(|p| p.as_path()).collect()),
    vec!["kept.md".to_string()],
    "only the ordinary file is ingested"
  );
  assert_eq!(
    names(done.not_ingested.iter().map(|w| w.path.as_path()).collect()),
    vec!["bad?name.md".to_string(), "reference.md".to_string()],
    "each declined file is named: {:?}",
    done.not_ingested
  );
  assert!(
    done
      .not_ingested
      .iter()
      .all(|w| !w.reason.trim().is_empty()),
    "every declined file carries a reason: {:?}",
    done.not_ingested
  );
  assert!(
    done.prune_deferred.is_empty() && done.pruned.is_empty(),
    "a declined file is unheld, so the prune refuses rather than defers: deferred {:?}, pruned {:?}",
    done.prune_deferred,
    done.pruned
  );

  let facade = fixture.facade_on_disk();
  let held = facade
    .canon()
    .threads
    .iter()
    .find(|t| t.id == "ST0011")
    .expect("ST0011 is in canon");
  assert_eq!(
    held
      .attachments
      .iter()
      .find(|a| a.path == "reference.md")
      .and_then(|a| a.text.clone()),
    Some(canon_reference),
    "canon wins: the bucket's differing copy did not replace it"
  );
  assert!(
    held.attachments.iter().any(|a| a.path == "kept.md"),
    "the ingested file is in canon"
  );
}

/// **AN AUTHORED `info.md` BELOW A VIEW'S DEPTH IS CARRIED, AND AN UNNAMEABLE
/// FILE IS WITHHELD WITH ITS WAY OUT** (issue 0461, vc's ruling of 2026-09-19).
/// Lamplight's shape: ST0037's `WP/_superseded/<nn>/info.md` are authored files
/// at depth 4. `classify` called them attachments and `parse` refused the name,
/// so the ingest skipped them and the prune withheld them on every run with a
/// line that named no remedy. Beside it, a name that stays unaddressable (a
/// `?` cannot survive the URL) must be withheld with the rename-and-attach
/// remedy, and the thread's own `WP/01/info.md` view is not carried at all.
#[test]
fn a_deep_info_md_is_carried_and_an_unnameable_file_is_withheld_with_its_remedy() {
  let fixture = Fixture::new();
  fixture.write_thread(&sample_thread("ST0012"));
  let bucket = "intent/st/COMPLETED/ST0012";
  fixture.write_file(
    &format!("{bucket}/WP/_superseded/01/info.md"),
    "# Superseded WP 01\n\nphrase-deep-info-authored.\n",
  );
  fixture.write_file(
    &format!("{bucket}/bad?name.md"),
    "# bad\n\nUnaddressable.\n",
  );

  let done =
    Facade::upgrade(&fixture.project(), &facade_ctx()).expect("the upgrade runs over both files");
  let project = fixture.project();
  let root = project.st_dir().join("COMPLETED").join("ST0012");
  let ingested: Vec<String> = done
    .ingested
    .iter()
    .filter_map(|p| p.strip_prefix(&root).ok())
    .map(|p| p.to_string_lossy().into_owned())
    .collect();
  assert_eq!(
    ingested,
    vec!["WP/_superseded/01/info.md".to_string()],
    "the deep info.md is the author's and is ingested; not ingested: {:?}",
    done.not_ingested
  );

  let facade = fixture.facade_on_disk();
  let held = facade
    .canon()
    .threads
    .iter()
    .find(|t| t.id == "ST0012")
    .expect("ST0012 is in canon");
  assert!(
    held
      .attachments
      .iter()
      .any(|a| a.path == "WP/_superseded/01/info.md"
        && a
          .text
          .as_deref()
          .is_some_and(|t| t.contains("phrase-deep-info-authored"))),
    "canon carries the deep info.md with its own content: {:?}",
    held.attachments.iter().map(|a| &a.path).collect::<Vec<_>>()
  );

  let withheld = legacy::leftovers(&project, facade.canon()).withheld;
  let names: Vec<String> = withheld
    .iter()
    .filter_map(|w| w.path.strip_prefix(&root).ok())
    .map(|p| p.to_string_lossy().into_owned())
    .collect();
  assert_eq!(
    names,
    vec!["bad?name.md".to_string()],
    "only the unnameable file is withheld: {withheld:?}"
  );
  assert!(
    withheld[0]
      .reason
      .contains("rename the file, then `intent st attach ST0012 <new name> --from <file>`"),
    "the withheld line names the rename-and-attach remedy: {}",
    withheld[0].reason
  );
}
