#!/usr/bin/env bats
# ST0045: mechanical guards on the Whiteboard Protocol 3.0 rollout.
#
# AT-02.1 (AC-02.1): the shipped in-whiteboard/SKILL.md carries no LIVE reference
#   to the retired 2.0 flat-file model -- a 2.0 token is allowed only on a line
#   that names it as retired/superseded/legacy/2.0.
# AT-02.2 (AC-02.2): the chaining skills reference the 3.0 subcommands, and no
#   shipped chaining skill / canon narrative doc hard-codes the flat per-stream
#   model. Red-first: in-session/in-finish/working-with-llms.md describe the 2.0
#   "stream files" model until this thread rewrites them to per-node boards.
# @test names are cited by AT-02.1 / AT-02.2 in intent/st/ST0045/acceptance.md.

load "../lib/test_helper.bash"

# 2.0 flat-file tokens. A line carrying one is a violation UNLESS it also carries
# a retired-context marker (retired | supersede | legacy | 2.0). The asks.md token
# is boundary-guarded so it does not match the substring in `tasks.md`.
TWO_OH_TOKENS='[^[:alpha:]]asks\.md|lamplight\.md|cookies\.md|per-stream|stream file|other-stream|<stream>\.md'
RETIRED_MARKER='retired|supersede|legacy|2\.0'

@test "in-whiteboard SKILL.md has no live 2.0 references" {
  skill="$INTENT_HOME/intent/plugins/claude/skills/in-whiteboard/SKILL.md"
  offenders="$(grep -nE "$TWO_OH_TOKENS" "$skill" | grep -viE "$RETIRED_MARKER" || true)"
  if [ -n "$offenders" ]; then
    echo "Live 2.0 references in in-whiteboard/SKILL.md:"
    echo "$offenders"
    return 1
  fi
}

@test "chaining skills reference 3.0 subcommands and no shipped doc hard-codes the flat-file model" {
  skills="$INTENT_HOME/intent/plugins/claude/skills"
  docs="$INTENT_HOME/intent/docs"
  # The chaining skills invoke the 3.0 subcommands by name.
  grep -q '/in-whiteboard pickup' "$skills/in-session/SKILL.md"
  grep -q '/in-whiteboard release' "$skills/in-finish/SKILL.md"
  # No shipped chaining skill / canon doc describes the retired flat per-stream model.
  offenders="$(grep -nE "$TWO_OH_TOKENS" \
    "$skills/in-session/SKILL.md" \
    "$skills/in-finish/SKILL.md" \
    "$docs/working-with-llms.md" \
    | grep -viE "$RETIRED_MARKER" || true)"
  if [ -n "$offenders" ]; then
    echo "Flat-file 2.0 model hard-coded in a shipped skill/doc:"
    echo "$offenders"
    return 1
  fi
}
