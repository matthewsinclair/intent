#!/usr/bin/env bats
# Tests for the author rule pack (ST0052 WP02). AT-02.1 -- covers AC-02.1
# (every author rule is schema-valid). Also backstops the non-test ACs:
# AC-02.2 (style = mechanical tier, craft = judgment tier) via the category
# split, and AC-02.3 (the mechanical trope pass references the single
# trope-catalogue home, not a vendored indicator set).

load "../lib/test_helper.bash"

AUTHOR_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/author"

author_rules() {
  cat <<'EOF'
style/banned-filler-and-house-style|IN-AU-STYLE-001
style/no-vanity-metrics|IN-AU-STYLE-002
style/front-matter-and-objectives|IN-AU-STYLE-003
style/heading-hygiene|IN-AU-STYLE-004
style/mechanical-trope-pass|IN-AU-STYLE-005
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
# Two-tier split (AC-02.2): style = mechanical, craft = judgment
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
# D5: mechanical trope pass references the single catalogue (AC-02.3),
# not a vendored indicator set
# ====================================================================

@test "author pack: mechanical trope pass cites the in-detrope catalogue" {
  assert_file_contains \
    "$AUTHOR_ROOT/style/mechanical-trope-pass/RULE.md" \
    'in-detrope/data/trope-catalog.md'
}

@test "author pack: no vendored trope-indicator file (Highlander)" {
  local stray
  stray=$(find "$AUTHOR_ROOT" -iname '*trope-indicator*' -o -iname '*indicators*')
  [ -z "$stray" ] || {
    echo "author pack: unexpected vendored indicator file(s):" >&2
    echo "$stray" >&2
    return 1
  }
}

@test "author pack: full-trope-diagnosis and mechanical-trope-pass cross-link" {
  assert_file_contains \
    "$AUTHOR_ROOT/style/mechanical-trope-pass/RULE.md" 'IN-AU-CRAFT-003'
  assert_file_contains \
    "$AUTHOR_ROOT/craft/full-trope-diagnosis/RULE.md" 'IN-AU-STYLE-005'
}
