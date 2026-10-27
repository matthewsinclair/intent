#!/usr/bin/env bats
# Tests for the require-in-session.sh strict UserPromptSubmit gate.
#
# v2.11.5: covers the INTENT_SKIP_IN_SESSION_GATE env-var bypass added so
# non-interactive automation can spawn `claude -p`
# against an Intent project without being silently blocked. The bypass
# branch is the new code path; the slash-command and sentinel branches
# are smokes that the prepended bypass block does not regress them.

bats_require_minimum_version 1.5.0
load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/require-in-session.sh"

# Every gated arm runs under a temp HOME holding a stub releaser, because the
# gate fails open where $HOME/.claude/skills/in-session/scripts/release-gate.sh
# is absent (issue 0601); without the stub, every block arm would turn on
# whether this machine happens to have the skill installed.
stub_releaser_home() {
  export HOME="$BATS_TEST_TMPDIR/home"
  mkdir -p "$HOME/.claude/skills/in-session/scripts"
  : > "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
}

setup_gate() {
  stub_releaser_home
  GATE_SENTINEL_DIR="/tmp/intent"
  mkdir -p "$GATE_SENTINEL_DIR"
  GATE_SESSION_ID="bats-$(date +%s)-$$"
  GATE_SENTINEL="$GATE_SENTINEL_DIR/in-session-${GATE_SESSION_ID}.sentinel"
  rm -f "$GATE_SENTINEL"
}

teardown_gate() {
  rm -f "$GATE_SENTINEL"
}

# --------------------------------------------------------------------
# File presence and shape
# --------------------------------------------------------------------

@test "require-in-session.sh exists and is executable" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]
}

# --------------------------------------------------------------------
# Bypass branch (v2.11.5)
# --------------------------------------------------------------------

@test "INTENT_SKIP_IN_SESSION_GATE=1 short-circuits with exit 0 and empty stderr" {
  setup_gate
  # No sentinel exists, prompt is not a slash command -- normally would
  # block. Bypass must short-circuit before any other check.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE=1 \
    bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"hello"}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
  [ -z "$stderr" ]
  teardown_gate
}

@test "INTENT_SKIP_IN_SESSION_GATE=1 short-circuits with empty stdin" {
  # Treeindex pipes prompt content via heredoc/echo; the bypass must not
  # depend on payload shape.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE=1 bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
}

@test "empty INTENT_SKIP_IN_SESSION_GATE still gates" {
  setup_gate
  # ${VAR:-} treats empty as unset for [ -n ... ]; the bypass must be
  # value-bearing, not presence-bearing.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE= CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"/in-session must run"* ]]
  teardown_gate
}

@test "unset INTENT_SKIP_IN_SESSION_GATE still gates" {
  setup_gate
  # Smoke for the set -u + ${VAR:-} interaction. No bypass var at all.
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"/in-session must run"* ]]
  teardown_gate
}

@test "gate resolves session_id from CLAUDE_CODE_SESSION_ID, not the payload" {
  setup_gate
  # Identity is the env var, not payload.session_id. The Expected-sentinel
  # message must name the env id even when the payload carries a different one.
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"session_id":"payload-decoy","prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"in-session-${GATE_SESSION_ID}.sentinel"* ]]
  teardown_gate
}

# --------------------------------------------------------------------
# Existing pass-throughs (regression smokes for the prepended bypass)
# --------------------------------------------------------------------

@test "slash-command prompt passes through without bypass" {
  setup_gate
  # No sentinel, no bypass, but prompt is a slash command -- must exit 0.
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"/help"}'
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
  teardown_gate
}

@test "sentinel pass-through still works without bypass" {
  setup_gate
  touch "$GATE_SENTINEL"
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
  teardown_gate
}

# --------------------------------------------------------------------
# Without jq on PATH (issue 0563): the gate still recognises a slash
# command, so `/in-session` can release it, and a block names the missing jq.
# --------------------------------------------------------------------

no_jq_path() {
  NOJQ_BIN="$BATS_TEST_TMPDIR/nojq-bin"
  mkdir -p "$NOJQ_BIN"
  ln -sf "$(command -v cat)" "$NOJQ_BIN/cat"
  ln -sf "$(command -v bash)" "$NOJQ_BIN/bash"
}

@test "without jq, /in-session passes through" {
  setup_gate
  no_jq_path
  run --separate-stderr env PATH="$NOJQ_BIN" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    "$NOJQ_BIN/bash" "$SCRIPT" <<< '{"session_id":"x","prompt":"/in-session"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [ -z "$stderr" ]
  teardown_gate
}

@test "without jq, a slash command after whitespace in the JSON passes through" {
  setup_gate
  no_jq_path
  run --separate-stderr env PATH="$NOJQ_BIN" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    "$NOJQ_BIN/bash" "$SCRIPT" <<< '{"prompt" : "/help"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  teardown_gate
}

@test "without jq, a plain prompt is blocked and the block names the missing jq" {
  setup_gate
  no_jq_path
  run --separate-stderr env PATH="$NOJQ_BIN" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    "$NOJQ_BIN/bash" "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"/in-session must run"* ]]
  [[ "$stderr" == *"jq is not on PATH"* ]]
  teardown_gate
}

@test "without jq, prompt text quoting the key cannot forge a slash command" {
  setup_gate
  no_jq_path
  run --separate-stderr env PATH="$NOJQ_BIN" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    "$NOJQ_BIN/bash" "$SCRIPT" <<< '{"prompt":"say \"prompt\":\"/in-session\" now"}'
  [ "$status" -eq 2 ] || fail "a forged key passed: status $status"
  teardown_gate
}

@test "with jq, a plain prompt's block does not mention jq" {
  setup_gate
  require_tool jq "the jq-present block's wording" || return 1
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" != *"jq"* ]]
  teardown_gate
}

# --------------------------------------------------------------------
# No releaser on this machine (issue 0601): nothing inside the session
# could release the gate, so it fails open, says so once, and names the
# install verb. With the releaser present it blocks as before.
# --------------------------------------------------------------------

@test "without the releaser, a plain prompt passes and the notice names the install verb" {
  setup_gate
  rm -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [ -z "$stderr" ]
  [[ "$output" == '{"systemMessage":'* ]] || fail "not the JSON notice: $output"
  [[ "$output" == *'"hookEventName":"UserPromptSubmit"'* ]]
  [[ "$output" == *"intent claude skills install in-session"* ]]
  [ -f "$GATE_SENTINEL" ]

  # Said once: the second prompt in the session passes silently.
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"again"}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
  teardown_gate
}

@test "without the releaser, the notice is one valid JSON object" {
  setup_gate
  require_tool jq "the notice's JSON shape" || return 1
  rm -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash -c "bash '$SCRIPT' <<< '{\"prompt\":\"hello\"}' | jq -r '.hookSpecificOutput.additionalContext == .systemMessage'"
  [ "$status" -eq 0 ]
  [ "$output" = "true" ]
  teardown_gate
}

@test "with CLAUDE_CONFIG_DIR set, the gate finds the releaser there and blocks" {
  setup_gate
  local config="$BATS_TEST_TMPDIR/claude-config"
  mkdir -p "$config/skills/in-session/scripts"
  : > "$config/skills/in-session/scripts/release-gate.sh"
  rm -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
  run --separate-stderr env CLAUDE_CONFIG_DIR="$config" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ] || fail "status $status, output: $output"
  [[ "$stderr" == *"Expected sentinel:"* ]]
  [ ! -f "$GATE_SENTINEL" ]
  teardown_gate
}

@test "with CLAUDE_CONFIG_DIR set, a releaser only under HOME does not count" {
  setup_gate
  [ -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh" ]
  run --separate-stderr env CLAUDE_CONFIG_DIR="$BATS_TEST_TMPDIR/empty-config" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [[ "$output" == *"intent claude skills install in-session"* ]]
  teardown_gate
}

@test "with the releaser and no sentinel, a plain prompt is blocked" {
  setup_gate
  [ -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh" ]
  run --separate-stderr env CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [ -z "$output" ]
  [[ "$stderr" == *"Expected sentinel:"* ]]
  teardown_gate
}

# --------------------------------------------------------------------
# ST0086 AT-04.4
#
# The second duty: on a prompt that passes, the gate prints what
# `intent estate slots pending` prints, the asks waiting for the host's
# controller; on a block, or beside the fail-open notice, it prints none of
# it, and a pending verb that fails reaches neither the prompt nor its
# stderr. A stub `intent` first on PATH stands in for the verb.
# --------------------------------------------------------------------

stub_intent() {
  STUB_BIN="$BATS_TEST_TMPDIR/stub-bin"
  mkdir -p "$STUB_BIN"
  cat > "$STUB_BIN/intent" <<'EOS'
#!/usr/bin/env bash
if [ "$1 $2 $3" = "estate slots pending" ]; then
  out="${STUB_PENDING_OUT-asked: 01STUBASK heavy by molt-vc -- a judge}"
  [ -n "$out" ] && printf '%s\n' "$out"
  [ -n "${STUB_PENDING_ERR:-}" ] && printf '%s\n' "$STUB_PENDING_ERR" >&2
  exit "${STUB_PENDING_RC:-0}"
fi
exit 0
EOS
  chmod +x "$STUB_BIN/intent"
}

@test "a prompt that passes on its sentinel names the controller's waiting asks" {
  setup_gate
  stub_intent
  touch "$GATE_SENTINEL"
  run --separate-stderr env PATH="$STUB_BIN:$PATH" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [[ "$output" == *"asked: 01STUBASK heavy by molt-vc -- a judge"* ]] || fail "no asks: $output"
  teardown_gate
}

@test "a slash command names the controller's waiting asks too" {
  setup_gate
  stub_intent
  run --separate-stderr env PATH="$STUB_BIN:$PATH" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"/help"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [[ "$output" == *"asked: 01STUBASK"* ]] || fail "no asks: $output"
  teardown_gate
}

@test "a blocked prompt names no asks" {
  setup_gate
  stub_intent
  run --separate-stderr env PATH="$STUB_BIN:$PATH" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 2 ]
  [ -z "$output" ] || fail "asks beside a block: $output"
  teardown_gate
}

@test "the fail-open notice stays one JSON object while asks wait" {
  setup_gate
  stub_intent
  rm -f "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
  run --separate-stderr env PATH="$STUB_BIN:$PATH" CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" \
    bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ] || fail "status $status, stderr: $stderr"
  [[ "$output" == '{"systemMessage":'* ]] || fail "not the JSON notice: $output"
  [[ "$output" != *"01STUBASK"* ]] || fail "asks beside the notice: $output"
  teardown_gate
}

@test "a pending verb that fails neither blocks the prompt nor reaches it" {
  setup_gate
  stub_intent
  touch "$GATE_SENTINEL"
  run --separate-stderr env PATH="$STUB_BIN:$PATH" STUB_PENDING_OUT= \
    STUB_PENDING_ERR="error: the controller's record could not be read" STUB_PENDING_RC=1 \
    CLAUDE_CODE_SESSION_ID="$GATE_SESSION_ID" bash "$SCRIPT" <<< '{"prompt":"hello"}'
  [ "$status" -eq 0 ] || fail "status $status"
  [ -z "$output" ] || fail "text reached the prompt: $output"
  [ -z "$stderr" ] || fail "stderr: $stderr"
  teardown_gate
}
