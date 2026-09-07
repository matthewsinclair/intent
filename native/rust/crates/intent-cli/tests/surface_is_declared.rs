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

/// Offered-but-undeclared paths that are KNOWN, OWNED, and not this file's to
/// fix -- reported by name on every run, and not gated.
///
/// **THIS IS `flag_reachability.rs`'s `INHERITED_UNREAD` IDIOM AND IT IS
/// DELIBERATE, NOT A SOFTENING.** Gating on these would make this a
/// permanently-red check over a decision nobody here can take, which is the
/// guard that must be bypassed, which is the guard nobody keeps. Anything NOT
/// on this list reddens immediately -- so the class `0217` names is closed from
/// today even though this instance is not.
///
/// **AND THE RATCHET IS NOT THE CRITERION.** `AC-06.13` stays UNSATISFIED while
/// this list is non-empty. Greening on the ratchet's existence would certify
/// the process instead of the property, which is the ruling `AT-06.8` already
/// carries in those words.
///
/// # Every member is one thing, and the count is not the one on the record
///
/// `245dcdbe` gave nine families an explicit `help` verb and `0217` records
/// nine. **Measured here on 2026-09-07: SIXTEEN.** clap generates a `help`
/// subcommand on any command that has subcommands, so the population is every
/// family with verbs and not only the nine that got an explicit arm. The number
/// is derived by this file rather than transcribed from the issue, which is why
/// the disagreement is visible at all.
///
/// **THE REMEDY IS A ROW-SHAPE DECISION AND IS vc's**, per `0217`: either the
/// rows are authored -- a row describing a real verb is the table being CORRECT
/// rather than growing -- or `help` is declared once as a property of a family.
/// Nothing here prejudges which; this list shrinks to empty either way.
const HELP_NOT_DECLARED: &[&str] = &[
  "ac help",
  "agents help",
  "app help",
  "at help",
  "claude help",
  "config help",
  "daemon help",
  "ext help",
  "issues help",
  "lang help",
  "llm help",
  "modules help",
  "plugin help",
  "st help",
  "todo help",
  "wp help",
];

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
fn subcommands_in(help: &str) -> Vec<String> {
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
    if let Some(name) = rest.split_whitespace().next() {
      out.push(name.to_string());
    }
  }
  out
}

/// Every subcommand path the binary offers, depth-first from the root.
///
/// **The `visited` set is a termination proof, not an optimisation.** A help
/// tree that ever cycled would hang the suite rather than fail it, and a hang
/// is the one failure mode nobody reads. Measured on this build: 170 paths over
/// three levels, and `<family> help --help` lists no commands, so `help` does
/// not recurse.
fn offered_paths() -> Vec<Vec<String>> {
  let mut found = Vec::new();
  let mut visited: BTreeSet<Vec<String>> = BTreeSet::new();
  let mut queue: Vec<Vec<String>> = vec![Vec::new()];
  while let Some(path) = queue.pop() {
    for verb in subcommands_in(&help_for(&path)) {
      let mut child = path.clone();
      child.push(verb);
      if !visited.insert(child.clone()) {
        continue;
      }
      found.push(child.clone());
      queue.push(child);
    }
  }
  found.sort();
  found
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
  entries.iter().any(|e| {
    e.path == parent
      && e
        .args
        .iter()
        .any(|a| a.kind == "subcommand" && a.values.iter().any(|v| v == leaf))
  })
}

/// **THE GATE.** Every offered path is declared, or is a named, owned exception.
#[test]
fn every_subcommand_the_binary_offers_is_declared_by_the_table() {
  let table = dispatch::table();
  let offered = offered_paths();

  let mut undeclared = Vec::new();
  for path in &offered {
    let spelling = path.join(" ");
    if declared(&table, path) || HELP_NOT_DECLARED.contains(&spelling.as_str()) {
      continue;
    }
    undeclared.push(spelling);
  }

  assert!(
    undeclared.is_empty(),
    "the binary offers {} subcommand(s) the dispatch table does not declare, and they are not on \
     the named exception list. This is issue 0217's class: the table is the SSOT and a verb that \
     is not in it was never reviewed, never rendered into the reference docs, and is invisible to \
     every other instrument, all of which walk the table. Either declare the row, or -- if the \
     verb should not exist -- take it out of the spine:\n  {}",
    undeclared.len(),
    undeclared.join("\n  ")
  );

  eprintln!(
    "offered {}, declared {}, known-undeclared {}",
    offered.len(),
    offered.len() - HELP_NOT_DECLARED.len(),
    HELP_NOT_DECLARED.len()
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
    offered.len() > 100,
    "this binary ships well over a hundred subcommand paths; {} means the walker stopped early \
     or the help format moved under it",
    offered.len()
  );
  assert!(
    offered.iter().any(|p| p.len() == 3),
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
      offered.contains(&expected),
      "a path known to be offered is missing from the walk: {expected:?}"
    );
  }
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

/// The exception list describes today's binary, not the day it was written.
///
/// **A RATCHET THAT IS NEVER RE-DRIVEN ROTS INTO A SILENT EXEMPTION.** Each of
/// two failures is invisible without this: an entry whose verb was fixed goes
/// on excusing a path that no longer needs it, and an entry that stops being
/// offered at all leaves the list looking like work when it is not. Both make
/// the gate's population smaller than it reads.
#[test]
fn every_named_exception_is_still_offered_and_still_undeclared() {
  let table = dispatch::table();
  let offered: BTreeSet<String> = offered_paths().iter().map(|p| p.join(" ")).collect();

  for spelling in HELP_NOT_DECLARED {
    let segments: Vec<String> = spelling.split(' ').map(str::to_string).collect();
    assert!(
      offered.contains(*spelling),
      "`{spelling}` is on the exception list and the binary no longer offers it -- delete the row \
       rather than leaving a name that excuses nothing"
    );
    assert!(
      !declared(&table, &segments),
      "`{spelling}` is on the exception list and the table now DECLARES it. That is the fix \
       landing: take it off the list, and when the list is empty AC-06.13 is satisfied"
    );
  }
}
