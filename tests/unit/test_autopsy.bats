#!/usr/bin/env bats
# Tests for in-autopsy skill and full directory install (ST0021)

load "../lib/test_helper.bash"

# Setup/teardown for autopsy tests
setup() {
  # Create temp dir outside of Intent project
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Use a fake HOME so tests never touch real ~/.claude
  setup_fake_home
}

teardown() {
  # Restore real HOME
  teardown_fake_home

  # Clean up test directory
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# ====================================================================
# Skill source existence
# ====================================================================

@test "in-autopsy SKILL.md exists in source" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/SKILL.md"
}

@test "in-autopsy scripts directory exists" {
  assert_directory_exists "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts"
}

@test "in-autopsy autopsy.exs exists" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
}

@test "in-autopsy banned-words.txt exists" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts/banned-words.txt"
}

@test "in-autopsy SKILL.md has frontmatter with description" {
  run head -3 "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/SKILL.md"
  assert_success
  assert_output_contains "---"
  assert_output_contains "description:"
}

# ====================================================================
# Skill list includes in-autopsy
# ====================================================================

@test "claude skills list shows in-autopsy" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "in-autopsy"
}

@test "claude skills list shows in-autopsy as NOT INSTALLED" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "in-autopsy"
  assert_output_contains "[NOT INSTALLED]"
}

# ====================================================================
# Full directory install
# ====================================================================

@test "claude skills install copies SKILL.md for in-autopsy" {
  run run_intent claude skills install in-autopsy --force
  assert_success
  assert_output_contains "installing: in-autopsy"
  assert_output_contains "installed"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/SKILL.md"
}

@test "claude skills install copies scripts directory for in-autopsy" {
  run run_intent claude skills install in-autopsy --force
  assert_success
  assert_directory_exists "$HOME/.claude/skills/in-autopsy/scripts"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/banned-words.txt"
}

@test "claude skills install --all includes in-autopsy with scripts" {
  run run_intent claude skills install --all --force
  assert_success
  assert_output_contains "installing: in-autopsy"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/banned-words.txt"
}

@test "existing skills still install correctly with directory copy" {
  run run_intent claude skills install in-essentials --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/in-essentials/SKILL.md"
}

@test "in-autopsy shows as INSTALLED after install" {
  run run_intent claude skills install in-autopsy --force
  assert_success

  run run_intent claude skills list
  assert_success
  assert_output_contains "in-autopsy"
  assert_output_contains "[INSTALLED]"
}

# ====================================================================
# Sync with full directory
# ====================================================================

@test "claude skills sync updates in-autopsy scripts" {
  # Install first
  run run_intent claude skills install in-autopsy --force
  assert_success

  # Modify the installed script to simulate a change
  echo "# modified" >> "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"

  # Modify source SKILL.md checksum by touching the installed one
  echo "# checksum change" >> "$HOME/.claude/skills/in-autopsy/SKILL.md"

  # Sync should detect change and update
  run run_intent claude skills sync --force
  assert_success
  assert_output_contains "in-autopsy"

  # The modification to autopsy.exs should be overwritten
  run grep "# modified" "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"
  assert_failure
}

# ====================================================================
# Show command
# ====================================================================

@test "claude skills show displays in-autopsy info" {
  run run_intent claude skills show in-autopsy
  assert_success
  assert_output_contains "Skill: in-autopsy"
  assert_output_contains "Description:"
  assert_output_contains "Content:"
}

@test "claude skills show in-autopsy extracts description from frontmatter" {
  run run_intent claude skills show in-autopsy
  assert_success
  assert_output_contains "Session forensics"
}

# ====================================================================
# Uninstall
# ====================================================================

@test "claude skills uninstall removes in-autopsy completely" {
  # Install first
  run run_intent claude skills install in-autopsy --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/in-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"

  # Uninstall
  run run_intent claude skills uninstall in-autopsy --force
  assert_success
  assert_output_contains "removed"

  # Verify entire directory is gone
  [ ! -d "$HOME/.claude/skills/in-autopsy" ] || fail "Skill directory should be removed"
}

# ====================================================================
# Script basic execution
# ====================================================================

@test "autopsy.exs --help shows usage" {
  skip_if_no_elixir
  run elixir "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs" --help
  assert_success
  assert_output_contains "autopsy.exs"
  assert_output_contains "Usage:"
  assert_output_contains "--days"
}

@test "banned-words.txt has expected format" {
  local bw="${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts/banned-words.txt"

  # Should have pattern|label lines
  run grep "|" "$bw"
  assert_success

  # Should have ai_ism entries
  run grep "ai_ism" "$bw"
  assert_success

  # Should have deferral entries
  run grep "deferral" "$bw"
  assert_success

  # Should have comments
  run grep "^#" "$bw"
  assert_success
}

# ====================================================================
# Full lifecycle
# ====================================================================

@test "in-autopsy full lifecycle: install, sync, uninstall" {
  # Install
  run run_intent claude skills install in-autopsy --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/in-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"

  # Sync (should be up to date)
  run run_intent claude skills sync
  assert_success
  assert_output_contains "up to date"

  # Uninstall
  run run_intent claude skills uninstall in-autopsy --force
  assert_success
  [ ! -d "$HOME/.claude/skills/in-autopsy" ] || fail "Skill directory should be removed"

  # List should show not installed
  run run_intent claude skills list
  assert_success
  assert_output_contains "in-autopsy"
  assert_output_contains "[NOT INSTALLED]"
}

# ====================================================================
# Helpers
# ====================================================================

skip_if_no_elixir() {
  if ! command -v elixir >/dev/null 2>&1; then
    skip "Elixir not installed"
  fi
}

# --- FENCES (2026-08-24 config sweep) -------------------------------------
#
# The detector measured roughly 5% signal and printed the raw total as its
# headline: 283 raw -> 29 assistant-authored -> 14 unquoted on Intent's own
# corpus, reproduced independently by three other estates. These hold the fix.

_autopsy_fixture() {
  local dir="$HOME/.claude/projects/-fixture-Proj"
  mkdir -p "$dir"
  # Four turns, each isolating one behaviour. Written as real transcript lines
  # so the parser is exercised rather than stubbed.
  {
    # assistant, authored, the SAME banned word TWICE -> per-occurrence = 2
    printf '%s\n' '{"type":"assistant","message":{"content":[{"type":"text","text":"Certainly this works. Certainly it does."}]}}'
    # assistant, authored, contains a bare "not" near the word: must NOT be
    # scored as negated -- ordinary English is not a negation cue.
    printf '%s\n' '{"type":"assistant","message":{"content":[{"type":"text","text":"This is not the same thing. Absolutely a distinct case."}]}}'
    # a PEER message passing through this session -> turn_kind peer_message
    printf '%s\n' '{"type":"user","message":{"content":[{"type":"text","text":"<cross-session-message from-name=\"other-vc\">Certainly the peer wrote this.</cross-session-message>"}]}}'
    # a genuine negation: the rule being discussed, not broken
    printf '%s\n' '{"type":"assistant","message":{"content":[{"type":"text","text":"Never use Certainly at the start of a reply."}]}}'
  } > "$dir/fixture-session.jsonl"
  echo "$dir"
}

@test "autopsy emits one record per OCCURRENCE, not one per (text, pattern)" {
  local script="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
  command -v elixir >/dev/null 2>&1 || skip "elixir not installed"
  _autopsy_fixture >/dev/null
  local out="$TEST_TEMP_DIR/f.json"

  run elixir "$script" --days 3650 --project fixture -o "$out"
  assert_success

  # "Certainly" appears TWICE in one assistant turn. First-occurrence-judges-all
  # would emit 1; per-occurrence emits 2.
  local n
  n="$(jq '[.banned_pattern_violations[] | select(.word=="Certainly" and .turn_kind=="authored" and .role=="assistant")] | length' "$out")"
  [ "$n" -ge 3 ] || fail "expected >=3 authored Certainly records (2 in one turn + 1 negated), got $n"
}

@test "autopsy tags turn_kind so a record can be classified from itself" {
  local script="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
  command -v elixir >/dev/null 2>&1 || skip "elixir not installed"
  _autopsy_fixture >/dev/null
  local out="$TEST_TEMP_DIR/f.json"

  run elixir "$script" --days 3650 --project fixture -o "$out"
  assert_success

  local peer
  peer="$(jq '[.banned_pattern_violations[] | select(.turn_kind=="peer_message")] | length' "$out")"
  [ "$peer" -ge 1 ] || fail "peer message not classified; context window cannot reach the marker, so scan-time tagging is the only way"

  jq -e '.banned_pattern_violations[0] | has("word")' "$out" >/dev/null || fail "no 'word' field -- the --banned-words input implies it"
  jq -e '.banned_pattern_violations[0] | has("turn_kind")' "$out" >/dev/null || fail "no 'turn_kind' field"
}

@test "autopsy does not treat a bare 'not' as a negation cue" {
  local script="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
  command -v elixir >/dev/null 2>&1 || skip "elixir not installed"
  _autopsy_fixture >/dev/null
  local out="$TEST_TEMP_DIR/f.json"

  run elixir "$script" --days 3650 --project fixture -o "$out"
  assert_success

  # "This is not the same thing. Absolutely a distinct case."
  # 71 of 103 suppressions rode on bare not/no before this was fixed.
  local suppressed
  suppressed="$(jq '[.banned_pattern_violations[] | select(.word=="Absolutely" and .is_negated==true)] | length' "$out")"
  [ "$suppressed" -eq 0 ] || fail "bare 'not' suppressed a real match again"

  # POSITIVE CONTROL: a GENUINE cue must still suppress, or the test above
  # passes on a negation check that never fires at all.
  local genuine
  genuine="$(jq '[.banned_pattern_violations[] | select(.is_negated==true)] | length' "$out")"
  [ "$genuine" -ge 1 ] || fail "no negation detected anywhere -- 'Never use Certainly' should have been suppressed"
}

@test "autopsy summary prints the funnel, not the raw total alone" {
  local script="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
  command -v elixir >/dev/null 2>&1 || skip "elixir not installed"
  _autopsy_fixture >/dev/null

  run elixir "$script" --days 3650 --project fixture -o "$TEST_TEMP_DIR/f.json"
  assert_success
  [[ "$output" == *"authored here"* ]] || fail "summary lost the authored line: $output"
  [[ "$output" == *"ACTIONABLE"* ]] || fail "summary lost the actionable line: $output"
}
