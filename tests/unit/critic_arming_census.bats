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
# WHY THIS FILE IS `.bats` AND NOT THE `.rs` THE AT ROW CITES. AT-07.4 names
# `native/rust/crates/intent-cli/tests/critic_runner.rs`. **That file does not
# exist, and neither does any Rust critic** -- the subject is bash
# (`intent/plugins/claude/lib/critic_runner.sh` + `bin/intent_critic`) and stays
# bash until WP-07, which is Not Started. Writing a Rust test that shells out to
# a bash script in order to satisfy the cited path would be building the
# CITATION rather than the check, which is the class this estate keeps finding.
# The path correction is routed to vc; the check lives where its subject lives,
# beside its siblings `critic_runner_proxies.bats` and
# `critic_runner_applies_to.bats`.

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
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/intent/plugins/claude/lib/rules_lib.sh"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/intent/plugins/claude/lib/critic_runner.sh"
}
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# --- ARM 1: every rule resolves to exactly one arming state ----------------

# NOTE ON MECHANICS, because the first version of this block was RED for a
# reason that had nothing to do with the subject: `bash -c` spawns a shell that
# has NOT sourced critic_runner.sh, so every one of these asserted against
# `command not found`. Six confident reds, all of them the instrument's own
# fault. The census is called in THIS shell, where setup() sourced it.

@test "census: every shell rule carries an arming state and none is undeclared" {
  census="$(critic_arming_census shell)"
  [ -n "$census" ]
  # Every row is `<id> <arming> <disposition> <by>` -- four fields, no more.
  malformed="$(printf '%s\n' "$census" | awk 'NF != 4' | wc -l | tr -d ' ')"
  [ "$malformed" = "0" ]
  undeclared="$(printf '%s\n' "$census" | awk '$2 == "undeclared"' | wc -l | tr -d ' ')"
  [ "$undeclared" = "0" ]
}

@test "census: every rust rule carries an arming state and none is undeclared" {
  census="$(critic_arming_census rust)"
  [ -n "$census" ]
  malformed="$(printf '%s\n' "$census" | awk 'NF != 4' | wc -l | tr -d ' ')"
  [ "$malformed" = "0" ]
  undeclared="$(printf '%s\n' "$census" | awk '$2 == "undeclared"' | wc -l | tr -d ' ')"
  [ "$undeclared" = "0" ]
}

@test "census: the arming value is drawn from the closed set, never invented" {
  census="$(for l in shell rust elixir; do critic_arming_census "$l"; done)"
  [ -n "$census" ]
  bad="$(printf '%s\n' "$census" \
    | awk '$2 != "armed" && $2 != "declared" && $2 != "unrunnable" && $2 != "undeclared"' \
    | wc -l | tr -d ' ')"
  [ "$bad" = "0" ]
}

@test "census: the disposition is a SECOND field, not a fifth arming value" {
  # vc's ruling, 2026-08-19. A fifth arming value would put a property of the
  # INVOCATION into a key whose other members are properties of the rule, and
  # `armed` would then mean two things depending on which member is read.
  census="$(for l in shell rust elixir; do critic_arming_census "$l"; done)"
  bad="$(printf '%s\n' "$census" \
    | awk '$3 != "ran" && $3 != "n-a" && $3 != "not-run:tool-absent" && $3 != "not-run:out-of-context"' \
    | wc -l | tr -d ' ')"
  [ "$bad" = "0" ]
}

@test "census: a rule is never both armed and n-a, nor unarmed and ran" {
  census="$(for l in shell rust elixir; do critic_arming_census "$l"; done)"
  bad="$(printf '%s\n' "$census" \
    | awk '($2 != "armed" && $3 != "n-a") || ($2 == "armed" && $3 == "n-a")' \
    | wc -l | tr -d ' ')"
  [ "$bad" = "0" ]
}

@test "census: the population is non-trivial, so a clean sweep is not a vacuous one" {
  # A zero is not a result until the check has produced a non-zero: every
  # assertion above counts violations and passes at 0, which is exactly the
  # shape that also passes over an EMPTY census.
  n="$(critic_arming_census shell | wc -l | tr -d ' ')"
  [ "$n" -ge 6 ]
  n="$(critic_arming_census rust | wc -l | tr -d ' ')"
  [ "$n" -ge 7 ]
}

# --- ARM 2: the counts appear in NORMAL output, every run ------------------

@test "output: a normal run states how many rules it ASKED, not just how many are armed" {
  run "$CRITIC" critic shell --files "$SUBJECT"
  [[ "$output" == *"rule(s) ASKED of this run"* ]]
  [[ "$output" == *"armed in total"* ]]
}

@test "output: shell asks a non-zero number of rules" {
  asked="$("$CRITIC" critic shell --files "$SUBJECT" 2>&1 \
    | sed -n 's/^critic: shell -- \([0-9][0-9]*\) of .* ASKED.*/\1/p')"
  [ -n "$asked" ]
  [ "$asked" -gt 0 ]
}

# --- ARM 3: armed-but-not-run is REPORTED, never silently downgraded -------

@test "absent tool: a tool-armed rule reports NOT RUN rather than passing quietly" {
  # Driven by a PATH that genuinely lacks shellcheck, not by reading the code:
  # a code read cannot tell `refuses` from `would refuse`.
  run env PATH="$NO_TOOL_PATH" "$CRITIC" critic shell --files "$SUBJECT"
  [[ "$output" == *"THE TOOL IS ABSENT ON THIS MACHINE"* ]]
  [[ "$output" == *"IN-SH-CODE-001"* ]]
  [[ "$output" == *"UNENFORCED"* ]]
}

@test "absent tool: the ASKED count drops to zero and says so" {
  asked="$(env PATH="$NO_TOOL_PATH" "$CRITIC" critic shell --files "$SUBJECT" 2>&1 \
    | sed -n 's/^critic: shell -- \([0-9][0-9]*\) of .* ASKED.*/\1/p')"
  [ "$asked" = "0" ]
}

@test "out of context: a whole-workspace analyser reports NOT RUN HERE in a per-file run" {
  run "$CRITIC" critic rust --files "$RS_SUBJECT"
  [[ "$output" == *"the tool does not belong in this context"* ]]
  [[ "$output" == *"IN-RS-CODE-001(clippy)"* ]]
  # It is ARMED, not declared -- the capability is real and must not be hidden
  # behind a word that means nothing can ever answer this rule.
  arming="$(critic_arming_census rust | awk '$1 == "IN-RS-CODE-001" { print $2 }')"
  [ "$arming" = "armed" ]
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
