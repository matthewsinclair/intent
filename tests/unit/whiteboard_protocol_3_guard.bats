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

# ---- issue 0012: the header block is line-oriented, NOT YAML ---------------
#
# `ws hygiene` was the whiteboard's lint gate and did not check that a board's
# header block was readable at all: the one channel the protocol specifies as
# machine-read was the one nothing machine-checked. The second half was the
# design fork -- the tooling did not merely tolerate the invalid-YAML form, it
# PREFERRED it, because `ws list` stripped the delimiters without unescaping. So
# a board corrected to valid YAML displayed `\"` mid-prose while the incorrect
# one displayed properly. The ruling: it is not YAML, and hygiene enforced the
# rule that was actually implemented all along.
#
# ==========================================================================
# FOUR ARMS DROVE `ws hygiene` AND ARE DELETED BY ST0069 AC-14.12, WHICH
# REMOVES THREE CHECKS AND REPLACES THEM WITH TWO READERS
# ==========================================================================
#
# They asserted: a focus quoting a phrase is valid and renders with its quotes
# intact; a value that is not a single line is refused; a board with no header
# block at all is refused; a missing recommended key warns without failing.
# **Those three checks are performed by nothing else after this landing, and
# saying so is the point of this comment.**
#
# The accounting, rather than a claim that nothing is lost:
#
# - A board is a GENERATED VIEW from the cutover on, so its header block is the
#   renderer's output and not a hand-written record. A lint on a hand-written
#   format cannot fail on a rendered one without the renderer having failed
#   first, which its own criteria cover.
# - The header-reading `wb register` is the reader that matters for a board
#   still being MIGRATED: it refuses a header it cannot read, at the moment the
#   board crosses into the model, which is later than `hygiene` fired and is
#   the crossing that actually matters.
# - The two whiteboard guards (`whiteboard-clock-guard.sh`,
#   `whiteboard-header-guard.sh`) run from the pre-commit gate and are the two
#   that ever caught anything at HEAD. They stay, and they are why the escape
#   forms are still refused.
#
# **The residue is the hand-authored board in an estate that has not migrated:
# for that board these three checks are gone, and the hook template that would
# carry them is ruled off WP-14's path.** Accepted with the reason written down
# rather than left for a later reader to reconstruct from an absence.

@test "the skill declares the block NOT YAML and the tool agrees" {
  # The word was the only thing out of step with the implementation, so the
  # canon must not reintroduce it. SKILL.md itself has to change for the skills
  # sync to propagate (checksums cover SKILL.md, not scripts beside it).
  run grep -c "The header block is NOT YAML" "$INTENT_HOME/intent/plugins/claude/skills/in-whiteboard/SKILL.md"
  assert_output "1"
  run bash -c "grep -c '^\`\`\`yaml' '$INTENT_HOME/intent/plugins/claude/skills/in-whiteboard/SKILL.md' || true"
  assert_output "0"
  # And the reader strips the display delimiter exactly once, in one place.
  run bash -c "grep -c \"sed 's/^\\\"//; s/\\\"\\\$//'\" '$INTENT_HOME/intent/plugins/claude/bin/intent_claude_cwi' || true"
  assert_output "0"
}
