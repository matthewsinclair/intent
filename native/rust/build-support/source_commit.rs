// The source-commit embed, shared by every binary crate's `build.rs` (AC-11.5).
//
// ONE HOME, TWO WRAPPERS, BECAUSE CARGO REQUIRES A `build.rs` PER PACKAGE AND
// HIGHLANDER FORBIDS TWO COPIES OF THE LOGIC. `intent-cli/build.rs` and
// `intentd/build.rs` are each three lines that `include!` this file and call
// `emit()`. Two byte-identical build scripts would drift -- and they would drift
// silently, because a provenance marker's whole failure mode is being wrong
// without being absent.
//
// CONSIDERED AND REJECTED: putting a single `build.rs` in `intentsvcs`, which
// both binaries already depend on. It would be one script rather than two, and
// it fails on an empirical question nobody has answered -- `#[used]` stops the
// COMPILER dropping a static, and whether the linker pulls an unreferenced
// object out of a dependency's rlib into a downstream binary is a different
// question, unproven here. A provenance marker that vanishes from the artefact
// is worse than a duplicated file. (`intentsvcs` also being another node's
// active crate is a reason to not touch it today, but it is not the reason
// this was rejected.)
//
// WHY THE BINARY AND NOT A RECORD BESIDE IT. `int macos stage` records
// `commit: <HEAD>` and `traceable: yes` -- where traceable means the working
// tree is clean -- and then COPIES binaries out of `target/release` that it
// never built. Measured with HEAD at 2026-08-17T15:11:14Z: `intent` was built
// three hours earlier and `intentd` FORTY-TWO hours earlier, forty-two hours
// apart from each other. A MANIFEST STATES WHAT WAS MEANT TO BE BUILT; ONLY THE
// ARTEFACT CAN ANSWER WHAT WAS.
//
// NO BUILD TIME IS EMBEDDED, AND THAT IS A RULE RATHER THAN A PREFERENCE. D42
// forbids it: the only legitimate time is the one a durable record was stamped
// with by the write that created it. The commit already carries an
// authoritative time in git, reachable from the sha this embeds, so a build
// stamp would add nothing and assert a time nobody can check -- it can only
// agree with the commit's own time or contradict it.
//
// A DIRTY BUILD NAMES NO COMMIT. It emits `dirty-<sha>`, which no correct
// parser reads as a commit id. The dirt is carried INSIDE the value rather than
// beside it in a second field, so a consumer that forgets to read the second
// field cannot silently treat a dirty build as clean.
//
// AND THERE IS DELIBERATELY NO `cargo:rerun-if-changed` ON `.git/HEAD`.
// Emitting NO line is not "no trigger": it restores cargo's default of
// re-running this script when any file in the PACKAGE changes, which is the
// trigger that tracks the code. **Emitting ANY `rerun-if-changed` REPLACES
// that default**, so a line naming `.git/HEAD` would swap a trigger that
// follows the code for one that does not follow it at all -- the embed would
// then go stale on CODE changes, permanently and silently. The naive fix is
// strictly worse than the gap, and worse in the direction nothing reports.
//
// **THE ORIGINAL REASON RECORDED HERE WAS FACTUALLY WRONG AND IS CORRECTED
// RATHER THAN DELETED** (cc 2026-08-18, measured after vc refused a reversal
// that cited it). It said "in a clone five sessions commit into, HEAD moves
// when ANYONE commits ANYTHING". **It does not.** `.git/HEAD` holds
// `ref: refs/heads/main` and is rewritten on a BRANCH SWITCH, not on a commit:
// measured in this repo with `.git/HEAD` at an mtime six months old while
// `.git/refs/heads/main` and `.git/logs/HEAD` had both moved seconds earlier
// with the commit just landed. So the rebuild storm this paragraph priced
// against would never have occurred -- **the conclusion was right and one of
// its two reasons was not**, which is the more dangerous shape, because the
// wrong reason is the one that makes the fix look obviously correct.
//
// If freshness is ever wanted the expressible form is BOTH lines --
// `rerun-if-changed=src` plus `.git/logs/HEAD`, the file that actually moves
// on a commit -- and that does cost a build-script re-run per peer commit,
// which is the cost originally priced, just against the wrong file.
//
// MEASURED COST, so it is a known limitation rather than a discovery waiting
// to happen: a HEAD move outside the package leaves the embed stale --
// witnessed with the binary still naming `b11ca6ac` at HEAD `010b2bbf`, and
// again on 2026-08-18 naming `dirty-4ef953db` at HEAD `c83f624c`. That
// staleness is what `int macos publish` refuses on, so it fails closed.
//
// AND STALENESS IS THE SMALLER HALF (dc). Even with the embed always fresh,
// the marker is `dirty-<HEAD>`, so two behaviourally different dirty builds at
// one commit share one value -- it is not a wrong answer, it is a right answer
// to a different question. **The marker names a commit; it was never an
// identity**, and no trigger makes it one. A paired reading needs a content
// hash (vc, AC-10.11), which is where that harm is actually closed.
//
// GIT ABSENT IS `unknown`, NOT A GUESS. A source tarball has no `.git`, and a
// determinate "this artefact cannot say" is a fact about the build; a
// fabricated or inherited sha would be a false one.

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
  let out = Command::new("git").args(args).output().ok()?;
  if !out.status.success() {
    return None;
  }
  Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Emits `INTENT_SOURCE_COMMIT` for the crate being built.
fn emit_source_commit() {
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
