#!/usr/bin/env bats
# Tests for the author rule pack (ST0052 WP02, refactored ST0053 WP01).
# AT-02.1 -- covers AC-02.1 (every author rule is schema-valid). Also backstops
# AC-02.2 (craft = judgment tier) via the category split.
#
# ST0053 WP01 lifted the shared mechanical style rules (banned filler, vanity
# metrics, heading hygiene, mechanical trope pass) out of `author` into the
# `IN-PR-*` prose base -- see rule_pack_prose.bats. The author pack now owns
# only its discipline-specific rules: one style rule (front-matter and
# objectives, book/course IA) plus the four craft rules.

load "../lib/test_helper.bash"

AUTHOR_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/author"

author_rules() {
  cat <<'EOF'
style/front-matter-and-objectives|IN-AU-STYLE-003
craft/voice-and-register-consistency|IN-AU-CRAFT-001
craft/continuity|IN-AU-CRAFT-002
craft/full-trope-diagnosis|IN-AU-CRAFT-003
craft/citation-and-attribution|IN-AU-CRAFT-004
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "author pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$AUTHOR_ROOT/$slug/RULE.md"
  done < <(author_rules)
}

@test "author pack has the expected total rule count" {
  local expected actual
  expected=$(author_rules | grep -c '|' || true)
  actual=$(find "$AUTHOR_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "author rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "author pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$AUTHOR_ROOT/$slug/RULE.md" "id: $id"
  done < <(author_rules)
}

@test "author pack: each rule declares language: author" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$AUTHOR_ROOT/$slug/RULE.md" 'language: author'
  done < <(author_rules)
}

# ====================================================================
# Validator agreement (AC-02.1)
# ====================================================================

@test "author pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$AUTHOR_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(author_rules)
}

@test "author pack: rules list reports every author id" {
  run run_intent claude rules list --lang author
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(author_rules)
}

# ====================================================================
# Tier split (AC-02.2): craft = judgment
# ====================================================================

@test "author pack: category matches the tier directory" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local cat="${slug%%/*}"
    assert_file_contains "$AUTHOR_ROOT/$slug/RULE.md" "category: $cat"
  done < <(author_rules)
}

@test "author pack: every craft (judgment) rule is severity recommendation" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    [ "${slug%%/*}" = "craft" ] || continue
    assert_file_contains "$AUTHOR_ROOT/$slug/RULE.md" 'severity: recommendation'
  done < <(author_rules)
}

# ====================================================================
# Textual-examples invariant (prose has no runtime -- CI-LIMITATIONS)
# ====================================================================

@test "author pack: each rule has a fenced markdown block in Bad section" {
  local fence='```markdown'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$AUTHOR_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced markdown block" >&2
      return 1
    }
  done < <(author_rules)
}

@test "author pack: each rule has a fenced markdown block in Good section" {
  local fence='```markdown'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$AUTHOR_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Good section missing fenced markdown block" >&2
      return 1
    }
  done < <(author_rules)
}

@test "author pack: no sibling .md example files alongside RULE.md" {
  local stray
  stray=$(find "$AUTHOR_ROOT" -name '*.md' -type f ! -name 'RULE.md')
  [ -z "$stray" ] || {
    echo "author pack: unexpected .md files present:" >&2
    echo "$stray" >&2
    return 1
  }
}

# ====================================================================
# Two-form detrope cross-pack link (ST0053 WP01): the author craft rule
# full-trope-diagnosis is the on-instruction companion to the prose base's
# mechanical trope pass (IN-PR-STYLE-004).
# ====================================================================

@test "author pack: full-trope-diagnosis links to the prose mechanical trope pass" {
  assert_file_contains \
    "$AUTHOR_ROOT/craft/full-trope-diagnosis/RULE.md" 'IN-PR-STYLE-004'
}
