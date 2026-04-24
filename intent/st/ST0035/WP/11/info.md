---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-11
title: "Extend intent claude upgrade to apply canon artefacts"
scope: Medium
status: WIP
---

# WP-11: Extend intent claude upgrade to apply canon artefacts

## Objective

Extend `intent/plugins/claude/bin/intent_claude_upgrade` so that `intent claude upgrade --apply` installs every canon artefact produced by WP04–WP09 into the target project: `.claude/settings.json` + helper script, `.git/hooks/pre-commit`, `.intent_critic.yml`, root `AGENTS.md` (via the updated generator), root `CLAUDE.md` (templated), root `usage-rules.md` (templated). Deletes `intent/llm/AGENTS.md` and any `_llm_preamble.md` on apply. Idempotent.

## Context

`intent claude upgrade` already exists — it stamps the version, syncs installed skills/subagents, and prunes deprecated artefacts. WP11 extends its scope to install the full canon set.

This is the connector WP. Every template / generator / artefact authored in WP04–WP09 flows through `intent claude upgrade --apply`. Without WP11 nothing reaches downstream projects.

The command has `--dry-run` by default and `--apply` as the gated form. Both semantics preserved.

## Deliverables

1. **Extended `intent_claude_upgrade` logic** to handle these artefacts:
   - Version stamp → 2.9.1 (already handled by existing logic; verify).
   - Install/refresh `.claude/settings.json` from `lib/templates/.claude/settings.json` (WP04). Preserve user-edited sections via markers; the hook stanzas come from canon.
   - Install `.claude/scripts/session-context.sh` (WP04). chmod +x.
   - Install `.git/hooks/pre-commit` from `lib/templates/hooks/pre-commit.sh` (WP06). chmod +x. If existing hook present and non-canonical, write to `.git/hooks/pre-commit.intent` and emit a chaining instruction to stderr.
   - Install `.intent_critic.yml` from `lib/templates/_intent_critic.yml` (WP07). If user-edited version exists, preserve.
   - Install root `AGENTS.md` via `intent agents sync` (WP08's retargeted generator). Delete any `intent/llm/AGENTS.md` symlink or file.
   - Install root `CLAUDE.md` from `lib/templates/llm/_CLAUDE.md` (WP09). Preserve user-marked sections.
   - Install root `usage-rules.md` from `lib/templates/llm/_usage-rules.md` (WP02). If user has their own, preserve (with a diff emitted on dry-run).
   - Plant `intent/llm/DECISION_TREE.md` if missing (9 projects need this per audit). Plant `intent/llm/MODULES.md` if missing. Both from canonical seed templates.
   - Delete `intent/llm/AGENTS.md` if present.
   - Delete `lib/templates/llm/_llm_preamble.md` if present (shouldn't be, but check).
2. **Dry-run output**: readable diff per artefact — user sees what `--apply` would do.
3. **Apply output**: per-artefact line confirming create/update/delete.
4. **Idempotence**: running `--apply` twice in a row produces no changes on the second run.
5. **Migration stub** (WP01 seed): `migrate_v2_9_0_to_v2_9_1()` now calls the full apply sequence — not just the stamp bump.
6. **Safety checks**:
   - Don't overwrite a file the user edited unless explicitly requested (`--force` flag for nuclear overwrite).
   - Don't install `.git/hooks/pre-commit` if there's a non-Intent hook there (chain via `.pre-commit.intent`).
   - Fail gracefully if `intent agents sync` (WP08) errors; emit diagnostic, don't continue.
7. **MODULES.md update**: register any new helper functions or libraries used.

## Approach

1. Read `intent/plugins/claude/bin/intent_claude_upgrade` — understand existing scaffold (dry-run vs apply, how it calls sub-generators, how it detects user-edited files).
2. Map each artefact (WP04–WP09) to a call in the upgrade sequence.
3. Design the safe-update helpers:
   - `canon_install_file(src, dst, [preserve_markers])` — copies src to dst with user-section preservation if markers present.
   - `canon_install_script(src, dst)` — copies + chmod +x.
   - `canon_delete_file(path, reason)` — git rm if tracked, rm if not.
4. Implement the extended sequence in `intent_claude_upgrade`, calling the helpers per artefact.
5. Build the dry-run output formatter.
6. Test on Intent itself (WP14 does the self-apply; WP11 tests in a scratch worktree).
7. BATS test suite: scenarios include fresh project, already-canon project (idempotence), user-edited files preservation, chain-not-overwrite on pre-existing hooks.
8. MODULES.md audit.
9. Commit.

## Acceptance Criteria

- [ ] `intent claude upgrade --dry-run` on Intent itself lists all expected changes (AGENTS.md root write, CLAUDE.md overlay, usage-rules.md refresh, .claude/settings.json install, pre-commit hook install, .intent_critic.yml plant, DECISION_TREE.md confirm, intent/llm/AGENTS.md delete).
- [ ] `intent claude upgrade --apply` on Intent itself produces those changes.
- [ ] Running `--apply` a second time produces zero changes (idempotence).
- [ ] User-edited files with preservation markers are preserved.
- [ ] Existing non-Intent `.git/hooks/pre-commit` is chained via `.git/hooks/pre-commit.intent`, not overwritten.
- [ ] Exit code 0 on success, non-zero on failure with a diagnostic.
- [ ] `tests/run_tests.sh` BATS tests for upgrade green.
- [ ] `intent doctor` post-apply returns clean.
- [ ] MODULES.md lists new helper functions.
- [ ] Commit follows Intent conventions.

### Tests to add

- **BATS test**: fresh scratch project; run `upgrade --apply`; confirm all canon artefacts installed.
- **BATS test**: same project, run `upgrade --apply` again; confirm zero changes (idempotence).
- **BATS test**: project with user-edited CLAUDE.md in user-section; run apply; user content preserved.
- **BATS test**: project with pre-existing non-Intent pre-commit hook; apply chains, doesn't overwrite.
- **BATS test**: `--dry-run` produces expected output without modifying files.

### Tests to update

- Any existing BATS tests for `intent_claude_upgrade` that assert specific sub-step behaviour — extend for new artefacts.

## Dependencies

- **Blocked by**: WP04 (settings.json template), WP06 (pre-commit template), WP07 (intent_critic.yml template), WP08 (AGENTS.md generator retarget), WP09 (CLAUDE.md overlay template).
- **Blocks**: WP14 (self-apply), WP15 (canary), WP16 (fleet).

## Implementation Notes

- **User-section preservation markers**: adopt consistent marker syntax across templates. HTML comment style (`<!-- intent:user-section-start -->`) is robust (markdown-agnostic). Verify WP09's template uses the same markers.
- **`intent agents sync` invocation from upgrade**: call the binary, don't re-implement. Respect its exit codes; surface errors.
- **Performance**: upgrade is infrequent but should complete in < 30s on a medium project.
- **Atomic-per-artefact**: each install is a discrete step; if one fails, previous succeed (no transactional rollback — fail-forward). Emit clear diagnostics so user can resume.
- **Don't touch user memory files**: `~/.claude/projects/<dir>/memory/` is user-owned; never write there from upgrade.
- **Cwd-resilient**: follow `INTENT_ORIG_CWD` pattern from ST0033.

## Risks and Edge Cases

- **Risk**: WP11 ships, but one of WP04-WP09 has drifted (e.g., WP09 anchor names moved). **Mitigation**: WP11 ships _after_ WP04-WP09 are all stable. Coordinate closely.
- **Risk**: A fleet project has deeply customised `.claude/settings.json` that doesn't fit the preservation-marker pattern. **Mitigation**: `--dry-run` shows the diff; user can opt out of settings.json install via a flag (`--skip-settings`) or manually reconcile.
- **Risk**: `.git/hooks/pre-commit` chaining breaks. **Mitigation**: document clearly; user has `.pre-commit.intent` available to review before committing to chain.
- **Risk**: `intent agents sync` fails silently mid-upgrade. **Mitigation**: check exit code; abort upgrade on failure.
- **Edge**: Project is inside a git worktree or submodule. Hook install path may differ. Test in a worktree.
- **Edge**: Read-only filesystem (unlikely but possible in CI). Emit clear error and abort.

## Verification Steps

1. Clone Intent's repo to a scratch location. `intent claude upgrade --dry-run` — read the plan.
2. `intent claude upgrade --apply` — watch changes.
3. `git diff` — confirm expected changes.
4. `intent claude upgrade --apply` again — confirm zero diff.
5. Manually edit `CLAUDE.md` in the user section; re-apply; confirm user content survives.
6. Drop a pre-existing pre-commit hook into `.git/hooks/pre-commit`; apply; confirm chain file created.
7. BATS green.

## Size and Estimate

- **Size**: M (Medium). 3 sessions.
- Session 1: Map canon → install helpers; extend upgrade logic for each artefact.
- Session 2: User-preservation logic; chain-don't-overwrite for hooks.
- Session 3: BATS tests; MODULES.md; dry-run output formatter; commit.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Idempotence verified.
- [ ] BATS green.
- [ ] Scratch-project manual test clean.
- [ ] MODULES.md updated.
- [ ] Committed.
