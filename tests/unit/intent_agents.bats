#!/usr/bin/env bats
# Tests for intent/plugins/agents/bin/intent_agents (ST0035/WP-08).
#
# Scenarios: init writes root AGENTS.md (not intent/llm/), init --template
# elixir places AGENTS.md at root and RULES/ARCHITECTURE under intent/llm/,
# sync replaces legacy symlinks with real files, sync is idempotent, sync
# renders dynamic skills/subagents from .claude/, sync renders fallbacks
# when .claude/ is absent, validate passes on clean projects, validate
# warns on legacy symlinks.

load "../lib/test_helper.bash"

@test "intent_agents script is executable" {
  [ -x "${INTENT_HOME}/intent/plugins/agents/bin/intent_agents" ]
}

@test "intent_agents script syntax is valid" {
  run bash -n "${INTENT_HOME}/intent/plugins/agents/bin/intent_agents"
  assert_success
}

@test "intent agents help shows the canon summary" {
  run run_intent agents help
  assert_success
  assert_output_contains "Usage: intent agents <command>"
  assert_output_contains "primary tool-agnostic LLM config"
  assert_output_contains "init"
  assert_output_contains "sync"
  assert_output_contains "validate"
}

@test "init writes root AGENTS.md as a regular file, not intent/llm/AGENTS.md" {
  local project
  project="$(create_test_project "AgentsInit")"
  cd "$project"
  run run_intent agents init
  assert_success
  assert_file_exists "$project/AGENTS.md"
  assert_file_not_exists "$project/intent/llm/AGENTS.md"
  [ ! -L "$project/AGENTS.md" ] || fail "AGENTS.md should be a regular file, not a symlink"
}

@test "init --template elixir writes root AGENTS.md and keeps RULES/ARCHITECTURE under intent/llm/" {
  local project
  project="$(create_test_project "AgentsElixirInit")"
  cd "$project"
  run run_intent agents init --template elixir
  assert_success
  assert_file_exists "$project/AGENTS.md"
  assert_file_exists "$project/intent/llm/RULES.md"
  assert_file_exists "$project/intent/llm/ARCHITECTURE.md"
  assert_file_not_exists "$project/intent/llm/AGENTS.md"
  # Elixir template refs explicitly point at intent/llm/ siblings now.
  assert_file_contains "$project/AGENTS.md" "intent/llm/RULES.md"
  assert_file_contains "$project/intent/llm/RULES.md" "RULES.md"
}

@test "sync replaces a pre-existing symlink with a real file" {
  local project
  project="$(create_test_project "AgentsSyncSymlink")"
  cd "$project"
  mkdir -p intent/llm
  echo "# legacy AGENTS.md" > intent/llm/AGENTS.md
  ln -s intent/llm/AGENTS.md AGENTS.md
  [ -L "$project/AGENTS.md" ] || fail "pre-condition: root AGENTS.md should be a symlink"

  run run_intent agents sync
  assert_success
  [ ! -L "$project/AGENTS.md" ] || fail "post-condition: AGENTS.md should not be a symlink"
  [ -f "$project/AGENTS.md" ] || fail "post-condition: AGENTS.md should be a regular file"
}

@test "sync is idempotent -- two consecutive runs produce byte-identical output" {
  local project
  project="$(create_test_project "AgentsIdempotent")"
  cd "$project"
  run_intent agents init >/dev/null
  run_intent agents sync >/dev/null
  cp AGENTS.md "${TEST_TEMP_DIR}/run1.md"
  run_intent agents sync >/dev/null
  diff "${TEST_TEMP_DIR}/run1.md" AGENTS.md || fail "two syncs produced different output"
}

@test "sync renders installed skills and subagents from .claude/" {
  local project
  project="$(create_test_project "AgentsDynamic")"
  cd "$project"

  mkdir -p .claude/skills/example-skill
  cat > .claude/skills/example-skill/SKILL.md <<'EOF'
---
description: "Example skill for dynamic section test"
---
# Example
EOF

  mkdir -p .claude/agents
  cat > .claude/agents/example-agent.md <<'EOF'
---
name: example-agent
description: Example agent for dynamic section test
---
Body.
EOF

  run_intent agents init >/dev/null
  assert_file_contains "$project/AGENTS.md" "**example-skill**"
  assert_file_contains "$project/AGENTS.md" "Example skill for dynamic section test"
  assert_file_contains "$project/AGENTS.md" "**example-agent**"
  assert_file_contains "$project/AGENTS.md" "Example agent for dynamic section test"
}

@test "sync renders fallbacks when .claude/ is absent" {
  local project
  project="$(create_test_project "AgentsNoClaude")"
  cd "$project"
  run_intent agents init >/dev/null
  assert_file_contains "$project/AGENTS.md" "No skills installed"
  assert_file_contains "$project/AGENTS.md" "No subagents installed"
}

@test "validate passes on a clean project (regular file at root)" {
  local project
  project="$(create_test_project "AgentsValidate")"
  cd "$project"
  run_intent agents init >/dev/null
  run run_intent agents validate
  assert_success
  assert_output_contains "validation passed"
  assert_output_contains "regular file"
}

@test "validate warns on a legacy symlink (exit 0, warning on output)" {
  local project
  project="$(create_test_project "AgentsValidateSymlink")"
  cd "$project"
  mkdir -p intent/llm
  echo "# legacy" > intent/llm/AGENTS.md
  ln -s intent/llm/AGENTS.md AGENTS.md

  run run_intent agents validate
  assert_success
  assert_output_contains "legacy canon"
  assert_output_contains "intent agents sync"
}

@test "enriched content includes canon section pointers (critic, rules, extensions, hooks, FAQ)" {
  local project
  project="$(create_test_project "AgentsCanon")"
  cd "$project"
  run_intent agents init >/dev/null
  assert_file_contains "$project/AGENTS.md" "## Critic Family"
  assert_file_contains "$project/AGENTS.md" "## Rule Library"
  assert_file_contains "$project/AGENTS.md" "## Extensions"
  assert_file_contains "$project/AGENTS.md" "## Session Hooks"
  assert_file_contains "$project/AGENTS.md" "## Socrates vs Diogenes FAQ"
  assert_file_contains "$project/AGENTS.md" "working-with-llms.md#socrates-vs-diogenes-faq"
  assert_file_contains "$project/AGENTS.md" "working-with-llms.md#session-hook-architecture"
}
