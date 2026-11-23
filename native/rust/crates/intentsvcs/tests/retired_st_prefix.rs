//! **`st_prefix` is retired from the config type and kept in the file** (issue
//! 0040, hv ruling 2026-08-16), and the thread id has one authority on its
//! form.
//!
//! v3 recognises a steel thread by `id::ThreadId::parse`, which is the one
//! authority on the form, so the knob is not read. A config that still carries
//! it keeps the declaration byte for byte rather than losing it on the first
//! rewrite.

use crate::common::Fixture;
use intentsvcs::id::{THREAD_DIGITS, THREAD_PREFIX, ThreadId};

/// The retired key, by the name it has in `config.json`.
const ST_PREFIX: &str = "st_prefix";

/// **The field is retired from the TYPE and kept in the FILE.**
///
/// Dropping it on the first rewrite would be a silent data change, in the
/// artefact rather than in the estate.
///
/// AT-14.4 (ST0095 AC-14.2): a config that still carries `st_prefix` loads, and
/// keeps it.
#[test]
fn the_retired_declaration_survives_in_the_config_it_was_written_in() {
  let fixture = Fixture::new();
  let declared = "XX";
  let mut config: serde_json::Value =
    serde_json::from_str(&fixture.read("intent/.config/config.json")).expect("the fixture config");
  config[ST_PREFIX] = serde_json::Value::String(declared.to_string());
  fixture.write_file(
    "intent/.config/config.json",
    &serde_json::to_string(&config).expect("serialise the seeded config"),
  );
  let config = fixture.project().config().clone();

  assert_eq!(
    config.extra.get(ST_PREFIX).and_then(|v| v.as_str()),
    Some(declared),
    "the declaration must be carried in `extra`, verbatim, so a rewrite cannot drop it"
  );
  let round_tripped = serde_json::to_string(&config).expect("serialise the config");
  assert!(
    round_tripped.contains(&format!("\"{ST_PREFIX}\":\"{declared}\"")),
    "and it must survive a serialise: {round_tripped}"
  );
}

/// **One encoding of the id form, not four.**
///
/// `6` was `"ST".len() + 4` written a second way, in a file that would not move
/// if the first one did. `ThreadId` is now the only place the form is decided:
/// `from_ordinal` makes the stored form and `parse` reads it, and the round
/// trip is what holds them to each other.
#[test]
fn the_id_form_has_exactly_one_authority() {
  for seq in [1u32, 9, 10, 56, 999, 1000, 9999] {
    let id = ThreadId::from_ordinal(seq).expect("an ordinal that fits the form");
    let stored = id.stored();
    assert_eq!(
      stored.len(),
      THREAD_PREFIX.len() + THREAD_DIGITS,
      "fixed width: {stored}"
    );
    assert_eq!(
      ThreadId::parse(&stored),
      Ok(id),
      "its own output must be recognised, and read back as the same ordinal: {stored}"
    );
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
    assert!(
      ThreadId::parse(bad).is_err(),
      "must not be a thread id, and so yields no sequence: {bad:?}"
    );
  }
}
