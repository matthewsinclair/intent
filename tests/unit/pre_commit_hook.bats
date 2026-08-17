#!/usr/bin/env bats
# Tests for lib/templates/hooks/pre-commit.sh (ST0035/WP-06).
#
# Stands up a scratch Intent-flavoured git repo, installs the hook,
# exercises the three contract scenarios: bad fixture blocks (exit 1),
# good fixture passes (exit 0 with severity tuned), missing intent CLI
# fails open (exit 0 with advisory).

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

@test "intent CLI missing → fail-open (exit 0, advisory on stderr)" {
  # Strip PATH to just /usr/bin:/bin so `intent` is not resolvable.
  # Use git -c so user config still works.
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add intent/.config mix.exs test/bad_test.exs
  PATH="/usr/bin:/bin" run git commit -m "no-intent"
  assert_success
  assert_output_contains "'intent' CLI not on PATH"
}

@test "non-Intent repo → fail-open (exit 0, advisory on stderr)" {
  # Remove intent/.config/ so hook's fail-open check fires.
  rm -rf intent/.config
  mkdir -p test && cp "$FIX_BAD" test/bad_test.exs
  git add mix.exs test/bad_test.exs
  run git commit -m "non-intent"
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
  assert_output_contains "NO whiteboard guard ran"
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
  assert_output_contains "NO whiteboard guard ran"
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
  refute_output_contains "NO whiteboard guard ran"
}

@test "resolver works, guard files absent: reports the one hole, NOT total failure" {
  # The other direction. A fix that shouted "ALL guards missing" whenever any
  # single guard was absent would pass the test above and be just as wrong.
  mkdir -p "${TEST_TEMP_DIR}/empty-home/lib/templates/hooks"
  shim_intent nohome
  stage_bad_board
  PATH="${TEST_TEMP_DIR}/shim:$PATH" run git commit -m "guard-absent"

  assert_success
  assert_output_contains "whiteboard-clock-guard.sh was not found"
  assert_output_contains "timestamps are UNCHECKED"
  refute_output_contains "NO whiteboard guard ran"
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
