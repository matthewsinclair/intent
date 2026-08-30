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
// THE DIRT IS MEASURED OVER `native/rust/**` AND NOT THE WHOLE WORKTREE, AND
// THAT SCOPE IS THE ENTIRE VALUE OF THE FLAG (dc 2026-08-19, vc's release
// condition for minting the drift field). Measured: EVERY binary in this estate
// carried `dirty-`, including a release build of EXACTLY HEAD, because five
// nodes share this clone and somebody always holds an uncommitted board. A flag
// that is always set carries no information -- and it is worse than absent,
// because it occupies the slot where a real signal would go and reads as if it
// had been checked (ic). The marker's claim is about the BINARY, so its evidence
// has to be the paths that enter the binary.
//
// IT IS A BOOLEAN OVER A SUBTREE AND IT IS NOT A MAGNITUDE. It answers "does
// this artefact contain uncommitted code in its own sources" and cannot say how
// much, or which commit that code diverges from: one uncommitted line and five
// hundred set it identically. The question is named here because leaving it
// unnamed produces two OPPOSITE misreads -- a clean flag over a large
// uncommitted change reads as the flag being broken, and a set flag reads as a
// big change.
//
// NARROWING MAKES IT READ CLEAN WHERE IT USED TO READ DIRTY. That is the fix
// working, not the flag failing, and it is written here BEFORE the change lands
// because afterwards it is only a defence of a change already made: a reader who
// remembers that this was permanently set has no reason to believe otherwise.
//
// THE RELEASE GATE DOES NOT WEAKEN, WHICH IS WHY NARROWING IS SAFE. The
// whole-tree control is at `int macos publish`, DOWNSTREAM of this, and it has a
// second wall behind it: publish also refuses a non-release version, read off the
// staged binary. `int macos stage` does NOT refuse -- it records `checkout_clean`
// plus the blocker list and prints "DIRTY -- publish will refuse this", by its own
// argument that the control goes where the harm is, because a guard that makes the
// pipeline untestable without a pristine checkout gets worked around rather than
// kept. Narrowing is safe because the refusal is downstream of this flag, not
// beside it. Those answer a policy question about the CHECKOUT; this answers a
// question about the ARTEFACT. Conflating the two is what made this flag useless
// -- separating them is what lets both mean something.
//
// **THE SENTENCE THIS REPLACES CLAIMED `stage` REFUSED, AND IT WAS AUTHORED IN
// THIS SAME COMMIT AS THE SAFETY ARGUMENT FOR THE NARROWING** (dc, corrected
// after cc read it and told me my own pipeline would stop). The argument was
// true and its evidence named the wrong step -- so a reader auditing whether the
// narrowing was safe went to `stage` and got one of two wrong answers: that the
// gate had weakened, or that the comment was right and they could stop looking.
// cc took the second, within hours.
//
// AND THE FIXED FIRST INSTANCE OF THIS EXACT CLASS IS TWENTY LINES INTO THE FILE
// THIS DESCRIBES (cc). `cmd_stage` once had a field called `traceable` -- a word
// about the ARTEFACTS -- written by a check that asks `git status` about the
// CHECKOUT, and it was repaired by RENAMING THE FIELD rather than rewording the
// prose, on the explicit ground that "a sentence can go wrong again, and a field
// named for its own subject cannot carry the other claim in the first place."
// That structural remedy was chosen because prose was judged insufficient, and
// then the same day the same claim about the same step went back into prose in
// this file. **There is no structural move available to a comment describing code
// it does not live beside** -- which is the reason to name the shape here rather
// than only fix the sentence.
//
// GIT ABSENT IS `unknown`, NOT A GUESS. A source tarball has no `.git`, and a
// determinate "this artefact cannot say" is a fact about the build; a
// fabricated or inherited sha would be a false one.

use std::process::Command;

/// The paths whose dirt can actually reach the binary.
///
/// `:(top)` is load-bearing: cargo runs a build script with its CWD at the
/// PACKAGE root (`crates/intent-cli`), and a bare relative pathspec would
/// resolve against that and silently match nothing -- a scope that excludes
/// everything reports clean forever, which is the same defect as a scope that
/// includes everything, in the direction nothing reports.
/// **THE SCOPE IS THE BUILD'S INPUTS, AND THAT IS WIDER THAN `native/rust` BY
/// EXACTLY ONE PATH.** `surface/dispatch-table.json` is `include_str!`'d into
/// `intent-cli` (`dispatch.rs:45`), so its bytes are IN the binary. It was
/// omitted here until 2026-08-26 and the omission had two faces: a build dirty
/// only in `surface/` was not stamped `dirty-`, **so the artefact could not
/// disown what the build guard already refused**; and once identity started
/// being asked over this same scope, a `surface/`-only commit would have changed
/// the binary WITHOUT changing the stamp. **41 of the last 50 commits touching
/// `surface/` do not touch `native/rust`**, so that was not a corner case.
///
/// `sharedtarget.lib`'s `SHARED_TARGET_DIRT_SCOPES` is the same list on the
/// shell side, and `shared_artefact_build_guard.sh` arm 6 asserts the guard's
/// scope CONTAINS this one. Equality satisfies containment; **do not tighten
/// that arm to equality**, because the containment form is what permitted this
/// widening in the first place.
/// **`docs/design/` IS HERE BECAUSE `intentd` EMBEDS THE LOGO FROM IT**
/// (`web.rs`, `AC-08.9`). The web face's shell serves the project's mark with
/// `include_str!("../../../../../docs/design/intent-logo.svg")`, so that file
/// is a compile-time INPUT to the shared binary while sitting outside
/// `native/rust` -- and a build made while it was mid-edit would have been
/// approved by the guard and baked in. **The widening was anticipated by the
/// paragraph above rather than improvised**: arm 6b measured the gap, named
/// the path, and pointed at this list as the thing at fault.
///
/// **THE ALTERNATIVE WAS A SECOND COPY OF THE LOGO UNDER `native/rust` AND IT
/// IS THE WRONG FIX** (vc's ruling, 2026-08-30). One mark, one home; a copy is
/// a thing to update when the mark changes, and a stale logo still renders.
const DIRT_SCOPE: &[&str] = &[":(top)native/rust", ":(top)surface", ":(top)docs/design"];

fn git(args: &[&str]) -> Option<String> {
  let out = Command::new("git").args(args).output().ok()?;
  if !out.status.success() {
    return None;
  }
  Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Emits `INTENT_SOURCE_COMMIT` and `INTENT_SOURCE_COMMIT_MARKER` for the crate
/// being built.
///
/// **THE MARKER'S FORMAT LIVES HERE, AND UNTIL 2026-08-22 IT LIVED IN TWO
/// CRATES.** `intent-cli/src/lib.rs` and `intentd/src/main.rs` each carried a
/// byte-identical `concat!("[intent-source-commit:", env!(..), "]")`. That is
/// the EMIT side of the same defect just closed on the PARSE side, where four
/// shell implementations were reduced to one -- **and it is the more dangerous
/// half, because one parser against two formats fails per-binary and quietly**:
/// `self_provenance_check.sh` reports each artefact separately, so a forked
/// literal prints a clean line for one binary and `carries NO marker` for the
/// other, on a DIAGNOSTIC arm that never fails. One grep hardened, one format
/// forked, nothing refusing (vc found it).
///
/// **THE OBVIOUS FIX WAS THE WRONG ONE AND THE PREMISE UNDER IT WAS FALSE.** A
/// shared `macro_rules!` in `intentsvcs` was proposed on the grounds that both
/// crates already depend on it. **`intentd` has no `[dependencies]` section at
/// all** -- it depends on nothing -- so that shape means ADDING a dependency to
/// host a format literal, which is precisely what this file already refused as
/// *reshaping the crate for the sake of where a marker lives*. (The macro shape
/// itself is sound and was driven: an exported `macro_rules!` containing `env!`
/// expands in the CONSUMING crate and reads that crate's build env, with the
/// defining crate never knowing the value. Sound, and not needed.)
///
/// Emitting the whole marker costs no dependency and no macro, and puts the
/// format in the file that was ALREADY the one home for the emit side. Each
/// crate's static becomes `env!("INTENT_SOURCE_COMMIT_MARKER")` -- still a
/// `&'static str` literal in rodata, so `#[used]` behaves exactly as before.
///
/// `INTENT_SOURCE_COMMIT` stays: `intent-cli/src/lib.rs:26` exposes the bare
/// value as `pub const SOURCE_COMMIT`, which is a real consumer and not a
/// duplicate of this one.
fn emit_source_commit() {
  // **BOTH QUESTIONS ARE ASKED ABOUT THE SAME SUBJECT, AND THAT IS THE WHOLE
  // FIX.** Until 2026-08-26 identity came from an UNSCOPED `rev-parse HEAD`
  // while dirt came from a SCOPED `status`, so the value meant "the repo's HEAD,
  // annotated with whether the artefact was dirty" -- two subjects in one
  // string. It was internally inconsistent rather than merely awkward, and it
  // showed twice: a commit anywhere in the repo landing during a ~60s build
  // REDDED A CORRECT PAIR, and the marker REWROTE THE STAMP OF BYTE-IDENTICAL
  // CODE, so two builds of the same source carried different stamps.
  //
  // **WHAT IT COSTS, STATED RATHER THAN DISCOVERED: THE STAMP NO LONGER
  // IDENTIFIES THE REPO.** Two repo states with identical build inputs now
  // produce identical stamps, and "which commit was this built at" is no longer
  // answerable from the artefact. That is the correct trade, because the two
  // questions already have two homes: the release TAG records the repo, and this
  // marker records the SUBJECT. An artefact should carry what it IS.
  let mut ident: Vec<&str> = vec!["rev-list", "-1", "HEAD", "--"];
  ident.extend_from_slice(DIRT_SCOPE);
  let mut dirt: Vec<&str> = vec!["status", "--porcelain", "--"];
  dirt.extend_from_slice(DIRT_SCOPE);

  let value = match git(&ident) {
    None => "unknown".to_string(),
    // **`rev-list` ANSWERS rc 0 WITH EMPTY OUTPUT WHEN NO COMMIT TOUCHES THE
    // SCOPE**, which `rev-parse HEAD` never did -- so nothing in the previous
    // shape had any reason to guard it. Without this arm the emit is
    // `INTENT_SOURCE_COMMIT=` and the marker reads `[intent-source-commit:]`.
    // **An empty stamp is not a smaller claim than a sha; it is a broken one,
    // and it would pass every arm we have.** Driven before it was written.
    Some(sha) if sha.is_empty() => "unknown".to_string(),
    Some(sha) => match git(&dirt) {
      Some(s) if s.is_empty() => sha,
      Some(_) => format!("dirty-{sha}"),
      // git answered the identity call and refused `status`: this cannot say
      // whether the tree was clean, and an unprovable claim is worth less
      // than none.
      None => "unknown".to_string(),
    },
  };

  println!("cargo:rustc-env=INTENT_SOURCE_COMMIT={value}");
  // SELF-DELIMITING, and the brackets are load-bearing rather than decorative:
  // rodata packs string literals with no separator, so an unterminated marker
  // runs into whatever the linker laid down next -- measured during this row's
  // canary as `intent-source-commit:<sha>unsafe`, with `unsafe` belonging to an
  // unrelated literal. `artefact_source_commit`'s `[^]]*` stops at the bracket
  // this line provides, so these two lines are one contract with one home each.
  println!("cargo:rustc-env=INTENT_SOURCE_COMMIT_MARKER=[intent-source-commit:{value}]");
}
