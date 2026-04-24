---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-09
title: "Rewrite root CLAUDE.md template as Claude overlay"
scope: Small
status: Not Started
---

# WP-09: Rewrite root CLAUDE.md template as Claude overlay

## Objective

Rewrite `lib/templates/llm/_CLAUDE.md` as a Claude-specific overlay template that imports `AGENTS.md` by reference and adds Claude-specific directives: memory directory pointer, `/in-session` auto-load expectation, `.claude/settings.json` hook pointer, file-mapping notes. Stops duplicating AGENTS.md content. Short by design.

## Context

Design D2: CLAUDE.md complements AGENTS.md, it doesn't compete. Anthropic positions CLAUDE.md as the Claude-specific overlay; AGENTS.md is the tool-agnostic primary. Intent's current `_CLAUDE.md` template duplicates much of what AGENTS.md says. WP09 fixes that.

Template is used by `intent init` and `intent claude upgrade` to seed/refresh `CLAUDE.md` in downstream projects. User can hand-edit post-init; the generator preserves user-marked sections.

## Deliverables

1. **Rewritten template** at `lib/templates/llm/_CLAUDE.md`. Structure:
   - **Reference block** at top: "This project uses Intent v{{VERSION}}. Start with `AGENTS.md` for project overview. This file adds Claude-specific directives."
   - **Required skills**: direct Claude to run `/in-session` at session start. Reference `intent/docs/working-with-llms.md` for rationale.
   - **Memory directory**: pointer to `~/.claude/projects/<dir>/memory/` for persistent memories (as already supported by Claude Code).
   - **Hooks**: one-liner that `.claude/settings.json` has hooks configured; link to `working-with-llms.md#session-hooks` for details.
   - **File mapping**: key files and where to find them (intent/, .intent/, AGENTS.md, usage-rules.md, `working-with-llms.md`).
   - **Rules of the road**: brief pointers to Intent's rules (Highlander, Thin Coordinator, No Silent Errors) — cross-references `usage-rules.md` for the full list. No duplication.
   - **Critic dispatch**: `/in-review` stage 2 invokes `critic-<lang>`; on-demand via `Task(subagent_type="critic-<lang>")`.
   - **Author + project-specific**: placeholder section for user content (preserved across regeneration).
2. **Placeholders**: `{{PROJECT_NAME}}`, `{{INTENT_VERSION}}`, `{{LANG}}`, any others needed. Sed substitution in the generator.
3. **MODULES.md update**: template already registered; re-verify content.
4. **Length budget**: target < 100 lines (currently ~40 in the existing template; enriched but stays short).

## Approach

1. Read existing `lib/templates/llm/_CLAUDE.md` — identify content that duplicates AGENTS.md and pull out.
2. Draft the new overlay structure per Deliverables list.
3. Verify all cross-references to `working-with-llms.md` use correct anchors (coordinate with WP03 author).
4. Add user-preservation markers (pattern reusing current generator behaviour).
5. MODULES.md audit.
6. Commit.

## Acceptance Criteria

- [ ] `lib/templates/llm/_CLAUDE.md` is ≤ 100 lines.
- [ ] Template has reference block at top pointing at AGENTS.md.
- [ ] Template explicitly directs Claude to run `/in-session` at session start.
- [ ] Template points at memory directory conventions.
- [ ] Template references `.claude/settings.json` hooks + `working-with-llms.md#session-hooks`.
- [ ] Template has file-mapping section.
- [ ] Template cross-references `usage-rules.md` for full rules (no duplication of rule text).
- [ ] Template has critic dispatch section (brief).
- [ ] Template has user-preservation marker section.
- [ ] Placeholders (`{{PROJECT_NAME}}`, `{{INTENT_VERSION}}`, `{{LANG}}`) present and sed-substitutable.
- [ ] No content duplicated from AGENTS.md template.
- [ ] MODULES.md up to date.
- [ ] Commit follows Intent conventions.

### Tests to add

- **BATS test**: `intent init` on a scratch project seeds `CLAUDE.md` from this template; placeholders substituted correctly.
- **BATS test**: running `intent claude upgrade --apply` on a project with a hand-edited `CLAUDE.md` user-section preserves the user content.

### Tests to update

- Any existing BATS tests that test `CLAUDE.md` content for specific strings that may have moved to AGENTS.md.

## Dependencies

- **Blocked by**: WP08 (template references root AGENTS.md).
- **Blocks**: WP11 (upgrade ships the new template), WP13 (Intent's own CLAUDE.md rewritten accordingly).

## Implementation Notes

- **User-preservation pattern**: likely uses HTML comment markers (e.g., `<!-- user-section-start -->` / `<!-- user-section-end -->`) per existing generator. Confirm and reuse; don't introduce a new pattern.
- **Short > long**: resist the urge to explain. CLAUDE.md is not the doc; `working-with-llms.md` is. Point and move on.
- **Anchor stability**: cross-references to `working-with-llms.md#section-name` depend on that file's anchors being stable. Coordinate with WP03 — WP03 should finalize anchors before WP09 ships cross-refs.
- **Language-agnostic**: template must work for bash, Elixir, docs-only projects equally. Skip language-specific content in the canon section; leave to the user-section.

## Risks and Edge Cases

- **Risk**: Projects with complex hand-edited CLAUDE.md content that doesn't fit the user-section marker. **Mitigation**: generator preserves unrecognised content by default; only replaces canonical sections. Tested in WP09 BATS.
- **Risk**: Some projects have `CLAUDE.md` at `.claude/CLAUDE.md` (old convention). **Mitigation**: canon is root CLAUDE.md. If `.claude/CLAUDE.md` exists, log and prefer the root (non-destructive).
- **Risk**: Cross-reference anchors in `working-with-llms.md` may shift. **Mitigation**: WP03 locks anchors before WP09.
- **Edge**: Project has no `.claude/` dir. Template still renders — references are informational.

## Verification Steps

1. Diff old vs new template — confirm significant content reduction (duplication removed).
2. `wc -l lib/templates/llm/_CLAUDE.md` ≤ 100.
3. `intent init` in a scratch dir; verify CLAUDE.md renders correctly; placeholders resolved.
4. `grep "AGENTS.md\|working-with-llms" CLAUDE.md` — cross-refs present.
5. `grep -c "Highlander\|Thin Coordinator\|No Silent Errors" CLAUDE.md` — mentions exist but brief.
6. BATS tests green.

## Size and Estimate

- **Size**: S (Small). 1–2 sessions.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Template committed.
- [ ] BATS tests green.
- [ ] MODULES.md verified.
- [ ] Anchor-level cross-refs to WP03's doc verified.
- [ ] Committed.
