#!/usr/bin/env bats
# Highlander audit for skills vs. rules (WP03).
#
# After WP03, skills no longer inline rule prose — they reference rules by
# ID. The proxy signal for "is this skill still inlining rule content?" is
# the presence of fenced code blocks: each WP03-refactored skill should
# carry zero ```fences. Rule text lives in RULE.md; skills are pointers.
#
# This test is not strict about every in-* skill: it applies specifically to
# the skills WP03 rewrote to reference the rule library.

load "../lib/test_helper.bash"

SKILL_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills"

# Skills that WP03 refactored to reference rules. These MUST have zero
# fenced code blocks.
wp03_refactored_skills() {
  cat <<'EOF'
in-elixir-essentials
in-elixir-testing
in-ash-ecto-essentials
in-phoenix-liveview
in-standards
EOF
}

@test "WP03-refactored skills carry no fenced code blocks" {
  local slug skill count
  while read -r slug; do
    [ -z "$slug" ] && continue
    skill="$SKILL_ROOT/$slug/SKILL.md"
    count=$(grep -cE '^```' "$skill" || true)
    [ "$count" = "0" ] || {
      echo "skill $slug has $count fenced code blocks in $skill" >&2
      echo "rule prose belongs in RULE.md, not SKILL.md" >&2
      return 1
    }
  done < <(wp03_refactored_skills)
}

@test "WP03-refactored skills are thin (under 150 lines)" {
  # A thin pointer skill should not be huge. 150 lines is generous; alarm
  # if any skill balloons past it.
  local slug skill count
  while read -r slug; do
    [ -z "$slug" ] && continue
    skill="$SKILL_ROOT/$slug/SKILL.md"
    count=$(wc -l < "$skill" | tr -d ' ')
    [ "$count" -le 150 ] || {
      echo "skill $slug is $count lines — did rule content creep back in?" >&2
      return 1
    }
  done < <(wp03_refactored_skills)
}

@test "canon subagents directory does not contain elixir subagent" {
  local elixir_dir="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/elixir"
  [ ! -d "$elixir_dir" ] || {
    echo "elixir subagent directory still exists at $elixir_dir" >&2
    echo "WP03 requires it be fully deleted — content is now in rules/elixir/" >&2
    return 1
  }
}

@test "canon manifest does not list elixir subagent" {
  local manifest="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/.manifest/global-agents.json"
  assert_file_exists "$manifest"
  if grep -q '"name": "elixir"' "$manifest"; then
    echo "elixir still listed in $manifest" >&2
    echo "WP03 requires the entry be removed" >&2
    return 1
  fi
}
