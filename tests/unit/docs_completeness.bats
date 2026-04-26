#!/usr/bin/env bats
# Tests for v2.9.0 documentation completeness (WP10).
#
# Three groups:
#   1. Presence: the three new docs exist; cross-references from CLAUDE.md /
#      MODULES.md / DECISION_TREE.md resolve.
#   2. No dead refs: no doc references the deleted `elixir` subagent path or
#      the canon `subagents/worker-bee/` path as if either were active.
#   3. Idempotent sync: `intent agents sync` run twice produces identical
#      AGENTS.md.

load "../lib/test_helper.bash"

NEW_DOCS=(
  "intent/docs/writing-extensions.md"
  "intent/docs/rules.md"
  "intent/docs/critics.md"
)

DELETED_PATHS=(
  "intent/plugins/claude/subagents/elixir/"
  "intent/plugins/claude/subagents/worker-bee/"
)

# ====================================================================
# Group 1: presence + cross-reference resolution
# ====================================================================

@test "writing-extensions.md exists" {
  assert_file_exists "${INTENT_PROJECT_ROOT}/intent/docs/writing-extensions.md"
}

@test "rules.md exists" {
  assert_file_exists "${INTENT_PROJECT_ROOT}/intent/docs/rules.md"
}

@test "critics.md exists" {
  assert_file_exists "${INTENT_PROJECT_ROOT}/intent/docs/critics.md"
}

@test "CLAUDE.md cross-references all three new docs" {
  local claude_md="${INTENT_PROJECT_ROOT}/CLAUDE.md"
  for doc in "${NEW_DOCS[@]}"; do
    grep -qF "$doc" "$claude_md" || fail "CLAUDE.md does not reference $doc"
  done
}

@test "MODULES.md registers all three new docs" {
  local modules_md="${INTENT_PROJECT_ROOT}/intent/llm/MODULES.md"
  for doc in "${NEW_DOCS[@]}"; do
    grep -qF "$doc" "$modules_md" || fail "MODULES.md does not register $doc"
  done
}

@test "DECISION_TREE.md mentions the rule library and ext system" {
  local dt="${INTENT_PROJECT_ROOT}/intent/llm/DECISION_TREE.md"
  grep -qF "rules/agnostic" "$dt" || fail "DECISION_TREE.md missing rules/agnostic placement"
  grep -qF "~/.intent/ext/" "$dt" || fail "DECISION_TREE.md missing user-extension placement"
  grep -qF "rule, a skill, or a subagent" "$dt" || fail "DECISION_TREE.md missing rule-vs-skill-vs-subagent branch"
}

@test "every .md cross-reference from new docs to siblings resolves" {
  # For each new doc, find references like `intent/docs/<file>.md` and verify
  # the target file exists in the working tree.
  local doc target
  for doc in "${NEW_DOCS[@]}"; do
    local doc_path="${INTENT_PROJECT_ROOT}/${doc}"
    while IFS= read -r target; do
      [ -z "$target" ] && continue
      assert_file_exists "${INTENT_PROJECT_ROOT}/${target}"
    done < <(grep -oE 'intent/docs/[a-zA-Z0-9_-]+\.md' "$doc_path" | sort -u)
  done
}

# ====================================================================
# Group 2: no dead refs to deleted/relocated subagents
# ====================================================================

@test "no_dead_refs: no doc cites the deleted intent/plugins/claude/subagents/elixir/ path" {
  # Search across CLAUDE.md, intent/docs/, intent/llm/, lib/help/.
  # The path may legitimately appear in historical references inside
  # ST design docs or release notes, but not in active user-facing docs.
  local hits
  hits=$(grep -rln "intent/plugins/claude/subagents/elixir" \
    "${INTENT_PROJECT_ROOT}/CLAUDE.md" \
    "${INTENT_PROJECT_ROOT}/intent/docs" \
    "${INTENT_PROJECT_ROOT}/intent/llm" \
    "${INTENT_PROJECT_ROOT}/lib/help" 2>/dev/null || true)
  [ -z "$hits" ] || fail "Dead reference to deleted elixir subagent path: $hits"
}

@test "no_dead_refs: no doc cites the canon subagents/worker-bee/ path as active" {
  # The relocated worker-bee lives at ~/.intent/ext/worker-bee/ now. The seed
  # at lib/templates/ext-seeds/worker-bee/ is allowed and expected. We grep
  # for the canon path specifically.
  local hits
  hits=$(grep -rln "intent/plugins/claude/subagents/worker-bee" \
    "${INTENT_PROJECT_ROOT}/CLAUDE.md" \
    "${INTENT_PROJECT_ROOT}/intent/docs" \
    "${INTENT_PROJECT_ROOT}/intent/llm" \
    "${INTENT_PROJECT_ROOT}/lib/help" 2>/dev/null || true)
  [ -z "$hits" ] || fail "Dead reference to canon worker-bee path: $hits"
}

@test "no_dead_refs: CLAUDE.md does not list 'elixir' as an active Available Agent" {
  # The Available Agents block in CLAUDE.md must not enumerate elixir as
  # a current canon subagent. critic-elixir is fine; the bare word elixir
  # in a numbered list item is not.
  local claude_md="${INTENT_PROJECT_ROOT}/CLAUDE.md"
  ! grep -qE '^[0-9]+\.\s+\*\*elixir\*\*' "$claude_md" \
    || fail "CLAUDE.md still lists 'elixir' as an Available Agent"
}

# ====================================================================
# Group 3: agents_sync_idempotent
# ====================================================================

@test "agents_sync_idempotent: two sync runs produce identical AGENTS.md" {
  # Run sync twice in the real Intent project root; second run must produce
  # byte-identical output. Backups created by the sync command are removed
  # at the end of the test to leave the tree clean.
  local agents_md="${INTENT_PROJECT_ROOT}/AGENTS.md"
  local first second
  # macOS BSD mktemp does not substitute the X's when followed by a suffix:
  # `mktemp /tmp/foo-XXXXXX.md` creates the LITERAL file foo-XXXXXX.md, so
  # subsequent runs collide. Use a plain template (no suffix); this test
  # cares about file content, not the extension.
  first="$(mktemp /tmp/agents-sync-1-XXXXXX)"
  second="$(mktemp /tmp/agents-sync-2-XXXXXX)"

  cd "${INTENT_PROJECT_ROOT}" || fail "could not cd to project root"

  "${INTENT_BIN_DIR}/intent" agents sync >/dev/null 2>&1 \
    || fail "first intent agents sync failed"
  cp "$agents_md" "$first"

  "${INTENT_BIN_DIR}/intent" agents sync >/dev/null 2>&1 \
    || fail "second intent agents sync failed"
  cp "$agents_md" "$second"

  diff "$first" "$second" >/dev/null \
    || fail "agents sync is not idempotent: outputs differ between runs"

  # Clean up sync-created backup if present.
  rm -f "${agents_md}.bak"
  rm -f "$first" "$second"
}
