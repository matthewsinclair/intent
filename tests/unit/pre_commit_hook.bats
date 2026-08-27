#!/usr/bin/env bats
# Tests for lib/templates/hooks/pre-commit.sh (ST0035/WP-06).
#
# Stands up a scratch Intent-flavoured git repo, installs the hook,
# exercises the contract scenarios: bad fixture blocks (exit 1), good
# fixture passes (exit 0 with severity tuned), a non-Intent repo skips, and
# a missing intent CLI **in an Intent project REFUSES** (exit 1).
#
# THE CLI ARM WAS FAIL-OPEN AND IS NOW FAIL-CLOSED (hv, 2026-08-27). Its
# stated justification -- do not block work in a non-Intent repo -- is the
# job of the config.json test, which is the precise one and now runs FIRST.
# One test was standing in for the other, and the substitute could not tell
# the two populations apart.

load "../lib/test_helper.bash"

# INTENT_HOOK_TEMPLATE redirects every test in this file at another copy of the
# hook, so these can be mutation-tested against a deliberately broken one without
# editing what ships. Unset -- every normal run -- it is the plain path.
HOOK="${INTENT_HOOK_TEMPLATE:-${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh}"
FIX_BAD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/bad_test.exs"
FIX_GOOD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/good_test.exs"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-hook-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Minimal Intent project skeleton.
  # `languages` field (v2.11.0+, ST0037) is the explicit declaration that
  # tells the hook which critics to invoke. Tests that target a different
  # language set should override this in their own setup.
  mkdir -p intent/.config
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":["elixir"]}
EOF
  touch mix.exs

  git init -q .
  git config user.email t@t.com
  git config user.name Tester

  cp "$HOOK" .git/hooks/pre-commit
  chmod +x .git/hooks/pre-commit
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "hook template exists and is executable" {
  [ -x "$HOOK" ]
}

@test "hook template syntax is valid" {
  run bash -n "$HOOK"
  assert_success
}

@test "staged bad fixture blocks the commit (exit 1)" {
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  run git commit -m "bad"
  [ "$status" -ne 0 ]
  assert_output_contains "commit blocked by findings"
  assert_output_contains "IN-EX-TEST-001"
}

@test "staged good fixture at critical threshold passes (exit 0)" {
  cat > .intent_critic.yml <<'EOF'
severity_min: critical
disabled: []
EOF
  mkdir -p test && cp "$FIX_GOOD" test/good_test.exs
  git add intent/.config mix.exs test/good_test.exs .intent_critic.yml
  run git commit -m "good"
  assert_success
}

@test "--no-verify bypasses the hook" {
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  run git commit --no-verify -m "bypass"
  assert_success
}

@test "single-step case in lib/*.ex does NOT block commit (ST0039 / Conflab regression)" {
  # Conflab field report 2026-04-29: pre-commit gate flagged every
  # `case ... do` line via IN-EX-CODE-004's counter proxy. ST0039 strips
  # the counter; only the `error -> error` forwarder line remains, which
  # this fixture does not contain.
  cat > .intent_critic.yml <<'EOF'
severity_min: warning
disabled: []
EOF
  mkdir -p lib
  cat > lib/finder.ex <<'EOF'
defmodule Finder do
  def find_user(id) do
    case Repo.get(User, id) do
      nil -> {:error, :not_found}
      user -> {:ok, user}
    end
  end
end
EOF
  git add intent/.config mix.exs lib/finder.ex .intent_critic.yml
  run git commit -m "single-step case"
  assert_success
}

@test "compliant async test does NOT block commit (ST0039 / Conflab regression)" {
  # Conflab field report 2026-04-29: pre-commit gate flagged the compliant
  # `use ExUnit.Case, async: true` line via IN-EX-TEST-003's inverted
  # proxy. ST0039 strips the proxy entirely.
  cat > .intent_critic.yml <<'EOF'
severity_min: warning
disabled: []
EOF
  mkdir -p test
  cat > test/foo_test.exs <<'EOF'
defmodule FooTest do
  use ExUnit.Case, async: true

  test "trivial" do
    assert 1 + 1 == 2
  end
end
EOF
  git add intent/.config mix.exs test/foo_test.exs .intent_critic.yml
  run git commit -m "compliant async test"
  assert_success
}

@test "intent CLI missing in an Intent project → REFUSES (exit 1)" {
  # Strip PATH to just /usr/bin:/bin so `intent` is not resolvable.
  #
  # **A GATE THAT CANNOT RUN, IN A PROJECT THAT DECLARED IT, IS A FAILURE AND
  # NEVER A SKIP.** This arm was `assert_success` until hv ruled it 2026-08-27:
  # all 17 estates carrying the hook are Intent projects, so the fail-open
  # protected nobody and cost 12 ungated commits across 3 estates in one
  # 9-minute window. A skip is indistinguishable from a pass downstream.
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  PATH="/usr/bin:/bin" run git commit -m "no-intent"
  assert_failure
  assert_output_contains "'intent' CLI is not runnable, and this IS an Intent project"
  # STATE A: nothing on PATH by that name. The ONLY state where "install" is right.
  assert_output_contains "no PATH entry named 'intent' exists at all"
  assert_output_contains "install Intent"
}

@test "non-Intent repo → skips (exit 0, advisory on stderr)" {
  # Remove intent/.config/ so the config test fires. THIS ARM IS UNCHANGED BY
  # the ruling above and must stay green: the gate still does not apply where
  # the project does not declare it.
  rm -rf intent/.config
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add mix.exs test/bad_test.exs
  run git commit -m "non-intent"
  assert_success
  assert_output_contains "not inside an Intent project"
}

# ---- THE FOUR ABSENCES `command -v` CANNOT TELL APART ----
#
# **THE REMEDY WAS WRONG FOR MOST OPERATORS WHO WOULD EVER SEE THIS ARM.** ic
# measured it live on 2026-08-27: during a release build the CLI goes ABSENT
# rather than merely changing, so every node hit this arm at once and the message
# told them to INSTALL INTENT -- which races a build already in flight. On this
# estate `~/.local/bin/intent` is a symlink into the release tree, so the window
# is real, estate-wide, and lasts as long as a build.
#
# Each arm below PLANTS one state and asserts the remedy that state actually
# needs. A single arm proving "it refuses" would pass for all five while four of
# them printed the wrong advice.

@test "CLI absence B: a dangling symlink says WAIT, not install" {
  mkdir -p "${TEST_TEMP_DIR}/shim"
  ln -s "${TEST_TEMP_DIR}/gone-with-the-build" "${TEST_TEMP_DIR}/shim/intent"
  echo "x" > f.txt
  git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:/usr/bin:/bin" run git commit -m "dangling"
  assert_failure
  assert_output_contains "is a link whose target does not resolve"
  assert_output_contains "DO NOT reinstall"
  assert_output_contains "wait for the build to finish"
  # The point of the arm: it must NOT give state A's advice.
  refute_output_contains "no PATH entry named"
}

@test "CLI absence C/E do NOT reach the CLI arm -- command -v does not test executability" {
  # **THE ARM THAT CORRECTED THE PREMISE, AND IT WAS WRITTEN AS TWO PASSING
  # TESTS OF A REMEDY THAT COULD NEVER PRINT.** ic and cc both recorded that
  # `command -v` answers empty for FOUR states. Measured under bash on five
  # planted states, it is THREE: `command -v` does NOT test executability, so a
  # plain non-executable file (C) and a link to a non-executable target (E) are
  # both FOUND, sail past the CLI arm, and fail at the invocation site as exit
  # 126 -- where the gate reports the language unenforced and does NOT block.
  #
  # This arm pins that, rather than the comment carrying it alone: the two
  # branches written for C and E were unreachable, which is dead code reading as
  # coverage in the very file that exists to stop exactly that.
  #
  # **IT ALSO PINS A REAL GAP.** Under hv's ruling 4 -- a gate that cannot locate
  # what it needs REFUSES, it does not skip -- a 126 SHOULD block. It does not
  # today. When someone fixes that, this arm reds and tells them the CLI-arm
  # premise is the thing to re-check, which is the message the next person needs.
  mkdir -p "${TEST_TEMP_DIR}/shim"
  printf '#!/bin/sh\n' > "${TEST_TEMP_DIR}/shim/intent"
  chmod 644 "${TEST_TEMP_DIR}/shim/intent"
  echo "x" > f.txt
  git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:/usr/bin:/bin" run git commit -m "notexec"

  # It does NOT reach the CLI arm: none of that arm's states are printed.
  refute_output_contains "'intent' CLI is not runnable"
  refute_output_contains "no PATH entry named"
  refute_output_contains "is a link whose target does not resolve"

  # It lands on the 126 path instead, which today does not block. Asserting the
  # CURRENT behaviour, flagged above as the thing ruling 4 says to change.
  assert_output_contains "exit 126"
  assert_output_contains "UNENFORCED"
}

@test "CLI absence D: a DIRECTORY named intent is named, not swept into the residue" {
  # **THIS ARM EXISTS BECAUSE DRIVING THE TABLE FOUND THE STATE.** A directory is
  # searchable, so `-x` passes on it, and it landed in the catch-all where the
  # gate said it could not name the problem -- while the problem was one `-d`
  # away. The table committed the very error its own comment warns about, and
  # only running it showed that.
  mkdir -p "${TEST_TEMP_DIR}/shim/intent"
  echo "x" > f.txt
  git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:/usr/bin:/bin" run git commit -m "isadir"
  assert_failure
  assert_output_contains "is a DIRECTORY, not a program"
  assert_output_contains "reorder PATH"
  refute_output_contains "cannot name from the filesystem"
}

@test "non-Intent repo with NO intent CLI still skips (the reorder is safe)" {
  # **THE ARM THAT CARRIES THE WHOLE ARGUMENT FOR THE SWAP.** The CLI check used
  # to run FIRST and skip, justified as protecting exactly this case: a hook
  # copied by hand into a repo that is not an Intent project, on a machine with
  # no Intent installed. Both conditions at once, which no other arm covers.
  #
  # If the config test did not fully cover that population, this is where the
  # reorder would show up -- as a commit blocked in a repo the gate has no
  # business in. It exits 0, so the fail-open was protecting nobody the precise
  # test was not already protecting.
  rm -rf intent/.config
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add mix.exs test/bad_test.exs
  PATH="/usr/bin:/bin" run git commit -m "neither"
  assert_success
  assert_output_contains "not inside an Intent project"
}

@test "reads severity_min from .intent_critic.yml" {
  # With severity_min=critical, warnings alone should not block.
  cat > .intent_critic.yml <<'EOF'
severity_min: critical
disabled: []
EOF
  mkdir -p test && cp "$FIX_GOOD" test/good_test.exs
  git add intent/.config mix.exs test/good_test.exs .intent_critic.yml
  # good fixture + critical threshold → clean
  run git commit -m "clean under critical"
  assert_success
}

@test "empty languages array → hook runs no critics → commit proceeds" {
  # ST0037: a project with `languages: []` declares no language critics.
  # The hook walks the (empty) array and the AGGREGATE stays 0. A bad
  # fixture that would otherwise trigger critic-elixir is staged but the
  # commit proceeds because the elixir critic is not invoked.
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":[]}
EOF
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  run git commit -m "empty langs"
  assert_success
}

@test "languages without elixir → bad elixir fixture not flagged" {
  # ST0037: a shell-only declaration must not invoke critic-elixir even if
  # an Elixir test file is staged. Demonstrates the explicit-config
  # contract: file presence is not detection.
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":["shell"]}
EOF
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  run git commit -m "shell only"
  assert_success
}

@test "declared prose language (author/content) does not error the gate or fail-open (issue 0003)" {
  # A project that declares author/content must commit cleanly: the gate skips
  # languages with no headless code critic instead of invoking them and printing
  # a per-language "invocation error ... fail-open" line on every commit.
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":["author","content"]}
EOF
  echo "# a doc" > README.md
  git add intent/.config mix.exs README.md
  run git commit -m "prose langs"
  assert_success
  refute_output_contains "invocation error"
  refute_output_contains "fail-open"
  refute_output_contains "must be a language"
}

@test "mixed code+prose: elixir critic still fires, prose skipped cleanly (issue 0003)" {
  # elixir must still block on a bad fixture while author is skipped without the
  # drift noise -- the gate runs code critics and skips prose disciplines.
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":["elixir","author"]}
EOF
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  run git commit -m "elixir + author"
  [ "$status" -ne 0 ]
  assert_output_contains "commit blocked by findings"
  refute_output_contains "invocation error"
  refute_output_contains "fail-open"
}

@test "honours disabled rule id" {
  cat > .intent_critic.yml <<'EOF'
severity_min: warning
disabled:
  - IN-EX-TEST-001
  - IN-EX-TEST-003
  - IN-EX-CODE-006
EOF
  # With the three firing rules all disabled, even the bad fixture should
  # produce no findings at or above warning → commit proceeds.
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs .intent_critic.yml
  run git commit -m "all-disabled"
  assert_success
}

# --------------------------------------------------------------------
# Whiteboard guard resolution (issue 0042)
# --------------------------------------------------------------------
#
# The hook resolves guard BODIES at runtime out of `intent info`'s INTENT_HOME,
# because only this file is copied into a consumer project and a consumer has no
# lib/templates/ of its own. That is the right design; what was wrong was one
# `else` branch handling two different absences.
#
# When resolution FAILS, every guard is missing at once -- so the loop printed one
# mild "not found" per guard and enforced nothing, which reads as two small holes
# rather than "the gate did not run". It fails open, so the commit proceeds and
# nothing else ever reports it.
#
# "Fails" is deliberately not "comes back empty", and the gap between those two
# is a regression that shipped. The first fix tested emptiness, which WAS the true
# signature while `intent info` was unimplemented and printed no INTENT_HOME line
# at all. `info` now prints `INTENT_HOME: <not set>` -- non-empty -- so the branch
# went unreachable in the one condition it exists for, and a brew-shaped install
# was back to two mild warnings with nothing enforcing anything. The hook now gates
# on whether the resolution is a DIRECTORY, which covers both live causes: a binary
# running outside its own install tree, and a v3 binary shadowing a v2 install
# (issues 0036/0043).
#
# Fail-open is DELIBERATE and these tests pin it. A gate that blocks every commit
# the moment `intent` is shadowed is 0043 rebuilt on the git side.

# Put an `intent` on PATH whose `info` we control. Everything else defers to the
# real CLI so the rest of the hook behaves normally.
shim_intent() {  # shim_intent v3 | notset | livehome | nohome | real
  mkdir -p "${TEST_TEMP_DIR}/shim"
  {
    echo '#!/bin/sh'
    echo 'if [ "$1" = "info" ]; then'
    case "$1" in
      v3)     echo '  echo "error: info is a known command that is not implemented yet" >&2' ; echo '  exit 2' ;;
      # What a published build actually does, copied from a measured run of the
      # v3 binary staged outside its own tree: the line is still printed (cc kept
      # it deliberately so this gate had something to parse) carrying v2's
      # `<not set>` token, and the failure is in the exit code.
      notset) echo "  echo '  INTENT_HOME:     <not set>'" ; echo '  exit 1' ;;
      # Resolution is GOOD and the command still reports failure -- the shape a
      # migration refusal on `info` would produce in every unmigrated project.
      # The guards must still run; see the departure note in the hook.
      livehome) echo "  echo '  INTENT_HOME: ${INTENT_PROJECT_ROOT}'" ; echo '  exit 1' ;;
      nohome) echo "  echo '  INTENT_HOME: ${TEST_TEMP_DIR}/empty-home'" ; echo '  exit 0' ;;
      real)   echo "  echo '  INTENT_HOME: ${INTENT_PROJECT_ROOT}'" ; echo '  exit 0' ;;
    esac
    echo 'fi'
    echo "exec '${INTENT_BIN}' \"\$@\""
  } > "${TEST_TEMP_DIR}/shim/intent"
  chmod +x "${TEST_TEMP_DIR}/shim/intent"
}

# A board carrying a stamp with no trailing Z -- clock guard check B, which is
# syntactic and needs no clock, so it cannot be flaky.
stage_bad_board() {
  mkdir -p intent/whiteboard/dc
  printf -- '---\nnode: dc\nheartbeat_at: 2026-08-16 19:50\n---\n' > intent/whiteboard/dc/wip.md
  git add intent/.config intent/whiteboard
}

@test "resolver absent: reports TOTAL non-enforcement once, not one hole per guard" {
  shim_intent v3
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "resolver-absent"

  # Fail-open is the contract, and it is why the message has to carry the weight.
  assert_success
  assert_output_contains "NO guard ran for this commit"
  assert_output_contains "not one is missing, ALL are"
  assert_output_contains "exit 2"

  # The two absences must not be confusable. The per-guard wording appearing here
  # is the defect itself: it is what made total failure read as a couple of holes.
  refute_output_contains "was not found"
}

@test "resolver answers with a NON-PLACE: still TOTAL non-enforcement, not two small holes" {
  # THE REGRESSION THE FIRST FIX SHIPPED WITH, and the case a `brew install` of
  # v3 produces. `info` resolves nothing, prints `INTENT_HOME: <not set>`, and
  # exits 1 -- measured against the real binary staged outside its own tree, not
  # imagined. The parse therefore SUCCEEDS and yields a non-empty non-path, the
  # loop hunts for guards under `<not set>/lib/templates/hooks/`, and every guard
  # is missing for one reason while the output claims two independent holes.
  #
  # That is 0042's exact symptom wearing 0042's fix, and nothing connected the two
  # changes: the coupling is a `sed` over another command's display text.
  shim_intent notset
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "resolver-nonplace"

  assert_success
  assert_output_contains "NO guard ran for this commit"
  assert_output_contains "not one is missing, ALL are"
  # The operator has to SEE the non-place. Described rather than quoted, this
  # reads as a legitimate answer; quoted, it is self-evidently not a path.
  assert_output_contains "<not set>"
  refute_output_contains "was not found"
}

@test "resolver reports failure but resolves a REAL directory: the guards still run" {
  # THE DEPARTURE, PINNED. vc and I agreed the fix should branch on the exit code
  # as well; it reports on the code and gates on `-d` instead, and this is the
  # test that makes the difference observable rather than a comment.
  #
  # Gating on rc would make every guard conditional on an exit code whose meanings
  # are still being settled -- vc's 0045 measured that `Facade::open` gates EVERY
  # command and the migration refusal returns 1. The day `info` inherits that, rc
  # is non-zero in every unmigrated project (ie every consumer, the moment before
  # it upgrades) while INTENT_HOME resolves perfectly. Gating there would silently
  # stop the guards estate-wide: the exact class this branch exists to prevent,
  # delivered by the fix for it.
  #
  # So a bad stamp must STILL be refused here, and the failing resolver must still
  # be said out loud. Re-adding rc to the gate turns this test red with the
  # user-visible symptom, which is the one worth reading.
  shim_intent livehome
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "failing-info-live-home"

  [ "$status" -ne 0 ]
  assert_output_contains "whiteboard timestamp cannot be a real clock read"
  assert_output_contains "exited 1"
  refute_output_contains "NO guard ran for this commit"
}

@test "resolver resolves but the install has NO RUNNER: total failure, and NOT the resolver's fault" {
  # THE THIRD ABSENCE, and it arrived with the delegated roster. `INTENT_HOME`
  # resolves to a real directory and the guard runner is not in it -- an install
  # older than the mechanism. Total, like an absent resolver, but the remedy
  # shares nothing with it, so the message must not send the operator at
  # `intent info`, which is the one component working correctly here.
  mkdir -p "${TEST_TEMP_DIR}/empty-home/lib/templates/hooks"
  shim_intent nohome
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "runner-absent"

  assert_success
  assert_output_contains "this install has no guard runner"
  assert_output_contains "the resolver is not the problem"
  # The per-guard wording appearing here would be the collapse this whole
  # taxonomy exists to prevent, one level up from issue 0042.
  refute_output_contains "was not found;"
}

@test "resolver works, guard files absent: reports the one hole, NOT total failure" {
  # The other direction. A fix that shouted "ALL guards missing" whenever any
  # single guard was absent would pass the test above and be just as wrong.
  #
  # THE RUNNER IS INSTALLED AND THE GUARDS ARE NOT, which is what makes this
  # distinct from the test above rather than a second copy of it. Before the
  # roster was delegated an empty `lib/templates/hooks` WAS this case; it is now
  # the more-broken one, so the fixture has to say which it means.
  mkdir -p "${TEST_TEMP_DIR}/empty-home/lib/templates/hooks"
  cp "${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit-guards.sh" \
    "${TEST_TEMP_DIR}/empty-home/lib/templates/hooks/"
  shim_intent nohome
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "guard-absent"

  assert_success
  assert_output_contains "whiteboard-clock-guard.sh was not found"
  assert_output_contains "timestamps are UNCHECKED"
  refute_output_contains "NO guard ran for this commit"
}

@test "resolver works and guards are present: a bad stamp BLOCKS the commit" {
  # THE CANARY. Without this, both tests above pass on a hook that never runs a
  # guard at all -- and silence from a branch that did not execute reads exactly
  # like silence from one that ran and passed. This is what makes the other two
  # mean anything: the same fixture that is waved through above is refused here.
  shim_intent real
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "bad-stamp"

  [ "$status" -ne 0 ]
  assert_output_contains "whiteboard timestamp cannot be a real clock read"
}

# ---------------------------------------------------------------------------
# An unrecognised critic exit: the gate must not diagnose a cause it never
# measured, and must say what happened to the COMMIT
# ---------------------------------------------------------------------------

# A shim whose `intent critic <lang>` exits with a code chosen per language.
# `info` is answered from the real tree so the guard block is not the subject
# of these tests.
shim_critic() {  # shim_critic <lang>=<rc> ...
  mkdir -p "${TEST_TEMP_DIR}/shim"
  {
    echo '#!/bin/sh'
    echo 'if [ "$1" = "info" ]; then'
    echo "  echo '  INTENT_HOME: ${INTENT_PROJECT_ROOT}'"
    echo '  exit 0'
    echo 'fi'
    echo 'if [ "$1" = "critic" ]; then'
    echo '  case "$2" in'
    for pair in "$@"; do
      echo "    ${pair%%=*}) exit ${pair##*=} ;;"
    done
    echo '    *) exit 0 ;;'
    echo '  esac'
    echo 'fi'
    echo "exec '${INTENT_BIN}' \"\$@\""
  } > "${TEST_TEMP_DIR}/shim/intent"
  chmod +x "${TEST_TEMP_DIR}/shim/intent"
}

declare_languages() {  # declare_languages elixir rust ...
  local langs=""
  for l in "$@"; do langs="${langs}${langs:+,}\"${l}\""; done
  cat > intent/.config/config.json <<EOF
{"intent_version":"2.11.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z","languages":[${langs}]}
EOF
  git add intent/.config
}

@test "an unrecognised critic exit states the CONSEQUENCE and does not diagnose a CAUSE" {
  # The arm read `invocation error (exit $rc); fail-open` -- a diagnosis the gate
  # never made. It knows the code was unrecognised and nothing else. Under a v3
  # binary it printed that over a checker that ran perfectly and simply is not
  # built yet.
  declare_languages elixir
  shim_critic elixir=2
  echo x > f.txt && git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "unrecognised-code"

  # The fail-open is a RULING and is unchanged: only the claim changes.
  assert_success
  assert_output_contains "did not check (exit 2)"
  assert_output_contains "elixir is UNENFORCED in this commit"

  # THE DEFECT ITSELF. The gate cannot tell a broken tool from an unimplemented
  # command, and must not name either.
  refute_output_contains "invocation error"
}

@test "the unenforced digest carries a denominator, so 1 of N cannot read as N of N" {
  # A report that never changes trains its reader to stop looking. One of five
  # is a bad day; five of five is a gate that is not running at all, and those
  # must not look alike -- which is what the denominator is for.
  declare_languages elixir rust shell
  shim_critic elixir=2 rust=2 shell=0
  echo x > f.txt && git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "digest-denominator"

  assert_success
  assert_output_contains "2 of 3 declared language(s) went UNENFORCED"
  assert_output_contains "elixir rust"
  # A fail-open that does not say it failed open is the thing being fixed.
  assert_output_contains "the commit is NOT blocked by this"
}

@test "total non-enforcement is visibly different from partial" {
  declare_languages elixir rust shell
  shim_critic elixir=2 rust=2 shell=2
  echo x > f.txt && git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "total-non-enforcement"

  assert_success
  assert_output_contains "3 of 3 declared language(s) went UNENFORCED"
}

@test "the blocking arms are unaffected: findings block, and so does a REFUSAL" {
  # The whole point of touching this case is that the fail-open arm was the only
  # one that needed to change. If either blocking arm moved, the fix went too far.
  declare_languages elixir
  shim_critic elixir=1
  echo x > f.txt && git add f.txt
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "findings-block"
  assert_failure
  assert_output_contains "commit blocked by findings"

  shim_critic elixir=3
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "refusal-blocks"
  assert_failure
  assert_output_contains "REFUSED"
  assert_output_contains "commit blocked by findings"
}
