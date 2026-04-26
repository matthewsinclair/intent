#!/usr/bin/env bats
# Tests for lib/templates/hooks/pre-commit.sh (ST0035/WP-06).
#
# Stands up a scratch Intent-flavoured git repo, installs the hook,
# exercises the three contract scenarios: bad fixture blocks (exit 1),
# good fixture passes (exit 0 with severity tuned), missing intent CLI
# fails open (exit 0 with advisory).

load "../lib/test_helper.bash"

HOOK="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh"
FIX_BAD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/bad_test.exs"
FIX_GOOD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/good_test.exs"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-hook-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Minimal Intent project skeleton
  mkdir -p intent/.config
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.10.0","project_name":"HookTest","author":"t","created_date":"2026-04-24T00:00:00Z"}
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
  cp "$FIX_BAD" test_bad.exs
  git add intent/.config mix.exs test_bad.exs
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
  cp "$FIX_GOOD" test_good.exs
  git add intent/.config mix.exs test_good.exs .intent_critic.yml
  run git commit -m "good"
  assert_success
}

@test "--no-verify bypasses the hook" {
  cp "$FIX_BAD" test_bad.exs
  git add intent/.config mix.exs test_bad.exs
  run git commit --no-verify -m "bypass"
  assert_success
}

@test "intent CLI missing → fail-open (exit 0, advisory on stderr)" {
  # Strip PATH to just /usr/bin:/bin so `intent` is not resolvable.
  # Use git -c so user config still works.
  cp "$FIX_BAD" test_bad.exs
  git add intent/.config mix.exs test_bad.exs
  PATH="/usr/bin:/bin" run git commit -m "no-intent"
  assert_success
  assert_output_contains "'intent' CLI not on PATH"
}

@test "non-Intent repo → fail-open (exit 0, advisory on stderr)" {
  # Remove intent/.config/ so hook's fail-open check fires.
  rm -rf intent/.config
  cp "$FIX_BAD" test_bad.exs
  git add mix.exs test_bad.exs
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
  cp "$FIX_GOOD" test_good.exs
  git add intent/.config mix.exs test_good.exs .intent_critic.yml
  # good fixture + critical threshold → clean
  run git commit -m "clean under critical"
  assert_success
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
  cp "$FIX_BAD" test_bad.exs
  git add intent/.config mix.exs test_bad.exs .intent_critic.yml
  run git commit -m "all-disabled"
  assert_success
}
