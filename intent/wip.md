---
verblock: "10 Jul 2026:v1.00: cc - v2.17.1 shipped (ST0055 intent issues command)"
intent_version: 2.17.1
---

# Work In Progress

## Current State

**v2.17.1 SHIPPED (2026-07-10).** `intent issues` -- a lightweight, directory-per-issue tracker (ST0055), across two same-day releases: **v2.17.0** (minor -- the command + a pipe-in-title fix + `scripts/release` -> `bin/release`) and **v2.17.1** (patch -- `issue_file` multi-file-primary robustness + fleet issue-tree normalisation). Tags `v2.17.0` (`b7e94e2`) + `v2.17.1` (`e7360b8`), both remotes + GitHub releases. Full detail: `intent/done.md`, `intent/st/COMPLETED/ST0055/`, `docs/releases/2.17.0/`, CHANGELOG `[2.17.0]`+`[2.17.1]`.

Prior: v2.16.1 (ST0054 usage-rules v1.x), v2.16.0 (ST0053 content pack). Fleet upgrades additively. No release in flight.

## Next Up

1. **Push fleet issue-normalisation commits (separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) are local-only; Conflab already pushed.
2. **Intent issue 0002 (OPEN):** `intent todo` shows `[?]` for a non-canonical status -- route todo's status read through `canonical_status`. `intent/issues/OPEN/0002/`.
3. **Utilz-side todo guard (separate repo):** `generator: utilz todo` + symmetric refuse-to-clobber guard.
4. **AT-name traceability (vc deferral):** `acceptance.md` ATs grep-able to bats `@test` names -- hv to ratify.
5. `/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007.
6. `bin/release` v2 polish (auto config.json bump); Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007; ST0040/ST0041 deferred.

## Recent

- **2026-07-10**: v2.17.0 + v2.17.1 -- ST0055 `intent issues` + pipe fix + `scripts`->`bin` + fleet normalise. Dogfood issues 0001 (closed) + 0002 (open).
- **2026-07-09**: v2.16.1 (ST0054). **2026-07-08**: v2.16.0 (ST0053). Earlier: `intent/done.md`.

## Parked

_(None.)_
