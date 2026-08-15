//! The clap surface, BUILT from the dispatch table (AC-05.1).
//!
//! clap's builder API rather than its derive macros, and that is the whole
//! design: the table is runtime data, so the surface can be constructed FROM
//! it. A derive-based spine would be a second declaration of the same facts,
//! kept in step by whoever remembered -- which is the drift AC-05.1 forbids.
//! Here the table IS the surface, and `dispatch_ssot.rs` proves it in both
//! directions.
//!
//! **INV-02 lives here.** v2 exits 1 on every usage error; clap exits 2. D17
//! carries v2's codes over, so the override is applied once, at the framework
//! layer, rather than discovered as a hundred red conformance tests.

use clap::{Arg, ArgAction, Command};

use crate::dispatch::{self, Entry, Table};

/// v2's exit codes (INV-04): success, failure, and the `intent critic` 2.
pub const EXIT_OK: i32 = 0;
pub const EXIT_ERROR: i32 = 1;

/// Build the whole surface from the table.
pub fn build(table: &Table) -> Command {
  let mut root = Command::new("intent")
    .version(env!("CARGO_PKG_VERSION"))
    .about("Intent: steel threads, work packages and the acceptance contract")
    // v2 prints its own usage block and exits 1; clap's help/version exit 0
    // and write to stdout, which is the one place we take clap's behaviour
    // deliberately (INV-07 records that v2's `--help` reporting failure on 10
    // of 27 commands is a DEFECT, not a contract).
    .subcommand_required(false)
    .arg_required_else_help(false)
    .disable_help_subcommand(true);

  for family in &table.families {
    let Some(family_entry) = family.entries.iter().find(|e| e.verb().is_none()) else {
      continue;
    };
    if !family_entry.is_shipped() {
      continue;
    }

    let mut cmd = Command::new(family.name.clone()).about(family_entry.help.clone());
    let verbs: Vec<&Entry> = family
      .entries
      .iter()
      .filter(|e| e.verb().is_some() && e.is_shipped())
      .collect();

    for entry in &verbs {
      cmd = cmd.subcommand(leaf(entry));
    }
    if !verbs.is_empty() {
      // **THE FAMILY'S OWN FLAGS AND POSITIONALS ARE ATTACHED HERE**, and
      // before this they were not: `with_args` was called only on the verbless
      // branch, so a flag declared on the family row existed on every leaf and
      // nowhere else. `todo` declares `--json`; `intent todo --json` exited 1
      // while `intent todo list --json` worked. The flag existed everywhere
      // except on the command that declares it (ic, measured).
      //
      // Not routed through `with_args`, deliberately: that function also
      // expands a slot's `values` into subcommands, and here the verbs are
      // sibling ENTRIES rather than slot values, so it would build them twice.
      cmd = flags(positionals(cmd, family_entry), family_entry);

      // **`subcommand_required` COMES FROM THE DECLARED ARITY**, and was
      // hardcoded `true`. `arity: "0..1"` with a default verb means the bare
      // command is legal -- v2 exits 0 on `intent todo`, v3 exited 1, on 8 of
      // 8 reachable families that declare it. The rule was already implemented
      // correctly three lines away in `with_args` and hardcoded wrongly here:
      // **one rule, two implementations, and only one of them right** is the
      // Highlander failure rather than a typo.
      let required = family_entry
        .args
        .iter()
        .find(|a| a.kind == "subcommand")
        .is_none_or(|slot| slot.arity == "1");
      cmd = cmd
        .subcommand_required(required)
        .arg_required_else_help(false);
    } else {
      cmd = with_args(cmd, family_entry);
    }
    root = root.subcommand(cmd);
  }

  // Commands v3 adds. They are top-level families with no sibling verbs, so
  // they take the same path a verbless family takes -- from the operator's
  // side `intent search` is a command or it is not, and where the table
  // recorded it makes no difference to that.
  for entry in &table.new_surface {
    if !entry.is_shipped() {
      continue;
    }
    root = root.subcommand(with_args(
      Command::new(entry.path.clone()).about(entry.help.clone()),
      entry,
    ));
  }
  root
}

/// One leaf verb, with its positional arguments and flags.
fn leaf(entry: &Entry) -> Command {
  let verb = entry.verb().unwrap_or(&entry.path);
  with_args(
    Command::new(verb.to_string()).about(entry.help.clone()),
    entry,
  )
}

fn with_args(mut cmd: Command, entry: &Entry) -> Command {
  // A `subcommand` pseudo-arg carrying VALUES is the surface's third level:
  // `intent claude skills install` is `claude skills` with `install` in its
  // verb slot. Build those values as real subcommands, and hang the entry's
  // remaining positionals off each of them -- `install` takes the skill name,
  // the parent does not.
  //
  // Skipping them (as this did) cost more than the missing verbs. Where a
  // free-form positional sat beside the slot -- `claude skills` declares
  // `name` at arity `0..n` -- it silently swallowed whatever was typed, so
  // `intent claude skills bogus-verb` was ACCEPTED. A surface that accepts an
  // invented verb is a No Silent Errors failure, not a gap.
  if let Some(slot) = entry
    .args
    .iter()
    .find(|a| a.kind == "subcommand" && !a.values.is_empty())
  {
    for value in &slot.values {
      let mut leaf = Command::new(value.clone());
      leaf = positionals(leaf, entry);
      leaf = flags(leaf, entry);
      cmd = cmd.subcommand(leaf);
    }
    // `arity: "1"` means the slot must be filled; `0..1` means the bare
    // command is legal and does something of its own.
    return flags(cmd.subcommand_required(slot.arity == "1"), entry);
  }

  cmd = positionals(cmd, entry);
  flags(cmd, entry)
}

fn positionals(mut cmd: Command, entry: &Entry) -> Command {
  for arg in &entry.args {
    // A valueless `subcommand` pseudo-arg describes a family's verb slot,
    // which is filled by sibling entries rather than by a positional.
    if arg.kind == "subcommand" {
      continue;
    }
    let required = arg.arity == "1";
    // `0..n` is the table's open-ended spelling and carries neither `+` nor
    // `*`, so a check for those two alone read it as a single value.
    let multiple = arg.arity.contains('+') || arg.arity.contains('*') || arg.arity.ends_with('n');
    let mut a = Arg::new(arg.name.clone())
      .required(required)
      .value_name(arg.name.to_uppercase());
    a = if multiple {
      a.action(ArgAction::Append).num_args(1..)
    } else {
      a.action(ArgAction::Set)
    };
    cmd = cmd.arg(a);
  }
  cmd
}

fn flags(mut cmd: Command, entry: &Entry) -> Command {
  for flag in &entry.flags {
    // `help` spellings are clap's own; re-declaring them collides. This is a
    // fact about clap rather than a policy, so it stays even though the table
    // now also marks those rows `intrinsic` -- `ships()` would skip them
    // anyway, and the day it does not, this is what stops the collision.
    if flag
      .spellings
      .iter()
      .any(|s| s == "--help" || s == "-h" || s == "help")
    {
      continue;
    }
    let short = flag
      .spellings
      .iter()
      .find(|s| s.len() == 2 && s.starts_with('-') && !s.starts_with("--"))
      .and_then(|s| s.chars().nth(1));
    // **A SHORT-ONLY FLAG IS BUILT, NOT DROPPED.** This was `let Some(long) =
    // ... else { continue }`, so a flag with no long spelling vanished with no
    // diagnostic -- three `keep` flags declared in the table and present in no
    // surface (`claude subagents -v`, `claude skills -v`, `fileindex -r`).
    // IN-AG-NO-SILENT-001, three times, and invisible from either end: the
    // table said it shipped and the binary said no such flag.
    let long = flag
      .spellings
      .iter()
      .find(|s| s.starts_with("--"))
      .map(|s| s.trim_start_matches('-').to_string());
    // A flag with neither spelling cannot be built at all, and the table and
    // the spine disagree about what exists. Refusing is the only honest move:
    // continuing here is what hid the three above.
    let id = match (&long, short) {
      (Some(long), _) => long.clone(),
      (None, Some(short)) => short.to_string(),
      (None, None) => panic!(
        "dispatch table: a flag on `{}` declares no usable spelling ({:?}); the table claims a \
         flag the spine cannot build",
        entry.path, flag.spellings
      ),
    };
    let mut a = Arg::new(id).help(flag.help.clone());
    if let Some(long) = long {
      a = a.long(long);
    }
    if let Some(short) = short {
      a = a.short(short);
    }
    a = if flag.kind == "bool" {
      a.action(ArgAction::SetTrue)
    } else {
      a.action(ArgAction::Set)
    };
    cmd = cmd.arg(a);
  }
  cmd
}

/// Parse argv, applying INV-02: a usage error exits 1 in v2's voice.
///
/// clap's own rendering is replaced rather than reformatted. Its message shape
/// ("error: unexpected argument '--x' found", plus a usage block and a tip) is
/// a different contract from v2's single `error: ...` line, and the BATS estate
/// asserts v2's. Help and version are the exception: they are not failures, so
/// they print and exit 0.
pub fn parse(argv: Vec<String>) -> Result<clap::ArgMatches, i32> {
  let table = dispatch::table();
  match build(&table).try_get_matches_from(argv) {
    Ok(matches) => Ok(matches),
    Err(e) => {
      use clap::error::ErrorKind;
      match e.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
          print!("{e}");
          Err(EXIT_OK)
        }
        _ => {
          eprintln!("error: {}", first_line(&e.render().to_string()));
          Err(EXIT_ERROR)
        }
      }
    }
  }
}

/// clap renders a multi-line block; v2 speaks one line. Take the message and
/// drop its `error: ` prefix so ours is not doubled.
fn first_line(rendered: &str) -> String {
  rendered
    .lines()
    .next()
    .unwrap_or("invalid usage")
    .trim_start_matches("error: ")
    .trim()
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_surface_carries_every_shipped_family() {
    let table = dispatch::table();
    let root = build(&table);
    let names: Vec<String> = root
      .get_subcommands()
      .map(|c| c.get_name().to_string())
      .collect();
    for expected in ["st", "wp", "ac", "at", "issues", "todo", "doctor"] {
      assert!(names.contains(&expected.to_string()), "missing {expected}");
    }
    assert!(
      !names.contains(&"organize".to_string()),
      "a ratified retire does not reach the surface"
    );
  }

  #[test]
  fn a_first_line_keeps_one_error_prefix() {
    assert_eq!(
      first_line("error: unexpected argument '--x'\n\nUsage: intent st list"),
      "unexpected argument '--x'"
    );
  }
}
