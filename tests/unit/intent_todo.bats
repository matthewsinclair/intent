#!/usr/bin/env bats
# ST0050: intent todo -- a projection of intent/st/** into a flat DOING/TODO/DONE
# view (markdown + --json). This file is the WP-01 read-path harness and the
# WP-04 AC-04.1 rollup.

load "../lib/test_helper.bash"

# grep -F with `--` so a line starting with "-" (a GFM list item) is not read
# as a grep option -- the house assert_file_contains lacks the `--`.
assert_line() {
  grep -qF -- "$2" "$1" || { echo "missing in $1: $2"; cat "$1"; return 1; }
}

# Build a steel-thread fixture directly, for precise control over
# status / completed / on-hold without driving the full CLI + close-gate.
mk_st() {  # mk_st <base> <ID> <status> <title> [completed] [onhold]
  local base="$1" id="$2" status="$3" title="$4" completed="${5:-}" onhold="${6:-}"
  mkdir -p "$base/$id"
  {
    echo "---"
    echo "status: $status"
    echo "completed: $completed"   # always present (empty if none) so intent st done can stamp it
    [ -n "$onhold" ] && echo "on-hold: $onhold"
    echo "---"
    echo ""
    echo "# $id: $title"
  } > "$base/$id/info.md"
}

mk_wp() {  # mk_wp <st_dir> <NN> <status> <title>
  local st_dir="$1" nn="$2" status="$3" title="$4"
  mkdir -p "$st_dir/WP/$nn"
  {
    echo "---"
    echo "status: $status"
    echo "title: $title"
    echo "---"
  } > "$st_dir/WP/$nn/info.md"
}

setup_todo_project() {
  local d
  d=$(create_test_project "Todo Test")
  cd "$d"
  mk_st "intent/st" "ST0001" "WIP" "active thread"
  mk_wp "intent/st/ST0001" "01" "Done" "first wp"
  mk_wp "intent/st/ST0001" "02" "WIP" "second wp"
  mk_st "intent/st/NOT-STARTED" "ST0002" "Not Started" "queued thread"
  # UTC to match the watermark: read_done_watermark + flush both use `date -u`.
  # A local `date +%Y%m%d` stamps tomorrow-in-UTC during the window where local
  # (eg UTC+1) has ticked over the day but UTC has not, so flush cannot clear it.
  mk_st "intent/st/COMPLETED" "ST0003" "Completed" "shipped today" "$(date -u +%Y%m%d)"
  mk_st "intent/st/COMPLETED" "ST0004" "Completed" "shipped long ago" "20200101"
  mk_st "intent/st" "ST0005" "WIP" "held thread" "" "TRUE"
}

@test "update projects threads and WPs into DOING/TODO/DONE buckets" {
  setup_todo_project
  run run_intent todo update
  assert_success
  assert_line intent/todo.md "- [-] ST0001: active thread"
  assert_line intent/todo.md "  - [x] 01: first wp"
  assert_line intent/todo.md "  - [-] 02: second wp"
  assert_line intent/todo.md "- [ ] ST0002: queued thread"
  assert_line intent/todo.md "- [x] ST0003: shipped today"
}

@test "checkbox glyphs map each status" {
  setup_todo_project
  run_intent todo update
  assert_file_contains intent/todo.md "[-] ST0001"
  assert_file_contains intent/todo.md "[ ] ST0002"
  assert_file_contains intent/todo.md "[x] ST0003"
}

@test "non-canonical status string renders its real glyph, not [?] (issue 0002)" {
  # A thread whose info.md carries the directory-name form "NOT-STARTED" (a
  # synonym intent st accepts via canonical_status) must render as TODO "[ ]",
  # not the unknown-status "[?]". Before the fix status_box keyed on the raw
  # frontmatter value and fell through to "?"; now it canonicalises first, so
  # intent todo and intent st agree about the same thread's status.
  setup_todo_project
  mk_st "intent/st/NOT-STARTED" "ST0006" "NOT-STARTED" "noncanonical status"
  run_intent todo update
  assert_line intent/todo.md "- [ ] ST0006: noncanonical status"
  run grep -cF -- "[?] ST0006" intent/todo.md
  assert_output "0"
}

@test "DONE bucket self-sweeps to today (older completions drop off)" {
  setup_todo_project
  run_intent todo update
  assert_file_contains intent/todo.md "ST0003: shipped today"
  run grep -c "ST0004: shipped long ago" intent/todo.md
  assert_output "0"
}

@test "on-hold thread is tagged in DOING" {
  setup_todo_project
  run_intent todo update
  assert_line intent/todo.md "- [-] ST0005: held thread (on-hold)"
}

@test "todo.md is a generator marker + bucket headings + data (no human title/legend/provenance)" {
  setup_todo_project
  run_intent todo update
  assert_file_contains intent/todo.md "generator: intent todo"
  assert_file_contains intent/todo.md "## DOING"
  assert_file_contains intent/todo.md "## TODO"
  assert_file_contains intent/todo.md "## DONE"
  run grep -c "# Intent Todo" intent/todo.md
  assert_output "0"
  run grep -c "_Generated" intent/todo.md
  assert_output "0"
  run grep -c "Legend" intent/todo.md
  assert_output "0"
}

# Mutual-guard with utilz `todo` (a fork sharing this file format): each tool
# stamps `generator: <tool> todo` and refuses to overwrite a file owned by the other.
@test "update refuses to overwrite a foreign (utilz todo) file" {
  setup_todo_project
  printf '%s\n' '---' 'generator: utilz todo' 'history: _hist/YYYYMMDD.md' '---' '' '## DOING' '' '01:[-] a utilz item' > intent/todo.md
  run run_intent todo update
  assert_failure
  assert_output_contains "refusing to overwrite"
  assert_file_contains intent/todo.md "generator: utilz todo"
  assert_file_contains intent/todo.md "01:[-] a utilz item"
}

@test "update refuses a utilz file that predates the marker (title/history frontmatter)" {
  setup_todo_project
  printf '%s\n' '---' 'title: "# TODO"' 'history: _hist/YYYYMMDD.md' '---' '' '## DOING' '' '01:[-] a utilz item' > intent/todo.md
  run run_intent todo update
  assert_failure
  assert_output_contains "foreign frontmatter"
  assert_file_contains intent/todo.md "01:[-] a utilz item"
}

@test "update regenerates a legacy intent file (no frontmatter) and stamps the marker" {
  setup_todo_project
  printf '%s\n' '## DOING' '' '_(none)_' '' '## TODO' '' '_(none)_' '' '## DONE:2020-01-01T00:00:00Z' '' '_(none)_' > intent/todo.md
  run run_intent todo update
  assert_success
  assert_file_contains intent/todo.md "generator: intent todo"
  assert_file_contains intent/todo.md "ST0001: active thread"
}

@test "output is prettier-stable" {
  command -v prettier >/dev/null 2>&1 || skip "prettier not installed"
  setup_todo_project
  run_intent todo update
  cp intent/todo.md before.md
  prettier --write intent/todo.md >/dev/null 2>&1
  run diff before.md intent/todo.md
  assert_success
}

@test "list prints the file and help shows usage" {
  setup_todo_project
  run run_intent todo list
  assert_success
  assert_output_contains "## DOING"
  run run_intent todo help
  assert_output_contains "Usage: intent todo"
}

@test "--json emits a valid keyed-by-bucket structure" {
  setup_todo_project
  run run_intent todo --json
  assert_success
  echo "$output" | jq -e '.doing and .todo and .done' >/dev/null
  echo "$output" | jq -e '.doing[] | select(.id=="ST0001") | .work_packages | length == 2' >/dev/null
  echo "$output" | jq -e '.todo[] | select(.id=="ST0002")' >/dev/null
  echo "$output" | jq -e '.done[] | select(.id=="ST0003")' >/dev/null
}

# ---- WP-02: mutation verbs (done / notdone / toggle) ----

@test "done marks a thread done (via intent st done) and regenerates" {
  local d
  d=$(create_test_project "Todo Done")
  cd "$d"
  mk_st "intent/st" "ST0001" "WIP" "closing thread"
  write_exempt_acceptance "intent/st/ST0001"
  run run_intent todo done ST0001
  assert_success
  assert_file_exists "intent/st/COMPLETED/ST0001/info.md"
  assert_line intent/todo.md "- [x] ST0001: closing thread"
}

@test "done inherits the acceptance close-gate: a BLOCKED contract is refused" {
  local d
  d=$(create_test_project "Todo Gate")
  cd "$d"
  mk_st "intent/st" "ST0001" "WIP" "blocked thread"
  printf '%s\n' '---' 'st_id: ST0001' '---' '# Acceptance (no ACs -> BLOCKED)' > intent/st/ST0001/acceptance.md
  run run_intent todo done ST0001
  assert_failure
  assert_output_contains "BLOCKED"
  assert_file_not_exists "intent/st/COMPLETED/ST0001/info.md"
}

@test "notdone reopens a completed thread to WIP" {
  local d
  d=$(create_test_project "Todo Reopen")
  cd "$d"
  mk_st "intent/st/COMPLETED" "ST0001" "Completed" "shipped thread" "$(date +%Y%m%d)"
  run run_intent todo notdone ST0001
  assert_success
  run grep -m1 "^status:" intent/st/ST0001/info.md
  assert_output_contains "WIP"
}

@test "toggle flips done/not-done from the current status" {
  local d
  d=$(create_test_project "Todo Toggle")
  cd "$d"
  mk_st "intent/st" "ST0001" "WIP" "toggle thread"
  write_exempt_acceptance "intent/st/ST0001"
  run run_intent todo toggle ST0001
  assert_success
  assert_file_exists "intent/st/COMPLETED/ST0001/info.md"
  run run_intent todo toggle ST0001
  assert_success
  run grep -m1 "^status:" intent/st/ST0001/info.md
  assert_output_contains "WIP"
}

# ---- WP-06: DONE lifecycle (flush / prune) + ISO completion timestamp ----

@test "st done stamps completed: as an ISO 8601 UTC timestamp" {
  local d
  d=$(create_test_project "Todo ISO")
  cd "$d"
  mk_st "intent/st" "ST0001" "WIP" "closing thread"
  write_exempt_acceptance "intent/st/ST0001"
  run run_intent st done ST0001
  assert_success
  run grep -m1 "^completed:" intent/st/COMPLETED/ST0001/info.md
  assert_success
  [[ "$output" =~ ^completed:\ [0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] \
    || { echo "not ISO 8601: $output"; false; }
}

@test "DONE bucket is watermarked and lists completions at/after it" {
  setup_todo_project
  run_intent todo update
  run grep -qE '^## DONE:[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z' intent/todo.md
  assert_success
  # ST0003 completed today (legacy %Y%m%d) is tolerated and >= the default
  # today-00:00 watermark; ST0004 (2020) is before it and drops off.
  assert_file_contains intent/todo.md "ST0003: shipped today"
  run grep -c "ST0004: shipped long ago" intent/todo.md
  assert_output "0"
}

@test "done --flush advances the watermark and empties DONE; real status untouched" {
  setup_todo_project
  run_intent todo update
  assert_file_contains intent/todo.md "ST0003: shipped today"
  run run_intent todo done --flush
  assert_success
  run grep -c "ST0003: shipped today" intent/todo.md
  assert_output "0"
  # flush clears the view, not the data -- ST0003 stays Completed on disk
  assert_file_exists "intent/st/COMPLETED/ST0003/info.md"
}

@test "done --prune emits the pruned items then flushes" {
  setup_todo_project
  run_intent todo update
  run run_intent todo done --prune
  assert_success
  assert_output_contains "ST0003: shipped today"
  run grep -c "ST0003: shipped today" intent/todo.md
  assert_output "0"
}

# ---- WP-03: CLI integration (guards -- satisfied by the auto-discovery help
# listing + the intent_<command> fall-through dispatch) ----

@test "todo is registered in the intent help listing" {
  local d
  d=$(create_test_project "Todo Help")
  cd "$d"
  run run_intent help
  assert_success
  assert_output_contains "todo"
}

@test "intent todo dispatches via bin/intent's default fall-through" {
  setup_todo_project
  # run_intent invokes the real bin/intent dispatcher; there is no dedicated
  # `todo)` case, so a working `todo update` proves the intent_<command> route.
  run run_intent todo update
  assert_success
  assert_file_exists "intent/todo.md"
  assert_file_contains intent/todo.md "## DOING"
}
