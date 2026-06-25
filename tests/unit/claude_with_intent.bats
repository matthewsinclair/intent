#!/usr/bin/env bats
# ST0047: behavioural tests for the promoted `intent claude start|ws`
# (claude_with_intent), ported from the Baize prototype's cwi_test.sh.
#
# Drives the command through the real `intent claude` dispatch against a scratch
# whiteboard (CWI_WB) and the launch via the CWI_DRY_RUN seam -- never spawns a
# real claude, never touches a real board. @test names are cited by AT-0N.M in
# intent/st/ST0047/acceptance.md.

load "../lib/test_helper.bash"

INTENT="$INTENT_BIN_DIR/intent"

# Override the shared setup(): the cwi command resolves the CURRENT project via
# find_project_root, so the test must run inside a project. Provision a throwaway
# Intent project + a scratch whiteboard root (CWI_WB).
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-cwi-XXXXXX)"
  PROJECT="$(create_test_project "CWI Test" "$TEST_TEMP_DIR/proj")"
  cd "$PROJECT" || exit 1
  WB="$TEST_TEMP_DIR/wb"
  mkdir -p "$WB"
}

teardown() {
  cd "$INTENT_HOME" 2>/dev/null || true
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

ws() { env CWI_WB="$WB" "$INTENT" claude ws "$@"; }

# ---- WP-01: ws new + ws list ----

@test "AT-01.1 ws new scaffolds a protocol-3.0 node" {
  ws new hv
  ws new cc
  assert_file_exists "$WB/cc/wip.md"
  assert_file_exists "$WB/cc/.history/.gitkeep"
  assert_file_exists "$WB/cc/inbox.hv.md"
  assert_file_exists "$WB/hv/inbox.cc.md"
  grep -q '^node: cc' "$WB/cc/wip.md"
  grep -q '^# inbox: hv -> cc' "$WB/cc/inbox.hv.md"
  grep -qF '_(empty)_' "$WB/cc/inbox.hv.md"
}

@test "AT-01.4 hv is workstream zero (active) by default; working nodes paused" {
  ws new hv
  grep -q '^status: active' "$WB/hv/wip.md"
  ws new cc
  grep -q '^status: paused' "$WB/cc/wip.md"
}

@test "AT-01.2 ws new refuses an existing wsid with no mutation" {
  ws new cc
  before="$(cat "$WB/cc/wip.md")"
  run ws new cc
  assert_failure
  [ "$(cat "$WB/cc/wip.md")" = "$before" ]
}

@test "AT-01.3 ws new rejects invalid ids with no partial scaffold" {
  run ws new BadCaps
  assert_failure
  [ ! -e "$WB/BadCaps" ]
  run ws new 'a b'
  assert_failure
  run ws new waytoolongworkstream
  assert_failure
  run ws new back-end
  assert_success
  assert_directory_exists "$WB/back-end"
}

@test "AT-01.5 ws list prints one line per node from frontmatter, no writes" {
  ws new hv
  ws new cc
  before="$(find "$WB" -type f -exec cksum {} + | sort)"
  run ws list
  assert_success
  echo "$output" | grep -qE '^hv '
  echo "$output" | grep -qE '^cc '
  after="$(find "$WB" -type f -exec cksum {} + | sort)"
  [ "$before" = "$after" ]
}

# ---- WP-02: start (launch + provision-if-absent) ----

@test "AT-02.1 start assembles the verified claude argv (dry-run)" {
  ws new hv
  ws new cc
  run env CWI_WB="$WB" CWI_DRY_RUN=full "$INTENT" claude start cc
  assert_success
  assert_output_contains 'effort max --permission-mode auto'
  assert_output_contains '/in-session'
}

@test "AT-02.2 compose_ctx carries identity + pickup + plan instruction, not the board" {
  ws new hv
  ws new cc
  run env CWI_WB="$WB" CWI_DRY_RUN=full "$INTENT" claude start cc
  assert_success
  assert_output_contains 'YOU ARE: cc'
  assert_output_contains 'pickup cc'
  assert_output_contains 'detailed plan'
}

@test "AT-02.3 provision-if-absent: n exits clean (no mutation), y scaffolds then launches" {
  ws new hv
  run env CWI_WB="$WB" "$INTENT" claude start ghost <<< "n"
  assert_failure
  [ ! -e "$WB/ghost" ]
  run env CWI_WB="$WB" CWI_DRY_RUN=1 "$INTENT" claude start newbie <<< "y"
  assert_success
  assert_directory_exists "$WB/newbie"
}

# ---- WP-03: archive + hygiene ----

@test "AT-03.1 ws archive retires a node, history intact, gone from list" {
  ws new hv
  ws new cc
  ws archive cc
  [ ! -d "$WB/cc" ]
  assert_file_exists "$WB/.archived/cc/wip.md"
  assert_file_exists "$WB/.archived/cc/.history/.gitkeep"
  run ws list
  assert_success
  ! echo "$output" | grep -qE '^cc '
}

@test "AT-03.2 ws hygiene flags a corrupted fixture non-zero with a report" {
  ws new hv
  ws new cc
  rm -f "$WB/cc/.history/.gitkeep"
  run ws hygiene
  assert_failure
  assert_output_contains 'MISSING'
}

@test "AT-03.3 ws hygiene leaves DOING content untouched" {
  ws new hv
  ws new cc
  before="$(cat "$WB/cc/wip.md")"
  run ws hygiene
  [ "$(cat "$WB/cc/wip.md")" = "$before" ]
}

@test "AT-03.4 ws hygiene passes a clean board (zero exit, no mutation)" {
  ws new hv
  ws new cc
  before="$(find "$WB" -type f -exec cksum {} + | sort)"
  run ws hygiene
  assert_success
  assert_output_contains 'hygiene: ok'
  after="$(find "$WB" -type f -exec cksum {} + | sort)"
  [ "$before" = "$after" ]
}

# ---- WP-04: promotion (dispatch + skill SSOT) ----

@test "AT-04.1 invocable as intent claude start|ws through the dispatch" {
  ws new hv
  run ws list
  assert_success
  assert_output_contains 'hv'
  run env CWI_WB="$WB" CWI_DRY_RUN=1 "$INTENT" claude start hv
  assert_success
  assert_output_contains 'effort max'
}

@test "AT-04.2 in-whiteboard skill points at the script with no lazy-inbox drift (SSOT)" {
  skill="$INTENT_HOME/intent/plugins/claude/skills/in-whiteboard/SKILL.md"
  grep -q 'intent claude ws new' "$skill"
  ! grep -qE 'not pre-created|never pre-seeded' "$skill"
}
