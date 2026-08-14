//! AT-05.1 / AC-05.1: the dispatch table is the SSOT -- the clap surface and
//! its help text are GENERATED from it, asserted by test.
//!
//! **Asserted in both directions, against the shipped binary.** Nothing in the
//! table may be absent from the surface, and nothing in the surface may be
//! absent from the table. One direction alone is not the property: a spine
//! that shipped every table entry plus five invented ones would pass a
//! table-to-surface check, and a spine that shipped three entries would pass a
//! surface-to-table check.
//!
//! The comparison runs against the REAL binary's `--help` output rather than
//! against the loader's structs. Comparing the loader to the table would prove
//! that serde works; comparing the binary to the table proves the surface a
//! user meets came from it.

use std::process::Command;

use intent_cli::dispatch;

fn help(args: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .arg("--help")
    .output()
    .expect("run the v3 binary");
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// The family names the binary actually offers, read out of its help.
fn surface_families() -> Vec<String> {
  let text = help(&[]);
  let commands = text
    .split("Commands:")
    .nth(1)
    .expect("the root help lists commands");
  commands
    .lines()
    .map(str::trim_start)
    .take_while(|l| !l.starts_with("Options:"))
    .filter_map(|l| l.split_whitespace().next())
    .filter(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
    .map(str::to_string)
    .collect()
}

#[test]
fn every_shipped_family_in_the_table_reaches_the_surface() {
  let table = dispatch::table();
  let surface = surface_families();

  let mut missing = Vec::new();
  for family in &table.families {
    let Some(entry) = family.entries.iter().find(|e| e.verb().is_none()) else {
      continue;
    };
    if !entry.is_shipped() {
      continue;
    }
    if !surface.contains(&family.name) {
      missing.push(family.name.clone());
    }
  }
  assert!(
    missing.is_empty(),
    "the table ships these families and the binary does not offer them: {missing:?}"
  );
}

#[test]
fn nothing_reaches_the_surface_that_is_not_in_the_table() {
  let table = dispatch::table();
  let known: Vec<&str> = table.families.iter().map(|f| f.name.as_str()).collect();

  let invented: Vec<String> = surface_families()
    .into_iter()
    .filter(|f| !known.contains(&f.as_str()) && f != "help")
    .collect();
  assert!(
    invented.is_empty(),
    "the binary offers commands the table does not describe: {invented:?} -- a surface entry with no table row is a second, undocumented declaration"
  );
}

#[test]
fn a_retired_family_does_not_reach_the_surface() {
  let surface = surface_families();
  assert!(
    !surface.contains(&"organize".to_string()),
    "`organize` is a ratified retire (hv, 2026-08-14): a strictly structured model cannot hold data in the wrong place, so the disorder it repairs cannot arise"
  );
}

/// Every leaf verb, per family, both ways.
#[test]
fn every_shipped_verb_reaches_its_family_and_none_is_invented() {
  let table = dispatch::table();
  let mut problems = Vec::new();

  for family in &table.families {
    let shipped: Vec<&str> = family
      .entries
      .iter()
      .filter(|e| e.is_shipped())
      .filter_map(|e| e.verb())
      .collect();
    if shipped.is_empty() {
      continue;
    }

    let text = help(&[family.name.as_str()]);
    let Some(block) = text.split("Commands:").nth(1) else {
      problems.push(format!("{}: help lists no commands", family.name));
      continue;
    };
    let offered: Vec<String> = block
      .lines()
      .map(str::trim_start)
      .take_while(|l| !l.starts_with("Options:"))
      .filter_map(|l| l.split_whitespace().next())
      .filter(|w| !w.is_empty())
      .map(str::to_string)
      .collect();

    for verb in &shipped {
      if !offered.iter().any(|o| o == verb) {
        problems.push(format!(
          "{} {verb}: in the table, absent from the surface",
          family.name
        ));
      }
    }
    for verb in &offered {
      if verb != "help" && !shipped.contains(&verb.as_str()) {
        problems.push(format!(
          "{} {verb}: on the surface, absent from the table",
          family.name
        ));
      }
    }
  }

  assert!(
    problems.is_empty(),
    "surface and table disagree:\n  {}",
    problems.join("\n  ")
  );
}

/// The help TEXT comes from the table too, not from a second set of strings.
#[test]
fn help_text_is_the_tables_help_text() {
  let table = dispatch::table();
  let text = help(&["st"]);

  let new_entry = table
    .families
    .iter()
    .find(|f| f.name == "st")
    .and_then(|f| f.entries.iter().find(|e| e.path == "st new"))
    .expect("st new is in the table");
  assert!(
    !new_entry.help.is_empty(),
    "precondition: the table carries help text for st new"
  );
  assert!(
    text.contains(&new_entry.help),
    "the binary's help for `st new` is not the table's:\n  table: {}\n  surface: {text}",
    new_entry.help
  );
}

/// The table is versioned and stamped with what it was measured against, so a
/// surface built from it can always name its provenance.
#[test]
fn the_table_names_its_schema_and_the_commit_it_was_measured_at() {
  let table = dispatch::table();
  assert!(table.schema.starts_with("intent/dispatch-table@"));
  assert!(
    !table.measured_at.is_empty(),
    "a measurement names the revision it covers, never HEAD"
  );
}

/// The invariants ic recorded are carried, so INV-02's pin has something to
/// point at from inside the binary.
#[test]
fn the_recorded_invariants_are_carried() {
  let table = dispatch::table();
  let ids: Vec<&str> = table.invariants.iter().map(|i| i.id.as_str()).collect();
  assert!(ids.contains(&"INV-02"), "got: {ids:?}");
  assert!(ids.contains(&"INV-04"), "the critic exit-code exception");
}
