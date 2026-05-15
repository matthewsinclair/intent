#!/usr/bin/env bats
# Tests for the Lua rule pack (WP06).

load "../lib/test_helper.bash"

LUA_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/lua"

lua_rules() {
  cat <<'EOF'
code/local-over-global|IN-LU-CODE-001
code/tables-as-structs|IN-LU-CODE-002
code/metatables-sparingly|IN-LU-CODE-003
code/pcall-for-errors|IN-LU-CODE-004
code/module-return-pattern|IN-LU-CODE-005
code/dispatch-table-over-if-chain|IN-LU-CODE-006
test/busted-describe-it-structure|IN-LU-TEST-001
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "lua pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$LUA_ROOT/$slug/RULE.md"
  done < <(lua_rules)
}

@test "lua pack has the expected total rule count" {
  local expected actual
  expected=$(lua_rules | grep -c '|' || true)
  actual=$(find "$LUA_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "lua rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "lua pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$LUA_ROOT/$slug/RULE.md" "id: $id"
  done < <(lua_rules)
}

@test "lua pack: each rule declares language: lua" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$LUA_ROOT/$slug/RULE.md" 'language: lua'
  done < <(lua_rules)
}

# ====================================================================
# Validator agreement
# ====================================================================

@test "lua pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$LUA_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(lua_rules)
}

@test "lua pack: rules list reports every lua id" {
  run run_intent claude rules list --lang lua
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(lua_rules)
}

# ====================================================================
# Textual-examples invariant
# ====================================================================

@test "lua pack: each rule has a fenced lua code block in Bad section" {
  local fence='```lua'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$LUA_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced lua block" >&2
      return 1
    }
  done < <(lua_rules)
}

@test "lua pack: each rule has a fenced lua code block in Good section" {
  local fence='```lua'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$LUA_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Good section missing fenced lua block" >&2
      return 1
    }
  done < <(lua_rules)
}

@test "lua pack: no sibling .lua files alongside RULE.md" {
  local stray
  stray=$(find "$LUA_ROOT" -name '*.lua' -type f)
  [ -z "$stray" ] || {
    echo "lua pack: unexpected .lua files present:" >&2
    echo "$stray" >&2
    return 1
  }
}
