#!/usr/bin/env bats
# Tests for multi-root ext discovery precedence, shadow warnings, and env-var
# escape hatch across intent claude subagents / skills / rules.

load "../lib/test_helper.bash"

EXT_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures/extensions"

# ====================================================================
# subagents list: ext discovery
# ====================================================================

@test "subagents list shows ext subagents when INTENT_EXT_DIR is set" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude subagents list
  assert_success
  assert_output_contains "Extensions:"
  assert_output_contains "ext-sample-agent"
  assert_output_contains "[ext:valid-ext]"
}

@test "subagents list tags shadowed canon names with shadows-canon marker" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude subagents list
  assert_success
  assert_output_contains "[ext:shadow-ext, shadows canon]"
}

@test "INTENT_EXT_DISABLE=1 suppresses ext discovery in subagents list" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  export INTENT_EXT_DISABLE=1
  run run_intent claude subagents list
  assert_success
  refute_output_contains "Extensions:"
  refute_output_contains "ext-sample-agent"
  refute_output_contains "shadows canon"
}

@test "subagents list still shows canon agents when ext is set" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude subagents list
  assert_success
  assert_output_contains "Global:"
  assert_output_contains "intent"
  assert_output_contains "elixir"
}

# ====================================================================
# skills list: ext discovery
# ====================================================================

@test "skills list shows ext skills when INTENT_EXT_DIR is set" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude skills list
  assert_success
  assert_output_contains "Extensions:"
  assert_output_contains "in-ext-sample"
  assert_output_contains "[ext:valid-ext]"
}

@test "INTENT_EXT_DISABLE=1 suppresses ext skill discovery" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  export INTENT_EXT_DISABLE=1
  run run_intent claude skills list
  assert_success
  refute_output_contains "in-ext-sample"
  refute_output_contains "Extensions:"
}

@test "skills list still shows canon skills when ext is set" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude skills list
  assert_success
  assert_output_contains "in-essentials"
  assert_output_contains "in-elixir-essentials"
}

# ====================================================================
# rules list: ext discovery
# ====================================================================

@test "claude rules list enumerates ext rules with ext provenance tag" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude rules list
  assert_success
  assert_output_contains "IN-AG-EXT-001"
  assert_output_contains "ext:valid-ext"
}

@test "INTENT_EXT_DISABLE=1 suppresses ext rule discovery" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  export INTENT_EXT_DISABLE=1
  run run_intent claude rules list
  assert_success
  refute_output_contains "IN-AG-EXT-001"
}

# ====================================================================
# Precedence: ext wins for install resolution
# ====================================================================

@test "subagents show on a shadowed name emits shadow warning" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude subagents show intent
  # shadow warning reaches stderr, which bats folds into output
  [[ "$output" == *"shadowed"* ]] || [[ "$output" == *"shadows canon"* ]]
}

# ====================================================================
# INTENT_EXT_DIR override semantics
# ====================================================================

@test "INTENT_EXT_DIR pointing at a nonexistent dir does not crash list commands" {
  export INTENT_EXT_DIR="/tmp/intent-no-such-ext-dir-xyz"
  run run_intent claude subagents list
  assert_success
  run run_intent claude skills list
  assert_success
  run run_intent claude rules list
  assert_success
}

@test "INTENT_EXT_DIR pointing at an empty dir reports empty extensions" {
  export INTENT_EXT_DIR="$TEST_TEMP_DIR"
  run run_intent ext list
  assert_success
  assert_output_contains "no extensions installed"
}

# ====================================================================
# Dotfiles and _ directories are excluded
# ====================================================================

@test "ext enumeration skips dotfile and underscore directories" {
  local sandbox="$TEST_TEMP_DIR/ext-skip-test"
  mkdir -p "$sandbox/.hidden/subagents/foo" "$sandbox/_reserved/subagents/bar" "$sandbox/valid-one/subagents/baz"
  cat > "$sandbox/.hidden/subagents/foo/agent.md" << 'EOF'
---
name: foo
description: Should be excluded
---
# foo
EOF
  cat > "$sandbox/_reserved/subagents/bar/agent.md" << 'EOF'
---
name: bar
description: Should be excluded
---
# bar
EOF
  cat > "$sandbox/valid-one/subagents/baz/agent.md" << 'EOF'
---
name: baz
description: Should be visible
---
# baz
EOF

  export INTENT_EXT_DIR="$sandbox"
  run run_intent claude subagents list
  assert_success
  assert_output_contains "baz"
  refute_output_contains "foo"
  refute_output_contains "bar"
}
