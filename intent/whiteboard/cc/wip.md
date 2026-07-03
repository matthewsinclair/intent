---
node: cc
name: Control Claude
role: control
session_id: ff217fb1-894b-458c-b98d-f1967721af37
heartbeat_at: 2026-07-03T15:24Z
status: active
focus: "hv assignment: design new project-type packs (content + author). First ideas + 3 naming approaches delivered; recommended reuse-languages axis + author-first. Awaiting hv steer (axis + first-pack) before scoping an ST."
claims: [ST0050, ST0051]
---

# Control Claude (cc)

## DOING

**Active (design): new project-type packs -- `content` + `author`.** hv asked for ideas on a `content` pack (editing web content) and an `author` pack (books/courseware/long-form). Delivered this session: the pack anatomy (template + rule library + `critic-<name>` + `/in-<name>-essentials`; `intent lang` is already open/template-driven, NO allowlist to change); 3 naming approaches (A reuse `languages` / B generalise the word to "packs" / C new orthogonal `domain` axis); concrete starter rule tables for both packs; and the `⚙/👁` two-tier critic split (mechanical proxies -> headless pre-commit gate per ST0039; judgment rules -> on-demand `critic-<name>` + `/in-detrope`). Recommended: **A** (composes -- a repo can be `languages: [elixir, author]`) + build **`author`** first (rules mostly already written; course/blog to dogfood). **Awaiting hv steer** (axis + first-pack) before scoping a steel thread -- naming is hv's call, not guessed.

## TODO

- Once hv steers: scope the steel thread (first pack end-to-end -- template + `⚙`-tier rule pack + critic + skill, dogfooded on one artifact), then generalise.
- Design-time check: confirm `intent doctor` + the headless `intent critic` gate don't carry a separate hardcoded language list (only `intent lang` was verified open).
- Parked: v2.14.1 follow-ups (from vc's audit, in `intent/wip.md`): AC-01.8 enumeration Highlander, AT-name traceability, `intent upgrade` false-no-op + `scripts/release` `confirm()` hardening.

## Watch-outs

- Everything pushed -- main (`3f4452a`+) and the `v2.14.0` tag on both remotes; nothing outstanding.

## Decisions (ratified)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (DONE = "completed since the last flush") -- the acceptance-verify flag is closed.
- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
