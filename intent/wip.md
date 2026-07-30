---
verblock: "30 Jul 2026:v1.03: cc - v2.18.0 shipped (release + upgrade convergence)"
intent_version: 2.18.0
---

# Work In Progress

## Current State

**v2.18.0 SHIPPED (2026-07-30).** Minor, because it adds a subcommand (`intent lang sync`) and changes `intent upgrade` for every consumer. It completes v2.17.4, which shipped the same morning: the generator corrections in v2.17.4 were right but could not REACH an existing project, and that turned out to be the more interesting bug. `intent upgrade` now converges the tool-managed Language Packs block via a new `lang_packs` ledger step, so a consumer picks up a generator correction by upgrading rather than by knowing which command to re-run by hand.

Alongside it, `bin/release` was fixed to stamp every sidecar BEFORE the tag. Every published tag until now was internally inconsistent -- v2.17.2, v2.17.3 and v2.17.4 each carried the previous version in `config.json` and `CLAUDE.md`, because the correcting wrap commit landed after the tag. **`v2.18.0` is verified as the first self-consistent tag Intent has cut**, and needed no wrap.

**v2.17.4 SHIPPED (2026-07-30)**, closing four issues in two pairs: **0006 + 0007** (high -- a `sed` non-match is invisible, so the acceptance parser could report a write it never performed and read a green test as unsatisfied) and **0005 + 0008** (medium -- a generator asserting into a consumer repo something true only where the tool lives). **0009** is open: the structural half of 0008, filesystem probes versus the declared `languages` array.

Prior: v2.17.3 (issue 0004), v2.17.2 (0002 + 0003), v2.17.1 + v2.17.0 (ST0055), v2.16.1 (ST0054). No release in flight.

## Next Up

1. **Consumer sweep -- now one command per project.** `intent upgrade` in Utilz / Lamplight / Baize heals both the `Bash 4.0+` prerequisite (0008) and the dangling Language Packs entries (0005) in a single pass; hand edits to `RULES-<lang>.md` survive. All three still carry the old canon. Lamplight and Baize will lose the shell prerequisite entirely unless they declare `shell` -- correct, neither is a shell project. Worth watching the first one rather than running all three back to back: the upgrade path has one real exercise behind it (a poisoned fixture), not a live estate.
2. **Lamplight contract sweep (separate repo):** ST0276 (11 rows written `status: **green`), plus `ST0298` `GREEN`, `ST0270` `BOTH`, `ST0198` `BUILT`. The parser fix does not flip these -- emphasis is deliberately not tolerated -- but every offending row is now named on `ac list` / `ac gate`, so the sweep is mechanical.
3. **Issue 0009 (low, open):** `intent agents sync` answers "what languages?" by filesystem probe while the rest of Intent reads the declared array. Acting on it changes every consumer's generated `AGENTS.md`, which is the decision the issue exists to force.
4. **hv ruling -- issue 0004 item 4 (`ac status` exit code).** The reported inconsistency does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). Current design: `status` = reporter (verdict on stdout), `gate` = gate (verdict in `$?`). Changing it is a deliberate behaviour change to a documented reporter plus a test rewrite -- own issue if wanted.
5. **Push fleet issue-normalisation commits (separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) are local-only; Conflab already pushed. Utilz-side todo guard: `generator: utilz todo` + symmetric guard.
6. `/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007.
7. AT-name traceability (vc deferral -- and now load-bearing: two ST0043 guards had to keep their `@test` names verbatim because ST0043's contract cites them); Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007; ST0040/ST0041 deferred.

## Recent

- **2026-07-30**: v2.18.0 -- `lang_packs` ledger step + `intent lang sync`; `bin/release` sidecar stamping, `INTENT_HOME` pin, dirty-tree refusal. First self-consistent tag.
- **2026-07-30**: v2.17.4 -- issues 0005 + 0006 + 0007 + 0008 fixed + closed; 0009 filed.
- **2026-07-24**: v2.17.3 -- issue 0004 (close-gate vacuous pass on an unresolvable ST/WP).
- **2026-07-13**: v2.17.2 -- issues 0002 + 0003. **2026-07-10**: v2.17.0 + v2.17.1 (ST0055). Earlier: `intent/done.md`.

## Parked

_(None.)_
