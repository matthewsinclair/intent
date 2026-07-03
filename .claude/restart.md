# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Intent has a whiteboard (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.15.0 SHIPPED (ST0052 author project-type pack)

v2.15.0 shipped (tag `v2.15.0`, release commit `33e5d57`, both remotes + GitHub release); Intent self-upgraded 2.14.0 -> 2.15.0 (post-tag wrap `425fa59`; `intent doctor` green). A minor -- new project-type surface; opt-in, no migration. **ST0052 -- the `author` pack**, the first non-code discipline on the `languages` axis: the `AU` language code; nine `IN-AU-*` rules in two tiers (`style` mechanical / `craft` judgment); the `critic-author` subagent (two-form detrope -- the mechanical trope pass runs by default off the single `in-detrope` catalogue, the full `/in-detrope` is an on-instruction handoff it never runs); `intent lang init author` canon; and `/in-author-essentials` + `author -> critic-author` dispatch in `/in-review` + `/in-session`. Six WPs, closed 21/21 through its own gate. The headless prose gate is deferred (D4): critic-author is on-demand only. Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`; notes `docs/releases/2.15.0/`.

## Open follow-ups (non-blocking)

- **v2.14.1 (from the vc audit):** AC-01.8 enumeration Highlander (`intent todo` emitters double-walk `intent/st/**`); AT-name traceability (`acceptance.md` `::names` vs real `@test`); `intent upgrade` false-no-op (`detect_project_version` skips the config stamp for a fleet member -- recurred on the v2.15.0 wrap, bumped manually) + `scripts/release` `confirm()` should read `/dev/tty`.
- **v2.15.x:** the `content` (web-content) pack -- copies the author-pack shape (heading-hygiene, front-matter, the shared mechanical surface); the deferred headless `intent critic author` gate (needs D4 path-based file-selection + a house-style suppression layer -- the dogfood showed Intent's own `--` style trips the dash-overuse trope).

## Backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Fleet

Members pick up v2.15.0 on their next `intent upgrade` -- the `author` pack is available but inert until a project runs `intent lang init author` (opt-in; zero behaviour change otherwise). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
