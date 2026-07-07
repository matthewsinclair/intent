# Claude Code Session Restart -- narrative state

## Current state (2026-07-07)

**v2.15.1 PENDING -- ready to cut.** A patch bundling the `intent wp list` terminal-width fix (user-facing) + internal quality work (`intent todo` enumeration Highlander, `scripts/release confirm()` hardening, dead-code cleanup). All committed + pushed both remotes; `CHANGELOG [2.15.1] - in progress` written. hv cuts it: `scripts/release v2.15.1` (full suite + confirm in the terminal). Commit list in `intent/wip.md`.

**v2.15.0 is SHIPPED (ST0052 -- the author project-type pack).** Tag `v2.15.0` (release commit `33e5d57`) on both remotes + GitHub release; Intent self-upgraded 2.14.0 -> 2.15.0 (post-tag wrap `425fa59`, `intent doctor` green). A minor -- new project-type surface; opt-in, no migration. **ST0052**: the first non-code discipline on Intent's `languages` axis. A project declaring `languages: [author]` gets an authoring rule pack (nine `IN-AU-*` rules, `style` mechanical + `craft` judgment), `critic-author` (two-form detrope: mechanical by default off the single `in-detrope` catalogue, full `/in-detrope` as an on-instruction handoff), `intent lang init author` canon, and `/in-author-essentials` + `author -> critic-author` dispatch in `/in-review` + `/in-session` -- activated exactly the way a code language is (D1: reuse the axis). Six WPs, 21/21 through its own gate; the self-dogfood (manual mechanical tier) validated D3 (mechanical produces candidates, judgment confirms) + D4 (headless prose gate deferred -- Intent's own `--` style trips the dash-overuse trope). Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`; notes `docs/releases/2.15.0/`.

**v2.14.0 (prior) SHIPPED (ST0050 + ST0051).** `intent todo` -- a flat DOING/TODO/DONE projection of `intent/st/**` that cannot drift; and a generated-file width fix (`dft_width`, default 120). Detail: `intent/st/COMPLETED/ST0050/` + `ST0051/`.

**v2.13.1 (prior) SHIPPED (ST0048 + ST0049).** Acceptance close-gate fail-by-default (`acceptance: exempt` the sole escape); retroactive release notes. Detail: `intent/st/COMPLETED/ST0048/` + `ST0049/`.

## Open follow-ups (non-blocking)

- **v2.14.1 (vc audit) -- 3 of 4 closed in the 2.15.1 line:** AC-01.8 enumeration Highlander DONE (`4973a30`); `scripts/release confirm()` DONE (`06b386a` -- `/dev/tty` + stray-input tolerance); `intent upgrade` false-no-op CONFIRMED NOT A BUG (`detect_project_version` reads project config, `get_intent_version` reads the tool `VERSION` -- distinct sources; the manual bump was release sequencing). AT-name traceability DEFERRED -- needs a framework-wide convention (tag `@test` with its `AT-id`, or bake into `intent at`), hv to ratify.
- **v2.16.0:** ST0053 content (web-content) pack STOOD UP (scope framing only) -- reuse the shared prose surface, do NOT copy `IN-AU-*`; awaiting hv ratification of D1 (lang code) / D2 (prose base pack, load-bearing) / D3 (critic reuse). No WPs until ratified. Detail: `intent/st/NOT-STARTED/ST0053/`.
- **Deferred:** the headless `intent critic author` gate (D4: path-based selection + a house-style suppression layer).

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0052/` -- closed thread docs (this release).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.15.0.md` -- shipped-work ledger / narrative.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
