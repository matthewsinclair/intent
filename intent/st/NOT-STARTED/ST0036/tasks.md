# Tasks - ST0036: Directory relocation `.intent/` → `intent/.config/` (v2.10.0)

## Work packages (provisional — Phase 0 forensic elaboration populates each `WP/NN/info.md`)

- [ ] **WP01** — Migration function: complete `migrate_v2_9_0_to_v2_10_0` to perform the atomic relocation (.intent/ -> intent/.config/ + sentinel + recovery handling). Size: M. Deps: —.
- [ ] **WP02** — Path probes: update `bin/intent_config::load_intent_config` (project-root walk), `bin/intent_helpers::require_project_root`, `bin/intent_doctor` checks. Size: S. Deps: WP01.
- [ ] **WP03** — Literal sweep: replace `.intent/` with `intent/.config/` across `bin/`, `intent/plugins/`, `lib/`, `intent/docs/`, `intent/usr/`. Guard against matches inside `~/.intent/ext/` prose. Size: M. Deps: WP02.
- [ ] **WP04** — Template + generator updates: `lib/templates/llm/_CLAUDE.md`, root `AGENTS.md` generator, `usage-rules.md` template, any hook templates referring to `.intent/`. Size: S. Deps: WP03.
- [ ] **WP05** — BATS fixtures + helpers: `tests/lib/test_helper.bash::create_test_project` emits `intent/.config/config.json`; every BATS that asserts `.intent/...` flips. Size: M. Deps: WP03.
- [ ] **WP06** — `.gitignore` + `.treeindexignore` canonical patterns: ignore `intent/.config/cache/` + `intent/.config/backup/` (or whatever D3 resolves); remove old `.intent/*` entries. Size: XS. Deps: WP03.
- [ ] **WP07** — Migration guide doc: `intent/docs/migration-v2.10.0.md` explaining the move + user-side script updates. Size: XS. Deps: WP01.
- [ ] **WP08** — Intent self-apply: run the migration on Intent itself; verify. Deliberately lands **before** ST0035/WP14. Size: S. Deps: WP01–WP07.
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

No WP starts until Phase 0 produces all nine `WP/NN/info.md` files with forensic detail (objectives, deliverables, acceptance criteria, dependencies, risks, verification) and passes the user review checkpoint.

## Coordination with ST0035

ST0036 WPs 1–8 land between ST0035/WP13 (Intent's own CLAUDE.md update) and ST0035/WP14 (Intent self-dogfood). Once ST0036/WP08 (Intent self-apply) completes, ST0035/WP14 re-scope includes verifying _both_ canon AND directory relocation. ST0035/WP15 + WP16 roll both to canary + fleet in one pass.

## Task Notes

- All WP sizing is provisional. Forensic elaboration may split or combine WPs.
- Individual `WP/NN/info.md` files are created via `intent wp new ST0036 "<title>"` during Phase 0.
- Migration function name `migrate_v2_9_0_to_v2_10_0` is fixed (already retargeted during ST0035); WP01 fills in its body.
