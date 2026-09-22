#!/usr/bin/env bats
# NOT-APPLICABLE IS AN ANSWER A DISPATCHED GUARD CAN GIVE (issue 0506).
#
# The runner settles applicability two ways and only one of them was ever
# reportable. A guard whose subject is a FILE is settled before dispatch by the
# roster's path test and counted in SKIPPED. A guard whose subject is a
# DECLARATION cannot be settled that way -- every Intent project carries
# `intent/.config/config.json`, so a path test on it can only say yes -- and
# until 0506 the runner read exactly two answers from a guard it had
# dispatched: 0, counted in RAN, and non-zero, which blocked. The class existed
# and nothing dispatched could reach it, so `staged-format-guard.sh` printed
# prose instead, and prose reaches no summary, no `--list-guards` and no tally.
#
# WHY THIS FILE EXISTS RATHER THAN ARMS IN EITHER NEIGHBOUR. `guard_dispatch.bats`
# declares in its own header that it checks the structural links and does NOT
# execute the guards; `pre_commit_hook.bats` drives the whole chain through a
# real `git commit`. This is neither: it executes the RUNNER to assert how it
# CLASSIFIES what it dispatched. Putting it in either neighbour would have meant
# contradicting that file's stated scope in order to borrow its setup.
#
# WHAT THE ARMS ARE FOR, and it is the reason there are controls at all: the
# headline arms assert that something STOPS blocking and STOPS being counted.
# An arm asserting an absence passes for free against a runner that dispatched
# nothing, so each one is paired with the same guard answering a code that must
# still block. A green here means the classification moved for 3 and only for 3.

load "../lib/test_helper.bash"

RUNNER="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit-guards.sh"
GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/staged-format-guard.sh"

# A scratch repo that declares project guards answering the codes given.
# `$1` is the formatters declaration verbatim ('' for none); the rest are exit
# codes, one planted guard each, in order.
scratch_repo() {
  local repo="$TEST_TEMP_DIR/repo" decl="$1"
  shift
  mkdir -p "$repo/intent/.config" "$repo/gd"
  cd "$repo" || return 1
  git init -q -b main >/dev/null
  git config user.email "test@example.com"
  git config user.name "Test"

  local guards="" i=0 rc
  for rc in "$@"; do
    printf '#!/usr/bin/env bash\nexit %s\n' "$rc" > "gd/g${i}.sh"
    chmod +x "gd/g${i}.sh"
    guards="${guards}${guards:+,}{\"run\":[\"gd/g${i}.sh\"]}"
    i=$((i + 1))
  done

  {
    printf '{\n  "intent_version": "3.2.0"'
    [ -n "$decl" ] && printf ',\n  "formatters": [%s]' "$decl"
    [ -n "$guards" ] && printf ',\n  "guards": [%s]' "$guards"
    printf '\n}\n'
  } > intent/.config/config.json

  git add -A
  git commit -q -m "init"
}

# The project-guard loop is the only one whose population this file controls,
# and the runner reads it with jq. An absent jq makes every project arm below
# REFUSE rather than dispatch, which would look like a failure of the thing
# under test. Declared once, as a precondition, rather than skipped per arm:
# a skip is how an unarmed run reads identical to a passing one (issue 0512).
@test "precondition: jq is present, so the project-guard loop dispatches at all" {
  command -v jq >/dev/null 2>&1
}

@test "a project guard answering 3 is counted SKIPPED and does not block" {
  scratch_repo "" 3
  run bash "$RUNNER"
  [ "$status" -eq 0 ]
  [[ "$output" == *"project: 0 ran, 1 skipped"* ]]
}

@test "THE CONTROL: the same guard answering 1 is counted RAN and blocks" {
  # Without this, the arm above passes against a runner that dispatched nothing
  # at all -- a guard that never ran also fails to block and also adds nothing
  # to `ran`. The two arms differ in exactly one byte of the planted guard.
  scratch_repo "" 1
  run bash "$RUNNER"
  [ "$status" -ne 0 ]
  [[ "$output" == *"project: 1 ran, 0 skipped"* ]]
}

@test "2 is the shell's own error and still blocks -- the vocabulary did not widen" {
  # 3 was chosen because 1 and 2 were taken: 1 is BLOCKED and 2 is what bash
  # answers when it cannot run the file. If 2 had drifted into the new class,
  # a guard the runner could not execute would read as one that did not apply.
  scratch_repo "" 2
  run bash "$RUNNER"
  [ "$status" -ne 0 ]
  [[ "$output" == *"project: 1 ran, 0 skipped"* ]]
}

@test "0 is unchanged: a guard that ran and passed is still counted RAN" {
  scratch_repo "" 0
  run bash "$RUNNER"
  [ "$status" -eq 0 ]
  [[ "$output" == *"project: 1 ran, 0 skipped"* ]]
}

@test "the three codes are classified independently in one run" {
  # Each counter must take its own member rather than the run taking one
  # verdict. Driven together because a per-arm pass says nothing about a loop
  # that resets or aggregates wrongly across iterations.
  scratch_repo "" 0 3 1
  run bash "$RUNNER"
  [ "$status" -ne 0 ]
  [[ "$output" == *"project: 2 ran, 1 skipped"* ]]
}

# THE GUARD'S OWN CONTRACT IS NOT RESTATED HERE. That `staged-format-guard.sh`
# answers 3 when nothing is declared, prints nothing, and still answers 0 and
# speaks when a formatter IS declared, is asserted in
# `tests/unit/staged_format_guard.bats` arm 1 and its neighbours, beside the
# rest of that guard's behaviour. Two arms asserting it were drafted here and
# removed: the runner's classification and the guard's verdict are different
# subjects, and a second copy of the second one is the drift this estate's own
# rule forbids. What this file owns is what the RUNNER does with the answer.

@test "the canon loop moves the format guard from RAN to SKIPPED, by DELTA not by count" {
  # Absolute tallies depend on which canon guards a scratch tree makes
  # applicable, so the assertion is the delta between two runs of the same
  # repository differing only in the declaration. One guard crosses, in one
  # direction, and the total is conserved.
  scratch_repo ""
  run bash "$RUNNER"
  local undeclared="$output"
  local u_ran u_skip
  u_ran="$(sed -n 's/^guards: \([0-9]*\) ran.*/\1/p' <<<"$undeclared")"
  u_skip="$(sed -n 's/^guards: [0-9]* ran, \([0-9]*\) skipped.*/\1/p' <<<"$undeclared")"

  printf '{\n  "intent_version": "3.2.0",\n  "formatters": ["markdown"]\n}\n' > intent/.config/config.json
  run bash "$RUNNER"
  local d_ran d_skip
  d_ran="$(sed -n 's/^guards: \([0-9]*\) ran.*/\1/p' <<<"$output")"
  d_skip="$(sed -n 's/^guards: [0-9]* ran, \([0-9]*\) skipped.*/\1/p' <<<"$output")"

  # Both readings must have parsed, or the arithmetic below compares empties.
  [ -n "$u_ran" ] && [ -n "$u_skip" ] && [ -n "$d_ran" ] && [ -n "$d_skip" ]
  [ "$d_ran" -eq $((u_ran + 1)) ]
  [ "$d_skip" -eq $((u_skip - 1)) ]
  [ $((u_ran + u_skip)) -eq $((d_ran + d_skip)) ]
}
