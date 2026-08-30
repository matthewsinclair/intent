//! **`populations.self_loop` is bound to the ratified machines** -- issue 0050.
//!
//! `no_op` states what a verb prints when it is asked for the state the entity
//! is already in, and it is required on every row of `populations.self_loop`.
//! That population is the set of rows whose verb is an edge in one of the four
//! ratified machines -- which makes it derivable in principle and AUTHORED in
//! fact, because the machines live in `transitions.rs` and the generator that
//! checks the table reads JSON.
//!
//! **So the generator can only see the WAKE of a short list, not the omission.**
//! `NO_OP_SKEW` refuses a row that carries a `no_op` while absent from the
//! population -- somebody knew the verb self-loops and the list did not -- and
//! that is a real check, but a verb that self-loops, is missing from the list,
//! AND carries no `no_op` passes every arm it has. The generator's own comment
//! says so in those words. This file is the arm that closes it, and it is the
//! same shape as `dispatch_ssot.rs` binding `Entry::is_shipped()` to
//! `populations.shipped`: the authored list is compared to the thing it claims
//! to describe, in the one language that can read both.
//!
//! **Every list in this table that went short, went short in silence.**
//! `SURFACE_NONRETURNING` lost `claude upgrade` to a key name that defined only
//! one of its two admission reasons. `lib_surface.sh` lost the eight
//! `new_surface` rows to `.families[].entries[]`. Neither had a reader in a
//! language that could see the source of truth; both looked complete.
//!
//! **The mapping is verb-to-path and it is mechanical except where it is not.**
//! `st.done` is `st done`, `issues.close` is `issues close`, `wp.rescope` is `wp
//! rescope` -- dot to space, across every machine. The exceptions are declared
//! below rather than hidden in a normaliser, because an exception nobody can
//! enumerate is how the next list goes short.
//!
//! **Mutation-proved 2026-08-17, because a test that has only ever been green
//! has been performed rather than measured** -- cc's sentence, from finding that
//! the workspace was 485/0 both before and after the nineteen-arm sweep, so no
//! test asserted either spelling. Three mutants, failing set predicted in
//! advance for each:
//!
//! | mutant                              | predicted     | observed                          |
//! | ----------------------------------- | ------------- | --------------------------------- |
//! | drop `wp done` from `self_loop`     | verb-has-row  | exact, named `wp.done`            |
//! | add `st frobnicate` to `self_loop`  | row-traces    | exact, named it; other two passed |
//! | `FANS_OUT` -> `at nonesuch`         | fan-out-exists| fan-out AND row-traces both red   |
//!
//! The third failing two tests is correct rather than sloppy: pointing the
//! fan-out at a row that does not exist also orphans the real `at na`, and both
//! statements are true of that mutation.
//!
//! **The harness had the bug the tests are about.** Its restore ran on relative
//! paths while a helper had changed directory, so the trap that was supposed to
//! undo each mutant in a live five-node clone silently did not fire, and the
//! mutations sat on disk until checked. A guard that does not run looks exactly
//! like a guard that found nothing.

use std::collections::BTreeSet;

use intent_cli::dispatch;
use intentsvcs::transitions;

/// Verbs whose CLI spelling is not `verb.replace('.', " ")`, and why.
///
/// **`at.set` FANS OUT; THE FIAT-CLOSE EDGES RENAME -- one table, two reasons,
/// and the second one arrived with the fiat close.** One machine verb reaching three
/// rows is the fan-out case: v2 spelled the AT statuses as separate commands
/// and the rows are parity-bound to that, so there is no `intent at set`. One
/// machine verb reaching ONE differently-named row is the other, and the
/// default derivation (`verb.replace('.', " ")`) cannot express it.
///
/// **A FIAT-CLOSE EDGE IS SPELLED `fc` AT THE SURFACE, TOP-LEVEL, WHATEVER
/// MACHINE IT SITS ON, and that is a ruling rather than a shortening** (ic,
/// 2026-08-29). `AC-00.3` makes a fiat close
/// cross-family by construction -- closing an ST fiat-closes its open children,
/// which are WPs, criteria and tests -- so a family-rooted `ac fc` could not
/// reach them, and a family verb writing into two other families is the thing
/// family roots exist to prevent. **Each MACHINE keeps its own token** --
/// `ac.fc` names an edge on `Criterion.state`, `at.fc` one on
/// `AcceptanceTest.status` -- because the token records which machine moved.
/// Only the path is shared, and it is shared because there is one command.
///
/// **THE ENTRIES ARRIVE ONE PER MACHINE AS ITS EDGE LANDS, AND THAT IS NOT A
/// LIST GOING STALE.** Each names an edge that exists today. A machine whose
/// `Fiat` variant is still mid-cascade has no edge for this file to map, and
/// writing its row ahead of the code would be a claim about something nobody
/// has built -- the defect this directory keeps finding, in the one file whose
/// job is to catch it.
///
/// A machine verb with no row at all would be issue 0052 -- `wp.rescope` was
/// exactly that until its row landed -- so this list must never be used to
/// excuse an absent row. Every entry here names rows that EXIST.
const FANS_OUT: &[(&str, &[&str])] = &[
  ("at.set", &["at green", "at red", "at na"]),
  ("ac.fc", &["fc"]),
  ("at.fc", &["fc"]),
  // **THE THIRD AND FOURTH MACHINES, ARRIVING AS THEIR EDGES LANDED** (dc,
  // `d4526c1b`), which is what the note above says these entries do. Both are
  // `fc` for the same reason as their siblings: one command, and the TARGET
  // decides which machine moved. `intent fc <TARGET> [CHILD]` takes an optional
  // child, so omitting it fiat-closes the thread or work package itself.
  //
  // **THESE TWO WERE STRUCK BEFORE THEY WERE RATIFIED, AND THAT IS THE REASON
  // TO ADD THEM RATHER THAN TO EXEMPT THEM.** dc and hv ruled that ST and WP
  // get no status VARIANT -- `fiat` sits BESIDE a status that stays
  // `completed`/`done` -- and the earlier reading of that ruling was that
  // `st.fc`/`wp.fc` were struck outright. They were not: the edges exist, on
  // the same machines, writing a field beside the status rather than a new
  // status. A verb whose edge exists and whose row exists belongs here.
  ("st.fc", &["fc"]),
  ("wp.fc", &["fc"]),
];

/// Rows that belong in the population without owning a machine verb.
///
/// **These DELEGATE, and the no-op reaches the user through them.** `todo done`
/// calls `st_done`/`wp_done` and is, at HEAD, the only arm in the whole
/// renderer that reports a no-op at all -- which is the asymmetry issue 0050 is
/// about. A population that excluded it would leave the one row that gets this
/// right unable to say so.
const DELEGATES: &[&str] = &["todo done", "todo notdone", "todo toggle"];

fn population() -> BTreeSet<String> {
  let raw: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  let list = raw["populations"]["self_loop"].as_array().expect(
    "the table has no `populations.self_loop`, so this test would pass by having nothing to \
     compare -- which is the exact failure it exists to prevent",
  );
  list
    .iter()
    .map(|v| {
      v.as_str()
        .expect("every `self_loop` member is a path string")
        .to_string()
    })
    .collect()
}

/// Every verb declared on an edge of any ratified machine, deduplicated.
fn machine_verbs() -> BTreeSet<&'static str> {
  transitions::FIELDS
    .iter()
    .filter_map(|f| match &f.disposition {
      transitions::Disposition::State { edges, .. } => Some(edges.iter()),
      _ => None,
    })
    .flatten()
    .map(|e| e.verb)
    .collect()
}

fn rows_for(verb: &str) -> Vec<String> {
  if let Some((_, paths)) = FANS_OUT.iter().find(|(v, _)| *v == verb) {
    return paths.iter().map(|s| s.to_string()).collect();
  }
  vec![verb.replace('.', " ")]
}

/// **The direction the generator cannot check: a machine verb the list omits.**
#[test]
fn every_ratified_verb_has_a_row_in_the_self_loop_population() {
  let pop = population();
  let mut missing = Vec::new();

  for verb in machine_verbs() {
    for path in rows_for(verb) {
      if !pop.contains(&path) {
        missing.push(format!("`{verb}` -> expected row `{path}`"));
      }
    }
  }

  assert!(
    missing.is_empty(),
    "{} ratified verb(s) reach a row that `populations.self_loop` does not name, so those rows \
     are not required to declare a `no_op` and the generator cannot tell:\n  {}\n\nAdd them to \
     the population. If a verb genuinely has no CLI row, that is not a population bug -- it is \
     issue 0052, a machine the surface cannot reach, and it wants a row rather than an \
     exemption.",
    missing.len(),
    missing.join("\n  ")
  );
}

/// The other direction, so the population cannot grow rows that describe nothing.
///
/// **Not symmetric with the check above, and the asymmetry is the point.** A
/// missing member silently drops a requirement; a spurious one only ever adds
/// an unneeded declaration. So this fails with a much narrower claim: the row
/// traces to no verb, so nobody can say what its `no_op` is a statement about.
#[test]
fn every_self_loop_row_traces_to_a_ratified_verb_or_a_declared_delegate() {
  let verbs = machine_verbs();
  let reachable: BTreeSet<String> = verbs
    .iter()
    .flat_map(|v| rows_for(v))
    .chain(DELEGATES.iter().map(|s| s.to_string()))
    .collect();

  let pop = population();
  let orphans: Vec<&String> = pop.iter().filter(|p| !reachable.contains(*p)).collect();

  assert!(
    orphans.is_empty(),
    "`populations.self_loop` names {} row(s) that are not a ratified verb and not a declared \
     delegate:\n  {}\n\nEither the machine lost an edge, or the row was added on a belief about \
     the surface rather than the model.",
    orphans.len(),
    orphans
      .iter()
      .map(|s| format!("`{s}`"))
      .collect::<Vec<_>>()
      .join("\n  ")
  );
}

/// The fan-out table must not be a place absent rows go to hide.
///
/// **`FANS_OUT` is the one hand-written mapping in this file, so it is the one
/// thing that can quietly absorb a defect.** An entry naming a row that does
/// not exist would make `every_ratified_verb_has_a_row` pass by pointing at
/// nothing -- the same failure as a `not_probed` exclusion naming a command
/// that does not ship, which the generator already refuses one layer over.
#[test]
fn the_fan_out_mapping_names_only_rows_that_exist() {
  let table = dispatch::table();
  let mut absent = Vec::new();

  for (verb, paths) in FANS_OUT {
    for path in *paths {
      if dispatch::entry(&table, path).is_none() {
        absent.push(format!("`{verb}` fans out to `{path}`, which is not a row"));
      }
    }
  }
  for path in DELEGATES {
    if dispatch::entry(&table, path).is_none() {
      absent.push(format!("`{path}` is a declared delegate and is not a row"));
    }
  }

  assert!(
    absent.is_empty(),
    "the mapping in this file points at rows the table does not have, so the checks above pass \
     by comparing against nothing:\n  {}",
    absent.join("\n  ")
  );
}
