#!/usr/bin/env bats
# Tests for the Swift rule pack (WP06).

load "../lib/test_helper.bash"

SWIFT_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/swift"

swift_rules() {
  cat <<'EOF'
code/guard-over-nested-if|IN-SW-CODE-001
code/optionals-over-sentinels|IN-SW-CODE-002
code/structured-concurrency|IN-SW-CODE-003
code/access-control-narrowest|IN-SW-CODE-004
code/codable-over-manual-json|IN-SW-CODE-005
test/xctassertequal-specific-values|IN-SW-TEST-001
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "swift pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$SWIFT_ROOT/$slug/RULE.md"
  done < <(swift_rules)
}

@test "swift pack has the expected total rule count" {
  local expected actual
  expected=$(swift_rules | grep -c '|' || true)
  actual=$(find "$SWIFT_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "swift rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "swift pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$SWIFT_ROOT/$slug/RULE.md" "id: $id"
  done < <(swift_rules)
}

@test "swift pack: each rule declares language: swift" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$SWIFT_ROOT/$slug/RULE.md" 'language: swift'
  done < <(swift_rules)
}

# ====================================================================
# Validator agreement
# ====================================================================

@test "swift pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$SWIFT_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(swift_rules)
}

@test "swift pack: rules list reports every swift id" {
  run run_intent claude rules list --lang swift
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(swift_rules)
}

# ====================================================================
# Textual-examples invariant
# ====================================================================

@test "swift pack: each rule has a fenced swift code block in Bad section" {
  local fence='```swift'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$SWIFT_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced swift block" >&2
      return 1
    }
  done < <(swift_rules)
}

@test "swift pack: each rule has a fenced swift code block in Good section" {
  local fence='```swift'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$SWIFT_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Good section missing fenced swift block" >&2
      return 1
    }
  done < <(swift_rules)
}

@test "swift pack: no sibling .swift files alongside RULE.md" {
  local stray
  stray=$(find "$SWIFT_ROOT" -name '*.swift' -type f)
  [ -z "$stray" ] || {
    echo "swift pack: unexpected .swift files present:" >&2
    echo "$stray" >&2
    return 1
  }
}
