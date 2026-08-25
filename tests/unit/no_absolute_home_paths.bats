#!/usr/bin/env bats
# Issue 0016: the project template baked an absolute INTENT_HOME path into every
# scaffolded project's .claude/settings.json.
#
# Hook resolution is a RUNTIME question and it was being answered at write time,
# so the resolved value froze into a tracked file: the hooks worked on exactly
# one machine, and a public repository published one user's home directory path.
#
# SCOPE, stated rather than assumed. These guard the files that FUNCTION as
# config -- what Intent ships to consumers, and this project's own live
# .claude/ stack.
#
# The original carve-out here named two exclusions. One of them is gone twice
# over: `intent treeindex` wrote absolute paths into intent/.treeindex/** by
# design, that tree stopped being TRACKED (issue 0018), and hv then retired the
# command outright (2026-08-15, executed 2026-08-25) so nothing writes there at
# all. The guard has nothing to look away from. What remains is historical
# prose -- completed steel threads and CHANGELOG entries quoting paths as
# record -- which is deliberately not rewritten, because it is the account of
# what was true at the time.

load "../lib/test_helper.bash"

# Any absolute path into a user home directory, on either platform layout.
HOME_PATH_RE='(/Users/[a-z]|/home/[a-z])'

@test "the shipped .claude/ template carries no absolute home path" {
  run bash -c "grep -rIE '$HOME_PATH_RE' '$INTENT_HOME/lib/templates/.claude/' || true"
  assert_output ""
}

@test "this project's own TRACKED .claude/ stack carries no absolute home path" {
  # The live instance the issue reported: our own settings.json carried the
  # maintainer's home path, and this repository is public. Scoped to TRACKED
  # files because publishing is the harm -- .claude/settings.local.json is the
  # per-machine permission allowlist, gitignored by design, and absolute paths
  # in it are correct rather than a leak.
  run bash -c "cd '$INTENT_HOME' && git ls-files -z .claude/ | xargs -0 grep -lIE '$HOME_PATH_RE' 2>/dev/null || true"
  assert_output ""
}

@test "settings.json needs no substitution and is byte-identical to the template" {
  # The whole point: nothing in this file is per-machine, so there is nothing to
  # substitute and no way for it to drift between checkouts.
  run bash -c "grep -c '\[\[' '$INTENT_HOME/lib/templates/.claude/settings.json' || true"
  assert_output "0"
  run diff -q "$INTENT_HOME/.claude/settings.json" "$INTENT_HOME/lib/templates/.claude/settings.json"
  assert_success
}

@test "the canon engine has no INTENT_HOME substitution left to reintroduce it" {
  # The SUBSTITUTION ARM, not any mention -- the code comment explaining why the
  # arm is gone necessarily names it, and a guard that cannot tell an
  # explanation from an implementation is worse than no guard.
  # grep -F on the exact substitution text. An ERE for this is a thicket of
  # escaped brackets and backslashes, and the first attempt was INVALID -- grep
  # errored, `|| true` swallowed it, and the guard could never fail. A check
  # that cannot fail is the defect this whole release is about.
  needle='INTENT_HOME\]\]'
  run bash -c "grep -cF '$needle' '$INTENT_HOME/intent/plugins/claude/bin/intent_claude_upgrade' || true"
  assert_output "0"
}

@test "a scaffolded project's hooks are portable and actually run" {
  project_dir=$(create_test_project "Hook Portability")
  cd "$project_dir"
  # The engine prompts before writing; feed it a yes and check what it wrote.
  run bash -c "yes | '$INTENT_BIN' claude upgrade --apply >/dev/null 2>&1; true"
  [ -f .claude/settings.json ]
  run bash -c "grep -cE '$HOME_PATH_RE' .claude/settings.json || true"
  assert_output "0"
  run diff -q .claude/settings.json "$INTENT_HOME/lib/templates/.claude/settings.json"
  assert_success
}

@test "the hook runner passes stdin and the exit code through untouched" {
  # The UserPromptSubmit gate signals "block this prompt" with exit 2
  # specifically, and reads the event JSON on stdin. A wrapper that swallowed
  # either would turn the gate into a no-op or into a hard block on every
  # prompt, so this is the contract that matters most about the indirection.
  run bash -c "printf '%s' '{\"prompt\":\"do something\"}' | CLAUDE_CODE_SESSION_ID=bats-none '$INTENT_BIN' claude hook require-in-session"
  [ "$status" -eq 2 ]

  # stdin is genuinely parsed: a slash command passes through.
  run bash -c "printf '%s' '{\"prompt\":\"/in-session\"}' | CLAUDE_CODE_SESSION_ID=bats-none '$INTENT_BIN' claude hook require-in-session"
  [ "$status" -eq 0 ]

  # And the sentinel releases it.
  mkdir -p /tmp/intent && touch /tmp/intent/in-session-bats-yes.sentinel
  run bash -c "printf '%s' '{\"prompt\":\"do something\"}' | CLAUDE_CODE_SESSION_ID=bats-yes '$INTENT_BIN' claude hook require-in-session"
  rm -f /tmp/intent/in-session-bats-yes.sentinel
  [ "$status" -eq 0 ]
}

@test "an unknown hook name is refused by name, not left to the shell" {
  run bash -c "'$INTENT_BIN' claude hook definitely-not-a-hook"
  assert_failure
  assert_output_contains "unknown hook"
}
