//! INV-09: **every spelling of one capability agrees about whether it exists.**
//!
//! `ST0058 AC-00.6` states this narrowly -- _a flag and its subcommand twin
//! agree about whether the capability exists_ -- after `intent3 --version`
//! returned rc=0 while `intent3 version` returned the unwired marker. **It
//! refuses DISAGREEMENT, not duplication**, and two spellings that agree are
//! precisely what it asks for. An earlier reading in `tui-design.md` §9 turned
//! _must agree_ into _must not both exist_ and deleted `intent browse` on the
//! strength of it.
//!
//! # WHY THIS FILE EXISTS RATHER THAN A NOTE ON THE ROW
//!
//! `twin_of` is classified `declaration` in `key_classes`, whose contract is
//! blunt: **code must read this key -- if no type deserializes it, the
//! behaviour it describes is either absent or hand-written to match by
//! coincidence, and the coincidence never surfaces, because agreement looks
//! exactly like correctness.** A relation the register states and nothing
//! walks is the stale-canon shape the register's own orphan-invariant refusal
//! already refuses one level up.
//!
//! # WHAT THIS FILE CANNOT SEE, STATED SO THE GREEN IS NOT READ TOO WIDELY
//!
//! INV-09's other half is that **two argument shapes of one row are also two
//! spellings**, and the live witness is `fc`: `intent fc <thread> AC-nn.n`
//! reaches the arm, `intent fc <thread> AT-nn.n` answers _`fc` is a known
//! command that is not implemented yet_ -- **for a verb this build provides.**
//! That defect lives BELOW a row, and this file is keyed on rows, so it cannot
//! reach it. cc found the same limit in their gate arm and in the AC-08.2
//! dual-path harness, and recorded it as a declared boundary rather than
//! widening a sentence. **The unit a defect lives in can be smaller than the
//! unit an instrument is keyed on**, and saying so is worth more than a green.

use intent_cli::dispatch;

/// Every `twin_of` names a real capability, and the two spellings agree on
/// everything a caller could observe about whether it exists.
#[test]
fn every_declared_twin_exists_and_agrees() {
  let table = dispatch::table();
  let entries = dispatch::shipped_entries(&table);

  let twinned: Vec<&dispatch::Entry> = entries
    .iter()
    .copied()
    .filter(|e| !e.twin_of.is_empty())
    .collect();

  assert!(
    !twinned.is_empty(),
    "no row declares `twin_of`, so this file asserts nothing -- if the last twin was retired, \
     retire INV-09 with it rather than leaving a green that measures an empty set"
  );

  for entry in &twinned {
    // The twin is spelled `<path> <flag>`; the row it must agree with is the
    // path half. Read from the declaration rather than assumed, because a twin
    // naming a row that does not exist is the first thing that can go wrong.
    let (partner_path, flag) = entry
      .twin_of
      .split_once(" --")
      .map(|(p, f)| (p, Some(format!("--{f}"))))
      .unwrap_or((entry.twin_of.as_str(), None));

    let partner = entries
      .iter()
      .find(|e| e.path == partner_path)
      .unwrap_or_else(|| {
        panic!(
          "`{}` declares `twin_of: {}`, and no shipped row is spelled `{partner_path}`. \
           A twin present by one spelling and absent by the other is exactly what INV-09 \
           refuses",
          entry.path, entry.twin_of
        )
      });

    if let Some(flag) = &flag {
      assert!(
        partner
          .flags
          .iter()
          .any(|f| f.spellings.iter().any(|s| s == flag)),
        "`{}` declares its twin as `{}`, but `{partner_path}` carries no `{flag}` flag. \
         The subcommand spelling would exist and the flag spelling would not, which is the \
         `--version` / `version` disagreement one verb over",
        entry.path,
        entry.twin_of
      );
    }

    // **The agreement is about what a CALLER can observe**, not about the rows
    // being identical -- they are deliberately not, or there would be no
    // reason for two spellings.
    assert_eq!(
      entry.exposed_on_mcp, partner.exposed_on_mcp,
      "`{}` and `{partner_path}` disagree about MCP exposure. One capability reachable by a \
       machine through one spelling and withheld through the other is the same defect the \
       criterion names, wearing a different hat",
      entry.path
    );
    assert_eq!(
      entry.read_or_mutate, partner.read_or_mutate,
      "`{}` and `{partner_path}` disagree about whether the capability reads or mutates",
      entry.path
    );
    assert_eq!(
      entry.recoverability, partner.recoverability,
      "`{}` and `{partner_path}` disagree about recoverability -- and the MCP withhold list \
       is DERIVED from that field, so a disagreement here silently becomes a disagreement \
       about exposure",
      entry.path
    );
  }
}

/// **A flag declaring `arity` uses the register's own vocabulary, not a second
/// one.**
///
/// `arity` was extended from `args` to `flags` rather than minted, so the
/// value set must stay one set. A flag inventing `optional` or `?` would be
/// the two-vocabularies-for-one-concept shape the extension exists to avoid.
#[test]
fn flag_arity_speaks_the_registers_own_vocabulary() {
  let table = dispatch::table();
  const VOCABULARY: [&str; 4] = ["1", "0..1", "0..n", "1..n"];

  let declared: Vec<(String, String, String)> = dispatch::shipped_entries(&table)
    .iter()
    .flat_map(|e| {
      e.flags.iter().filter(|f| !f.arity.is_empty()).map(|f| {
        (
          e.path.clone(),
          f.spellings.first().cloned().unwrap_or_default(),
          f.arity.clone(),
        )
      })
    })
    .collect();

  assert!(
    !declared.is_empty(),
    "no flag declares `arity`, so this file measures nothing -- the field was added for \
     `--editor[=program]` and a green over an empty set would not say it arrived"
  );

  for (path, spelling, arity) in &declared {
    assert!(
      VOCABULARY.contains(&arity.as_str()),
      "`{path} {spelling}` declares arity `{arity}`, which is outside the vocabulary the 125 \
       args already use ({}). One concept, one value set",
      VOCABULARY.join(", ")
    );
  }
}
