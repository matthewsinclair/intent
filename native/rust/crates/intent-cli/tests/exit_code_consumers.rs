//! **Every place the SHIPPED CANON invokes `intent`, declared with what it does
//! on a non-zero code.**
//!
//! Issue 0044's third proposed fix, and issues 0038, 0042, 0043 and 0045 are
//! the argument for it: **each was diagnosed against whichever consumer
//! happened to be in view.** The count of consumers was revised upward four
//! times -- one, then four, then "at least six", then eleven -- and every
//! revision was somebody noticing another one by hand. vc's words: *the twelfth
//! will be written by someone who has not read this table.*
//!
//! So the table is not prose. **A file in the shipped canon that names `intent`
//! must be classified here, and one that is not reds this test** -- which is
//! the only mechanism that reaches a consumer nobody has thought of yet.
//!
//! **Scope is the SHIPPED CANON, and that is a boundary rather than a
//! shortcut.** `lib/templates/` is what every consumer project receives, so a
//! wrong assumption here is replicated across the fleet. `bin/.devbin/` has
//! three more callers (vc enumerated them in 0045) and is dc's lane; a guard
//! that reds a peer's work without their agreement is a lane violation wearing
//! diligence, so that half is offered rather than imposed.
//!
//! **Markdown is excluded BY CONSTRUCTION, not by taste.** Documentation names
//! commands constantly -- 93 of the 99 matches across the canon are prose in
//! `.md` -- and none of it reads an exit code. A roster covering them would
//! fire on every documentation edit, and a check that fires on a routine state
//! is the one people learn to ignore.
//!
//! **The residual is stated rather than hidden**: the key is (file, command
//! family), so a SECOND invocation of an already-declared family in an
//! already-declared file is not caught. A new file is, and a new family in an
//! existing file is. That is a much smaller hole than the one it replaces, and
//! closing it would mean asserting a count -- which churns on every message
//! reworded near a call.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use testkit::workspace_root;

/// What a consumer does when `intent` answers non-zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Policy {
  /// It runs `intent` and acts on the result.
  Invokes(&'static str),
  /// The name appears in a message, never as a command. Classified rather than
  /// filtered: "is this line prose" has no mechanical answer, and a heuristic
  /// that gets it mostly right is how a check earns its reputation for noise.
  Names(&'static str),
}

/// THE ROSTER. One row per (shipped canon file, `intent` family).
const CONSUMERS: &[(&str, &str, Policy)] = &[
  (
    ".claude/settings.json",
    "claude",
    Policy::Invokes(
      "TWO hooks. `SessionStart` -> session-context: non-zero is advisory, the session opens and the \
       context silently does not arrive. `UserPromptSubmit` -> require-in-session: **2 BLOCKS THE PROMPT**, \
       and the block cannot be cleared from inside the session. Both measured against Claude Code 2.1.233. \
       This is issue 0043, and what makes it safe is structural: `claude hook` DELEGATES to the script, so \
       every 2 a hook consumer sees is the script's own.",
    ),
  ),
  (
    "hooks/pre-commit.sh",
    "critic",
    Policy::Invokes(
      "The gate loop. 0 = clean, **1 = findings, BLOCKS THE COMMIT**, 2+ = invocation error and fails open. \
       Issue 0038 is this row: every v3 failure took 1, so an unimplemented command read as findings that \
       did not exist. Issue 0045 is this row too, from the other side -- anything answering 1 for a \
       PROJECT-STATE reason blocks every commit, which is why `critic` must not be built on `Facade::open`.",
    ),
  ),
  (
    "hooks/pre-commit.sh",
    "info",
    Policy::Invokes(
      "**A STDOUT CONTRACT, AND NO EXIT-CODE CONTRACT AT ALL.** It parses `INTENT_HOME:` out of stdout with \
       `sed` and builds the whiteboard guards' paths from the result. Issue 0042: with the value empty both \
       guards silently stopped enforcing, at ANY exit code -- so no choice of constant could have fixed it. \
       The status IS captured now (`wb_info_rc`) and printed, though nothing branches on it.",
    ),
  ),
  (
    ".claude/scripts/post-tool-advisory.sh",
    "critic",
    Policy::Invokes(
      "`|| true` plus `2>/dev/null`: the status is discarded and so is stderr. **The hedge outlived the \
       condition it hedged for** -- its comment says the `|| true` exists because `intent critic` had not \
       landed yet. Under v3 the advisory is permanently silent with no indication, and the redirect will \
       keep hiding real errors once `critic` is built. Latent rather than live: it is wired into neither \
       shipped `settings.json`.",
    ),
  ),
  (
    "hooks/pre-commit.sh",
    "gate",
    Policy::Names("the gate's own messages name themselves `intent gate:` / `intent critic gate:`"),
  ),
  (
    "hooks/module_check_hook.json",
    "modules",
    Policy::Names(
      "an advisory `echo` telling the author to run `intent modules check` -- the hook itself runs `grep`",
    ),
  ),
];

/// The shipped canon: what a consumer project receives.
fn canon_root() -> PathBuf {
  workspace_root()
    .parent()
    .and_then(Path::parent)
    .expect("the rust workspace sits two levels under the Intent install")
    .join("lib/templates")
}

/// Every (canon-relative path, `intent` family) an EXECUTABLE canon file names.
///
/// Full-line shell comments are skipped: they are documentation that happens to
/// live in a script, and the classification above already carries what the
/// comments say.
fn found() -> BTreeSet<(String, String)> {
  let root = canon_root();
  let mut out = BTreeSet::new();
  let mut stack = vec![root.clone()];
  while let Some(dir) = stack.pop() {
    let Ok(entries) = std::fs::read_dir(&dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        stack.push(path);
        continue;
      }
      let executable = matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("sh" | "json" | "yml")
      );
      if !executable {
        continue;
      }
      let Ok(text) = std::fs::read_to_string(&path) else {
        continue;
      };
      let rel = path
        .strip_prefix(&root)
        .expect("under the canon root")
        .display()
        .to_string();
      for line in text.lines() {
        if line.trim_start().starts_with('#') {
          continue;
        }
        for (i, _) in line.match_indices("intent ") {
          // `bin/intent ...` and `.../intent ...` are a different binary being
          // invoked by path, which is a caller of v2 rather than a consumer of
          // these codes.
          if line[..i].ends_with('/') {
            continue;
          }
          let rest = &line[i + "intent ".len()..];
          let family: String = rest
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '-')
            .collect();
          if family.is_empty() {
            continue;
          }
          out.insert((rel.clone(), family));
        }
      }
    }
  }
  out
}

/// **The fixture proves itself.** A scan that found nothing would agree with
/// every assertion below and would look exactly like a clean estate.
#[test]
fn the_scan_reaches_the_shipped_canon() {
  let found = found();
  assert!(
    found.len() >= 4,
    "the canon scan found {} (file, command) pairs under {}. Every assertion here iterates that set, so a scan reaching nothing passes silently -- which is the \
     shape of the problem this file exists for",
    found.len(),
    canon_root().display()
  );
}

/// **A canon file that names `intent` and is not on the roster REDS.**
///
/// This is the whole point: the twelfth consumer is written by someone who has
/// not read the table, and nothing except a check can put the table in front of
/// them at the moment they write it.
#[test]
fn every_shipped_consumer_is_declared() {
  let declared: BTreeSet<(String, String)> = CONSUMERS
    .iter()
    .map(|(f, c, _)| (f.to_string(), c.to_string()))
    .collect();

  let found = found();
  let undeclared: Vec<&(String, String)> =
    found.iter().filter(|k| !declared.contains(*k)).collect();

  assert!(
    undeclared.is_empty(),
    "these shipped-canon files invoke or name `intent` and are not declared in CONSUMERS:\n  {}\n\n**Declare it, with what it does when `intent` answers \
     non-zero.** Four issues -- 0038, 0042, 0043, 0045 -- were each diagnosed against whichever consumer happened to be in view, and the count of consumers was \
     revised upward four times by people noticing them one at a time. If it only NAMES the command in a message, say so with `Policy::Names` -- that is a \
     one-line declaration and it is what stops the next reader wondering.",
    undeclared
      .iter()
      .map(|(f, c)| format!("{f}  ->  intent {c}"))
      .collect::<Vec<_>>()
      .join("\n  ")
  );
}

/// **The mirror: a roster row for something that is no longer there.**
///
/// A stale declaration is worse than a missing one, because it reads as
/// coverage. `post-tool-advisory.sh`'s own `|| true` is the cautionary case in
/// this very file -- a hedge that outlived its condition and now hides errors
/// it was never meant to hide.
#[test]
fn no_roster_row_describes_a_consumer_that_is_gone() {
  let found = found();
  let stale: Vec<String> = CONSUMERS
    .iter()
    .map(|(f, c, _)| (f.to_string(), c.to_string()))
    .filter(|k| !found.contains(k))
    .map(|(f, c)| format!("{f}  ->  intent {c}"))
    .collect();

  assert!(
    stale.is_empty(),
    "these roster rows describe a consumer the shipped canon no longer has:\n  {}\n\nRemove the row. A table that lists consumers which do not exist is one \
     nobody trusts the rest of",
    stale.join("\n  ")
  );
}

/// **Every declaration says something.** A roster whose reasons are empty is a
/// list of filenames, and the reason is the entire value here -- the exit code
/// is a property of the CALLER's contract, so the contract is what has to be
/// written down.
#[test]
fn every_declaration_states_what_a_non_zero_code_does() {
  for (file, cmd, policy) in CONSUMERS {
    let text = match policy {
      Policy::Invokes(t) | Policy::Names(t) => t,
    };
    assert!(
      text.len() > 40,
      "`{file} -> intent {cmd}` is declared with {text:?}, which does not say what happens on a non-zero code. The filename was never the hard part"
    );
  }
}
