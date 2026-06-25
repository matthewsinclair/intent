---
verblock: "25 Jun 2026:v0.93: matts - v2.13.0 fleet-swept (12 projects to 2.13.0); backup cp -R fix on main"
intent_version: 2.13.0
---

# Work In Progress

## Current State

**2026-06-25 -- v2.13.0 SHIPPED (ST0047).** Tag `v2.13.0` (commit `c6b8f70`) on both remotes + GitHub release; Intent self-upgraded clean (2.12.0 -> 2.13.0, `intent doctor` green) -- the Phase 8 canary. `intent claude start` + `intent claude ws new|list|archive|hygiene` -- the MAAC whiteboard launcher + workstream lifecycle (`intent/plugins/claude/bin/intent_claude_cwi`), promoted from the Baize prototype to first-class Intent and served centrally to every project (`find_project_root`, no per-project install). The `/in-whiteboard` skill now points at `ws new` for scaffolding (Highlander SSOT; lazy-inbox drift reconciled to the eager pre-seed). Intent dogfoods MAAC: `intent/whiteboard/` stood up with `hv` + `cc` + `vc` (no `ic`). ATs ported to `tests/unit/claude_with_intent.bats` (green); critic-shell No-Silent fixes folded in (guarded the `ws archive` `mv` + `ws new` writes); ST0047 gate GREEN (18/18). Baize prototype retired (`cc2438f`). Fleet swept (2026-06-25): the 12 in-scope `~/Devel/prj` projects upgraded to 2.13.0 + languages baselined (elixir+shell + real exceptions); Baize + Lamplight left for matts (active MAAC). A backup `cp -R` robustness fix (`e36d3b3`) is on main for the next release. Provenance: Lamplight pioneered by convention (the operational reference), Baize first productised (the MVP), Intent now first-class.

**2026-06-15 -- v2.12.0 SHIPPED (ST0043 + ST0045).** Tag `v2.12.0` (commit `4e5ac15`) on both remotes + GitHub release; post-tag wrap `5f8dace` (config.json `intent_version` -> 2.12.0 + history header finalised) pushed. Both STs closed through their own gates (ST0043 8/8, ST0045 9/9) and relocated to `intent/st/COMPLETED/`; full suite green (matts). **First fleet upgrade clean**: Lamplight 2.11.13 -> 2.12.0 ran through the new orchestrator -- state-probed ledger correctly no-op'd both already-satisfied steps (relocate_config, languages_field), single stamp last, `intent doctor` green on 2.12.0, 3.0 whiteboard skill installed. **ST0043**: convergent `intent upgrade` orchestrator (`bin/intent_upgrade` ~150 lines) + new `bin/intent_migrations` ledger + `bin/intent_helpers` pruned 2026->369 lines (all sub-v2.9.0 migration code) + single version stamper + canon-engine VERSION_BUMP/sed-portability fixes (Linux-safe). **ST0045**: Whiteboard Protocol 3.0 (per-node dirs + single-writer inboxes + `hv`) -- skill completeness + reference-vs-skill drift closed + guard test. **Close-gate**: F1 (malformed AC/AT lines block, not silently dropped); F6 left opt-in-by-presence (missing acceptance.md stays open). Narrative: `intent/history/v2.12.0.md`.

**2026-06-14 -- v2.11.14 SHIPPED (intent organize Linux fix).** `intent organize` exited 1 after the first move on bash 5.x (Linux): `((counter++))` returns 1 at zero and `set -e` aborts; macOS bash 3.2 was lenient, hiding it behind green CI from v2.11.12. The whole `((x++))` class is now `x=$((x + 1))` (six sites: `bin/intent_organize` x3, `bin/intent_helpers` x3) and pinned by `tests/unit/set_e_increment_guard.bats`. Tag `60dd82a`, both remotes + GitHub release. Narrative: `intent/history/v2.11.14.md`.

**2026-06-14 -- v2.11.13 SHIPPED (ST0044).** `acceptance.md` is now a default steel-thread doc plus the AC/AT process that makes "done" an externally-verified event. The thread was dogfooded on itself with matts as verifier -- every WP ran the five-step, and with 16/16 ACs satisfied `intent st done ST0044` passed the close-gate it built. Relocated to `intent/st/COMPLETED/ST0044/` and shipped as a standalone patch (tag `5cc676a`, both remotes + GitHub release) on opt-in-by-presence grounds. Narrative: `intent/history/v2.11.13.md`; ledger: `intent/done.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.14 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**v2.13.0 is SHIPPED + fleet-swept** (12 projects to 2.13.0, languages baselined; Baize + Lamplight matts-manual). On main for the next release: a backup `cp -R` fix (`e36d3b3`) so a broken symlink no longer aborts an upgrade. Standing backlog:

1. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
3. **Deferred backlog**: Homebrew tap; `scripts/release` v2 polish (incl. the config.json version-bump gap -- the "Upgraded Intent to X" step is still manual); CI per-platform-leg surfacing (macOS-green masked a Linux `set -e` break for four releases -- the pre-release suite + release pre-flight run macOS only); cosmetic: `intent_claude_upgrade` Phase-1 prints `(run 'intent upgrade' to bump)` even when already mid-`intent upgrade`; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. **ST0040 deferred items** (revisit on field evidence).
5. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-25**: Fleet swept to v2.13.0 -- 12 `~/Devel/prj` projects upgraded + languages baselined (elixir+shell + real exceptions: Conflab elixir/shell/lua/rust/swift, Prolix swift). Two findings logged: stale pre-2.10.0 broken symlinks aborting the backup (fixed `e36d3b3`, `cp -R`) + doc-tooling `lua` false-positives (corrected on MicroGPTEx/Courses). Baize + Lamplight matts-manual.
- **2026-06-25**: v2.13.0 shipped (tag `c6b8f70`, both remotes + GitHub release) -- ST0047 `intent claude start|ws` (MAAC whiteboard launcher + workstream lifecycle) promoted from the Baize prototype to first-class Intent; `/in-whiteboard` SSOT converged; Intent whiteboard stood up (hv+cc+vc); Baize prototype retired (`cc2438f`). See `intent/history/v2.13.0.md`.
- **2026-06-15**: v2.12.0 shipped -- ST0043 convergent `intent upgrade` orchestrator (sub-v2.9.0 migration pruned, single stamper, Linux-safe canon engine) + ST0045 Whiteboard Protocol 3.0; close-gate F1 malformed-line block. Tag `4e5ac15`; Lamplight canary clean. See `intent/history/v2.12.0.md`.
- **2026-06-14**: v2.11.14 shipped -- `intent organize` Linux fix (`((x++))` under `set -e` aborts on bash 5.x); whole class -> `x=$((x + 1))` (six sites) + guard test; tag `60dd82a`. See `intent/history/v2.11.14.md`.
- **2026-06-14**: v2.11.13 shipped (ST0044) -- `acceptance.md` as a default doc + the AC/AT process; dogfooded on itself, closed through its own gate (16/16); tag `5cc676a` on both remotes + GitHub release. See `intent/history/v2.11.13.md`.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
