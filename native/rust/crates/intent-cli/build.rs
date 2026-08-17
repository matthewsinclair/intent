//! Embeds the commit this binary was built from, so the ARTEFACT can answer for
//! itself (AC-11.5's binary arm).
//!
//! WHY THE BINARY AND NOT A RECORD BESIDE IT. `int macos stage` records
//! `commit: <HEAD>` and `traceable: yes` -- where traceable means the working
//! tree is clean -- and then COPIES binaries out of `target/release` that it
//! never built. Measured with HEAD at 2026-08-17T15:11:14Z: `intent` was built
//! three hours earlier and `intentd` FORTY-TWO hours earlier, forty-two hours
//! apart from each other. Commit the tree and the whole chain goes green over
//! bytes from two different earlier trees. A MANIFEST STATES WHAT WAS MEANT TO
//! BE BUILT; ONLY THE ARTEFACT CAN ANSWER WHAT WAS.
//!
//! NO BUILD TIME IS EMBEDDED, AND THAT IS A RULE RATHER THAN A PREFERENCE. The
//! reflex build script stamps a timestamp beside the sha. D42 forbids it: the
//! only legitimate time is the one a durable record was stamped with by the
//! write that created it, and a build clock is a second clock with extra steps.
//! The commit already carries an authoritative time in git, reachable from the
//! sha this embeds, so a build stamp would add nothing and assert a time nobody
//! can check.
//!
//! A DIRTY BUILD NAMES NO COMMIT. It emits `dirty-<sha>`, which no correct
//! parser can mistake for a commit id, rather than the bare sha of the tree it
//! was nearly built from. A build made from a peer's uncommitted work is
//! perfectly self-consistent -- that is the whole finding behind AC-11.5 -- so
//! the one thing it must not do is name a plausible commit. The sha is kept
//! inside the marker because "which commit was this nearly" is the first
//! question anyone asks, and dropping it would trade a real answer for a
//! shorter string.
//!
//! STALENESS FAILS CLOSED, WHICH IS THE ACTUAL ENGINEERING PROBLEM HERE.
//! cargo caches build-script output, so a binary can carry the PREVIOUS
//! commit's sha -- which is the very defect this exists to catch, wearing the
//! face of the fix. The consumer compares the embedded sha against the commit
//! and REFUSES on disagreement, so a stale embed produces a refusal that names
//! the rebuild as its remedy, never a plausible answer.
//!
//! AND THERE IS DELIBERATELY NO `cargo:rerun-if-changed` ON `.git/HEAD` (cc's
//! argument, taken in full, and it is the better call). The obvious way to stop
//! the embed going stale is to re-run this whenever HEAD moves. In a clone four
//! sessions commit into, HEAD moves when ANYONE commits ANYTHING -- a whiteboard
//! fold, a parity-tool edit, a document -- so that trigger does not mean "the
//! code changed", it means "a peer committed". `intent-cli`'s test legs spawn
//! `CARGO_BIN_EXE_intent`, so they would relink on every peer commit all day.
//! **The trigger exists to prevent exactly the state the fail-closed check
//! already refuses, so it buys nothing and charges four people for it.** Belt
//! and braces is usually free; here the belt costs four people's build times
//! and the braces already hold.
//!
//! Emitting NO `rerun-if-changed` line is not "no trigger": it restores cargo's
//! default of re-running this script when any file in the PACKAGE changes,
//! which tracks the code rather than the clone's commit traffic. That is the
//! trigger this actually wants.
//!
//! GIT ABSENT IS `unknown`, NOT A GUESS. A source tarball has no `.git`, and a
//! determinate "this artefact cannot say" is a fact about the build; a fabricated
//! or inherited sha would be a false one.

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
  let out = Command::new("git").args(args).output().ok()?;
  if !out.status.success() {
    return None;
  }
  Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn main() {
  let value = match git(&["rev-parse", "HEAD"]) {
    None => "unknown".to_string(),
    Some(sha) => match git(&["status", "--porcelain"]) {
      Some(s) if s.is_empty() => sha,
      Some(_) => format!("dirty-{sha}"),
      // git answered `rev-parse` and refused `status`: this cannot say whether
      // the tree was clean, and an unprovable claim is worth less than none.
      None => "unknown".to_string(),
    },
  };

  println!("cargo:rustc-env=INTENT_SOURCE_COMMIT={value}");
}
