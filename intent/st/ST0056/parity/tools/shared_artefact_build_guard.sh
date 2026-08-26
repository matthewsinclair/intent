#!/bin/bash
# shared_artefact_build_guard.sh -- AC-11.6 / AT-11.6.
#
# A REBUILD INTO THE SHARED ARTEFACT PATH IS REFUSED WHEN THE TREE IS DIRTY, and
# the refusal happens BEFORE the artefact exists. `native/rust/target/release/`
# is shared by every node in this checkout with no per-node isolation underneath
# it; the founding episode is a build at 19:33Z on 2026-08-18 from a live tree
# carrying uncommitted WP-01 source, which produced a working-looking binary that
# emptied the store for every node until rebuilt.
#
# THE RED ARM IS THE REAL EPISODE AND IT IS FIRST. That ordering is not a style
# choice: this row spent three revisions cited at mechanisms that could not have
# fired on the episode that produced it -- `prepush` clones HEAD to a temp dir and
# never touches the shared path at all -- and every one of those citations was
# offered with a true, driven measurement of a DIFFERENT property attached. A
# green arm alone would have passed under all three. So the first thing this file
# proves is that the guard REFUSES, on a tree shaped like the one that caused it.
#
# WHY IT DRIVES A SCRATCH REPO AND NEVER THIS ONE. The subject is a build guard,
# and the only honest way to test a build guard against the live tree would be to
# run a build into the shared slot -- which is the act the guard exists to
# prevent. The predicate is therefore separable by design (`sharedtarget.lib`),
# and every arm below drives it in a disposable repo with planted state. Nothing
# here builds anything, and nothing here reads or writes
# `native/rust/target/release/`.
#
# REACH, AND IT IS THE HONEST LIMIT OF THIS FILE: THE REDIRECT HAS NEVER
# EXECUTED. Arms 1 to 6 DRIVE the predicate in disposable repos and are real
# behaviour. Arms 7 and 8 READ THE SOURCE of `cmd/local` -- they establish that
# the ordering and the redirect are PRESENT, not that running a dirty build
# actually writes the private target dir and actually leaves the shared one
# alone. Confirming that needs a real cargo build, which this file deliberately
# does not do. One claim here is taken from source rather than from behaviour and
# it is named so that nobody reads a green as covering it.
#
# ARM 4 IS THE ONE THAT WOULD HAVE CAUGHT THE ORIGINAL DEFECT, AND IT IS ABOUT
# ORDER RATHER THAN OUTCOME. `cmd/local`'s `verify_pair` ALREADY refused a
# `dirty-` marker before any of this existed -- it simply ran after `cargo build`
# had overwritten the shared binaries, so it reported the damage rather than
# preventing it. A guard whose verdict is correct and late is indistinguishable
# from no guard, to everyone downstream of the artefact. Outcome arms cannot see
# that; only an order arm can.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

LIB="$ROOT/bin/.devbin/cmd/shared/sharedtarget.lib"
LOCAL_CMD="$ROOT/bin/.devbin/cmd/local"
# THE GUARDED BUILD MOVED OUT OF `cmd/local` ON 2026-08-26 AND THIS SUBJECT
# MOVED WITH IT. Arms 7 and 8 read the body of the ONE guarded release build,
# and that body now lives in a lib because FOUR other entrances -- `build all`,
# `build cli`, `build daemon` and `cmd/cli`'s fallback -- wrote the same shared
# artefact with a bare `cargo build` while this arm truthfully reported that
# `cmd_build` consulted the verdict. **The arm was right and its subject was one
# door of five.**
RELEASEBUILD_LIB="$ROOT/bin/.devbin/cmd/shared/releasebuild.lib"
MARKER_SRC="$ROOT/native/rust/build-support/source_commit.rs"

rc=0
pass=0
fail() { printf 'shared-artefact-guard: %s\n' "$1" >&2; rc=1; }
ok()   { pass=$((pass + 1)); printf 'shared-artefact-guard: ok -- %s\n' "$1"; }

if [ ! -r "$LIB" ]; then
  fail "no predicate library at $LIB -- the guard cannot be checked because it does not exist"
  exit 1
fi
# shellcheck source=/dev/null
. "$LIB"

# scratch_repo <dir> -- a minimal repo with a committed native/rust tree.
#
# `-c` rather than `git config`: a scratch repo inherits nothing, and a machine
# without a global identity would otherwise fail the COMMIT rather than the arm,
# which reads as a guard failure and is not one.
scratch_repo() {
  local d="$1"
  # SEE `sharedtarget.lib`s isolation note. `git -C` does NOT override GIT_DIR or
  # GIT_INDEX_FILE, and git exports both into hook environments -- so under the
  # pre-commit gate these three calls ran against THIS repository`s index and
  # staged the fixture into it, naming an object the outer repo cannot resolve
  # (`error: Error building trees`). It blocked every commit in the estate.
  local G="env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE -u GIT_OBJECT_DIRECTORY -u GIT_COMMON_DIR git"
  $G -C "$d" init -q 2>/dev/null || return 1
  mkdir -p "$d/native/rust/crates/thing/src"
  printf 'fn main() {}\n' > "$d/native/rust/crates/thing/src/main.rs"
  printf 'placeholder\n' > "$d/README.md"
  $G -C "$d" add -A >/dev/null 2>&1 || return 1
  $G -C "$d" -c user.email=guard@test -c user.name=guard commit -qm base >/dev/null 2>&1 || return 1
  return 0
}

TMP="$(mktemp -d)" || { fail "cannot make a scratch directory"; exit 1; }
trap 'rm -rf "$TMP"' EXIT

# ---------------------------------------------------------------- ARM 1: RED
# A tree dirty under native/rust, building into the shared release path, must be
# REFUSED with the offending paths named. This is the 19:33Z shape.
red="$TMP/red"
mkdir -p "$red"
if ! scratch_repo "$red"; then
  fail "arm 1 could not build its fixture -- an arm that cannot run is not an arm that passed"
else
  printf 'fn main() { /* uncommitted */ }\n' > "$red/native/rust/crates/thing/src/main.rs"
  verdict="$(shared_target_verdict "$red")"
  case "$verdict" in
    refuse:*) ok "arm 1 RED -- a dirty native/rust tree is refused: ${verdict#refuse:}" ;;
    *) fail "arm 1 RED -- a dirty native/rust tree returned '$verdict'; this is the episode the criterion was minted for and the guard let it through" ;;
  esac
  # AC-11.6 requires the refusal to NAME the paths. A refusal that does not say
  # what is dirty sends the operator to `git status` to re-derive what the guard
  # already knew, and the first thing anybody does with a guard they cannot act
  # on is bypass it.
  named="$(shared_target_dirt_report "$red")"
  case "$named" in
    *"crates/thing/src/main.rs"*) ok "arm 1 RED -- the refusal names the offending path" ;;
    *) fail "arm 1 RED -- the refusal does not name the offending path; it reported: ${named:-<nothing>}" ;;
  esac
fi

# ------------------------------------------------------------- ARM 2: UNTRACKED
# An UNTRACKED file under native/rust must refuse too. It is not a nicety: the
# live tree on the day this was written carried exactly one (an untracked test
# beside a modified source file), and a guard reading only modifications would
# have called that tree clean.
unt="$TMP/untracked"
mkdir -p "$unt"
if ! scratch_repo "$unt"; then
  fail "arm 2 could not build its fixture"
else
  printf 'fn helper() {}\n' > "$unt/native/rust/crates/thing/src/extra.rs"
  verdict="$(shared_target_verdict "$unt")"
  case "$verdict" in
    refuse:*) ok "arm 2 -- an untracked native/rust file is refused" ;;
    *) fail "arm 2 -- an untracked native/rust file returned '$verdict'; modifications are not the only way bytes enter a build" ;;
  esac
fi

# --------------------------------------------------------------- ARM 3: GREEN
# A clean tree takes the shared path. Present as a CONTROL rather than as the
# evidence: an instrument that only ever refuses is a freeze, and a freeze passes
# arm 1 for the wrong reason.
green="$TMP/green"
mkdir -p "$green"
if ! scratch_repo "$green"; then
  fail "arm 3 could not build its fixture"
else
  verdict="$(shared_target_verdict "$green")"
  case "$verdict" in
    ok) ok "arm 3 GREEN -- a clean tree is allowed into the shared path" ;;
    *) fail "arm 3 GREEN -- a clean tree returned '$verdict'; a guard that refuses everything is a freeze, and a freeze gets bypassed" ;;
  esac
fi

# ------------------------------------------------------- ARM 4: DIRT IS SCOPED
# Dirt OUTSIDE native/rust must NOT refuse. This is the arm that keeps the guard
# usable: five nodes share this clone and somebody always holds an uncommitted
# whiteboard file. Measured 2026-08-19 -- with a worktree-wide scope EVERY binary
# in this estate carried `dirty-`, including a release build of exactly HEAD. A
# flag that is always set carries no information, and it is worse than absent
# because it occupies the slot where a real signal would go.
outside="$TMP/outside"
mkdir -p "$outside"
if ! scratch_repo "$outside"; then
  fail "arm 4 could not build its fixture"
else
  printf 'a dirty board, which enters no binary\n' > "$outside/README.md"
  verdict="$(shared_target_verdict "$outside")"
  case "$verdict" in
    ok) ok "arm 4 -- dirt outside native/rust does not refuse the shared path" ;;
    *) fail "arm 4 -- dirt outside native/rust returned '$verdict'; this scope makes the guard a freeze that gets bypassed" ;;
  esac
fi

# ---------------------------------------------------- ARM 5: FAILS CLOSED
# git unreachable must NOT read as clean. Empty output means clean to any reader
# that does not distinguish them, and that is the one direction a guard on the
# build path must never fail.
nogit="$TMP/nogit"
mkdir -p "$nogit/native/rust"
verdict="$(shared_target_verdict "$nogit")"
case "$verdict" in
  refuse:*) ok "arm 5 -- an unreachable git refuses rather than reading as clean" ;;
  *) fail "arm 5 -- an unreachable git returned '$verdict'; undecidable is not a pass on the path that overwrites the shared artefact" ;;
esac

# ------------------------------------------------- ARM 6: SCOPE AGREES WITH THE MARKER
# The guard's scope and the embedded provenance marker's `DIRT_SCOPE` are the
# same question asked by two parties. If they diverge, the shared slot can hold
# an artefact this guard approved and the artefact itself disowns -- approved
# clean, stamped `dirty-`. Compared here rather than coupled at runtime: a shell
# library parsing a Rust constant on every invocation fails in the dark, and a
# test that compares them fails in the light.
# THE ASSERTION IS CONTAINMENT, NOT EQUALITY, AND THE EARLIER EQUALITY WAS WRONG
# IN A WAY THAT WOULD HAVE FORBIDDEN THE FIX IT NEEDED. Being STRICTER than the
# marker is safe: this guard then refuses builds the marker would have called
# clean. Being LOOSER is the unsafe direction, because the shared slot can hold
# an artefact this guard approved and the artefact itself disowns. An equality
# arm cannot tell those apart -- it reds on both -- and it red on the widening
# that closed a live incident.
if [ ! -r "$MARKER_SRC" ]; then
  fail "arm 6 -- no marker source at $MARKER_SRC, so scope containment cannot be checked"
else
  # DIRT_SCOPE BECAME A LIST ON 2026-08-26 and this parser moved with it, in the
  # same commit, because a reader left behind reports "the constant moved or was
  # renamed" -- a true sentence about the wrong thing.
  marker_scopes="$(sed -n 's/^const DIRT_SCOPE: &\[&str\] = &\[\(.*\)\];$/\1/p' "$MARKER_SRC" |
    tr ',' '\n' | sed -e 's/^[[:space:]]*"//' -e 's/"[[:space:]]*$//' | grep .)"
  if [ -z "$marker_scopes" ]; then
    fail "arm 6 -- could not read DIRT_SCOPE from $MARKER_SRC; the constant moved, was renamed or changed shape, and an unread scope is not a contained one"
  else
    uncontained=""
    while IFS= read -r ms; do
      covered=0
      for sc in "${SHARED_TARGET_DIRT_SCOPES[@]}"; do
        [ "$sc" = "$ms" ] && covered=1
      done
      [ "$covered" -eq 1 ] || uncontained="$uncontained $ms"
    done <<< "$marker_scopes"
    if [ -z "$uncontained" ]; then
      ok "arm 6 -- the guard's scope CONTAINS every marker DIRT_SCOPE entry ($(printf '%s' "$marker_scopes" | tr '\n' ' '))"
    else
      fail "arm 6 -- the guard's scope does not contain the marker's$uncontained. Looser than the marker means the shared slot can hold an artefact this guard approved and the artefact disowns."
    fi
  fi

  # ARM 6c -- THE MARKER ASKS **IDENTITY** OVER THE SCOPE TOO, NOT JUST DIRT.
  #
  # Arm 6 checks that the two scopes agree. It is structurally blind to the
  # defect that made this fix necessary: until 2026-08-26 the marker asked DIRT
  # over DIRT_SCOPE and IDENTITY over an unscoped `rev-parse HEAD`, so one
  # string answered two questions about two different subjects -- and arm 6 was
  # green throughout, because the scope it compared was the one that was already
  # right.
  #
  # THE BODY IS READ WITH COMMENTS STRIPPED, following arm 7, which failed its
  # own first draft by matching a header sentence ABOUT an invocation. That is
  # not hypothetical here: the fix's own doc comment names `rev-parse HEAD`
  # twice, explaining what it replaced. **A guard that reads prose would refuse
  # the very change it exists to enforce.**
  emit_body="$(awk '/^fn emit_source_commit\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' "$MARKER_SRC" |
    sed 's://.*::')"
  if [ -z "$emit_body" ]; then
    fail "arm 6c -- could not extract emit_source_commit from $MARKER_SRC; an unread body is not a scoped one"
  elif printf '%s' "$emit_body" | grep -q 'rev-parse'; then
    fail "arm 6c -- emit_source_commit still asks identity with rev-parse, which is UNSCOPED. The stamp would mean 'the repo's HEAD, annotated with whether the artefact was dirty' -- two subjects in one string."
  elif ! printf '%s' "$emit_body" | grep -q 'rev-list'; then
    fail "arm 6c -- emit_source_commit asks identity with neither rev-parse nor rev-list; the call changed shape and this arm cannot say what subject it names"
  elif ! printf '%s' "$emit_body" | grep -q 'DIRT_SCOPE'; then
    fail "arm 6c -- emit_source_commit's identity call does not reach DIRT_SCOPE, so identity and dirt describe different subjects again"
  else
    ok "arm 6c -- the marker asks identity AND dirt over DIRT_SCOPE"
  fi
fi

# ------------------------------ ARM 6b: THE SCOPE COVERS WHAT THE BUILD COMPILES
# **THE DECLARED SCOPE IS CHECKED RATHER THAN DERIVED, AND THIS ARM IS THE
# PROMPT THAT MAKES DECLARING SAFE.** `intent-cli` embeds
# `surface/dispatch-table.json` with `include_str!` at COMPILE time, from OUTSIDE
# the crate tree -- so it is a build input the original `native/rust` scope could
# not see. **Measured live on 2026-08-25 (vc, on themselves): an announced,
# peer-cleared, guard-approved build on a clean `native/rust` baked a peer's
# half-written dispatch row into the shared binary, and every `intent3 sync`
# then panicked on it.** The peer reverted the file within minutes, **so the
# source that explained the binary no longer existed anywhere** -- a dirty
# `native/rust` build at least leaves its cause behind to be found.
#
# **A DECLARED SCOPE THAT MUST BE HAND-UPDATED WHEN AN UNRELATED FILE GAINS AN
# `include_str!` IS A GUARD WHOSE CORRECTNESS DEPENDS ON A STEP NOBODY IS
# PROMPTED TO TAKE** (vc). Deriving the scope at runtime would mean parsing Rust
# from shell on every invocation, which is a coupling that fails in the dark.
# This arm is the third option: declare it, and red in the light the day a second
# outside-the-tree embed appears.
embeds="$(grep -rho 'include_str!("[^"]*")\|include_bytes!("[^"]*")' "$ROOT/native/rust/crates" 2>/dev/null \
          | grep -oE '"\.\./[^"]*"' | tr -d '"' | sort -u)"
uncovered=""
while read -r rel; do
  [ -n "$rel" ] || continue
  # Only paths that climb OUT of native/rust are this arm's business; anything
  # resolving inside it is already covered by the marker's own scope.
  case "$rel" in
    */surface/*) covered_by=":(top)surface" ;;
    *) covered_by="" ;;
  esac
  hit=0
  for sc in "${SHARED_TARGET_DIRT_SCOPES[@]}"; do
    [ -n "$covered_by" ] && [ "$sc" = "$covered_by" ] && hit=1
  done
  [ "$hit" -eq 1 ] || uncovered="$uncovered $rel"
done <<EOF
$embeds
EOF
if [ -z "$embeds" ]; then
  fail "arm 6b -- found NO outside-the-tree embeds at all; dispatch.rs is known to carry one, so the probe is broken rather than the tree being clean"
elif [ -z "$uncovered" ]; then
  ok "arm 6b -- every embed reaching outside native/rust is covered by the declared scope"
else
  fail "arm 6b -- these compile-time embeds reach outside native/rust and the declared scope does NOT cover them:$uncovered. A build mid-edit in one of them is approved by this guard and baked into the shared binary."
fi

# ------------------------------------------- ARM 7: THE VERDICT PRECEDES THE BUILD
# THE ORDER IS THE CRITERION. `verify_pair` already refused a `dirty-` marker
# before this guard existed; it ran after `cargo build` had replaced the shared
# binaries, so it named the damage instead of preventing it. AC-11.6 weighed that
# exact ordering and ruled for the earlier refusal in its own words -- before the
# artefact exists, rather than producing a working-looking binary four nodes then
# invoke. An outcome arm cannot see this; a correct verdict arriving late passes
# every one of arms 1 to 6.
if [ ! -r "$RELEASEBUILD_LIB" ]; then
  fail "arm 7 -- no guarded build at $RELEASEBUILD_LIB"
else
  # THE BODY OF `cmd_build`, NOT THE WHOLE FILE, AND COMMENTS STRIPPED. Both
  # narrowings are corrections to this arm's own first draft, which reported a
  # FAILURE against correct code: it grepped the whole file and matched the
  # header sentence "WHY `build` IS NOT `cargo build --release`" at line 31 --
  # prose ABOUT a cargo invocation, read as one. That is this estate's oldest
  # instrument defect (a grep cannot tell a statement from a sentence about a
  # statement) arriving inside the tool written to catch a different one.
  #
  # It could equally have failed the other way and that is the worse direction:
  # a comment MENTIONING the verdict, sitting above a real cargo line, would have
  # made the arm PASS while the call site was absent or late. Scoping to the
  # function body also removes the file-order-versus-call-order gap, since both
  # statements live in the one function and lexical order is execution order
  # inside it.
  body="$(awk '/^guarded_release_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' "$RELEASEBUILD_LIB" \
          | grep -vE '^[[:space:]]*#')"
  v_line="$(printf '%s\n' "$body" | grep -n 'shared_target_verdict' | head -1 | cut -d: -f1)"
  c_line="$(printf '%s\n' "$body" | grep -n 'cargo \(clean\|build\)' | head -1 | cut -d: -f1)"
  if [ -z "$body" ]; then
    fail "arm 7 -- could not extract guarded_release_build from $RELEASEBUILD_LIB; the function moved or was renamed, and an unread body is not an ordered one"
  elif [ -z "$v_line" ]; then
    fail "arm 7 -- guarded_release_build never consults the guard; the predicate exists and nothing calls it, which is a guard nothing dispatches"
  elif [ -z "$c_line" ]; then
    fail "arm 7 -- no cargo invocation inside guarded_release_build, so the order cannot be established"
  elif [ "$v_line" -lt "$c_line" ]; then
    ok "arm 7 -- the verdict is taken before the first cargo invocation inside guarded_release_build"
  else
    fail "arm 7 -- the verdict is taken AFTER cargo inside guarded_release_build. A correct verdict arriving after the shared artefact is overwritten reports the damage instead of preventing it, which is the defect this row was minted against."
  fi
fi

# --------------------------------------- ARM 8: A DIRTY BUILD IS REDIRECTED, NOT BLOCKED
# The guard must not be a freeze. AC-11.6 as amended sends a dirty build to a
# private CARGO_TARGET_DIR marked `dirty-<sha>`; without that redirect this guard
# would REFUSE the remedy another rostered guard prints -- `bin/intent3`'s
# currency check hands every node `int local build` when the shared pair is
# stale, and a stale pair on a dirty tree would then have no way forward at all.
# Two guards, one instructing a node to do what the other exists to prevent.
if [ ! -r "$LOCAL_CMD" ]; then
  : # already reported by arm 7
elif grep -q 'PRIVATE_RELEASE_DIR' "$RELEASEBUILD_LIB" && grep -q 'CARGO_TARGET_DIR' "$RELEASEBUILD_LIB"; then
  ok "arm 8 -- a refused build is redirected to a private CARGO_TARGET_DIR rather than blocked"
else
  fail "arm 8 -- no private redirect in $RELEASEBUILD_LIB; a guard that stops the build outright is a freeze, and a freeze gets bypassed"
fi


# ------------------------------- ARM 9: THE HOOK ENVIRONMENT DOES NOT LEAK IN
# **THE ARM THAT DID NOT EXIST WHEN THIS FILE WENT GREEN, AND ITS ABSENCE
# BLOCKED EVERY COMMIT IN THE ESTATE.** `git -C <dir>` changes the WORKING
# DIRECTORY and does NOT override `GIT_DIR`, `GIT_INDEX_FILE` or `GIT_WORK_TREE`
# -- and git EXPORTS ALL THREE INTO HOOK ENVIRONMENTS. Rostered `gated`, this
# file first ran from inside the pre-commit hook, where its fixtures' `git add`
# staged into THE OUTER REPOSITORY'S index, naming an object that repository
# cannot resolve. `error: Error building trees`, on every commit, for every node.
#
# **THE HARNESS WAS THE ONE ENVIRONMENT WHERE THE DEFECT COULD NOT APPEAR.** Nine
# arms, mutation-proven four ways, all run standalone -- where `GIT_DIR` is unset
# and `git -C` is sufficient. **Mutation testing varies the SUBJECT and holds the
# ENVIRONMENT fixed, so no number of mutations could have found this.** The
# estate's own class, arriving one axis over: not an instrument scoped narrower
# than its criterion, but an instrument whose HARNESS excluded the failure.
#
# **AND IT WAS NOT ONLY THE FIXTURES.** The same inheritance made
# `shared_target_verdict` answer about the HOOK'S repository rather than the one
# it was handed -- measured, with `GIT_DIR` set and cwd an empty fixture, `status`
# printing the outer repository's tracked paths as deleted. **A predicate whose
# whole contract is _about the tree you named_ was silently about a different
# one.** Arms 1-6 would have passed anyway, reading the outer tree and getting
# plausible answers from it, which is why this arm asserts the SUBJECT and not
# just the absence of damage.
outer="$TMP/outer"; inner="$TMP/inner"
mkdir -p "$outer" "$inner"
if ! scratch_repo "$outer" || ! scratch_repo "$inner"; then
  fail "arm 9 could not build its two fixtures"
else
  # `outer` is DIRTY, `inner` is CLEAN. Under a leaking environment the verdict
  # about `inner` comes back as the dirty answer, which is the whole tell.
  printf 'fn main() { /* the outer tree is dirty */ }\n' > "$outer/native/rust/crates/thing/src/main.rs"
  outer_index_before="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  verdict="$(GIT_DIR="$outer/.git" GIT_INDEX_FILE="$outer/.git/index" shared_target_verdict "$inner")"
  outer_index_after="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  case "$verdict" in
    ok) ok "arm 9 -- with GIT_DIR set, the verdict is still about the tree it was HANDED" ;;
    *)  fail "arm 9 -- with GIT_DIR set, a CLEAN subject returned '$verdict'; the predicate answered about the hook's repository rather than the one it was given" ;;
  esac
  # The second half: a fixture must never write to the ambient index. This is the
  # arm that would have caught the estate-wide block directly.
  scratch_repo "$TMP/leak" >/dev/null 2>&1 || true
  GIT_DIR="$outer/.git" GIT_INDEX_FILE="$outer/.git/index" scratch_repo "$TMP/leak2" >/dev/null 2>&1 || true
  outer_index_final="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  if [ "$outer_index_after" = "$outer_index_final" ] && [ -n "$outer_index_after" ]; then
    ok "arm 9 -- building a fixture under a set GIT_INDEX_FILE does not touch the ambient index"
  else
    fail "arm 9 -- building a fixture MUTATED the ambient index ($outer_index_after -> $outer_index_final). This is the estate-wide commit block: the fixture stages into the outer repository, naming an object it cannot resolve."
  fi
fi

# ARM 10 -- THE ENTRANCE CENSUS, AND IT IS THE ARM THAT CLOSES THE CLASS RATHER
# THAN THE INSTANCE.
#
# Arms 7 and 8 read the ONE guarded build and say nothing about how many other
# doors exist. That is exactly how this defect lived: `int build all`,
# `int build cli`, `int build daemon` and `cmd/cli`'s fallback each wrote the
# shared artefact with a bare `cargo build` while arm 7 truthfully reported that
# the guarded path consulted the verdict. **The arm was right; its subject was
# one door of five.** Nothing here would stop a sixth being added tomorrow, so
# this arm asks the question those arms structurally cannot.
#
# THE PREDICATE: a cargo invocation that CAN produce a RELEASE build must carry,
# in its own file, evidence that it cannot reach the shared tree --
# `clone_workspace` (it builds inside a clone), `CARGO_TARGET_DIR` (redirected),
# `refuse_single_package_release` (it refuses first), or `guarded_release_build`
# (it IS the guarded path). A debug-only invocation needs none of these: that is
# the stated exception, and it is stated because a debug build exists to run
# uncommitted code.
#
# THE COUNT IS THIS ARM'S ONLY EVIDENCE THAT IT LOOKED AT ANYTHING. Its first
# draft reported `(1 examined)` against a tree carrying four release-capable
# invocations, because `grep -n` on a SINGLE file emits `<line>:<text>` with no
# filename, so `cut -d: -f3-` discarded the statement and kept nothing. It passed.
# **A census that examines one of four and reports green is the false-green this
# whole file exists to refuse**, so the number is printed and is the thing to
# read first.
#
# FILE-SCOPED EVIDENCE, DELIBERATELY, AND ITS WEAKNESS IS NAMED RATHER THAN
# HIDDEN: a file could carry `clone_workspace` for one invocation and build the
# shared tree in another. That is a weaker check than per-invocation dataflow and
# a far stronger one than none, which is what exists today. It cannot be fooled
# by accident -- only by writing the token deliberately -- and this file's own
# history says the failure mode to design against is a door nobody noticed, not
# a door someone disguised.
step_cargo_census() {
  local f line stmt bad="" total=0
  while IFS= read -r f; do
    # A line that can be release: an explicit `--release`, or a `$profile`
    # variable that expands to it. Comments and prose ABOUT cargo are excluded --
    # this arm's own ancestor failed a correct file by matching a header
    # sentence, which is this estate's oldest instrument defect.
    while IFS= read -r line; do
      # BACKTICKED SPANS ARE PROSE, NOT STATEMENTS, AND STRIPPING THEM FIRST IS
      # NOT OPTIONAL. The `#`-comment filter above does not reach text inside a
      # `cat <<'USAGE'` heredoc: this arm's first working draft failed
      # `cmd/macos` on its own help text -- "and any \`cargo build --release\`
      # will overwrite underneath you" -- a sentence WARNING about the very
      # hazard, read as the hazard. That is the defect arm 7's header already
      # records as this estate's oldest, met again by the arm written to widen
      # it. It also silences the two `printf` lines in `cmd/cache` and
      # `cmd/local` that name `cargo clean` while telling an operator what to do.
      stmt="$(printf '%s' "$line" | sed 's/`[^`]*`//g')"
      case "$stmt" in
        *"cargo build"*|*"cargo clean"*) ;;
        *) continue ;;
      esac
      case "$stmt" in
        *--release*|*'$profile'*) ;;
        *) continue ;;
      esac
      total=$((total + 1))
      if grep -q 'clone_workspace\|CARGO_TARGET_DIR\|refuse_single_package_release\|guarded_release_build' "$f"; then
        continue
      fi
      bad="$bad       $f: ${line#"${line%%[![:space:]]*}"}
"
    done <<EOF
$(grep -n 'cargo build\|cargo clean' "$f" 2>/dev/null | grep -vE '^[0-9]+:[[:space:]]*#' | cut -d: -f2-)
EOF
  done <<EOF
$(find "$ROOT/bin/.devbin" -type f 2>/dev/null | sort)
EOF
  if [ -n "$bad" ]; then
    fail "arm 10 -- a cargo invocation that can build RELEASE reaches the shared artefact with no guard:
$bad       Each must build in a clone, redirect CARGO_TARGET_DIR, refuse first, or BE
       the guarded path. A bare release build here is a new unguarded entrance,
       which is what split the pair on 2026-08-26."
  else
    ok "arm 10 -- every release-capable cargo invocation under bin/.devbin is guarded, redirected, cloned or refusing ($total examined)"
  fi
}
step_cargo_census

printf 'shared-artefact-guard: %d arm(s) passed\n' "$pass"
exit "$rc"
