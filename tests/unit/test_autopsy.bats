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

# ====================================================================
# Full directory install
# ====================================================================

@test "claude skills install copies scripts directory for in-autopsy" {
  run run_intent claude skills install in-autopsy --force
  assert_success
  assert_directory_exists "$HOME/.claude/skills/in-autopsy/scripts"
  assert_file_exists "$HOME/.claude/skills/in-autopsy/scripts/autopsy.exs"
}

@test "existing skills still install correctly with directory copy" {
  run run_intent claude skills install in-essentials --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/in-essentials/SKILL.md"
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

  # Every file it installed is gone, and the directory it emptied with them.
  [ ! -e "$HOME/.claude/skills/in-autopsy" ] || fail "the skill directory should be removed"
}

# ====================================================================
# Script basic execution
# ====================================================================

@test "autopsy.exs --help shows usage" {
  require_elixir "the autopsy detector's behaviour on a real transcript"
  run elixir "${INTENT_HOME}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs" --help
  assert_success
  assert_output_contains "autopsy.exs"
  assert_output_contains "Usage:"
  assert_output_contains "--days"
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
  # The files go and the emptied directory stays (0218).
  [ ! -e "$HOME/.claude/skills/in-autopsy/SKILL.md" ] || fail "SKILL.md should be removed"
  [ -z "$(ls -A "$HOME/.claude/skills/in-autopsy")" ] || fail "the skill directory should be left empty"

  # The listing is the canon roster with each row's install state (0150), so an
  # uninstalled canon skill keeps its one row and reads `-` for its install
  # state; the emptied directory is not a second row.
  run run_intent claude skills list
  assert_success
  echo "$output" | grep -qE '^in-autopsy +- +canon$' || fail "in-autopsy should list as not installed"
  [ "$(echo "$output" | grep -c 'in-autopsy')" = "1" ] || fail "in-autopsy should have exactly one row"
}


# ====================================================================
# The detectors (STee6564b): one fixture session, one arm for each check
# ====================================================================

# The fixture's absolute paths. No file is read at them.
FX_CHECKOUT=/fx/main
FX_DAY=2026-01-02
FX_SLOT_ENDED=01AAAAAAAAAAAAAAAAAAAAAAAA
FX_SLOT_OPEN=01BBBBBBBBBBBBBBBBBBBBBBBB

require_elixir() {
  require_tool elixir "$1"
}

# Appends one transcript line, and records its line number under a name.
_emit() {
  printf '%s\n' "$2" >> "$FX"
  printf '%s=%s\n' "$1" "$(wc -l < "$FX" | tr -d ' ')" >> "$MARKS"
}

_line() {
  sed -n "s/^$1=//p" "$MARKS"
}

_at() {
  printf '%sT%s.000Z' "$FX_DAY" "$1"
}

# _hear <name> <time> <text> [extra fields as JSON]: a `user` entry.
_hear() {
  _emit "$1" "$(jq -nc --arg t "$(_at "$2")" --arg x "$3" --arg cwd "$FX_CHECKOUT" --argjson e "${4:-{\}}" \
    '{type:"user",timestamp:$t,cwd:$cwd,message:{role:"user",content:$x}} + $e')"
}

_say() {
  _emit "$1" "$(jq -nc --arg t "$(_at "$2")" --arg x "$3" --arg cwd "$FX_CHECKOUT" \
    '{type:"assistant",timestamp:$t,cwd:$cwd,message:{role:"assistant",content:[{type:"text",text:$x}]}}')"
}

# _tool <name> <time> <tool> <input as JSON> [cwd]: a tool call whose id is its name.
_tool() {
  _emit "$1" "$(jq -nc --arg t "$(_at "$2")" --arg id "$1" --arg tool "$3" --argjson in "$4" --arg cwd "${5:-$FX_CHECKOUT}" \
    '{type:"assistant",timestamp:$t,cwd:$cwd,message:{role:"assistant",content:[{type:"tool_use",id:$id,name:$tool,input:$in}]}}')"
}

_bash() {
  _tool "$1" "$2" Bash "$(jq -nc --arg c "$3" '{command:$c}')" "${4:-}"
}

_msg() {
  _tool "$1" "$2" SendMessage "$(jq -nc --arg to "$3" --arg m "$4" '{to:$to,message:$m}')"
}

# _result <name> <text> [true when the call failed]: the result of the call <name>.
_result() {
  _emit "$1_r" "$(jq -nc --arg id "$1" --arg x "$2" --argjson err "${3:-false}" \
    '{type:"user",message:{role:"user",content:[{type:"tool_result",tool_use_id:$id,content:$x,is_error:$err}]}}')"
}

_fixture() {
  _emit agent '{"type":"agent-name","agentName":"proj-dc"}'

  # Who wrote each entry. Every one of these holds the word "flag".
  _hear person_flag 10:00:00 "Please flag this for the autopsy."
  _hear seat_flag 10:00:01 "vc to dc: flag this." '{"origin":{"kind":"peer","name":"proj-vc","from":"uds:/x"}}'
  _hear subagent_flag 10:00:02 "[Subagent hand-back] flag" '{"origin":{"kind":"peer","from":"a1","senderTaskId":"a1"}}'
  _hear task_flag 10:00:03 "<task-notification>flag</task-notification>" '{"origin":{"kind":"task-notification"}}'
  _hear summary_flag 10:00:04 "This session is being continued. The user said flag." '{"isCompactSummary":true}'
  _hear skill_flag 10:00:05 "Base directory for this skill: /x. flag" '{"isMeta":true}'

  # Stamps. The clock is read once, at 10:01.
  _bash clock 10:01:05 "date -u +'%Y-%m-%d %H:%MZ'"
  _result clock "$FX_DAY 10:01Z"
  _msg stamp_good 10:01:20 proj-vc "dc to vc, $FX_DAY 10:01Z by date -u: read."
  _msg stamp_ahead 10:02:00 proj-vc "dc to vc, $FX_DAY 10:09Z by date -u: ahead."
  _hear turn_a 10:20:00 "next"
  _hear turn_b 10:20:01 "next"
  _msg stamp_unread 10:20:30 proj-vc "dc to vc, $FX_DAY 10:15Z by date -u: no read."
  _msg stamp_noz 10:21:00 proj-vc "The clock said $FX_DAY 10:01 before."

  # Commits and the paths the verbs printed.
  _hear turn_c 10:30:00 "do the board work"
  _bash verb_out 10:30:10 'intent wb add todo "x" --node dc'
  _result verb_out $'ok: dc todo 1\nwrote: intent/whiteboard/dc/board.json\nre-rendered: intent/todo.md'
  _bash add_good 10:30:20 'git add -- intent/whiteboard/dc/board.json'
  _bash commit_good 10:30:30 'git commit --only intent/whiteboard/dc/board.json -m "wb(dc): one"'
  _bash add_all 10:30:40 'git add -A'
  _bash commit_stray 10:30:50 'git commit --only intent/whiteboard/cc/board.json -m "wb: stray"'
  _bash commit_rerendered 10:31:00 'git commit --only intent/todo.md -m "todo"'
  _bash commit_wt 10:31:10 'git commit -m "in a worktree"' "$FX_CHECKOUT/tmp/wt"
  _bash commit_no_only 10:31:20 'git commit -m "all of it"'
  _bash verb_uncommitted 10:31:30 'intent wb add todo "y" --node dc'
  _result verb_uncommitted $'ok: dc todo 2\nwrote: intent/whiteboard/dc/wip.md'

  # The whiteboard.
  _bash other_seat 10:32:00 'intent wb add todo "z" --node cc'
  _tool hand_edit 10:32:10 Edit "$(jq -nc --arg p "$FX_CHECKOUT/intent/whiteboard/dc/wip.md" '{file_path:$p,old_string:"a",new_string:"b"}')"
  _tool edit_good 10:32:20 Edit "$(jq -nc --arg p "$FX_CHECKOUT/lib/x.ex" '{file_path:$p,old_string:"a",new_string:"b"}')"
  _bash ask_hv 10:32:30 'intent wb ask hv "a question" --node dc'
  _bash ask_vc 10:32:40 'intent wb ask vc "a question" --node dc'
  _bash release 10:32:50 'intent wb release --node dc'

  # The Desk and the acceptance tests.
  _bash at_red_bad 10:33:00 'intent at red AT-1'
  _bash heavy_bad 10:33:10 'bin/devbin test shell'
  _bash at_red_good 10:33:20 'intent at red AT-2'
  _bash at_green_bad 10:33:30 'intent at green AT-2'
  _bash slot_start_good 10:33:40 "intent estate slots start $FX_SLOT_ENDED --node dc"
  _msg start_good 10:33:45 proj-vc "START: the run on $FX_SLOT_ENDED"
  _bash heavy_good 10:33:50 'bin/devbin test shell'
  _bash at_green_good 10:34:00 'intent at green AT-3'
  _bash slot_release 10:34:10 "intent estate slots release $FX_SLOT_ENDED --node dc"
  _msg end_good 10:34:15 proj-vc "END: the run on $FX_SLOT_ENDED is green"
  _msg start_bad 10:34:20 proj-cc "START: a run with no end"
  _bash slot_start_bad 10:34:30 "intent estate slots start $FX_SLOT_OPEN --node dc"
  _bash fiat 10:34:40 'intent fc ST0001 --reason "not worth the work"'

  # Worktrees.
  _bash wt_forced 10:35:00 'git worktree remove --force tmp/wt-a'
  _bash attic 10:35:10 'git update-ref refs/attic/dc/wt-b HEAD'
  _bash wt_good 10:35:20 'git worktree remove tmp/wt-b'

  # What the tools said. The turn ends with text, so its failures are not silent.
  _bash refuse_once 10:36:00 'intent wb show nobody'
  _result refuse_once $'Exit code 1\nerror: no node named nobody' true
  _bash refuse_1 10:36:10 'intent estate slots ask gate "x" --lane a,b'
  _result refuse_1 $'Exit code 2\nerror: an ask names at most one preferred lane' true
  _bash refuse_2 10:36:20 'intent estate slots ask gate "y" --lane c,d'
  _result refuse_2 $'Exit code 2\nerror: an ask names at most one preferred lane' true
  _bash exit_good 10:36:30 'false'
  _result exit_good 'Exit code 1' true
  _say closing 10:36:40 "Four commands failed, and I read each error."

  # A failure with no text after it in its turn.
  _hear turn_d 10:40:00 "one more"
  _bash exit_bad 10:40:10 'false'
  _result exit_bad 'Exit code 1' true

  # The session's last act, and a line that is not JSON.
  _hear turn_e 10:41:00 "finish"
  _bash release_last 10:41:10 'intent wb release --node dc'
  _emit garbage '{"type":"user", this line is not JSON'
}

_autopsy() {
  CLAUDE_CONFIG_DIR="$FX_CONFIG" elixir "$AUTOPSY" "$@"
}

setup_file() {
  export AUTOPSY="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs"
  export FX_CONFIG="${BATS_FILE_TMPDIR}/config"
  export FX="${FX_CONFIG}/projects/-fx-One/fixture-session.jsonl"
  export MARKS="${BATS_FILE_TMPDIR}/marks"
  export OUT="${BATS_FILE_TMPDIR}/findings.json"
  export RUN_LOG="${BATS_FILE_TMPDIR}/run.log"
  mkdir -p "$(dirname "$FX")" "${FX_CONFIG}/projects/-fx-Two"
  _fixture

  # With no elixir, each arm fails in `require_elixir` and names the tool.
  command -v elixir >/dev/null 2>&1 || return 0
  _autopsy --all --since "$FX_DAY" --project fx-One --checkout "$FX_CHECKOUT" --mediator vc \
    --views 'intent/whiteboard/**' --heavy '^bin/devbin test' -o "$OUT" > "$RUN_LOG" 2>&1 || true
}

# True when the check has a finding at the named fixture line.
_found() {
  jq -e --arg c "$1" --argjson l "$(_line "$2")" 'any(.findings[]; .check == $c and .line == $l)' "$OUT" >/dev/null
}

# _arm <check> <the line that breaks the rule> <a line that keeps it>
_arm() {
  require_elixir "what the $1 check finds"
  [ -s "$OUT" ] || fail "autopsy.exs wrote no output: $(cat "$RUN_LOG")"
  _found "$1" "$2" || fail "$1 did not find the line that breaks the rule ($2)"
  ! _found "$1" "$3" || fail "$1 fired on a line that keeps the rule ($3)"
}

@test "autopsy check stamp_ahead_of_clock" { _arm stamp_ahead_of_clock stamp_ahead stamp_good; }
@test "autopsy check stamp_without_read" { _arm stamp_without_read stamp_unread stamp_good; }
@test "autopsy check stamp_without_z" { _arm stamp_without_z stamp_noz stamp_good; }
@test "autopsy check commit_without_only" { _arm commit_without_only commit_no_only commit_good; }
@test "autopsy check commit_without_only leaves a worktree's commit" { _arm commit_without_only commit_no_only commit_wt; }
@test "autopsy check add_all" { _arm add_all add_all add_good; }
@test "autopsy check commit_path_not_printed" { _arm commit_path_not_printed commit_stray commit_good; }
@test "autopsy check printed_path_not_committed" { _arm printed_path_not_committed verb_uncommitted_r verb_out_r; }
@test "autopsy check commit_of_rerendered" { _arm commit_of_rerendered commit_rerendered commit_good; }
@test "autopsy check write_as_other_seat" { _arm write_as_other_seat other_seat verb_out; }
@test "autopsy check hand_edit_of_view" { _arm hand_edit_of_view hand_edit edit_good; }
@test "autopsy check release_then_work" { _arm release_then_work release release_last; }
@test "autopsy check ask_hv_past_mediator" { _arm ask_hv_past_mediator ask_hv ask_vc; }
@test "autopsy check heavy_run_without_slot" { _arm heavy_run_without_slot heavy_bad heavy_good; }
@test "autopsy check slot_not_released" { _arm slot_not_released slot_start_bad slot_start_good; }
@test "autopsy check start_without_end" { _arm start_without_end start_bad start_good; }
@test "autopsy check at_red_without_run" { _arm at_red_without_run at_red_bad at_red_good; }
@test "autopsy check at_green_without_run" { _arm at_green_without_run at_green_bad at_green_good; }
@test "autopsy check fiat_close" { _arm fiat_close fiat verb_out; }
@test "autopsy check worktree_remove_forced" { _arm worktree_remove_forced wt_forced wt_good; }
@test "autopsy check worktree_remove_without_attic" { _arm worktree_remove_without_attic wt_forced wt_good; }
@test "autopsy check refusal_repeated" { _arm refusal_repeated refuse_2_r refuse_once_r; }
@test "autopsy check exit_not_acted_on" { _arm exit_not_acted_on exit_bad exit_good; }

@test "autopsy has an arm for every check it ships" {
  require_elixir "which checks autopsy ships"
  local check
  for check in $(jq -r '.checks_run[].check' "$OUT"); do
    grep -q "^@test \"autopsy check ${check}\"" "$BATS_TEST_FILENAME" || fail "no arm for the check ${check}"
  done
  # Every check ran on the fixture, so a clean arm above is not a check that did not run.
  run jq -r '.checks_run[] | select(.ran_on == []) | .check' "$OUT"
  [ -z "$output" ] || fail "these checks did not run on the fixture: $output"
}

@test "autopsy reads a person's words only from an entry a person typed" {
  require_elixir "who autopsy says wrote each entry"
  local name source
  # Each entry holds the word "flag". Only the typed one is a person's signal.
  jq -e --argjson l "$(_line person_flag)" 'any(.signals[]; .by == "person" and .line == $l)' "$OUT" >/dev/null ||
    fail "the typed entry gave no person signal"
  for name in seat_flag subagent_flag task_flag summary_flag skill_flag; do
    ! jq -e --argjson l "$(_line "$name")" 'any(.signals[]; .by == "person" and .line == $l)' "$OUT" >/dev/null ||
      fail "$name was read as a person's words"
  done
  for name in person seat subagent task_notification compaction_summary skill_content; do
    case "$name" in
      task_notification) source=task_flag ;;
      compaction_summary) source=summary_flag ;;
      skill_content) source=skill_flag ;;
      *) source="${name}_flag" ;;
    esac
    jq -e --arg s "$name" --argjson l "$(_line "$source")" 'any(.sessions[].user_entries[]; .source == $s and .line == $l)' "$OUT" >/dev/null ||
      fail "the entry at $source is not named as $name"
  done
  jq -e '.sessions[0].seat == "dc"' "$OUT" >/dev/null || fail "the session's seat was not read from its agent-name entry"
}

@test "autopsy reports what it could not read or could not run" {
  require_elixir "what autopsy reports when it cannot read or run"
  jq -e --argjson l "$(_line garbage)" 'any(.unread[]; .line == $l)' "$OUT" >/dev/null ||
    fail "the line that is not JSON is not in unread"

  run _autopsy --all --project fx-One --no-such-argument
  assert_failure
  assert_output_contains "remedy:"

  # A check that needs a project value does not run without it, and the output says so.
  local out="$TEST_TEMP_DIR/bare.json"
  run _autopsy --all --since "$FX_DAY" --project fx-One -o "$out"
  assert_success
  jq -e 'any(.checks_run[]; .check == "heavy_run_without_slot" and .ran_on == [] and .not_run != [])' "$out" >/dev/null ||
    fail "a check with no --heavy value was not reported as not run"
  jq -e '.run.pattern_scan | startswith("did not run")' "$out" >/dev/null || fail "the missing pattern file was not reported"
}

@test "autopsy refuses a project name that matches more than one folder" {
  require_elixir "what autopsy does with an ambiguous project name"
  run _autopsy --all --project fx-
  assert_failure
  assert_output_contains "fx-One"
  assert_output_contains "fx-Two"
}

@test "autopsy scans what the seat wrote with the pattern file of the run" {
  require_elixir "what autopsy's pattern scan finds"
  local patterns="$TEST_TEMP_DIR/patterns.txt" out="$TEST_TEMP_DIR/scan.json"
  {
    echo '# scope :: pattern :: source'
    echo 'prose :: \bno read\b :: a test rule'
    echo 'command :: git add -A :: /in-essentials rule 9'
  } > "$patterns"
  run _autopsy --all --since "$FX_DAY" --project fx-One --patterns "$patterns" -o "$out"
  assert_success
  jq -e --argjson l "$(_line stamp_unread)" 'any(.patterns[]; .scope == "prose" and .line == $l and .source == "a test rule")' "$out" >/dev/null ||
    fail "the prose pattern did not find the message that holds it"
  jq -e --argjson l "$(_line add_all)" 'any(.patterns[]; .scope == "command" and .line == $l)' "$out" >/dev/null ||
    fail "the command pattern did not find the command that holds it"
  ! jq -e --argjson l "$(_line add_good)" 'any(.patterns[]; .line == $l)' "$out" >/dev/null ||
    fail "a pattern matched a line that does not hold it"

  echo 'prose :: ( :: an unclosed group' > "$patterns"
  run _autopsy --all --project fx-One --patterns "$patterns"
  assert_failure
  assert_output_contains "line 1"
}
