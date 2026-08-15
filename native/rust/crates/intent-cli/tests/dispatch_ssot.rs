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
  // Both halves of the table describe the surface: `families` is the ported v2
  // surface, `new_surface` the commands v3 adds. A check that knew only the
  // first would read every ADDITION as an undocumented invention -- which is
  // exactly what it did when `search` and `schema` were first wired.
  let known: Vec<&str> = table
    .families
    .iter()
    .map(|f| f.name.as_str())
    .chain(table.new_surface.iter().map(|e| e.path.as_str()))
    .collect();

  let invented: Vec<String> = surface_families()
    .into_iter()
    .filter(|f| !known.contains(&f.as_str()) && f != "help")
    .collect();
  assert!(
    invented.is_empty(),
    "the binary offers commands the table does not describe: {invented:?} -- a surface entry with no table row is a second, undocumented declaration"
  );
}

/// The other direction for the added commands. Without this, `new_surface`
/// could be widened to silence the check above while the command itself never
/// shipped -- the table would describe a surface nobody could reach.
#[test]
fn every_added_command_in_the_table_reaches_the_surface() {
  let table = dispatch::table();
  let surface = surface_families();

  let missing: Vec<String> = table
    .new_surface
    .iter()
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .filter(|p| !surface.contains(p))
    .collect();
  assert!(
    missing.is_empty(),
    "the table declares these added commands and the binary does not offer them: {missing:?}"
  );
  assert!(
    surface.contains(&"search".to_string()) && surface.contains(&"schema".to_string()),
    "AC-06.4 and AC-06.5 name these two by hand, so they are asserted by name as well as by the sweep"
  );
}

/// An unbuilt verb names the work package that OWES it, read from the table.
///
/// The message used to say WP-06 for everything. `daemon` is WP-08's and `mcp`
/// is WP-09's, so that was wrong for two of the six added commands -- and
/// wrong in the confident voice of a fact, which is the kind of wrong that
/// gets believed.
#[test]
fn an_unbuilt_command_names_the_work_package_that_owes_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  let run = |args: &[&str]| {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(args)
      .current_dir(dir.path())
      .output()
      .expect("run the v3 binary");
    String::from_utf8_lossy(&out.stderr).to_string()
  };

  let mcp = run(&["mcp"]);
  assert!(
    mcp.contains("WP-09"),
    "`intent mcp` is WP-09's to build: {mcp}"
  );
  let ingest = run(&["ingest"]);
  assert!(
    ingest.contains("WP-03"),
    "`intent ingest` is WP-03's: {ingest}"
  );
  assert_ne!(
    mcp, ingest,
    "two different owners must not render the same sentence"
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
