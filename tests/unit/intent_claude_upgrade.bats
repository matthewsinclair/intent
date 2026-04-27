#!/usr/bin/env bats
# Tests for `intent claude upgrade --apply` (ST0035/WP-11).
#
# Five spec scenarios from intent/st/ST0035/WP/11/info.md (lines 76-82):
#   1. Fresh project --apply -> all canon artefacts installed
#   2. Re-run --apply -> zero changes (idempotence)
#   3. User-edited CLAUDE.md user-section -> preserved on refresh
#   4. Pre-existing non-Intent pre-commit hook -> chained, not overwritten
#   5. --dry-run -> no file modifications

load "../lib/test_helper.bash"

# Override setup() to additionally isolate HOME so installed subagents and
# skills on the host machine do not bleed into the upgrade probes (which
# would enqueue UPDATE_SUBAGENT / UPDATE_SKILL actions and bias snapshots).
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  export HOME="$TEST_TEMP_DIR/home"
  mkdir -p "$HOME"
  cd "$TEST_TEMP_DIR" || exit 1
}

teardown() {
  if [ -d "$TEST_TEMP_DIR" ]; then
    cd "$INTENT_PROJECT_ROOT" || exit 1
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# Create + initialise a fresh Intent project at $TEST_TEMP_DIR/<name>.
# Sets PROJ_DIR and cds there. `intent init` creates .git automatically.
init_scratch() {
  local name="${1:-scratch}"
  PROJ_DIR="$TEST_TEMP_DIR/$name"
  mkdir -p "$PROJ_DIR"
  cd "$PROJ_DIR"
  run_intent init >/dev/null 2>&1 || fail "intent init failed in $PROJ_DIR"
}

# Snapshot tree state (paths + sha1) for idempotence assertions. Excludes
# .git internals (mtime-noisy and irrelevant to canon state).
tree_snapshot() {
  ( cd "$1" && find . -type f \
      -not -path './.git/*' \
      -not -name '*.swp' \
      | sort \
      | xargs shasum 2>/dev/null )
}

@test "fresh project --apply installs all canon artefacts" {
  init_scratch fresh

  run run_intent claude upgrade --apply
  assert_success

  assert_file_exists "$PROJ_DIR/.claude/settings.json"
  assert_file_exists "$PROJ_DIR/.claude/scripts/session-context.sh"
  assert_file_exists "$PROJ_DIR/.claude/scripts/require-in-session.sh"
  assert_file_exists "$PROJ_DIR/.claude/scripts/post-tool-advisory.sh"
  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit"
  assert_file_exists "$PROJ_DIR/.intent_critic.yml"
  assert_file_exists "$PROJ_DIR/CLAUDE.md"
  assert_file_exists "$PROJ_DIR/usage-rules.md"
  assert_file_exists "$PROJ_DIR/intent/llm/MODULES.md"
  assert_file_exists "$PROJ_DIR/intent/llm/DECISION_TREE.md"

  [ -x "$PROJ_DIR/.claude/scripts/session-context.sh" ] || fail "session-context.sh not executable"
  [ -x "$PROJ_DIR/.git/hooks/pre-commit" ] || fail "pre-commit hook not executable"

  # The pre-commit carries our marker so the next probe identifies it.
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "intent critic gate"
  # CLAUDE.md carries the Intent footer marker (so refresh path activates).
  assert_file_contains "$PROJ_DIR/CLAUDE.md" "Generated from"
}

@test "re-running --apply produces no file changes (idempotence)" {
  init_scratch idem
  run_intent claude upgrade --apply >/dev/null 2>&1

  before="$(tree_snapshot "$PROJ_DIR")"

  run run_intent claude upgrade --apply
  assert_success

  after="$(tree_snapshot "$PROJ_DIR")"
  if [ "$before" != "$after" ]; then
    diff <(echo "$before") <(echo "$after")
    fail "tree changed on second --apply (not idempotent)"
  fi
}

@test "user-edited CLAUDE.md user-section is preserved on refresh" {
  init_scratch preserve
  run_intent claude upgrade --apply >/dev/null 2>&1

  # Inject custom directives inside the user section AND drift a non-user
  # heading so REFRESH_CLAUDE_MD is enqueued (otherwise nothing to do).
  awk '
    /<!-- user:end -->/ && !done {
      print "MY CUSTOM DIRECTIVE"
      print "Line two of custom content."
      done = 1
    }
    { print }
  ' "$PROJ_DIR/CLAUDE.md" > "$PROJ_DIR/CLAUDE.md.new"
  mv "$PROJ_DIR/CLAUDE.md.new" "$PROJ_DIR/CLAUDE.md"
  sed -i '' 's/## Project-specific$/## Project-specific (DRIFTED)/' "$PROJ_DIR/CLAUDE.md"

  run run_intent claude upgrade --apply
  assert_success

  assert_file_contains "$PROJ_DIR/CLAUDE.md" "MY CUSTOM DIRECTIVE"
  assert_file_contains "$PROJ_DIR/CLAUDE.md" "Line two of custom content."

  # Drifted heading reverted to canonical.
  if grep -qF "## Project-specific (DRIFTED)" "$PROJ_DIR/CLAUDE.md"; then
    fail "drifted heading should have been reverted by refresh"
  fi
}

@test "pre-existing non-Intent pre-commit hook is chained, not overwritten" {
  init_scratch chain

  rm -f "$PROJ_DIR/.git/hooks/pre-commit"
  printf '%s\n' '#!/bin/sh' '# user hook' 'echo running' 'exit 0' \
    > "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"
  user_sha="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"

  run run_intent claude upgrade --apply
  assert_success

  new_sha="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"
  [ "$user_sha" = "$new_sha" ] || fail "user pre-commit was modified (sha changed)"

  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit.intent"
  [ -x "$PROJ_DIR/.git/hooks/pre-commit.intent" ] || fail "chained hook not executable"

  assert_output_contains "Intent critic gate (ST0035 canon)"
  assert_output_contains "git rev-parse --git-path hooks"
}

@test "PROJECT_NAME resolves from config.json (not basename of relative path)" {
  init_scratch myproj

  # Force the relative-path codepath: cd into the project, invoke with --project-dir "."
  cd "$PROJ_DIR" || fail "cannot cd"
  run run_intent claude upgrade --apply --project-dir .
  assert_success

  # CLAUDE.md title must be the canonical project name from config.json,
  # not "." (which is what basename "." returns).
  local first_line
  first_line=$(head -1 "$PROJ_DIR/CLAUDE.md")
  if [ "$first_line" = "# ." ]; then
    fail "CLAUDE.md title is '# .' (PROJECT_NAME basename bug returned)"
  fi
  [ "$first_line" = "# myproj" ] || fail "expected '# myproj', got '$first_line'"
}

@test "canon-installer always installs _default RULES/ARCHITECTURE (not language-specific)" {
  init_scratch any

  run run_intent claude upgrade --apply
  assert_success

  # _default template markers present.
  assert_file_contains "$PROJ_DIR/intent/llm/RULES.md" "intent/plugins/claude/rules/<lang>/"
  assert_file_contains "$PROJ_DIR/intent/llm/ARCHITECTURE.md" "System architecture and design decisions"

  # Elixir-template markers absent (would indicate accidental fallback to elixir/).
  if grep -qF "Core Elixir Rules" "$PROJ_DIR/intent/llm/RULES.md"; then
    fail "intent/llm/RULES.md got Elixir template (canon-installer must use _default)"
  fi
  if grep -qF "Phoenix/Ash web application" "$PROJ_DIR/intent/llm/ARCHITECTURE.md"; then
    fail "intent/llm/ARCHITECTURE.md got Elixir template (canon-installer must use _default)"
  fi
}

@test "canon-installer ignores language markers (multi-language reality)" {
  init_scratch polyglot
  # Stage markers for multiple languages -- canon-installer must still use
  # _default and not pick any single one as "primary".
  touch "$PROJ_DIR/mix.exs" "$PROJ_DIR/Cargo.toml" "$PROJ_DIR/Package.swift"

  run run_intent claude upgrade --apply
  assert_success

  # Still _default -- no Elixir/Rust/Swift template selection.
  if grep -qF "Core Elixir Rules" "$PROJ_DIR/intent/llm/RULES.md"; then
    fail "polyglot project picked Elixir template (canon-installer must always use _default)"
  fi
  assert_file_contains "$PROJ_DIR/intent/llm/RULES.md" "intent/plugins/claude/rules/<lang>/"
}

@test "--dry-run does not modify the filesystem" {
  init_scratch dry

  before="$(tree_snapshot "$PROJ_DIR")"

  run run_intent claude upgrade
  assert_success

  after="$(tree_snapshot "$PROJ_DIR")"
  if [ "$before" != "$after" ]; then
    diff <(echo "$before") <(echo "$after")
    fail "tree changed on dry-run"
  fi

  assert_file_not_exists "$PROJ_DIR/.claude/settings.json"
  assert_file_not_exists "$PROJ_DIR/.intent_critic.yml"
  assert_file_not_exists "$PROJ_DIR/usage-rules.md"
}
