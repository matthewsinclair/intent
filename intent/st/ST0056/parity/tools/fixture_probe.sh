#!/bin/bash
# fixture_probe.sh -- the SECOND predicate, beside burn.
#
# Burn asks: does this file reach the v2 CLI?
# This asks:  does this file hardcode a v2 estate path?
#
# THEY ARE DIFFERENT QUESTIONS AND ONLY THE FIRST WAS BEING MEASURED. Burn is a
# v2-side measurement by construction -- both its runs are v2, one with the
# binary redirected -- so it can see whether a test TALKS to the CLI and cannot
# see whether the test's own SETUP survives v3's file layout. A file can burn
# 12/12, earn `keep`, and still fail every one of those tests under v3 before a
# single assertion runs, because its fixture wrote to a directory v3 does not
# have. `keep` then means "safe to point at the v3 binary", which is a promise
# the burn evidence never made.
#
# Found by cc (2026-08-14 23:47Z), who ran the 31 `keep` files against the real
# v3 binary and got 8 that cannot construct their fixtures at all. Measured, not
# inferred: 17 of 17 reds across two files, one cause. This tool generalises
# their crude grep to the whole estate and separates two hazards their needle
# merges -- see below, because the distinction changes the remedy and therefore
# the cost.
#
# WHAT MAKES A PATH A HAZARD. Both come from the v3 file-layout table at
# design.md:45-56, and neither is a guess about v3:
#
#   status-dir  `intent/st/{COMPLETED,NOT-STARTED,CANCELLED}/` -- v3 holds
#               status as a FIELD in `st/<ID>/thread.json`. There is no status
#               directory to write into, so the write fails outright.
#
#   gen-view    a hand-written `info.md` / `acceptance.md` under an st path --
#               v3 GENERATES both. This one is nastier than a failed write: the
#               write succeeds and is then outvoted by regeneration, or refused
#               by the skew check. A test that sets up state this way is not
#               broken loudly; it is wrong quietly.
#
# Authored prose (`design.md`, `impl.md`, `tasks.md`) stays authored in v3, so a
# fixture touching those is NOT flagged. The point of separating the classes is
# that they are not the same repair. A file that hand-builds the estate converts
# to CLI-built fixtures, which is real work. A file that builds through the CLI
# and then reaches in at a literal path needs the path resolved, which is not.
#
# THIS TOOL REPORTS EXPOSURE, NOT BREAKAGE, and the line matters. Whether a
# given file actually goes red under v3 is a v3-side question and belongs to
# whoever is running it there. What is measurable from here is whether the file
# hardcodes a layout assumption at all -- the necessary condition. Reporting the
# necessary condition as if it were the sufficient one is the same error as
# reading burn as portability, committed twice in one artefact.
#
# RUNS NO TESTS. Pure static read, safe while a peer holds the estate.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

[ -d "$ROOT/tests" ] || { echo "fixture_probe: no tests/ under ROOT=$ROOT" >&2; exit 2; }

# ---------------------------------------------------------------------------
# THE NEEDLE VALIDATES ITSELF AGAINST THE SOURCE OF TRUTH.
#
# The status directories are not a list worth hardcoding from memory: v2 creates
# exactly three (`bin/intent_st` mkdir -p, and the same three are the search
# roots in `bin/intent_helpers`), and a WIP thread has NO status directory at
# all -- it sits at `intent/st/<ID>/` directly, which is why ST0056 does.
#
# `normalise_status` in bin/intent_st is a FILTER-TOKEN table, not a directory
# table; it emits WIP / TBC / HOLD, none of which is ever a directory. Building
# a needle from it -- the obvious move, since it looks like the vocabulary --
# gives three members that cannot match anything. That is harmless here (the
# needle would over-cover, and over-covering is the safe direction) but it is
# the sort of thing that reads as rigour while being wrong, so the list is taken
# from the mkdir and CHECKED, not transcribed.
# ---------------------------------------------------------------------------
STATUS_DIRS="COMPLETED NOT-STARTED CANCELLED"

verify_vocabulary() {
  local src="$ROOT/bin/intent_st" found d
  [ -f "$src" ] || { echo "fixture_probe: cannot read $src to verify the status vocabulary" >&2; return 1; }
  # The single line that creates them. If v2 grows a fourth status directory
  # this stops matching and the probe refuses rather than quietly under-reporting.
  found="$(grep -oE 'mkdir -p "\$BASE_DIR/[A-Z-]+"( "\$BASE_DIR/[A-Z-]+")*' "$src" | head -1)"
  [ -n "$found" ] || { echo "fixture_probe: could not locate the status-directory mkdir in $src -- vocabulary unverified, refusing" >&2; return 1; }
  for d in $STATUS_DIRS; do
    case "$found" in
      *"/$d\""*) ;;
      *) echo "fixture_probe: '$d' is in the needle but not in $src's mkdir -- vocabulary drifted, refusing" >&2; return 1 ;;
    esac
  done
  # And the other direction: a directory created there that the needle misses.
  for d in $(printf '%s' "$found" | grep -oE 'BASE_DIR/[A-Z-]+' | cut -d/ -f2); do
    case " $STATUS_DIRS " in
      *" $d "*) ;;
      *) echo "fixture_probe: $src creates '$d' but the needle does not look for it -- refusing rather than under-reporting" >&2; return 1 ;;
    esac
  done
  return 0
}

status_re() {
  local alt
  alt="$(printf '%s' "$STATUS_DIRS" | tr ' ' '|')"
  printf 'intent/st/(%s)/' "$alt"
}

count_re() { grep -cE "$1" "$2" 2>/dev/null | head -1; }

# ---------------------------------------------------------------------------
# A HAND-WRITTEN GENERATED VIEW needs one hop of variable resolution, and the
# calibration is what proved it. The first version of this needle looked for the
# literal path adjacent to a redirect, which reads as reasonable and finds
# nothing: the live pattern assigns the path to a variable first
# (`ACC="intent/st/NOT-STARTED/ST0001/acceptance.md"`) and writes through it
# three lines later (`cat > "$ACC"`). One regex cannot see across that gap, so
# it returned a clean zero for a file that does exactly the thing being looked
# for -- the canary refused it, which is the entire reason the canary is wired
# into the tool rather than left to the operator.
#
# So: collect the variables holding such a path, then count WRITES to either the
# literal or one of those variables. Deliberately NOT flagging a mere reference
# -- a test asserting the content of a generated view is a conformance test
# doing its job, and flagging it would invert the finding.
#
# One hop only, and that limit is stated rather than discovered: a path built in
# two steps (`D=".../ST0001"; ACC="$D/acceptance.md"`) is not resolved and would
# be missed. No file in this estate does that today; the canary is what would
# notice if one started.
# ---------------------------------------------------------------------------
GENVIEW_AWK='
  BEGIN { n = 0 }
  # A variable assigned a generated-view path under an st directory.
  match($0, /[A-Za-z_][A-Za-z0-9_]*=["'\''"]?[^"'\''" ]*intent\/st\/[^"'\''" ]*\/(info|acceptance)\.md/) {
    a = substr($0, RSTART, RLENGTH)
    eq = index(a, "=")
    vars[substr(a, 1, eq - 1)] = 1
  }
  {
    line = $0
    # Writes: redirect, tee, sed -i, or the DESTINATION of cp/mv.
    is_write = (line ~ /(^|[^0-9<>])>>?[ \t]*["'\''"]?[^ \t"'\''"]/) || (line ~ /\|[ \t]*tee /) || (line ~ /sed -i/) || (line ~ /^[ \t]*(cp|mv) /)
    if (!is_write) next
    if (line ~ /intent\/st\/[^"'\''" ]*\/(info|acceptance)\.md/) { n++; next }
    for (v in vars)
      if (line ~ ("\\$\\{?" v "\\}?")) { n++; next }
  }
  END { print n }
'

count_genview() { awk "$GENVIEW_AWK" "$1" 2>/dev/null; }

# ---------------------------------------------------------------------------
# CALIBRATION. A measuring instrument must be shown to report non-zero where it
# should AND zero where it should, before its output is believed -- especially
# when it reports zero, because a zero and a broken needle are indistinguishable
# by inspection. This has already bitten twice on this thread from two different
# nodes, so it is wired into the tool rather than left to whoever runs it.
#
# The controls are named files with known content, so a canary that stops being
# a canary (the file is edited, or retired) fails loudly instead of silently
# certifying a dead needle.
# ---------------------------------------------------------------------------
calibrate() {
  local re n fail=0
  re="$(status_re)"

  # POSITIVE: st_commands.bats hand-builds status directories in bulk.
  if [ -f "$ROOT/tests/unit/st_commands.bats" ]; then
    n="$(count_re "$re" "$ROOT/tests/unit/st_commands.bats")"
    [ "${n:-0}" -gt 0 ] || { echo "fixture_probe: CANARY FAILED -- status-dir needle finds nothing in st_commands.bats, which is known to hand-build them. The needle is dead; a zero from it would mean nothing." >&2; fail=1; }
  else
    echo "fixture_probe: CANARY MISSING -- tests/unit/st_commands.bats is gone; the positive control for the status-dir needle no longer exists." >&2; fail=1
  fi

  # NEGATIVE: core_functionality.bats drives the CLI and hardcodes no estate path.
  if [ -f "$ROOT/tests/integration/core_functionality.bats" ]; then
    n="$(count_re "$re" "$ROOT/tests/integration/core_functionality.bats")"
    [ "${n:-0}" -eq 0 ] || { echo "fixture_probe: CANARY FAILED -- status-dir needle fires on core_functionality.bats, which drives the CLI and hardcodes no estate path. The needle is over-matching, so every positive is suspect." >&2; fail=1; }
  else
    echo "fixture_probe: CANARY MISSING -- tests/integration/core_functionality.bats is gone; the negative control no longer exists." >&2; fail=1
  fi

  # POSITIVE for the gen-view needle: ac_offscope_states.bats writes acceptance.md
  # THROUGH A VARIABLE, which is the case the first needle silently missed.
  if [ -f "$ROOT/tests/unit/ac_offscope_states.bats" ]; then
    n="$(count_genview "$ROOT/tests/unit/ac_offscope_states.bats")"
    [ "${n:-0}" -gt 0 ] || { echo "fixture_probe: CANARY FAILED -- gen-view needle finds nothing in ac_offscope_states.bats, which writes acceptance.md by hand into an st path." >&2; fail=1; }
  else
    echo "fixture_probe: CANARY MISSING -- tests/unit/ac_offscope_states.bats is gone; the gen-view positive control no longer exists." >&2; fail=1
  fi

  # NEGATIVE for the gen-view needle, and this is the control that matters most:
  # the needle must NOT fire on a file that only READS or ASSERTS a generated
  # view. Without this, widening the needle until the positive control passes
  # would be indistinguishable from making it match everything -- which is the
  # failure mode that produced the 26-of-40 false positive on the other side of
  # this thread. Generated with a known-shape fixture rather than borrowed from
  # the estate, so it cannot rot into a non-control the way a real file can.
  local neg
  neg="$(mktemp "${TMPDIR:-/tmp}/fixture_probe_canary.XXXXXX")"
  cat > "$neg" <<'CANARY'
@test "reads but never writes a generated view" {
  run cat "intent/st/NOT-STARTED/ST0001/acceptance.md"
  assert_output --partial "AC-01.1"
  grep -q "AC-01.2" intent/st/NOT-STARTED/ST0001/acceptance.md
}
CANARY
  n="$(count_genview "$neg")"
  rm -f "$neg"
  [ "${n:-0}" -eq 0 ] || { echo "fixture_probe: CANARY FAILED -- gen-view needle fires on a file that only reads a generated view ($n hits). It is matching references, not writes, so every positive is suspect." >&2; fail=1; }

  return $fail
}

if [ "${1:-}" = "--calibrate" ]; then
  verify_vocabulary || exit 2
  calibrate || exit 2
  echo "fixture_probe: vocabulary verified against bin/intent_st; both needles behave on a positive and a negative control."
  exit 0
fi

verify_vocabulary || { echo "fixture_probe: refusing to report against an unverified vocabulary" >&2; exit 2; }
calibrate || { echo "fixture_probe: refusing to report from an uncalibrated needle" >&2; exit 2; }

SRE="$(status_re)"

printf 'file\tstatus_dir\tgen_view\texposure\n'
while IFS= read -r f; do
  rel="${f#$ROOT/}"
  sd="$(count_re "$SRE" "$f")"; sd="${sd:-0}"
  gv="$(count_genview "$f")"; gv="${gv:-0}"
  if   [ "$sd" -gt 0 ] && [ "$gv" -gt 0 ]; then exposure="status-dir+gen-view"
  elif [ "$sd" -gt 0 ]; then exposure="status-dir"
  elif [ "$gv" -gt 0 ]; then exposure="gen-view"
  else exposure="none"
  fi
  printf '%s\t%s\t%s\t%s\n' "$rel" "$sd" "$gv" "$exposure"
done < <(find "$ROOT/tests" -name '*.bats' -type f | sort)
