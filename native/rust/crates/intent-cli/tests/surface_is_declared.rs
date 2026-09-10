//! AT-06.13 / AC-06.13: every subcommand the BUILT BINARY offers resolves to a
//! path the dispatch table declares.
//!
//! # The direction is the whole point
//!
//! `flag_reachability.rs` walks the TABLE and asks the binary about each entry.
//! Every other instrument in the estate does the same. **A verb present in the
//! binary and absent from the table is outside that population by
//! construction**, so no amount of care in those checks can see it -- the same
//! shape as `declared_kind_check.sh` being unable to see an artefact at a path
//! no row cites, one level up.
//!
//! Issue `0217` is the demonstration rather than the argument: `245dcdbe` added
//! an explicit `help` verb to nine shipped families, and **every instrument in
//! the estate stayed green** -- intent-cli 463 + 252 + 1, intentsvcs 1183 + 170,
//! `cargo check --workspace --all-targets` clean. Nothing was careless. The
//! defect was simply not in anybody's population.
//!
//! So this file walks the other way, and it is a SEPARATE file rather than an
//! arm of `flag_reachability.rs` deliberately: folding them would put two
//! denominators behind one exit code, and the two questions fail for unrelated
//! reasons.
//!
//! # The artefact is this build's binary, never the shared pair
//!
//! `env!("CARGO_BIN_EXE_intent")` -- the binding `flag_reachability.rs` already
//! uses. **Not the shared release pair**, which describes whatever was last
//! built and is guarded against rebuild while `native/rust` is dirty: a test
//! bound to it would be unrunnable exactly when the surface it checks is being
//! changed, and would otherwise report a green taken off a stale artefact. That
//! is not hypothetical -- it happened on the morning of 2026-09-02.
//!
//! # What "the binary offers" means, and the reach of that
//!
//! It means what `--help` lists, walked recursively. That is the surface an
//! operator reads, and it is the surface `0217` is about.
//!
//! **A HIDDEN subcommand is therefore out of reach, and that is stated rather
//! than quietly true.** clap omits hidden aliases from help, so a help walk
//! cannot see one; `retired_commands::a_hidden_alias_is_accepted_and_appears_in_no_output`
//! holds that ground instead. A hidden spelling is by definition not offered,
//! so the gap is between this check and a stronger one, not between this check
//! and the row.

use std::collections::BTreeSet;
use std::process::Command;

use intent_cli::dispatch;

/// **THE RATCHET IS GONE BECAUSE THE REMEDY LANDED, NOT BECAUSE IT WAS
/// WAIVED.** This file carried `HELP_NOT_DECLARED`, sixteen `<family> help`
/// spellings the binary offers and no row declared, with the note that
/// `AC-06.13` stays UNSATISFIED while the list is non-empty -- greening on a
/// ratchet's existence certifies the process instead of the property.
///
/// **vc RULED IT 2026-09-09 UNDER hv's PEN (`0217`): the rows are DERIVED.**
/// Not hand-authored -- sixteen identical rows describing one uniform
/// behaviour is the transcription `AC-17.15` forbids -- and not exempted,
/// because an exemption puts the verb outside this file's population by
/// construction, which is the defect `0217` names, reintroduced one level in.
/// So [`dispatch::family_builds_out_its_verbs`] is the one home for the
/// spine's own predicate: the spine calls it to ADD the verb and [`declared`]
/// calls it to DECLARE one. The list shrank to empty by the table learning to
/// say the thing, which is what the note said would happen.
///
/// **AND THE COUNT WAS SIXTEEN, NOT THE NINE ON `0217`'s RECORD** -- derived
/// here 2026-09-07, re-derived two independent ways 2026-09-09, and corrected
/// at the issue rather than annotated.
/// One `--help` render from the binary under test.
///
/// `current_dir` is a temp dir and `HOME` is the fixture home, so the walk
/// describes the BUILD and not the project it happened to run in. clap answers
/// `--help` before dispatch, so neither should matter; pinning them means a
/// future arm that reads a project cannot quietly make this walk depend on
/// where it was invoked.
fn help_for(path: &[String]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(path)
    .arg("--help")
    .current_dir(std::env::temp_dir())
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(path))
    .output()
    .expect("run the binary under test for --help");
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// The subcommand names clap lists under `Commands:`.
///
/// **A row is exactly two spaces then the name.** A wrapped description aligns
/// to the description column, which is deeper, so it is skipped rather than
/// mistaken for a command -- measured at zero occurrences on this build, and
/// guarded anyway because a longer help string introduces one silently.
fn subcommands_in(help: &str) -> Vec<(String, Vec<String>)> {
  let mut out = Vec::new();
  let mut inside = false;
  for line in help.lines() {
    if line.starts_with("Commands:") {
      inside = true;
      continue;
    }
    if !inside {
      continue;
    }
    if line.trim().is_empty() {
      break;
    }
    let Some(rest) = line.strip_prefix("  ") else {
      continue;
    };
    if rest.starts_with(char::is_whitespace) {
      continue;
    }
    let mut fields = rest.splitn(2, char::is_whitespace);
    let Some(name) = fields.next() else {
      continue;
    };
    // **`[alias: rm]` IS THE ONLY PLACE AN ALIAS APPEARS**, and it is inside
    // the description rather than in the name column. Matched as a literal
    // rather than a pattern: the first attempt at this used `\[aliases?: `,
    // which can never match `[alias: ` because the `?` binds to the `e` and
    // not to the word, and it reported ZERO aliases on a surface with four.
    // A pattern that cannot match its subject returns a confident zero, and a
    // zero is the one result that never looks like a bug in the query.
    let tail = fields.next().unwrap_or_default();
    let mut aliases = Vec::new();
    for opener in ["[alias: ", "[aliases: "] {
      if let Some(at) = tail.find(opener) {
        let after = &tail[at + opener.len()..];
        if let Some(close) = after.find(']') {
          for spelling in after[..close].split(',') {
            let spelling = spelling.trim();
            if !spelling.is_empty() {
              aliases.push(spelling.to_string());
            }
          }
        }
      }
    }
    out.push((name.to_string(), aliases));
  }
  out
}

/// What the walk found, with the aliases kept separately.
///
/// **THE SPLIT EXISTS SO THE CENSUS CAN PROVE IT RECOGNISED AN ALIAS.** A
/// parse that silently recognises none is indistinguishable from a surface
/// that has none, and nothing else in this file can tell those apart.
struct Offered {
  /// Every spelling, names and aliases alike -- the gate's population.
  all: Vec<Vec<String>>,
  /// Just the alias spellings.
  aliases: Vec<Vec<String>>,
}

/// Every subcommand path the binary offers, depth-first from the root.
///
/// **The `visited` set is a termination proof, not an optimisation.** A help
/// tree that ever cycled would hang the suite rather than fail it, and a hang
/// is the one failure mode nobody reads. Measured on this build: 170 paths over
/// three levels, and `<family> help --help` lists no commands, so `help` does
/// not recurse.
fn offered_paths() -> Offered {
  let mut found = Vec::new();
  let mut aliases = Vec::new();
  let mut visited: BTreeSet<Vec<String>> = BTreeSet::new();
  let mut queue: Vec<Vec<String>> = vec![Vec::new()];
  while let Some(path) = queue.pop() {
    for (verb, alias_spellings) in subcommands_in(&help_for(&path)) {
      let mut child = path.clone();
      child.push(verb);
      if !visited.insert(child.clone()) {
        continue;
      }
      found.push(child.clone());
      // An alias is a spelling an operator can type, so it belongs in the
      // gate's population -- but it names the SAME command, so recursing into
      // it would walk the whole subtree twice under two names.
      for alias in alias_spellings {
        let mut spelling = path.clone();
        spelling.push(alias);
        if visited.insert(spelling.clone()) {
          found.push(spelling.clone());
          aliases.push(spelling);
        }
      }
      queue.push(child);
    }
  }
  found.sort();
  aliases.sort();
  Offered {
    all: found,
    aliases,
  }
}

/// Whether the table declares this path, by either of the two forms it uses.
///
/// **THE SECOND FORM IS LOAD-BEARING AND OMITTING IT COSTS 21 FALSE FINDINGS.**
/// A family is modelled either as one row per verb, or -- the `daemon` pattern
/// -- as ONE row whose `subcommand` argument declares its verbs as `values`.
/// `claude rules`, `claude skills`, `claude subagents`, `claude ws`,
/// `agents template` and `surface` all take the second shape, and reading only
/// entry paths reports every one of their leaves as undeclared. Measured before
/// this function existed: 37 undeclared, of which 21 were this mistake.
///
/// The vocabulary is read by KIND rather than by argument name, because the
/// name is not stable across families -- `command`, `verb` and `subcommand` are
/// all in use. That is the same read
/// `unmigrated_surface::the_surface_exemption_states_the_precondition_it_rests_on`
/// already makes.
fn declared(table: &dispatch::Table, segments: &[String]) -> bool {
  let path = segments.join(" ");
  let entries = dispatch::shipped_entries(table);

  if entries
    .iter()
    .any(|e| e.spellings().iter().any(|s| s.join(" ") == path))
  {
    return true;
  }

  let Some((leaf, head)) = segments.split_last() else {
    return false;
  };
  if head.is_empty() {
    return false;
  }
  let parent = head.join(" ");
  if entries.iter().any(|e| {
    e.path == parent
      && e
        .args
        .iter()
        .any(|a| a.kind == "subcommand" && a.values.iter().any(|v| v == leaf))
  }) {
    return true;
  }

  // **FORM THREE: THE SYNTHETIC `help`, WHICH NO ROW DECLARES AND SIXTEEN
  // FAMILIES OFFER.** The spine adds it to every shipped family that has at
  // least one shipped verb, and `0217` ruled those rows DERIVED rather than
  // authored or exempted -- so the predicate asks the spine's own function
  // rather than carrying a list that would rot on the seventeenth family.
  head.len() == 1
    && leaf == "help"
    && table
      .families
      .iter()
      .any(|f| f.name == head[0] && dispatch::family_builds_out_its_verbs(f))
}

/// **THE GATE.** Every offered path is declared, or is a named, owned exception.
#[test]
fn every_subcommand_the_binary_offers_is_declared_by_the_table() {
  let table = dispatch::table();
  let offered = offered_paths();

  let mut undeclared = Vec::new();
  for path in &offered.all {
    if declared(&table, path) {
      continue;
    }
    undeclared.push(path.join(" "));
  }

  assert!(
    undeclared.is_empty(),
    "the binary offers {} subcommand(s) the dispatch table does not declare. This is issue \
     0217's class: the table is the SSOT and a verb that \
     is not in it was never reviewed, never rendered into the reference docs, and is invisible to \
     every other instrument, all of which walk the table. Either declare the row, or -- if the \
     verb should not exist -- take it out of the spine:\n  {}",
    undeclared.len(),
    undeclared.join("\n  ")
  );

  eprintln!(
    "surface-is-declared: {} spelling(s) offered, {} of them aliases, 0 undeclared",
    offered.all.len(),
    offered.aliases.len()
  );
}

/// The walk sees a real surface, and a deep one.
///
/// **WITHOUT THIS THE GATE ABOVE PASSES ON A BROKEN WALKER.** An `offered_paths`
/// that returned nothing -- a renamed help section, a changed indent, a binary
/// that failed to run -- satisfies "no undeclared path" perfectly, and reports
/// a clean surface while checking none of it. The assertions are on the SHAPE
/// of the population rather than on an exact count, which would be a second
/// place to update on every legitimate surface change.
#[test]
fn the_walk_reaches_the_surface_it_claims_to_check() {
  let offered = offered_paths();

  assert!(
    offered.all.len() > 100,
    "this binary ships well over a hundred subcommand paths; {} means the walker stopped early \
     or the help format moved under it",
    offered.all.len()
  );
  assert!(
    offered.all.iter().any(|p| p.len() == 3),
    "the walk must recurse past the second level, or every leaf of a nested family goes \
     unexamined while the check reports a pass"
  );
  for expected in [
    vec!["st".to_string(), "list".to_string()],
    vec![
      "claude".to_string(),
      "skills".to_string(),
      "install".to_string(),
    ],
  ] {
    assert!(
      offered.all.contains(&expected),
      "a path known to be offered is missing from the walk: {expected:?}"
    );
  }

  // **AND THE ALIAS PARSE IS PINNED SEPARATELY, BECAUSE ITS FAILURE IS
  // SILENT.** `lang remove` prints `[alias: rm]`. A parse that finds no
  // aliases reports exactly what a correct parse of an alias-free surface
  // reports, so without this arm every alias could leave the gate's population
  // and nothing here would move. Measured 2026-09-09: four visible aliases.
  assert!(
    !offered.aliases.is_empty(),
    "the walk recognised no aliases at all -- either the surface lost them, or, far likelier, \
     the parse cannot see one and every alias is silently outside this file's population"
  );
}

/// The resolver can say NO, and says YES for each declared form.
///
/// **A predicate that answered `declared` for everything would satisfy the gate
/// above for any binary at all**, which is the failure this file would be least
/// likely to notice about itself: the gate's green and this predicate's
/// blindness look identical from outside.
#[test]
fn the_declared_predicate_answers_both_ways_and_knows_both_forms() {
  let table = dispatch::table();
  let seg = |s: &str| s.split(' ').map(str::to_string).collect::<Vec<_>>();

  assert!(
    !declared(&table, &seg("st definitely-not-a-verb")),
    "the predicate must be able to refuse, or the gate proves nothing"
  );
  assert!(
    !declared(&table, &seg("not-a-family")),
    "and refuse at the root as well as under a real family"
  );
  assert!(
    declared(&table, &seg("st list")),
    "form one: a verb declared as its own entry path"
  );
  assert!(
    declared(&table, &seg("claude skills install")),
    "form two: a verb declared as a value on its parent's subcommand argument -- the form whose \
     absence reported 21 working commands as undeclared"
  );
  assert!(
    declared(&table, &seg("lang rm")),
    "and an ALIAS is a spelling the binary genuinely offers, so `Entry::spellings()` is the read \
     rather than `Entry::path`"
  );
}

/// The derivation that replaced the ratchet actually reaches the surface.
///
/// **AN EMPTY EXCEPTION LIST AND A DERIVATION THAT RETURNS NOTHING LOOK
/// IDENTICAL FROM THE GATE.** `HELP_NOT_DECLARED` used to be re-driven here --
/// each named path still offered, still undeclared -- and deleting it removes
/// that check along with the list. What replaces it is the floor: sixteen
/// `<family> help` spellings are offered, no row declares one, and the gate is
/// green, so [`declared`]'s third form must be answering. If the derivation
/// ever returned empty they would all become undeclared at once.
///
/// **WHAT THIS DELIBERATELY DOES NOT DO IS COMPARE THE DERIVATION TO THE
/// SPINE.** An earlier draft asserted `family_builds_out_its_verbs` over the families
/// equals the families the binary offers `help` on. That reads like the
/// strongest arm available and is worth nothing: the spine ADDS the verb
/// through that same function, so both sides move together. **Driven rather
/// than reasoned -- the predicate was mutated from *at least one verb* to
/// *more than one verb*, the surface changed, and every arm stayed green.**
/// A test that imports the value it asserts has stopped testing.
///
/// Repairing it would mean a second implementation of the predicate living
/// here to disagree with the first, which is the second home `0217`'s ruling
/// existed to avoid. The spine and the declaration cannot disagree about WHICH
/// families get `help`, so no arm should claim to watch for it.
#[test]
fn the_synthetic_help_derivation_reaches_every_family_that_offers_it() {
  let table = dispatch::table();
  let offered = offered_paths();

  let help_paths: Vec<&Vec<String>> = offered
    .all
    .iter()
    .filter(|p| p.len() == 2 && p[1] == "help")
    .collect();

  assert!(
    !help_paths.is_empty(),
    "the binary offered `help` on no family, so the derivation this arm is about is unexercised \
     and its green says nothing"
  );
  for path in help_paths {
    assert!(
      declared(&table, path),
      "`{}` is offered and the table does not declare it. No row spells this verb -- it reaches \
       the population only through `dispatch::family_builds_out_its_verbs`, so this is that \
       derivation failing rather than a row anybody forgot to write",
      path.join(" ")
    );
  }
}
