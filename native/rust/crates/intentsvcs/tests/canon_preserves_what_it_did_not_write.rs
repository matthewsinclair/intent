//! `intent claude upgrade` -- **the canon applier must preserve wiring it has
//! never heard of**, and must not overwrite a hand-authored file to cure
//! variation.
//!
//! # Why this file exists rather than a "does it write the files" test
//!
//! Writing the five artefacts is the easy half and a test of it would pass on
//! an implementation that regenerated the pre-commit hook wholesale. **The hard
//! half is everything the applier must NOT do**, and each arm here is a real
//! consumer's real file:
//!
//! - Lamplight wires four guards through its chain block and **two of them are
//!   not in Intent canon at all** (`whiteboard-inbox-guard`, `live-doc-budget`).
//!   A canon-aware regenerator writes the guards it knows and drops the rest,
//!   silently, in a file nobody reads until a guard stops firing.
//! - Lamplight's block sits at lines 4-9, **not at the top**, so an
//!   implementation keying off "the first N lines" passes on a fixture built
//!   top-down and destroys a consumer whose block sits lower.
//! - A project's `CLAUDE.md` may be entirely hand-authored. Overwriting it is
//!   the cure being worse than the disease hv's instruction names.
//!
//! # The negative control is the point
//!
//! Every arm below is written so that **deleting the behaviour it guards makes
//! it fail**. `a_regenerator_would_fail_this` states that explicitly: it is the
//! arm that separates "preserved because we were careful" from "preserved
//! because nothing was there to lose".

use intentsvcs::canon::insert_chain_block;

/// A consumer hook whose own guards Intent canon has never heard of, with a
/// header comment between the preamble and the first real command.
const CONSUMER_HOOK: &str = "\
#!/usr/bin/env bash
set -euo pipefail

# this project's own guards, not Intent's
bash \"$(dirname \"$0\")/whiteboard-inbox-guard.sh\" || exit 1
bash \"$(dirname \"$0\")/live-doc-budget.sh\" || exit 1
";

#[test]
fn wiring_canon_has_never_heard_of_survives() {
  let out = insert_chain_block(CONSUMER_HOOK).expect("a hook with no block is edited");
  for guard in ["whiteboard-inbox-guard", "live-doc-budget"] {
    assert!(
      out.contains(guard),
      "`{guard}` was dropped. It is not in Intent canon, which is exactly why a \
       regenerator loses it and a region edit keeps it.\n--- got ---\n{out}"
    );
  }
  assert!(out.contains("# >>> intent-chain-block >>>"));
  assert!(out.contains("# <<< intent-chain-block <<<"));
}

/// **THE ARM THAT MAKES THE ONE ABOVE MEAN SOMETHING.**
///
/// A test asserting "the guards are still there" passes trivially if the
/// implementation simply appends. This pins the property that actually
/// distinguishes a region edit: **every original line is still present, in its
/// original order**, and the only additions are the block's own lines.
#[test]
fn a_regenerator_would_fail_this() {
  let out = insert_chain_block(CONSUMER_HOOK).expect("edited");
  let original: Vec<&str> = CONSUMER_HOOK.lines().collect();
  let produced: Vec<&str> = out.lines().collect();

  let mut i = 0;
  for line in &original {
    match produced[i..].iter().position(|p| p == line) {
      Some(offset) => i += offset + 1,
      None => panic!(
        "original line {line:?} is absent or out of order in the result -- this is a \
         regeneration, not a region edit\n--- got ---\n{out}"
      ),
    }
  }

  // And nothing beyond the block was invented.
  let added = produced.len() - original.len();
  assert!(
    added <= 7,
    "the edit added {added} lines; the chain block is 6 plus a blank. Anything more \
     means content was synthesised into a consumer's hook.\n--- got ---\n{out}"
  );
}

/// A block that does NOT sit at the top must still be found.
///
/// **Lamplight's sits at lines 4-9.** An implementation that scans only the
/// first few lines reports "absent", inserts a second block, and the hook then
/// runs the chain twice -- which is not obviously wrong from the output.
#[test]
fn an_existing_block_below_the_top_is_recognised() {
  let already = "\
#!/usr/bin/env bash
set -euo pipefail

# >>> intent-chain-block >>>
_intent_chain=\"$(git rev-parse --git-path hooks 2>/dev/null)/pre-commit.intent\"
if [ -x \"$_intent_chain\" ]; then
  \"$_intent_chain\" \"$@\" || exit $?
fi
# <<< intent-chain-block <<<

bash \"$(dirname \"$0\")/live-doc-budget.sh\" || exit 1
";
  assert!(
    insert_chain_block(already).is_none(),
    "a block below the top was not recognised, so a second one would be inserted and \
     the chain would run twice"
  );
}

/// Idempotence, stated as its own arm because it is the property that makes the
/// applier a converger rather than a writer: run it twice, change nothing.
#[test]
fn a_second_pass_changes_nothing() {
  let once = insert_chain_block(CONSUMER_HOOK).expect("first pass edits");
  assert!(
    insert_chain_block(&once).is_none(),
    "the second pass wanted to edit again, so the applier is not idempotent and every \
     run would move the hook's mtime"
  );
}

/// A hook that is nothing but preamble still gets the block.
///
/// The insertion point is "after the preamble", and a file that is ALL preamble
/// never reaches it. Without the fallback the block is silently not installed
/// into the very hooks most likely to be near-empty.
#[test]
fn a_preamble_only_hook_still_gets_the_block() {
  let out = insert_chain_block("#!/usr/bin/env bash\nset -euo pipefail\n").expect("edited");
  assert!(
    out.contains("# >>> intent-chain-block >>>"),
    "an all-preamble hook never reached the insertion point and got no block:\n{out}"
  );
}

/// An absent hook is a whole file, shebang included -- an inserted block with
/// no shebang above it is not executable as a hook.
#[test]
fn an_absent_hook_is_written_whole() {
  let out = insert_chain_block("").expect("an empty hook is written");
  assert!(
    out.starts_with("#!"),
    "no shebang, so git cannot execute it:\n{out}"
  );
  assert!(out.contains("# >>> intent-chain-block >>>"));
}
