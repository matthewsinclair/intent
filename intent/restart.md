# Claude Code Session Restart — narrative state

## Current state (2026-04-25, end of session — 11 of 18 WPs Done + WP-11 Session 1 shipped + ST0036 opened)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WP-11 mid-flight (Session 1 shipped, Sessions 2+3 remain). ST0036 (Directory relocation) opened as Phase 0 stub, ships bundled.**

### Version retarget (mid-session)

- Retargeted v2.9.1 → v2.10.0 to bundle ST0036 (`.intent/` → `intent/.config/`) as a single breaking release.
- `VERSION`, `.intent/config.json`, `bin/intent_helpers` (`migrate_v2_9_0_to_v2_10_0` + `needs_v2_10_0_upgrade`), `bin/intent_upgrade` chain, CHANGELOG, AGENTS.md all updated.
- Retarget commit: `b760b39`.

### ST0035 shape

- **Done (11)**: WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP08, WP09, WP10, WP12.
- **WIP (1)**: WP11 — Session 1 committed (`e999f82`); Sessions 2+3 remain.
- **Not Started (6)**: WP13, WP14, WP15, WP16, WP17, WP18.

Critical path remaining: `WP11 → WP14 → WP15 → WP16 → WP17`. WP13 (S) can run in parallel before WP14. WP17 has a second gating input (WP18 — user-doc audit).

### ST0036 shape (new)

Phase 0 stub only at `intent/st/NOT-STARTED/ST0036/`:

- `info.md` — objective, why bundled with ST0035, scope, success criteria, Phase 0 gate.
- `design.md` — provisional canon decisions D1–D5 (new path `intent/.config/`, atomic fail-forward migration, whole-tree preservation, shared rollout with ST0035, CHANGELOG + migration guide), risk register, open questions.
- `tasks.md` — 9 provisional WPs with T-shirt sizing (WP01 migration function, WP02 path probes, WP03 literal sweep, WP04 templates, WP05 BATS, WP06 gitignore, WP07 migration guide, WP08 Intent self-apply, WP09 merge with ST0035 fleet rollout).

Forensic `WP/NN/info.md` elaboration deferred until ST0036 is actively picked up (projected after ST0035/WP13 lands). Phase 0 review gate before any ST0036 WP01 start.

### Progress since last restart-note (post-compact after WP10)

This session kicked off WP-11 (Session 1 of 3):

- **WP-11 Session 1 shipped (commit `e999f82`)**: `intent claude upgrade --apply` extended to install the full v2.10.0 canon in one shot.
  - 7 canon-install helpers in `intent_claude_upgrade`: `canon_install_file`, `canon_install_script`, `canon_delete_file`, `canon_refresh_with_user_section` (marker-bounded preservation for CLAUDE.md), `canon_substitute_placeholders` (PROJECT_NAME / AUTHOR / DATE / INTENT_VERSION / INTENT_HOME), `canon_template_matches_installed` (placeholder-aware drift probe — critical for idempotence), `canon_extract_user_section`, `canon_claude_md_is_generated`, `canon_resolve_author`.
  - 11 new action codes across Phases 1/2/3: `INSTALL_SETTINGS`, `INSTALL_HOOK_SCRIPT:<name>`, `INSTALL_PRE_COMMIT`, `CHAIN_PRE_COMMIT`, `INSTALL_CRITIC_CONFIG`, `INSTALL_CLAUDE_MD`, `REFRESH_CLAUDE_MD`, `INSTALL_USAGE_RULES`, `PLANT_MODULES`, `PLANT_DECISION_TREE`, `DELETE_LEGACY_AGENTS`.
  - REGENERATE/CREATE AGENTS.md now calls `intent agents sync` (the WP-08 generator) instead of copying the Elixir template.
  - `migrate_v2_9_0_to_v2_10_0` in `bin/intent_helpers` now invokes `intent claude upgrade --apply` after the stamp bump so fleet upgrades install canon in one migration step.
  - Fixed two pre-existing bugs uncovered during scratch-project test: AGENTS.md version regex didn't match multi-digit semver (`v2.10.0`); hook-script drift probe compared raw template (with `[[INTENT_HOME]]`) against substituted install, always reporting DIVERGED.
  - Verified on `/tmp/wp11-scratch-XXXXXX`: fresh install lands all artefacts; second `--apply` produces zero actions (idempotent); full suite **762/762 green**.

**Sessions 2+3 deferred**:

- Session 2: edge cases — diff-in-dry-run for divergent user-authored CLAUDE.md; richer hook-chain instructions (including a prepared snippet the user can paste); read-only FS + submodule handling.
- Session 3: BATS suite (5 scenarios from spec: fresh project, idempotent re-apply, user-edit preservation, pre-existing hook chain, `--dry-run` no-op); MODULES.md audit registering the helpers; Done commit.

### Lessons worth keeping

- **Bundled semver bumps are cheap before release tag.** ST0035 was mid-flight at v2.9.1; retargeting to v2.10.0 to bundle ST0036 cost ~5 files' worth of string replacement (VERSION, config.json, helpers, upgrade chain, CHANGELOG). Zero rollout cost since no tag existed. Rule: check for uncommitted version bumps before committing to a breaking release strategy.
- **Deprecation sweeps leave ghost code paths.** Deleting a file doesn't delete its readers/writers. WP10's "two rm commands" ballooned into 8 file updates because `intent_init`, `intent_helpers::_generate_basic_agents_md`, `intent_doctor`, `intent_claude_upgrade`, and `docs_completeness.bats` all still pointed at `intent/llm/AGENTS.md`. Always grep for the deleted path and scope the WP accordingly.
- **Test suite hides stale-file false positives.** `docs_completeness.bats::agents_sync_idempotent` was passing post-WP08 because both runs copied the same stale intent/llm/AGENTS.md file (not the newly-written root one). 762 tests green can still hide a silent test. Periodic audit: do my tests actually exercise the code path they claim to?
- **"AGENTS.md is a symlink" migration works by deletion then write.** `_replace_symlink_if_present` in `intent_agents` handles the legacy-layout case (root AGENTS.md → symlink to intent/llm/AGENTS.md). Idempotent, safe to call on any project state.
- **Idempotence requires placeholder-aware drift compare.** The first WP-11 scratch test reported `.claude/scripts/post-tool-advisory.sh` as DIVERGED on every re-run because the probe was comparing the raw template (with `[[INTENT_HOME]]`) against the install (with the substituted absolute path). Hook scripts and settings.json need a probe that substitutes placeholders into a temp template before `cmp`. Generalised this into `canon_template_matches_installed` and used it everywhere a placeholder template is involved. Without this, every install path that does substitution must explicitly re-substitute for the diff probe.
- **Scratch-project end-to-end test catches what BATS can't.** Running `intent claude upgrade --apply` against a real `intent init` scratch dir exposed two pre-existing version-regex bugs (multi-digit semver, hook-script placeholder drift) that the existing BATS suite never tripped. Worth doing before the BATS work, not after — cheaper to fix mechanics in a scratch worktree than in a fixture.

## Resume target (WP-11 Session 2 — edge cases + dry-run polish)

WP-11 Session 1 shipped the install machinery (`e999f82`). Sessions 2 + 3 still to land.

**Session 2 scope** (edge cases + UX polish):

- Diff-in-dry-run for divergent user-authored CLAUDE.md so the user sees what the canon overlay would add.
- Richer hook-chain instructions in `CHAIN_PRE_COMMIT` (a ready-to-paste snippet for the existing `.git/hooks/pre-commit`, not just a hint).
- Read-only FS / submodule / non-standard `.git` handling (test in a worktree to confirm the hook install path is correct).
- Optional `--force` flag for nuclear overwrite of user-edited files (per spec deliverable #6).
- Optional `--skip-settings` flag for projects with deeply customised `.claude/settings.json` that doesn't fit the marker pattern (per spec risk mitigation).

**Session 3 scope** (verification + Done):

- BATS suite — 5 scenarios from the spec (lines 76–82 of `intent/st/ST0035/WP/11/info.md`):
  1. Fresh scratch project `--apply` → all canon artefacts installed.
  2. Re-run `--apply` → zero changes (idempotence).
  3. User-edited CLAUDE.md user-section → preserved on refresh.
  4. Pre-existing non-Intent pre-commit hook → chained, not overwritten.
  5. `--dry-run` → no file modifications.
- MODULES.md audit: register the canon-install helper functions (only the new ones; existing rows stay).
- Dry-run output formatter polish (column alignment for the `.claude/scripts/<name>.sh` lines that overflow the 32-char column today; widened to 38 in Session 1 but worth a final pass).
- Done commit (`intent wp done ST0035/11`).

Downstream: WP14 (Intent self-dogfood) runs `intent claude upgrade --apply` on Intent itself as the first target. WP15 (canary) + WP16 (fleet) apply it across the rollout universe.

## Rollout universe (17 projects, unchanged)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (inside Laksa), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet (WP16) highest-delta first (Multiplyer, Arca trio), ends with Pplr.

## Resolved decisions (all 5, retargeted #1)

1. Version: **2.10.0** (retargeted from 2.9.1 mid-ST to bundle ST0036).
2. Hook enforcement: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. Reassess post-rollout.
3. Pre-commit critic threshold: CRITICAL + WARNING blocks (default; tunable per-project via `.intent_critic.yml severity_min`).
4. PostToolUse advisory critic: off by default. Opt-in via `.intent_critic.yml post_tool_use_advisory: true` + manual stanza add to `.claude/settings.local.json`.
5. Cancelled STs go to `intent/st/CANCELLED/`; deprecation annotation inline.

## Session conventions (unchanged)

- T-shirt sizing only.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (gotcha: `ST0035` or `35`, not `0035` — leading zero is parsed as octal).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- Fail-forward: no backwards-compat shims.
- Document first, code next, hard review gate.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Recent commits (chronological)

- `e999f82` — WP-11 Session 1: canon-install helpers + extended upgrade.
- `052ba9d` — session wrap for previous compact (11 of 18).
- `2e99857` — mark ST0035/WP-10 Done.
- `1ae5f61` — WP-10: delete deprecated artefacts + flip residual code paths.
- `f4c68b9` — ST0036 Phase 0 stub.
- `b760b39` — retarget ST0035 v2.9.1 → v2.10.0.
- `09cad07` — mark ST0035/WP-09 Done.
- `d3c147d` — WP-09: rewrite \_CLAUDE.md as Claude overlay.
- `61fad69` — mark ST0035/WP-08 Done.
- `546dc3d` — WP-08: root AGENTS.md generator rewrite.
