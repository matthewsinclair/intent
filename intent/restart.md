# Claude Code Session Restart -- narrative state

## Current state (2026-07-09)

**v2.16.1 SHIPPED (ST0054 -- usage-rules.md aligned to `usage_rules` v1.x -- plus four companion chores).** Tag `v2.16.1` (`d2ddb96`) both remotes + GitHub release; post-tag wrap `18bf8cc` (config.json + CLAUDE.md -> 2.16.1, doctor green, tree clean). A patch -- documentation / skills alignment plus self-contained CLI hygiene; no generator or rule-library change.

ST0054 was surfaced by a Laksa deps-hygiene sweep (usage_rules 0.1.26 -> 1.2.6): Intent's docs, `/in-standards`, and the `_usage-rules.md` template still described the pre-v1.0, argument-driven `usage_rules`. The `usage_rules` v1.x rebuild moved config into `mix.exs` (the `:usage_rules` key), removed the CLI-argument form, and added agent-skills generation into `.claude/skills/` -- a live collision with Intent's own curated skills. The fix (docs only -- `intent agents sync` does not and should not call `mix usage_rules.sync`): rewrote the `working-with-llms.md` interop section to the config-driven model, named the two distinct `usage-rules.md` artifacts (Intent's hand-authored project contract vs the library's per-dep files), added the topical `deps/*/usage-rules/*.md` sub-rule folders to `/in-standards` + the Elixir/Ash peer skills, de-staled the `_usage-rules.md` template, and stated the coexistence policy: Intent projects stay Intent-native (curated skills + on-demand deps reads are source of truth; leave the library's skill-gen off). Ground-truthed against `../Laksa/deps/usage_rules/README.md`. 3 WPs, 6/6 ACs through the gate.

Four companion chores rode the release (hv-scoped, no new STs): **C1** -- `st sync --write` now writes deterministic canonical GFM into `steel_threads.md` (was terminal-width-dependent and linter-masked; `render_table` gained a content-fit markdown mode on the file-persist path only, display unchanged). **C2** -- `localfold` / `globalfold` fold-scope vocabulary is now Intent canon (`/in-finish` defines both; `/in-whiteboard` cross-references the per-node ops). **C3** -- `intent todo` surfaced in `/in-essentials` / `/in-start` / `/in-next`. **C4** -- `intent todo` <-> `utilz todo` mutual generator-marker guard. Both code changes (C1, C4) passed critic-shell clean.

**v2.16.0 (prior) SHIPPED (ST0053).** The `content` (web-content) pack + the `IN-PR-*` prose base; `critic-author` -> `critic-prose`. Detail: `intent/history/v2.16.0.md`.

**v2.15.1 (prior) SHIPPED.** Patch: one shared `render_table` (`st list` == `st sync`); `intent todo` Highlander; `confirm()` hardening. Tag `2cdb5b5`.

## Open follow-ups (non-blocking)

- **Utilz-side todo guard (v2.16.1 C4 follow-up, separate repo):** add `generator: utilz todo` frontmatter + the symmetric refuse-to-clobber guard to `utilz todo`, closing the mutual-guard loop (handoff note delivered to hv).
- **AT-name traceability (v2.14.1 vc-audit deferral):** make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention (tag each `@test` with its `AT-id`, or bake into `intent at`); hv to ratify, then apply uniformly.
- **`scripts/release` v2 polish:** auto-stamp the config.json version bump (still a manual post-tag wrap).
- **Deferred:** the headless `intent critic prose` gate (D4 -- path-based selection + a house-style suppression layer; the ST0053 dogfood re-confirmed the need).
- Dead link: `docs/blog/README.md` lists a post `0007` whose file is missing.

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0054/` -- closed thread docs (this release).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` -- shipped-work ledger; `CHANGELOG.md` `[2.16.1]` is this patch's release note (no separate history narrative, matching the v2.15.1 precedent).

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
