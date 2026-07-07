#!/usr/bin/env bats
# Tests for the IN-PR-* prose base pack (ST0053 WP01). The shared mechanical
# prose-hygiene surface, lifted out of the author pack so both `author` and
# `content` depend on one copy (Highlander -- no divergent banned-filler /
# trope-pass / heading-hygiene between disciplines).
#
# Covers: every prose rule is schema-valid, is enumerated under --lang prose,
# is mechanical (style tier), carries its IN-AU migration alias, and (D5) the
# mechanical trope pass still cites the single in-detrope catalogue rather than
# a vendored indicator set.

load "../lib/test_helper.bash"

PROSE_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/prose"

# slug|id|old-alias
prose_rules() {
  cat <<'EOF'
style/banned-filler-and-house-style|IN-PR-STYLE-001|IN-AU-STYLE-001
style/no-vanity-metrics|IN-PR-STYLE-002|IN-AU-STYLE-002
style/heading-hygiene|IN-PR-STYLE-003|IN-AU-STYLE-004
style/mechanical-trope-pass|IN-PR-STYLE-004|IN-AU-STYLE-005
EOF
}

# ====================================================================
# Presence + count
# ====================================================================

@test "prose pack: every catalogued rule exists" {
  while IFS='|' read -r slug _ _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$PROSE_ROOT/$slug/RULE.md"
  done < <(prose_rules)
}

@test "prose pack has the expected total rule count" {
  local expected actual
  expected=$(prose_rules | grep -c '|' || true)
  actual=$(find "$PROSE_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "prose rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID + language + migration alias
# ====================================================================

@test "prose pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$PROSE_ROOT/$slug/RULE.md" "id: $id"
  done < <(prose_rules)
}

@test "prose pack: each rule declares language: prose" {
  while IFS='|' read -r slug _ _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$PROSE_ROOT/$slug/RULE.md" 'language: prose'
  done < <(prose_rules)
}

@test "prose pack: each rule carries its IN-AU migration alias" {
  while IFS='|' read -r slug _ old; do
    [ -z "$slug" ] && continue
    assert_file_contains "$PROSE_ROOT/$slug/RULE.md" "$old"
  done < <(prose_rules)
}

# ====================================================================
# Validator agreement + enumeration
# ====================================================================

@test "prose pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _ _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$PROSE_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(prose_rules)
}

@test "prose pack: rules list reports every prose id" {
  run run_intent claude rules list --lang prose
  assert_success
  while IFS='|' read -r _ id _; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(prose_rules)
}

# ====================================================================
# Mechanical tier: every prose base rule is style (greppable)
# ====================================================================

@test "prose pack: every rule is the mechanical style tier" {
  while IFS='|' read -r slug _ _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$PROSE_ROOT/$slug/RULE.md" 'category: style'
  done < <(prose_rules)
}

# ====================================================================
# Textual-examples invariant (prose has no runtime -- CI-LIMITATIONS)
# ====================================================================

@test "prose pack: each rule has a fenced markdown block in Bad section" {
  local fence='```markdown'
  while IFS='|' read -r slug _ _; do
    [ -z "$slug" ] && continue
    local rule="$PROSE_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced markdown block" >&2
      return 1
    }
  done < <(prose_rules)
}

@test "prose pack: no sibling .md example files alongside RULE.md" {
  local stray
  stray=$(find "$PROSE_ROOT" -name '*.md' -type f ! -name 'RULE.md')
  [ -z "$stray" ] || {
    echo "prose pack: unexpected .md files present:" >&2
    echo "$stray" >&2
    return 1
  }
}

# ====================================================================
# D5: the mechanical trope pass references the single catalogue
# (AC-02.3, moved here with the rule), not a vendored indicator set
# ====================================================================

@test "prose pack: mechanical trope pass cites the in-detrope catalogue" {
  assert_file_contains \
    "$PROSE_ROOT/style/mechanical-trope-pass/RULE.md" \
    'in-detrope/data/trope-catalog.md'
}

@test "prose pack: no vendored trope-indicator file (Highlander)" {
  local stray
  stray=$(find "$PROSE_ROOT" -iname '*trope-indicator*' -o -iname '*indicators*')
  [ -z "$stray" ] || {
    echo "prose pack: unexpected vendored indicator file(s):" >&2
    echo "$stray" >&2
    return 1
  }
}
