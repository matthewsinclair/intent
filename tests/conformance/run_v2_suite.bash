#!/usr/bin/env bash
# AT-05.2 / AT-00.1 / AT-06.1 -- the conformance harness.
#
# Runs the v2 BATS estate against a chosen `intent` binary and reports the
# result per file, classified. The v3 binary is certified by the incumbent:
# v3 is green when v2's own suite cannot tell the difference, except where the
# parity contract says so IN ADVANCE (parity.md's ratified deviation classes).
#
# WHY A RUNNER AND NOT JUST `bats tests/`.
#
# Three things this does that a bare `bats` invocation cannot:
#
#  1. It RETARGETS. ic's seam is `INTENT_BIN` (tests/lib/test_helper.bash:21,
#     `INTENT_BIN="${INTENT_BIN:-${INTENT_BIN_DIR}/intent}"`), and without it
#     every run silently measures v2 -- a green that means nothing, which is
#     worse than a red.
#  2. It BURNS FIRST. Before trusting any result it points INTENT_BIN at
#     /usr/bin/false and requires the named files to FAIL. A retargeted suite
#     that cannot fail proves nothing, and this is the check that catches a
#     file which resolves `intent` some other way and never sees the override.
#  3. It CLASSIFIES. A failure is only useful next to the reason it was
#     expected, so output is register-shaped -- one row per file -- rather than
#     a pass rate.
#
# EXPECT RED, AND EXPECT IT FOR RATIFIED REASONS. v3 changes file layout by
# ratification (parity.md), so every v2 test asserting bytes in a file that is
# now a generated view SHOULD fail. A first run that came back green would mean
# the file list had been scoped to what already passes.
#
# Usage:
#   bash tests/conformance/run_v2_suite.bash <file>...      # named files
#   INTENT_BIN=/path/to/intent bash ... <file>...           # explicit binary
#   BURN=0 bash ... <file>...                               # skip the burn-in
#
# The full 98-file estate is deliberately NOT the default: hv runs the full
# suite externally, and a harness that invites a long unattended run from a
# tool session is a harness that will be run that way by accident.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"

die() {
  echo "error: $1" >&2
  exit 1
}

command -v bats >/dev/null 2>&1 || die "bats is required and was not found on PATH"

[ $# -gt 0 ] || die "name at least one .bats file -- the full estate is hv's to run, not this harness's default"

# The binary under test. Defaults to the v3 build so a bare invocation measures
# what this work package is about, never the incumbent by accident.
INTENT_BIN="${INTENT_BIN:-$ROOT/target/debug/intent}"
[ -x "$INTENT_BIN" ] || die "no executable at $INTENT_BIN -- run 'cargo build' first, or set INTENT_BIN"

FILES=("$@")
for f in "${FILES[@]}"; do
  [ -f "$f" ] || die "no such test file: $f"
done

echo "conformance: $INTENT_BIN"
echo "conformance: $(${INTENT_BIN} --version 2>/dev/null || echo 'version unavailable')"
echo "conformance: ${#FILES[@]} file(s)"
echo

# ---------------------------------------------------------------------------
# The burn-in. A retargeted suite that cannot fail proves nothing.
# ---------------------------------------------------------------------------
#
# Point INTENT_BIN at a binary that always fails and require each file to go
# red. A file that stays GREEN under /usr/bin/false never reached the override
# -- it resolved `intent` from PATH, or invoked `bin/intent_<sub>` directly, or
# tests something that does not touch the CLI at all. Any of those means its
# result in the real run is not evidence about the v3 binary.
burn() {
  local burned=0 unburned=()
  for f in "${FILES[@]}"; do
    if INTENT_BIN=/usr/bin/false bats "$f" >/dev/null 2>&1; then
      unburned+=("$f")
    else
      burned=$((burned + 1))
    fi
  done
  echo "burn-in: $burned/${#FILES[@]} file(s) fail against /usr/bin/false"
  if [ ${#unburned[@]} -gt 0 ]; then
    echo "burn-in: these files pass even with a broken binary, so their result below is NOT evidence about the CLI:"
    for f in "${unburned[@]}"; do
      echo "  unburned: $f"
    done
  fi
  echo
}

[ "${BURN:-1}" = "1" ] && burn

# ---------------------------------------------------------------------------
# The real run, one row per file.
# ---------------------------------------------------------------------------
pass=0
fail=0
declare -a rows

for f in "${FILES[@]}"; do
  out="$(INTENT_BIN="$INTENT_BIN" bats "$f" 2>&1)"
  status=$?
  total="$(echo "$out" | grep -cE '^(ok|not ok) ' || true)"
  notok="$(echo "$out" | grep -cE '^not ok ' || true)"
  if [ "$status" -eq 0 ]; then
    pass=$((pass + 1))
    rows+=("$(printf '| %-52s | %5s | %5s | GREEN  |' "$f" "$total" "0")")
  else
    fail=$((fail + 1))
    rows+=("$(printf '| %-52s | %5s | %5s | RED    |' "$f" "$total" "$notok")")
    # The failing case names, so a register row can be written from this
    # output without re-running anything.
    echo "$out" | grep -E '^not ok ' | sed 's/^/    /' >> "${FAILLOG:-/dev/null}"
  fi
done

printf '| %-52s | %5s | %5s | %-6s |\n' "FILE" "TESTS" "FAILED" "RESULT"
printf '| %-52s | %5s | %5s | %-6s |\n' "$(printf -- '-%.0s' {1..52})" "-----" "-----" "------"
for row in "${rows[@]}"; do
  echo "$row"
done

echo
echo "conformance: $pass green, $fail red, of ${#FILES[@]} file(s)"
echo
echo "A RED here is not automatically a defect. parity.md ratifies the deviation"
echo "classes in advance -- file layout, issues directories, generated-view"
echo "banners, manual-edit workflows, and 'corrected' for a v2 behaviour that is"
echo "simply wrong. Classify each red keep/retire/deviate in the register; an"
echo "UNCLASSIFIED red is the defect, not a red as such."

# Exit non-zero when anything was red, so CI and `st done` can read it.
[ "$fail" -eq 0 ]
