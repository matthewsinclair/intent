---
verblock: "11 Jun 2026:v0.78: matts - ST0042+ST0041 complete; tree release-ready; done work archived to done.md + history/"
intent_version: 2.11.11
---

# Work In Progress

## Current State

**2026-06-11 — ST0042 + ST0041 arc COMPLETE. Tree is release-ready; user runs `bash scripts/release --patch` next (v2.11.11 -> v2.11.12). Nothing pushed yet — everything rides this release.**

All nine ST0042 WPs executed and committed (one fix commit + one done commit each), full suite green at every step (final run user-verified). ST0042 and ST0041 marked Completed via `intent st done` and relocated to `intent/st/COMPLETED/`. The MFIC harvest is written into ST0041's design.md/impl.md; the execution-phase leak write-up is in ST0042's impl.md. CHANGELOG `[Unreleased]` carries the full Added/Fixed/Removed set for the release notes. Done-work detail: `intent/done.md` (terse ledger) + `intent/history/v2.11.*.md` (verbose release narratives, split out of this file).

Gate decisions that shaped the arc (2026-06-11, all executed): `intent audit` retired (Highlander; critics are the canonical engine; credo templates survive via `intent st zero`); WP-06 excluded upgrade-subsystem dead scaffolding (ST0043 owns those deletions); patch release after the ST0042 WPs, then ST0043 targets v2.12.0 minor.

## Next Up

1. **Release v2.11.12** — user runs `bash scripts/release --patch` (interactive confirm; never `--no-confirm`). After it ships: refresh the Current State block here, `intent/restart.md`, and `.claude/restart.md`.
2. **ST0043 — Rethink `intent upgrade`** (WIP, not started; targets **v2.12.0 minor**). The full Architecture-B design (convergent end-state + structural-step ledger), confirmed defects, and delete/keep migration path live in `intent/st/ST0043/info.md`. This is its own arc in a fresh session. Owns: `update_config_version` inlines, dead migration scaffolding (`needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, `create_project_backup`), and the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit (regex sweep rewrites historical migration dates).
3. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project before any remediation.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked; Conflab's own backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish (`--rollback`, log-to-file mirror); `$N`-in-SKILL.md trap audit on remaining `in-*` skills; shell-critic-inception blog draft (`docs/blog/_drafts/`); **skill-sync script-change blind spot** — `intent claude skills sync` checksums `SKILL.md` only, so script-only edits need `install --force`; key the checksum on the whole skill dir.
6. **ST0040 deferred items** (revisit only on field evidence): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` claim-scope enforcement; `intent/.config/whiteboard.json`; `/in-whiteboard verify <stream>` subcommand.
7. **ST0041 deferred harvest candidates** (revisit only on field evidence, recorded in `intent/st/COMPLETED/ST0041/impl.md`): promote the suite-resident mechanical guards to an independent CI gate; cross-session test authorship via the whiteboard Verifier role; blocking authority for any advisory control.

## Recent

- **2026-06-11**: ST0042 (Fable 5 review — review + all nine WPs) and ST0041 (MFIC exploration + harvest) completed. `intent st cancel` added; `intent audit` retired; `intent organize` + `intent llm usage_rules` resurrected; config eval eliminated; Highlander consolidation pass; canon docs reconciled; mechanical guards pinned. See `intent/done.md`.
- **2026-06-03**: v2.11.11 shipped — rules-path drift fix. See `intent/history/v2.11.11.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
