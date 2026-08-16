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
///
/// **The third one was named in this comment and never declared** -- which is
/// issue 0038. Every v3 failure took `EXIT_ERROR`, so a command this build has
/// not implemented yet reported itself in the code that means "your code is
/// bad", and the consumer that reads that code -- the shipped pre-commit gate
/// -- blocked the commit and printed a remedy naming findings that do not
/// exist. Measured against v2 in this repository rather than inferred:
///
/// | event                                    | v2 | v3 |
/// | ---------------------------------------- | -- | -- |
/// | success                                  | 0  | 0  |
/// | unknown subcommand                       | 1  | 1  |
/// | usage error (a required argument absent) | 1  | 1  |
/// | a negative verdict from a gate           | 1  | 1  |
/// | the tooling cannot answer                | 2  | 2  |
///
/// **Two of the three cases issue 0038 proposed separating are already right
/// and have to stay 1**, because that is what v2 does and D17 carries v2's
/// codes over. Only the last row was wrong, and v2 reaches it by one route
/// (`intent critic` with a language it does not have) where v3 reaches it by
/// another (a table row this build has not wired), which is the same event:
/// the question was well formed and this binary cannot answer it.
pub const EXIT_OK: i32 = 0;
pub const EXIT_ERROR: i32 = 1;
/// **EVERY SHIPPED CONSUMER OF THIS BINARY'S EXIT CODES, ENUMERATED ONCE**
/// (issue 0043, swept 2026-08-16 at `caff2d17`). A comment here naming only the
/// pre-commit gate is how 0043 happened: `2` was chosen against the one caller
/// in view, and a different caller spells the opposite decision with the same
/// number.
///
/// | consumer                                   | invokes                    | what `2` MEANS there | action              |
/// | ------------------------------------------ | -------------------------- | ------------------- | ------------------- |
/// | `.claude/settings.json` `UserPromptSubmit`  | `claude hook require-in-session` | deliberate refusal | **blocks the prompt** |
/// | `.claude/settings.json` `SessionStart`      | `claude hook session-context`    | advisory | does NOT block -- **and the hook never runs** |
/// | Claude Code `Stop` (not wired to `intent`)  | --                         | refuse to stop      | 24s hang, zero output |
/// | `hooks/pre-commit.sh:175` (critic loop)     | `critic <lang>`            | tooling unavailable | **fails open**, commit proceeds |
/// | `.claude/scripts/post-tool-advisory.sh:73`  | `critic <lang>`            | nothing -- `\|\| true` | advisory suppressed |
/// | `hooks/pre-commit.sh:104`                   | `info`                     | **nothing at all**  | see below |
///
/// **The first three rows are MEASURED against Claude Code 2.1.233** (vc, five
/// arms, 2026-08-16), not read off a contract. Two of them matter beyond the
/// count. `SessionStart` failing open is a finding rather than a relief: the
/// session opens, `session-context.sh` never executes, and the project context
/// plus the `/in-session` reminder -- the documented entry to the whole gate --
/// **silently do not arrive**, so the migrated experience was a contextless
/// session that then refused its first prompt. And `Stop` is clean only by
/// accident of being a bare `echo`; routing it through `intent claude hook`,
/// which is the obvious tidying move, would arm a fourth distinct failure from
/// this one constant.
///
/// **So `2` has four measured meanings across four contracts, and there is no
/// value of this constant that is right for four contracts that disagree.**
/// The fix is not a better number, it is per-caller -- and for the Claude Code
/// side it is already structural rather than a choice: **`claude hook` is the
/// single door Claude Code reaches this binary through, and it DELEGATES.** It
/// execs the script, so every `2` a hook consumer sees is the script's own
/// deliberate one; no path inside `render::hook` produces `Unavailable`, which
/// `the_hook_door_never_answers_in_the_callers_refusal_code` holds it to.
///
/// **The last row is the one that changes the shape of the problem, and it is
/// why 0042 could never have been fixed by choosing a better number.** That
/// consumer does not read the exit code; it parses `INTENT_HOME:` out of
/// STDOUT and ignores the status entirely. An unimplemented command writes
/// nothing to stdout, so the path came back empty and both whiteboard guards
/// stopped enforcing -- at ANY exit code. **Some callers have a stdout
/// contract, not an exit-code contract**, and a command they depend on is
/// unfixable from this constant in either direction.
///
/// One more, from the same measurement, because it defeats the obvious
/// detector: **on a blocked prompt the `claude` process itself exits 0.** The
/// block is in-band, in the output stream. Any wrapper checking the process
/// status sees success while the model never saw the prompt -- a second silent
/// failure sitting in the exact layer somebody would reach for to catch the
/// first. Assert on OUTPUT there, never on the code.
///
/// Two things deliberately NOT in the table, because a record is worth only
/// what its exclusions are worth. **`bin/.devbin/cmd/build.d/release:373` calls
/// `$PROJECT_ROOT/bin/intent doctor` by absolute path** -- that is v2's frozen
/// shell entry point, not this binary, so it is a caller of `intent` and not a
/// consumer of these codes; it would become one the day that path is
/// repointed. And **`int prepush` does not invoke the binary at all** (checked
/// rather than inherited: its only occurrence of the word is devbin's own usage
/// line), so the fourth caller named on the issue does not hold as stated
/// though its premise -- that the list is longer than two -- does.
///
/// **The general rule the table exists to make unmissable: an exit code is a
/// property of the CALLER's contract, not of this tool.** Before changing any
/// value here, or before routing a new failure to one, read the column that
/// says what each consumer does with it.
pub const EXIT_UNAVAILABLE: i32 = 2;

/// How a command failed, and therefore which code reports it.
///
/// The error channel was a bare `String`, which answers "what do I print" and
/// nothing else. **Two further questions were already riding on it out of
/// band**: an EMPTY string meant "the verdict is on stdout, print nothing
/// more" (four sites -- the close gate, `at lint`, `doctor`, `sync`), and
/// there was no way at all to say "this build cannot answer", which is why
/// that case borrowed the code for "the answer is no".
///
/// A sentinel value standing in for a kind is readable exactly once, by
/// whoever wrote it. Naming the kinds puts the exit code beside the meaning
/// instead of leaving it to the caller to remember.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
  /// The command ran and the answer is no. Exit 1, message to stderr.
  Error(String),
  /// The verdict is already on stdout, where machines read it. Exit 1, silent.
  Verdict,
  /// This build cannot answer the question at all. Exit 2 -- v2's code for an
  /// unavailable tool, which consumers written against v2 treat as fail-open
  /// rather than as a negative verdict about their own work.
  Unavailable(String),
}

impl Failure {
  pub fn code(&self) -> i32 {
    match self {
      Failure::Error(_) | Failure::Verdict => EXIT_ERROR,
      Failure::Unavailable(_) => EXIT_UNAVAILABLE,
    }
  }

  /// What to write to stderr, if anything. `None` is not "the empty string" --
  /// it is the verdict case, which has already written to stdout.
  pub fn message(&self) -> Option<&str> {
    match self {
      Failure::Error(m) | Failure::Unavailable(m) => Some(m),
      Failure::Verdict => None,
    }
  }
}

/// So `?` still carries the message-only helpers (`arg`, `open`, `fail`) that
/// have no opinion about the code, and they keep the default.
impl From<String> for Failure {
  fn from(message: String) -> Self {
    Failure::Error(message)
  }
}

impl From<&str> for Failure {
  fn from(message: &str) -> Self {
    Failure::Error(message.to_string())
  }
}

/// Build the whole surface from the table.
pub fn build(table: &Table) -> Command {
  let mut root = Command::new("intent")
    .version(env!("CARGO_PKG_VERSION"))
    // FROM THE TABLE, like every other help string on the surface (EXP-08).
    // This was the ONE `.about("...")` literal in the CLI: every family, entry,
    // verb and flag below already reads its help from the table, so the root was
    // the single place the SSOT claim was false -- and it is the first line an
    // agent reads from `intent --help`. It also went through a different code
    // path from the rest, so `help_text_is_the_tables_help_text()` could not see
    // it: that test spot-checks one command on the argument that the MECHANISM
    // carries the others, and the root was not on the mechanism.
    .about(table.root_help.clone())
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

    let mut cmd = command_for(&family.name, family_entry);
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
    root = root.subcommand(with_args(command_for(&entry.path, entry), entry));
  }
  root
}

/// A clap command built from a table entry: its name, its help, and the v2
/// spellings it still has to answer to.
///
/// **Every Entry-backed Command is built here**, and that is the point rather
/// than tidiness. Issue 0039 was a property that had to hold at three separate
/// construction sites -- the family, the leaf verb, and the new-surface top
/// level -- and held at none of them, because there was no single place where
/// "a Command made from an Entry" was expressed. `.about(entry.help)` was
/// already being repeated at all three; the aliases would have been the fourth
/// thing to remember three times.
///
/// Callers filter on `is_shipped()` before reaching here, which is what keeps
/// `st organise` retired: it is an alias on a `retire` row, and registering it
/// would bring a withdrawn command back through its old spelling while the
/// canonical one stays gone.
fn command_for(name: &str, entry: &Entry) -> Command {
  let mut cmd = Command::new(name.to_string()).about(entry.help.clone());
  for alias in entry.alias_verbs() {
    // VISIBLE, because v2 lists them: `done|notdone <stid> <atid>  Aliases for
    // green | red`. A hidden alias would work and be undiscoverable, which is
    // a different way of not shipping it.
    cmd = cmd.visible_alias(alias.to_string());
  }
  cmd
}

/// One leaf verb, with its positional arguments and flags.
fn leaf(entry: &Entry) -> Command {
  let verb = entry.verb().unwrap_or(&entry.path);
  with_args(command_for(verb, entry), entry)
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
    // **BOTH predicates now come from the table's own type** (ic, measured
    // 2026-08-16). The inline `arity == "1"` here was false for `1..n`, which
    // declares a MINIMUM of one -- so `intent lang init` with no language
    // parsed cleanly and fell through to the renderer, where v2 refuses it
    // outright (`bin/intent_lang:251`). Two rows were affected, `lang init` and
    // `lang remove`, and both are latent only because `lang` is unwired: the
    // day WP-07 wires it, the renderer is handed an empty list.
    //
    // The `multiple` half was already correct and moves for the same reason
    // rather than a different one. Two readers of one grammar drift, and this
    // one drifted in the half nobody had a test for -- ic's own first test for
    // `required()` asserted the wrong answer and PASSED, because it was
    // written by reading the implementation instead of the meaning of `<x>`.
    let mut a = Arg::new(arg.name.clone())
      .required(arg.required())
      .value_name(arg.name.to_uppercase());
    a = if arg.repeated() {
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
    // **THE DISPOSITION, HONOURED AT THE LEVEL IT IS DECLARED** (EXP-05). The
    // entry-level check drops a retired COMMAND; without this, a retired or
    // undecided flag on a SHIPPED command was built regardless, so `--help`
    // advertised what no renderer would answer. See [`Flag::ships`] for why
    // `pending` is out rather than in.
    if !flag.ships() {
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
    // **The authored placeholder, where clap otherwise prints the argument's
    // internal id.** `--evidence <evidence>` was clap's fallback showing our
    // own identifier back to the reader; the table says `<ref>`. 35 rows
    // declared one and the surface showed none of them (issue 0035).
    //
    // The delimiters are stripped because clap adds its own: the table writes
    // `<text>` and `[dir]` as the reader should SEE them, and passing either
    // through unstripped renders `<<text>>`. A value with no delimiters --
    // `open|closed|all` -- is passed whole and clap wraps it once.
    if let Some(value) = &flag.value
      && flag.kind != "bool"
    {
      let (name, repeated) = placeholder(value);
      a = a.value_name(name.to_string());
      // **`...` in the table is a statement about ARITY, not decoration**, and
      // it is the only shape here that is. `--files <path> ...` means a list,
      // so the row is wired to `num_args`, and clap then renders the ellipsis
      // itself. Passing the whole string through as a name instead is what
      // produced `--files <<path> ...>` -- the double-bracket rendering this
      // strip exists to prevent, showing up on the one row that was not a plain
      // `<name>`.
      if repeated {
        a = a.num_args(1..);
      }
    }
    // **Declared requiredness reaches the parser**, so the usage line stops
    // presenting a required flag as optional. It is belt-and-braces rather than
    // the only thing standing there: the facade guards the same values and
    // catches the case this cannot, an EMPTY value that satisfies clap.
    if flag.required {
      a = a.required(true);
    }
    // A default makes the flag always-present downstream, so it is applied only
    // where the table names a literal one.
    if let Some(default) = &flag.default
      && flag.kind != "bool"
    {
      a = a.default_value(default.clone());
    }
    cmd = cmd.arg(a);
  }
  cmd
}

/// A declared `value` split into the name clap should print and whether the row
/// describes a REPEATED value.
///
/// Three shapes, because the table writes three and collapsing them loses
/// something each time:
///
/// - `<name>` / `[dir]` -- the delimiters are the table showing the reader what
///   they will see, and clap adds its own pair, so they come off.
/// - `<path> ...` -- a name plus an arity. The name comes off the same way and
///   the ellipsis becomes `num_args`, which is where the fact belongs.
/// - `open|closed|all` -- no delimiters, so the whole string is the name and
///   clap wraps it once. The author wrote the alternation to be read as one.
fn placeholder(value: &str) -> (&str, bool) {
  let trimmed = value.trim();
  let repeated = trimmed.ends_with("...");
  let head = trimmed.trim_end_matches("...").trim();
  let inner = head
    .strip_prefix('<')
    .and_then(|v| v.strip_suffix('>'))
    .or_else(|| head.strip_prefix('[').and_then(|v| v.strip_suffix(']')))
    .unwrap_or(head);
  (inner, repeated)
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
///
/// **Some clap errors put the whole message on the first line and some put a
/// HEADER there and the content beneath it**, indented, one item per line:
/// "the following required arguments were not provided:" names nothing at all
/// on its own. Taking line one was correct for every error this binary could
/// produce until `required` reached the parser -- **at which point the one
/// message that most needed to name a flag became the one that named
/// nothing.** A latent hole in a helper, opened by a change three files away,
/// and the output was a complete sentence promising information it had
/// dropped.
///
/// So a header ending in `:` absorbs the indented items that follow it, up to
/// the blank line before clap's usage block, and they join onto the one line v2
/// speaks.
fn first_line(rendered: &str) -> String {
  let mut lines = rendered.lines();
  let head = lines
    .next()
    .unwrap_or("invalid usage")
    .trim_start_matches("error: ")
    .trim()
    .to_string();
  if !head.ends_with(':') {
    return head;
  }
  let items: Vec<&str> = lines
    .take_while(|l| !l.trim().is_empty())
    .map(str::trim)
    .filter(|l| !l.is_empty())
    .collect();
  if items.is_empty() {
    return head;
  }
  format!("{head} {}", items.join(", "))
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

  /// **Every slot the table declares mandatory is mandatory ON THE SURFACE.**
  ///
  /// `dispatch::Arg::required()` already had a test; nothing checked that the
  /// spine CALLED it, and it did not -- an inline `arity == "1"` here read
  /// `1..n` as optional, so `intent lang init` with no language parsed cleanly
  /// where v2 refuses it. A correct predicate with a second, wrong copy at the
  /// only call site is indistinguishable from having no predicate at all.
  ///
  /// Driven through `try_get_matches_from` rather than by re-reading the table,
  /// because the question is what the BUILT surface does. Asserting over every
  /// declared row rather than the two measured ones: a fix that repaired
  /// `lang init` by name would pass a two-row test and leave the mechanism.
  #[test]
  fn a_slot_the_table_declares_mandatory_is_refused_when_it_is_absent() {
    let table = dispatch::table();
    let mut checked = 0;
    let mut accepted = Vec::new();

    for family in &table.families {
      for entry in &family.entries {
        if !entry.is_shipped() {
          continue;
        }
        // Only rows whose FIRST positional is mandatory: with a leading
        // optional slot, clap has no missing-argument to report.
        let Some(first) = entry.args.iter().find(|a| a.kind != "subcommand") else {
          continue;
        };
        if !first.required() {
          continue;
        }
        checked += 1;

        let argv: Vec<String> = std::iter::once("intent".to_string())
          .chain(entry.path.split_whitespace().map(str::to_string))
          .collect();
        if build(&table).try_get_matches_from(&argv).is_ok() {
          accepted.push(format!(
            "`{}` declares <{}> at arity {:?} and parsed with it absent",
            entry.path, first.name, first.arity
          ));
        }
      }
    }

    assert!(
      checked > 20,
      "only {checked} rows declared a mandatory first positional -- the walk is broken, and a broken walk passes this test vacuously"
    );
    assert!(
      accepted.is_empty(),
      "the surface accepted an invocation missing an argument the table declares mandatory:\n  {}\n\nThe renderer would be handed an empty value where v2 \
       refuses the invocation outright.",
      accepted.join("\n  ")
    );
  }

  #[test]
  fn a_first_line_keeps_one_error_prefix() {
    assert_eq!(
      first_line("error: unexpected argument '--x'\n\nUsage: intent st list"),
      "unexpected argument '--x'"
    );
  }

  /// **A header that names nothing absorbs the list beneath it.**
  ///
  /// clap puts required-argument errors on two levels: a sentence ending in
  /// `:`, then the arguments, indented, one per line. Taking line one alone
  /// printed "the following required arguments were not provided:" and stopped
  /// -- a complete sentence promising information it had just dropped, on the
  /// one error whose whole job is to name a flag.
  #[test]
  fn a_header_ending_in_a_colon_carries_its_items_onto_the_one_line() {
    assert_eq!(
      first_line(
        "error: the following required arguments were not provided:\n  --evidence <ref>\n\nUsage: intent ac satisfy --evidence <ref> <STID> <ACID>\n"
      ),
      "the following required arguments were not provided: --evidence <ref>"
    );
    assert_eq!(
      first_line("error: two are missing:\n  --a <x>\n  --b <y>\n\nUsage: ...\n"),
      "two are missing: --a <x>, --b <y>",
      "several items join rather than the first one winning"
    );
    // The usage block is NOT part of the message, and a header with nothing
    // under it stays exactly as it was rather than swallowing what follows.
    assert_eq!(
      first_line("error: something ended in a colon:\n\nUsage: intent st list"),
      "something ended in a colon:"
    );
  }

  #[test]
  fn a_declared_placeholder_is_split_into_a_name_and_an_arity() {
    assert_eq!(placeholder("<text>"), ("text", false));
    assert_eq!(placeholder("[dir]"), ("dir", false));
    assert_eq!(placeholder("<path> ..."), ("path", true));
    // No delimiters: the alternation IS the name, and clap wraps it once.
    assert_eq!(placeholder("open|closed|all"), ("open|closed|all", false));
  }
}
