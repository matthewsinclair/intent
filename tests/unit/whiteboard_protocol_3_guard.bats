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
# `ws hygiene` is the whiteboard's lint gate and did not check that a board's
# header block was readable at all: the one channel the protocol specifies as
# machine-read was the one nothing machine-checked. The second half was the
# design fork -- the tooling did not merely tolerate the invalid-YAML form, it
# PREFERRED it, because `ws list` stripped the delimiters without unescaping. So
# a board corrected to valid YAML displayed `\"` mid-prose while the incorrect
# one displayed properly. The ruling: it is not YAML, and hygiene enforces the
# rule that was actually implemented all along.

wb_project() {
  project_dir=$(create_test_project "WB Format Test")
  cd "$project_dir"
  run run_intent claude ws new hv
  assert_success
  run run_intent claude ws new cc
  assert_success
  BOARD="intent/whiteboard/cc/wip.md"
}

@test "a focus quoting a phrase is valid and renders with its quotes intact" {
  wb_project
  # The shape that made two of five live boards unparseable as YAML. Quoting a
  # phrase inside a hand-written prose field is the natural thing to write, and
  # nothing in the loop ever said not to -- because nothing should.
  python3 - "$BOARD" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read().replace(
  'focus: "(new workstream -- not yet started)"',
  'focus: "the counted body is the SENT body, not the "2x gap""')
open(p, 'w').write(s)
PY
  run run_intent claude ws hygiene
  assert_success

  # And it displays as written: the surrounding pair is a delimiter, the inner
  # quotes are content.
  run run_intent claude ws list
  assert_success
  # Anchored on `focus: ` so the surrounding DELIMITER is proven stripped: a
  # substring match alone passes happily on `focus: "the counted...""`.
  assert_output_contains 'focus: the counted body is the SENT body, not the "2x gap"'
  refute_output_contains '\"'
}

@test "hygiene rejects a value that is not a single line" {
  wb_project
  # A multi-line or block scalar is the shape that genuinely breaks every
  # line-oriented reader, and it was accepted in silence.
  python3 - "$BOARD" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read().replace('claims: []', 'claims: []\nnotes: >\n  a block scalar\n  spanning lines')
open(p, 'w').write(s)
PY
  run run_intent claude ws hygiene
  assert_failure
  assert_output_contains "line-oriented"
  assert_output_contains "a block scalar"
}

@test "hygiene rejects a board with no header block at all" {
  wb_project
  printf '# cc\n\nno header block here\n' > "$BOARD"
  run run_intent claude ws hygiene
  assert_failure
  assert_output_contains "no leading --- header block"
}

@test "a missing recommended key warns without failing the gate" {
  wb_project
  # Degraded is not malformed: a peer cannot run the active-peer test without
  # heartbeat_at, but hygiene must not newly fail boards that predate the rule.
  python3 - "$BOARD" <<'PY'
import sys
p = sys.argv[1]
s = "".join(l for l in open(p) if not l.startswith("heartbeat_at:"))
open(p, 'w').write(s)
PY
  run run_intent claude ws hygiene
  assert_success
  assert_output_contains "no heartbeat_at:"
}

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
