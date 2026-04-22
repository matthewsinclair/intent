#!/usr/bin/env bats
# Tests that every runnable Elixir rule example exits 0 under standalone `elixir`.
#
# The schema contract (rule-schema.md § Exit code contract) says both `good*.exs`
# and `bad*.exs` MUST exit 0. Critics detect antipatterns by reading source;
# runtime failure would force awkward contrivances in the bad examples and
# would not match the way Critics actually work.
#
# This file enumerates every code and test rule that ships runnable examples
# and runs them. Ash/Phoenix/LiveView rules are inline-only (Mix-project deps)
# and are deliberately excluded from this test.
#
# Skipped when Elixir is not on PATH — CI environments without Elixir get a
# graceful skip rather than a failure. Local dev should always have Elixir.

load "../lib/test_helper.bash"

ELIXIR_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir"

setup_file() {
  if ! command -v elixir >/dev/null 2>&1; then
    export ELIXIR_MISSING=1
  fi
}

# Rules with runnable examples. Ash/Phoenix/LV rules are INTENTIONALLY ABSENT —
# they rely on Mix projects with framework dependencies and cannot run under
# standalone `elixir`.
runnable_code_rules() {
  cat <<'EOF'
pattern-match-over-conditionals
tagged-tuple-returns
impl-true-on-callbacks
with-for-railway
no-silent-failures
module-highlander
EOF
}

runnable_test_rules() {
  cat <<'EOF'
strong-assertions
no-process-sleep
async-by-default
start-supervised
no-control-flow-in-tests
real-code-over-mocks
test-highlander-shared-setup
EOF
}

# ====================================================================
# good.exs / good_test.exs exit 0
# ====================================================================

@test "every code-rule good.exs exits 0 under standalone elixir" {
  [ -n "${ELIXIR_MISSING:-}" ] && skip "elixir not on PATH"
  local slug path
  while read -r slug; do
    [ -z "$slug" ] && continue
    path="$ELIXIR_ROOT/code/$slug/good.exs"
    run elixir "$path"
    [ "$status" -eq 0 ] || {
      echo "good.exs failed: $path" >&2
      echo "output: $output" >&2
      return 1
    }
  done < <(runnable_code_rules)
}

@test "every test-rule good_test.exs exits 0 under standalone elixir" {
  [ -n "${ELIXIR_MISSING:-}" ] && skip "elixir not on PATH"
  local slug path
  while read -r slug; do
    [ -z "$slug" ] && continue
    path="$ELIXIR_ROOT/test/$slug/good_test.exs"
    run elixir "$path"
    [ "$status" -eq 0 ] || {
      echo "good_test.exs failed: $path" >&2
      echo "output: $output" >&2
      return 1
    }
  done < <(runnable_test_rules)
}

# ====================================================================
# bad.exs / bad_test.exs ALSO exit 0 — antipatterns are detected statically,
# not at runtime. This is a deliberate upstream convention.
# ====================================================================

@test "every code-rule bad.exs exits 0 (antipattern still runs cleanly)" {
  [ -n "${ELIXIR_MISSING:-}" ] && skip "elixir not on PATH"
  local slug path
  while read -r slug; do
    [ -z "$slug" ] && continue
    path="$ELIXIR_ROOT/code/$slug/bad.exs"
    run elixir "$path"
    [ "$status" -eq 0 ] || {
      echo "bad.exs failed (must exit 0 per schema contract): $path" >&2
      echo "output: $output" >&2
      return 1
    }
  done < <(runnable_code_rules)
}

@test "every test-rule bad_test.exs exits 0 (antipattern still runs cleanly)" {
  [ -n "${ELIXIR_MISSING:-}" ] && skip "elixir not on PATH"
  local slug path
  while read -r slug; do
    [ -z "$slug" ] && continue
    path="$ELIXIR_ROOT/test/$slug/bad_test.exs"
    run elixir "$path"
    [ "$status" -eq 0 ] || {
      echo "bad_test.exs failed (must exit 0 per schema contract): $path" >&2
      echo "output: $output" >&2
      return 1
    }
  done < <(runnable_test_rules)
}

# ====================================================================
# First-non-empty-line invariant — the validator checks this, but pin it
# directly here too so a regression is obvious.
# ====================================================================

first_nonempty_line() {
  local path="$1"
  awk 'NF { print; exit }' "$path"
}

@test "every runnable example starts with '# EXPECTED: passes'" {
  local slug path expected
  expected="# EXPECTED: passes"

  while read -r slug; do
    [ -z "$slug" ] && continue
    for variant in good.exs bad.exs; do
      path="$ELIXIR_ROOT/code/$slug/$variant"
      local actual
      actual=$(first_nonempty_line "$path")
      [ "$actual" = "$expected" ] || {
        echo "first-line mismatch at $path" >&2
        echo "  expected: $expected" >&2
        echo "  actual:   $actual" >&2
        return 1
      }
    done
  done < <(runnable_code_rules)

  while read -r slug; do
    [ -z "$slug" ] && continue
    for variant in good_test.exs bad_test.exs; do
      path="$ELIXIR_ROOT/test/$slug/$variant"
      local actual
      actual=$(first_nonempty_line "$path")
      [ "$actual" = "$expected" ] || {
        echo "first-line mismatch at $path" >&2
        echo "  expected: $expected" >&2
        echo "  actual:   $actual" >&2
        return 1
      }
    done
  done < <(runnable_test_rules)
}
