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
      cmd = cmd.subcommand_required(true).arg_required_else_help(false);
    } else {
      cmd = with_args(cmd, family_entry);
    }
    root = root.subcommand(cmd);
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
  for arg in &entry.args {
    // A `subcommand` pseudo-arg in the table describes the family's verb slot,
    // which clap models as a subcommand rather than a positional.
    if arg.kind == "subcommand" {
      continue;
    }
    let required = arg.arity == "1";
    let multiple = arg.arity.contains('+') || arg.arity.contains('*');
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

  for flag in &entry.flags {
    // `help` spellings are clap's own; re-declaring them collides.
    if flag
      .spellings
      .iter()
      .any(|s| s == "--help" || s == "-h" || s == "help")
    {
      continue;
    }
    let Some(long) = flag
      .spellings
      .iter()
      .find(|s| s.starts_with("--"))
      .map(|s| s.trim_start_matches('-').to_string())
    else {
      continue;
    };
    let mut a = Arg::new(long.clone()).long(long).help(flag.help.clone());
    if let Some(short) = flag
      .spellings
      .iter()
      .find(|s| s.len() == 2 && s.starts_with('-'))
      .and_then(|s| s.chars().nth(1))
    {
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
