#!/usr/bin/env bats
# ST0047: behavioural tests for `intent claude start` (claude_with_intent),
# ported from the Baize prototype's cwi_test.sh.
#
# Drives the command through the real `intent claude` dispatch in a throwaway
# project whose nodes are registered through `intent wb register`, and the
# launch via the CWI_DRY_RUN seam -- never spawns a real claude, never touches a
# real board. @test names are cited by AT-0N.M in ST0047's contract.
#
# ==========================================================================
# THE WS ARMS WERE DELETED BY ST0069 AC-14.12, AND WHAT IS LEFT IS WHAT ONLY
# THIS FILE CAN SEE
# ==========================================================================
#
# `intent claude ws <new|list|archive|hygiene>` managed the whiteboard as
# FILES. WP-14 makes a node a ROW in the store, so the family is retired in
# `dispatch-table.json` and every arm that drove it -- AT-01.1 to AT-01.5,
# AT-03.1 to AT-03.4 -- drives a spelling that now refuses at rc=2. They are
# deleted rather than re-pointed: their subject is gone, not moved, and
# `intent wb`'s own criteria (ST0069 WP-14) are where scaffolding, listing and
# archiving are tested now, against the model rather than against a directory.
#
# Others went for Highlander rather than for retirement:
#
# - **AT-02.3** drove `start` on an absent node, which used to PROMPT to
#   scaffold one and now refuses. The refusal lives beside the door it belongs
#   to, in `claude_cwi_door.rs`.
# - **AT-04.1** drove the dispatch, which `claude_cwi_door.rs` drives with a
#   fixture install rather than this file's real one.
# - **AT-04.2** asserted the in-whiteboard skill names `intent claude ws new`.
#   **It was already red at HEAD and nothing reported it**: AC-14.10 rewrote
#   that skill and took the verb out, which is the requirement being MET. The
#   property it was reaching for -- no shipped skill names a verb the surface
#   does not have -- is `no_skill_names_an_unshipped_verb.rs`, derived from the
#   register rather than from a hand-written grep, and a second reader beside
#   it would go stale the same way this one did.
#
# What survives is the composed CONTEXT, which no Rust arm reads in full:
# `CWI_DRY_RUN=full` is the only surface that prints it, and its content is the
# whole point of the launcher.
#
# Since issue 0609 the door resolves the project and reads the node from the
# store, then runs the launcher from the project root with the node's display
# name, so the nodes here are registered through the verb rather than written
# as rendered boards the launcher used to read.

load "../lib/test_helper.bash"

INTENT="$INTENT_BIN"

# Override the shared setup(): `claude start` resolves the CURRENT project and
# reads the node from its store, so the test runs inside a throwaway Intent
# project with its nodes registered, under a sandboxed HOME.
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-cwi-XXXXXX)"
  setup_fake_home
  PROJECT="$(create_test_project "CWI Test" "$TEST_TEMP_DIR/proj")"
  cd "$PROJECT" || exit 1
}

teardown() {
  teardown_fake_home
  cd "$INTENT_HOME" 2>/dev/null || true
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# A node on the board, registered through the verb that puts one there.
register_node() { # $1 moniker  $2 display name  $3 role
  run "$INTENT" wb register "$1" --name "$2" --role "$3"
  assert_success
}

# ---- WP-02: start (launch + composed context) ----

@test "AT-02.1 start assembles the verified claude argv (dry-run)" {
  register_node hv "Hypervisor" hypervisor
  register_node cc "Control Claude" control
  run env CWI_DRY_RUN=full "$INTENT" claude start cc
  assert_success
  assert_output_contains 'effort max --permission-mode auto'
  assert_output_contains '/in-session'
}

@test "AT-02.2 compose_ctx carries identity + pickup + plan instruction, not the board" {
  register_node hv "Hypervisor" hypervisor
  register_node cc "Control Claude" control
  run env CWI_DRY_RUN=full "$INTENT" claude start cc
  assert_success
  assert_output_contains 'YOU ARE: cc (Control Claude)'
  assert_output_contains 'pickup cc'
  assert_output_contains 'detailed plan'
}
