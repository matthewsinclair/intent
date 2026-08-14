#!/bin/bash
# retarget.sh -- thread every top-level CLI invocation in the BATS estate through
# $INTENT_BIN. Run inside a sacrificial worktree; never against the live tree.
#
# Four spellings reach the top-level binary. They are distinguished from the 146
# `${INTENT_BIN_DIR}/intent_<sub>` sub-script calls by the character that FOLLOWS
# "intent": a quote or a space for the CLI, an underscore for a sub-script. An
# unanchored s|${INTENT_BIN_DIR}/intent|$INTENT_BIN| would rewrite
# `${INTENT_BIN_DIR}/intent_treeindex` into `$INTENT_BIN_treeindex` -- 146 silently
# broken call sites resolving to an unset variable.
#
# Every substitution asserts its expected count before rewriting and asserts zero
# survivors after. A perl -pi that matches nothing reports success, so an
# unasserted sweep cannot tell "already applied" from "pattern wrong".
#
# NOTE on `set -e`: the post-rewrite verification greps SUCCEED by finding
# nothing, which is exit 1. Every count is therefore taken through count(), which
# absorbs grep's no-match status. An earlier version of this script omitted that
# and died silently after the first sweep, leaving the estate half-rewritten --
# the same silent-partial-migration shape this contract exists to prevent.

set -uo pipefail

WT="${WT:?set WT to the worktree root}"
cd "$WT"

fail=0

# count <ere> -- occurrences across the estate; 0 rather than a failed pipeline.
count() {
  grep -rhoE "$1" tests --include='*.bats' 2>/dev/null | wc -l | tr -d ' '
}

# sweep <label> <expected> <ere> <perl-expr>
sweep() {
  local label="$1" want="$2" pat="$3" expr="$4" got left
  got=$(count "$pat")
  # Idempotent: zero occurrences of the OLD form plus a populated NEW form means
  # this sweep already ran. Distinguishing that from "pattern wrong" matters --
  # vc and WP-05 re-run this script, and a re-run that reports failure on
  # success trains people to ignore its output.
  if [ "$got" = "0" ] && [ "$(count '\$INTENT_BIN\b')" != "0" ]; then
    printf '  ok: %-26s already applied\n' "$label"
    return
  fi
  if [ "$got" != "$want" ]; then
    echo "retarget: $label -- expected $want, found $got; refusing to rewrite" >&2
    fail=1
    return
  fi
  for f in $(grep -rlE "$pat" tests --include='*.bats' 2>/dev/null); do
    perl -pi -e "$expr" "$f"
  done
  left=$(count "$pat")
  if [ "$left" != "0" ]; then
    echo "retarget: $label -- $left survived the rewrite" >&2
    fail=1
    return
  fi
  printf '  ok: %-26s %3s rewritten\n' "$label" "$want"
}

# --- 1. the helper: define INTENT_BIN and route run_intent through it ---------
H=tests/lib/test_helper.bash
if grep -q 'INTENT_BIN=' "$H"; then
  echo "  ok: helper already carries INTENT_BIN"
else
  perl -0pi -e 's{(INTENT_TEMP_DIR="\$\{INTENT_PROJECT_ROOT\}/tests/tmp"\n)}{$1\n# The CLI under test. Defaults to the shell implementation in this repo; set\n# INTENT_BIN in the environment to run the estate against another one (ST0056:\n# the v3 binary) without editing a single test.\n#\n# INTENT_BIN_DIR is NOT a substitute. It names a DIRECTORY of 27 scripts, and the\n# ~146 `\$\{INTENT_BIN_DIR\}/intent_<sub>` call sites invoke those directly,\n# bypassing the bin/intent dispatcher and everything it does (PROJECT_ROOT\n# resolution, INTENT_ORIG_CWD, cd to project root -- bin/intent:198-218). Those\n# have no equivalent under a single binary and are classified in the register,\n# not mechanically retargeted here.\nINTENT_BIN="\$\{INTENT_BIN:-\$\{INTENT_BIN_DIR\}/intent\}"\nexport INTENT_BIN\n}' "$H"
  perl -0pi -e 's{run_intent\(\) \{\n  "\$\{INTENT_BIN_DIR\}/intent" "\$\@"\n\}}{run_intent() \{\n  "\$INTENT_BIN" "\$\@"\n\}}' "$H"
  if grep -q 'INTENT_BIN=' "$H" && grep -q '"\$INTENT_BIN" "\$@"' "$H"; then
    printf '  ok: %-26s patched\n' 'test_helper.bash'
  else
    echo "retarget: helper patch did not apply" >&2
    fail=1
  fi
fi

# --- 2. the direct call sites ------------------------------------------------
sweep 'form A (double-quoted)' 71 \
  '"\$\{INTENT_BIN_DIR\}/intent"' \
  's{"\$\{INTENT_BIN_DIR\}/intent"}{"\$INTENT_BIN"}g'

sweep 'form B (single-quoted)' 14 \
  "'\\\$\\{INTENT_BIN_DIR\\}/intent'" \
  "s{'\\\$\\{INTENT_BIN_DIR\\}/intent'}{'\\\$INTENT_BIN'}g"

sweep 'form C (bare + space)' 10 \
  '\$\{INTENT_BIN_DIR\}/intent ' \
  's{\$\{INTENT_BIN_DIR\}/intent }{\$INTENT_BIN }g'

sweep 'form D (INTENT_HOME)' 8 \
  "'\\\$INTENT_HOME/bin/intent'" \
  "s{'\\\$INTENT_HOME/bin/intent'}{'\\\$INTENT_BIN'}g"

# --- 2b. the unbraced form ---------------------------------------------------
# Two occurrences of "$INTENT_BIN_DIR/intent" without braces, textually identical
# and needing OPPOSITE treatment, so this is file-scoped rather than swept:
#   claude_with_intent.bats:12  INTENT="$INTENT_BIN_DIR/intent"   -- an alias 14
#     tests invoke through; retarget it or those tests silently keep exercising
#     the shell CLI no matter what INTENT_BIN is set to. Found by burn-ratio, not
#     by the sweep: the file reported zero burn while looking like a CLI test.
#   basic.bats:12  assert_file_exists "$INTENT_BIN_DIR/intent"    -- asserts the
#     shell binary is PRESENT in this repo. Not an invocation. Retargeting it
#     would quietly convert a layout assertion into a v3 assertion; it is a
#     register retire instead.
CWI=tests/unit/claude_with_intent.bats
if grep -q 'INTENT="\$INTENT_BIN_DIR/intent"' "$CWI"; then
  perl -pi -e 's{INTENT="\$INTENT_BIN_DIR/intent"}{INTENT="\$INTENT_BIN"}' "$CWI"
  if grep -q 'INTENT="\$INTENT_BIN"' "$CWI"; then
    printf '  ok: %-26s %3s rewritten\n' 'form E (unbraced alias)' 1
  else
    echo "retarget: form E did not apply in $CWI" >&2; fail=1
  fi
else
  printf '  ok: %-26s already applied\n' 'form E (unbraced alias)'
fi

if ! grep -q 'assert_file_exists "\$INTENT_BIN_DIR/intent"' tests/unit/basic.bats; then
  echo "retarget: basic.bats layout assertion was rewritten; it must not be" >&2
  fail=1
else
  printf '  ok: %-26s preserved\n' 'basic.bats layout assert'
fi

# --- 3. what must NOT have changed -------------------------------------------
subs=$(count '\$\{INTENT_BIN_DIR\}/intent_[a-z_]+')
if [ "$subs" != "146" ]; then
  echo "retarget: sub-script call sites went 146 -> $subs; the sweep bled" >&2
  fail=1
else
  printf '  ok: %-26s %3s untouched (register bucket)\n' 'sub-script call sites' 146
fi

corrupt=$(count '\$INTENT_BIN_[a-z]')
if [ "$corrupt" != "0" ]; then
  echo "retarget: \$INTENT_BIN_<suffix> x$corrupt -- a substitution ate an underscore" >&2
  fail=1
else
  printf '  ok: %-26s none\n' 'corruption signature'
fi

exit "$fail"
