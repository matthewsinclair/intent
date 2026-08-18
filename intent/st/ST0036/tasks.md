# Tasks - ST0036: Directory relocation `.intent/` → `intent/.config/` (v2.10.0)

## Work packages (Phase 0 elaborated 2026-04-26; each `WP/NN/info.md` carries forensic detail)

- [x] **WP01** — Migration function: complete `migrate_v2_9_0_to_v2_10_0` to perform the atomic relocation (.intent/ -> intent/.config/ + sentinel + recovery handling). Size: M. Deps: —. **Done 2026-04-26 (`4dcccce`).**
- [x] **WP02** — Path probes: update `bin/intent_config::load_intent_config` (project-root walk), `bin/intent_helpers::require_project_root`, `bin/intent_doctor` checks. Size: S. Deps: WP01. **Done 2026-04-26 (`5369afd`); narrow `detect_project_version` exception added in fix `33a99d0`.**
- [x] **WP03** — Literal sweep: replace `.intent/` with `intent/.config/` across `bin/`, `intent/plugins/`, `lib/`, `intent/docs/`, `intent/usr/`. Guard against matches inside `~/.intent/ext/` prose. Size: M. Deps: WP02. **Done 2026-04-26 (`777c5b0`).**
- [x] **WP04** — Template + generator updates: `lib/templates/llm/_CLAUDE.md`, root `AGENTS.md` generator, `usage-rules.md` template, any hook templates referring to `.intent/`. Size: S. Deps: WP03. **Done 2026-04-26 (`5f8b61e` + earlier `f04db11`); only material flip was `lib/templates/hooks/pre-commit.sh` (4 hits); `_usage-rules.md` Project Structure flip N/A (no such section).**
- [x] **WP05** — BATS fixtures + helpers: `tests/lib/test_helper.bash::create_test_project` emits `intent/.config/config.json`; every BATS that asserts `.intent/...` flips. Size: M. Deps: WP03. **Done 2026-04-26 (`b62ea58`); 11 BATS files flipped + new `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` (6 scenarios) + doctor sentinel scenario in `global_commands.bats`. Real bug fixed: macOS BSD `mktemp` does not substitute X's followed by `.md` suffix (caused `agents_sync_idempotent` false-fail).**
- [x] **WP06** — `.gitignore` + `.treeindexignore` canonical patterns: ignore `intent/.config/cache/` + `intent/.config/backup/` (or whatever D3 resolves); remove old `.intent/*` entries. Size: XS. Deps: WP03. **Done 2026-04-26 (`32df058`); new `lib/templates/_treeindexignore` is single source (Highlander cleanup of `bin/intent_treeindex` heredoc); canon installer ships it via new `INSTALL_TREEINDEXIGNORE` action.**
- [x] **WP07** — Migration guide doc: `intent/docs/migration-v2.10.0.md` explaining the move + user-side script updates. Size: XS. Deps: WP01. **Done 2026-04-26 (`1debc03`).**
- [ ] **WP08** — Intent self-apply: run the migration on Intent itself; verify. Deliberately lands **before** ST0035/WP14. Size: S. Deps: WP01–WP07. **Half-done in working tree (manual `mv .intent intent/.config` uncommitted; surfaced 0 hard-coded layout-bound bugs); next step is to formalise via `intent upgrade` so the canon-apply Phase 3 also runs, then commit the rename + canon-apply diff together.**
- [ ] **WP09** — Merge point with ST0035 fleet rollout: no standalone rollout WP; folds into ST0035/WP15 (canary) + WP16 (fleet). Size: XS (coordination note only). Deps: WP08 + ST0035/WP13.

## Dependencies at a glance

| WP   | Blocks                                      | Blocked by                 |
| ---- | ------------------------------------------- | -------------------------- |
| WP01 | 02, 07, 08                                  | —                          |
| WP02 | 03                                          | 01                         |
| WP03 | 04, 05, 06                                  | 02                         |
| WP04 | 08                                          | 03                         |
| WP05 | 08                                          | 03                         |
| WP06 | 08                                          | 03                         |
| WP07 | 08                                          | 01                         |
| WP08 | 09                                          | 01, 02, 03, 04, 05, 06, 07 |
| WP09 | — (coordination bucket, not implementation) | 08 + ST0035/WP13           |

## Phase 0 gate

Phase 0 elaboration complete (2026-04-26): all nine `WP/NN/info.md` files populated with objectives, context, deliverables, approach, acceptance criteria, dependencies, implementation notes, risks, verification steps, and exit checklists. **Awaiting user review.** No WP01 work begins until the review checkpoint passes.

## Coordination with ST0035

ST0036 WPs 1–8 land between ST0035/WP13 (Intent's own CLAUDE.md update) and ST0035/WP14 (Intent self-dogfood). Once ST0036/WP08 (Intent self-apply) completes, ST0035/WP14 re-scope includes verifying _both_ canon AND directory relocation. ST0035/WP15 + WP16 roll both to canary + fleet in one pass.

## Task Notes

- WP sizing finalised in Phase 0 elaboration. T-shirt distribution: 3 × M (WP01, WP03, WP05), 3 × S (WP02, WP04, WP08), 3 × XS (WP06, WP07, WP09).
- All nine `WP/NN/info.md` files created via `intent wp new ST0036 "<title>"` and populated 2026-04-26.
- Migration function name `migrate_v2_9_0_to_v2_10_0` is fixed (already retargeted during ST0035); WP01 fills in its body.
- Cross-WP recurring concerns surfaced during Phase 0 elaboration:
  - **Sentinel anchor coordination**: WP01 emits a recovery diagnostic pointing at `intent/docs/migration-v2.10.0.md#recovery-from-interrupted-migration`. WP07 must own that exact anchor.
  - **WP01 idempotence check**: must be on layout state (`intent/.config/` exists?) not on stamp value (`intent_version == "2.10.0"`?). Stamp-based check would early-return on Intent itself, which is already stamped 2.10.0 but not yet relocated. Locked in WP01 acceptance criteria #2.
  - **AGENTS.md cosmetic drift**: mentioned in WP04 + WP08 — same convention as ST0035 sessions: regen happens during sync; commit policy is per-WP.
  - **`~/.intent/ext/` preservation**: every WP that does literal-flips needs to guard against false-positive matches on the user-level extension root.
