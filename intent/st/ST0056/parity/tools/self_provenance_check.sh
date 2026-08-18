#!/bin/bash
# self_provenance_check.sh -- an artefact must answer for its own provenance.
#
# AC-11.5 / AT-11.5: an artefact asserts its own provenance, and the assertion is
# read from the artefact, never from a record written beside it. TWO of the row's
# three arms live here -- the vendored tree against its committed manifest, and
# the binary against the commit embedded in it. The third is in Devbin and is
# deliberately not cited from this side.
#
# THE HEADER ABOVE USED TO SAY "the binary arm waits on an embedded build
# commit", AND IT IS CORRECTED RATHER THAN QUIETLY REPLACED, because the reason
# it was written is the reason to keep it visible: AT-11.5 was held at `red`
# while arm 1 passed, on the ground that a green AT would satisfy AC-11.5 on one
# third of its criterion (v2's gate satisfies an AC on the FIRST green AT
# covering it -- issue 0032). That hold was correct and it is now discharged for
# arm 2 by building the thing rather than by relaxing the argument.
#
# WHY IT READS THE INDEX AND NOT THE WORKTREE, WHICH IS THE ENTIRE POINT. On
# 2026-08-17 this repository's HEAD carried a manifest that disagreed with the
# file beside it: `bin/devbin` hashed 41017e54, the committed manifest recorded
# 8016f112, and a clean extract of the commit reported 26 files matching and one
# DIVERGED. Every local check was GREEN, because an uncommitted manifest
# regeneration had made the WORKTREE self-consistent an hour earlier. The node
# who reported "27 of 27, stock devbin" was reading a tree that agreed with
# itself for a reason nobody else could reproduce.
#
# A TREE THAT AGREES WITH ITSELF BECAUSE SOMEONE RE-DERIVED THE RECORD LOCALLY IS
# INDISTINGUISHABLE FROM A TREE THAT AGREES BECAUSE IT IS CORRECT. So the subject
# has to be what the commit holds. The index is the right referent rather than
# HEAD: it is exactly the content about to become a commit, so a partial stage of
# the vendored set is judged as it will actually land.
#
# THE MECHANISM THAT PRODUCED IT, RECORDED BECAUSE THE FIX IS NOT "BE CAREFUL".
# A re-vendor is TWO FACTS -- the files and the record -- written by one act.
# `git commit --only` names paths, the author named the files, and the record
# stayed behind. The estate already carried that rule for the MOVE case ("naming
# the new path leaves the deletion staged") and it was not carried across,
# because a vendor does not look like a move. Four rules were broken by their own
# authors on this day, every one written down and none of them a mechanism; what
# they all lacked is something that fires while the act is performed. This fires.
#
# SCOPE IS THE COMMIT, NOT THE DIRECTORY, and that is a rule this toolchain has
# already paid for: an instrument that globs the worktree freezes every node's
# commits on a peer's in-flight file, and a guard that must be bypassed is a
# guard nobody keeps. Nothing here reads the working tree at all.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
MANIFEST="bin/.devbin/manifest.sha256"

die() {
  echo "self-provenance: $*" >&2
  exit 2
}

cd "$ROOT" || die "cannot reach the project root at $ROOT"
git rev-parse --git-dir >/dev/null 2>&1 ||
  die "not a git checkout, so there is no commit to read a vendored tree out of"

matched=0
diverged=0
absent=0
rc=0
has_manifest=1

# DETERMINATE ABSENCE IS A FACT ABOUT THE PROJECT, REPORTED AND PASSED. A project
# that vendors nothing has nothing to disagree with. It is stated rather than
# skipped in silence, because "no vendored tree" and "the check did not run" are
# different answers and only one of them is reassuring.
#
# THIS SET A FLAG AND `exit 0` UNTIL 2026-08-17, AND THE EXIT WAS CORRECT RIGHT
# UP UNTIL IT WAS NOT. When arm 1 was the whole file, "no vendored tree" really
# was the end of the run. Adding arm 2 silently made this a short-circuit that
# skipped it: in ANY project without a vendored devbin -- which is every consumer
# project, and every fixture -- the binary arm became dead code, and a binary
# carrying an obviously wrong sha went unreported at exit 0. Measured, not
# reasoned: a scratch repo with no manifest and a planted
# `[intent-source-commit:deadbeef...]` reported only the no-manifest line, with a
# control confirming the marker was findable in the file.
#
# It was found by trying to drive arm 2's third branch in a repo that has no
# manifest -- so the reachability probe found a different defect from the one it
# was looking for, which is the kind worth the ten minutes. THE GENERAL FORM IS
# ALREADY ON THIS ESTATE'S BOARDS: adding a second consumer to a script does not
# revisit the first one's exits, and an exit written when there was one arm is a
# claim that the run is over.
if ! git cat-file -e ":$MANIFEST" 2>/dev/null; then
  echo "self-provenance: no $MANIFEST in the commit -- this project vendors no devbin, so there is nothing to disagree with."
  has_manifest=0
fi

if [ "$has_manifest" -eq 1 ]; then
  # `rel`, NOT `path`. zsh ties `path` to `PATH`, so `read -r sha path` replaces the
  # shell's entire PATH with a filename on the first iteration and every subsequent
  # command reports `command not found` -- and the corruption OUTLIVES the loop. It
  # has bitten two nodes on this estate in one day, both while measuring THIS
  # manifest, because `read -r sha path` is the obvious spelling for a `.sha256`
  # line. This file is bash and would survive it; the spelling is avoided anyway,
  # because the next person to copy this loop may not be.
  while read -r want rel; do
    [ -n "$rel" ] || continue
    if ! git cat-file -e ":$rel" 2>/dev/null; then
      echo "self-provenance: $rel is named by the committed manifest and is NOT in the commit" >&2
      absent=$((absent + 1))
      rc=1
      continue
    fi
    got="$(git cat-file blob ":$rel" | shasum -a 256 | awk '{print $1}')"
    if [ "$got" = "$want" ]; then
      matched=$((matched + 1))
    else
      echo "self-provenance: $rel DIVERGED -- the commit holds bytes the committed manifest does not describe" >&2
      echo "    manifest records  $want" >&2
      echo "    the commit holds  $got" >&2
      diverged=$((diverged + 1))
      rc=1
    fi
  done < <(git cat-file blob ":$MANIFEST" | awk '/^#/ { next } NF >= 2 { p = $2; for (i = 3; i <= NF; i++) p = p " " $i; print $1, p }')

  # THE MATCHED COUNT IS THE INSTRUMENT'S OWN CANARY, AND IT IS REPORTED WHETHER OR
  # NOT ANYTHING FAILED. A run with zero matches is a broken run, not a wholly
  # forked tree -- measured on this estate when a `read -r sha path` loop destroyed
  # PATH and reported all 27 files diverged, one step away from being filed as an
  # issue. A wrong ZERO certifies absence and a wrong MAXIMUM certifies
  # catastrophe, and the second is the more persuasive because it looks like
  # diligence rewarded. The matches are what say the tool ran.
  #
  # This still `die`s, so an INSTRUMENT failure stops the run before arm 2. That is
  # deliberate: a broken instrument should not go on to render a second verdict.
  [ "$matched" -gt 0 ] ||
    die "the committed manifest named files and NOT ONE hashed -- this is the instrument failing, not $((diverged + absent)) genuine divergences"

  if [ "$rc" -eq 0 ]; then
    echo "self-provenance: $matched vendored file(s) in the commit match the manifest committed beside them."
  else
    echo "self-provenance: $matched matched, $diverged diverged, $absent absent -- the commit's vendored tree disagrees with its own record" >&2
    echo "    A re-vendor is two facts, the files and the record. Stage BOTH:" >&2
    echo "      git add $MANIFEST <the vendored files>" >&2
  fi
fi

# --------------------------------------------------------------------------
# ARM 2 -- THE BINARY NAMES ITS OWN COMMIT (AC-11.5, previously declared unbuilt).
#
# The arm above asks whether a vendored TREE agrees with the record committed
# beside it. This asks the harder half of the same criterion: can the ARTEFACT
# answer for itself, with nothing written beside it consulted at all. `build.rs`
# embeds the source commit into `intent` at build time and this reads it back
# out of the binary.
#
# WHY IT IS NEEDED, MEASURED RATHER THAN ARGUED. `int macos stage` records
# `commit: <HEAD>` and a clean-checkout flag -- and then COPIES binaries out of
# `target/release` it never built. (That flag was called `traceable` when this
# measurement was taken, which is a word about the ARTEFACTS over a field written
# by `git status`; it is `checkout_clean` now, and the rename is recorded here
# because this paragraph is the evidence and evidence names what it read.)
# With HEAD at 2026-08-17T15:11:14Z, `intent` was three hours older than the
# commit and `intentd` FORTY-TWO hours older, forty-two hours apart from each
# other. cc's framing is the one to keep: that is not a stale field, it is A
# RELEASE THAT IS TWO TREES WEARING ONE SHA. Every other check in that pipeline
# asks whether the bytes agree with each other, and bytes built from a peer's
# uncommitted work are perfectly self-consistent.
#
# IT REPORTS AND DOES NOT GATE, DELIBERATELY, AND THE PRECEDENT IS IN THIS
# TOOLCHAIN. `ratified_in_check.sh` reports until its table is clean and then
# graduates to refusing. The same reasoning is stronger here: this script is
# GATED IN `int precommit`, so an arm that failed on a binary lacking the marker
# would block every commit in a five-session clone until every stale artefact in
# every `target/` had been rebuilt -- and `target/` is shared mutable state
# several sessions write to. A GUARD THAT MUST BE BYPASSED IS A GUARD NOBODY
# KEEPS; that lesson was paid for on this estate when a worktree-globbing
# instrument froze every node's commits on a peer's in-flight file. The refusal
# belongs where the HARM is, which is publication: `int macos publish` refuses an
# artefact that cannot name its source commit, and it can now read that from the
# binary rather than from the sidecar the criterion rejects.
#
# `dirty-<sha>` IS NOT A FINDING HERE. A development build from a dirty tree
# SAYING it is dirty is the mechanism working, and in this clone the tree is
# dirty almost always. It is reported as what it is.
#
# AND IT IS NOT AN IDENTITY, WHICH IS A SEPARATE FACT THAT COST REAL DAMAGE
# BEFORE IT WAS STATED IN THE OUTPUT. The marker names a COMMIT; a commit does
# not determine an artefact when the tree is dirty. Measured 2026-08-18: three
# distinct `intent` binaries carried `dirty-18197aaf` inside one day, read
# independently by three nodes, and one of them wrote empty views over 57 and 82
# rows of the estate. vc compared two of them by marker and read them as the same
# artefact. So every line this arm prints now carries a sha256, and the reason it
# does is printed too -- a caveat that lives only in a comment is not a caveat the
# reader has, which is this file's own earlier defect at `:179-181` recurring one
# concern over.
#
# WHAT IS DELIBERATELY NOT CLAIMED: that a rebuild of unchanged inputs reproduces
# the hash. ic offered an event as evidence -- `intentd` byte-identical across a
# rebuild -- and the mtime showed nothing had rewritten it; the file was never
# rebuilt in the window measured. The claim may well be true and it is untested,
# so the output asserts only what was demonstrated: the hash distinguished two
# artefacts the marker could not.
#
# THE MARKER IS SELF-DELIMITING AND THE EXTRACTION RELIES ON THAT. Rodata packs
# string literals with no separator, so an unterminated marker runs into
# whatever the linker laid down next -- measured during the canary as
# `intent-source-commit:<sha>unsafe`, with `unsafe` from an unrelated literal.
# A greedy match would have captured the neighbour silently.
#
# `INTENT_SELF_PROV_BIN` EXISTS SO THIS ARM CAN BE DRIVEN AGAINST A BINARY THAT
# IS NOT IN THE SHARED `target/`, AND THAT IS A REQUIREMENT RATHER THAN A
# CONVENIENCE. Five sessions write `native/rust/target/`, so building there to
# test a check is the "running a gate to test it mutates somebody else's tree"
# failure this estate has already paid for. A check that can only be driven by
# mutating shared state is a check nobody canaries. It is not a security control
# and does not pretend to be one -- this arm REPORTS, so pointing it at a
# friendly binary buys a liar nothing that simply not running it would not.
# BOTH BINARIES, NOT JUST `intent`. A release ships two, and the one that was
# measured FORTY-TWO hours older than the commit it was recorded under was
# `intentd` -- so asking only `intent` would leave the motivating artefact
# unexamined while the output still read as one verdict. cc's form, and it is
# the store-scope lesson one layer up: A CHECK THAT COVERS PART OF A COMPOUND
# ARTEFACT REPORTS ON THE ARTEFACT.
# embed_coverage <binary-name> -- covered | uncovered | unknown.
#
# Answers whether a crate producing a binary of this NAME carries a `build.rs`
# that includes the shared embed, so the marker-absent branch can say which of
# two causes it is looking at instead of naming one. Matched on `name = "<b>"`,
# which catches both a `[package] name` and a `[[bin]] name` -- `intent` is
# produced by the crate `intent-cli`, so mapping binary to crate by directory
# name would be wrong for the very first binary this ships.
#
# `unknown` is a real answer and is reported as one. A function that folded "no
# such crate" into "uncovered" would state something it had not established,
# which is the defect this whole change is correcting.
embed_coverage() {
  local want="$1" ct d
  for ct in native/rust/crates/*/Cargo.toml; do
    [ -f "$ct" ] || continue
    grep -q "^name *= *\"$want\"" "$ct" || continue
    d="$(dirname "$ct")"
    if [ -f "$d/build.rs" ] && grep -q 'source_commit.rs' "$d/build.rs"; then
      printf 'covered'
    else
      printf 'uncovered'
    fi
    return 0
  done
  printf 'unknown'
}

SELF_PROV_BINS="${INTENT_SELF_PROV_BIN:-}"
if [ -z "$SELF_PROV_BINS" ]; then
  for b in intent intentd; do
    if [ -f "native/rust/target/release/$b" ]; then
      SELF_PROV_BINS="$SELF_PROV_BINS native/rust/target/release/$b"
    elif [ -f "native/rust/target/debug/$b" ]; then
      SELF_PROV_BINS="$SELF_PROV_BINS native/rust/target/debug/$b"
    fi
  done
fi

if [ -z "$SELF_PROV_BINS" ]; then
  # DETERMINATE ABSENCE, STATED RATHER THAN SKIPPED -- the same rule as the
  # no-manifest case above. "Nothing is built here" and "the check did not run"
  # are different answers and only one of them is reassuring.
  echo "self-provenance: no built binaries in native/rust/target -- nothing to ask for its provenance."
else
  head_sha="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
  # THE ARM ANNOUNCES ITS OWN CONTRACT, BECAUSE FOUR OF THE FIVE BRANCHES BELOW
  # ARE WORDED AS FINDINGS AND NONE OF THEM SETS `rc`. A comment saying so was
  # never enough: it lived at :179 and never reached a reader, who quite
  # reasonably read a passing line as an outstanding item and carried it into
  # four places. The caveat has to travel with the OUTPUT, because the output
  # is what gets quoted. Enforcement for this criterion lives at publication
  # (AC-11.5), not here.
  echo "self-provenance: the binary lines below are DIAGNOSTIC and this arm never fails -- enforcement is at \`int macos publish\`, which refuses an artefact that cannot name the tag's commit."
  echo "self-provenance: THE MARKER IS PROVENANCE AND IS NOT AN IDENTITY. It names a COMMIT, and a commit does not determine an artefact when the tree is dirty, so the sha256 on each line below is what distinguishes one build from another. Measured 2026-08-18: THREE distinct \`intent\` binaries carried \`dirty-18197aaf\` within one day -- read independently by three nodes -- and one of them emptied two of the estate's views. Pin by the hash, never by the marker. NOT CLAIMED HERE: that rebuilding unchanged inputs reproduces the hash; that is untested, and the case offered as evidence for it turned out to be a file nothing had rewritten."
  for BIN in $SELF_PROV_BINS; do
    # THE IDENTITY, COMPUTED FOR EVERY BRANCH RATHER THAN FOR THE INTERESTING ONE.
    # A reader comparing two runs of this tool reads the line, and until now the
    # only distinguishing token on it was the marker -- which is exactly the
    # comparison that failed: vc recorded two binaries as the same artefact
    # because their markers matched byte for byte.
    binsha="$(shasum -a 256 "$BIN" 2>/dev/null | cut -c1-16)"
    marker="$(strings "$BIN" 2>/dev/null | grep -o '\[intent-source-commit:[^]]*\]' | head -1)"
    embedded="${marker#\[intent-source-commit:}"
    embedded="${embedded%\]}"

    if [ -z "$marker" ]; then
      echo "self-provenance: $BIN [sha256 $binsha] carries NO source-commit marker -- it cannot name the commit it was built from."
      # WHICH OF THE TWO CAUSES, DERIVED RATHER THAN ASSERTED (ic, 2026-08-17).
      # What this branch OBSERVES is an absent marker. It used to PRINT "this
      # binary predates the embed" -- a cause, and one of at least two: the
      # binary is older than the embed, or the embed never covered its crate.
      # Today the first is true, and it is true because of a fact OUTSIDE this
      # check -- that both binary-producing crates carry a `build.rs`. The day a
      # third binary is added without one, an unchanged check would report the
      # wrong cause with exactly the confidence it reports the right one now.
      #
      # Derived from the crate that declares this binary NAME, not from a list of
      # binaries kept here: a needle list proves the cases somebody thought of.
      case "$(embed_coverage "$(basename "$BIN")")" in
        covered)
          echo "    Its crate DOES carry the embed, so these bytes predate it. Rebuild." ;;
        uncovered)
          echo "    Its crate carries NO build.rs including the shared embed, so nothing" \
               "was ever going to mark this binary. The embed must cover it first." ;;
        *)
          echo "    No crate here declares a binary by that name, so which of the two" \
               "causes this is cannot be established from the tree. Stated, not guessed." ;;
      esac
    elif [ "$embedded" = "unknown" ]; then
      echo "self-provenance: $BIN [sha256 $binsha] says its source commit is UNKNOWN -- built where git could not answer."
    elif [ "${embedded#dirty-}" != "$embedded" ]; then
      echo "self-provenance: $BIN [sha256 $binsha] was built from an UNCOMMITTED tree ($embedded) -- its bytes match no commit, and this marker does not distinguish it from any other build of the same dirty tree."
    elif [ "$embedded" = "$head_sha" ]; then
      echo "self-provenance: $BIN [sha256 $binsha] names $embedded, which is the current commit."
    else
      echo "self-provenance: $BIN [sha256 $binsha] names $embedded; the checkout is at $head_sha -- the binary is from an earlier tree."
    fi
  done
fi

exit $rc
