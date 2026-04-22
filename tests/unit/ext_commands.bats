#!/usr/bin/env bats
# Tests for intent ext command surface (list, show, validate, new)
# Uses tests/fixtures/extensions/ as the INTENT_EXT_DIR source.

load "../lib/test_helper.bash"

EXT_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures/extensions"

# ====================================================================
# list
# ====================================================================

@test "ext list reports empty-set status when INTENT_EXT_DIR does not exist" {
  export INTENT_EXT_DIR="/tmp/intent-no-such-dir-xyz123"
  run run_intent ext list
  assert_success
  assert_output_contains "no extensions installed"
}

@test "ext list reports disabled status when INTENT_EXT_DISABLE=1" {
  export INTENT_EXT_DISABLE=1
  run run_intent ext list
  assert_success
  assert_output_contains "ext discovery disabled"
}

@test "ext list enumerates all fixture extensions" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext list
  assert_success
  assert_output_contains "valid-ext"
  assert_output_contains "malformed-ext"
  assert_output_contains "shadow-ext"
  assert_output_contains "traversal-ext"
}

@test "ext list flags malformed extensions without crashing" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext list
  assert_success
  assert_output_contains "malformed-ext"
  assert_output_contains "malformed"
}

@test "ext list shows version for valid extensions" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext list
  assert_success
  assert_output_contains "valid-ext"
  assert_output_contains "1.0.0"
}

# ====================================================================
# show
# ====================================================================

@test "ext show requires a name" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show
  assert_failure
  assert_output_contains "extension name required"
}

@test "ext show prints manifest fields for a valid extension" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show valid-ext
  assert_success
  assert_output_contains "Extension: valid-ext"
  assert_output_contains "Status:       valid"
  assert_output_contains "Version:      1.0.0"
  assert_output_contains "subagents: 1"
  assert_output_contains "skills:    1"
  assert_output_contains "rules:     1"
}

@test "ext show lists each declared contribution" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show valid-ext
  assert_success
  assert_output_contains "ext-sample-agent"
  assert_output_contains "in-ext-sample"
  assert_output_contains "IN-AG-EXT-001"
}

@test "ext show emits shadow warning for canon collisions" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show shadow-ext
  assert_success
  [[ "$output" == *"shadows canon"* ]]
}

@test "ext show fails with clear error on missing extension" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show no-such-ext
  assert_failure
  assert_output_contains "not found"
}

@test "ext show fails cleanly on malformed extension" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext show malformed-ext
  assert_failure
  assert_output_contains "malformed"
}

# ====================================================================
# validate
# ====================================================================

@test "ext validate passes valid-ext" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate valid-ext
  assert_success
  assert_output_contains "1 ok"
}

@test "ext validate fails malformed-ext with schema error" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate malformed-ext
  assert_failure
  assert_output_contains "missing required field 'schema'"
}

@test "ext validate rejects path traversal" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate traversal-ext
  assert_failure
  assert_output_contains "path traversal"
}

@test "ext validate passes shadow-ext (shadow is a runtime warning, not a validation error)" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate shadow-ext
  assert_success
  assert_output_contains "1 ok"
}

@test "ext validate with no name checks every extension and summarises" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate
  assert_failure
  assert_output_contains "summary:"
  assert_output_contains "4 checked"
}

@test "ext validate fails on unknown extension" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent ext validate no-such-ext
  assert_failure
  assert_output_contains "not found"
}

# ====================================================================
# new
# ====================================================================

@test "ext new requires a name" {
  local sandbox="$TEST_TEMP_DIR/ext-new-noargs"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new --subagent
  assert_failure
  assert_output_contains "extension name required"
}

@test "ext new requires --subagent / --skill / --rule-pack" {
  local sandbox="$TEST_TEMP_DIR/ext-new-notype"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new hello
  assert_failure
  assert_output_contains "--subagent, --skill, or --rule-pack required"
}

@test "ext new --subagent scaffolds a valid extension" {
  local sandbox="$TEST_TEMP_DIR/ext-new-subagent"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new hello-agent --subagent
  assert_success
  assert_output_contains "scaffolded subagent extension"

  # Round-trip through validate
  run run_intent ext validate hello-agent
  assert_success
  assert_output_contains "1 ok"

  assert_file_exists "$sandbox/hello-agent/extension.json"
  assert_file_exists "$sandbox/hello-agent/subagents/hello-agent/agent.md"
  assert_file_exists "$sandbox/hello-agent/subagents/hello-agent/metadata.json"
  assert_file_exists "$sandbox/hello-agent/README.md"
}

@test "ext new --skill scaffolds a valid extension" {
  local sandbox="$TEST_TEMP_DIR/ext-new-skill"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new my-skill --skill
  assert_success

  run run_intent ext validate my-skill
  assert_success
  assert_output_contains "1 ok"

  assert_file_exists "$sandbox/my-skill/skills/my-skill/SKILL.md"
}

@test "ext new --rule-pack scaffolds a valid extension with placeholder rule" {
  local sandbox="$TEST_TEMP_DIR/ext-new-rulepack"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new my-rules --rule-pack
  assert_success

  run run_intent ext validate my-rules
  assert_success
  assert_output_contains "1 ok"

  assert_file_exists "$sandbox/my-rules/rules/agnostic/my-rules-sample/RULE.md"
}

@test "ext new refuses to overwrite existing extension" {
  local sandbox="$TEST_TEMP_DIR/ext-new-collision"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new hello-agent --subagent
  assert_success

  run run_intent ext new hello-agent --subagent
  assert_failure
  assert_output_contains "already exists"
}

@test "ext new rejects invalid name" {
  local sandbox="$TEST_TEMP_DIR/ext-new-badname"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new "Bad-Name" --subagent
  assert_failure
  assert_output_contains "must match"
}

@test "ext new rejects unknown flag" {
  local sandbox="$TEST_TEMP_DIR/ext-new-badflag"
  mkdir -p "$sandbox"
  export INTENT_EXT_DIR="$sandbox"
  run run_intent ext new hello --subagent --bogus
  assert_failure
  assert_output_contains "unknown flag"
}

# ====================================================================
# help
# ====================================================================

@test "ext help prints usage" {
  run run_intent ext help
  assert_success
  assert_output_contains "intent ext"
  assert_output_contains "list"
  assert_output_contains "show"
  assert_output_contains "validate"
  assert_output_contains "new"
}

@test "ext with unknown subcommand fails with usage hint" {
  run run_intent ext no-such-command
  assert_failure
  assert_output_contains "unknown ext subcommand"
}
