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

//!
//! # TWO POPULATIONS, AND THE SECOND EXISTS BECAUSE THE FIRST CANNOT SEE THE
//! ROW'S OWN LIVE FALSIFIER
//!
//! `every_declared_twin_exists_and_agrees` walks rows that DECLARE `twin_of`.
//! Measured 2026-08-30, that population is **one row** -- `browse`, twinned to
//! `edit --browser` -- and it is filtered from `shipped_entries`, so a retired
//! row is outside it twice over.
//!
//! **`--help` / `help` is invisible to it on both counts, and that pair is
//! `AC-00.6`'s live falsifier**: `intent --help` answers rc=0 while `intent
//! help` refuses rc=2 as retired, in one binary, with the failing spelling the
//! one a person types first. `help` is retired, so it is not in
//! `shipped_entries`; and it declares no `twin_of`, so it would be filtered out
//! even if it were.
//!
//! **THE GENERAL FORM IS THE PART WORTH KEEPING: A DECLARATION-DRIVEN
//! INSTRUMENT'S POPULATION IS CHOSEN BY THE SAME JUDGEMENT THAT WOULD HAVE
//! PREVENTED THE DEFECT.** Nobody wrote `twin_of: "--help"` on the `help` row
//! for precisely the reason nobody noticed the disagreement -- they did not
//! think of `--help` as `help`'s twin. So a declared population can only hold
//! disagreements somebody already suspected, which is the set needing an
//! instrument least. **This is not the empty-set vacuity this estate greps
//! for** -- the set is non-empty and the guard against emptiness is already
//! below. It is narrower and worse: a population that excludes the defect by
//! construction.
//!
//! So `every_twin_the_surface_actually_has_agrees` DERIVES its population from
//! the built surface. At every node it asks whether a long flag's token is also
//! a command name this binary KNOWS AND REFUSES.
//!
//! **THIS FILE DOES NOT CLOSE `AC-00.6` AND MUST NOT BE READ AS DOING SO.** The
//! row closes when `intent help` answers. The derived arm measures whether the
//! estate's account of the disagreement is honest and current: a NEW
//! disagreement goes red, and so does a FIXED one still carried as expected.

use intent_cli::dispatch;
use intent_cli::spine;

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

/// **THE DISAGREEMENTS THIS BUILD IS KNOWN TO CARRY, EACH WITH THE DECISION IT
/// IS WAITING ON.**
///
/// A pair reaches this roster only when the binary KNOWS the subcommand
/// spelling and REFUSES it while the flag spelling answers. A flag whose token
/// simply names nothing is not a member: `--format` with no `format`
/// subcommand is one spelling of one capability, not two spellings disagreeing
/// about whether it exists.
///
/// **THE ROSTER IS TWO-SIDED BY CONSTRUCTION, AND THAT IS THE ONLY THING THAT
/// MAKES IT A ROSTER RATHER THAN A COMMENT WITH A TEST'S SYNTAX.** An entry
/// whose disagreement has been FIXED goes red here exactly as loudly as an
/// undeclared one. Otherwise an exception outlives the decision that granted
/// it, which is the failure this estate has already recorded once against a
/// guard exempting itself in prose.
/// **EMPTY IS A MEASURED STATE HERE, NOT AN UNWRITTEN ROSTER.**
///
/// It held one entry -- `help` -- from the day this arm was built until
/// 2026-08-30, when `intent help` began answering rc=0 byte-identically to
/// `intent --help` (`a9f03ab7`, hv's root-only ruling, ST0058 `AC-00.6`,
/// intent#0086 CLOSED). **The entry named its own discharge condition in its
/// last sentence** -- *the row closes when `intent help` ANSWERS, not when its
/// refusal is better worded* -- and that is exactly what happened, so it was
/// deleted rather than reworded.
///
/// **THE ARM THAT CAUGHT IT INVERTS THE USUAL POLARITY AND THAT IS THE POINT.**
/// A roster of excused defects normally fails silently in one direction: the
/// exception outlives the decision that granted it and goes on excusing
/// something that is gone, while reading like diligence. This one goes RED when
/// a listed pair stops disagreeing, so an entry cannot outlive its grant
/// quietly. Landing the fix turned a green test red, and the red was the
/// notification.
///
/// A new entry needs the decision it waits on, in the same shape: what must
/// become true, not why the disagreement is tolerable.
const EXPECTED_DISAGREEMENTS: [(&str, &str); 0] = [];

/// Every twin pair the BUILT SURFACE actually has, rather than every twin pair
/// a row remembered to declare.
///
/// See this file's header for why the declared population cannot reach the one
/// disagreement `AC-00.6` was re-driven on.
#[test]
fn every_twin_the_surface_actually_has_agrees() {
  use std::collections::BTreeSet;

  let table = dispatch::table();

  // **`build()` IS LOAD-BEARING AND NOT TIDINESS.** clap synthesises `--help`
  // and `--version` when a command is finalised, so an unfinalised surface has
  // no `help` flag at all -- and this arm would then measure a population with
  // its own subject removed, which is the exact defect the header describes
  // one level up. Positive control: the agreeing set below must be non-empty,
  // and its only member is the synthesised `--version` / `version` pair.
  let mut cli = spine::build(&table);
  cli.build();

  // Every spelling this build knows and refuses, keyed the way a caller types
  // it. `retired_and_unreachable` is the one definition of "retired" -- it
  // filters the declared roster by what the BUILT surface actually answers, so
  // a name reclaimed by a new program (`organize`) is correctly absent here.
  let refused: BTreeSet<String> = spine::retired_and_unreachable(&table)
    .into_iter()
    .flat_map(|(_, gone)| gone.into_iter().map(|spelling| spelling.join(" ")))
    .collect();

  let mut agreeing: Vec<String> = Vec::new();
  let mut disagreeing: Vec<(String, String)> = Vec::new();

  let mut queue = vec![(&cli, Vec::<String>::new())];
  while let Some((node, path)) = queue.pop() {
    for arg in node.get_arguments() {
      let Some(long) = arg.get_long() else { continue };
      let typed = path
        .iter()
        .cloned()
        .chain(std::iter::once(long.to_string()))
        .collect::<Vec<String>>()
        .join(" ");
      if node.find_subcommand(long).is_some() {
        agreeing.push(typed);
      } else if refused.contains(&typed) {
        let flag_spelling = match path.is_empty() {
          true => format!("--{long}"),
          false => format!("{} --{long}", path.join(" ")),
        };
        disagreeing.push((long.to_string(), flag_spelling));
      }
    }
    for sub in node.get_subcommands() {
      let mut next = path.clone();
      next.push(sub.get_name().to_string());
      queue.push((sub, next));
    }
  }

  assert!(
    !agreeing.is_empty(),
    "no token in the built surface is spelled BOTH as a flag and as a subcommand, so this arm \
     measured an empty set and would pass against any binary. `--version` / `version` is the \
     pair that must be here -- if clap stopped synthesising it, or `build()` above stopped \
     being called, this arm's subject is gone and its green means nothing"
  );

  let found: BTreeSet<&str> = disagreeing
    .iter()
    .map(|(token, _)| token.as_str())
    .collect();
  let expected: BTreeSet<&str> = EXPECTED_DISAGREEMENTS.iter().map(|(t, _)| *t).collect();

  let undeclared: Vec<&&str> = found.difference(&expected).collect();
  assert!(
    undeclared.is_empty(),
    "a capability answers by its flag spelling and is REFUSED by its subcommand twin, and \
     nothing in this estate says so: {undeclared:?}. This is `INV-09` / `ST0058 AC-00.6` -- \
     the failing spelling is the one a person types first. Either fix the disagreement or add \
     it to `EXPECTED_DISAGREEMENTS` with the decision it waits on. Full set found: {disagreeing:?}"
  );

  let stale: Vec<&&str> = expected.difference(&found).collect();
  assert!(
    stale.is_empty(),
    "`EXPECTED_DISAGREEMENTS` names {stale:?}, and the built surface no longer disagrees about \
     it. **This is the direction a roster usually fails silently in**: the exception outlives \
     the decision that granted it and goes on excusing a defect that is gone, while reading \
     like diligence. Delete the entry -- and if it was `help`, `ST0058 AC-00.6` may now be \
     satisfiable, which is the whole reason this arm inverts"
  );
}
