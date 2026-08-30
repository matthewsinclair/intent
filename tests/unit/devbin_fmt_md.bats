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

# Every root markdown file carrying a LANGUAGE-TAGGED fence -- the population
# `embeddedLanguageFormatting` acts on, discovered rather than named.
fenced_root_md() {
  local f
  for f in "$ROOT"/*.md; do
    [ -f "$f" ] || continue
    grep -qE '^```[a-z]+$' "$f" && echo "$f"
  done
}

@test "PREMISE: some root markdown still carries the embedded fence this protects" {
  # Without this the test below passes vacuously the day the fences are gone --
  # a green meaning "nothing to protect" reads identically to one meaning
  # "protected".
  #
  # **IT ASKS ABOUT THE POPULATION AND NOT ABOUT ONE FILE, BECAUSE NAMING THE
  # FILE IS WHAT BROKE IT.** This arm read `README.md` and went red on
  # 2026-08-29 when the front door stopped describing a retired tool and lost
  # its ```markdown fence with it. Nothing was wrong: the guard's subject had
  # MOVED, to `AGENTS.md` and `usage-rules.md`, and an instrument naming one
  # member of a population reports the member leaving as the property failing.
  # Discovered, so it re-aims itself the next time content moves.
  run fenced_root_md
  [ -n "$output" ]
}

@test "a fmt sweep leaves every fenced root markdown byte-identical" {
  # The behavioural check. Skipped rather than failed where prettier cannot be
  # reached, because a guard that reds on a missing toolchain is a guard someone
  # disables -- and the structural assertions above still run everywhere.
  command -v npx >/dev/null 2>&1 || skip "npx unavailable"
  npx --yes --no-install prettier --version >/dev/null 2>&1 ||
    skip "prettier unavailable offline"

  # The gate's own flags, from lib/cmd/fmt. Run WITHOUT --write, so the check
  # cannot itself be the thing that damages the file.
  #
  # **OVER THE WHOLE FENCED POPULATION**, for the reason the premise gives: a
  # sweep proven safe on one file says nothing about the file the content moved
  # to.
  # **THE ASSERTION IS ABOUT WHAT IS INSIDE THE FENCES, NOT ABOUT THE WHOLE
  # FILE, AND THE DISTINCTION IS THE ENTIRE POINT OF THIS GUARD.** Reflowing
  # prose and realigning tables is the gate's DECLARED JOB -- this estate forbids
  # hand-wrapped markdown -- so a file that has never been swept differing from
  # its swept form is the gate working. `usage-rules.md` differs by 86 lines of
  # table realignment right now and none of it is damage. A whole-file byte
  # comparison cannot tell those two apart, and passed here only because the file
  # it named happened to be swept already.
  #
  # **AND IT ESTABLISHES ITS OWN POSITIVE CONTROL BEFORE ASSERTING ANYTHING,
  # because without one it passes whether the guard is on or off.** Measured
  # 2026-08-30: with `embeddedLanguageFormatting` flipped to `auto` this arm
  # STILL PASSED. The root fences are ```bash, which prettier has no formatter
  # for, and one ```yaml block already in prettier's canonical form -- so there
  # was nothing in the corpus the protection could be shown to protect. That is
  # the premise arm's own failure one level down: it asks whether a fence EXISTS
  # when the property needs a fence prettier would CHANGE.
  #
  # So the corpus is asked first, by running the same sweep with the guard
  # explicitly OFF. If that moves nothing, the protection is real but currently
  # unwitnessable here, and this SKIPS with that reason rather than reporting a
  # green nobody earned.
  fenced() { awk '/^```[a-z]+$/{inf=1;next} /^```$/{inf=0} inf' "$1"; }
  sweep() { bash -c "cd '$ROOT' && npx --yes prettier --prose-wrap never --embedded-language-formatting=$1 '$2' 2>/dev/null"; }

  local f name demonstrable="" mangled=""
  while read -r f; do
    name="$(basename "$f")"
    sweep off "$name" > "$BATS_TEST_TMPDIR/on.md"
    sweep auto "$name" > "$BATS_TEST_TMPDIR/off.md"
    fenced "$f" > "$BATS_TEST_TMPDIR/orig.txt"
    fenced "$BATS_TEST_TMPDIR/off.md" > "$BATS_TEST_TMPDIR/off.txt"
    fenced "$BATS_TEST_TMPDIR/on.md" > "$BATS_TEST_TMPDIR/on.txt"

    diff -q "$BATS_TEST_TMPDIR/orig.txt" "$BATS_TEST_TMPDIR/off.txt" >/dev/null ||
      demonstrable="${demonstrable} ${name}"
    diff -q "$BATS_TEST_TMPDIR/orig.txt" "$BATS_TEST_TMPDIR/on.txt" >/dev/null ||
      mangled="${mangled} ${name}"
  done < <(fenced_root_md)

  if [ -n "$mangled" ]; then
    echo "a fmt sweep would rewrite content INSIDE a fenced block in:${mangled}"
    return 1
  fi
  [ -n "$demonstrable" ] ||
    skip "no root fence currently holds content prettier would reformat (```bash has no formatter, the one ```yaml block is already canonical) -- the config is still correct and this arm cannot witness it today"
}
