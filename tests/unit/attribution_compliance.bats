#!/usr/bin/env bats
# Tests for MIT attribution compliance of elixir-test-critic-derived rules.
#
# Guards the invariant declared in rules/_schema/attribution-policy.md:
#   Every rule with `upstream_id:` set in its frontmatter MUST have a
#   matching row in the attribution file's "Rules derived from upstream
#   principles" table. Without this invariant, MIT attribution silently
#   drifts out of sync with the rules themselves.
#
# The attribution file also carries the pinned upstream commit and the full
# MIT notice — those are static-content tests; the row invariant is the
# dynamic one that bites when rules are added or removed.

load "../lib/test_helper.bash"

ATTRIBUTION_FILE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/_attribution/elixir-test-critic.md"
ELIXIR_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir"

# ====================================================================
# Static attribution content
# ====================================================================

@test "attribution file exists" {
  assert_file_exists "$ATTRIBUTION_FILE"
}

@test "attribution file carries the pinned upstream commit" {
  assert_file_contains "$ATTRIBUTION_FILE" '1d9aa40700dab7370b4abd338ce11b922e914b14'
}

@test "attribution file carries the MIT copyright" {
  assert_file_contains "$ATTRIBUTION_FILE" 'Copyright (c) 2026 Manuel Zubieta'
}

@test "attribution file carries the MIT permission text" {
  assert_file_contains "$ATTRIBUTION_FILE" 'Permission is hereby granted, free of charge'
}

# ====================================================================
# upstream_id:-to-attribution invariant
# ====================================================================
#
# For every rule under rules/elixir/**/RULE.md with `upstream_id:` in
# frontmatter, the attribution file must list both the rule ID and its
# upstream slug in the same table row.

collect_upstream_rules() {
  # Emit one line per rule, tab-separated: <rule_id>\t<upstream_slug>
  local rule id upstream
  while IFS= read -r rule; do
    id=$(awk '/^id:[[:space:]]+/ { print $2; exit }' "$rule")
    upstream=$(awk '/^upstream_id:[[:space:]]+/ { print $2; exit }' "$rule")
    [ -n "$upstream" ] || continue
    printf '%s\t%s\n' "$id" "$upstream"
  done < <(find "$ELIXIR_ROOT" -name 'RULE.md' -type f)
}

@test "every rule with upstream_id has a matching attribution row" {
  local id upstream
  while IFS=$'\t' read -r id upstream; do
    [ -z "$id" ] && continue
    grep -qE "^\|[[:space:]]+\`$id\`[[:space:]]+\|[[:space:]]+\`$upstream\`" "$ATTRIBUTION_FILE" || {
      echo "missing attribution row for rule $id (upstream_id: $upstream) in $ATTRIBUTION_FILE" >&2
      return 1
    }
  done < <(collect_upstream_rules)
}

@test "attribution table lists exactly the rules with upstream_id set" {
  # Parse the attribution table to find claimed Intent rule IDs.
  local claimed_ids
  claimed_ids=$(awk -F'|' '
    /^\|[[:space:]]+`IN-EX/ {
      gsub(/`/, "", $2)
      gsub(/[[:space:]]/, "", $2)
      print $2
    }
  ' "$ATTRIBUTION_FILE" | sort -u)

  # And find the real set of rules with upstream_id set.
  local real_ids
  real_ids=$(collect_upstream_rules | cut -f1 | sort -u)

  [ "$claimed_ids" = "$real_ids" ] || {
    echo "attribution table mismatch" >&2
    echo "--- attribution table claims:" >&2
    echo "$claimed_ids" >&2
    echo "--- rules actually carrying upstream_id:" >&2
    echo "$real_ids" >&2
    return 1
  }
}

# ====================================================================
# upstream_id format
# ====================================================================

@test "every upstream_id is a non-empty kebab-case slug" {
  local id upstream
  while IFS=$'\t' read -r id upstream; do
    [ -z "$id" ] && continue
    [[ "$upstream" =~ ^[a-z][a-z0-9-]*$ ]] || {
      echo "rule $id has malformed upstream_id: '$upstream'" >&2
      return 1
    }
  done < <(collect_upstream_rules)
}
