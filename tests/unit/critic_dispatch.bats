#!/usr/bin/env bats
# Tests for /in-review stage-2 language dispatch.
#
# v2.11.0+ (ST0037): the dispatcher reads the project's `languages` array
# from `intent/.config/config.json` instead of probing for filesystem
# markers. These tests verify the skill describes that flow, names every
# supported critic, and that a sandbox with a seeded `languages` field
# resolves to the matching critic.

load "../lib/test_helper.bash"

SKILL_FILE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-review/SKILL.md"

# Canonical dispatch table. Each line: <language-config-value>|<expected-critic>.

dispatch_rows() {
  cat <<'EOF'
elixir|critic-elixir
rust|critic-rust
swift|critic-swift
lua|critic-lua
shell|critic-shell
EOF
}

# ====================================================================
# Skill file health
# ====================================================================

@test "in-review skill file exists" {
  assert_file_exists "$SKILL_FILE"
}

@test "in-review skill declares the stage-2 language-dispatch block" {
  assert_file_contains "$SKILL_FILE" "Language dispatch"
}

@test "in-review skill reads languages from intent/.config/config.json" {
  assert_file_contains "$SKILL_FILE" "intent/.config/config.json"
  assert_file_contains "$SKILL_FILE" "languages"
}

# ====================================================================
# Dispatch table completeness
# ====================================================================

@test "in-review skill names every critic in the dispatch table" {
  while IFS='|' read -r _ critic; do
    [ -z "$critic" ] && continue
    assert_file_contains "$SKILL_FILE" "$critic"
  done < <(dispatch_rows)
}

@test "in-review skill names every language in the dispatch table" {
  while IFS='|' read -r lang _; do
    [ -z "$lang" ] && continue
    assert_file_contains "$SKILL_FILE" "\`$lang\`"
  done < <(dispatch_rows)
}

@test "in-review skill shows a Task() invocation for each critic" {
  for critic in critic-elixir critic-rust critic-swift critic-lua critic-shell; do
    assert_file_contains "$SKILL_FILE" "subagent_type=\"$critic\""
  done
}

# ====================================================================
# Config-driven sandbox resolution
# ====================================================================
#
# For each minimal sandbox with a seeded languages array, the skill's
# documented behaviour is to dispatch to the matching critic. These tests
# model that behaviour by reading the config field and asserting the
# expected mapping holds.

resolve_critic_from_config() {
  local sandbox="$1"
  local config="$sandbox/intent/.config/config.json"
  [ -f "$config" ] || { echo "none"; return 0; }
  command -v jq >/dev/null 2>&1 || { echo "none"; return 0; }
  local lang
  lang=$(jq -r '(.languages // []) | first // ""' "$config")
  [ -z "$lang" ] && { echo "none"; return 0; }
  while IFS='|' read -r row_lang row_critic; do
    [ -z "$row_lang" ] && continue
    if [ "$row_lang" = "$lang" ]; then
      echo "$row_critic"
      return 0
    fi
  done < <(dispatch_rows)
  echo "none"
}

setup_config_sandbox() {
  local sandbox="$1"
  local langs_json="$2"
  mkdir -p "$sandbox/intent/.config"
  printf '{"languages":%s}\n' "$langs_json" > "$sandbox/intent/.config/config.json"
}

@test "dispatch: languages=[elixir] resolves to critic-elixir" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["elixir"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-elixir" ]
}

@test "dispatch: languages=[rust] resolves to critic-rust" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["rust"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-rust" ]
}

@test "dispatch: languages=[swift] resolves to critic-swift" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["swift"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-swift" ]
}

@test "dispatch: languages=[lua] resolves to critic-lua" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["lua"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-lua" ]
}

@test "dispatch: languages=[shell] resolves to critic-shell" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["shell"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-shell" ]
}

@test "dispatch: empty languages resolves to no critic" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '[]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "none" ]
}

@test "dispatch: missing config resolves to no critic" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "none" ]
}

# ====================================================================
# Polyglot path
# ====================================================================
#
# When the languages array has more than one entry, the user has explicitly
# declared a polyglot. The skill dispatches one critic per language with
# narrowed target globs.

@test "in-review skill documents the polyglot dispatch path" {
  assert_file_contains "$SKILL_FILE" "Polyglot"
  grep -qiE 'multiple|each critic|narrowed' "$SKILL_FILE"
}

@test "dispatch: languages=[elixir,rust] reports first as primary" {
  setup_config_sandbox "$TEST_TEMP_DIR/sand" '["elixir","rust"]'
  [ "$(resolve_critic_from_config "$TEST_TEMP_DIR/sand")" = "critic-elixir" ]
}

# ====================================================================
# Regression guard: no filesystem probes
# ====================================================================
#
# v2.10.x had a probe table here that was a regression against design
# intent. ST0037 replaced it with the config read above. If anyone
# re-introduces filesystem-marker detection in the dispatch block, this
# test catches it.

@test "in-review skill does not document filesystem-probe-based detection" {
  run grep -E '(mix\.exs|Cargo\.toml|Package\.swift|\.luarc\.json) (exists|→|->)' "$SKILL_FILE"
  [ "$status" -ne 0 ]
}
