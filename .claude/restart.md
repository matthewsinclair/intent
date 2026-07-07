# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Intent has a whiteboard (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.16.0 SHIPPED (ST0053 content pack + IN-PR-* prose base)

**v2.16.0 SHIPPED** (tag `v2.16.0` `6a6c7d8`, both remotes + GitHub release; post-tag config wrap `dc6deca` -> intent_version 2.16.0, doctor green). A minor -- new project-type surface + a shared base pack; opt-in, zero behaviour change until a project runs `intent lang init content`. **ST0053 -- the `content` (web-content) pack + the `IN-PR-*` prose base.** The four shared mechanical rules (banned filler, vanity metrics, heading hygiene, mechanical trope pass) lifted out of `author` into a `prose` base (Highlander); `author` refactored onto it. `content` adds six `IN-CO-*` web rules (page meta, alt-text, links / scannability, CTA, reading-level) in `style`/`craft` tiers. `critic-author` renamed to `critic-prose`, parameterised by declared language (loads the base + author and/or content per config `languages`). `intent lang init content` canon; `/in-content-essentials` + `content -> critic-prose` dispatch. Six WPs, 15/15 through the gate; dogfooded on `docs/blog/*.md` (description/canonical added to 7 posts, "seamless" reworded); `overall` dropped from `IN-PR-STYLE-001` (v2). Detail: `intent/st/COMPLETED/ST0053/`; narrative `intent/history/v2.16.0.md`.

**v2.15.1 (prior) SHIPPED.** Shared `render_table` (terminal-fit, `st list` == `st sync`) + `intent todo` Highlander + `confirm()` hardening + CI apt. Tag `2cdb5b5`, wrap `0e7039d`.

## Open follow-ups (non-blocking)

- **AT-name traceability (v2.14.1 vc deferral):** make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention; hv to ratify.
- **`scripts/release` v2 polish:** auto-stamp the config.json version bump (still a manual post-tag wrap).
- **Deferred:** the headless `intent critic prose` gate (D4 -- path-based selection + house-style suppression; the ST0053 dogfood re-confirmed the need).
- Dead link: `docs/blog/README.md` lists a post `0007` whose file is missing.

## Backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Fleet

Members pick up v2.16.0 on their next `intent upgrade` -- the `content` pack + `IN-PR-*` base are inert until a project runs `intent lang init content`; `critic-author` is renamed to `critic-prose`. Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
