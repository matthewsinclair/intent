---
verblock: "13 Jul 2026:v1.01: cc - v2.17.2 shipped (issues 0002 + 0003 fixes)"
intent_version: 2.17.2
---

# Work In Progress

## Current State

**v2.17.2 SHIPPED (2026-07-13).** Patch fixing two dogfooded CLI bugs (no ST; hv ruled fix-under-issue): **issue 0002** (`intent todo` rendered `[?]` for a non-canonical status -- `canonical_status` relocated to `bin/intent_helpers` so `intent todo` and `intent st` share the one synonym table) and **issue 0003** (the pre-commit critic gate errored + fail-opened on declared `author`/`content` -- one language registry in `critic_runner.sh`; `intent critic` no-ops prose at exit 0; the gate defers to the exit code). Both closed. Tag `v2.17.2` (`22c409e`), wrap `e525f04`, both remotes + GitHub release. Detail: `intent/done.md`, the two CLOSED issue files, CHANGELOG `[2.17.2]`.

Prior: v2.17.1 + v2.17.0 (ST0055 `intent issues`), v2.16.1 (ST0054), v2.16.0 (ST0053). Fleet upgrades additively. No release in flight.

## Next Up

1. **Push fleet issue-normalisation commits (separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) are local-only; Conflab already pushed.
2. **Utilz-side todo guard (separate repo):** `generator: utilz todo` + symmetric refuse-to-clobber guard.
3. **AT-name traceability (vc deferral):** `acceptance.md` ATs grep-able to bats `@test` names -- hv to ratify.
4. `/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007.
5. `bin/release` v2 polish (auto config.json bump); Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007; ST0040/ST0041 deferred.

## Recent

- **2026-07-13**: v2.17.2 -- issues 0002 (`intent todo` canonical status) + 0003 (critic prose-language gate) fixed + closed.
- **2026-07-10**: v2.17.0 + v2.17.1 -- ST0055 `intent issues` + pipe fix + `scripts`->`bin` + fleet normalise. Dogfood issues 0001 + 0002 (both now closed).
- **2026-07-09**: v2.16.1 (ST0054). **2026-07-08**: v2.16.0 (ST0053). Earlier: `intent/done.md`.

## Parked

_(None.)_
