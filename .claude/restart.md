# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Intent has a whiteboard (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.15.1 pending (patch) atop v2.15.0 SHIPPED

**v2.15.1 is PENDING -- ready to cut.** A patch: the `intent wp list` terminal-width fix (user-facing) + internal quality work (`intent todo` enumeration Highlander, `scripts/release confirm()` hardening, dead-code cleanup). All committed + pushed both remotes; `CHANGELOG [2.15.1] - in progress` written. hv cuts it: `scripts/release v2.15.1` (runs the whole bats suite as pre-flight + the confirm gate in the terminal, so cc cannot run it headless). Commit list in `intent/wip.md`.

v2.15.0 shipped (tag `v2.15.0`, release commit `33e5d57`, both remotes + GitHub release); Intent self-upgraded 2.14.0 -> 2.15.0 (post-tag wrap `425fa59`; `intent doctor` green). A minor -- new project-type surface; opt-in, no migration. **ST0052 -- the `author` pack**, the first non-code discipline on the `languages` axis: the `AU` language code; nine `IN-AU-*` rules in two tiers (`style` mechanical / `craft` judgment); the `critic-author` subagent (two-form detrope -- the mechanical trope pass runs by default off the single `in-detrope` catalogue, the full `/in-detrope` is an on-instruction handoff it never runs); `intent lang init author` canon; and `/in-author-essentials` + `author -> critic-author` dispatch in `/in-review` + `/in-session`. Six WPs, closed 21/21 through its own gate. The headless prose gate is deferred (D4): critic-author is on-demand only. Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`; notes `docs/releases/2.15.0/`.

## Open follow-ups (non-blocking)

- **v2.14.1 (vc audit) -- 3 of 4 closed in the 2.15.1 line:** AC-01.8 enumeration Highlander DONE (`4973a30`); `scripts/release confirm()` DONE (`06b386a` -- `/dev/tty` + stray input); `intent upgrade` false-no-op CONFIRMED NOT A BUG (`detect_project_version` reads project config, `get_intent_version` reads the tool `VERSION` -- distinct sources; the manual bump was release sequencing, not a detect bug). AT-name traceability DEFERRED -- needs a framework-wide convention (tag `@test` with its `AT-id`, or bake into `intent at`); hv to ratify.
- **v2.16.0:** ST0053 content (web-content) pack STOOD UP (scope framing only) -- reuse the shared prose surface, do NOT copy `IN-AU-*` into `IN-WC-*`; awaiting hv ratification of D1 (lang code) / D2 (prose base pack, load-bearing) / D3 (critic reuse). No WPs until ratified (`intent/st/NOT-STARTED/ST0053/`). Plus the deferred headless `intent critic author` gate (D4).

## Backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Fleet

Members pick up v2.15.0 on their next `intent upgrade` -- the `author` pack is available but inert until a project runs `intent lang init author` (opt-in; zero behaviour change otherwise). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
