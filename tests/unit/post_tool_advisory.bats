#!/usr/bin/env bats
# The opt-in PostToolUse critic advisory (issue 0578): a clean critic run adds
# nothing to the model's context, and a run with findings hands them on.
# `intent` is a stub on PATH so each arm fixes the critic's status and output.

bats_require_minimum_version 1.5.0
load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/post-tool-advisory.sh"

setup() {
  require_tool jq "the advisory, which reads its payload with jq," || return 1
  ADV_PROJECT="$BATS_TEST_TMPDIR/proj"
  ADV_BIN="$BATS_TEST_TMPDIR/bin"
  mkdir -p "$ADV_PROJECT" "$ADV_BIN"
  printf 'post_tool_use_advisory: true\n' > "$ADV_PROJECT/.intent_critic.yml"
  printf '#!/bin/sh\necho hi\n' > "$ADV_PROJECT/x.sh"
  ADV_PAYLOAD='{"tool_name":"Edit","tool_input":{"file_path":"'"$ADV_PROJECT/x.sh"'"}}'
}

stub_critic() {
  # $1 = exit status, $2 = stdout. `intent lang of` answers $ADV_LANG, `shell`
  # unless an arm sets it (issue 0617), and every call is logged to
  # $ADV_BIN/calls so an arm can say which verbs the hook asked.
  printf '#!/bin/sh\nprintf "%%s\\n" "$*" >> "%s/calls"\nif [ "$1" = lang ] && [ "$2" = of ]; then printf "%%s" "%s"; exit 0; fi\nprintf "%%s\\n" "%s"\nexit %s\n' \
    "$ADV_BIN" "${ADV_LANG-shell}" "$2" "$1" > "$ADV_BIN/intent"
  chmod +x "$ADV_BIN/intent"
}

@test "the hook asks \`intent lang of\` for the file's language and keeps no table of its own (issue 0617)" {
  # Red on the hook that chose the critic from a `case` on the extension: it
  # never asks the binary, so no `lang of` call is logged.
  stub_critic 1 "x.sh:2 IN-SH-CODE-001 warning: FINDING-MARKER"
  run --separate-stderr env PATH="$ADV_BIN:$PATH" CLAUDE_PROJECT_DIR="$ADV_PROJECT" bash "$SCRIPT" <<< "$ADV_PAYLOAD"
  [ "$status" -eq 0 ]
  grep -qxF "lang of $ADV_PROJECT/x.sh" "$ADV_BIN/calls" || fail "the hook did not ask the binary: $(cat "$ADV_BIN/calls")"
  grep -q '^critic shell ' "$ADV_BIN/calls" || fail "the critic was not run for the language it was told: $(cat "$ADV_BIN/calls")"
  run grep -nE '^[[:space:]]*\*\.[a-z]+[|)]' "$SCRIPT"
  [ "$status" -eq 1 ] || fail "the hook still carries an extension table: $output"
}

@test "a file whose extension maps to no language runs no critic and adds nothing (issue 0617)" {
  ADV_LANG="" stub_critic 1 "x.sh:2 IN-SH-CODE-001 warning: FINDING-MARKER"
  run --separate-stderr env PATH="$ADV_BIN:$PATH" CLAUDE_PROJECT_DIR="$ADV_PROJECT" bash "$SCRIPT" <<< "$ADV_PAYLOAD"
  [ "$status" -eq 0 ]
  [ -z "$output" ] || fail "a file with no language reached the model: $output"
  if grep -q '^critic ' "$ADV_BIN/calls"; then
    fail "a critic ran for a file with no language: $(cat "$ADV_BIN/calls")"
  fi
}

@test "a clean critic run adds nothing, although the critic printed its census" {
  stub_critic 0 "critic: shell -- 3 of 3 rule(s) ASKED of this run; ok: no shell findings at severity >= warning across 1 file(s)"
  run --separate-stderr env PATH="$ADV_BIN:$PATH" CLAUDE_PROJECT_DIR="$ADV_PROJECT" bash "$SCRIPT" <<< "$ADV_PAYLOAD"
  [ "$status" -eq 0 ]
  [ -z "$output" ] || fail "a clean run reached the model: $output"
}

@test "a critic run with findings hands them to the model as additionalContext" {
  stub_critic 1 "x.sh:2 IN-SH-CODE-001 warning: FINDING-MARKER"
  run --separate-stderr env PATH="$ADV_BIN:$PATH" CLAUDE_PROJECT_DIR="$ADV_PROJECT" bash "$SCRIPT" <<< "$ADV_PAYLOAD"
  [ "$status" -eq 0 ]
  run jq -r '.hookSpecificOutput.additionalContext' <<< "$output"
  [[ "$output" == *"FINDING-MARKER"* ]] || fail "findings did not reach the model: $output"
}

@test "a critic that could not answer adds nothing and never blocks" {
  stub_critic 2 ""
  run --separate-stderr env PATH="$ADV_BIN:$PATH" CLAUDE_PROJECT_DIR="$ADV_PROJECT" bash "$SCRIPT" <<< "$ADV_PAYLOAD"
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}
