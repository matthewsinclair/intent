//! **A verb's leaf takes the flags its slot declares and no other** (issue 0661).
//!
//! The dispatch table declares each verb's flags once, on its verb slot
//! (`value_flags`), and the spine builds every verb's leaf from that list. These
//! arms compare the built command tree with the table, or parse argv and stop,
//! so they write nothing. That is what lets them reach the rows
//! `flag_reachability.rs` must not drive: `estate control`, `estate lane` and
//! `estate slots` act on the host when they run, so `populations.not_probed`
//! lists them, and the sweep never saw their leaves offer flags the verbs
//! refused.
//!
//! **WHAT THE COMPARISON GUARDS, AND WHAT IT CANNOT SEE.** A leaf's flags and
//! the table now come from one home, so the comparison guards the spine's
//! construction: a leaf built with its row's whole flag set again, or a slot's
//! parent carrying flags its bare command never reads. It cannot see a flag the
//! table gives a verb whose arm never reads it. `flag_reachability.rs` checks
//! that some arm in render reads each declared flag, and cannot say which arm.
//!
//! The table is read here as JSON, not through the schema reader's
//! `value_flags` field, so these arms compile against the spine as it stood
//! before the fix, where they fail: every leaf carried its row's whole flag set,
//! and so did the slot's parent.

use std::collections::BTreeSet;

use clap::Command;
use intent_cli::dispatch;
use intent_cli::spine;

/// The raw JSON of the row at `path`, wherever the table declares it.
fn raw_row<'a>(raw: &'a serde_json::Value, path: &str) -> &'a serde_json::Value {
  raw["families"]
    .as_array()
    .into_iter()
    .flatten()
    .flat_map(|f| f["entries"].as_array().into_iter().flatten())
    .chain(raw["new_surface"].as_array().into_iter().flatten())
    .find(|e| e["path"] == path)
    .unwrap_or_else(|| panic!("the table declares `{path}`"))
}

/// The ids clap reads the flags in `spellings` by, for the row's shipped flags.
fn ids_of(entry: &dispatch::Entry, spellings: &serde_json::Value) -> BTreeSet<String> {
  spellings
    .as_array()
    .into_iter()
    .flatten()
    .map(|s| {
      let s = s.as_str().expect("a spelling is a string");
      entry
        .flags
        .iter()
        .filter(|f| f.ships())
        .find(|f| f.spellings.iter().any(|x| x == s))
        .and_then(dispatch::Flag::arg_id)
        .unwrap_or_else(|| panic!("`{}` declares no shipped flag `{s}`", entry.path))
    })
    .collect()
}

/// The ids of the flags a built command carries: its args that are not
/// positionals, without clap's own `help` and `version`.
fn flags_on(cmd: &Command) -> BTreeSet<String> {
  cmd
    .get_arguments()
    .filter(|a| !a.is_positional())
    .map(|a| a.get_id().to_string())
    .filter(|id| id != "help" && id != "version")
    .collect()
}

/// The built command at a row's path.
fn command_at<'a>(root: &'a Command, path: &str) -> &'a Command {
  path.split(' ').fold(root, |cmd, segment| {
    cmd
      .find_subcommand(segment)
      .unwrap_or_else(|| panic!("the spine builds `{path}`"))
  })
}

/// Every shipped row whose verb slot carries values and whose flags ship.
fn slot_rows(table: &dispatch::Table) -> Vec<&dispatch::Entry> {
  dispatch::shipped_entries(table)
    .into_iter()
    .filter(|e| {
      e.args
        .iter()
        .any(|a| a.kind == "subcommand" && !a.values.is_empty())
        && e.flags.iter().any(|f| f.ships())
    })
    .collect()
}

#[test]
fn every_verb_leaf_carries_exactly_the_flags_its_slot_declares() {
  let table = dispatch::table();
  let raw: serde_json::Value = serde_json::from_str(dispatch::TABLE).expect("the table parses");
  let root = spine::build(&table);
  let rows = slot_rows(&table);
  assert!(
    rows.iter().any(|e| e.path == "estate slots"),
    "the population reaches the row issue 0661 was found on, or this arm compares nothing that matters"
  );
  let mut wrong = Vec::new();
  for entry in rows {
    let slot = raw_row(&raw, &entry.path)["args"]
      .as_array()
      .into_iter()
      .flatten()
      .find(|a| a["type"] == "subcommand" && a["values"].as_array().is_some_and(|v| !v.is_empty()))
      .expect("the row's verb slot");
    let parent = command_at(&root, &entry.path);
    for verb in slot["values"].as_array().into_iter().flatten() {
      let verb = verb.as_str().expect("a verb is a string");
      let declared = ids_of(entry, &slot["value_flags"][verb]);
      let leaf = parent
        .find_subcommand(verb)
        .unwrap_or_else(|| panic!("`{} {verb}` is built", entry.path));
      let built = flags_on(leaf);
      if built != declared {
        wrong.push(format!(
          "`{} {verb}` carries {built:?} where its slot declares {declared:?}",
          entry.path
        ));
      }
    }
  }
  assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn a_slots_parent_carries_only_the_flags_its_bare_command_reads() {
  let table = dispatch::table();
  let raw: serde_json::Value = serde_json::from_str(dispatch::TABLE).expect("the table parses");
  let root = spine::build(&table);
  let mut wrong = Vec::new();
  for entry in slot_rows(&table) {
    let slot = raw_row(&raw, &entry.path)["args"]
      .as_array()
      .into_iter()
      .flatten()
      .find(|a| a["type"] == "subcommand" && a["values"].as_array().is_some_and(|v| !v.is_empty()))
      .expect("the row's verb slot");
    // A required slot refuses the bare command, and an optional one runs its
    // default verb when it names one, and no verb when it does not.
    let bare_runs = slot["arity"]
      .as_str()
      .is_some_and(|a| a.starts_with('0'))
      .then(|| slot["default"].as_str())
      .flatten();
    let declared = bare_runs.map_or_else(BTreeSet::new, |verb| {
      ids_of(entry, &slot["value_flags"][verb])
    });
    let built = flags_on(command_at(&root, &entry.path));
    if built != declared {
      wrong.push(format!(
        "`{}` carries {built:?} where its bare command reads {declared:?}",
        entry.path
      ));
    }
  }
  assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn a_flag_given_before_the_verb_is_refused_at_parse() {
  let root = spine::build(&dispatch::table());
  let err = root
    .try_get_matches_from([
      "intent", "estate", "slots", "--named", "tip=x", "release", "01X",
    ])
    .expect_err("`--named` before the verb names no verb that reads it");
  assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
}

#[test]
fn a_bare_command_still_takes_its_default_verbs_flags() {
  let root = spine::build(&dispatch::table());
  for argv in [
    &["intent", "claude", "rules", "--lang", "rust"][..],
    &["intent", "claude", "rules", "list", "--lang", "rust"][..],
  ] {
    root.clone().try_get_matches_from(argv).unwrap_or_else(|e| {
      panic!("{argv:?}: bare `claude rules` is `list`, which reads `--lang`: {e}")
    });
  }
}
