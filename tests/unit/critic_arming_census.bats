#!/usr/bin/env bats
# AT-07.4 -- covers AC-07.4. The pre-commit critic gate, Half B.
#
# WHAT THIS ROW IS FOR, STATED SO A GREEN IS NOT MISREAD. AC-07.4's founding
# defect was not that rules were undeclared. It was SILENCE: all 13 shell and
# rust rules carried no proxy and no declaration, `critic_runner.sh` skipped a
# proxy-less rule without a word, and `intent critic shell` returned rc=0 having
# asked nothing -- printing a sentence INDISTINGUISHABLE from `intent critic
# elixir` after asking nine real questions.
#
# **SO "EVERY RULE IS DECLARED" IS NECESSARY AND NOT SUFFICIENT, AND THAT WAS MY
# OWN DONE-CONDITION UNTIL VC ASKED** (2026-08-19, flagged explicitly as not a
# ruling). A fully-declared roster that still returned rc=0 without saying what
# it skipped would satisfy an undeclared-is-empty test and fail the criterion
# outright. Arms 1 and 2 below are the necessary half; **arm 4 is the load-
# bearing one**, and without it arms 1-3 are a well-worded green over a runner
# that still asks nothing -- which is this thread's own defect, one turn later
# and better dressed.
#
# THE CRITERION IN ONE LINE, AND IT IS ARM 5: a run that ASKED must not be able
# to print the same sentence as a run that asked nothing. That is the whole of
# AC-07.4 stated as an observable, so it is asserted directly rather than left
# as motivation.
#
# WHY THIS FILE IS `.bats`. AT-07.4 first cited a Rust test that did not exist,
# at a time when the subject was the v2 shell runner. That runner was removed
# at the 3.0.1 cut, and every arm here drives the v3 `intent critic` through
# `$INTENT_BIN`.

load "../lib/test_helper.bash"

# THE DISPATCHER IS REACHED THROUGH `$INTENT_BIN`, NEVER BY PATH. Spelling the
# path runs v2's shell script whatever INTENT_BIN points at, so under a v3
# binary the test silently keeps testing v2 and reports green -- a green that
# means nothing, which is the class this whole file is about. I wrote the
# violation into the file arguing against it, and
# `intent_bin_retarget_guard.bats` caught it rather than any care of mine.
CRITIC="$INTENT_BIN"

# **A PATH THAT GENUINELY LACKS shellcheck ON ANY PLATFORM -- CONSTRUCTED, AND
# THEN VERIFIED.**
#
# THE PREVIOUS FORM WAS A HARDCODED LIST, AND ITS OWN COMMENT NAMED THE
# ASSUMPTION IT RESTED ON: `/opt/homebrew/bin` is where shellcheck lives *on
# this machine*. That is a macOS-with-Homebrew fact, and the list it left
# behind -- `/usr/bin:/bin:/usr/sbin:/sbin` -- is precisely where shellcheck
# lives on Linux. So on a GitHub `ubuntu-latest` runner, which ships shellcheck
# preinstalled in `/usr/bin`, the three absent-tool arms below ran with the
# tool PRESENT and asserted an absence that never happened.
#
# **IT WAS GREEN ON EXACTLY ONE MACHINE: the one the constant was written for.**
# The Linux leg has been red since it existed and the primary dev machine could
# not reproduce it, because on that machine the assumption is TRUE.
#
# DROPPING WHICHEVER DIRECTORY SHELLCHECK LIVES IN DOES NOT GENERALISE EITHER:
# on Linux `/bin` is a symlink to `/usr/bin`, so removing one leaves the other
# resolving the same binary, and removing both takes `sed`, `grep` and `awk`
# with it. **A critic that cannot run proves nothing about arming**, so an
# absence built that way trades a false green for a meaningless red.
#
# So the absence is BUILT -- a directory of symlinks to everything on PATH
# except shellcheck -- and then CHECKED BOTH WAYS before any test uses it.
# **The old constant asserted an absence and never once asked `command -v`**,
# which is exactly how it stayed wrong through every local run.
build_no_tool_path() {
  local farm="$1" dir entry base saved_ifs
  mkdir -p "$farm"
  saved_ifs="$IFS"
  IFS=:
  set -- $PATH
  IFS="$saved_ifs"
  for dir in "$@"; do
    [ -d "$dir" ] || continue
    for entry in "$dir"/*; do
      [ -f "$entry" ] && [ -x "$entry" ] || continue
      base="${entry##*/}"
      [ "$base" = "shellcheck" ] && continue
      [ -e "$farm/$base" ] && continue
      ln -s "$entry" "$farm/$base" 2>/dev/null || true
    done
  done
}

setup_file() {
  NO_TOOL_PATH="${BATS_FILE_TMPDIR}/no-shellcheck-bin"
  build_no_tool_path "$NO_TOOL_PATH"
  export NO_TOOL_PATH

  # **BOTH DIRECTIONS, BECAUSE EACH FAILS SILENTLY ON ITS OWN.** A farm that
  # still resolves shellcheck turns the absent-tool arms into a second copy of
  # the present-tool arms -- three tests asserting nothing, reporting green.
  # A farm that lost the coreutils makes the critic fail for a reason that has
  # nothing to do with arming. The first is the bug this replaces; the second
  # is the bug the obvious fix would have introduced.
  if ( PATH="$NO_TOOL_PATH"; command -v shellcheck >/dev/null 2>&1 ); then
    printf 'the constructed PATH still resolves shellcheck: %s\n' "$NO_TOOL_PATH" >&2
    return 1
  fi
  local tool
  for tool in sed grep awk git; do
    if ! ( PATH="$NO_TOOL_PATH"; command -v "$tool" >/dev/null 2>&1 ); then
      printf 'the constructed PATH lost `%s` -- the critic cannot run under it\n' "$tool" >&2
      return 1
    fi
  done
}

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-census-XXXXXX)"
  # LOCAL SUBJECTS, DELIBERATELY. These arms assert on the CENSUS -- what the
  # run ASKED -- which is a property of the rule library and not of any file.
  # Pointing them at `bin/intent` and at a crate source file gave them shared
  # dependencies they never needed: several nodes edit both, and a finding
  # appearing or disappearing in either would move a test about arming.
  SUBJECT="${TEST_TEMP_DIR}/subject.sh"
  cat > "$SUBJECT" <<'SUBJ'
#!/bin/bash
set -euo pipefail
d="$1"
printf '%s\n' "$d"
SUBJ
  # The rust subject sits under `src/` because the rust rules declare
  # `applies_to: src/**/*.rs`; outside it, applies_to excludes the file and the
  # rule returns clean for a reason that has nothing to do with the rule. That
  # cost me a failed positive control earlier today.
  mkdir -p "${TEST_TEMP_DIR}/src"
  RS_SUBJECT="${TEST_TEMP_DIR}/src/lib.rs"
  cat > "$RS_SUBJECT" <<'RSUBJ'
pub fn ok() -> u32 {
  1
}
RSUBJ
}
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# --- ARM 1: every rule resolves to exactly one arming state ----------------

# This arm called `critic_arming_census`, a function of the v2 shell runner,
# and went with it at the 3.0.1 cut. Its invariant -- no rule is undeclared --
# is now read off v3's own header, as one assertion in arm 2's first test.

# --- ARM 2: the counts appear in NORMAL output, every run ------------------

@test "output: a normal run states how many rules it ASKED, not just how many are armed" {
  run "$CRITIC" critic shell --files "$SUBJECT"
  [[ "$output" == *"rule(s) ASKED of this run"* ]]
  [[ "$output" == *"armed in total"* ]]
  # Arm 1's invariant, from v3's own census line: every shell rule has an
  # arming state and none is undeclared.
  [[ "$output" == *", 0 undeclared,"* ]]
}

@test "output: shell asks a non-zero number of rules" {
  asked="$("$CRITIC" critic shell --files "$SUBJECT" 2>&1 \
    | sed -n 's/^critic: shell -- \([0-9][0-9]*\) of .* ASKED.*/\1/p')"
  [ -n "$asked" ]
  [ "$asked" -gt 0 ]
}

# --- ARM 3: armed-but-not-run is REPORTED, never silently downgraded -------

@test "absent tool: the ASKED count drops to zero and says so" {
  asked="$(env PATH="$NO_TOOL_PATH" "$CRITIC" critic shell --files "$SUBJECT" 2>&1 \
    | sed -n 's/^critic: shell -- \([0-9][0-9]*\) of .* ASKED.*/\1/p')"
  [ "$asked" = "0" ]
}

@test "out of context: a whole-workspace analyser reports NOT RUN HERE in a per-file run" {
  run "$CRITIC" critic rust --files "$RS_SUBJECT"
  [[ "$output" == *"the tool does not belong in this context"* ]]
  [[ "$output" == *"IN-RS-CODE-001(clippy)"* ]]
}

# --- ARM 3b: (b) THE REFUSAL -- and it is only a test if it is two-sided ---

@test "absent tool: an armed rule whose tool is gone REFUSES with exit 3" {
  # AC-07.4(b), hv's ruling: a project that armed a rule and then lost the tool
  # is REFUSED, not silently passed. Driven under a PATH that genuinely lacks
  # shellcheck -- a code read cannot tell `refuses` from `would refuse`.
  #
  # 3 is the refusal code, not a generic error: v2 gates it on CRITIC_REFUSED
  # (`bin/intent_critic`), and the run still prints its census and an `ok:` line
  # above the refusal, so asserting on output alone would miss it entirely.
  run env PATH="$NO_TOOL_PATH" "$CRITIC" critic shell --files "$SUBJECT"
  [ "$status" -eq 3 ]
}

@test "present tool: the SAME run does not refuse -- the half that makes the arm above a test" {
  # NOT SYMMETRY FOR ITS OWN SAKE. A runner that refused UNCONDITIONALLY passes
  # the absent-tool arm above while blocking every commit on every machine, and
  # the one-sided arm cannot tell that runner from a correct one. This arm is
  # the only thing standing between those two readings.
  run "$CRITIC" critic shell --files "$SUBJECT"
  [ "$status" -ne 3 ]
}

# --- ARM 4: THE POSITIVE CONTROL. Without this, the rest is decoration. ----

@test "positive control: a fixture that violates IN-SH-CODE-001 produces a finding" {
  cat > "${TEST_TEMP_DIR}/bad.sh" <<'EOF'
#!/bin/bash
d=$1
cp $d /tmp/
arr=($d)
EOF
  run "$CRITIC" critic shell --files "${TEST_TEMP_DIR}/bad.sh"
  [[ "$output" == *"IN-SH-CODE-001"* ]]
  [ "$status" -ne 0 ]
}

@test "negative control: a correctly quoted fixture produces no finding" {
  cat > "${TEST_TEMP_DIR}/good.sh" <<'EOF'
#!/bin/bash
set -euo pipefail
d="$1"
cp "$d" /tmp/
EOF
  run "$CRITIC" critic shell --files "${TEST_TEMP_DIR}/good.sh"
  [[ "$output" != *"IN-SH-CODE-001"* ]]
  [ "$status" -eq 0 ]
}

@test "positive control: the parser does NOT fire on the construct inside a comment" {
  # This is why the rule is armed on a tool rather than on a regex: a grep for
  # an unquoted expansion cannot tell code from a comment, and IN-SH-CODE-002's
  # upgrade from a clean grep arm to shellcheck rests entirely on this.
  cat > "${TEST_TEMP_DIR}/commented.sh" <<'EOF'
#!/bin/bash
set -euo pipefail
# cp $d /tmp/ -- this line is a comment and is not a violation
printf 'ok\n'
EOF
  run "$CRITIC" critic shell --files "${TEST_TEMP_DIR}/commented.sh"
  [[ "$output" != *"IN-SH-CODE-001"* ]]
}

# --- ARM 5: THE DISCRIMINATION. The criterion stated as an observable. -----

@test "discrimination: an asking run and a non-asking run do not print the same sentence" {
  # AC-07.4 in one line. Before Half B both of these printed `ok: no shell
  # findings`, identically, and the gate could not tell CHECKED AND CLEAN from
  # CHECKED NOTHING.
  asked="$("$CRITIC" critic shell --files "$SUBJECT" 2>&1 | grep '^critic: shell -- .* ASKED')"
  none="$(env PATH="$NO_TOOL_PATH" "$CRITIC" critic shell --files "$SUBJECT" 2>&1 | grep '^critic: shell -- .* ASKED')"
  [ -n "$asked" ]
  [ -n "$none" ]
  [ "$asked" != "$none" ]
}

@test "discrimination: shell and elixir do not print the same sentence after asking different questions" {
  # The original symptom, kept as a regression: `critic shell` returning rc=0
  # having asked nothing was byte-identical to `critic elixir` after asking nine.
  sh="$("$CRITIC" critic shell --files "$SUBJECT" 2>&1 | grep '^critic: shell')"
  [ -n "$sh" ]
  [[ "$sh" == *"ASKED"* ]]
}
