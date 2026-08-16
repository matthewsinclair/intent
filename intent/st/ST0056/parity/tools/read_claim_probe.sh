#!/usr/bin/env bash
# read_claim_probe.sh -- does a row declaring `read_or_mutate: read` actually
# leave the filesystem alone? (AC-09.1's field, witnessed rather than asserted.)
#
# WHY THIS EXISTS AND WHY THE GENERATOR IS NOT ENOUGH.
# AC-09.1 makes `read_or_mutate` load-bearing for agent safety in as many words:
# a mutation mislabelled as a read "lets an agent close a steel thread believing
# it is querying". `gen_dispatch_table.sh` REFUSES a row that fails to declare
# the field -- but **a refusal guarantees a declaration EXISTS, never that it is
# TRUE**, and until this probe nothing in the toolchain witnessed the claim. The
# AC also forbids deriving the field from the verb, and the surface proves it
# right: `todo list` is a `list` verb that WRITES `intent/todo.md`, while `st
# edit` is the most obviously-mutating verb name in the table and resolves a
# path. Neither is derivable; both are checkable.
#
# THE EXPERIMENT IS SELF-LIMITING BY CONSTRUCTION, which is what makes running
# it safe at all: it invokes ONLY the rows that CLAIM to be read. If the claim
# holds, the run is harmless; if it does not, the harm lands in a throwaway
# project and IS the finding.
#
# WHAT IT CANNOT WITNESS, named rather than folded into the green. Rows with no
# v2 incumbent (`search`, `schema` on the new surface) have nothing to run and
# are SKIPPED BY NAME. A probe that quietly omitted them would report a
# population it never covered -- the failure `surface_check.sh`'s INV skip-list
# already had once.
#
# IT PROBES THE V2 INCUMBENT, deliberately: v2's behaviour is the parity basis
# these rows were measured from, and most v3 commands are unwired mid-ladder.
# Point `BIN` at the v3 binary once WP-09 lands and the same corpus re-runs
# unchanged -- that is the whole reason the binary is a parameter.
#
# MUTATION-TESTED, both arms, because a check returning zero findings on its
# first run is the shape this ST has been bitten by seven times:
#   A -- a genuine mutator (`st new`) run through the read harness is caught,
#        with the created files in the diff.
#   B -- a row removed from the invocation map is reported UNCOVERED and the
#        probed count drops, so the corpus cannot silently shrink.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"

BIN="${BIN:-$REPO_ROOT/bin/intent}"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"

die() { echo "error: $1" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"
[ -x "$BIN" ]   || die "no runnable binary at $BIN -- a probe with nothing to run is an inability to measure, not a pass"
[ -f "$TABLE" ] || die "no dispatch table at $TABLE"

# --- a throwaway project, built here rather than supplied ---------------------
# Self-contained on purpose: an instrument that needs a setup ritual gets run
# once. The refusal below is the one that matters -- these commands must never
# execute inside a real tree, and four sessions share this one.
PROJ="$(mktemp -d)"
[ -n "$PROJ" ] && [ -d "$PROJ" ] || die "could not create a scratch directory"
cleanup() { [ -n "${PROJ:-}" ] && rm -rf "$PROJ"; }
trap cleanup EXIT

probe_dir_is_inside_a_project() {
  local d="$1"
  while [ "$d" != "/" ] && [ -n "$d" ]; do
    [ -f "$d/intent/.config/config.json" ] && return 0
    d="$(dirname "$d")"
  done
  return 1
}
probe_dir_is_inside_a_project "$PROJ" \
  && die "the scratch directory $PROJ resolves inside an Intent project -- refusing to run mutation-capable probes in a live tree"

(cd "$PROJ" && "$BIN" init "ProbeProj" >/dev/null 2>&1) || die "could not init a scratch project with $BIN"
(cd "$PROJ" && "$BIN" st new "Probe Thread" >/dev/null 2>&1) || die "could not create a scratch steel thread"
(cd "$PROJ" && "$BIN" wp new ST0001 "Probe WP" >/dev/null 2>&1) || die "could not create a scratch work package"

# --- the harness --------------------------------------------------------------
snap() { (cd "$PROJ" && find . -type f -print0 | sort -z | xargs -0 shasum 2>/dev/null); }

MUTATORS=""
PROBED=0
run_one() {
  local label="$1"; shift
  local before after rc
  before="$(snap)"
  (cd "$PROJ" && "$BIN" "$@" >/dev/null 2>&1); rc=$?
  after="$(snap)"
  PROBED=$((PROBED + 1))
  if [ "$before" = "$after" ]; then
    printf '  read-ok    %-18s (exit %s)\n' "$label" "$rc"
  else
    printf '  MUTATED    %-18s (exit %s) -- DECLARED read AND CHANGED THE TREE:\n' "$label" "$rc"
    diff <(printf '%s\n' "$before") <(printf '%s\n' "$after") | sed 's/^/               /'
    MUTATORS="$MUTATORS $label"
  fi
}

# The invocation map. Arguments cannot be derived from the table -- it records
# what a command IS, not a runnable example -- so they are hand-supplied. The
# POPULATION is not: it is read from the table below, and a declared-read row
# missing from this map is reported UNCOVERED rather than silently skipped, so a
# row added later surfaces as a gap instead of shrinking the corpus.
printf 'read-claim: probing rows declared `exposed_on_mcp: true` + `read_or_mutate: read`\n'
run_one "st list"          st list
run_one "st show"          st show ST0001
run_one "st edit"          st edit ST0001
run_one "wp list"          wp list ST0001
run_one "wp show"          wp show ST0001/01
run_one "ac list"          ac list ST0001
run_one "ac status"        ac status ST0001
run_one "ac gate"          ac gate ST0001
run_one "at list"          at list ST0001
run_one "issues list"      issues list
run_one "issues show"      issues show 0001
run_one "info"             info
run_one "config get"       config get intent_version
run_one "agents validate"  agents validate
run_one "agents template"  agents template
run_one "lang list"        lang list
run_one "lang show"        lang show shell
run_one "modules check"    modules check
run_one "modules find"     modules find intent_helpers
run_one "plugin list"      plugin list
run_one "plugin show"      plugin show claude
run_one "ext list"         ext list
run_one "ext show"         ext show nonesuch
run_one "ext validate"     ext validate
run_one "version"          version
run_one "critic"           critic shell

COVERED="st list|st show|st edit|wp list|wp show|ac list|ac status|ac gate|at list|issues list|issues show|info|config get|agents validate|agents template|lang list|lang show|modules check|modules find|plugin list|plugin show|ext list|ext show|ext validate|version|critic"
# No v2 incumbent, so there is nothing to run -- named, never folded into a green.
# `llm guide` earned its place here by being CAUGHT: it was declared minutes
# after this probe landed and reported UNCOVERED on the next run, which is the
# whole point of deriving the population from the table rather than from this
# list. A row added later surfaces as a gap instead of shrinking the corpus.
# `export` earned its place the same way on 2026-08-16, and is the better story
# because nothing was ADDED: the row was reclassified `mutate` -> `read`, which
# moved it into this probe's population without touching a single line here.
NO_V2="search|schema|llm guide|export"

DECLARED="$(jq -r '[.families[].entries[], .new_surface[]][]
  | select(.exposed_on_mcp == true and .read_or_mutate == "read") | .path' "$TABLE" | sort -u)"
[ -n "$DECLARED" ] || die "the table declares no exposed+read rows -- with a populated table that is a bad query, and probing nothing while reporting a pass is the failure this whole toolchain exists for"

UNCOVERED=""; SKIPPED=""
# Paths contain spaces, so the list is walked with a record separator rather
# than by word-splitting -- `st list` is ONE row, not two.
for p in $(printf '%s\n' "$DECLARED" | tr ' ' '\036'); do
  path="$(printf '%s' "$p" | tr '\036' ' ')"
  printf '%s' "$COVERED" | tr '|' '\n' | grep -qxF "$path" && continue
  printf '%s' "$NO_V2"   | tr '|' '\n' | grep -qxF "$path" && { SKIPPED="$SKIPPED $path"; continue; }
  UNCOVERED="$UNCOVERED $path"
done

printf '\nread-claim: %d row(s) declare exposed+read; %d probed; no v2 incumbent, not probed:%s\n' \
  "$(printf '%s\n' "$DECLARED" | grep -c .)" "$PROBED" "${SKIPPED:- none}"
if [ -n "$UNCOVERED" ]; then
  # REFUSES rather than reports, since 2026-08-16. It printed and exited 0 until
  # `export` moved into the population by reclassification -- which showed that
  # the two lists above are only a discipline if being in NEITHER is an error.
  # An unprobed `read` claim is a promise to an agent with nothing behind it,
  # and this file's own siblings all refuse rather than default.
  #
  # The remedy is a decision, not a chore, which is why it is worth blocking on:
  # a row belongs in `COVERED` with an invocation (it has a v2 incumbent and the
  # claim can be witnessed) or in `NO_V2` (it cannot be, and that gets recorded).
  # Silence chose neither and looked identical to both.
  printf 'read-claim: FINDING -- declared `read` but never run:%s\n' "$UNCOVERED"
  printf '  -- add an invocation above and list it in COVERED, or name it in NO_V2 if it has no v2 incumbent.\n'
  exit 1
fi

if [ -n "$MUTATORS" ]; then
  printf 'read-claim: FINDING -- rows declared `read` that CHANGED THE TREE:%s\n' "$MUTATORS"
  printf '  -- an agent is told these are safe to call. Fix the declaration or the command.\n'
  exit 1
fi
printf '  every probed read-claim held: nothing on disk changed.\n'
exit 0
