---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-04
title: "Template and generator updates"
scope: Small
status: Not Started
---

# WP-04: Template and generator updates

## Objective

Flip every reference to `.intent/` in `lib/templates/` and in any code path that _generates_ prose (the AGENTS.md generator, the `_CLAUDE.md` template, the `_usage-rules.md` template, hook templates, ext-seed templates) so that downstream-project content emitted by Intent uses `intent/.config/`.

## Context

`lib/templates/` is the single source of truth for content emitted into downstream projects (per Highlander rule #6 in CLAUDE.md). Recon found 10 `.intent/` literals in `lib/templates/` plus several generator paths that string-build `.intent/...` from constants. Specific surface:

- `lib/templates/llm/_CLAUDE.md` -- file map line `- \`.intent/\` -- configuration and metadata.` (line 25 of the WP09 template -- needs flipping).
- `lib/templates/llm/_usage-rules.md` -- the Project Structure tree includes `.intent/` -- flips.
- `lib/templates/.claude/scripts/*.sh` -- hook scripts that may reference `.intent/` (unlikely but check).
- `lib/templates/_intent_critic.yml` -- doesn't reference `.intent/` (its own filename is unrelated).
- `lib/templates/llm/_MODULES.md`, `_DECISION_TREE.md`, `_ARCHETYPES.md`, `_DEPENDENCY_GRAPH.md` -- check each.
- `lib/templates/ext-seeds/worker-bee/` -- subagent + skill seed files; check for `.intent/` references.

Generator surface (paths that _write_ content to downstream projects):

- `intent/plugins/agents/bin/intent_agents` -- AGENTS.md generator. Currently emits a file map; the file-map block must reference `intent/.config/`.
- `intent/plugins/claude/bin/intent_claude_upgrade` -- canon-installer; references several paths in its diagnostic prints. Flip any `.intent/` strings.
- `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- if it constructs paths.

WP04's job: change the templates so a _fresh_ `intent claude upgrade --apply` on a v2.10.0 project emits prose referencing the new layout. WP08 then dogfoods this by running the upgrade on Intent itself.

## Deliverables

1. **`lib/templates/llm/_CLAUDE.md`** file map line flipped: `- \`intent/.config/\` -- configuration and metadata.`
2. **`lib/templates/llm/_usage-rules.md`** Project Structure tree updated to show `intent/.config/` (with `cache/`, `backup/`, `config.json` underneath as appropriate).
3. **`lib/templates/llm/_MODULES.md`, `_DECISION_TREE.md`, `_ARCHETYPES.md`, `_DEPENDENCY_GRAPH.md`** -- any `.intent/` references flipped.
4. **`lib/templates/.claude/scripts/*.sh`** -- audit for `.intent/` references; flip if present.
5. **`lib/templates/_intent_critic.yml`** -- spot-check; likely no changes.
6. **`lib/templates/ext-seeds/worker-bee/`** -- audit subagent + skill files for `.intent/` references; flip if present.
7. **`intent/plugins/agents/bin/intent_agents`** -- generator emits `intent/.config/` in the file map block of AGENTS.md.
8. **`intent/plugins/claude/bin/intent_claude_upgrade`** -- diagnostic prints, action labels, dry-run output reference `intent/.config/`.
9. **`intent/plugins/claude/lib/claude_plugin_helpers.sh`** -- audit; flip path-construction.
10. **Verification on Intent itself (dry-run only)**: `intent agents sync --dry-run` (if such a flag exists) and `intent claude upgrade` (default dry-run) emit content referencing `intent/.config/`. Apply happens in WP08.

## Approach

1. `grep -rn '\.intent/' lib/templates/` -- enumerate every hit; categorise per WP03 rules (flip vs keep).
2. Apply flips file-by-file.
3. Generator-side: read `intent/plugins/agents/bin/intent_agents` for the file-map emission code; flip the path constant or string.
4. Read `intent/plugins/claude/bin/intent_claude_upgrade` for any diagnostic prints referencing `.intent/`; flip.
5. Run `intent claude upgrade` (dry-run) on Intent itself; verify the dry-run output references `intent/.config/` in: pretty-print actions, diff previews, action labels.
6. Run `intent agents sync` on Intent itself; verify regenerated AGENTS.md file map shows `intent/.config/` (do NOT commit the AGENTS.md regen as part of WP04 -- AGENTS.md drift handling per existing convention).

## Acceptance Criteria

- [ ] `grep -rn '\.intent/' lib/templates/` returns only intentional preservations.
- [ ] `lib/templates/llm/_CLAUDE.md` file map shows `intent/.config/`.
- [ ] `lib/templates/llm/_usage-rules.md` Project Structure shows `intent/.config/`.
- [ ] `intent agents sync` regenerates AGENTS.md with `intent/.config/` in its file map.
- [ ] `intent claude upgrade` (dry-run) shows action labels and previews referencing `intent/.config/`.
- [ ] Hook templates and ext-seeds audited; no stale `.intent/` references.

### Tests to add

- None directly. WP05 owns end-to-end BATS for the upgrade flow.

### Tests to update

- Any BATS that asserts on template content (`tests/unit/intent_claude_upgrade.bats`, `tests/unit/intent_agents_*.bats`, `tests/unit/template_*.bats`) -- assertions about file-map content flip. WP05 owns the per-test flip.

## Dependencies

- **Blocks**: WP08 (self-apply uses the updated templates).
- **Blocked by**: WP03 (literal sweep precedent and CHANGELOG entry).

## Implementation Notes

- The `_CLAUDE.md` template flip is a one-line change. After WP13 (just done), Intent's own root CLAUDE.md uses the WP09 template structure -- so this flip will also be reflected in Intent's CLAUDE.md when WP08's self-apply runs `intent claude upgrade --apply` (which refreshes the canon shell while preserving the user section). Round-trip dogfood works.
- For `_usage-rules.md`: the Project Structure tree is the place readers go to learn the layout. Update it carefully -- nested under `intent/`, `.config/` is now a sibling of `st/`, `docs/`, `llm/`, `plugins/`, `usr/`, `.treeindex/`.
- The AGENTS.md generator (`intent_agents`) emits a section that lists project layout. After flip, regenerated AGENTS.md shows `intent/.config/` -- expected.
- `intent_claude_upgrade` has many diagnostic prints; sweep them all for stale literals.
- `intent_agents sync` regenerates AGENTS.md every run -- after WP04 lands, the next sync run produces a diff (date stamp + content). Confirm the content diff is exactly what we expected (file-map flip).

## Risks and Edge Cases

- **Templated heredocs in scripts**: a script might string-build `.intent/...` inline rather than from `lib/templates/`. WP02's grep over `bin/` should have caught these; `intent/plugins/*/bin/*` deserves a similar sweep.
- **`worker-bee` ext-seed**: this seeds a user-level extension at `~/.intent/ext/worker-bee/`. The seed itself is read-only canon (`lib/templates/ext-seeds/worker-bee/`) but its contents may include path examples. Check for `.intent/` (per-project) vs `~/.intent/` (user-level extension root).
- **Help text in CLI scripts**: `intent help upgrade` shows examples; ensure they reference the new path.
- **AGENTS.md drift**: every run of `intent agents sync` regenerates with today's date stamp + minor whitespace -- pre-existing cosmetic concern. WP04 changes regen content; commit the WP04 changes in `intent/plugins/agents/bin/intent_agents` and `lib/templates/...` but leave the AGENTS.md regen for WP08's self-apply.

## Verification Steps

1. `grep -rn '\.intent/' lib/templates/` -- 0 hits or only documented keeps.
2. `grep -rn '\.intent/' intent/plugins/{agents,claude}/bin/` -- 0 hits or only documented keeps.
3. `intent claude upgrade` on Intent (dry-run) -- review output for `intent/.config/` references in the appropriate places.
4. `intent agents sync` -- regenerated AGENTS.md shows `intent/.config/` in file map (visible via `git diff AGENTS.md`).

## Size and Estimate

- **Size**: S (Small). One session.

## Exit Checklist

- [ ] All template-side `.intent/` literals flipped.
- [ ] Generator-side prints flipped.
- [ ] Help text updated.
- [ ] `intent agents sync` regen visually correct.
- [ ] `intent claude upgrade` (dry-run) output visually correct.
- [ ] AGENTS.md regen NOT committed (handled in WP08 self-apply).
- [ ] Committed: `refactor: ST0036/WP-04 templates and generators emit intent/.config`.
