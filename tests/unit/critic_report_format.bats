#!/usr/bin/env bats
# Tests for the Critic report format (WP07).
#
# The report format is specified in intent/docs/critics.md and emitted by
# each critic-<lang> subagent. These tests assert the format is parse-stable:
# a known-shape report can be mechanically mined for rule IDs, severities,
# counts, and the Rules-applied footer. If a critic drifts from the format,
# this test fails before downstream tools that rely on the format do.
#
# The tests work against a synthetic fixture report assembled inline, not a
# live critic run. Format specification is the contract; this file guards it.

load "../lib/test_helper.bash"

make_report() {
  cat <<'EOF'
## Critic Report: critic-elixir code lib/accounts.ex

CRITICAL
- IN-AG-HIGHLANDER-001 (highlander) lib/accounts.ex:16
  Two modules define `valid_email?/1` with different regexes.
  Extract one canonical EmailAddress module.

- IN-EX-CODE-005 (no-silent-failures) lib/accounts.ex:21
  `case ... _ -> nil end` swallows every error.
  Return tagged tuples and match `{:error, reason}` explicitly.

WARNING
- IN-EX-CODE-001 (pattern-match-over-conditionals) lib/accounts.ex:5
  Nested `if` on struct fields.
  Replace with multi-clause function heads.

RECOMMENDATION
- (test-spec-missing) test/accounts_test.exs:1
  No adjacent spec file.
  Run `Task(subagent_type="diogenes", prompt="specify test/accounts_test.exs")`.

Summary: 2 critical, 1 warning, 1 recommendation, 0 style.
Rules applied: 4 agnostic, 6 language-specific.
EOF
}

# ====================================================================
# Heading
# ====================================================================

@test "report format: heading matches critic-report pattern" {
  make_report | head -1 | grep -qE '^## Critic Report: critic-[a-z]+ (code|test|test-check) '
}

# ====================================================================
# Severity sections
# ====================================================================

@test "report format: severity labels are the four canonical uppercase words" {
  local sections
  sections=$(make_report | grep -E '^(CRITICAL|WARNING|RECOMMENDATION|STYLE)$' | sort -u)
  [ "$sections" = "$(printf 'CRITICAL\nRECOMMENDATION\nWARNING')" ]
}

@test "report format: sections appear in decreasing-severity order" {
  local order
  order=$(make_report | grep -nE '^(CRITICAL|WARNING|RECOMMENDATION|STYLE)$' | awk -F: '{print $2}')
  local expected
  expected="$(printf 'CRITICAL\nWARNING\nRECOMMENDATION')"
  [ "$order" = "$expected" ]
}

# ====================================================================
# Finding format: `- <id> (<slug>) <file>:<line>`
# ====================================================================

@test "report format: every rule-id finding has the (slug) suffix" {
  local ids_with_slug ids_total
  ids_with_slug=$(make_report | grep -cE '^- IN-[A-Z]{2,3}-[A-Z]+-[0-9]+ \([a-z0-9-]+\) ')
  ids_total=$(make_report | grep -cE '^- IN-[A-Z]{2,3}-[A-Z]+-[0-9]+')
  [ "$ids_with_slug" -eq "$ids_total" ]
  [ "$ids_total" -gt 0 ]
}

@test "report format: every id-bearing finding carries a file:line suffix" {
  make_report | grep -E '^- IN-' | grep -qvE ':[0-9]+$' && {
    echo "at least one finding line lacks a trailing :LINE"
    make_report | grep -E '^- IN-'
    return 1
  } || true
  local all matched
  all=$(make_report | grep -cE '^- IN-')
  matched=$(make_report | grep -cE '^- IN-[A-Z]{2,3}-[A-Z]+-[0-9]+ \([a-z0-9-]+\) .+:[0-9]+$')
  [ "$all" -eq "$matched" ]
}

@test "report format: non-rule findings use parenthesised slug and file:line" {
  # e.g. recommendation handoffs that do not cite a canonical rule id.
  make_report | grep -E '^- \([a-z0-9-]+\) .+:[0-9]+$' | head -1 | grep -qE '^- \([a-z0-9-]+\)'
}

# ====================================================================
# Summary and Rules-applied lines
# ====================================================================

@test "report format: Summary line counts all four severities in order" {
  make_report | grep -qE '^Summary: [0-9]+ critical, [0-9]+ warning, [0-9]+ recommendation, [0-9]+ style\.$'
}

@test "report format: Rules-applied line splits agnostic / language-specific" {
  make_report | grep -qE '^Rules applied: [0-9]+ agnostic, [0-9]+ language-specific\.$'
}

@test "report format: Summary precedes Rules applied" {
  local summary_line rules_line
  summary_line=$(make_report | grep -nE '^Summary:' | head -1 | awk -F: '{print $1}')
  rules_line=$(make_report | grep -nE '^Rules applied:' | head -1 | awk -F: '{print $1}')
  [ -n "$summary_line" ] && [ -n "$rules_line" ] && [ "$rules_line" -gt "$summary_line" ]
}

# ====================================================================
# Zero-finding path
# ====================================================================

make_clean_report() {
  cat <<'EOF'
## Critic Report: critic-rust code src/parser.rs

Summary: 0 critical, 0 warning, 0 recommendation, 0 style.
Rules applied: 4 agnostic, 5 language-specific.
EOF
}

@test "report format: zero-finding report still has Summary and Rules-applied lines" {
  make_clean_report | grep -qE '^Summary: 0 critical, 0 warning, 0 recommendation, 0 style\.$'
  make_clean_report | grep -qE '^Rules applied: [0-9]+ agnostic, [0-9]+ language-specific\.$'
}

@test "report format: zero-finding report has no severity section headers" {
  local headers
  headers=$(make_clean_report | grep -cE '^(CRITICAL|WARNING|RECOMMENDATION|STYLE)$' || true)
  [ "$headers" -eq 0 ]
}
