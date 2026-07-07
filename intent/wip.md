---
verblock: "07 Jul 2026:v0.97: cc - 2.15.1 patch pending (wp-list width + vc follow-ups); ST0053 stood up"
intent_version: 2.15.0
---

# Work In Progress

## Current State

**v2.15.0 SHIPPED (2026-07-03).** Tag `v2.15.0` (`33e5d57`) both remotes + GitHub release; Intent self-upgraded 2.14.0 -> 2.15.0. **ST0052 -- the `author` project-type pack**, the first non-code discipline on Intent's `languages` axis (reuses the axis, not a parallel field, so a repo can be `languages: [elixir, author]`). Six WPs, 21/21 through its own gate; headless prose gate deferred (D4). Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`.

**v2.15.1 PENDING -- ready to cut.** A patch bundling the `intent wp list` terminal-width fix (user-facing) plus internal quality work. All committed + pushed to both remotes; `CHANGELOG` `## [2.15.1] - in progress` written (the wp fix is the listed entry; the internal fixes ride along unlisted). **hv cuts it:** `scripts/release v2.15.1` (runs the full suite + the confirm gate in the terminal). Contents:

- `intent wp list` sizes columns to the terminal width (was a hardcoded 30-col Title cap) -- `d2abfc7`, guard test added.
- `intent todo` enumeration single-sourced (AC-01.8 Highlander) -- `4973a30`, output byte-identical.
- `scripts/release confirm()` hardened (`/dev/tty` + stray-input tolerance) -- `06b386a`.
- Dead `get_terminal_width` calls removed in `ext`/`plugin list` -- `4eafb82`.

Fleet picks up v2.15.0 on each member's next `intent upgrade` (the `author` pack is inert until a project runs `intent lang init author`). Out of scope: Pplr, Sites (inside Laksa), llm-tropes.

## Next Up

**Cut v2.15.1** (above) -- the only gated step; needs hv's terminal.

**v2.14.1 follow-ups (from the vc audit) -- 3 of 4 closed in the 2.15.1 line:**

1. AC-01.8 enumeration Highlander -- **DONE** (`4973a30`). One `list_bucket_dirs` / `list_done_dirs` enumeration + one `done_is_member` predicate feed both the markdown and JSON emitters, which now differ only in per-thread rendering.
2. `scripts/release confirm()` -- **DONE** (`06b386a`). Reads + prompts via `/dev/tty`; strips CR/whitespace before matching; aborts cleanly when there is no tty and no `--no-confirm`.
3. `intent upgrade` false-no-op -- **CONFIRMED NOT A BUG.** `detect_project_version` reads only the project `config.json`; `get_intent_version` reads the tool `VERSION` file -- distinct sources, so a fleet member cannot read its own tool version. The v2.15.0 manual bump was release _sequencing_ (`scripts/release` stamps config after the tag), tracked under "scripts/release v2 polish" below.
4. AT-name traceability -- **DEFERRED (needs a convention).** Making `acceptance.md` ATs grep-able to bats `@test` names is a framework-wide decision (eg tag each `@test` with its `AT-id`, or bake it into `intent at`), not a one-off patch on one completed thread. hv to ratify the convention, then apply uniformly.

**v2.16.0 -- ST0053 content (web-content) pack -- STOOD UP (scope framing only).** Central decision: reuse the shared prose surface, do NOT copy `IN-AU-*` into `IN-WC-*`. Awaiting hv ratification of D1 (language code `WC`/`CO`), D2 (prose base-pack shape -- the load-bearing one), D3 (critic reuse). No WPs until ratified. Detail: `intent/st/NOT-STARTED/ST0053/`.

**Deferred:** headless `intent critic author` gate (D4 -- needs path-based file-selection + a house-style suppression layer).

**Standing backlog:**

1. `/in-review` Elixir fleet sweep -- Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. Conflab pre-existing test findings (`IN-EX-TEST-001` / `005` / `007`).
3. Homebrew tap; `scripts/release` v2 polish (config.json bump is still a manual post-tag wrap -- auto-stamp it as part of the release); CI per-platform-leg surfacing; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. ST0040 + ST0041 deferred items (revisit on field evidence).

## Recent

- **2026-07-07**: 2.15.1 patch line prepped -- `intent wp list` width fix + `intent todo` enumeration Highlander + `confirm()` hardening + dead-code cleanup; 3/4 vc follow-ups closed. ST0053 content pack stood up (awaiting scope ratification).
- **2026-07-03**: v2.15.0 shipped (`33e5d57`) -- ST0052 author project-type pack. See `intent/history/v2.15.0.md`.
- **2026-07-02**: v2.14.0 shipped -- ST0050 `intent todo` + ST0051 width. See `intent/history/v2.14.0.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
