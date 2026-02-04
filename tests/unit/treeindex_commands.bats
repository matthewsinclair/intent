#!/usr/bin/env bats
# Test suite for intent_treeindex command

load ../lib/test_helper

# --- Helper: create test project with shadow dir ---
create_treeindex_project() {
  local project_dir=$(create_test_project "Treeindex Test")
  mkdir -p "$project_dir/intent/.treeindex"
  echo "$project_dir"
}

# --- Helper: set up mock Claude ---
setup_claude_mock() {
  mkdir -p "$TEST_TEMP_DIR/mock_bin"
  cat > "$TEST_TEMP_DIR/mock_bin/claude" << 'MOCK'
#!/bin/bash
# Mock Claude that outputs a valid .treeindex body
input=$(cat)
dirname_arg=$(echo "$input" | grep -o 'for [^/]*/') || true
dirname_arg="${dirname_arg#for }"
: "${dirname_arg:=test_dir/}"

cat << EOF
# ${dirname_arg}

Mock summary for testing purposes. This directory contains test files.

## Files

- \`test.ex\` -- Test file for mock purposes
EOF
MOCK
  chmod +x "$TEST_TEMP_DIR/mock_bin/claude"
  export PATH="$TEST_TEMP_DIR/mock_bin:$PATH"
}

# ============================================================
# Help/Usage Tests
# ============================================================

@test "treeindex: shows help with -h flag" {
  run "${INTENT_BIN_DIR}/intent_treeindex" -h
  assert_failure
  assert_output_contains "Usage: intent treeindex"
  assert_output_contains "Generate .treeindex directory summaries using Claude AI"
}

@test "treeindex: shows help with --help flag" {
  run "${INTENT_BIN_DIR}/intent_treeindex" --help
  assert_failure
  assert_output_contains "Usage: intent treeindex"
}

@test "treeindex: shows usage when no DIR given" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  run "${INTENT_BIN_DIR}/intent_treeindex"
  assert_failure
  assert_output_contains "Usage: intent treeindex"
}

@test "treeindex: errors when not in Intent project" {
  cd "$TEST_TEMP_DIR"
  mkdir -p somedir

  run "${INTENT_BIN_DIR}/intent_treeindex" somedir
  assert_failure
  assert_output_contains "Not in an Intent project"
}

@test "treeindex: rejects project root as DIR" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  run "${INTENT_BIN_DIR}/intent_treeindex" . --dry-run
  assert_failure
  assert_output_contains "subdirectory of the project root"
}

# ============================================================
# Shadow Path Mapping Tests
# ============================================================

@test "treeindex: shadow path maps lib/my_app to intent/.treeindex/lib/my_app" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app
  echo "defmodule MyApp do end" > lib/my_app/app.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --depth 0
  assert_success

  # Verify the shadow file is at the CORRECT path
  assert_file_exists "$project_dir/intent/.treeindex/lib/my_app/.treeindex"

  # Verify no junk paths were created (no absolute path leakage)
  [ ! -d "$project_dir/intent/.treeindex/Users" ] || fail "Absolute path leaked into shadow tree"
  [ ! -d "$project_dir/intent/.treeindex/tmp" ] || fail "Absolute path leaked into shadow tree"
}

@test "treeindex: shadow path for nested dir is correct" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app/accounts
  echo "defmodule Accounts do end" > lib/my_app/accounts/accounts.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app/accounts --depth 0
  assert_success

  assert_file_exists "$project_dir/intent/.treeindex/lib/my_app/accounts/.treeindex"
}

@test "treeindex: creates shadow directories on demand" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app
  echo "defmodule MyApp do end" > lib/my_app/app.ex

  # No shadow dirs exist yet
  [ ! -d "$project_dir/intent/.treeindex/lib" ] || fail "Shadow dir should not exist before generation"

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --depth 0
  assert_success

  assert_file_exists "$project_dir/intent/.treeindex/lib/my_app/.treeindex"
}

# ============================================================
# Fingerprint Tests
# ============================================================

@test "treeindex: fingerprint detects missing shadow as stale" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/test_dir
  echo "hello" > lib/test_dir/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/test_dir --check --depth 0
  assert_success
  assert_output_contains "stale"
}

@test "treeindex: fingerprint changes when file added" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/fp_test
  echo "hello" > lib/fp_test/file1.ex

  setup_claude_mock

  # Generate
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_test --depth 0
  assert_success
  assert_output_contains "-- generated"

  # Should be up to date
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_test --check --depth 0
  assert_success
  assert_output_contains "up to date"

  # Add a file
  echo "world" > lib/fp_test/file2.ex

  # Should now be stale
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_test --check --depth 0
  assert_success
  assert_output_contains "stale"
}

@test "treeindex: fingerprint changes when file size changes" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/fp_size_test
  echo "small" > lib/fp_size_test/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_size_test --depth 0
  assert_success

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_size_test --check --depth 0
  assert_success
  assert_output_contains "up to date"

  # Change file size
  echo "this is a much larger content now with more bytes" > lib/fp_size_test/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/fp_size_test --check --depth 0
  assert_success
  assert_output_contains "stale"
}

@test "treeindex: fingerprint ignores .DS_Store" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/ds_test
  echo "hello" > lib/ds_test/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/ds_test --depth 0
  assert_success

  # Add .DS_Store
  echo "garbage" > lib/ds_test/.DS_Store

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/ds_test --check --depth 0
  assert_success
  assert_output_contains "up to date"
}

@test "treeindex: fingerprint includes subdirectory names" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/subdir_test
  echo "hello" > lib/subdir_test/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/subdir_test --depth 0
  assert_success

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/subdir_test --check --depth 0
  assert_success
  assert_output_contains "up to date"

  # Add a subdirectory
  mkdir lib/subdir_test/new_child

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/subdir_test --check --depth 0
  assert_success
  assert_output_contains "stale"
}

# ============================================================
# --check Mode Tests
# ============================================================

@test "treeindex: --check reports stale without generating" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/check_test
  echo "hello" > lib/check_test/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/check_test --check --depth 0
  assert_success
  assert_output_contains "stale"

  # Shadow file should NOT exist
  assert_file_not_exists "$project_dir/intent/.treeindex/lib/check_test/.treeindex"
}

@test "treeindex: --check reports up-to-date" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/check_uptodate
  echo "hello" > lib/check_uptodate/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/check_uptodate --depth 0
  assert_success

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/check_uptodate --check --depth 0
  assert_success
  assert_output_contains "up to date"
}

@test "treeindex: --check does not require claude on PATH" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/no_claude_test
  echo "hello" > lib/no_claude_test/file1.ex

  run env PATH="/usr/bin:/bin" "${INTENT_BIN_DIR}/intent_treeindex" lib/no_claude_test --check --depth 0
  assert_success
  assert_output_contains "stale"
}

# ============================================================
# --dry-run Mode Tests
# ============================================================

@test "treeindex: --dry-run shows plan without writing" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/dryrun_test
  echo "hello" > lib/dryrun_test/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/dryrun_test --dry-run --depth 0
  assert_success
  assert_output_contains "would generate"

  assert_file_not_exists "$project_dir/intent/.treeindex/lib/dryrun_test/.treeindex"
}

@test "treeindex: --dry-run does not create shadow directories" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/dryrun_nodir
  echo "hello" > lib/dryrun_nodir/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/dryrun_nodir --dry-run --depth 0
  assert_success

  [ ! -d "$project_dir/intent/.treeindex/lib/dryrun_nodir" ] || fail "Shadow directory was created during --dry-run"
}

# ============================================================
# Directory Walking Tests
# ============================================================

@test "treeindex: collects directories at default depth 2" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app/accounts/nested
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/accounts/user.ex
  echo "hello" > lib/my_app/accounts/nested/deep.ex

  # default depth is 2: lib/my_app + children + grandchildren
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run
  assert_success
  assert_output_contains "lib/my_app/accounts/nested"
  assert_output_contains "lib/my_app/accounts"
  assert_output_contains "lib/my_app"
}

@test "treeindex: respects --depth 0 (target only)" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app/accounts
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/accounts/user.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run --depth 0
  assert_success
  assert_output_contains "lib/my_app"
  refute_output_contains "lib/my_app/accounts"
}

@test "treeindex: respects --depth 1" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app/accounts/nested
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/accounts/user.ex
  echo "hello" > lib/my_app/accounts/nested/deep.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run --depth 1
  assert_success
  assert_output_contains "lib/my_app/accounts"
  assert_output_contains "lib/my_app"
  refute_output_contains "nested"
}

@test "treeindex: skips ignored directories" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app
  mkdir -p lib/my_app/_build
  mkdir -p lib/my_app/node_modules
  mkdir -p lib/my_app/.git
  mkdir -p lib/my_app/src
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/src/real.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run
  assert_success
  assert_output_contains "lib/my_app/src"
  refute_output_contains "_build"
  refute_output_contains "node_modules"
}

@test "treeindex: sorts directories bottom-up (deepest first)" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/my_app/accounts
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/accounts/user.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run --depth 1
  assert_success

  # accounts (deeper) should appear before my_app in the output
  local accounts_line=$(echo "$output" | grep -n "accounts" | head -1 | cut -d: -f1)
  local my_app_line=$(echo "$output" | grep -n "lib/my_app --" | head -1 | cut -d: -f1)

  [ "$accounts_line" -lt "$my_app_line" ] || fail "Directories not sorted bottom-up: accounts=$accounts_line, my_app=$my_app_line"
}

# ============================================================
# Generation Tests (with mock Claude)
# ============================================================

@test "treeindex: generates .treeindex at correct shadow path" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/gen_test
  echo "defmodule GenTest do end" > lib/gen_test/gen.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/gen_test --depth 0
  assert_success
  assert_output_contains "-- generated"

  # Verify shadow file at correct path
  assert_file_exists "$project_dir/intent/.treeindex/lib/gen_test/.treeindex"

  # Verify content is not empty
  local content=$(cat "$project_dir/intent/.treeindex/lib/gen_test/.treeindex")
  [ -n "$content" ] || fail "Generated .treeindex is empty"
}

@test "treeindex: prepends fingerprint header" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/header_test
  echo "defmodule HeaderTest do end" > lib/header_test/header.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/header_test --depth 0
  assert_success

  local shadow_file="$project_dir/intent/.treeindex/lib/header_test/.treeindex"
  assert_file_exists "$shadow_file"

  local header
  header="$(head -1 "$shadow_file")"
  [[ "$header" =~ ^"<!-- treeindex v1 fingerprint:"[a-f0-9]{8}" generated:".*" -->"$ ]] || fail "Header format incorrect: $header"
}

@test "treeindex: skips up-to-date directories" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/skip_test
  echo "hello" > lib/skip_test/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/skip_test --depth 0
  assert_success
  assert_output_contains "-- generated"

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/skip_test --depth 0
  assert_success
  assert_output_contains "up to date"
  refute_output_contains "-- generated"
}

@test "treeindex: --force regenerates even when up-to-date" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/force_test
  echo "hello" > lib/force_test/file1.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/force_test --depth 0
  assert_success
  assert_output_contains "-- generated"

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/force_test --force --depth 0
  assert_success
  assert_output_contains "-- generated"
}

@test "treeindex: skips directories with no source files" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/empty_dir

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/empty_dir --dry-run --depth 0
  assert_success
  assert_output_contains "empty, skipping"
}

@test "treeindex: reports progress to stderr" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/progress_test/sub1
  echo "hello" > lib/progress_test/app.ex
  echo "hello" > lib/progress_test/sub1/mod.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/progress_test --depth 1
  assert_success
  assert_output_contains "[1/"
  assert_output_contains "[2/"
  assert_output_contains "total"
}

@test "treeindex: multi-level generation creates correct shadow tree" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  mkdir -p lib/app/accounts
  mkdir -p lib/app/tokens
  echo "defmodule App do end" > lib/app/app.ex
  echo "defmodule Accounts do end" > lib/app/accounts/accounts.ex
  echo "defmodule Tokens do end" > lib/app/tokens/tokens.ex

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/app --depth 1
  assert_success

  # All three shadow files should exist at correct paths
  assert_file_exists "$project_dir/intent/.treeindex/lib/app/.treeindex"
  assert_file_exists "$project_dir/intent/.treeindex/lib/app/accounts/.treeindex"
  assert_file_exists "$project_dir/intent/.treeindex/lib/app/tokens/.treeindex"

  # No garbage in the shadow tree
  [ ! -d "$project_dir/intent/.treeindex/Users" ] || fail "Absolute path leaked"
  [ ! -d "$project_dir/intent/.treeindex/tmp" ] || fail "Temp path leaked"
}

# ============================================================
# .treeindexignore Tests
# ============================================================

@test "treeindex: auto-creates .treeindexignore on first run" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  # Remove .treeindexignore if it exists
  rm -f "$project_dir/intent/.treeindex/.treeindexignore"

  mkdir -p lib/auto_ignore
  echo "hello" > lib/auto_ignore/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/auto_ignore --dry-run --depth 0
  assert_success

  assert_file_exists "$project_dir/intent/.treeindex/.treeindexignore"
}

@test "treeindex: .treeindexignore contains expected default patterns" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  rm -f "$project_dir/intent/.treeindex/.treeindexignore"

  mkdir -p lib/default_patterns
  echo "hello" > lib/default_patterns/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/default_patterns --dry-run --depth 0
  assert_success

  local ignore_file="$project_dir/intent/.treeindex/.treeindexignore"
  assert_file_exists "$ignore_file"

  # Check key default patterns are present
  grep -q "_build/" "$ignore_file" || fail "Missing _build/ pattern"
  grep -q "node_modules/" "$ignore_file" || fail "Missing node_modules/ pattern"
  grep -q ".git/" "$ignore_file" || fail "Missing .git/ pattern"
  grep -q ".DS_Store" "$ignore_file" || fail "Missing .DS_Store pattern"
  grep -q '*.beam' "$ignore_file" || fail "Missing *.beam pattern"
}

@test "treeindex: does not overwrite existing .treeindexignore" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  # Write custom ignore file
  echo "custom_pattern/" > "$project_dir/intent/.treeindex/.treeindexignore"

  mkdir -p lib/no_overwrite
  echo "hello" > lib/no_overwrite/file1.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/no_overwrite --dry-run --depth 0
  assert_success

  local content
  content="$(cat "$project_dir/intent/.treeindex/.treeindexignore")"
  [ "$content" = "custom_pattern/" ] || fail "Ignore file was overwritten: $content"
}

@test "treeindex: respects custom directory ignore patterns" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  # Write ignore file with custom_skip/ pattern
  cat > "$project_dir/intent/.treeindex/.treeindexignore" << 'EOF'
custom_skip/
EOF

  mkdir -p lib/my_app/custom_skip
  mkdir -p lib/my_app/keep_this
  echo "hello" > lib/my_app/app.ex
  echo "hello" > lib/my_app/custom_skip/hidden.ex
  echo "hello" > lib/my_app/keep_this/visible.ex

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/my_app --dry-run --depth 1
  assert_success
  assert_output_contains "keep_this"
  refute_output_contains "custom_skip"
}

@test "treeindex: respects custom file ignore patterns" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  # Ignore .log files
  cat > "$project_dir/intent/.treeindex/.treeindexignore" << 'EOF'
*.log
EOF

  mkdir -p lib/file_ignore
  echo "real code" > lib/file_ignore/app.ex
  echo "log data" > lib/file_ignore/debug.log

  setup_claude_mock

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/file_ignore --depth 0
  assert_success
  assert_output_contains "-- generated"

  # Fingerprint should not change when only a .log file is added
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/file_ignore --check --depth 0
  assert_success
  assert_output_contains "up to date"

  echo "more logs" >> lib/file_ignore/debug.log

  run "${INTENT_BIN_DIR}/intent_treeindex" lib/file_ignore --check --depth 0
  assert_success
  assert_output_contains "up to date"
}

@test "treeindex: empty .treeindexignore means no exclusions" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  # Empty ignore file
  echo "" > "$project_dir/intent/.treeindex/.treeindexignore"

  mkdir -p lib/no_exclude/_build
  echo "hello" > lib/no_exclude/app.ex
  echo "hello" > lib/no_exclude/_build/compiled.beam

  # _build should NOT be pruned since ignore file is empty
  run "${INTENT_BIN_DIR}/intent_treeindex" lib/no_exclude --dry-run --depth 1
  assert_success
  assert_output_contains "_build"
}

# ============================================================
# Argument Validation Tests
# ============================================================

@test "treeindex: errors on non-existent directory" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  run "${INTENT_BIN_DIR}/intent_treeindex" nonexistent_dir
  assert_failure
  assert_output_contains "Directory does not exist"
}

@test "treeindex: errors on directory outside project root" {
  local project_dir=$(create_treeindex_project)
  cd "$project_dir"

  run "${INTENT_BIN_DIR}/intent_treeindex" /tmp
  assert_failure
  assert_output_contains "must be under project root"
}
