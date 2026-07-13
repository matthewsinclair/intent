# Claude Code Session Restart -- narrative state

## Current state (2026-07-13)

**v2.17.2 SHIPPED.** Patch fixing two dogfooded CLI bugs, closed as issues (no ST -- hv ruled fix-under-issue):

- **Issue 0002** -- `intent todo` rendered `[?]` for a non-canonical status string (eg the directory-name form `NOT-STARTED`); `status_box` keyed on the raw frontmatter value while `intent st` routes through `canonical_status`. Fix: `canonical_status` (the one synonym table) relocated `bin/intent_st` -> `bin/intent_helpers` (both `intent st` and `intent todo` source it, not each other); `status_box` canonicalises before mapping.
- **Issue 0003** -- the pre-commit critic gate errored (exit 2) + fail-opened on every commit in a project declaring `author`/`content`, because `intent critic` rejected prose. Fix: one language registry in `critic_runner.sh`; `intent critic` no-ops prose at exit 0 (critique is on-demand via `critic-prose`, `.md`/`.mdx`/`.html` only) + `intent critic --languages`; the gate is unchanged (defers to the exit code). A gate-side `--languages` skip was built + reverted (a broken CLI could silently skip a real code critic).

Tag `v2.17.2` (`22c409e`), wrap `e525f04`, both remotes + GitHub release. CHANGELOG `[2.17.2]` is the release note (patch precedent). Prior: v2.17.1 + v2.17.0 (ST0055 `intent issues`), v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Where detail lives

- `.claude/restart.md` -- next-session focus. `intent/wip.md` -- current state + backlog.
- `intent/done.md` -- shipped ledger (July; older months in `intent/history/YYYYMM-done.md`). `intent/st/COMPLETED/ST0055/` -- closed thread. `docs/releases/2.17.0/` + CHANGELOG `[2.17.0]`/`[2.17.1]`/`[2.17.2]`; issues 0002+0003 in `intent/issues/CLOSED/`.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`.
