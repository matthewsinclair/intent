---
verblock: "11 Jun 2026:v0.79: matts - v2.11.12 shipped; next arc is ST0043"
intent_version: 2.11.12
---

# Work In Progress

## Current State

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) pushed to both remotes; GitHub release at https://github.com/matthewsinclair/intent/releases/tag/v2.11.12. The release carries the full ST0042 arc (Fable 5 architectural review, all nine WPs) + the ST0041 MFIC harvest; both STs Completed and in `intent/st/COMPLETED/`. Verbose narrative: `intent/history/v2.11.12.md`; terse ledger: `intent/done.md`; release notes: CHANGELOG `[2.11.12]`.

Fleet picks up v2.11.12 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

1. **ST0043 — Rethink `intent upgrade`** (WIP, not started; targets **v2.12.0 minor**). The full Architecture-B design (convergent end-state + structural-step ledger), confirmed defects, and delete/keep migration path live in `intent/st/ST0043/info.md`. This is its own arc in a fresh session. Owns: the two-installer collapse, `update_config_version` inlines, dead migration scaffolding (`needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, `create_project_backup`), and the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit (regex sweep rewrites historical migration dates).
2. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project before any remediation.
3. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked; Conflab's own backlog.
4. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish (`--rollback`, log-to-file mirror); `$N`-in-SKILL.md trap audit on remaining `in-*` skills; shell-critic-inception blog draft (`docs/blog/_drafts/`); **skill-sync script-change blind spot** — `intent claude skills sync` checksums `SKILL.md` only, so script-only edits need `install --force`; key the checksum on the whole skill dir.
5. **ST0040 deferred items** (revisit only on field evidence): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` claim-scope enforcement; `intent/.config/whiteboard.json`; `/in-whiteboard verify <stream>` subcommand.
6. **ST0041 deferred harvest candidates** (revisit only on field evidence, recorded in `intent/st/COMPLETED/ST0041/impl.md`): promote the suite-resident mechanical guards to an independent CI gate; cross-session test authorship via the whiteboard Verifier role; blocking authority for any advisory control.

## Recent

- **2026-06-11**: v2.11.12 shipped — ST0042 (Fable 5 review, nine WPs) + ST0041 (MFIC harvest). `intent st cancel` added; `intent audit` retired; `intent organize` + `intent llm usage_rules` resurrected; config eval eliminated; Highlander consolidation; canon docs reconciled; mechanical guards pinned. See `intent/history/v2.11.12.md`.
- **2026-06-03**: v2.11.11 shipped — rules-path drift fix. See `intent/history/v2.11.11.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
