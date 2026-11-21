//! **WHAT A VERB THAT SETS REALISATION SAYS FOLLOWS THE FORMAT** (todo 78, on
//! ST0096 WP-01): 3.4.0's manifest keeps its sentences byte for byte, and an
//! upgraded project says what its records carry, with no manifest phrase
//! dropped into a sentence written for a file.

use intentsvcs::canon_format::CanonFormat;
use intentsvcs::facade::Declared;
use intentsvcs::intentfiles::{RealisationWords, RealisedCount, named, words};

use crate::common::Fixture;

const URL: &str = "intent:///threads/ST0001";
/// The files `st hydrate` reports on disk, and those it wrote.
const ON_DISK: usize = 3;
const WRITTEN: usize = 1;
/// The entries a manifest declares in a report.
const DECLARES: usize = 1;

/// The counts an upgraded project's report gives.
const COUNT: RealisedCount = RealisedCount {
  threads: 2,
  issues: 1,
};

/// A report of `organize --default` in 3.4.0's format.
fn on_manifest(wrote: bool, was_present: bool) -> Declared {
  Declared {
    path: std::path::PathBuf::new(),
    wrote,
    was_present,
    declares: DECLARES,
    realised: None,
  }
}

/// A report of `organize --default` in an upgraded project.
fn on_records(wrote: bool) -> Declared {
  Declared {
    path: std::path::PathBuf::new(),
    wrote,
    was_present: true,
    declares: COUNT.total(),
    realised: Some(COUNT),
  }
}

/// 3.4.0's format: every sentence is the one the verbs printed before the
/// words had a home, with the manifest's path where it stood.
#[test]
fn the_manifest_format_keeps_its_sentences_byte_for_byte() {
  let fx = Fixture::ordinal();
  let project = fx.project();
  let manifest = named(&project, CanonFormat::V3, true);
  let w = words(&project, CanonFormat::V3);
  let entry = "entry";

  assert_eq!(
    w.hydrated(URL, ON_DISK, WRITTEN),
    format!(
      "ok: {URL} hydrated -- listed in {manifest}, {ON_DISK} file(s) on disk, {WRITTEN} written by this run"
    )
  );
  assert_eq!(
    w.already_dehydrated(URL),
    format!(
      "ok: {URL} was already dehydrated -- not listed in {manifest}, nothing realised to remove"
    )
  );
  assert_eq!(w.unrealised(), format!("  delisted: {manifest}"));
  assert_eq!(
    w.unchanged(),
    format!("  unchanged: {manifest} -- the id was not listed, and the files were present anyway")
  );
  assert_eq!(
    w.default_needs_a_person(),
    format!(
      "error: `--default --force` regenerates `{manifest}` from status and needs a person to confirm it\n  remedy: run it in a terminal. There is deliberately no flag or environment variable that answers for you -- bare `intent organize --default` writes the declaration when there is none and changes nothing when there is"
    )
  );
  assert_eq!(
    w.default_prompt(),
    [
      format!("about to regenerate {manifest} from thread status, and then APPLY it."),
      "this is the destructive arm: it realises every thread the regenerated file declares, and it REMOVES the files of every realised thread it does not.".to_string(),
      "a thread whose preconditions are unmet keeps every one of its files, and the refusal names the thread and the precondition.".to_string(),
      "`intent organize --default` without --force writes the declaration and removes nothing.".to_string(),
    ]
  );
  assert_eq!(
    w.default_not_confirmed(),
    format!("error: not confirmed, so `{manifest}` is unchanged")
  );
  assert_eq!(
    w.default_outcome(&on_manifest(false, true), false),
    [
      format!("ok: {manifest} is already present and declares {DECLARES} {entry}"),
      "    nothing was written. `intent organize --default --force` regenerates it from status, after confirming on a terminal.".to_string(),
    ]
  );
  assert_eq!(
    w.default_outcome(&on_manifest(true, true), true),
    [format!(
      "ok: {manifest} regenerated from status -- now declares {DECLARES} {entry}"
    )]
  );
  assert_eq!(
    w.default_outcome(&on_manifest(true, false), true),
    [format!(
      "ok: {manifest} written -- declares {DECLARES} {entry} (every WIP thread, and nothing else)"
    )]
  );
  assert_eq!(
    w.default_left_alone(),
    "    no thread file was created or removed. `intent organize` previews what this declaration implies."
  );
  assert_eq!(
    w.default_applying(),
    "    applying the regenerated declaration in the same run."
  );
}

/// An upgraded project: each sentence names the record, and `--default` counts
/// threads and issues apart (cc's meaning, 2026-11-20).
#[test]
fn an_upgraded_project_says_what_its_records_carry() {
  let fx = Fixture::new();
  let w = words(&fx.project(), CanonFormat::V4);
  let realised = format!(
    "{} thread(s) and {} issue(s) realised",
    COUNT.threads, COUNT.issues
  );

  assert_eq!(
    w.hydrated(URL, ON_DISK, WRITTEN),
    format!(
      "ok: {URL} hydrated -- realised on its canon record, {ON_DISK} file(s) on disk, {WRITTEN} written by this run"
    )
  );
  assert_eq!(
    w.already_dehydrated(URL),
    format!(
      "ok: {URL} was already dehydrated -- not realised on its canon record, nothing realised to remove"
    )
  );
  assert_eq!(w.unrealised(), "  unrealised: its canon record");
  assert_eq!(
    w.unchanged(),
    "  unchanged: its canon record -- the id was not realised, and the files were present anyway"
  );
  assert!(
    w.default_needs_a_person().starts_with(
      "error: `--default --force` resets every canon record's realisation from status"
    ),
    "{}",
    w.default_needs_a_person()
  );
  assert_eq!(
    w.default_prompt()[0],
    "about to reset every canon record's realisation from thread status, and then APPLY it."
  );
  assert_eq!(
    w.default_not_confirmed(),
    "error: not confirmed, so no canon record's realisation changed"
  );
  assert_eq!(
    w.default_outcome(&on_records(false), false)[0],
    format!("ok: realisation is already recorded on the canon records -- {realised}")
  );
  assert_eq!(
    w.default_outcome(&on_records(true), true),
    [format!(
      "ok: realisation reset from status -- now {realised} (every WIP thread and every open issue)"
    )]
  );
  assert_eq!(
    w.default_outcome(&on_records(false), true),
    [format!(
      "ok: realisation reset from status -- every record already matched it, so none changed: {realised} (every WIP thread and every open issue)"
    )]
  );
  assert_eq!(
    w.default_applying(),
    "    applying the reset realisation in the same run."
  );

  // No sentence written for a file survives into the records' words.
  let every = [
    w.hydrated(URL, ON_DISK, WRITTEN),
    w.already_dehydrated(URL),
    w.unrealised(),
    w.unchanged(),
    w.default_needs_a_person(),
    w.default_not_confirmed(),
    w.default_left_alone().to_string(),
    w.default_applying().to_string(),
  ]
  .into_iter()
  .chain(w.default_prompt())
  .chain(w.default_outcome(&on_records(false), false))
  .chain(w.default_outcome(&on_records(true), true))
  .chain(w.default_outcome(&on_records(false), true));
  for line in every {
    assert!(
      !line.contains("listed") && !line.contains("regenerat") && !line.contains("entr"),
      "a manifest's word in an upgraded project's sentence: {line}"
    );
  }
}

/// The two words a close's note takes from the format: 3.4.0's manifest
/// unlists, and an upgraded record is unrealised. The note they make is read
/// whole, as the verb prints it, in intent-cli's
/// a_close_names_what_the_next_organize_removes.rs.
#[test]
fn the_closing_words_are_each_formats_own() {
  assert_eq!(
    RealisationWords::closing_unrealises(CanonFormat::V3),
    "unlists"
  );
  assert_eq!(
    RealisationWords::closing_kept(CanonFormat::V3),
    "closes without unlisting"
  );
  assert_eq!(
    RealisationWords::closing_unrealises(CanonFormat::V4),
    "unrealises"
  );
  assert_eq!(
    RealisationWords::closing_kept(CanonFormat::V4),
    "closes and leaves it realised"
  );
}

/// The close note's machine form carries the paths and nothing of the
/// format: the format is how the note is WORDED, and a reader of the JSON
/// reads no words (todo 78, critic-rust on 630f7d0f8).
#[test]
fn the_close_notes_json_is_its_kind_and_paths_in_either_format() {
  for format in [CanonFormat::V3, CanonFormat::V4] {
    let json =
      intentsvcs::facade::notes_json(&[intentsvcs::facade::Note::DehydratesOnNextOrganize {
        paths: vec!["intent/st/ST0001/info.md".to_string()],
        format,
      }]);
    let note = json
      .as_array()
      .and_then(|notes| notes.first())
      .and_then(|note| note.as_object())
      .unwrap_or_else(|| panic!("one note object in {json}"));
    let keys: std::collections::BTreeSet<&str> = note.keys().map(String::as_str).collect();
    assert_eq!(
      keys,
      ["kind", "paths"].into_iter().collect(),
      "{format:?}: {json}"
    );
  }
}
