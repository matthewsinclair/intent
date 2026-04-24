---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-02
title: "Refresh root usage-rules.md to current state"
scope: Small
status: Not Started
---

# WP-02: Refresh root usage-rules.md to current state

## Objective

Rewrite Intent's root `usage-rules.md` to reflect the v2.9.0+ surface: `/in-*` skill family, critic-\* subagents, rule library, extension system at `~/.intent/ext/`, worker-bee relocation note, hook architecture overview. Preserve the existing DO / NEVER structure. Also author a generic `lib/templates/llm/_usage-rules.md` template that downstream projects can instantiate via `intent claude upgrade --apply` (used by WP11 and WP16).

## Context

The current root `usage-rules.md` (297 lines, hand-authored) pre-dates v2.9.0 shipping. It covers steel threads, treeindex, AGENTS.md/RULES.md/ARCHITECTURE.md three-file split, `intent claude subagents|skills` commands, and a NEVER DO list — all still correct — but misses:

- `/in-*` skill family (`/in-session`, `/in-start`, `/in-next`, `/in-plan`, `/in-review`, `/in-verify`, `/in-debug`, `/in-finish`, `/in-handoff`, `/in-detrope`, `/in-autopsy`, `/in-cost-analysis`, `/in-tca-*` suite, language-specific ones).
- Critic subagent family (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`) and `intent claude rules` commands.
- Extension system at `~/.intent/ext/` and `intent ext` commands.
- Worker-bee relocation from canon to the reference extension.
- Hook architecture (once WP04 lands) — session hooks, pre-commit hook.

The refresh must stay terse and prescriptive (DO / NEVER form) — if the narrative wants to explain _why_, that's `working-with-llms.md` (WP03). `usage-rules.md` is the rules contract, not the user manual.

`usage-rules.md` is the Elixir-ecosystem-aligned file. `mix usage_rules.sync` in downstream Elixir projects discovers it (and other packages' usage-rules) and gathers them into AGENTS.md. Intent's file must therefore be parseable as-is — no surprises.

## Deliverables

1. **Updated root `usage-rules.md`** at `/Users/matts/Devel/prj/Intent/usage-rules.md` covering v2.9.0+ surface. Same DO / NEVER structure.
2. **New template** at `lib/templates/llm/_usage-rules.md` — the generic starting point for downstream projects. Substitution-style placeholders where needed (`{{PROJECT_NAME}}`, `{{LANG}}`, etc.) following Intent's existing sed substitution pattern.
3. **MODULES.md registration** for the new template (the existing template dir `lib/templates/llm/` already registered; this just adds the new file to the inventory).
4. **Cross-references added**: `usage-rules.md` gains a "See also" section pointing at `AGENTS.md` (for navigation) and `intent/docs/working-with-llms.md` (for rationale — note WP03 authors that file; WP02 writes the cross-ref but confirms the path).

## Approach

1. Read the current `usage-rules.md` end to end (already done in audit; reconfirm).
2. Identify sections that need updating, adding, or deleting:
   - **Update**: `Claude Subagents` section — add critic-\* list and short purpose of each.
   - **Update**: `Claude Skills` section — add full `/in-*` family; note `/in-session` auto-load behaviour.
   - **Update**: `AGENTS.md Management` section — reflect that AGENTS.md is now at root (real file) not intent/llm/.
   - **Add**: `Rule Library` section — `intent/plugins/claude/rules/`, `intent claude rules list|show|index`, rule ID format, single-source-of-truth rule.
   - **Add**: `Extensions` section — `~/.intent/ext/`, `intent ext list|show|validate|new`, how extensions shadow canon.
   - **Add**: `Session Hooks` section — brief: SessionStart reminds `/in-session`, Stop reminds `/in-finish`; configured in `.claude/settings.json`.
   - **Add**: `Critics and pre-commit` section — `bin/intent_critic`, pre-commit gate, severity threshold via `.intent_critic.yml`.
   - **Update**: `NEVER DO` section — add new items: never edit `.claude/settings.json` hook stanzas without updating the template; never bypass `.git/hooks/pre-commit` with `--no-verify` on shared branches without justification.
   - **Delete**: any stale references to `intent/llm/AGENTS.md` (the real AGENTS.md is at root now; noted in WP08/WP10).
3. Write the `lib/templates/llm/_usage-rules.md` template by copying the refreshed root file and replacing project-specific content with placeholders. Keep the structure identical for consistency.
4. Verify the file parses cleanly as markdown (markdownlint if available) and has no broken cross-references.
5. Commit: `docs: refresh root usage-rules.md to v2.9.0+ surface`.

## Acceptance Criteria

- [ ] `usage-rules.md` at root references all 23 `/in-*` skills (or explicitly groups them).
- [ ] `usage-rules.md` at root lists all 5 critic-\* subagents with one-line purpose each.
- [ ] `usage-rules.md` at root has a `Rule Library` section explaining location, command interface, and single-source-of-truth rule.
- [ ] `usage-rules.md` at root has an `Extensions` section explaining `~/.intent/ext/`, discovery, and `intent ext` command.
- [ ] `usage-rules.md` at root has a `Session Hooks` section (brief — WP04 authors the template, this doc references it).
- [ ] `usage-rules.md` at root has a `Critics and pre-commit` section (brief — WP05/WP06 author the runner and hook, this doc references them).
- [ ] `usage-rules.md` at root does NOT reference `intent/llm/AGENTS.md` (it's retired in WP10; the refreshed doc anticipates that).
- [ ] `usage-rules.md` at root retains the original DO / NEVER structure.
- [ ] `lib/templates/llm/_usage-rules.md` exists, mirrors the structure of the root file, uses placeholder substitutions where appropriate.
- [ ] Cross-references added to `AGENTS.md` and `intent/docs/working-with-llms.md`.
- [ ] `intent/llm/MODULES.md` lists `lib/templates/llm/_usage-rules.md`.
- [ ] No markdown linter errors (run project's linter).
- [ ] Commit follows Intent conventions, no Claude attribution.

### Tests to add

- None directly — usage-rules.md is prose. If there's an existing smoke test that validates root files exist, ensure it still passes.

### Tests to update

- None.

## Dependencies

- **Blocked by**: WP01 (version bump done; we can now reference v2.9.1 semantics).
- **Blocks**: WP03 (working-with-llms.md cross-references this file).

## Implementation Notes

- **Existing DO / NEVER voice**: keep it terse, imperative, scannable. If a rule doesn't fit the DO / NEVER pattern, move it to `working-with-llms.md`. This file is the contract.
- **Template placeholders**: use the existing sed-substitution pattern seen in other `lib/templates/llm/*` files. Examine `_CLAUDE.md` or `_MODULES.md` for the placeholder syntax (likely `{{NAME}}`-style; verify).
- **"How to work with LLMs" vs "How to use Intent"**: `usage-rules.md` is for both audiences (it tells LLMs what to DO/NEVER do, and it tells humans what to tell their LLMs). Keep language neutral — avoid "you" referring to either specifically.
- **Length discipline**: refreshed file should still fit in one scroll screen per section. If a section needs more than ~30 lines, push the long-form content to `working-with-llms.md` and leave a cross-reference.

## Risks and Edge Cases

- **Risk**: Refreshed file becomes too narrative — drifts into user manual territory. **Mitigation**: pass through once at the end to cull any prose that isn't a rule. If it's explanation, it belongs in WP03's doc.
- **Risk**: Template placeholder syntax doesn't match existing Intent sed substitution. **Mitigation**: grep for existing placeholder usage in `lib/templates/` and match exactly.
- **Risk**: Breaking change for downstream Elixir projects using `mix usage_rules.sync` on Intent's root `usage-rules.md`. **Mitigation**: unlikely — `mix usage_rules.sync` ingests markdown as-is; structural changes don't break it.

## Verification Steps

1. Diff old vs new: `git diff usage-rules.md` — confirm all sections listed in Acceptance Criteria are updated / added.
2. Open `lib/templates/llm/_usage-rules.md` — confirm placeholder substitution pattern matches siblings.
3. Run project markdown linter (if configured). Zero errors.
4. `grep -n intent/llm/AGENTS usage-rules.md` returns nothing.
5. `grep -n "~/.intent/ext" usage-rules.md` returns hits (Extensions section present).
6. `grep -n "critic-" usage-rules.md` returns hits for all 5 critics.

## Size and Estimate

- **Size**: S (Small). Two sessions likely.
- Session 1: Refresh root file; author all new sections.
- Session 2: Write template; verify cross-refs; linter; commit.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Root file and template both committed.
- [ ] MODULES.md entry added.
- [ ] No dangling cross-references (WP03 author will link back in their WP).
- [ ] Committed cleanly.
