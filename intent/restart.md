# Claude Code Session Restart -- narrative state

## Current state (2026-07-08)

**v2.16.0 SHIPPED (ST0053 -- the `content` pack + the `IN-PR-*` prose base).** Tag `v2.16.0` (`6a6c7d8`) both remotes + GitHub release; post-tag config wrap `dc6deca` (intent_version -> 2.16.0, doctor green). A minor -- new project-type surface + a shared base pack; opt-in, zero behaviour change until a project runs `intent lang init content`.

ST0053 is half refactor, half greenfield. The four mechanical prose-hygiene rules (banned filler, vanity metrics, heading hygiene, mechanical trope pass) were lifted out of `author` into a shared `IN-PR-*` prose base -- one copy, not a per-discipline fork -- and `author` refactored onto it (its four rules moved with migration aliases; `author` now owns only front-matter/objectives + its four craft rules). The `content` pack adds six `IN-CO-*` web rules in two tiers: `style` (mechanical) page-meta / alt-text / link-text; `craft` (judgment) scannability / primary-CTA / reading-level. `critic-author` was renamed to `critic-prose` and parameterised: it loads the base plus whichever of `author` / `content` the project declares (resolved from config `languages`). `intent lang init content` installs the canon; `/in-content-essentials` + `content -> critic-prose` dispatch wire it into `/in-session` and `/in-review`. Six WPs, 15/15 through the gate. Dogfooded on `docs/blog/*.md`: found every post missing description/canonical meta (fixed -- added, canonical = the GitHub repo location) + one trope tell ("seamless integration", reworded); the run also confirmed `overall` is the legitimate adjective sense, so it was dropped from `IN-PR-STYLE-001` (v2) per hv. Detail: `intent/st/COMPLETED/ST0053/`; narrative `intent/history/v2.16.0.md`; notes `docs/releases/2.16.0/`.

**v2.15.1 (prior) SHIPPED.** Patch: one shared `render_table` drives `st list` + `st sync` + `wp list` (fills the terminal width, content-fit floor, `st list` == `st sync`); `intent todo` enumeration Highlander; `scripts/release confirm()` hardening; CI apt hardening. Tag `2cdb5b5`, wrap `0e7039d`.

**v2.15.0 (prior) SHIPPED (ST0052).** The `author` project-type pack. Detail: `intent/history/v2.15.0.md`.

## Open follow-ups (non-blocking)

- **AT-name traceability (v2.14.1 vc-audit deferral):** make `acceptance.md` ATs grep-able to bats `@test` names -- a framework-wide convention (tag each `@test` with its `AT-id`, or bake into `intent at`); hv to ratify, then apply uniformly.
- **`scripts/release` v2 polish:** auto-stamp the config.json version bump (still a manual post-tag wrap).
- **Deferred:** the headless `intent critic prose` gate (D4 -- path-based selection + a house-style suppression layer; the ST0053 dogfood re-confirmed the need).
- Dead link: `docs/blog/README.md` lists a post `0007` whose file is missing.

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0053/` -- closed thread docs (this release).
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.16.0.md` -- shipped-work ledger / narrative.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
