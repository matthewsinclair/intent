#!/usr/bin/env bats
# ST0088 WP-03: what the critic and subagent cull left behind in the canon
# subagents, held by the canon itself. Every arm iterates over what git tracks
# in the directory, so a critic added later is held by the same arms and a file
# git ignores on a working machine, eg Finder's .DS_Store, judges nothing.

load "../lib/test_helper.bash"

SUBAGENTS_REL="intent/plugins/claude/subagents"
SUBAGENTS="${INTENT_PROJECT_ROOT}/${SUBAGENTS_REL}"

# Every tracked file under the subagents directory, relative to it.
tracked() {
  git -C "${INTENT_PROJECT_ROOT}" ls-files -- "${SUBAGENTS_REL}" | sed "s|^${SUBAGENTS_REL}/||"
}

# Each tracked subagent: a directory holding a tracked file.
subagents() {
  tracked | grep '/' | sed 's|/.*||' | sort -u
}

# The code critics: every critic-<lang> but critic-prose, whose disciplines
# have no headless runner.
code_critics() {
  subagents | grep '^critic-' | grep -vx 'critic-prose'
}

# The tracked critic-*/agent.md files whose text matches the pattern.
critics_matching() {
  local f
  for f in $(subagents | grep '^critic-'); do
    grep -q "$@" "${SUBAGENTS}/${f}/agent.md" && printf '%s\n' "${f}/agent.md"
  done
  true
}

# ST0088 AT-03.2: the Diogenes test-spec handoff is critic-elixir's alone.
@test "only critic-elixir hands off to diogenes" {
  local found
  found="$(critics_matching -F 'subagent_type="diogenes"')"
  [ "${found}" = "critic-elixir/agent.md" ] || { echo "handoff in: ${found}"; false; }
  found="$(critics_matching '^## Test-spec handoff')"
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
  bad="$(critics_matching '^description:.*rules/')"
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
# subagents git tracks, and `intent critic --languages` is the code critics'
# languages.
@test "subagents list and the critic runner agree with the tree" {
  local tree listed
  tree="$(subagents)"
  listed="$(run_intent claude subagents list | awk '$3 == "canon" {print $1}' | sort)"
  [ "${tree}" = "${listed}" ] || { echo "tree: ${tree}"; echo "listed: ${listed}"; false; }
  tree="$(code_critics | sed 's/^critic-//' | sort)"
  listed="$(run_intent critic --languages | sort)"
  [ "${tree}" = "${listed}" ] || { echo "critics: ${tree}"; echo "runner: ${listed}"; false; }
}

# ST0088 AT-03.3: the intent subagent, socrates's uninstalled README.md and
# the metadata.json files nothing read are gone, and git tracks nothing in a
# canon subagent but its agent.md.
@test "each canon subagent holds agent.md and nothing the installer never reads" {
  local extra
  extra="$(tracked | grep -v '^[^/]*/agent\.md$' || true)"
  [ -z "${extra}" ] || { echo "tracked beside agent.md: ${extra}"; false; }
  ! subagents | grep -qx 'intent' || { echo "the intent subagent is tracked"; false; }
}
