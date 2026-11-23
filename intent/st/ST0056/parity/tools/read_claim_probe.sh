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
# EVERY DECLARED ROW IS RUN, and only those: the population is read from the
# table, and the invocation map below is checked against it both ways before
# anything runs (issue Iae1e02a). A probe that quietly omitted a row would
# report a population it never covered -- the failure `surface_check.sh`'s INV
# skip-list already had once.
#
# It probes the v3 binary (issue 0330); `BIN` is a parameter so another build
# can be measured against the same corpus.
#
# MUTATION-TESTED, both arms, because a check returning zero findings on its
# first run is the shape this ST has been bitten by seven times:
#   A -- a genuine mutator (`st new`) run through the read harness is caught,
#        with the created files in the diff.
#   B -- a row removed from the invocation map is reported UNCOVERED, so the
#        corpus cannot silently shrink; a line naming a row the table does not
#        declare exposed+read is reported STALE, so it cannot silently grow.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"

# Issue 0330: the v3 binary. `bin/intent` was the v2 shell, deleted under AC-00.6.
BIN="${BIN:-$REPO_ROOT/native/rust/target/release/intent}"
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
# Issue If83ae7c: each id is the one the mint printed, read by mint.lib, never
# spelled. After ST0095 the first `st new` is not ST0001, and a probe of an id
# that was never made measures a refusal, not a read.
# shellcheck source=bin/.devbin/cmd/shared/mint.lib
. "$REPO_ROOT/bin/.devbin/cmd/shared/mint.lib" || die "could not source mint.lib"
out="$(cd "$PROJ" && "$BIN" st new "Probe Thread" 2>&1)" || die "could not create a scratch steel thread: $out"
ST="$(minted_id "$out")" || die "st new printed no id it minted: $out"
out="$(cd "$PROJ" && "$BIN" wp new "$ST" "Probe WP" 2>&1)" || die "could not create a scratch work package: $out"
WP="$(minted_id "$out")" || die "wp new printed no id it minted: $out"
out="$(cd "$PROJ" && "$BIN" issues add "Probe issue" 2>&1)" || die "could not create a scratch issue: $out"
ISSUE="$(minted_id "$out")" || die "issues add printed no id it minted: $out"
# A build before ST0095 WP-02 prints the issue's canon path on that line, and a
# probe of a path measures a refusal, not a read.
case "$ISSUE" in */* | *.json) die "issues add printed a path, not an id (a build before ST0095 WP-02?): $ISSUE" ;; esac
# A criterion's label is the caller's, not minted: the thread's own group.
AC="AC-00.1"
out="$(cd "$PROJ" && "$BIN" ac new "$ST" "$AC" --text "Probe criterion" 2>&1)" || die "could not create a scratch criterion: $out"

# --- the harness --------------------------------------------------------------
# What a read must leave alone is the project's state: every file, and what
# the store owns. NOT its runtime files, which any open of the store makes:
# SQLite's -wal and -shm sidecars, a lock, and the rolling snapshot under
# intent/.backup/; and NOT the derived search index, which a read may refresh
# (issue Iae1e02a, where those were the only changes graphql, id and wb brief
# made). What the store owns is read through `intent export --format json`,
# the store's own projection of its records and events, so this script keeps
# no list of tables of its own, and a write still in the WAL is seen.
snap() {
  (cd "$PROJ" && find . -type f ! -path './intent/.backup/*' ! -name '*.db' ! -name '*.db-wal' \
    ! -name '*.db-shm' ! -name '*.lock' -print0 | sort -z | xargs -0 shasum 2>/dev/null)
  local owned
  owned="$(cd "$PROJ" && "$BIN" export --format json 2>&1 </dev/null)" || owned="UNREADABLE: $owned"
  printf '%s  (what the store owns)\n' "$(shasum <<<"$owned" | cut -d' ' -f1)"
}

MUTATORS=""
PROBED=0
run_one() {
  local label="$1"; shift
  local before after rc
  before="$(snap)"
  (cd "$PROJ" && "$BIN" "$@" >/dev/null 2>&1 </dev/null); rc=$?
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

# THE INVOCATION MAP, AND THE ONE LIST THE PROBE READS (issue Iae1e02a). Each
# line is a declared path, a tab, and the arguments that run it, with {ST},
# {WP}, {AC} and {ISSUE} standing for the ids minted above. Arguments cannot
# be derived from the table -- it records what a command IS, not a runnable
# example -- so they are hand-supplied, and no argument holds a space.
#
# The map and the table are checked against each other BOTH WAYS before
# anything runs. A declared row the map lacks is UNCOVERED; a map line whose
# path the table does not declare `exposed_on_mcp: true` + `read_or_mutate:
# read` is STALE. Both refuse. The probe used to check only the first: a
# second list named what it covered, and the rows it ran were spelled in the
# calls, so `st edit` and `doctor`, declared `mutate`, were run anyway and
# reported as mislabelled reads when the table was right.
MAP="$(cat <<'MAP'
ac gate	ac gate {ST}
ac list	ac list {ST}
ac show	ac show {ST} {AC}
ac status	ac status {ST}
agents validate	agents validate
at lint	at lint {ST}
at list	at list {ST}
estate status	estate status
events	events
graphql	graphql {__typename}
id	id {ST}
index status	index status
issues list	issues list
issues show	issues show {ISSUE}
outstanding	outstanding
schema	schema
search	search probe
st list	st list
st show	st show {ST}
todo	todo
todo list	todo list
wb brief	wb brief cc
wb profile	wb profile list
wb show	wb show cc
wb status	wb status
wp list	wp list {ST}
wp show	wp show {WP}
MAP
)"

DECLARED="$(jq -r '[.families[].entries[], .new_surface[]][]
  | select(.exposed_on_mcp == true and .read_or_mutate == "read") | .path' "$TABLE" | sort -u)"
[ -n "$DECLARED" ] || die "the table declares no exposed+read rows -- with a populated table that is a bad query, and probing nothing while reporting a pass is the failure this whole toolchain exists for"
MAPPED="$(cut -f1 <<<"$MAP" | sort -u)"
[ "$(cut -f1 <<<"$MAP" | sort | uniq -d)" = "" ] || die "the map names a path twice: $(cut -f1 <<<"$MAP" | sort | uniq -d | tr '\n' ' ')"

# Paths hold spaces, so each list is compared line by line, never word-split.
UNCOVERED="$(comm -23 <(printf '%s\n' "$DECLARED") <(printf '%s\n' "$MAPPED"))"
STALE="$(comm -13 <(printf '%s\n' "$DECLARED") <(printf '%s\n' "$MAPPED"))"
if [ -n "$STALE" ]; then
  printf 'read-claim: FINDING -- the map runs rows the table does not declare exposed+read:\n'
  while IFS= read -r path; do
    printf '  %-18s %s\n' "$path" "$(jq -r --arg p "$path" '[.families[].entries[], .new_surface[]][]
      | select(.path == $p) | "declared read_or_mutate: \(.read_or_mutate), exposed_on_mcp: \(.exposed_on_mcp)"' "$TABLE" | head -1)"
  done <<<"$STALE"
  printf '  -- the table is the population: remove each line from the map, or correct its declaration in surface/dispatch-table.json. Nothing was run.\n'
  exit 1
fi
if [ -n "$UNCOVERED" ]; then
  # An unprobed `read` claim is a promise to an agent with nothing behind it,
  # and a reclassification is the cheapest way to acquire one: it adds no row
  # and no code (doctor, 2026-08-16; export, the same day).
  printf 'read-claim: FINDING -- declared `read` but never run: %s\n' "$(tr '\n' ' ' <<<"$UNCOVERED")"
  printf '  -- add a line for each to the map with the arguments that run it. Nothing was run.\n'
  exit 1
fi

printf 'read-claim: probing rows declared `exposed_on_mcp: true` + `read_or_mutate: read`\n'
while IFS=$'\t' read -r path args; do
  args="${args//\{ST\}/$ST}"; args="${args//\{WP\}/$WP}"
  args="${args//\{AC\}/$AC}"; args="${args//\{ISSUE\}/$ISSUE}"
  read -r -a argv <<<"$args"
  run_one "$path" "${argv[@]}"
done <<<"$MAP"

printf '\nread-claim: %d row(s) declare exposed+read; %d probed\n' \
  "$(printf '%s\n' "$DECLARED" | grep -c .)" "$PROBED"
if [ -n "$MUTATORS" ]; then
  printf 'read-claim: FINDING -- rows declared `read` that CHANGED THE TREE:%s\n' "$MUTATORS"
  printf '  -- an agent is told these are safe to call. Fix the declaration or the command.\n'
  exit 1
fi
printf '  every probed read-claim held: nothing on disk changed.\n'
exit 0
