---
verblock: "09 Jul 2026:v0.99: cc - v2.16.1 shipped (ST0054 usage-rules v1.x alignment + 4 companion chores)"
intent_version: 2.16.1
---

# Work In Progress

## Current State

**v2.16.1 SHIPPED (2026-07-09).** Tag `v2.16.1` (`d2ddb96`) both remotes + GitHub release; post-tag wrap `18bf8cc` (config.json + CLAUDE.md -> 2.16.1, doctor green, tree clean). A patch -- documentation / skills alignment plus self-contained CLI hygiene, no generator or rule-library change. **ST0054 -- usage-rules.md aligned to `usage_rules` v1.x** (surfaced by a Laksa deps-hygiene sweep, usage_rules 0.1.26 -> 1.2.6). `working-with-llms.md`, `/in-standards` (+ the Elixir/Ash peer skills), and the `_usage-rules.md` template described the pre-v1.0 argument-driven tool; now they describe the config-driven `:usage_rules` mix.exs model (no CLI args), name the two distinct `usage-rules.md` artifacts (Intent's hand-authored project contract vs the library's per-dep files), reference topical `deps/*/usage-rules/*.md` folders, and state the `.claude/skills` coexistence policy (Intent projects stay Intent-native; library skill-gen off). Ground-truthed against `../Laksa/deps/usage_rules/README.md` (1.2.6). 3 WPs, 6/6 ACs through the gate. Plus four companion chores bundled into the release (hv-scoped, no new STs): **C1** `st sync --write` now writes deterministic canonical GFM into `steel_threads.md` (was terminal-width-dependent and linter-masked; `render_table` gained a content-fit markdown mode routed only through the file-persist path, display unchanged); **C2** `localfold`/`globalfold` are now Intent canon (`/in-finish` + `/in-whiteboard`); **C3** `intent todo` surfaced in `/in-essentials` / `/in-start` / `/in-next`; **C4** `intent todo` <-> `utilz todo` mutual generator-marker guard (`generator: intent todo` frontmatter + refuse-to-clobber). Two critic-shell passes clean. Detail: `intent/st/COMPLETED/ST0054/`; CHANGELOG `[2.16.1]`; ledger `intent/done.md`.

**v2.16.0 (prior) SHIPPED (2026-07-08, ST0053).** The `content` (web-content) pack + the `IN-PR-*` shared prose base it stands on; `critic-author` renamed to `critic-prose`. Detail: `intent/history/v2.16.0.md`.

**v2.15.1 (prior) SHIPPED (2026-07-07).** Patch: one shared `render_table` drives `st list` + `st sync` + `wp list` (`st list` == `st sync`); `intent todo` enumeration Highlander; `confirm()` hardening. Tag `2cdb5b5`.

Fleet picks up v2.16.1 on each member's next `intent upgrade` -- the ST0054 doc/skill updates, the `intent todo` guard, and the localfold/globalfold canon (all additive/inert). Out of scope: Pplr, Sites (inside Laksa), llm-tropes.

## Next Up

No release in flight. Standing backlog:

1. **Utilz-side todo guard (v2.16.1 C4 follow-up, separate repo).** Add `generator: utilz todo` frontmatter + the symmetric refuse-to-clobber guard to `utilz todo`, closing the mutual-guard loop with `intent todo` (handoff note delivered to hv).
2. **AT-name traceability (v2.14.1 vc-audit deferral).** Make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention (tag each `@test` with its `AT-id`, or bake into `intent at`), hv to ratify then apply uniformly.
3. `/in-review` Elixir fleet sweep -- Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
4. Conflab pre-existing test findings (`IN-EX-TEST-001` / `005` / `007`).
5. `bin/release` v2 polish -- auto-stamp the config.json version bump as part of the release (it is still a manual post-tag wrap); Homebrew tap; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
6. Deferred: the headless `intent critic prose` gate (D4 -- path-based file-selection + a house-style suppression layer; the ST0053 dogfood re-confirmed the need).
7. `docs/blog/README.md` lists post `0007` whose file is missing (dead link).
8. ST0040 + ST0041 deferred items (revisit on field evidence).

## Recent

- **2026-07-09**: v2.16.1 shipped (`d2ddb96`, wrap `18bf8cc`) -- ST0054 usage-rules v1.x alignment + 4 companion chores (st-index markdown-persist fix, localfold/globalfold canon, `intent todo` surfacing, `intent`<->`utilz` todo guard). See CHANGELOG `[2.16.1]` + `intent/st/COMPLETED/ST0054/`.
- **2026-07-08**: v2.16.0 shipped (`6a6c7d8`) -- ST0053 content pack + `IN-PR-*` prose base + `critic-prose` rename. See `intent/history/v2.16.0.md`.
- **2026-07-03**: v2.15.0 shipped -- ST0052 author pack. See `intent/history/v2.15.0.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
