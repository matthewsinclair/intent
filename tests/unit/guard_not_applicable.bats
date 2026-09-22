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

# ---------------------------------------------------------------------------
# `--list-guards` carries the fourth answer as a DECLARED state (issue 0515)
# ---------------------------------------------------------------------------

@test "--list-guards names a self-classifying guard as such, without running it" {
  # 0506 left this column unable to say it, because the arm returns before any
  # dispatch on purpose. The roster now DECLARES it, so nothing is executed.
  #
  # **THE REPO IS BUILT RATHER THAN INHERITED, and the first version of this arm
  # did not do that and was wrong.** `--list-guards` settles the roster's PATH
  # test before it reads the declaration, so in a directory without
  # `intent/.config/config.json` every row reads `not-applicable` and the
  # declaration is never reached. Run from the source tree it passed; run by
  # bats in a temp CWD it failed, and the arm was measuring the directory it
  # happened to be given. A test decides its own environment or it measures the
  # machine it runs on.
  scratch_repo ""
  run bash "$RUNNER" --list-guards
  [ "$status" -eq 0 ]
  local row
  row="$(printf '%s\n' "$output" | grep 'staged-format-guard.sh')"
  [ -n "$row" ]
  [ "$(printf '%s' "$row" | awk -F'\t' '{print $4}')" = "self-classifying" ]
}

@test "THE CONTROL: an applicable guard that does NOT self-declare reads present" {
  # The discriminator is the roster's fourth field and nothing else, so the
  # control must be a guard whose path test ALSO passes here -- otherwise it
  # reads `not-applicable` and agrees with the arm above for the wrong reason.
  # `append-only-guard.sh` applies when `intent` exists, which `scratch_repo`
  # creates. Without this, a runner printing `self-classifying` on every row
  # would satisfy the arm above.
  scratch_repo ""
  run bash "$RUNNER" --list-guards
  [ "$status" -eq 0 ]
  local row
  row="$(printf '%s\n' "$output" | grep 'append-only-guard.sh')"
  [ -n "$row" ]
  [ "$(printf '%s' "$row" | awk -F'\t' '{print $4}')" = "present" ]
}

@test "every row still carries exactly FIVE tab columns" {
  # **THE ARM THAT PROTECTS THE CONSUMERS**, and the reason 0515 added a state
  # VALUE rather than a sixth column. `bin/.devbin/cmd/hooks` reads five fields
  # with `read -r name path when state owner`, so a sixth would land inside
  # `owner` and silently flip every row from `declared` to `shipped`; and
  # `migrated_guards_still_refuse.rs` asserts `r.len() == 5` and would fail
  # loudly. One consumer breaks silently and one breaks loudly, which is the
  # worst possible pair, so the column count is pinned here too.
  run bash "$RUNNER" --list-guards
  [ "$status" -eq 0 ]
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    [ "$(printf '%s' "$line" | awk -F'\t' '{print NF}')" -eq 5 ]
  done <<< "$output"
}

@test "the roster row's shape has ONE home, and it reads both arities" {
  # The prose field was `${g_rest#*|}` -- the REST of the line -- in one loop
  # and unread in the other, so a fourth field would have landed inside the
  # prose silently. Both loops now go through these, and the arm drives them on
  # a three-field row and a four-field row rather than asserting the source.
  # shellcheck disable=SC1090
  source <(sed -n '/^roster_unchecked()/,/^}/p;/^roster_self()/,/^}/p' "$RUNNER")
  [ "$(roster_unchecked 'g.sh|timestamps are UNCHECKED')" = "timestamps are UNCHECKED" ]
  [ -z "$(roster_self 'g.sh|timestamps are UNCHECKED')" ]
  [ "$(roster_unchecked 'g.sh|staged bytes are UNCHECKED|self')" = "staged bytes are UNCHECKED" ]
  [ "$(roster_self 'g.sh|staged bytes are UNCHECKED|self')" = "self" ]
}

@test "int hooks renders self-classifying as its own state, not through the fault arm" {
  # vc's ruling on 0515: `cmd/hooks` is the project's, not devbin's (the
  # manifest header lists `cmd/` as the project's), so the state gets a real
  # arm. The `<-` arm is how MISSING and every unknown state render, and a
  # declared state reading like a fault is the collapse that arm exists to catch.
  # The runner is planted, so the arm judges the RENDER and nothing else.
  local home="$TEST_TEMP_DIR/home"
  mkdir -p "$home/lib/templates/hooks"
  cat > "$home/lib/templates/hooks/pre-commit-guards.sh" <<'RUNNER'
if [ "${1:-}" = "--list-guards" ]; then
  printf 'sfg.sh\t/x/sfg.sh\tcfg.json\tself-classifying\tintent\n'
  printf 'odd.sh\t/x/odd.sh\tcfg.json\tbogus-state\tintent\n'
fi
RUNNER
  # shellcheck disable=SC1090
  source <(sed -n '/^shipped_guards()/,/^}/p' "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/hooks")
  # `shipped_guards` declares its own `local home`, so the stub must not read
  # that name: bash scopes dynamically and would hand it the empty local.
  FAKE_HOME="$home"
  resolve_guard_home() { GUARD_HOME="$FAKE_HOME"; GUARD_HOME_FROM="test"; }
  run shipped_guards
  [ "$status" -eq 0 ]
  printf '%s\n' "$output" | grep -q 'shipped: sfg.sh  (cfg.json exists; the guard settles per commit whether it applies)'
  [[ "$output" != *"sfg.sh  <-"* ]]
  # THE CONTROL: an unknown state still reaches the fault arm, so the arm above
  # is not green because the fault arm stopped rendering anything.
  printf '%s\n' "$output" | grep -q 'shipped: odd.sh  <- bogus-state (/x/odd.sh)'
}
