---
verblock: "14 Jun 2026:v0.85: matts - ST0044 COMPLETE (closed through its own gate, 16/16); ST0043 next"
intent_version: 2.11.12
---

# Work In Progress

## Current State

**2026-06-14 -- ST0044 COMPLETE.** `acceptance.md` is now a default steel-thread doc plus the AC/AT process that makes "done" an externally-verified event. The thread was dogfooded on itself with matts as verifier -- every WP ran the five-step, and with 16/16 ACs satisfied `intent st done ST0044` passed the close-gate it built. Now relocated to `intent/st/COMPLETED/ST0044/`. Pending release as a standalone patch (**v2.11.13**), on opt-in-by-presence grounds. Terse ledger: `intent/done.md`; narrative: `intent/history/v2.11.13.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.12 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0043 -- Rethink `intent upgrade`** (targets **v2.12.0 minor**, own session). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md` -- ratify the ACs open-gate with matts before any code. ST0043 owns the upgrade-subsystem deletions excluded from ST0042 + the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit. **ST0044 is done** -- ships separately as the v2.11.13 patch; ST0043 stays its own v2.12.0 minor. Standing backlog:

1. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
3. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. **ST0040 deferred items** (revisit on field evidence).
5. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: ST0044 COMPLETE -- `acceptance.md` as a default doc + the AC/AT process; dogfooded on itself, closed through its own gate (16/16). Commits `fa90bb2` / `95a3507` / `fc4f75a` / `0b1b3b5` / `9cb0bf6` on `main`, plus the WP-07 close. Ships as the v2.11.13 patch. Ledger: `intent/done.md`; narrative: `intent/history/v2.11.13.md`.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
