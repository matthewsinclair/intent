#!/usr/bin/env bats
# AT-24.5 / AC-24.5: every artefact that tells a model how to check for prior
# art names the INDEX as the route, names grep as the FALLBACK, and keeps the
# registry where a registry exists.
#
# A model picks a lookup from what its canon tells it, and the canon told it to
# grep -- in `IN-AG-HIGHLANDER-001` itself, the rule the index's own Highlander
# check exists to serve. An index that answers a question nobody is told to ask
# is an index nobody calls.
#
# THE POPULATION IS AUTHORED, AND THAT IS A RULING RATHER THAN A SHORTCUT (vc,
# 2026-09-12). A derived population would have to decide mechanically what
# "tells a model how to find code" means, and the estate has ~20 RULE.md files
# whose Detection heuristics grep as a CRITIC's mechanism rather than as advice
# -- sweeping those would change what the critics detect, which is a different
# criterion. So the six are named here, with the exclusions recorded in the
# commit that landed them.

load "../lib/test_helper.bash"

# The six. Each tells a model how to check for prior art, and each now routes
# through the index.
PRIOR_ART_ARTEFACTS=(
  "intent/plugins/claude/rules/agnostic/highlander/RULE.md"
  "intent/plugins/claude/skills/in-plan/SKILL.md"
  "intent/plugins/claude/skills/in-review/SKILL.md"
  "intent/plugins/claude/skills/in-standards/SKILL.md"
  "lib/templates/llm/_AGENTS.md"
  "lib/templates/llm/_CLAUDE.md"
)

# The two the templates generate. They are OUTPUT: they must carry the route,
# and they are never edited by hand.
GENERATED_ROOTS=(
  "AGENTS.md"
  "CLAUDE.md"
)

@test "every prior-art artefact routes through the index" {
  local missing=""
  local f
  for f in "${PRIOR_ART_ARTEFACTS[@]}" "${GENERATED_ROOTS[@]}"; do
    grep -q 'intent search --kind def' "${INTENT_HOME}/${f}" || missing="$missing $f"
  done
  [ -z "$missing" ] || fail "artefact(s) that do not name the index route:$missing"
}

@test "every prior-art artefact names grep as the fallback" {
  # The route is only trustable because the answer says when NOT to trust it.
  # An artefact naming the index and not the fallback tells a model to believe
  # an index that has just said it is incomplete.
  local missing=""
  local f
  for f in "${PRIOR_ART_ARTEFACTS[@]}" "${GENERATED_ROOTS[@]}"; do
    grep -q 'fall back to grep' "${INTENT_HOME}/${f}" || missing="$missing $f"
  done
  [ -z "$missing" ] || fail "artefact(s) that do not name the fallback:$missing"
}

@test "the registry is kept wherever it was named" {
  # `intent modules find` retires on hv's ruling (AC-20.6) and hv has not given
  # it, so this sweep must not have quietly retired it. The registry is searched
  # AS WELL, never INSTEAD.
  local missing=""
  local f
  for f in "${PRIOR_ART_ARTEFACTS[@]}"; do
    grep -q 'intent modules find' "${INTENT_HOME}/${f}" || missing="$missing $f"
  done
  [ -z "$missing" ] || fail "artefact(s) that lost the registry:$missing"
}

@test "the old wording is gone from the estate" {
  # The red arm of the sweep, kept as a test so the class cannot come back: the
  # Highlander rule said *grep for prior art in adjacent modules*, which named
  # the tool the index replaces, in the rule the index serves.
  local hits
  hits="$(grep -rn 'grep for prior art' "${INTENT_HOME}/intent/plugins" "${INTENT_HOME}/lib/templates" 2>/dev/null || true)"
  [ -z "$hits" ] || fail "the superseded wording is back: $hits"
}
