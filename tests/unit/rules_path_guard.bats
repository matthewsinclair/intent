#!/usr/bin/env bats
# Mechanical guard against rules-path drift (ST0042 T2).
#
# Three releases in a row (v2.11.11 + two prior partial fixes) chased the
# same defect class: propagated or generated artefacts telling agents the
# rule library lives at a local `intent/plugins/claude/rules/` path. That
# directory exists only inside the Intent tool itself; in a consuming
# project rules are reachable solely through `intent claude rules`.
# These tests grep every propagated/generated surface so the class cannot
# regress a fourth time.

load "../lib/test_helper.bash"

# Surfaces that leave the Intent repo: skills + subagents (installed into
# ~/.claude), canon templates (instantiated into consumer projects), agent
# templates, and the AGENTS.md generator's heredocs.
PROPAGATED_SURFACES=(
  "intent/plugins/claude/skills"
  "intent/plugins/claude/subagents"
  "intent/plugins/agents/templates"
  "intent/plugins/agents/bin/intent_agents"
  "lib/templates"
)

@test "no propagated or generated artefact references the local rules path" {
  local hits=""
  local surface
  for surface in "${PROPAGATED_SURFACES[@]}"; do
    hits="$hits$(grep -rn 'intent/plugins/claude/rules' "${INTENT_HOME}/${surface}" 2>/dev/null || true)"
  done
  [ -z "$hits" ] || fail "local rules path in propagated artefact(s): $hits"
}

@test "canon LLM templates carry only placeholders the installer substitutes" {
  # canon_substitute_placeholders handles PROJECT_NAME, AUTHOR, DATE,
  # INTENT_VERSION, INTENT_HOME. Anything else ([[LANG]] was the v2.11.x
  # escapee) ships verbatim into consumers' files.
  run grep -rnoE '\[\[[A-Z_]+\]\]' "${INTENT_HOME}/lib/templates/llm"
  local unknown
  unknown=$(printf '%s\n' "$output" | grep -vE '\[\[(PROJECT_NAME|AUTHOR|DATE|INTENT_VERSION|INTENT_HOME)\]\]' || true)
  [ -z "$unknown" ] || fail "unsubstitutable placeholder(s) in canon templates: $unknown"
}
