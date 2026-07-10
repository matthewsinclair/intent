# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.17.1 SHIPPED

**v2.17.1 SHIPPED (2026-07-10).** `intent issues` -- a lightweight, directory-per-issue tracker (ST0055). v2.17.0 (minor): the command + a pipe-in-title fix + `scripts/release` -> `bin/release`. v2.17.1 (patch): `issue_file` multi-file-primary robustness + fleet normalisation (Utilz/Conflab/Lamplight). Tags `v2.17.0` (`b7e94e2`) + `v2.17.1` (`e7360b8`), both remotes + GitHub releases. Detail: `intent/done.md`, `intent/st/COMPLETED/ST0055/`. Prior: v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Intent issue 0002 (OPEN): `intent todo` `[?]` on non-canonical status -- route through `canonical_status`.
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Backlog

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007; Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; ST0040/ST0041 deferred.

## Fleet

Members pick up v2.17.1 on next `intent upgrade` (intent issues command + pipe fix, additive). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`.
