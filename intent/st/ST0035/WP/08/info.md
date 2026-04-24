---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-08
title: "Rewrite root AGENTS.md generator"
scope: Medium
status: Done
---

# WP-08: Rewrite root AGENTS.md generator

## Objective

Modify `intent/plugins/agents/bin/intent_agents` so that `intent agents sync` writes `AGENTS.md` to the project **root** (as a real file, not a symlink), not to `intent/llm/AGENTS.md`. Enrich the template content with current-as-built Intent surface (critic family, rule library pointer, extension system, hooks overview, Socrates-vs-Diogenes FAQ pointer). `intent/llm/AGENTS.md` retires in WP10.

## Context

Design D1 and D4: AGENTS.md lives at root as a real file, matching community convention (agents.md spec, Linux Foundation Agentic AI Foundation). `intent/llm/AGENTS.md` was the old output location. Fleet audit shows inconsistency: some projects have root AGENTS.md as a symlink to `intent/llm/AGENTS.md`, some have root only, some have both.

`intent_agents` is the generator. `intent agents init` creates initial files; `intent agents sync` updates them; `intent agents validate` checks compliance. WP08 retargets `sync` and `init` to root, enriches template content, and updates `validate` to check the new location.

## Deliverables

1. **Generator update**: `intent/plugins/agents/bin/intent_agents` — output path flips from `intent/llm/AGENTS.md` to root `AGENTS.md`.
2. **Template update**: `intent/plugins/agents/templates/elixir/AGENTS.md` — enriched with v2.9.0+ content. Applies to the elixir template family; confirm non-elixir template paths too (bash, docs, etc.) or consolidate into a single template if feasible.
3. **Validator update**: `intent agents validate` checks root AGENTS.md (not intent/llm/).
4. **Real file, not symlink**: `intent agents sync` writes a real file. If the target is a symlink to `intent/llm/AGENTS.md`, `sync` detects and replaces it.
5. **Content enrichment** — new template sections:
   - Project overview (as today).
   - Prerequisites + setup commands.
   - Build and test commands (detected from language markers).
   - Code style summary (brief; details in `usage-rules.md` and `working-with-llms.md`).
   - Steel thread + work-package process summary (brief).
   - **Installed skills** (dynamic, read from `.claude/skills/`).
   - **Installed subagents** (dynamic, read from `.claude/agents/`).
   - **Critic family** — brief: `critic-<lang>` available; headless runner at `bin/intent_critic`; pre-commit gate via `.git/hooks/pre-commit`.
   - **Rule library pointer** — `intent/plugins/claude/rules/` discoverable via `intent claude rules list`.
   - **Extension pointer** — `~/.intent/ext/` discoverable via `intent ext list`.
   - **Socrates vs Diogenes FAQ pointer** — link to `intent/docs/working-with-llms.md#socrates-vs-diogenes`.
   - **Hooks overview** — SessionStart + Stop; link to `working-with-llms.md#session-hooks`.
   - Security considerations (as today).
   - Additional resources (cross-refs).
6. **MODULES.md update**: re-register the template path if any path changes.

## Approach

1. Read `intent/plugins/agents/bin/intent_agents` — understand current sync/init/validate logic.
2. Read `intent/plugins/agents/templates/elixir/AGENTS.md` — understand current content.
3. Plan the migration:
   - Add new output path (root AGENTS.md) as primary.
   - Detect and handle existing symlinks: if `AGENTS.md` is a symlink to `intent/llm/AGENTS.md`, `rm -f` the symlink then write the real file.
   - Preserve user-curated sections if any markers exist (current generator's behaviour — review).
4. Enrich the template. Some sections are dynamic (installed skills/subagents from `.claude/`); others are static (static pointers to `working-with-llms.md`, critic family overview).
5. Update `intent agents validate` to check root path.
6. Update `intent agents init` — same path change.
7. Verify: run `intent agents sync` on Intent's own project in a scratch worktree. Confirm root AGENTS.md gets the enriched content.
8. Test with fleet projects (lightly — no rollout yet; that's WP14/WP15).
9. MODULES.md audit.
10. Commit.

## Acceptance Criteria

- [ ] `intent agents init --template elixir` (dry run) reports output path as root `AGENTS.md`, not `intent/llm/AGENTS.md`.
- [ ] `intent agents sync` writes to root `AGENTS.md`; does NOT write to `intent/llm/AGENTS.md`.
- [ ] `intent agents sync` detects a pre-existing root symlink and replaces with real file.
- [ ] `intent agents validate` checks root `AGENTS.md` and returns clean when the template is current.
- [ ] Template includes all 10+ enriched sections (per Deliverables list).
- [ ] Template links to `intent/docs/working-with-llms.md#socrates-vs-diogenes`.
- [ ] Template links to `intent/docs/working-with-llms.md#session-hooks`.
- [ ] Dynamic sections (installed skills/subagents) read from `.claude/` and render per project.
- [ ] Regenerating is idempotent — running `sync` twice in a row produces no diff on the second run.
- [ ] MODULES.md updated.
- [ ] `tests/run_tests.sh` green (any existing agents-subcommand tests updated).
- [ ] Commit follows Intent conventions.

### Tests to add

- **BATS test**: `intent agents init` in a scratch project writes root AGENTS.md (not intent/llm/).
- **BATS test**: `intent agents sync` on a project with pre-existing symlink replaces with real file.
- **BATS test**: `intent agents sync` is idempotent (two runs, second produces empty diff).
- **BATS test**: `intent agents validate` passes after sync on a clean project.

### Tests to update

- Any existing BATS tests that assert output at `intent/llm/AGENTS.md` — flip to root.

## Dependencies

- **Blocked by**: WP03 (template references `working-with-llms.md` which must exist).
- **Blocks**: WP09 (CLAUDE.md template cross-references root AGENTS.md), WP10 (delete `intent/llm/AGENTS.md`), WP11 (upgrade uses the updated generator).

## Implementation Notes

- **Templates plural**: there may be multiple templates under `intent/plugins/agents/templates/` (currently only `elixir/`). Confirm at WP08 start. If more exist or are planned (e.g., `rust/`, `bash/`), update all consistently.
- **Dynamic section discovery**: read installed skills from `.claude/skills/` directory listing; read installed subagents similarly. Don't hardcode the canon list — render what's actually installed per project.
- **User-curated preservation**: the current generator preserves sections marked by user-authored markers. Preserve this contract. If enriched sections collide with user-markers, merge intelligently; document the algorithm.
- **Symlink replacement is safe**: the old symlink points at intent/llm/AGENTS.md which is being deleted in WP10 anyway. Replacing with a real file at root is the right migration.
- **Footer version stamp**: existing footer reads `_Generated by Intent v2.9.0 on YYYY-MM-DD_`. Bump to 2.9.1 (coordinate with WP01 version bump).

## Risks and Edge Cases

- **Risk**: Some fleet projects have hand-edited `intent/llm/AGENTS.md` content that must survive the move. **Mitigation**: WP08 generator detects non-template content in `intent/llm/AGENTS.md`, archives it to `intent/llm/AGENTS.md.pre-v2.9.1.bak`, and prints a migration notice. WP10 deletes `intent/llm/AGENTS.md` but preserves the `.bak` for a release cycle.
- **Risk**: Root AGENTS.md as a symlink breaks some tool. **Mitigation**: D1 already decided real file; the migration is one-way.
- **Risk**: Enriched template becomes too long — AGENTS.md is usually expected to be scannable. **Mitigation**: target < 200 lines generated. Longer narrative goes in `working-with-llms.md`.
- **Edge**: `.claude/` absent. Dynamic sections emit "No skills installed" / "No subagents installed" gracefully.
- **Edge**: Non-git project. Still works — `intent agents sync` doesn't require git.

## Verification Steps

1. In a scratch Intent project (copy of Intent), remove `AGENTS.md` and any `intent/llm/AGENTS.md` or symlink. Run `intent agents init --template elixir`. Confirm `AGENTS.md` created at root, nothing at `intent/llm/AGENTS.md`.
2. `ls -la AGENTS.md` → regular file, not symlink.
3. `grep "working-with-llms" AGENTS.md` — cross-references present.
4. `grep "critic-" AGENTS.md` — critic family mentioned.
5. `intent agents sync` — no diff.
6. Edit `AGENTS.md` by hand (add a user-marker section). Run `sync` — confirm user content preserved.
7. BATS tests pass.

## Size and Estimate

- **Size**: M (Medium). 2–3 sessions.
- Session 1: Generator retarget (output path change); symlink detection.
- Session 2: Template enrichment (10+ sections); dynamic discovery of installed skills/subagents.
- Session 3: Validator update; BATS; MODULES.md; commit.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Generator writes to root by default.
- [ ] Symlink migration works.
- [ ] Template has all enriched sections.
- [ ] BATS tests green.
- [ ] MODULES.md updated.
- [ ] Committed.
