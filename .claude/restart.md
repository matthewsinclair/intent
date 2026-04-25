# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected top commits (newest first): `e999f82` · `052ba9d` · `2e99857` · `1ae5f61` · `f4c68b9` · `b760b39` · `09cad07` · `d3c147d` · `61fad69` · `546dc3d`. If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate.
3. **Read `intent/restart.md` + `intent/wip.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/11/info.md`** — WP-11 is WIP; resume from Session 2 scope below.
5. If time permits before the active WP, **read `intent/st/ST0035/info.md` + `design.md`** for canon decisions refresher, and **`intent/st/NOT-STARTED/ST0036/info.md` + `design.md` + `tasks.md`** for the ST0036 context that bundles into v2.10.0.

## State (2026-04-25, end of session — 11 of 18 Done + WP-11 Session 1 shipped + ST0036 opened)

**Intent v2.10.0 in progress. ST0035 active; WP-11 mid-flight; ST0036 sibling Phase 0 stub opened (ships bundled).**

- 11 of 18 WPs Done: **WP01–WP10 + WP12**.
- WP-11 WIP: Session 1 committed (`e999f82`); Sessions 2 + 3 remain.
- Retargeted v2.9.1 → v2.10.0 to bundle ST0036 (directory relocation `.intent/` → `intent/.config/`).
- Decisions 1–5 resolved; decision 1 retargeted to 2.10.0.
- `.intent/config.json`: `intent_version: 2.10.0`.
- `VERSION`: `2.10.0`.
- Full test suite: 762/762 green.
- `intent doctor`: clean.

## WP-11 resume target — Session 2 + 3

Full spec: `intent/st/ST0035/WP/11/info.md`. Session 1 already shipped:

- 7 canon-install helpers in `intent_claude_upgrade` (`canon_install_file`, `canon_install_script`, `canon_delete_file`, `canon_refresh_with_user_section`, `canon_substitute_placeholders`, `canon_template_matches_installed`, plus supporting primitives).
- 11 new action codes through Phases 1/2/3 (`INSTALL_SETTINGS`, `INSTALL_HOOK_SCRIPT:<name>`, `INSTALL_PRE_COMMIT`, `CHAIN_PRE_COMMIT`, `INSTALL_CRITIC_CONFIG`, `INSTALL_CLAUDE_MD`, `REFRESH_CLAUDE_MD`, `INSTALL_USAGE_RULES`, `PLANT_MODULES`, `PLANT_DECISION_TREE`, `DELETE_LEGACY_AGENTS`).
- REGENERATE/CREATE AGENTS.md now calls `intent agents sync` (the WP-08 generator), not Elixir template copy.
- `migrate_v2_9_0_to_v2_10_0` invokes `intent claude upgrade --apply` after stamp bump.
- Two pre-existing version-regex bugs fixed (multi-digit semver, placeholder-aware drift compare).
- Idempotence verified on a scratch project; full suite 762/762 green. Commit `e999f82`.

**Session 2 — edge cases + dry-run polish**:

- Diff-in-dry-run for divergent user-authored CLAUDE.md (so the user sees what the canon overlay would add vs. their version).
- Richer `CHAIN_PRE_COMMIT` instructions (a ready-to-paste snippet for the existing hook, not just a hint).
- Read-only FS / submodule / non-standard `.git` handling (test in a worktree).
- Optional `--force` flag (nuclear overwrite of user-edited files; per spec deliverable #6).
- Optional `--skip-settings` flag (per spec risk mitigation).

**Session 3 — verification + Done**:

- BATS suite — 5 scenarios from spec lines 76–82:
  1. Fresh scratch project `--apply` → all canon artefacts installed.
  2. Re-run `--apply` → zero changes (idempotence).
  3. User-edited CLAUDE.md user-section → preserved on refresh.
  4. Pre-existing non-Intent pre-commit hook → chained, not overwritten.
  5. `--dry-run` → no file modifications.
- MODULES.md audit: register the new helper functions.
- Dry-run output formatter polish (column alignment for `.claude/scripts/<name>.sh` lines).
- `intent wp done ST0035/11`.

Downstream: WP14 (Intent self-dogfood) is the first consumer. WP15 (canary) + WP16 (fleet) use this as the rollout machinery.

## Next up after WP-11

1. **WP13** (S) — Update Intent's own `CLAUDE.md` to reference the canon. Needs WP09 ✓.
2. **ST0036 Phase 0 elaboration** — populate 9 `WP/NN/info.md` files. Gate before ST0036/WP01 starts.
3. **ST0036/WP01–WP08** — migration function, path probes, literal sweep, templates, BATS, gitignore, migration guide, Intent self-apply. Land BEFORE ST0035/WP14.
4. **WP14** (S) — Self-apply canon to Intent (dogfood). Post-ST0036, this carries BOTH canon + directory relocation in one pass.
5. **WP15/WP16/WP17** — Canary + fleet rollout + verification sweep. Fleet rollout also carries both concerns.
6. **WP18** (M) — `intent/usr/*.md` audit (can run in parallel with WP15/16; must land before WP17).

See `intent/st/ST0035/tasks.md` + `intent/st/NOT-STARTED/ST0036/tasks.md` for dependency graphs.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0.

## Lessons worth keeping (cumulative across recent sessions)

- **Mid-ST version retargets are cheap before release tag.** v2.9.1 → v2.10.0 was ~5 files of string replacement when no tag existed. Check the "is it tagged?" question before committing to a bundling strategy.
- **Deprecation sweeps leave ghost readers.** Deleting `intent/llm/AGENTS.md` required updating 5 other code paths that still wrote to it (`intent_init`, `_generate_basic_agents_md`, `intent_doctor`, `intent_claude_upgrade`, a BATS test). Always grep for the deleted path and scope the WP accordingly.
- **Test suite hides stale-file false positives.** `docs_completeness.bats::agents_sync_idempotent` was passing on stale `intent/llm/AGENTS.md` content post-WP08 — both runs copied the same file that `sync` wasn't even writing anymore. Periodic audit: does each test actually exercise the code path it claims to?
- **`_replace_symlink_if_present` is the migration primitive.** Any fleet project arriving with the old layout (root AGENTS.md → symlink to intent/llm/AGENTS.md) gets transparent, idempotent migration during `intent agents sync`.
- **Idempotence requires placeholder-aware drift compare.** Any canon file whose install path runs sed substitution must use the same substitution before the drift probe (`canon_template_matches_installed` does this generically). Comparing raw template vs substituted install is a correctness bug, not a cosmetic one — second `--apply` reports DIVERGED forever.
- **Scratch-project end-to-end test catches what BATS can't.** Running `intent claude upgrade --apply` against a real `intent init` scratch dir exposed two version-regex bugs the existing BATS suite never tripped. Run scratch tests before BATS, not after — cheaper iteration loop.

## Open follow-ups (outside ST0035 + ST0036)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 / ST0036 scope.
