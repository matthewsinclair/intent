---
verblock: "14 Jun 2026:v0.86: matts - v2.11.13 shipped (ST0044 standalone patch); ST0043 next"
intent_version: 2.11.13
---

# Work In Progress

## Current State

**2026-06-14 -- v2.11.13 SHIPPED (ST0044).** `acceptance.md` is now a default steel-thread doc plus the AC/AT process that makes "done" an externally-verified event. The thread was dogfooded on itself with matts as verifier -- every WP ran the five-step, and with 16/16 ACs satisfied `intent st done ST0044` passed the close-gate it built. Relocated to `intent/st/COMPLETED/ST0044/` and shipped as a standalone patch (tag `5cc676a`, both remotes + GitHub release) on opt-in-by-presence grounds. Narrative: `intent/history/v2.11.13.md`; ledger: `intent/done.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.13 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0043 -- Rethink `intent upgrade`** (targets **v2.12.0 minor**, own session). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md` -- ratify the ACs open-gate with matts before any code. ST0043 owns the upgrade-subsystem deletions excluded from ST0042 + the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit. **ST0044 shipped as v2.11.13** (2026-06-14); ST0043 stays its own v2.12.0 minor. Standing backlog:

1. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
3. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. **ST0040 deferred items** (revisit on field evidence).
5. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: v2.11.13 shipped (ST0044) -- `acceptance.md` as a default doc + the AC/AT process; dogfooded on itself, closed through its own gate (16/16); tag `5cc676a` on both remotes + GitHub release. See `intent/history/v2.11.13.md`.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
