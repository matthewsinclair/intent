//! `--default`'s help and `default_declaration`'s predicate say the same thing.
//!
//! **The help string was a FOURTH home for a rule AC-11.3 exists to keep
//! single-homed, and it drifted there because no test could see it** (ic,
//! 2026-08-27). AC-11.3's content is that init, migration and upgrade all reach
//! one function, proven by moving the function and watching every caller move.
//! Prose in the dispatch table is outside that proof by construction.
//!
//! # What it had drifted to, and why that is worse than a stale sentence
//!
//! The help read *"one STEELTHREAD line per thread that is not Completed or
//! Cancelled"*. `default_declaration` implements `status == Wip`, and hv
//! overruled the open definition on 2026-08-26 after seeing what it realised:
//! *"It should ONLY HAVE WIP STs!!!!!"*.
//!
//! **The stale wording was a DEFINITION BY EXCLUSION, which is the exact defect
//! hv's ruling corrected.** "Not Completed or Cancelled" reads as a careful rule
//! and is really a list of what to leave out, so Triage, Not Started and Hold
//! were swept in by default -- the 57-thread realised set hv objected to. The
//! help therefore did not merely disagree with the code, it disagreed in the
//! one direction that reproduces the original bug in the reader's head, and it
//! did so with confidence.
//!
//! # Why two assertions and not one
//!
//! Prose cannot be bound to a predicate by string equality without inventing a
//! spelling both must use, which is a fifth home. So each half is guarded
//! against its own failure:
//!
//! - **The predicate** is driven over EVERY `ThreadStatus` variant, so a change
//!   to what `default_declaration` admits reds here regardless of wording.
//! - **The help** must name the predicate positively and must not reintroduce
//!   exclusion phrasing, which is the shape that drifted last time.
//!
//! Neither alone closes it: the first passes while the help lies, and the
//! second passes while the code changes underneath.

use intentsvcs::intentfiles::default_declaration;
use intentsvcs::model::ThreadStatus;

/// Every variant, so a new one is a compile error here rather than a silent
/// omission. `match` on a sample value is what makes that true -- a `Vec`
/// literal would happily stay stale.
fn every_status() -> Vec<(String, ThreadStatus)> {
  let all = [
    ThreadStatus::Triage,
    ThreadStatus::NotStarted,
    ThreadStatus::Wip,
    ThreadStatus::Hold,
    ThreadStatus::Completed,
    ThreadStatus::Cancelled,
  ];
  // Exhaustiveness: adding a variant to the enum without adding it above fails
  // to compile, because this match has no wildcard arm.
  for status in &all {
    match status {
      ThreadStatus::Triage
      | ThreadStatus::NotStarted
      | ThreadStatus::Wip
      | ThreadStatus::Hold
      | ThreadStatus::Completed
      | ThreadStatus::Cancelled => {}
    }
  }
  all
    .iter()
    .enumerate()
    .map(|(i, s)| (format!("ST{:04}", i + 1), *s))
    .collect()
}

/// `--default`'s help text, as the binary actually ships it.
fn default_flag_help() -> String {
  let table: serde_json::Value =
    serde_json::from_str(intent_cli::dispatch::TABLE).expect("the table parses as JSON");
  let mut found = Vec::new();
  let mut visit = |v: &serde_json::Value| {
    if let Some(flags) = v.get("flags").and_then(|f| f.as_array()) {
      for f in flags {
        // **The key is `spellings`, a LIST, not `name`.** My first walker asked
        // for `name` and found zero flags in a table holding hundreds. It was
        // caught only because this function asserts EXACTLY ONE rather than
        // iterating whatever it found -- a lenient walker would have reported
        // a clean pass over an empty search.
        let is_default = f
          .get("spellings")
          .and_then(|s| s.as_array())
          .map(|a| a.iter().any(|s| s.as_str() == Some("--default")))
          .unwrap_or(false);
        if is_default && let Some(h) = f.get("help").and_then(|h| h.as_str()) {
          found.push(h.to_string());
        }
      }
    }
  };
  for group in ["families", "new_surface"] {
    if let Some(items) = table.get(group).and_then(|g| g.as_array()) {
      for item in items {
        visit(item);
        if let Some(entries) = item.get("entries").and_then(|e| e.as_array()) {
          for e in entries {
            visit(e);
          }
        }
      }
    }
  }
  assert_eq!(
    found.len(),
    1,
    "expected exactly one `--default` help string, found {}: {found:?}",
    found.len()
  );
  found.remove(0)
}

/// The predicate half: only WIP is realised, driven over every status.
#[test]
fn the_default_declaration_admits_wip_and_nothing_else() {
  let text = default_declaration(&every_status());
  let declared: Vec<&str> = text
    .lines()
    .filter_map(|l| l.strip_prefix("STEELTHREAD:"))
    .collect();
  assert_eq!(
    declared,
    vec!["ST0003"],
    "only the WIP thread should be declared, got {declared:?} from:\n{text}"
  );
}

/// The prose half: the help names the rule positively.
#[test]
fn the_help_names_wip_rather_than_describing_what_is_left_out() {
  let help = default_flag_help();
  assert!(
    help.contains("WIP"),
    "`--default`'s help does not name the predicate the code implements: {help}"
  );
  for exclusion in ["not Completed", "not Cancelled", "except", "other than"] {
    assert!(
      !help.contains(exclusion),
      "`--default`'s help defines by exclusion again ({exclusion:?}), which is the shape \
       that swept Triage, Not Started and Hold into the realised set: {help}"
    );
  }
}
