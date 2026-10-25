#!/usr/bin/env bats
# Every skill /in-session loads fits inside what a compact carries.
#
# A compact carries each skill invoked before it up to 20,000 characters, and a longer one arrives
# truncated, so /in-session has to load it again after every compact, in every node of every project
# that runs it. The set is in-session itself plus its chains_to list, read from its own frontmatter so
# this arm follows what in-session loads rather than a second list of it. Bytes are counted, not
# characters: a byte count is never smaller than the character count, so the check is never weaker
# than the limit, and it needs no locale.

load "../lib/test_helper.bash"

SKILLS="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills"
LIMIT=20000

startup_skills() {
  printf '%s\n' in-session
  sed -n '/^chains_to:/,/^---$/p' "$SKILLS/in-session/SKILL.md" | grep -oE '"in-[a-z-]+"' | tr -d '"'
}

# ST0088 AT-02.5: the trims keep every skill /in-session chains to under the cap.
@test "every startup skill fits the characters a compact carries" {
  local skills
  skills="$(startup_skills)"
  # The control: a frontmatter this cannot read would leave in-session alone and pass for nothing.
  [ "$(printf '%s\n' "$skills" | wc -l)" -gt 1 ] || fail "read no chains_to list from in-session: $skills"
  local over="" s bytes
  while IFS= read -r s; do
    [ -f "$SKILLS/$s/SKILL.md" ] || fail "in-session chains to $s, which has no SKILL.md"
    bytes="$(wc -c <"$SKILLS/$s/SKILL.md" | tr -d ' ')"
    [ "$bytes" -lt "$LIMIT" ] || over="$over $s=$bytes"
  done <<<"$skills"
  [ -z "$over" ] || fail "startup skill(s) at or over $LIMIT bytes, so a compact truncates them:$over"
}
