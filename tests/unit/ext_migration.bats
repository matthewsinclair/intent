#!/usr/bin/env bats
# Tests for the v2.8.2 -> v2.9.0 migration step.
#
# Covers:
#   - Version stamp on .intent/config.json
#   - ~/.intent/ext/ bootstrap with README
#   - worker-bee seed copy (and skip when present)
#   - Prune of ~/.claude/agents/elixir.md and worker-bee.md
#   - installed-agents.json manifest cleanup
#   - Idempotent re-run
#   - needs_v2_9_0_upgrade predicate
#   - generate_ext_readme content
#   - Static check that bin/intent_upgrade chains through the new step

load "../lib/test_helper.bash"

# Source the helpers so migration functions are callable directly.
# INTENT_HOME is exported by test_helper.bash and is needed by the seed-copy
# step inside migrate_v2_8_2_to_v2_9_0.
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  FAKE_HOME="$TEST_TEMP_DIR/home"
  PROJ_DIR="$TEST_TEMP_DIR/proj"
  mkdir -p "$FAKE_HOME" "$PROJ_DIR/.intent"
  cd "$PROJ_DIR" || exit 1
  source "${INTENT_BIN_DIR}/intent_helpers"
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  if [ -d "$TEST_TEMP_DIR" ]; then
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# Stamp a project at the given intent_version.
stamp_intent_version() {
  local version=$1
  cat > "$PROJ_DIR/.intent/config.json" <<EOF
{
  "intent_version": "$version",
  "project_name": "fake"
}
EOF
}

# ====================================================================
# needs_v2_9_0_upgrade predicate
# ====================================================================

@test "needs_v2_9_0_upgrade returns 0 for 2.8.2" {
  needs_v2_9_0_upgrade "2.8.2"
}

@test "needs_v2_9_0_upgrade returns 0 for 2.0.0" {
  needs_v2_9_0_upgrade "2.0.0"
}

@test "needs_v2_9_0_upgrade returns 0 for 2.8.1" {
  needs_v2_9_0_upgrade "2.8.1"
}

@test "needs_v2_9_0_upgrade returns 1 for 2.9.0" {
  ! needs_v2_9_0_upgrade "2.9.0"
}

@test "needs_v2_9_0_upgrade returns 1 for 2.9.5" {
  ! needs_v2_9_0_upgrade "2.9.5"
}

@test "needs_v2_9_0_upgrade returns 1 for 2.10.0" {
  ! needs_v2_9_0_upgrade "2.10.0"
}

@test "needs_v2_9_0_upgrade returns 1 for 3.0.0" {
  ! needs_v2_9_0_upgrade "3.0.0"
}

# ====================================================================
# generate_ext_readme
# ====================================================================

@test "generate_ext_readme writes a readable README" {
  local target="$TEST_TEMP_DIR/README.md"
  generate_ext_readme "$target"
  [ -f "$target" ]
  grep -q "Intent User Extensions" "$target"
  grep -q "intent ext list" "$target"
  grep -q "intent ext new" "$target"
  grep -q "worker-bee" "$target"
}

# ====================================================================
# migrate_v2_8_2_to_v2_9_0: version stamp
# ====================================================================

@test "migration stamps intent_version to 2.9.0" {
  HOME="$FAKE_HOME" stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  local stamped
  stamped=$(jq -r .intent_version "$PROJ_DIR/.intent/config.json")
  [ "$stamped" = "2.9.0" ]
}

@test "migration drops legacy .version key during stamp" {
  HOME="$FAKE_HOME" cat > "$PROJ_DIR/.intent/config.json" <<'EOF'
{
  "intent_version": "2.8.2",
  "version": "stale-string-from-1.x",
  "project_name": "fake"
}
EOF
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  local has_legacy
  has_legacy=$(jq 'has("version")' "$PROJ_DIR/.intent/config.json")
  [ "$has_legacy" = "false" ]
}

@test "migration tolerates absent .intent/config.json" {
  rm -rf "$PROJ_DIR/.intent"
  HOME="$FAKE_HOME" run migrate_v2_8_2_to_v2_9_0 .
  [ "$status" -eq 0 ]
}

# ====================================================================
# migrate_v2_8_2_to_v2_9_0: ~/.intent/ext/ bootstrap
# ====================================================================

@test "migration creates ~/.intent/ext/ when absent" {
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ -d "$FAKE_HOME/.intent/ext" ]
  [ -f "$FAKE_HOME/.intent/ext/README.md" ]
  grep -q "Intent User Extensions" "$FAKE_HOME/.intent/ext/README.md"
}

@test "migration leaves existing ~/.intent/ext/ alone" {
  mkdir -p "$FAKE_HOME/.intent/ext"
  echo "user content" > "$FAKE_HOME/.intent/ext/README.md"
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ "$(cat "$FAKE_HOME/.intent/ext/README.md")" = "user content" ]
}

# ====================================================================
# migrate_v2_8_2_to_v2_9_0: worker-bee seed
# ====================================================================

@test "migration seeds worker-bee from canon ext-seeds when absent" {
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ -d "$FAKE_HOME/.intent/ext/worker-bee" ]
  [ -f "$FAKE_HOME/.intent/ext/worker-bee/extension.json" ]
  [ -f "$FAKE_HOME/.intent/ext/worker-bee/subagents/worker-bee/agent.md" ]
}

@test "migration does not overwrite existing worker-bee ext" {
  mkdir -p "$FAKE_HOME/.intent/ext/worker-bee"
  echo "custom" > "$FAKE_HOME/.intent/ext/worker-bee/marker.txt"
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ -f "$FAKE_HOME/.intent/ext/worker-bee/marker.txt" ]
  [ "$(cat "$FAKE_HOME/.intent/ext/worker-bee/marker.txt")" = "custom" ]
}

# ====================================================================
# migrate_v2_8_2_to_v2_9_0: prune installed elixir + worker-bee
# ====================================================================

@test "migration prunes installed elixir agent file" {
  mkdir -p "$FAKE_HOME/.claude/agents"
  echo "stub" > "$FAKE_HOME/.claude/agents/elixir.md"
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ ! -f "$FAKE_HOME/.claude/agents/elixir.md" ]
}

@test "migration prunes installed worker-bee agent file" {
  mkdir -p "$FAKE_HOME/.claude/agents"
  echo "stub" > "$FAKE_HOME/.claude/agents/worker-bee.md"
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ ! -f "$FAKE_HOME/.claude/agents/worker-bee.md" ]
}

@test "migration tolerates missing ~/.claude/agents/ entirely" {
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" run migrate_v2_8_2_to_v2_9_0 .
  [ "$status" -eq 0 ]
}

@test "migration leaves other installed agents untouched" {
  mkdir -p "$FAKE_HOME/.claude/agents"
  echo "stub-elixir" > "$FAKE_HOME/.claude/agents/elixir.md"
  echo "stub-diogenes" > "$FAKE_HOME/.claude/agents/diogenes.md"
  echo "stub-socrates" > "$FAKE_HOME/.claude/agents/socrates.md"
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  [ ! -f "$FAKE_HOME/.claude/agents/elixir.md" ]
  [ -f "$FAKE_HOME/.claude/agents/diogenes.md" ]
  [ -f "$FAKE_HOME/.claude/agents/socrates.md" ]
}

# ====================================================================
# migrate_v2_8_2_to_v2_9_0: installed-agents.json manifest cleanup
# ====================================================================

@test "migration removes elixir entry from installed-agents.json" {
  mkdir -p "$FAKE_HOME/.intent/agents"
  cat > "$FAKE_HOME/.intent/agents/installed-agents.json" <<'EOF'
{
  "installed": [
    {"name": "elixir", "version": "1.0.0"},
    {"name": "diogenes", "version": "1.0.0"}
  ]
}
EOF
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  local has_elixir
  has_elixir=$(jq '[.installed[].name] | contains(["elixir"])' "$FAKE_HOME/.intent/agents/installed-agents.json")
  [ "$has_elixir" = "false" ]
}

@test "migration removes worker-bee entry from installed-agents.json" {
  mkdir -p "$FAKE_HOME/.intent/agents"
  cat > "$FAKE_HOME/.intent/agents/installed-agents.json" <<'EOF'
{
  "installed": [
    {"name": "worker-bee", "version": "1.0.0"},
    {"name": "diogenes", "version": "1.0.0"}
  ]
}
EOF
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  local has_wb
  has_wb=$(jq '[.installed[].name] | contains(["worker-bee"])' "$FAKE_HOME/.intent/agents/installed-agents.json")
  [ "$has_wb" = "false" ]
}

@test "migration preserves non-removed entries in installed-agents.json" {
  mkdir -p "$FAKE_HOME/.intent/agents"
  cat > "$FAKE_HOME/.intent/agents/installed-agents.json" <<'EOF'
{
  "installed": [
    {"name": "elixir", "version": "1.0.0"},
    {"name": "diogenes", "version": "1.0.0"},
    {"name": "socrates", "version": "1.0.0"}
  ]
}
EOF
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  local kept
  kept=$(jq -r '.installed[].name' "$FAKE_HOME/.intent/agents/installed-agents.json" | sort | tr '\n' ' ')
  [ "$kept" = "diogenes socrates " ]
}

@test "migration result is valid JSON" {
  mkdir -p "$FAKE_HOME/.intent/agents"
  cat > "$FAKE_HOME/.intent/agents/installed-agents.json" <<'EOF'
{"installed": [{"name": "elixir"}]}
EOF
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .
  jq -e . "$FAKE_HOME/.intent/agents/installed-agents.json" >/dev/null
}

# ====================================================================
# Idempotency
# ====================================================================

@test "migration is idempotent (second run is a no-op)" {
  stamp_intent_version "2.8.2"
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 .

  local first_stamp first_readme
  first_stamp=$(jq -r .intent_version "$PROJ_DIR/.intent/config.json")
  first_readme=$(cat "$FAKE_HOME/.intent/ext/README.md")

  HOME="$FAKE_HOME" run migrate_v2_8_2_to_v2_9_0 .
  [ "$status" -eq 0 ]

  local second_stamp second_readme
  second_stamp=$(jq -r .intent_version "$PROJ_DIR/.intent/config.json")
  second_readme=$(cat "$FAKE_HOME/.intent/ext/README.md")

  [ "$first_stamp" = "$second_stamp" ]
  [ "$first_readme" = "$second_readme" ]
  [ "$first_stamp" = "2.9.0" ]
}

# ====================================================================
# Chain coverage: v2.8.2 -> v2.9.0 via the previous step
# ====================================================================

@test "chain from v2.8.1 lands at 2.9.0 after both migrations" {
  stamp_intent_version "2.8.1"
  HOME="$FAKE_HOME" migrate_v2_8_1_to_v2_8_2 . > /dev/null
  HOME="$FAKE_HOME" migrate_v2_8_2_to_v2_9_0 . > /dev/null
  local stamped
  stamped=$(jq -r .intent_version "$PROJ_DIR/.intent/config.json")
  [ "$stamped" = "2.9.0" ]
}

# ====================================================================
# Static gate: bin/intent_upgrade chains through the new step
# ====================================================================

@test "bin/intent_upgrade gate check includes needs_v2_9_0_upgrade" {
  grep -q '! needs_v2_9_0_upgrade "$VERSION"' "${INTENT_BIN_DIR}/intent_upgrade"
}

@test "bin/intent_upgrade has a case for 2.8.2" {
  grep -E -q '^[[:space:]]*"2\.8\.2"\)' "${INTENT_BIN_DIR}/intent_upgrade"
}

@test "every chain in bin/intent_upgrade ends with migrate_v2_8_2_to_v2_9_0" {
  # Every prior chain should now terminate with the v2.9.0 call. There are 16
  # chains total (15 case-arms in the main case + 1 in the pre-v2 chain block).
  local count_v_8_2 count_v_9_0
  count_v_8_2=$(grep -c '^[[:space:]]*migrate_v2_8_1_to_v2_8_2 \.' "${INTENT_BIN_DIR}/intent_upgrade")
  count_v_9_0=$(grep -c '^[[:space:]]*migrate_v2_8_2_to_v2_9_0 \.' "${INTENT_BIN_DIR}/intent_upgrade")
  # Each previous-step call must be followed by a v2.9.0 call. The v2.9.0
  # count is exactly +1 because the new "2.8.2" case arm calls v2.9.0
  # without a preceding v2.8.1->v2.8.2 step.
  [ "$count_v_9_0" -eq "$((count_v_8_2 + 1))" ]
}
