#!/usr/bin/env bats
# ST0088 WP-02: what the in-standards merge, the detrope move and the config-dir
# change each left behind in the skill corpus, held by the corpus itself. The
# other ST0088 criteria are held by the corpus tests they name.

load "../lib/test_helper.bash"

SKILLS="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills"

# ST0088 AT-02.1: in-standards is gone, in-essentials took its place, and no
# loaded file names in-standards. intent/docs/skills-triage.md is the one
# exception: it records the ruling that removed it. The index-first paragraph's
# authored population is prior_art_route.bats's to hold.
@test "in-standards is gone and nothing that loads names it" {
  [ ! -e "${SKILLS}/in-standards" ]
  [ -f "${SKILLS}/in-essentials/SKILL.md" ]
  local named
  named="$(cd "${INTENT_PROJECT_ROOT}" && grep -rl 'in-standards' intent/plugins lib/templates intent/docs CLAUDE.md AGENTS.md usage-rules.md | grep -v '^intent/docs/skills-triage.md$' || true)"
  [ -z "${named}" ] || { echo "still named in: ${named}"; false; }
}

# ST0088 AT-02.2: the detrope section has one home, in-detrope, and the author
# and content packs point at that skill rather than restating the section.
@test "the detrope section lives in in-detrope alone and the packs point at it" {
  local homes
  homes="$(cd "${SKILLS}" && grep -l '^## Two forms, one home$' */SKILL.md)"
  [ "${homes}" = "in-detrope/SKILL.md" ] || { echo "homes: ${homes}"; false; }
  local pack
  for pack in in-author-essentials in-content-essentials; do
    grep -q 'in-detrope' "${SKILLS}/${pack}/SKILL.md" || { echo "${pack} does not point at in-detrope"; false; }
  done
}

# ST0088 AT-02.3: a skill or one of its scripts that names Claude Code's config
# dir also reads CLAUDE_CONFIG_DIR, so a machine with it set is found. The home
# fallback is still named, so the check is per file, not per line. The sites
# outside the skills (the _CLAUDE.md template, critic-elixir's agent.md, the
# critic contract) are ST0088 WP-03's.
@test "every skill that names ~/.claude reads CLAUDE_CONFIG_DIR" {
  local files f bad=""
  files="$(grep -rlE '~/\.claude|\$HOME/\.claude|\$\{HOME\}/\.claude' "${SKILLS}" || true)"
  for f in ${files}; do
    grep -q 'CLAUDE_CONFIG_DIR' "${f}" || bad="${bad} ${f#"${SKILLS}"/}"
  done
  [ -z "${bad}" ] || { echo "names ~/.claude without CLAUDE_CONFIG_DIR:${bad}"; false; }
}
