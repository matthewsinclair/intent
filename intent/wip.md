---
verblock: "24 Jul 2026:v1.02: cc - v2.17.3 shipped (issue 0004 fix)"
intent_version: 2.17.3
---

# Work In Progress

## Current State

**v2.17.3 SHIPPED (2026-07-24).** Patch closing the last vacuous-pass hole in the acceptance close-gate (no ST; standing hv fix-under-issue ruling). **Issue 0004**: `intent ac gate` reported a pass for a target it never evaluated -- an unresolvable ST/WP degraded to an empty AC set, and the gate found nothing unsatisfied and exited 0 in silence, indistinguishable from a verified contract. Target resolution is now a distinct, failable step ahead of evaluation, in one resolver (`resolve_target`) the whole `ac`/`at` family shares; the `/NN` segment is validated, which nothing did before; every verdict is announced, PASS included. Closed. Tag `v2.17.3` (`2828f89`), wrap `5793d7d`, both remotes + GitHub release. Detail: `intent/done.md`, `intent/issues/CLOSED/0004/`, CHANGELOG `[2.17.3]`.

Prior: v2.17.2 (issues 0002 + 0003), v2.17.1 + v2.17.0 (ST0055 `intent issues`), v2.16.1 (ST0054), v2.16.0 (ST0053). Fleet upgrades additively. No release in flight.

## Next Up

1. **hv ruling -- issue 0004 item 4 (`ac status` exit code).** hv asked for uniform non-zero exit on a BLOCKED `ac status`; the reported inconsistency does not reproduce (exits 0 on both shapes, and `intent_acceptance_cli.bats:111` asserts it). Current design: `status` = reporter (verdict on stdout), `gate` = gate (verdict in `$?`). Changing it is a deliberate behaviour change to a documented reporter plus a test rewrite -- own issue if wanted.
2. **Push fleet issue-normalisation commits (separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) are local-only; Conflab already pushed.
3. **Utilz-side todo guard (separate repo):** `generator: utilz todo` + symmetric refuse-to-clobber guard.
4. **AT-name traceability (vc deferral):** `acceptance.md` ATs grep-able to bats `@test` names -- hv to ratify.
5. `/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007.
6. `bin/release` v2 polish (auto config.json bump); Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007; ST0040/ST0041 deferred.

## Recent

- **2026-07-24**: v2.17.3 -- issue 0004 (close-gate vacuous pass on an unresolvable ST/WP) fixed + closed.
- **2026-07-13**: v2.17.2 -- issues 0002 (`intent todo` canonical status) + 0003 (critic prose-language gate) fixed + closed.
- **2026-07-10**: v2.17.0 + v2.17.1 -- ST0055 `intent issues` + pipe fix + `scripts`->`bin` + fleet normalise. Dogfood issues 0001 + 0002 (both now closed).
- **2026-07-09**: v2.16.1 (ST0054). **2026-07-08**: v2.16.0 (ST0053). Earlier: `intent/done.md`.

## Parked

_(None.)_
