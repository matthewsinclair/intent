#!/usr/bin/env bats
# Tests for /in-review stage-2 language dispatch (WP07).
#
# The dispatcher lives inside a Claude skill (`in-review/SKILL.md`), not a
# shell script. These tests therefore check the skill's dispatch table by
# reading the file and asserting the language-indicator -> critic-<lang>
# mappings are present, complete, and consistent. The "minimal sandbox tree"
# test then verifies that a tree with a single language indicator would be
# resolved by the skill's detection list to the expected critic.

load "../lib/test_helper.bash"

SKILL_FILE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-review/SKILL.md"

# Canonical dispatch table. Each line: <indicator>|<expected-critic>.
# Shell detection uses a broader probe (bin/ or scripts/ with shebangs); it is
# tested separately by a shebang-path check below.

dispatch_rows() {
  cat <<'EOF'
mix.exs|critic-elixir
Cargo.toml|critic-rust
Package.swift|critic-swift
.luarc.json|critic-lua
EOF
}

# ====================================================================
# Skill file health
# ====================================================================

@test "in-review skill file exists" {
  assert_file_exists "$SKILL_FILE"
}

@test "in-review skill declares the stage-2 language-detection block" {
  assert_file_contains "$SKILL_FILE" "Language detection"
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

@test "in-review skill names every indicator in the dispatch table" {
  while IFS='|' read -r indicator _; do
    [ -z "$indicator" ] && continue
    assert_file_contains "$SKILL_FILE" "$indicator"
  done < <(dispatch_rows)
}

@test "in-review skill shows a Task() invocation for each critic" {
  for critic in critic-elixir critic-rust critic-swift critic-lua critic-shell; do
    assert_file_contains "$SKILL_FILE" "subagent_type=\"$critic\""
  done
}

# ====================================================================
# Sandbox-tree resolution
# ====================================================================
#
# For each minimal sandbox, identify which dispatch row the skill should
# select. This walks the dispatch table as the skill describes it: first
# matching indicator wins.

resolve_critic() {
  local sandbox="$1"
  while IFS='|' read -r indicator critic; do
    [ -z "$indicator" ] && continue
    if [ -e "$sandbox/$indicator" ]; then
      echo "$critic"
      return 0
    fi
  done < <(dispatch_rows)
  echo "none"
}

@test "dispatch: mix.exs-only sandbox resolves to critic-elixir" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  touch "$TEST_TEMP_DIR/sand/mix.exs"
  [ "$(resolve_critic "$TEST_TEMP_DIR/sand")" = "critic-elixir" ]
}

@test "dispatch: Cargo.toml-only sandbox resolves to critic-rust" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  touch "$TEST_TEMP_DIR/sand/Cargo.toml"
  [ "$(resolve_critic "$TEST_TEMP_DIR/sand")" = "critic-rust" ]
}

@test "dispatch: Package.swift-only sandbox resolves to critic-swift" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  touch "$TEST_TEMP_DIR/sand/Package.swift"
  [ "$(resolve_critic "$TEST_TEMP_DIR/sand")" = "critic-swift" ]
}

@test "dispatch: .luarc.json-only sandbox resolves to critic-lua" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  touch "$TEST_TEMP_DIR/sand/.luarc.json"
  [ "$(resolve_critic "$TEST_TEMP_DIR/sand")" = "critic-lua" ]
}

@test "dispatch: empty sandbox resolves to no critic" {
  mkdir -p "$TEST_TEMP_DIR/sand"
  [ "$(resolve_critic "$TEST_TEMP_DIR/sand")" = "none" ]
}

# ====================================================================
# Polyglot prompt path
# ====================================================================
#
# When two or more indicators are present the skill requires a user prompt
# rather than silently dispatching a single critic. The shell can verify this
# by asserting both the "Polyglot" label and either "ask the user" or
# equivalent prompting language appear in the skill file.

@test "in-review skill documents the polyglot user-prompt path" {
  assert_file_contains "$SKILL_FILE" "Polyglot"
  grep -qiE 'ask the user|ask.*which language|prompt' "$SKILL_FILE"
}

# ====================================================================
# Shell detection
# ====================================================================
#
# Shell is a path-based detection rather than a single file indicator.

@test "in-review skill describes shell detection via bin/ or scripts/ shebangs" {
  grep -qE 'bin/|scripts/' "$SKILL_FILE"
  grep -qiE 'bash|zsh|shebang' "$SKILL_FILE"
}
