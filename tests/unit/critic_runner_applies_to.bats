#!/usr/bin/env bats
# Tests for ST0038: critic_apply_rule honours `applies_to` from rule
# frontmatter. Glob-to-regex with suffix anchor so umbrella layouts
# (apps/<app>/lib/..., apps/<app>/test/...) match rules declared as
# `lib/**/*.ex` or `test/**/*_test.exs`.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-applies-to-XXXXXX)"
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

# --- glob -> regex ---------------------------------------------------------

@test "glob_to_regex: lib/**/*.ex" {
  local r
  r="$(critic_glob_to_regex "lib/**/*.ex")"
  [ "$r" = "lib/(.*/)?[^/]*\\.ex" ]
}

@test "glob_to_regex: test/**/*_test.exs" {
  local r
  r="$(critic_glob_to_regex "test/**/*_test.exs")"
  [ "$r" = "test/(.*/)?[^/]*_test\\.exs" ]
}

@test "glob_to_regex: lib/**/live/**/*.ex" {
  local r
  r="$(critic_glob_to_regex "lib/**/live/**/*.ex")"
  [ "$r" = "lib/(.*/)?live/(.*/)?[^/]*\\.ex" ]
}

# --- applies_to honoring (TEST-002 has applies_to: ["test/**/*_test.exs"]) ---

@test "TEST-002 rule does NOT apply to apps/control/lib/foo.ex" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  ! critic_rule_applies_to_file "$rule" "apps/control/lib/foo.ex"
}

@test "TEST-002 rule DOES apply to apps/control/test/foo_test.exs (umbrella)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  critic_rule_applies_to_file "$rule" "apps/control/test/foo_test.exs"
}

@test "TEST-002 rule DOES apply to test/foo_test.exs (top-level)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  critic_rule_applies_to_file "$rule" "test/foo_test.exs"
}

@test "TEST-002 rule does NOT apply to lib/mix/tasks/control.boot.ex (no test/ in path)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  ! critic_rule_applies_to_file "$rule" "lib/mix/tasks/control.boot.ex"
}

# --- applies_to: ["lib/**/*.ex"] (CODE rules) ------------------------------

@test "no-silent-failures (lib code rule) DOES apply to apps/foo/lib/bar.ex" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/no-silent-failures/RULE.md"
  critic_rule_applies_to_file "$rule" "apps/foo/lib/bar.ex"
}

@test "no-silent-failures (lib code rule) does NOT apply to test/foo_test.exs" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/no-silent-failures/RULE.md"
  ! critic_rule_applies_to_file "$rule" "test/foo_test.exs"
}

# --- LiveView nested glob --------------------------------------------------

@test "thin-liveviews (lib/**/live/**/*.ex) applies to apps/web/lib/web/live/page_live.ex" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/lv/thin-liveviews/RULE.md"
  critic_rule_applies_to_file "$rule" "apps/web/lib/web/live/page_live.ex"
}

@test "thin-liveviews (lib/**/live/**/*.ex) does NOT apply to apps/web/lib/web/controllers/page_controller.ex" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/lv/thin-liveviews/RULE.md"
  ! critic_rule_applies_to_file "$rule" "apps/web/lib/web/controllers/page_controller.ex"
}

# --- Rule without applies_to (universal) -----------------------------------

@test "rule without applies_to applies to any file" {
  # Build a synthetic rule with no applies_to.
  cat > "$TEST_TEMP_DIR/RULE.md" <<EOF
---
id: SYNTH-NO-AT
language: elixir
category: code
severity: warning
title: Synthetic
status: active
version: 1
---
content
EOF
  critic_rule_applies_to_file "$TEST_TEMP_DIR/RULE.md" "anything/anywhere.ex"
  critic_rule_applies_to_file "$TEST_TEMP_DIR/RULE.md" "test/foo.exs"
  critic_rule_applies_to_file "$TEST_TEMP_DIR/RULE.md" "lib/foo.ex"
}

# --- ST0038 regression: tagged-tuple-returns has no greppable proxy --------

@test "IN-EX-CODE-002 has no greppable proxy (cannot be expressed as per-file regex)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/tagged-tuple-returns/RULE.md"
  local block
  block="$(critic_extract_greppable_block "$rule")"
  [ -z "$block" ]
}

@test "IN-EX-CODE-006 has no greppable proxy (cross-file concern)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/module-highlander/RULE.md"
  local block
  block="$(critic_extract_greppable_block "$rule")"
  [ -z "$block" ]
}

# --- ST0038 regression: TEST-002 still fires on test files -----------------

@test "IN-EX-TEST-002 still has its greppable proxy (single-file regex still valid)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  local block
  block="$(critic_extract_greppable_block "$rule")"
  [ -n "$block" ]
}
