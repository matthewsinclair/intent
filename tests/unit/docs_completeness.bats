#!/usr/bin/env bats
# Tests for v2.9.0 documentation completeness (WP10).
#
# Three groups:
#   1. Presence: the three new docs exist; cross-references from CLAUDE.md /
#      MODULES.md / DECISION_TREE.md resolve.
#   2. No dead refs: no doc references the deleted `elixir` subagent path as
#      if it were active.
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
  grep -qF "~/.local/share/intent/ext/" "$dt" || fail "DECISION_TREE.md missing user-extension placement"
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

# IN A SCRATCH PROJECT, NEVER THE CHECKOUT (dc's todo 87). `agents sync` is a
# writer: it rewrites AGENTS.md and records the file it wrote in its project's
# store. Run in the checkout, it moved the store of the tree under test on every
# suite run, and raced prior_art_route.bats, which reads the root's AGENTS.md.
# Idempotence is the renderer's property, so a project `intent init` makes
# answers it as well as this one does.
@test "agents_sync_idempotent: two sync runs produce identical AGENTS.md" {
  export HOME="$TEST_TEMP_DIR/home"
  mkdir -p "$HOME" "$TEST_TEMP_DIR/proj"
  unset_user_dirs
  cd "$TEST_TEMP_DIR/proj" || fail "could not cd to the scratch project"
  "$INTENT_BIN" init "Agents sync" >/dev/null 2>&1 \
    || fail "intent init did not make the scratch project"

  "$INTENT_BIN" agents sync >/dev/null 2>&1 \
    || fail "first intent agents sync failed"
  cp AGENTS.md "$TEST_TEMP_DIR/first"

  "$INTENT_BIN" agents sync >/dev/null 2>&1 \
    || fail "second intent agents sync failed"

  diff "$TEST_TEMP_DIR/first" AGENTS.md >/dev/null \
    || fail "agents sync is not idempotent: outputs differ between runs"
}

# ====================================================================
# Group 4: canon docs describe only as-built artefacts (ST0042 T9)
# ====================================================================
#
# The v2.11.x doc-drift audit found canon docs describing commands,
# paths, and hook shapes that never existed or had moved. These guards
# pin the specific phantoms so the class cannot regress.

CANON_DOCS=(
  "intent/docs/working-with-llms.md"
  "intent/docs/critics.md"
  "intent/docs/rules.md"
  "intent/docs/writing-extensions.md"
  "usage-rules.md"
  "README.md"
)

@test "no canon doc references the phantom 'skills status' subcommand" {
  run grep -rn 'skills status' "${INTENT_PROJECT_ROOT}/intent/docs" "${INTENT_PROJECT_ROOT}/usage-rules.md"
  [ "$status" -ne 0 ] || fail "phantom 'intent claude skills status' reference: $output"
}

@test "no canon doc uses the never-existed 'matchers' (plural) hook key" {
  # Real settings.json uses singular string 'matcher'; 'matchers' (array)
  # was documented but never shipped.
  run grep -rn '"matchers"\|matchers:' "${INTENT_PROJECT_ROOT}/intent/docs"
  [ "$status" -ne 0 ] || fail "phantom 'matchers' hook key: $output"
}

@test "no canon doc references the pre-v2.10 .intent/ config path" {
  # Config relocated to intent/.config/ in v2.10.0 (ST0036). Allow only
  # the user-ext root ~/.intent/ext and the nested intent/.config form.
  local hits=""
  local doc
  for doc in "${CANON_DOCS[@]}"; do
    hits="$hits$(grep -nE '(^|[^/~a-z])\.intent/config' "${INTENT_PROJECT_ROOT}/${doc}" 2>/dev/null || true)"
  done
  [ -z "$hits" ] || fail "pre-v2.10 .intent/config path in canon docs: $hits"
}

@test "no canon doc references phantom soft/strict hook script variants" {
  run grep -rnE 'user_prompt_submit_(soft|strict)\.sh|session_start\.sh|stop_reminder\.sh' \
    "${INTENT_PROJECT_ROOT}/intent/docs"
  [ "$status" -ne 0 ] || fail "phantom hook-script name: $output"
}

@test "usage-rules.md lists the in-whiteboard skill" {
  assert_file_contains "${INTENT_PROJECT_ROOT}/usage-rules.md" "/in-whiteboard"
}
