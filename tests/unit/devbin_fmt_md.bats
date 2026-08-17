#!/usr/bin/env bats
# `int fmt md` must not reach INSIDE a fenced code block.
#
# WHAT WENT WRONG, MEASURED 2026-08-17 by running the gate rather than reading it.
# `lib/cmd/fmt` runs `prettier --prose-wrap never --write` over the root markdown.
# Unwrapping PROSE is the point -- this estate forbids hand-wrapped markdown, so a
# file that has never been swept getting reflowed is the gate doing its declared
# job, not damage.
#
# But prettier also formats code INSIDE fenced blocks, by language. README.md
# carries a ```markdown fence whose content is an illustrative dialogue:
#
#   You: "Help me optimize the user service"
#   LLM: "What does the user service do? ..."
#   [You spend 10 minutes explaining...]
#
# prettier treated that as markdown to be reflowed and collapsed all three lines
# into one, destroying the example the block exists to show. That is not a format
# change, it is a formatter mangling user-facing content -- and it would have
# landed on the next `int fmt all` anyone ran before a commit.
#
# THE FIX IS A PROJECT CONFIG, NOT A PATCH TO THE VENDORED GATE.
# `.prettierrc.json` sets `embeddedLanguageFormatting: off`, which prettier reads
# on its own. `lib/cmd/fmt` is a file devbin OWNS (it is in
# bin/.devbin/manifest.sha256), so fixing it there would have been a second local
# fork of vendored code carrying a project decision that is not devbin's business.
# The config is the seam that already exists.

load "../lib/test_helper.bash"

ROOT="${INTENT_PROJECT_ROOT}"
CONFIG="${ROOT}/.prettierrc.json"
README="${ROOT}/README.md"

@test "the prettier config exists and turns embedded formatting off" {
  [ -f "$CONFIG" ]
  run jq -r '.embeddedLanguageFormatting' "$CONFIG"
  assert_success
  assert_output "off"
}

@test "PREMISE: README.md still carries the embedded fence this protects" {
  # Without this the test below passes vacuously the day the fence is removed --
  # a green that means "nothing to protect" reads identically to one meaning
  # "protected". If this ever reds, the guard above may genuinely be unnecessary.
  run grep -c '^```markdown$' "$README"
  assert_success
  [ "$output" -ge 1 ]
}

@test "a fmt sweep leaves README.md byte-identical" {
  # The behavioural check. Skipped rather than failed where prettier cannot be
  # reached, because a guard that reds on a missing toolchain is a guard someone
  # disables -- and the structural assertions above still run everywhere.
  command -v npx >/dev/null 2>&1 || skip "npx unavailable"
  npx --yes --no-install prettier --version >/dev/null 2>&1 ||
    skip "prettier unavailable offline"

  # The gate's own flags, from lib/cmd/fmt. Run WITHOUT --write, so the check
  # cannot itself be the thing that damages the file.
  run bash -c "cd '$ROOT' && npx --yes prettier --prose-wrap never README.md 2>/dev/null | diff -q - README.md"
  assert_success
}
