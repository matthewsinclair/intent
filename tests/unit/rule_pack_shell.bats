#!/usr/bin/env bats
# Tests for the shell rule pack (WP12).

load "../lib/test_helper.bash"

SHELL_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/shell"

shell_rules() {
  cat <<'EOF'
code/quote-expansions|IN-SH-CODE-001
code/no-parse-ls|IN-SH-CODE-002
code/set-euo-pipefail|IN-SH-CODE-003
code/setopt-err-exit|IN-SH-CODE-004
code/no-silent-exit-codes|IN-SH-CODE-005
code/module-highlander|IN-SH-CODE-006
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "shell pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$SHELL_ROOT/$slug/RULE.md"
  done < <(shell_rules)
}

@test "shell pack has the expected total rule count" {
  local expected actual
  expected=$(shell_rules | grep -c '|' || true)
  actual=$(find "$SHELL_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "shell rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "shell pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$SHELL_ROOT/$slug/RULE.md" "id: $id"
  done < <(shell_rules)
}

@test "shell pack: each rule declares language: shell" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$SHELL_ROOT/$slug/RULE.md" 'language: shell'
  done < <(shell_rules)
}

# ====================================================================
# Dialect tagging (bash-specific / zsh-specific)
# ====================================================================

@test "shell pack: bash-specific rule tagged correctly" {
  assert_file_contains "$SHELL_ROOT/code/set-euo-pipefail/RULE.md" 'bash-specific'
}

@test "shell pack: zsh-specific rule tagged correctly" {
  assert_file_contains "$SHELL_ROOT/code/setopt-err-exit/RULE.md" 'zsh-specific'
}

# ====================================================================
# Validator agreement
# ====================================================================

@test "shell pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$SHELL_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(shell_rules)
}

@test "shell pack: rules list reports every shell id" {
  run run_intent claude rules list --lang shell
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(shell_rules)
}

# ====================================================================
# Textual-examples invariant
# ====================================================================

@test "shell pack: each rule has a fenced bash or zsh code block in Bad section" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$SHELL_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qE '^```(bash|zsh|sh)$' || {
      echo "$rule: Bad section missing fenced bash/zsh/sh block" >&2
      return 1
    }
  done < <(shell_rules)
}

@test "shell pack: each rule has a fenced bash or zsh code block in Good section" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$SHELL_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qE '^```(bash|zsh|sh)$' || {
      echo "$rule: Good section missing fenced bash/zsh/sh block" >&2
      return 1
    }
  done < <(shell_rules)
}

# ====================================================================
# Concretised-by backlinks (WP04 agnostic update invariant)
# ====================================================================

@test "agnostic no-silent-errors lists IN-SH-CODE-005 in concretised_by" {
  assert_file_contains "${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/agnostic/no-silent-errors/RULE.md" 'IN-SH-CODE-005'
}

@test "agnostic highlander lists IN-SH-CODE-006 in concretised_by" {
  assert_file_contains "${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/agnostic/highlander/RULE.md" 'IN-SH-CODE-006'
}

# ====================================================================
# critic-shell subagent is registered
# ====================================================================

@test "critic-shell subagent directory exists with agent.md and metadata.json" {
  local critic_dir="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/critic-shell"
  [ -d "$critic_dir" ]
  [ -f "$critic_dir/agent.md" ]
  [ -f "$critic_dir/metadata.json" ]
}

@test "critic-shell registered in global-agents.json" {
  local manifest="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/.manifest/global-agents.json"
  jq -r '.agents[].name' "$manifest" | grep -qx "critic-shell"
}
