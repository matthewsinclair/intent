//! `AT-06.13` / `AC-06.13`: **every subcommand the built binary offers
//! resolves to a path the dispatch table declares.**
//!
//! **THE MIRROR OF `AC-06.8`, AND A SEPARATE INSTRUMENT BECAUSE IT IS A
//! SEPARATE POPULATION.** Checking TABLE -> SURFACE walks the table and asks
//! the binary about each entry; checking SURFACE -> TABLE must ENUMERATE THE
//! BINARY. Different walk, different denominator, different failure mode. **A
//! verb present in the binary and absent from the table is outside the
//! population of every existing check BY CONSTRUCTION** -- which is why
//! `245dcdbe` could add an undeclared subcommand to nine shipped families with
//! both suites, `cargo check --workspace --all-targets` and rustfmt all green
//! (issue `0217`).
//!
//! # The artefact, and what a green here does NOT say
//!
//! `env!("CARGO_BIN_EXE_intent")`: the binary cargo built for THIS invocation,
//! never the shared release pair. A test bound to the pair describes whatever
//! was last built by anyone -- its verdict changes with someone else's build,
//! and it cannot be run at all while `native/rust` is dirty, which is the
//! normal state while somebody is changing the very surface this checks.
//!
//! **SO THIS ROW SAYS NOTHING ABOUT WHAT IS DELIVERED.** A green is a claim
//! about the TREE. That gap is real, deliberate, and belongs to
//! `self-provenance`, which owns the delivered-artefact question. Named here
//! so nobody reads this row as covering a surface it never looked at.
//!
//! # Reach, stated at mint rather than at the first surprise
//!
//! The walk sees what `--help` OFFERS, at the depth it walks, and nothing
//! else.
//!
//! - **It does not judge whether a declared path is CORRECT**, only that the
//!   spelling is declared somewhere.
//! - **It does not see flags.** That is `AC-06.8`'s population.
//! - **IT CANNOT SEE A HIDDEN SUBCOMMAND OR A HIDDEN ALIAS.** `organize`
//!   carries the hidden alias `organise`, accepted and never printed, and
//!   nothing in this file can observe it. An undeclared verb added with
//!   `.hide(true)` is invisible here. The honest statement is that this
//!   instrument covers the surface an operator can DISCOVER; the accepted-but-
//!   unprinted surface is the other direction's to hold.
//!
//! # Aliases are IN the population, and that is a ruling
//!
//! The row requires an explicit answer on aliases rather than an inherited
//! one. Two were available: exclude them and report the excluded count, or
//! include them and resolve each against the entry it aliases. **This file
//! includes them**, because inclusion is strictly the stronger of the two at
//! no extra cost -- an alias the binary offers and the table does not declare
//! reds here, where under exclusion it would be invisible. Exclusion answers
//! *is `rm` a path* (it is not); inclusion answers *is `rm` DECLARED* (it is,
//! by `lang remove`), and the second is the question `AC-06.13` asks.
//!
//! **The first-run trap this avoids is real and is named in the row:** a naive
//! enumeration counting `rm` as an undeclared path cries wolf on run one, and
//! a check that cries wolf on run one is one nobody runs twice.
//!
//! # Why the controls are the work, and the comparison is not
//!
//! **THE CENSUS IS GREEN TODAY: 175 spellings offered, 0 unresolved.** So this
//! file buys nothing at all unless it can be shown to SEE its subject -- a
//! guard that cannot be driven to both verdicts is decoration, and an
//! instrument whose care points at the wrong axis passes a non-empty wrong
//! population and renders as coverage.
//!
//! **THE PROTOTYPE OF THIS FILE WAS WRONG THREE TIMES IN ONE EVENING, EACH
//! TIME RETURNING A COHERENT ANSWER RATHER THAN AN ERROR** (cc, 2026-09-09):
//!
//! 1. A `families[].entries[]` walk missed the top-level `new_surface[]` list
//!    and reported **17 shipped commands as undeclared** -- `backup`, `browse`,
//!    `edit`, `events`, `export`, `fc`, `graphql`, `help`, `ingest`, `mcp`,
//!    `organize`, `schema`, `search`, `surface`, `sync` among them. Every one
//!    is real, shipped and declared. It read as a five-alarm finding.
//! 2. The pattern `\[aliases?: ` can never match `[alias: ` -- the `?` binds to
//!    the `e`, not to the word -- so the census reported **0 aliases on a
//!    surface with 4**, which is indistinguishable from a correct census of a
//!    surface with none.
//! 3. Both looked like results.
//!
//! Hence: every source of the declared population is asserted non-empty
//! separately (in [`common::declared_spellings`]), the walk's depth and size
//! are pinned, the alias parse is pinned to a non-zero count, and the
//! comparison itself is a pure function driven to BOTH verdicts on planted
//! input.
//!
//! # The arm this file does NOT have, and why its absence is the right answer
//!
//! **THERE IS NO TWO-SIDED CHECK THAT THE SYNTHETIC `help` DERIVATION AGREES
//! WITH THE SPINE, BECAUSE THERE CANNOT BE ONE.** An earlier draft asserted
//! `families_with_synthetic_help(&table) == what the binary offers`, which
//! reads like the strongest arm here and is worth nothing: the spine ADDS the
//! verb through that same function, so both sides move together. **Driven
//! rather than reasoned -- the predicate was mutated from *at least one verb*
//! to *more than one verb*, the surface changed, and all six arms stayed
//! green.** That is `restart.md`'s *a test that imports the value it asserts
//! has stopped testing*, built in by the same commit that single-homed the
//! rule.
//!
//! **THE SINGLE HOME IS STILL THE RIGHT TRADE, AND IT IS WHY THE ARM IS GONE
//! RATHER THAN REPAIRED.** Repairing it means a second implementation of the
//! predicate living in this file to disagree with the first -- the second home
//! the refactor existed to remove. *Make the bad state unrepresentable rather
//! than checked for*: the spine and the declaration cannot disagree about
//! WHICH families get `help`, so no test should claim to be watching for it.
//! What remains falsifiable is that the derivation reaches the population at
//! all, and that is the arm that survives.

use std::collections::BTreeSet;
use std::process::Command;

use crate::common;

/// How the binary offered a spelling: as a subcommand's own name, or as an
/// alias printed beside it.
///
/// Kept apart from the resolution question deliberately -- the census must be
/// able to report how many aliases it RECOGNISED, because a parse that
/// silently recognises none looks exactly like a surface that has none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Spelling {
  Name,
  Alias,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Offering {
  path: Vec<String>,
  spelling: Spelling,
}

/// Run `intent <path> --help` and hand back its stdout.
///
/// `HOME` and the working directory are both fixtured. `--help` reads neither,
/// so this is not a fix for a live defect -- it is making the instrument
/// structurally incapable of joining the class of tests that spawn the binary
/// with no `current_dir` and can migrate the operator's real store.
fn help(path: &[String]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(path)
    .arg("--help")
    .env("HOME", testkit::fixture_home())
    .current_dir(testkit::fixture_home())
    .output()
    .expect("the cargo-built binary runs");
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The `Commands:` block of one help page, as (name, aliases).
///
/// **A COMMAND LINE IS EXACTLY TWO SPACES THEN NON-SPACE; ANYTHING DEEPER IS A
/// WRAPPED DESCRIPTION.** clap wraps long `about` text to the terminal width,
/// so which lines wrap is an environment property rather than a fact about the
/// surface. Continuation lines are returned separately rather than dropped, so
/// that a change in clap's layout reds this file instead of silently shrinking
/// its population.
fn commands(page: &str) -> (Vec<(String, Vec<String>)>, Vec<String>) {
  let mut found = Vec::new();
  let mut unparsed = Vec::new();
  let mut in_block = false;

  for line in page.lines() {
    if line == "Commands:" {
      in_block = true;
      continue;
    }
    if !in_block {
      continue;
    }
    if line.trim().is_empty() {
      break;
    }

    let Some(rest) = line.strip_prefix("  ") else {
      unparsed.push(line.to_string());
      continue;
    };
    if rest.starts_with(' ') {
      // A wrapped description, not a command.
      continue;
    }
    let mut parts = rest.splitn(2, char::is_whitespace);
    let name = parts.next().unwrap_or_default().to_string();
    let tail = parts.next().unwrap_or_default();

    // `[alias: rm]` and `[aliases: a, b]`. Written as a literal scan rather
    // than a pattern because the pattern is exactly what got this wrong once
    // already.
    let mut aliases = Vec::new();
    for opener in ["[alias: ", "[aliases: "] {
      if let Some(at) = tail.find(opener) {
        let after = &tail[at + opener.len()..];
        if let Some(end) = after.find(']') {
          for spelling in after[..end].split(',') {
            let spelling = spelling.trim();
            if !spelling.is_empty() {
              aliases.push(spelling.to_string());
            }
          }
        }
      }
    }
    found.push((name, aliases));
  }

  (found, unparsed)
}

/// Every spelling the binary offers, walked to exhaustion from the root.
fn offered() -> (Vec<Offering>, Vec<String>) {
  let mut out = Vec::new();
  let mut unparsed = Vec::new();
  let mut seen: BTreeSet<Vec<String>> = BTreeSet::new();
  let mut queue = vec![Vec::<String>::new()];

  while let Some(path) = queue.pop() {
    if !seen.insert(path.clone()) {
      continue;
    }
    let (found, mut bad) = commands(&help(&path));
    unparsed.append(&mut bad);

    for (name, aliases) in found {
      let mut child = path.clone();
      child.push(name);
      out.push(Offering {
        path: child.clone(),
        spelling: Spelling::Name,
      });
      for alias in aliases {
        let mut spelling = path.clone();
        spelling.push(alias);
        out.push(Offering {
          path: spelling,
          spelling: Spelling::Alias,
        });
      }
      queue.push(child);
    }
  }

  out.sort();
  (out, unparsed)
}

/// **THE PURE HALF, SO THE VERDICT CAN BE DRIVEN BOTH WAYS.**
///
/// The enumeration has to spawn a process and the comparison does not, so they
/// are separated: planting an undeclared subcommand in a real binary would
/// mean shipping one, while planting it in this function's input costs a line
/// and proves the same thing.
fn undeclared(offered: &[Offering], declared: &BTreeSet<Vec<String>>) -> Vec<String> {
  offered
    .iter()
    .filter(|o| !declared.contains(&o.path))
    .map(|o| o.path.join(" "))
    .collect()
}

#[test]
fn every_offered_subcommand_resolves_to_a_declared_spelling() {
  let (offered, unparsed) = offered();
  let declared = common::declared_spellings();

  assert!(
    unparsed.is_empty(),
    "the `Commands:` block held lines this walk could not read, so its population is smaller \
     than the surface and the census below is over an unstated subset:\n  {}",
    unparsed.join("\n  ")
  );

  let missing = undeclared(&offered, &declared);
  assert!(
    missing.is_empty(),
    "the binary offers {} spelling(s) the dispatch table does not declare. Each is reachable by \
     an operator and invisible to every TABLE -> SURFACE check, which is `0217` exactly:\n  {}",
    missing.len(),
    missing.join("\n  ")
  );
}

#[test]
fn the_walk_reaches_the_whole_surface_it_claims_to_cover() {
  let (offered, _) = offered();

  // **A CENSUS THAT REACHED NOTHING PASSES THE PROPERTY ABOVE PERFECTLY.**
  // These are the floors that make an empty or shallow walk an error rather
  // than a green. Deliberately floors and not equalities: an equality here
  // would red on every legitimate new command, which is the guard nobody
  // keeps.
  assert!(
    offered.len() > 100,
    "the walk found only {} spelling(s); the surface had 175 when this was written, so a figure \
     this low means the enumeration stopped early rather than the surface shrinking",
    offered.len()
  );
  let depth = offered.iter().map(|o| o.path.len()).max().unwrap_or(0);
  assert!(
    depth >= 3,
    "the walk reached depth {depth}. `claude ws new` is depth 3 and is declared as a subcommand \
     slot's `values` rather than as a path, so a walk that stops at 2 cannot see the one class \
     that has no rows of its own"
  );
}

#[test]
fn the_census_recognises_an_alias_when_the_surface_offers_one() {
  let (offered, _) = offered();
  let aliases = offered
    .iter()
    .filter(|o| o.spelling == Spelling::Alias)
    .count();

  // **THIS ARM EXISTS BECAUSE ITS ABSENCE ALREADY COST A WRONG ANSWER.** The
  // prototype's `\[aliases?: ` could not match `[alias: ` and reported zero,
  // which is exactly what a correct census of an alias-free surface reports.
  // Nothing else in this file can tell those two apart.
  assert!(
    aliases > 0,
    "the census recognised no aliases at all. `lang remove` prints `[alias: rm]`, so either the \
     surface lost its aliases or -- far likelier -- the parse cannot see one, and every alias \
     is silently outside this instrument's population"
  );
}

#[test]
fn a_planted_undeclared_subcommand_is_reported() {
  let declared = common::declared_spellings();
  let planted = vec![Offering {
    path: vec!["st".to_string(), "frobnicate".to_string()],
    spelling: Spelling::Name,
  }];

  assert_eq!(
    undeclared(&planted, &declared),
    vec!["st frobnicate".to_string()],
    "the comparison did not report a subcommand no row declares, so its green above means nothing"
  );
}

#[test]
fn a_declared_alias_is_not_reported_as_undeclared() {
  let declared = common::declared_spellings();
  let alias = vec![Offering {
    path: vec!["lang".to_string(), "rm".to_string()],
    spelling: Spelling::Alias,
  }];

  // The first-run trap the row names, pinned rather than avoided by accident.
  assert!(
    undeclared(&alias, &declared).is_empty(),
    "`lang rm` was reported undeclared. It is the alias of `lang remove`, which IS declared, so \
     this is the check crying wolf on its own first run"
  );
}

#[test]
fn the_synthetic_help_verb_reaches_the_declared_population() {
  let (offered, _) = offered();
  let declared = common::declared_spellings();

  let observed: BTreeSet<Vec<String>> = offered
    .iter()
    .filter(|o| o.path.len() == 2 && o.path[1] == "help")
    .map(|o| o.path.clone())
    .collect();

  // **THE FLOOR IS THE FALSIFIABLE PART.** `help` is declared by no row at
  // all; it reaches the population only through
  // `dispatch::families_with_synthetic_help`. If that derivation ever returns
  // nothing while the spine keeps adding the verb, sixteen reachable
  // subcommands leave the declared set at once -- which is `0217` restored by
  // the very mechanism that closed it.
  assert!(
    !observed.is_empty(),
    "the binary offered `help` on no family, so this arm has no subject and the derivation it      is about is unexercised"
  );
  for spelling in &observed {
    assert!(
      declared.contains(spelling),
      "`{}` is offered and not declared -- the synthetic-help derivation is not reaching the        population",
      spelling.join(" ")
    );
  }
}
