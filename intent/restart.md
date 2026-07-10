# Claude Code Session Restart -- narrative state

## Current state (2026-07-10)

**v2.17.1 SHIPPED.** `intent issues` -- a lightweight, directory-per-issue tracker (ST0055) -- across two same-day releases:

- **v2.17.0** (minor): the command (5 verbs -- `list`/`add`/`show`/`close`/`open`, `--severity`, `--json`, `new` alias) + a companion fix (a `|` in any title corrupted markdown tables; `sanitize_title` strips it at st/wp/issues input, `slugify` promoted to `intent_helpers`) + `scripts/release` -> `bin/release`. Tag `b7e94e2`, wrap `20c1b5f`.
- **v2.17.1** (patch): `issue_file` prefers the frontmatter-bearing primary in multi-`.md` issue dirs (legacy adoption) + fleet issue-tree normalisation (Utilz/Conflab/Lamplight to dir-per-issue canon). Tag `e7360b8`, wrap `309d8d8`.

Both remotes + GitHub releases. ST0055 = 5 WPs, 23/23 through the gate; two critic-shell passes clean. Dogfood: Intent issues 0001 (pipe bug, closed) + 0002 (todo `[?]` gap, open). Note: 2.17.1's `bin/release` was interrupted by a terminal crash after the local commit+tag; push/GitHub-release/wrap completed by hand (idempotent). Prior: v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Intent issue 0002 (OPEN): `intent todo` `[?]` on a non-canonical status -- route todo's status read through `canonical_status` (the synonym table `intent st` already uses).
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Where detail lives

- `.claude/restart.md` -- next-session focus. `intent/wip.md` -- current state + backlog.
- `intent/done.md` -- shipped ledger (July; older months in `intent/history/YYYYMM-done.md`). `intent/st/COMPLETED/ST0055/` -- closed thread. `docs/releases/2.17.0/` + CHANGELOG `[2.17.0]`/`[2.17.1]`.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`.
