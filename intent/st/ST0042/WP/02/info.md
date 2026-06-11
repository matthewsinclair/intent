---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: "Finish rules-path drift + mechanical guard"
scope: M
status: WIP
---

# WP-02: Finish rules-path drift + mechanical guard

## Objective

Complete the v2.11.11 rules-path fix class (theme T2): every propagated or generated artefact must describe rule access via the `intent claude rules` CLI (or correctly qualify install-local paths), and a mechanical guard must prevent a fourth regression of this class.

## Evidence

- F-PLG-1 (HIGH, confirmed): nine canon skills cite the dead local path `intent/plugins/claude/rules/...` -- `in-session`, `in-standards`, `in-review`, `in-ash-ecto-essentials`, `in-elixir-essentials`, `in-elixir-testing`, `in-tca-init`, `in-tca-synthesize`, `in-tca-finish` (eg `in-session/SKILL.md:42,47-50`).
- F-TPL-1 (HIGH, confirmed): `[[LANG]]` placeholder in `lib/templates/llm/_usage-rules.md:25,66` is substituted by no generator (`canon_substitute_placeholders`, `intent_claude_upgrade:228-238`, does not cover it) and ships verbatim to consumers.
- F-TPL-12 (MEDIUM, reported): generated `AGENTS.md`, `_CLAUDE.md`, `templates/elixir/AGENTS.md` reference `intent/docs/working-with-llms.md` etc. as project-local; they exist only in the Intent install.
- F-PLG-12 (LOW, reported): `lib/templates/_intent_critic.yml:9` points at a sample file absent in consumers.

## Deliverables

- All nine skills' SKILL.md route rule access through `intent claude rules list` / `show` (no local-path references); fleet propagation via the SKILL.md-checksum sync.
- `[[LANG]]` either substituted by the generator or removed from `_usage-rules.md`.
- Doc references in generated guidance qualified ("at the Intent install") or swapped to CLI access, matching `_usage-rules.md`'s correct form.
- `.intent_critic.yml` template comment fixed.
- Mechanical guard: a bats test that greps every propagated/generated artefact (skills, templates, agent templates, critic subagents) for the dead local path and fails on any hit.
- `tests/unit/in_session_skill.bats:70-73` (asserts the path is present) updated to assert the CLI form.

## Acceptance Criteria

- [ ] Guard test exists, is red against pre-fix tree (spot-proven), green after.
- [ ] Local-path grep over propagated/generated surfaces returns only legitimate repo-local contributor docs (excluded from the guard by design).
- [ ] Full bats suite green; skills re-synced via `intent claude skills sync`.

## Dependencies

- Sequenced after WP-04 (AGENTS.md generator fixes) to avoid touching the generator twice.
