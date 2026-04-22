#!/usr/bin/env bats
# Tests for the Rust rule pack (WP06).
#
# Guards three invariants:
#   1. Presence — every rule catalogued here exists at the expected path.
#   2. ID assignment — each rule declares its canonical IN-RS-* id.
#   3. Validator agreement — each rule passes `intent claude rules validate`.
#   4. Textual-examples invariant — Rust rules have fenced `rust` blocks in
#      ## Bad / ## Good, not sibling `.rs` files (per CI-LIMITATIONS.md).

load "../lib/test_helper.bash"

RUST_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/rust"

rust_rules() {
  cat <<'EOF'
code/result-over-panic|IN-RS-CODE-001
code/ownership-before-clone|IN-RS-CODE-002
code/traits-over-enums-for-behaviour|IN-RS-CODE-003
code/error-types-thiserror-anyhow|IN-RS-CODE-004
code/lifetime-elision-first|IN-RS-CODE-005
test/cfg-test-colocated|IN-RS-TEST-001
test/assert-matches-for-variants|IN-RS-TEST-002
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "rust pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$RUST_ROOT/$slug/RULE.md"
  done < <(rust_rules)
}

@test "rust pack has the expected total rule count" {
  local expected actual
  expected=$(rust_rules | grep -c '|' || true)
  actual=$(find "$RUST_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "rust rule count drift: expected $expected (test catalog), found $actual (filesystem)" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "rust pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$RUST_ROOT/$slug/RULE.md" "id: $id"
  done < <(rust_rules)
}

@test "rust pack: each rule declares language: rust" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$RUST_ROOT/$slug/RULE.md" 'language: rust'
  done < <(rust_rules)
}

# ====================================================================
# Validator agreement
# ====================================================================

@test "rust pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$RUST_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(rust_rules)
}

@test "rust pack: rules list reports every rust id" {
  run run_intent claude rules list --lang rust
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(rust_rules)
}

# ====================================================================
# Textual-examples invariant (CI-LIMITATIONS.md)
# ====================================================================

@test "rust pack: each rule has a fenced rust code block in Bad section" {
  local fence='```rust'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$RUST_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced rust block" >&2
      return 1
    }
  done < <(rust_rules)
}

@test "rust pack: each rule has a fenced rust code block in Good section" {
  local fence='```rust'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$RUST_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Good section missing fenced rust block" >&2
      return 1
    }
  done < <(rust_rules)
}

@test "rust pack: no sibling .rs files alongside RULE.md (textual-only invariant)" {
  local stray
  stray=$(find "$RUST_ROOT" -name '*.rs' -type f)
  [ -z "$stray" ] || {
    echo "rust pack: unexpected .rs files present (textual-only per CI-LIMITATIONS.md):" >&2
    echo "$stray" >&2
    return 1
  }
}
