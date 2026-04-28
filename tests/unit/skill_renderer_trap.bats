#!/usr/bin/env bats
# Regression guard for the skill-renderer token-stripping bug.
#
# Claude Code's skill renderer silently strips positional/special-param
# tokens ($0-$9, $@, $#, $*, $?, $!, ${N}) when SKILL.md files are
# injected into the model's prompt. An inline `awk '{print $1}'` becomes
# `awk '{print }'` and the downstream pipeline produces malformed output.
#
# This test scans every canon SKILL.md and fails if any of those tokens
# appear. Skill authors should move bash logic that uses positional
# fields into a real script file under <skill_dir>/scripts/ and invoke
# it by path -- the script lives on disk and the renderer never sees it.
#
# Discovered: 2026-04-28 via the v2.10.1 /in-session gate-firing bug.
# Memory note: feedback_skill_renderer_strips_dollar_n.md

load "../lib/test_helper.bash"

SKILLS_DIR="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills"
TRAP_PATTERN='\$([0-9]|@|#|\*|\?|!|\{[0-9@#*]\})'

@test "skills directory exists" {
  [ -d "$SKILLS_DIR" ]
}

@test "no canon SKILL.md inlines positional/special-param tokens" {
  run grep -rnE "$TRAP_PATTERN" --include='SKILL.md' "$SKILLS_DIR"
  if [ "$status" -eq 0 ]; then
    echo "Found renderer-trap tokens in canon SKILL.md files:"
    echo "$output"
    echo ""
    echo "These tokens are silently stripped by Claude Code's skill renderer"
    echo "when SKILL.md is injected into the prompt. Move bash logic that"
    echo "uses positional fields into a script file under scripts/ and"
    echo "invoke it by path."
    return 1
  fi
}

@test "in-session ships its release-gate.sh helper" {
  local helper="${SKILLS_DIR}/in-session/scripts/release-gate.sh"
  [ -f "$helper" ]
  [ -x "$helper" ]
}
