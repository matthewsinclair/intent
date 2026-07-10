#!/usr/bin/env bats
# Test suite for `intent issues` -- lightweight directory-per-issue tracker (ST0055).
# Covers the AC/AT boundary in intent/st/ST0055/acceptance.md.

load ../lib/test_helper

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-issues-test-XXXXXX)"
  PROJECT_DIR="$(create_test_project "Issues Test" "$TEST_TEMP_DIR/proj")"
  cd "$PROJECT_DIR" || exit 1
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

# --- WP-01: Foundation & format -----------------------------------------------

@test "dispatch: intent issues routes to bin/intent_issues inside a project" {
  run run_intent issues help
  assert_success
  assert_output_contains "intent issues"
}

@test "dispatch: intent issues fails cleanly outside an Intent project" {
  cd "$TEST_TEMP_DIR" || exit 1
  run run_intent issues list
  assert_failure
  assert_output_contains "not in an Intent project"
}

@test "help: usage lists the five verbs; unknown verb exits non-zero" {
  run run_intent issues help
  assert_success
  assert_output_contains "list"
  assert_output_contains "add"
  assert_output_contains "show"
  assert_output_contains "close"
  assert_output_contains "open"

  run run_intent issues bogus
  assert_failure
  assert_output_contains "Unknown issues command"
}

# --- WP-02: Create & list -----------------------------------------------------

@test "add: allocates id, writes OPEN/NNNN/NNNN-slug.md, prints ID:Title" {
  run run_intent issues add "First issue"
  assert_success
  assert_output_contains "0001:First issue"
  assert_file_exists "intent/issues/OPEN/0001/0001-first-issue.md"
  assert_file_contains "intent/issues/OPEN/0001/0001-first-issue.md" "status: OPEN"
  assert_file_contains "intent/issues/OPEN/0001/0001-first-issue.md" "severity: medium"
}

@test "add: second add increments id, no collision" {
  run run_intent issues add "First"
  run run_intent issues add "Second"
  assert_success
  assert_output_contains "0002:Second"
  assert_file_exists "intent/issues/OPEN/0002/0002-second.md"
}

@test "add: --severity sets severity; invalid severity errors" {
  run run_intent issues add --severity high "High one"
  assert_success
  assert_file_contains "intent/issues/OPEN/0001/0001-high-one.md" "severity: high"

  run run_intent issues add --severity bogus "Bad sev"
  assert_failure
  assert_output_contains "Invalid severity"
}

@test "add: 'new' is an alias for 'add'" {
  run run_intent issues new "Via new alias"
  assert_success
  assert_output_contains "0001:Via new alias"
}

@test "add: pipe in a title is sanitized (no table-corrupting bar stored)" {
  run run_intent issues add "a | b | c"
  assert_success
  assert_file_exists "intent/issues/OPEN/0001/0001-a-b-c.md"
  run bash -c "grep -m1 '^title:' intent/issues/OPEN/0001/0001-a-b-c.md"
  refute_output_contains "|"
}

@test "add: requires a title" {
  run run_intent issues add
  assert_failure
  assert_output_contains "title is required"
}

@test "list: default lists OPEN issues" {
  run run_intent issues add "Open one"
  run run_intent issues list
  assert_success
  assert_output_contains "0001"
  assert_output_contains "Open one"

  run run_intent issues
  assert_success
  assert_output_contains "Open one"
}

@test "list: --kind filters; empty bucket is clean; invalid kind errors" {
  run run_intent issues add "Only open"

  run run_intent issues list --kind closed
  assert_success
  assert_output_contains "no closed issues"

  run run_intent issues list --kind all
  assert_success
  assert_output_contains "Only open"

  run run_intent issues list --kind bogus
  assert_failure
  assert_output_contains "Invalid --kind"
}

@test "add: lazily scaffolds issues dirs, writes no per-project _templ" {
  [ ! -d "intent/issues" ] || fail "issues dir should not pre-exist"
  run run_intent issues add "Scaffold me"
  assert_success
  assert_directory_exists "intent/issues/OPEN"
  assert_directory_exists "intent/issues/CLOSED"
  assert_file_exists "intent/issues/OPEN/.gitkeep"
  [ ! -d "intent/issues/_templ" ] || fail "no per-project _templ should be written (Intent owns the template)"
}

# --- WP-03: Inspect & lifecycle -----------------------------------------------

@test "show: prints frontmatter and body" {
  run run_intent issues add "Showable"
  run run_intent issues show 1
  assert_success
  assert_output_contains "# 0001: Showable"
  assert_output_contains "## Summary"
}

@test "show: accepts an unpadded id" {
  run run_intent issues add "Padless"
  run run_intent issues show 1
  assert_success
  assert_output_contains "0001"
}

@test "show: picks the frontmatter-bearing primary among multi-file issue dirs" {
  run run_intent issues add "Primary issue"
  # A frontmatter-less satellite that sorts BEFORE the primary alphabetically.
  cat > "intent/issues/OPEN/0001/0001-aaa-resolved.md" <<'EOF'
# 0001: resolution note (no frontmatter)
EOF
  run run_intent issues show 1 --json
  assert_success
  echo "$output" | jq -e '.title == "Primary issue"'
}

@test "show --json: emits valid JSON with expected fields" {
  run run_intent issues add --severity low "Jsonme"
  run run_intent issues show 1 --json
  assert_success
  echo "$output" | jq -e '.id == "0001"'
  echo "$output" | jq -e '.status == "OPEN"'
  echo "$output" | jq -e '.severity == "low"'
  echo "$output" | jq -e '.title == "Jsonme"'
}

@test "close: moves OPEN/NNNN dir to CLOSED and mirrors status" {
  run run_intent issues add "Closeme"
  run run_intent issues close 1
  assert_success
  assert_directory_exists "intent/issues/CLOSED/0001"
  [ ! -d "intent/issues/OPEN/0001" ] || fail "issue should have left OPEN"
  assert_file_contains "intent/issues/CLOSED/0001/0001-closeme.md" "status: CLOSED"
}

@test "open: moves CLOSED/NNNN dir back to OPEN and mirrors status" {
  run run_intent issues add "Reopenme"
  run run_intent issues close 1
  run run_intent issues open 1
  assert_success
  assert_directory_exists "intent/issues/OPEN/0001"
  [ ! -d "intent/issues/CLOSED/0001" ] || fail "issue should have left CLOSED"
  assert_file_contains "intent/issues/OPEN/0001/0001-reopenme.md" "status: OPEN"
}

@test "read: a legacy RESOLVED issue surfaces as CLOSED (normalised)" {
  mkdir -p "intent/issues/CLOSED/0009"
  cat > "intent/issues/CLOSED/0009/0009-legacy.md" <<'EOF'
---
id: "0009"
title: Legacy resolved
date: 2026-01-01
reporter: someone
status: RESOLVED
severity: high
---
# 0009: Legacy resolved
EOF

  run run_intent issues list --kind closed
  assert_success
  assert_output_contains "CLOSED"
  refute_output_contains "RESOLVED"

  run run_intent issues show 9 --json
  assert_success
  echo "$output" | jq -e '.status == "CLOSED"'
}

@test "error: unknown id on show/close/open exits non-zero" {
  run run_intent issues show 99
  assert_failure
  assert_output_contains "not found"

  run run_intent issues close 99
  assert_failure

  run run_intent issues open 99
  assert_failure
}
