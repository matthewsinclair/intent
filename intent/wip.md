---
verblock: "14 Jun 2026:v0.87: matts - v2.11.14 organize Linux fix shipped on top of ST0044's v2.11.13; ST0043 next"
intent_version: 2.11.14
---

# Work In Progress

## Current State

**2026-06-14 -- v2.11.14 SHIPPED (intent organize Linux fix).** `intent organize` exited 1 after the first move on bash 5.x (Linux): `((counter++))` returns 1 at zero and `set -e` aborts; macOS bash 3.2 was lenient, hiding it behind green CI from v2.11.12. The whole `((x++))` class is now `x=$((x + 1))` (six sites: `bin/intent_organize` x3, `bin/intent_helpers` x3) and pinned by `tests/unit/set_e_increment_guard.bats`. Tag `60dd82a`, both remotes + GitHub release. Narrative: `intent/history/v2.11.14.md`.

**2026-06-14 -- v2.11.13 SHIPPED (ST0044).** `acceptance.md` is now a default steel-thread doc plus the AC/AT process that makes "done" an externally-verified event. The thread was dogfooded on itself with matts as verifier -- every WP ran the five-step, and with 16/16 ACs satisfied `intent st done ST0044` passed the close-gate it built. Relocated to `intent/st/COMPLETED/ST0044/` and shipped as a standalone patch (tag `5cc676a`, both remotes + GitHub release) on opt-in-by-presence grounds. Narrative: `intent/history/v2.11.13.md`; ledger: `intent/done.md`.

**2026-06-11 — v2.11.12 SHIPPED.** Tag `v2.11.12` (commit `574b015`) on both remotes; GitHub release. Full ST0042 (Fable 5 review) + ST0041 (MFIC harvest). Narrative: `intent/history/v2.11.12.md`; ledger: `intent/done.md`.

Fleet picks up v2.11.14 on each member's next `intent upgrade`. Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**Active: ST0043 -- Rethink `intent upgrade`** (targets **v2.12.0 minor**, own session). Architecture-B design + ACs in `intent/st/ST0043/{info,acceptance}.md` -- ratify the ACs open-gate with matts before any code. ST0043 owns the upgrade-subsystem deletions excluded from ST0042 + the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit. **ST0044 shipped as v2.11.13** (2026-06-14); ST0043 stays its own v2.12.0 minor. Standing backlog:

1. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked.
3. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish (incl. the config.json version-bump gap -- the "Upgraded Intent to X" step is still manual); CI per-platform-leg surfacing (macOS-green masked a Linux `set -e` break for four releases -- the pre-release suite + release pre-flight run macOS only); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. **ST0040 deferred items** (revisit on field evidence).
5. **ST0041 deferred harvest candidates** (revisit on field evidence).

## Recent

- **2026-06-14**: v2.11.14 shipped -- `intent organize` Linux fix (`((x++))` under `set -e` aborts on bash 5.x); whole class -> `x=$((x + 1))` (six sites) + guard test; tag `60dd82a`. See `intent/history/v2.11.14.md`.
- **2026-06-14**: v2.11.13 shipped (ST0044) -- `acceptance.md` as a default doc + the AC/AT process; dogfooded on itself, closed through its own gate (16/16); tag `5cc676a` on both remotes + GitHub release. See `intent/history/v2.11.13.md`.
- **2026-06-11**: v2.11.12 shipped — ST0042 + ST0041. See `intent/history/v2.11.12.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
