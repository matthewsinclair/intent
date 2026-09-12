#!/usr/bin/env bats
# AT-24.4: lib/templates/.claude/scripts/post-tool-symbol-context.sh
#
# The PostToolUse hook that appends the index's structural answer for the symbol
# a grep pattern named. Its three obligations, and one arm each: it appends the
# answer, it appends NOTHING when the index cannot answer for the paths that
# answer names, and it never blocks.
#
# THE FRESHNESS ARMS DRIVE A REAL INDEX RATHER THAN A FIXTURE ENVELOPE. A hand
# written JSON blob would test this file's jq and nothing about whether the
# envelope really carries what the rule reads -- which is the shape that let the
# search envelope ship write-only for as long as nothing read it back.

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/post-tool-symbol-context.sh"

# **THE BINARY UNDER TEST IS THE ONE THE HELPER NAMES, AND IT IS PUT ON `PATH`
# RATHER THAN CALLED BY PATH.** The hook is served from the install and resolves
# `intent` off `PATH` by design -- that is how a hook fix reaches every project
# through the installed Intent -- so a test that handed it an absolute path
# would be driving a different resolution from the shipped one. Prepending the
# built binary's own directory is what makes the hook run THIS build.
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-symctx-XXXXXX)"
  export HOME="${TEST_TEMP_DIR}/home"
  mkdir -p "$HOME"
  export PATH="$(dirname "$INTENT_BIN"):$PATH"
  PROJ="${TEST_TEMP_DIR}/proj"
  mkdir -p "$PROJ/src"
  cd "$PROJ" || exit 1
  "$INTENT_BIN" init SymCtx >/dev/null 2>&1
  "$INTENT_BIN" lang init rust >/dev/null 2>&1
  printf 'pub fn kestrel(a: u32, _b: u32) -> u32 { a }\n\npub fn caller() -> u32 { kestrel(1, 2) }\n' > src/lib.rs
  "$INTENT_BIN" index rebuild >/dev/null 2>&1
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Feed the hook one PostToolUse payload.
fire() {
  printf '%s' "$1" | CLAUDE_PROJECT_DIR="$PROJ" bash "$SCRIPT"
}

@test "the hook exists and shellcheck-clean bash parses it" {
  [ -f "$SCRIPT" ]
  bash -n "$SCRIPT"
}

@test "the sourced freshness predicate is NOT a hook" {
  # `hook_compat.rs` reads every *.sh in the scripts directory as a shipped hook
  # name, so the shared library carries a different extension. If it is ever
  # renamed to .sh this goes red before the Rust guard does.
  [ -f "${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/index-freshness.bash" ]
  [ ! -f "${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/index-freshness.sh" ]
  run "$INTENT_BIN" claude hook index-freshness
  [ "$status" -ne 0 ]
}

@test "a symbol-shaped pattern gets the index's definition and references" {
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kestrel"}}'
  [ "$status" -eq 0 ]
  [[ "$output" == *"src/lib.rs"* ]]
  [[ "$output" == *"def"* ]]
  [[ "$output" == *"ref"* ]]
}

@test "word anchors around the symbol are still a symbol" {
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"\\bkestrel\\b"}}'
  [ "$status" -eq 0 ]
  [[ "$output" == *"src/lib.rs"* ]]
}

@test "a pattern that is not one identifier is never answered" {
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kes.*el\\("}}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "a tool that is not Grep is never answered" {
  run fire '{"tool_name":"Read","tool_input":{"pattern":"kestrel"}}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "a symbol the index does not hold appends nothing" {
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"pelican"}}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "it appends NOTHING once a path in its own answer has moved" {
  # The control first: the same payload answers while the index is current, so a
  # silence below is the freshness rule and not a hook that never spoke.
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kestrel"}}'
  [ -n "$output" ]

  printf 'pub fn kestrel(a: u32, _b: u32) -> u32 { a }\n\npub fn caller() -> u32 { kestrel(1, 2) }\n\n// moved underneath the index\n' > "$PROJ/src/lib.rs"

  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kestrel"}}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]

  # And it speaks again once the index has caught up, so the rule is a gate
  # rather than a one-way latch.
  "$INTENT_BIN" index rebuild >/dev/null 2>&1
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kestrel"}}'
  [ -n "$output" ]
}

@test "the grep's own scope does not excuse a stale path in the answer" {
  # THE DEFECT THE FIRST BUILD HAD. Gating on the paths the GREP searched let a
  # grep confined to docs/ pass on the strength of docs/ being clean, and then
  # append hits in a src/ file that had moved. The subject of "complete for the
  # paths involved" is what the APPENDED ANSWER names.
  printf 'pub fn kestrel(a: u32, _b: u32) -> u32 { a }\n\npub fn caller() -> u32 { kestrel(1, 2) }\n\n// moved underneath the index\n' > "$PROJ/src/lib.rs"
  run fire '{"tool_name":"Grep","tool_input":{"pattern":"kestrel","path":"docs"}}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "it never blocks: exit 0 on rubbish, on no stdin payload, and outside a project" {
  run fire 'not json at all'
  [ "$status" -eq 0 ]

  run fire ''
  [ "$status" -eq 0 ]

  cd "$TEST_TEMP_DIR"
  run env CLAUDE_PROJECT_DIR="$TEST_TEMP_DIR" bash -c "printf '%s' '{\"tool_name\":\"Grep\",\"tool_input\":{\"pattern\":\"kestrel\"}}' | bash '$SCRIPT'"
  [ "$status" -eq 0 ]
}

@test "the shipped settings do NOT wire it, and the script says how to" {
  # **OFF BY DEFAULT IS A DECISION, NOT AN OMISSION.** Wiring it in the shipped
  # template turns it on for every project that takes the template, changing what
  # every session sees after every Grep -- so it follows `post-tool-advisory`:
  # shipped, documented, and switched on by the project rather than on its
  # behalf. This arm exists so that turning it on later is a deliberate edit
  # somebody makes here as well, rather than a line that slips into a template.
  run jq -e '.hooks | has("PostToolUse")' "${INTENT_PROJECT_ROOT}/lib/templates/.claude/settings.json"
  [ "$status" -ne 0 ]
  grep -q 'settings.local.json' "$SCRIPT"
  grep -q 'intent claude hook post-tool-symbol-context' "$SCRIPT"
}
