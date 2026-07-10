# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Intent has a whiteboard (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.16.1 SHIPPED (ST0054 usage-rules v1.x alignment + 4 companion chores)

**v2.16.1 SHIPPED** (tag `v2.16.1` `d2ddb96`, both remotes + GitHub release; post-tag wrap `18bf8cc` -> config.json + CLAUDE.md 2.16.1, doctor green, tree clean). A patch -- documentation / skills alignment + self-contained CLI hygiene; no generator or rule-library change. **ST0054 -- usage-rules.md aligned to `usage_rules` v1.x** (surfaced by a Laksa deps-hygiene sweep, usage_rules 0.1.26 -> 1.2.6). Intent's docs, `/in-standards`, and the `_usage-rules.md` template described the pre-v1.0 argument-driven tool; now the config-driven `:usage_rules` mix.exs model, the two distinct `usage-rules.md` artifacts (Intent's project contract vs the library's per-dep files), topical `deps/*/usage-rules/*.md` folders, and the `.claude/skills` coexistence policy (Intent stays Intent-native; library skill-gen off). 3 WPs, 6/6 ACs through the gate. Companion chores (hv-scoped, no new STs): **C1** `st sync --write` writes deterministic canonical GFM into `steel_threads.md` (`render_table` content-fit markdown mode on the file-persist path only; display unchanged); **C2** `localfold`/`globalfold` now Intent canon (`/in-finish` + `/in-whiteboard`); **C3** `intent todo` surfaced in `/in-essentials` / `/in-start` / `/in-next`; **C4** `intent todo` <-> `utilz todo` mutual generator-marker guard. Two critic-shell passes clean. Detail: `intent/st/COMPLETED/ST0054/`; CHANGELOG `[2.16.1]`.

**v2.16.0 (prior) SHIPPED.** ST0053 -- the `content` (web-content) pack + the `IN-PR-*` prose base; `critic-author` -> `critic-prose`. Narrative `intent/history/v2.16.0.md`.

**v2.15.1 (prior) SHIPPED.** Shared `render_table` (`st list` == `st sync`) + `intent todo` Highlander + `confirm()` hardening. Tag `2cdb5b5`.

## Open follow-ups (non-blocking)

- **Utilz-side todo guard (v2.16.1 C4 follow-up, separate repo):** add `generator: utilz todo` frontmatter + the symmetric refuse-to-clobber guard to `utilz todo` (handoff note delivered to hv).
- **AT-name traceability (v2.14.1 vc deferral):** make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention; hv to ratify.
- **`bin/release` v2 polish:** auto-stamp the config.json version bump (still a manual post-tag wrap).
- **Deferred:** the headless `intent critic prose` gate (D4 -- path-based selection + house-style suppression; the ST0053 dogfood re-confirmed the need).
- Dead link: `docs/blog/README.md` lists a post `0007` whose file is missing.

## Backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Fleet

Members pick up v2.16.1 on their next `intent upgrade` -- the ST0054 doc/skill updates, the `intent todo` generator-marker guard, and the localfold/globalfold canon (all additive/inert). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`.
