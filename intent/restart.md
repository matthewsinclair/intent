# Claude Code Session Restart -- narrative state

## Current state (2026-07-03)

**v2.15.0 is SHIPPED (ST0052 -- the author project-type pack).** Tag `v2.15.0` (release commit `33e5d57`) on both remotes + GitHub release; Intent self-upgraded 2.14.0 -> 2.15.0 (post-tag wrap `425fa59`, `intent doctor` green). A minor -- new project-type surface; opt-in, no migration. **ST0052**: the first non-code discipline on Intent's `languages` axis. A project declaring `languages: [author]` gets an authoring rule pack (nine `IN-AU-*` rules, `style` mechanical + `craft` judgment), `critic-author` (two-form detrope: mechanical by default off the single `in-detrope` catalogue, full `/in-detrope` as an on-instruction handoff), `intent lang init author` canon, and `/in-author-essentials` + `author -> critic-author` dispatch in `/in-review` + `/in-session` -- activated exactly the way a code language is (D1: reuse the axis). Six WPs, 21/21 through its own gate; the self-dogfood (manual mechanical tier) validated D3 (mechanical produces candidates, judgment confirms) + D4 (headless prose gate deferred -- Intent's own `--` style trips the dash-overuse trope). Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`; notes `docs/releases/2.15.0/`.

**v2.14.0 (prior) SHIPPED (ST0050 + ST0051).** `intent todo` -- a flat DOING/TODO/DONE projection of `intent/st/**` that cannot drift; and a generated-file width fix (`dft_width`, default 120). Detail: `intent/st/COMPLETED/ST0050/` + `ST0051/`.

**v2.13.1 (prior) SHIPPED (ST0048 + ST0049).** Acceptance close-gate fail-by-default (`acceptance: exempt` the sole escape); retroactive release notes. Detail: `intent/st/COMPLETED/ST0048/` + `ST0049/`.

## Open follow-ups (non-blocking)

- **v2.14.1 (vc audit):** AC-01.8 enumeration Highlander; AT-name traceability; `intent upgrade` false-no-op (`detect_project_version` skips the config stamp for a fleet member -- recurred on the v2.15.0 wrap, bumped manually) + `scripts/release` `confirm()` reads raw stdin (should read `/dev/tty`).
- **v2.15.x:** the `content` (web-content) pack (copies the author-pack shape); the deferred headless `intent critic author` gate (D4: path-based selection + a house-style suppression layer).

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0052/` -- closed thread docs (this release).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.15.0.md` -- shipped-work ledger / narrative.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
