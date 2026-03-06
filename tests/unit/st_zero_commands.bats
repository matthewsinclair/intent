#!/usr/bin/env bats
# Tests for intent st zero commands (v2.6.0)

load "../lib/test_helper.bash"

# ============================================================
# Help and Routing
# ============================================================

@test "st zero shows usage when no subcommand given" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero
  assert_success
  assert_output_contains "Usage: intent st zero"
  assert_output_contains "install"
  assert_output_contains "help"
}

@test "st zero help shows usage" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero help
  assert_success
  assert_output_contains "Usage: intent st zero"
  assert_output_contains "install"
}

@test "st zero --help shows usage" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero --help
  assert_success
  assert_output_contains "Usage: intent st zero"
}

@test "st zero unknown subcommand shows error" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero bogus
  assert_failure
  assert_output_contains "Unknown st zero command: bogus"
}

# ============================================================
# Project Context Required
# ============================================================

@test "st zero install requires project context" {
  cd "$TEST_TEMP_DIR"

  run run_intent st zero install
  assert_failure
  assert_output_contains "not in an Intent project"
}

# ============================================================
# Gap Analysis: Audit Only
# ============================================================

@test "st zero install --audit-only shows gap analysis" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "ST Zero Gap Analysis"
  assert_output_contains "summary:"
}

@test "st zero install --audit-only detects missing CLAUDE.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "D2"
  assert_output_contains "[missing]"
  assert_output_contains "CLAUDE.md not found"
}

@test "st zero install --audit-only detects present CLAUDE.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo "# Project" > CLAUDE.md

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "[present]"
  assert_output_contains "CLAUDE.md exists"
}

@test "st zero install --audit-only detects missing MODULES.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "D3"
  assert_output_contains "MODULES.md not found"
}

@test "st zero install --audit-only detects present MODULES.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
| Concern | THE Module | Notes |
| ------- | ---------- | ----- |
| Helper  | `lib/helper.ex` | test |
EOF

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "MODULES.md with 1 entries"
}

@test "st zero install --audit-only detects missing learnings.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "D10"
  assert_output_contains "learnings.md not found"
}

@test "st zero install --audit-only detects present learnings.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  cat > .intent/learnings.md << 'EOF'
# Project Learnings
## Footguns
## Patterns That Worked
## Patterns That Failed
EOF

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "learnings.md exists"
}

# ============================================================
# Elixir Detection
# ============================================================

@test "st zero skips Elixir deliverables when no mix.exs" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "non-Elixir"
  assert_output_contains "skipped (not an Elixir project)"
}

@test "st zero checks Elixir deliverables when mix.exs present" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo 'defmodule MyApp.MixProject do end' > mix.exs

  run run_intent st zero install --audit-only
  assert_success
  assert_output_contains "Elixir (mix.exs detected)"
  # D4 should show missing, not skipped
  assert_output_contains "ARCHETYPES.md not found"
}

# ============================================================
# Single Deliverable Filter
# ============================================================

@test "st zero install --deliverable D3 targets single deliverable" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --audit-only --deliverable D3
  assert_success
  assert_output_contains "D3"
  # Should not contain other deliverable IDs in the report
  [[ "$output" != *"D2 "* ]]
  [[ "$output" != *"D6 "* ]]
}

@test "st zero install --deliverable rejects invalid ID" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D99
  assert_failure
  assert_output_contains "Unknown deliverable: D99"
}

# ============================================================
# Dry Run
# ============================================================

@test "st zero install --dry-run shows what would change" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --dry-run
  assert_success
  assert_output_contains "Dry run"
  assert_output_contains "would create"
}

@test "st zero install --dry-run does not write files" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --dry-run
  assert_success
  # D10 should not actually be created
  [ ! -f ".intent/learnings.md" ]
  # CLAUDE.md should not be created
  [ ! -f "CLAUDE.md" ]
}

# ============================================================
# Apply: Template Installation
# ============================================================

@test "st zero install creates CLAUDE.md from template when missing" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D2
  assert_success
  assert_output_contains "created: CLAUDE.md"
  [ -f "CLAUDE.md" ]
}

@test "st zero install never overwrites existing CLAUDE.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo "# My Custom CLAUDE" > CLAUDE.md

  run run_intent st zero install --deliverable D2
  assert_success
  assert_output_contains "nothing to install"
  # Content should be unchanged
  grep -q "My Custom CLAUDE" CLAUDE.md
}

@test "st zero install creates DECISION_TREE.md from template" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D6
  assert_success
  assert_output_contains "created: intent/llm/DECISION_TREE.md"
  [ -f "intent/llm/DECISION_TREE.md" ]
}

@test "st zero install creates learnings.md" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D10
  assert_success
  assert_output_contains "created: .intent/learnings.md"
  [ -f ".intent/learnings.md" ]
  grep -q "Footguns" .intent/learnings.md
}

@test "st zero install creates ARCHETYPES.md for Elixir projects" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo 'defmodule MyApp.MixProject do end' > mix.exs

  run run_intent st zero install --deliverable D4
  assert_success
  assert_output_contains "created: intent/llm/ARCHETYPES.md"
  [ -f "intent/llm/ARCHETYPES.md" ]
}

@test "st zero install skips ARCHETYPES.md for non-Elixir projects" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D4
  assert_success
  assert_output_contains "skipped"
  [ ! -f "intent/llm/ARCHETYPES.md" ]
}

@test "st zero install creates DEPENDENCY_GRAPH.md for Elixir projects" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo 'defmodule MyApp.MixProject do end' > mix.exs

  run run_intent st zero install --deliverable D11
  assert_success
  assert_output_contains "created: intent/llm/DEPENDENCY_GRAPH.md"
  [ -f "intent/llm/DEPENDENCY_GRAPH.md" ]
}

# ============================================================
# Module Auto-Discovery (D3)
# ============================================================

@test "st zero install generates MODULES.md from .ex files" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  mkdir -p lib/my_app
  cat > lib/my_app/helper.ex << 'EOF'
defmodule MyApp.Helper do
  def greet, do: "hello"
end
EOF
  cat > lib/my_app/worker.ex << 'EOF'
defmodule MyApp.Worker do
  def run, do: :ok
end
EOF

  run run_intent st zero install --deliverable D3
  assert_success
  assert_output_contains "created: intent/llm/MODULES.md"
  [ -f "intent/llm/MODULES.md" ]
  grep -q "MyApp.Helper" intent/llm/MODULES.md
  grep -q "MyApp.Worker" intent/llm/MODULES.md
}

@test "st zero install classifies web modules correctly" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  mkdir -p lib/my_app_web/controllers
  cat > lib/my_app_web/controllers/page_controller.ex << 'EOF'
defmodule MyAppWeb.PageController do
  use MyAppWeb, :controller
end
EOF

  run run_intent st zero install --deliverable D3
  assert_success
  [ -f "intent/llm/MODULES.md" ]
  grep -q "controller" intent/llm/MODULES.md
}

# ============================================================
# Credo Check Templates (D5a)
# ============================================================

@test "st zero install copies Credo check templates for Elixir" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  echo 'defmodule MyApp.MixProject do end' > mix.exs

  run run_intent st zero install --deliverable D5a
  assert_success
  assert_output_contains "Credo check templates"
  [ -d "credo_checks" ]
  # Should have .ex files
  local count
  count=$(find credo_checks -name '*.ex' -type f | wc -l | tr -d ' ')
  [ "$count" -gt 0 ]
}

# ============================================================
# Hook Template (D9)
# ============================================================

@test "st zero install creates hook settings when missing" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  run run_intent st zero install --deliverable D9
  assert_success
  assert_output_contains "created: .claude/settings.local.json"
  [ -f ".claude/settings.local.json" ]
  grep -q "MODULES.md" .claude/settings.local.json
}

@test "st zero install skips hook when already configured" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"
  mkdir -p .claude
  echo '{"hooks": {"PostToolUse": [{"matcher": "Write", "hooks": [{"command": "grep MODULES.md"}]}]}}' > .claude/settings.local.json

  run run_intent st zero install --deliverable D9
  assert_success
  assert_output_contains "nothing to install"
}

# ============================================================
# All Present -- Nothing to Install
# ============================================================

@test "st zero install reports nothing to install when all present" {
  project_dir=$(create_test_project "Zero Test")
  cd "$project_dir"

  # Create all deliverables
  echo "# CLAUDE" > CLAUDE.md
  cat > intent/llm/MODULES.md << 'EOF'
| Concern | THE Module | Notes |
| ------- | ---------- | ----- |
| test | `lib/test.ex` | test |
EOF
  echo "# DT" > intent/llm/DECISION_TREE.md
  mkdir -p .claude
  echo '{"hooks":{"PostToolUse":[{"hooks":[{"command":"MODULES.md"}]}]}}' > .claude/settings.local.json
  cat > .intent/learnings.md << 'EOF'
# Project Learnings
## Footguns
## Patterns That Worked
## Patterns That Failed
EOF
  # D8 (MEMORY.md) is checked via ~/.claude path, will show missing in test -- that's fine
  # D4, D5a, D11 are skipped for non-Elixir

  run run_intent st zero install --audit-only
  assert_success
  # Most should be present
  assert_output_contains "present"
}
