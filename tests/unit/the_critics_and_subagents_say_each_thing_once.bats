#!/usr/bin/env bats
# ST0088 WP-03: what the critic and subagent cull left behind in the canon
# subagents, held by the canon itself. Every arm iterates over what the
# directory holds, so a critic added later is held by the same arms.

load "../lib/test_helper.bash"

SUBAGENTS="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents"

# The code critics: every critic-<lang> but critic-prose, whose disciplines
# have no headless runner.
code_critics() {
  local d
  for d in "${SUBAGENTS}"/critic-*/; do
    d="$(basename "${d}")"
    [ "${d}" = "critic-prose" ] || printf '%s\n' "${d}"
  done
}

# ST0088 AT-03.2: the Diogenes test-spec handoff is critic-elixir's alone.
@test "only critic-elixir hands off to diogenes" {
  local found
  found="$(cd "${SUBAGENTS}" && grep -lF 'subagent_type="diogenes"' critic-*/agent.md || true)"
  [ "${found}" = "critic-elixir/agent.md" ] || { echo "handoff in: ${found}"; false; }
  found="$(cd "${SUBAGENTS}" && grep -l '^## Test-spec handoff' critic-*/agent.md || true)"
  [ "${found}" = "critic-elixir/agent.md" ] || { echo "handoff section in: ${found}"; false; }
}

# ST0088 AT-03.2: each code critic says the commit gate runs the headless
# runner for its own language, not the subagent.
@test "every code critic says the gate runs intent critic for its language" {
  local c bad=""
  for c in $(code_critics); do
    grep -qF "it runs the headless \`intent critic ${c#critic-}\`" "${SUBAGENTS}/${c}/agent.md" || bad="${bad} ${c}"
  done
  [ -z "${bad}" ] || { echo "no gate sentence:${bad}"; false; }
}

# ST0088 AT-03.2: a critic's description names the pack the installed tool
# serves, never a rules/<lang>/ directory the project does not have.
@test "no critic description names a rules directory" {
  local bad
  bad="$(cd "${SUBAGENTS}" && grep -l '^description:.*rules/' critic-*/agent.md || true)"
  [ -z "${bad}" ] || { echo "names rules/ in its description: ${bad}"; false; }
}

# ST0088 AT-03.2: a code critic offers test-check exactly when the tool serves
# its pack a test-category rule, so no critic carries a test line its pack
# cannot back.
@test "a code critic offers test-check exactly when its pack has test rules" {
  local c lang rules offers bad=""
  for c in $(code_critics); do
    lang="${c#critic-}"
    rules="$(run_intent claude rules list --lang "${lang}" | awk '$4 == "test"' | wc -l | tr -d ' ')"
    offers=0
    grep -q 'test-check' "${SUBAGENTS}/${c}/agent.md" && offers=1
    if [ "${rules}" -gt 0 ] && [ "${offers}" -eq 0 ]; then bad="${bad} ${c}(test rules, no test-check)"; fi
    if [ "${rules}" -eq 0 ] && [ "${offers}" -eq 1 ]; then bad="${bad} ${c}(test-check, no test rules)"; fi
  done
  [ -z "${bad}" ] || { echo "disagree:${bad}"; false; }
}

# ST0088 AT-03.2: the canon rows of `intent claude subagents list` are the
# directories the tree holds, and `intent critic --languages` is the code
# critics' languages.
@test "subagents list and the critic runner agree with the tree" {
  local tree listed
  tree="$(cd "${SUBAGENTS}" && for d in */; do printf '%s\n' "${d%/}"; done | sort)"
  listed="$(run_intent claude subagents list | awk '$3 == "canon" {print $1}' | sort)"
  [ "${tree}" = "${listed}" ] || { echo "tree: ${tree}"; echo "listed: ${listed}"; false; }
  tree="$(code_critics | sed 's/^critic-//' | sort)"
  listed="$(run_intent critic --languages | sort)"
  [ "${tree}" = "${listed}" ] || { echo "critics: ${tree}"; echo "runner: ${listed}"; false; }
}

# ST0088 AT-03.3: the intent subagent, socrates's uninstalled README.md and
# the metadata.json files nothing read are gone, and only agent.md remains in
# each canon subagent.
@test "each canon subagent holds agent.md and nothing the installer never reads" {
  [ ! -e "${SUBAGENTS}/intent" ]
  [ ! -e "${SUBAGENTS}/socrates/README.md" ]
  local extra
  extra="$(cd "${SUBAGENTS}" && find . -type f ! -name agent.md)"
  [ -z "${extra}" ] || { echo "beside agent.md: ${extra}"; false; }
}
