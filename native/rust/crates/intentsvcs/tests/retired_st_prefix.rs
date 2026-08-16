//! **`st_prefix` retires, and the retirement is only safe because the migrator
//! says so** (issue 0040, hv ruling 2026-08-16).
//!
//! Deleting the field was the easy half and not the decision. `st_prefix`
//! appears in no ST0056 spec -- `data-model.md`'s project-config table lists
//! six fields and never included it -- so the design had already dropped the
//! knob and the type was behind the design rather than ahead of it.
//!
//! **The half that needed building is what happens to a project that SET it.**
//! v3 recognises a steel thread by `model::is_thread_id`, which is now the one
//! authority on the form. A project whose threads are named `XX0001` has none
//! of them recognised -- `thread_dirs` yields nothing, the scan finds nothing,
//! and every count reconciles perfectly against zero. **A migration that
//! reports a clean conversion of an estate it could not see is the
//! answers-confidently-from-partial-evidence bug with the evidence set to
//! zero**, and it is the exact failure the whole thread exists to end.
//!
//! Costs nothing today -- all sixteen fleet projects use the default, so this
//! is silent on every one of them. It exists for the reader outside the fleet,
//! who is the only person the retirement could ever have hurt.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{THREAD_PREFIX, is_thread_id, thread_id, thread_seq};

/// A v2 estate whose config declares `st_prefix` as `prefix`, carrying one
/// completed thread named on that prefix.
fn v2_estate(fixture: &Fixture, prefix: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"st_prefix\":\"{prefix}\",\"languages\":[\"rust\"]}}\n"
    ),
  );
  fixture.write_file(
    &format!("intent/st/{prefix}0001/info.md"),
    "---\nverblock: \"16 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: Completed\nslug: a-slug\ncreated: 20260816\ncompleted: 20260816\n---\n\n# A thread\n\n## Objective\n\nShip it.\n",
  );
}

/// **The harm is real before the finding is worth anything.** Asserted first,
/// because a test that only checked for the message would pass just as well if
/// a non-default prefix were harmless -- and then the finding would be noise.
#[test]
fn a_project_on_another_prefix_has_none_of_its_threads_recognised() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "XX");
  let scan = legacy::scan(&fixture.project()).expect("scan");

  assert!(
    scan.threads.is_empty(),
    "the premise of this whole finding is that v3 cannot see these threads; if it can, the finding is wrong rather than merely unnecessary: {:?}",
    scan.threads.iter().map(|t| &t.id).collect::<Vec<_>>()
  );
}

/// **And because it cannot see them, it must REFUSE rather than report a
/// clean, empty conversion.**
#[test]
fn a_retired_prefix_blocks_the_migration_and_names_the_field() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "XX");
  let scan = legacy::scan(&fixture.project()).expect("scan");

  let found: Vec<_> = scan
    .residue
    .iter()
    .filter(|f| f.class == FindingClass::RetiredSetting)
    .collect();
  assert_eq!(
    found.len(),
    1,
    "a project declaring a retired prefix must block, exactly once: residue={:?} carried={:?}",
    scan.residue,
    scan.carried
  );

  let detail = &found[0].detail;
  assert!(
    detail.contains("st_prefix") && detail.contains("XX"),
    "the finding names the field AND the value the operator actually wrote, because that is what they have to go and find: {detail}"
  );
  assert!(
    detail.contains(THREAD_PREFIX),
    "and what v3 fixed it to, or there is nothing to act on: {detail}"
  );
  assert!(
    found[0].class.remedy().contains("will not see them"),
    "the remedy must say the migration would MISS the artefacts -- an operator who reads this as a cosmetic rename will migrate anyway: {}",
    found[0].class.remedy()
  );

  assert!(
    scan.carried.is_empty(),
    "this is not a carry: carrying it would convert the project and leave the threads behind: {:?}",
    scan.carried
  );
}

/// **Declaring the default is not a defect.** Sixteen fleet projects declare
/// `st_prefix: ST`; nothing is lost and nothing changes, so a finding here
/// would be noise on every project that did nothing wrong -- and a migrator
/// that cries wolf on the whole fleet is one nobody reads.
#[test]
fn declaring_the_value_v3_fixed_it_to_is_silent() {
  let fixture = Fixture::new();
  v2_estate(&fixture, THREAD_PREFIX);
  let scan = legacy::scan(&fixture.project()).expect("scan");

  assert!(
    !scan
      .residue
      .iter()
      .chain(scan.carried.iter())
      .any(|f| f.class == FindingClass::RetiredSetting),
    "declaring the default must be silent: {:?}",
    scan.residue
  );
  assert_eq!(
    scan.threads.len(),
    1,
    "and the estate converts normally: {:?}",
    scan.residue
  );
}

/// **The field is retired from the TYPE and kept in the FILE.**
///
/// Dropping it on the first rewrite would be a silent data change of exactly
/// the kind the finding above exists to prevent -- one layer down, in the
/// artefact rather than in the estate.
#[test]
fn the_retired_declaration_survives_in_the_config_it_was_written_in() {
  let fixture = Fixture::new();
  v2_estate(&fixture, "XX");
  let config = fixture.project().config().clone();

  assert_eq!(
    config
      .extra
      .get(intentsvcs::project::RETIRED_ST_PREFIX_KEY)
      .and_then(|v| v.as_str()),
    Some("XX"),
    "the declaration must be carried in `extra`, verbatim, so a rewrite cannot drop it"
  );
  let round_tripped = serde_json::to_string(&config).expect("serialise the config");
  assert!(
    round_tripped.contains("\"st_prefix\":\"XX\""),
    "and it must survive a serialise: {round_tripped}"
  );
}

/// **One encoding of the id form, not four.**
///
/// `6` was `"ST".len() + 4` written a second way, in a file that would not move
/// if the first one did. These three functions are now the only place the form
/// is decided, and the round trip is what holds them to each other.
#[test]
fn the_id_form_has_exactly_one_authority() {
  for seq in [1u32, 9, 10, 56, 999, 1000, 9999] {
    let id = thread_id(seq);
    assert_eq!(id.len(), THREAD_PREFIX.len() + 4, "fixed width: {id}");
    assert!(is_thread_id(&id), "its own output must be recognised: {id}");
    assert_eq!(thread_seq(&id), Some(seq), "and read back: {id}");
  }

  // The forms the old hardcoded `len() == 6` accepted or rejected for reasons
  // it could not state. Each is here because it is a way through, not to pad
  // the count.
  for bad in [
    "ST001",    // too short
    "ST00001",  // too long
    "XX0001",   // another prefix -- the whole subject of the retirement
    "st0001",   // case
    "ST00O1",   // a letter O in the digits
    "ST",       // prefix alone
    "",         // empty
    "ST 001",   // a space
    "0001",     // digits alone
    "STST0001", // the prefix twice
  ] {
    assert!(!is_thread_id(bad), "must not be a thread id: {bad:?}");
    assert_eq!(thread_seq(bad), None, "and yields no sequence: {bad:?}");
  }
}
