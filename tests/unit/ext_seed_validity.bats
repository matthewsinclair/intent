#!/usr/bin/env bats
# Tests that the `lib/templates/ext-seeds/worker-bee/` seed is a valid
# Intent extension and can be consumed by `intent ext validate` + the
# migration that will land in WP09.
#
# The seed is the source-of-truth copy that gets copied to
# `~/.intent/ext/worker-bee/` on first upgrade to v2.9.0.

load "../lib/test_helper.bash"

SEED_ROOT="${INTENT_PROJECT_ROOT}/lib/templates/ext-seeds"
WB_SEED="${SEED_ROOT}/worker-bee"

# ====================================================================
# Directory layout
# ====================================================================

@test "worker-bee seed directory exists" {
  [ -d "$WB_SEED" ]
}

@test "worker-bee seed has extension.json manifest" {
  [ -f "$WB_SEED/extension.json" ]
}

@test "worker-bee seed has README.md" {
  [ -f "$WB_SEED/README.md" ]
}

@test "worker-bee seed has agent.md at declared contribution path" {
  [ -f "$WB_SEED/subagents/worker-bee/agent.md" ]
}

@test "worker-bee seed has metadata.json at declared contribution path" {
  [ -f "$WB_SEED/subagents/worker-bee/metadata.json" ]
}

@test "worker-bee seed preserves all original resource files" {
  local count
  count=$(find "$WB_SEED/subagents/worker-bee" -type f | wc -l | tr -d ' ')
  # 17 files moved from canon: agent.md, metadata.json, + 15 resources.
  [ "$count" -eq 17 ]
}

# ====================================================================
# Manifest well-formedness
# ====================================================================

@test "worker-bee manifest is valid JSON" {
  jq -e . "$WB_SEED/extension.json" >/dev/null
}

@test "worker-bee manifest declares schema intent-extension/v1" {
  local schema
  schema=$(jq -r '.schema' "$WB_SEED/extension.json")
  [ "$schema" = "intent-extension/v1" ]
}

@test "worker-bee manifest name matches directory" {
  local name
  name=$(jq -r '.name' "$WB_SEED/extension.json")
  [ "$name" = "worker-bee" ]
}

@test "worker-bee manifest version is semver" {
  local version
  version=$(jq -r '.version' "$WB_SEED/extension.json")
  echo "$version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9.-]+)?$'
}

@test "worker-bee manifest contributes a single subagent" {
  local count
  count=$(jq '.contributes.subagents | length' "$WB_SEED/extension.json")
  [ "$count" -eq 1 ]
}

@test "worker-bee manifest declares no skills or rules" {
  local skills rules
  skills=$(jq '.contributes.skills | length' "$WB_SEED/extension.json")
  rules=$(jq '.contributes.rules | length' "$WB_SEED/extension.json")
  [ "$skills" -eq 0 ]
  [ "$rules" -eq 0 ]
}

# ====================================================================
# Self-containment audit
# ====================================================================

@test "worker-bee seed has no absolute path references" {
  ! grep -rq '/Users/' "$WB_SEED"
}

@test "worker-bee seed has no \$INTENT_HOME references" {
  ! grep -rq '\$INTENT_HOME' "$WB_SEED"
}

@test "worker-bee seed has no canon subagent path references" {
  ! grep -rq 'intent/plugins/claude/subagents/worker-bee' "$WB_SEED"
}

# ====================================================================
# End-to-end validate
# ====================================================================

@test "intent ext validate passes for worker-bee seed" {
  INTENT_EXT_DIR="$SEED_ROOT" run run_intent ext validate worker-bee
  [ "$status" -eq 0 ]
  assert_output_contains "ok"
}

# ====================================================================
# Canon is clean
# ====================================================================

@test "canon subagents directory no longer contains worker-bee" {
  [ ! -d "${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/worker-bee" ]
}

@test "canon global-agents.json does not list worker-bee" {
  local manifest="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/.manifest/global-agents.json"
  ! jq -r '.agents[].name' "$manifest" | grep -qx "worker-bee"
}
