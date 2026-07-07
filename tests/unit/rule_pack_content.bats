#!/usr/bin/env bats
# Tests for the content rule pack (ST0053 WP02). AT-02.2 -- covers AC-02.2
# (the content pack's craft tier owns the web-distinct rules; style tier is
# mechanical; each rule is schema-valid and enumerated under --lang content).
# Also backstops AC-02.3 (Highlander): no content rule duplicates a shared
# IN-PR-* mechanical rule -- the base is depended on, not copied.

load "../lib/test_helper.bash"

CONTENT_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/content"

content_rules() {
  cat <<'EOF'
style/page-meta-present|IN-CO-STYLE-001
style/image-alt-text|IN-CO-STYLE-002
style/descriptive-link-text|IN-CO-STYLE-003
craft/scannability|IN-CO-CRAFT-001
craft/primary-cta|IN-CO-CRAFT-002
craft/reading-level|IN-CO-CRAFT-003
EOF
}

# ====================================================================
# Presence + count
# ====================================================================

@test "content pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$CONTENT_ROOT/$slug/RULE.md"
  done < <(content_rules)
}

@test "content pack has the expected total rule count" {
  local expected actual
  expected=$(content_rules | grep -c '|' || true)
  actual=$(find "$CONTENT_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "content rule count drift: expected $expected, found $actual" >&2
    return 1
  }
}

# ====================================================================
# ID + language
# ====================================================================

@test "content pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$CONTENT_ROOT/$slug/RULE.md" "id: $id"
  done < <(content_rules)
}

@test "content pack: each rule declares language: content" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$CONTENT_ROOT/$slug/RULE.md" 'language: content'
  done < <(content_rules)
}

# ====================================================================
# Validator agreement + enumeration (AC-02.2)
# ====================================================================

@test "content pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$CONTENT_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(content_rules)
}

@test "content pack: rules list reports every content id" {
  run run_intent claude rules list --lang content
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(content_rules)
}

# ====================================================================
# Two-tier split (AC-02.2): category matches the tier directory
# ====================================================================

@test "content pack: category matches the tier directory" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local cat="${slug%%/*}"
    assert_file_contains "$CONTENT_ROOT/$slug/RULE.md" "category: $cat"
  done < <(content_rules)
}

@test "content pack: the craft tier owns the web-distinct judgment rules" {
  # The D4 content-distinct rules that are NOT shared prose hygiene.
  assert_file_exists "$CONTENT_ROOT/craft/scannability/RULE.md"
  assert_file_exists "$CONTENT_ROOT/craft/primary-cta/RULE.md"
  assert_file_exists "$CONTENT_ROOT/craft/reading-level/RULE.md"
}

# ====================================================================
# Textual-examples invariant (prose has no runtime -- CI-LIMITATIONS)
# ====================================================================

@test "content pack: each rule has a fenced markdown block in Bad section" {
  local fence='```markdown'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$CONTENT_ROOT/$slug/RULE.md"
    awk '/^## Bad$/,/^## Good$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Bad section missing fenced markdown block" >&2
      return 1
    }
  done < <(content_rules)
}

@test "content pack: each rule has a fenced markdown block in Good section" {
  local fence='```markdown'
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    local rule="$CONTENT_ROOT/$slug/RULE.md"
    awk '/^## Good$/,/^## When This Applies$/' "$rule" | grep -qxF "$fence" || {
      echo "$rule: Good section missing fenced markdown block" >&2
      return 1
    }
  done < <(content_rules)
}

@test "content pack: no sibling .md example files alongside RULE.md" {
  local stray
  stray=$(find "$CONTENT_ROOT" -name '*.md' -type f ! -name 'RULE.md')
  [ -z "$stray" ] || {
    echo "content pack: unexpected .md files present:" >&2
    echo "$stray" >&2
    return 1
  }
}

# ====================================================================
# AC-02.3 Highlander: the content pack depends on the IN-PR base, it does
# not copy the shared mechanical rules.
# ====================================================================

@test "content pack: does not duplicate any prose-base mechanical rule slug" {
  local dup
  for slug in banned-filler-and-house-style no-vanity-metrics heading-hygiene mechanical-trope-pass; do
    dup=$(find "$CONTENT_ROOT" -type d -name "$slug")
    [ -z "$dup" ] || {
      echo "content pack duplicates prose-base rule '$slug': $dup" >&2
      return 1
    }
  done
}

@test "content pack: declares no IN-PR-* id (the base is referenced, not re-owned)" {
  local hit
  hit=$(grep -rl '^id: IN-PR-' "$CONTENT_ROOT" 2>/dev/null || true)
  [ -z "$hit" ] || {
    echo "content pack re-declares an IN-PR id: $hit" >&2
    return 1
  }
}
