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
  git -C "$d" init -q 2>/dev/null || return 1
  mkdir -p "$d/native/rust/crates/thing/src"
  printf 'fn main() {}\n' > "$d/native/rust/crates/thing/src/main.rs"
  printf 'placeholder\n' > "$d/README.md"
  git -C "$d" add -A >/dev/null 2>&1 || return 1
  git -C "$d" -c user.email=guard@test -c user.name=guard commit -qm base >/dev/null 2>&1 || return 1
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
if [ ! -r "$MARKER_SRC" ]; then
  fail "arm 6 -- no marker source at $MARKER_SRC, so scope agreement cannot be checked"
else
  marker_scope="$(sed -n 's/^const DIRT_SCOPE: &str = "\(.*\)";$/\1/p' "$MARKER_SRC")"
  if [ -z "$marker_scope" ]; then
    fail "arm 6 -- could not read DIRT_SCOPE from $MARKER_SRC; the constant moved or was renamed, and an unread scope is not an agreeing one"
  elif [ "$marker_scope" = "$SHARED_TARGET_DIRT_SCOPE" ]; then
    ok "arm 6 -- guard scope and marker DIRT_SCOPE agree ($marker_scope)"
  else
    fail "arm 6 -- SCOPE DISAGREEMENT: guard uses '$SHARED_TARGET_DIRT_SCOPE', marker uses '$marker_scope'. The shared slot could hold an artefact this guard approved and the artefact disowns."
  fi
fi

# ------------------------------------------- ARM 7: THE VERDICT PRECEDES THE BUILD
# THE ORDER IS THE CRITERION. `verify_pair` already refused a `dirty-` marker
# before this guard existed; it ran after `cargo build` had replaced the shared
# binaries, so it named the damage instead of preventing it. AC-11.6 weighed that
# exact ordering and ruled for the earlier refusal in its own words -- before the
# artefact exists, rather than producing a working-looking binary four nodes then
# invoke. An outcome arm cannot see this; a correct verdict arriving late passes
# every one of arms 1 to 6.
if [ ! -r "$LOCAL_CMD" ]; then
  fail "arm 7 -- no build command at $LOCAL_CMD"
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
  body="$(awk '/^cmd_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' "$LOCAL_CMD" \
          | grep -vE '^[[:space:]]*#')"
  v_line="$(printf '%s\n' "$body" | grep -n 'shared_target_verdict' | head -1 | cut -d: -f1)"
  c_line="$(printf '%s\n' "$body" | grep -n 'cargo \(clean\|build\)' | head -1 | cut -d: -f1)"
  if [ -z "$body" ]; then
    fail "arm 7 -- could not extract cmd_build from $LOCAL_CMD; the function moved or was renamed, and an unread body is not an ordered one"
  elif [ -z "$v_line" ]; then
    fail "arm 7 -- cmd_build never consults the guard; the predicate exists and nothing calls it, which is a guard nothing dispatches"
  elif [ -z "$c_line" ]; then
    fail "arm 7 -- no cargo invocation inside cmd_build, so the order cannot be established"
  elif [ "$v_line" -lt "$c_line" ]; then
    ok "arm 7 -- the verdict is taken before the first cargo invocation inside cmd_build"
  else
    fail "arm 7 -- the verdict is taken AFTER cargo inside cmd_build. A correct verdict arriving after the shared artefact is overwritten reports the damage instead of preventing it, which is the defect this row was minted against."
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
elif grep -q 'PRIVATE_RELEASE_DIR' "$LOCAL_CMD" && grep -q 'CARGO_TARGET_DIR' "$LOCAL_CMD"; then
  ok "arm 8 -- a refused build is redirected to a private CARGO_TARGET_DIR rather than blocked"
else
  fail "arm 8 -- no private redirect in $LOCAL_CMD; a guard that stops the build outright is a freeze, and a freeze gets bypassed"
fi

printf 'shared-artefact-guard: %d arm(s) passed\n' "$pass"
exit "$rc"
