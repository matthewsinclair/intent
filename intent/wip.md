---
verblock: "08 Jul 2026:v0.98: cc - v2.16.0 shipped (ST0053 content pack + IN-PR-* prose base)"
intent_version: 2.16.0
---

# Work In Progress

## Current State

**v2.16.0 SHIPPED (2026-07-08).** Tag `v2.16.0` (`6a6c7d8`) both remotes + GitHub release; post-tag config wrap `dc6deca` (intent_version -> 2.16.0). A minor -- new project-type surface + a shared base pack; opt-in, zero behaviour change until a project runs `intent lang init content`. **ST0053 -- the `content` (web-content) pack + the `IN-PR-*` prose base it stands on.** The four shared mechanical rules (banned filler, vanity metrics, heading hygiene, mechanical trope pass) lifted out of `author` into a `prose` base (Highlander -- one copy, not a per-discipline fork); `author` refactored onto it. `content` adds six `IN-CO-*` web rules (page meta, alt-text, links / scannability, CTA, reading-level). `critic-author` renamed to `critic-prose`, parameterised by declared language (loads the base + whichever of author/content the project declares). `intent lang init content` canon; `/in-content-essentials` + `content -> critic-prose` dispatch. Six WPs, 15/15 through the gate; dogfooded on `docs/blog/*.md` (findings applied); `overall` dropped from `IN-PR-STYLE-001` (v2) per hv. Detail: `intent/st/COMPLETED/ST0053/`; narrative `intent/history/v2.16.0.md`; notes `docs/releases/2.16.0/`.

**v2.15.1 (prior) SHIPPED (2026-07-07).** Patch: one shared `render_table` (`bin/intent_helpers`) drives `st list` + `st sync` + `wp list`, filling the terminal width with content-fit as the floor (`st list` == `st sync`); `intent todo` enumeration Highlander; `scripts/release confirm()` hardening (`/dev/tty` + stray-input tolerance); CI apt hardening. Tag `2cdb5b5`, wrap `0e7039d`.

**v2.15.0 (prior) SHIPPED (2026-07-03, ST0052).** The `author` project-type pack -- the first non-code discipline. Detail: `intent/history/v2.15.0.md`.

Fleet picks up v2.16.0 on each member's next `intent upgrade` (the `content` pack + `IN-PR-*` base are inert until a project runs `intent lang init content`; `critic-author` is renamed to `critic-prose`). Out of scope: Pplr, Sites (inside Laksa), llm-tropes.

## Next Up

No release in flight. Standing backlog:

1. **AT-name traceability (v2.14.1 vc-audit deferral).** Make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention (tag each `@test` with its `AT-id`, or bake into `intent at`), hv to ratify then apply uniformly.
2. `/in-review` Elixir fleet sweep -- Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
3. Conflab pre-existing test findings (`IN-EX-TEST-001` / `005` / `007`).
4. `scripts/release` v2 polish -- auto-stamp the config.json version bump as part of the release (it is still a manual post-tag wrap); Homebrew tap; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
5. Deferred: the headless `intent critic prose` gate (D4 -- path-based file-selection + a house-style suppression layer; the ST0053 dogfood re-confirmed the need).
6. `docs/blog/README.md` lists post `0007` whose file is missing (dead link).
7. ST0040 + ST0041 deferred items (revisit on field evidence).

## Recent

- **2026-07-08**: v2.16.0 shipped (`6a6c7d8`) -- ST0053 content pack + `IN-PR-*` prose base + `critic-prose` rename. Also v2.15.1 shipped same session (`2cdb5b5`, shared table renderer). See `intent/history/v2.16.0.md`.
- **2026-07-03**: v2.15.0 shipped -- ST0052 author pack. See `intent/history/v2.15.0.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
