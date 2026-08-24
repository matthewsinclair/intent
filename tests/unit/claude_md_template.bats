#!/usr/bin/env bats
# Tests for lib/templates/llm/_CLAUDE.md (ST0035/WP-09).
#
# Scenarios: template is under the length budget, contains the required
# canon landmarks, uses the canonical [[PLACEHOLDER]] syntax, and is
# sed-substituted correctly by `intent init` on a scratch project.

load "../lib/test_helper.bash"

TEMPLATE="${INTENT_PROJECT_ROOT}/lib/templates/llm/_CLAUDE.md"

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

@test "template references persistent memory directory" {
  assert_file_contains "$TEMPLATE" "~/.claude/projects"
  assert_file_contains "$TEMPLATE" "memory"
}

@test "template references .claude/settings.json hooks" {
  assert_file_contains "$TEMPLATE" ".claude/settings.json"
  assert_file_contains "$TEMPLATE" "session-hook-architecture"
}

@test "template includes a file map with canon landmarks" {
  assert_file_contains "$TEMPLATE" "usage-rules.md"
  assert_file_contains "$TEMPLATE" "working-with-llms.md"
  assert_file_contains "$TEMPLATE" "intent/llm/MODULES.md"
  assert_file_contains "$TEMPLATE" "intent/llm/DECISION_TREE.md"
}

# THE FOUR RULE IDS MOVED TO _AGENTS.md, AND THIS TEST MOVED WITH THEM.
#
# It used to assert the IDs were in _CLAUDE.md. They are not, deliberately:
# AGENTS.md is declared the primary tool-agnostic contract and carried NONE of
# the four rules it is said to hold, while the file described as "a Claude
# Code-specific overlay" carried all four. The layering was inverted, so a
# non-Claude agent following the stated reading order got none of the rules of
# the road. Measured 2026-08-24 and ruled by hv.
#
# The assertion is now STRONGER than it was, in both directions: the contract
# must hold the IDs, AND the overlay must not repeat them. A test that only
# checked presence would have passed on the duplication this change removed.
@test "the four rule IDs live in the AGENTS contract, not the Claude overlay" {
  local agents="${INTENT_PROJECT_ROOT}/lib/templates/llm/_AGENTS.md"
  local id
  for id in IN-AG-HIGHLANDER-001 IN-AG-PFIC-001 IN-AG-THIN-COORD-001 IN-AG-NO-SILENT-001; do
    assert_file_contains "$agents" "$id"
    grep -q "$id" "$TEMPLATE" \
      && fail "$id is restated in _CLAUDE.md; AGENTS.md is the contract and a second copy is the Highlander violation this move removed"
  done
  # The overlay still ROUTES to the rule bodies even though it does not name
  # the IDs -- a pointer with no way to follow it is worse than no pointer.
  assert_file_contains "$TEMPLATE" "intent claude rules show"
  # And it must say WHERE they are, or the reader has a prohibition and no
  # destination.
  assert_file_contains "$TEMPLATE" "AGENTS.md"
}

@test "template includes critic dispatch section" {
  assert_file_contains "$TEMPLATE" "critic-"
  assert_file_contains "$TEMPLATE" "Task(subagent_type"
  assert_file_contains "$TEMPLATE" "intent/docs/critics.md"
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

@test "intent init on a scratch project substitutes all placeholders" {
  local project_dir="${TEST_TEMP_DIR}/scratch"
  mkdir -p "$project_dir"
  cd "$project_dir" || exit 1

  INTENT_AUTHOR="TestUser" run run_intent init "ScratchProj"
  assert_success
  assert_file_exists "$project_dir/CLAUDE.md"

  # No placeholder should remain in the generated CLAUDE.md.
  run grep -F "[[" "$project_dir/CLAUDE.md"
  [ "$status" -ne 0 ] || fail "unsubstituted placeholders remain in generated CLAUDE.md"

  # Project name + author substituted from CLI arg / env.
  assert_file_contains "$project_dir/CLAUDE.md" "ScratchProj"
  assert_file_contains "$project_dir/CLAUDE.md" "TestUser"
}
