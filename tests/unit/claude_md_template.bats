#!/usr/bin/env bats
# Tests for lib/templates/llm/_CLAUDE.md (ST0035/WP-09).
#
# Scenarios: template is under the length budget, contains the required
# canon landmarks, uses the canonical [[PLACEHOLDER]] syntax, is
# sed-substituted correctly by `intent init` on a scratch project, and the
# CLAUDE.md that init writes opens with the `@AGENTS.md` import (ST0083).

load "../lib/test_helper.bash"

TEMPLATE="${INTENT_PROJECT_ROOT}/lib/templates/llm/_CLAUDE.md"
AGENTS_TEMPLATE="${INTENT_PROJECT_ROOT}/lib/templates/llm/_AGENTS.md"

@test "template exists" {
  [ -f "$TEMPLATE" ]
}

@test "template is within the 100-line budget" {
  local n
  n=$(wc -l < "$TEMPLATE")
  [ "$n" -le 100 ] || fail "template exceeds 100 lines ($n)"
}

@test "template references AGENTS.md as the primary contract" {
  assert_file_contains "$TEMPLATE" "AGENTS.md"
  assert_file_contains "$TEMPLATE" "primary"
}

@test "template directs Claude to run /in-session" {
  assert_file_contains "$TEMPLATE" "/in-session"
  assert_file_contains "$TEMPLATE" "compact"
}

@test "template references persistent memory directory under the config dir" {
  assert_file_contains "$TEMPLATE" "\${CLAUDE_CONFIG_DIR:-\$HOME/.claude}/projects"
  assert_file_contains "$TEMPLATE" "memory"
}

@test "template references .claude/settings.json hooks" {
  assert_file_contains "$TEMPLATE" ".claude/settings.json"
  assert_file_contains "$TEMPLATE" "session-hook-architecture"
}

# ST0088 WP-03: the file map has one home, _AGENTS.md, which _CLAUDE.md loads
# through its @AGENTS.md line.
@test "the file map lives in _AGENTS.md, which _CLAUDE.md imports" {
  assert_file_contains "$AGENTS_TEMPLATE" "usage-rules.md"
  assert_file_contains "$AGENTS_TEMPLATE" "working-with-llms.md"
  assert_file_contains "$AGENTS_TEMPLATE" "intent/llm/MODULES.md"
  assert_file_contains "$AGENTS_TEMPLATE" "intent/llm/DECISION_TREE.md"
  run grep -F "intent/llm/MODULES.md" "$TEMPLATE"
  assert_failure
}

# ST0088 WP-03: critic dispatch has one home, _AGENTS.md.
@test "critic dispatch lives in _AGENTS.md, not _CLAUDE.md" {
  assert_file_contains "$AGENTS_TEMPLATE" "critic-"
  assert_file_contains "$AGENTS_TEMPLATE" "Task(subagent_type"
  assert_file_contains "$AGENTS_TEMPLATE" "intent/docs/critics.md"
  run grep -F "Task(subagent_type" "$TEMPLATE"
  assert_failure
}

@test "template includes user-preservation markers" {
  assert_file_contains "$TEMPLATE" "user:start"
  assert_file_contains "$TEMPLATE" "user:end"
}

@test "template uses the three canonical placeholders" {
  assert_file_contains "$TEMPLATE" "[[PROJECT_NAME]]"
  assert_file_contains "$TEMPLATE" "[[INTENT_VERSION]]"
  assert_file_contains "$TEMPLATE" "[[AUTHOR]]"
}

@test "template carries no [[DATE]]: a generated file never stamps its own generation" {
  # ST0057 AC-00.4 (`b277013a`). `[[DATE]]` was REFUSED rather than substituted,
  # and it was the one real design call in that change.
  #
  # `RenderContext` carries facts about the tool or about the project's data,
  # never about the MOMENT OF RENDERING. A generated file that stamps its own
  # generation differs from itself on every run -- which is AC-03.17's churn
  # loop with a timestamp in it, and D42's rule reached from the other side.
  #
  # THIS IS A NEGATIVE ASSERTION ON PURPOSE, AND THAT IS THE WHOLE POINT OF IT.
  # Trimming the positive test from four placeholders to three RECORDS the
  # removal without DEFENDING it: a re-added `[[DATE]]` passes a
  # three-placeholder test in silence, and the refusal above is then something
  # the next author never meets. Going red is what puts them in front of it.
  #
  # Neither line breaks on the token being absent, and that was checked rather
  # than assumed: v3's renderer refuses an unknown token outright, and v2
  # substitutes with a plain `sed` chain (`bin/intent_init:134`) where an absent
  # token is a no-op.
  run grep -F '[[DATE]]' "$TEMPLATE"
  [ "$status" -ne 0 ] || fail "template re-introduced [[DATE]] -- a generated file that stamps its own generation differs from itself on every run (ST0057 AC-00.4, b277013a)"
}

@test "template routes rule access through the CLI, not a local rules directory" {
  # Regression guard for v2.11.11: the rules library is served by the installed
  # Intent tool, not vendored into a consuming project. The template must point
  # agents at `intent claude rules show/list`, never at a local
  # `intent/plugins/claude/rules/` path that does not exist in consumers.
  assert_file_contains "$TEMPLATE" "intent claude rules show"
  assert_file_contains "$TEMPLATE" "intent claude rules list"
  run grep -F 'intent/plugins/claude/rules' "$TEMPLATE"
  [ "$status" -ne 0 ] || fail "template still points at a local rules directory"
}

# `intent init` on a scratch project, leaving the run's status and output for
# the arm to judge. v3's one author source is `author` in the user config,
# which `bootstrap` writes from $USER (hv, 2026-08-27) -- never git, never an
# env var. The fixture writes that source directly, so HOME MUST be the sandbox
# first: without `setup_fake_home` this wrote the operator's real config (0368).
init_scratch_project() {
  SCRATCH_DIR="${TEST_TEMP_DIR}/scratch"
  mkdir -p "$SCRATCH_DIR"
  cd "$SCRATCH_DIR" || return 1
  setup_fake_home
  mkdir -p "$HOME/.config/intent"
  printf '{"author":"TestUser"}\n' > "$HOME/.config/intent/config.json"
  run run_intent init "ScratchProj"
}

@test "intent init on a scratch project substitutes all placeholders" {
  init_scratch_project
  assert_success
  assert_file_exists "$SCRATCH_DIR/CLAUDE.md"

  # No placeholder should remain in the generated CLAUDE.md.
  run grep -F "[[" "$SCRATCH_DIR/CLAUDE.md"
  [ "$status" -ne 0 ] || fail "unsubstituted placeholders remain in generated CLAUDE.md"

  # Project name from the CLI arg; author from the user config, in both files.
  assert_file_contains "$SCRATCH_DIR/CLAUDE.md" "ScratchProj"
  assert_file_contains "$SCRATCH_DIR/CLAUDE.md" "TestUser"
  assert_file_contains "$SCRATCH_DIR/intent/.config/config.json" '"author": "TestUser"'
}

@test "intent init writes a CLAUDE.md whose first line imports AGENTS.md" {
  # ST0083 AT-01.1, covering AC-01.1. Under Claude Code's default, a project with a CLAUDE.md
  # loads that file and never AGENTS.md, so a prose pointer to AGENTS.md is a
  # request the harness does not honour; the `@AGENTS.md` import is what loads
  # it. The line is read off the GENERATED file, which is what Claude Code
  # reads, so a renderer that dropped or moved it fails here as well as a
  # template that lost it.
  init_scratch_project
  assert_success
  local first
  first="$(head -n 1 "$SCRATCH_DIR/CLAUDE.md")"
  [ "$first" = "@AGENTS.md" ] || fail "the generated CLAUDE.md opens with '$first', not @AGENTS.md"
}
