---
verblock: "03 Jul 2026:v0.96: matts - v2.15.0 SHIPPED (ST0052 author project-type pack)"
intent_version: 2.15.0
---

# Work In Progress

## Current State

**2026-07-03 -- v2.15.0 SHIPPED (ST0052).** Tag `v2.15.0` (release commit `33e5d57`) on both remotes + GitHub release; Intent self-upgraded 2.14.0 -> 2.15.0 (post-tag wrap `425fa59`, `intent doctor` green). A minor -- new project-type surface; opt-in, no migration. **ST0052 -- the `author` project-type pack**, the first non-code discipline on Intent's `languages` axis. Reuses the language-pack machinery (D1: reuse the axis, not a parallel `domain:` field), so a courseware repo can be `languages: [elixir, author]` and load both packs. Six WPs, closed 21/21 through its own gate:

- **WP01** -- the `AU` language code across the rule-id validator, schema, and the canon enumerator `LANG_SUBDIRS` (+ a discovery correction: the enumerator gates `list`/`index` and lives in `rules_lib.sh`, not where `id-scheme.md` had pointed).
- **WP02** -- nine `IN-AU-*` rules in two tiers: `style` (mechanical, greppable) + `craft` (judgment, critic-as-reader). The mechanical trope pass references the single `in-detrope` catalogue, not a forked indicator set (Highlander; D5).
- **WP03** -- `critic-author`, the first non-code critic: read-only, `review` (mechanical) / `craft-check` (judgment) modes; two-form detrope (mechanical by default; the full `/in-detrope` an on-instruction handoff it recommends but never runs).
- **WP04** -- `intent lang init author` installs `RULES-author.md` + `ARCHITECTURE-author.md` and writes config `languages` (no `intent_lang` change; D2 allowlist-free).
- **WP05** -- `/in-author-essentials` skill + `author -> critic-author` dispatch in `/in-review` (D7 author-only/mixed exclusion) + `/in-session`.
- **WP06** -- self-dogfood (manual mechanical tier, since critic-author isn't live in the building session) + docs. Every trope-pass hit was a house-style false positive -- direct evidence for D3 (mechanical produces candidates; judgment confirms) and D4 (Intent's own `--` style trips the dash-overuse trope, so a headless gate would be swamped). No real defects.

The headless prose gate is deferred (D4): `critic-author` is on-demand (`Task`) only. Detail: `intent/st/COMPLETED/ST0052/`; narrative `intent/history/v2.15.0.md`; notes `docs/releases/2.15.0/`.

**2026-07-02 -- v2.14.0 SHIPPED (ST0050 `intent todo` + ST0051 width fix).** A projected DOING/TODO/DONE view of `intent/st/**` that cannot drift + a generated-file width fix (`dft_width`, default 120). Detail: `intent/st/COMPLETED/ST0050/` + `ST0051/`; narrative `intent/history/v2.14.0.md`.

Earlier releases (v2.13.x and back): `intent/done.md` ledger + `intent/history/`.

Fleet picks up v2.15.0 on each member's next `intent upgrade` -- the `author` pack is available but inert until a project runs `intent lang init author` (opt-in; zero behaviour change otherwise). Out of scope: Pplr, Sites (inside Laksa), llm-tropes (content-only).

## Next Up

**v2.14.1 follow-ups (from the vc audit -- non-blocking):**

1. **AC-01.8 enumeration Highlander** -- `intent todo`'s markdown + JSON emitters each re-walk `intent/st/**` and duplicate the `norm < since` predicate; AC-01.8 over-claims "no second traversal". Unify the enumeration (one pass feeds both) or reword the AC (a weakening -> matts' nod).
2. **AT-name traceability** -- `acceptance.md` AT `::names` don't match the real bats `@test` names; align them.
3. **`intent upgrade` false-no-op + `scripts/release` `confirm()`** -- `detect_project_version` silently skips the config stamp for a fleet member reading its own tool version (recurred on the v2.15.0 post-tag wrap `425fa59` -- bumped manually); confirm it can't. And the push `confirm()` reads raw stdin + strict `[yY]`, so a stray escape aborts the push -- read `/dev/tty` and tolerate stray input.

**v2.15.x:**

1. **The `content` (web-content) pack** -- copies the author-pack shape (heading-hygiene, front-matter, the shared mechanical surface); ST0052 built the pattern for it.
2. **Headless `intent critic author` gate** -- deferred (D4): needs path-based file-selection (`.md` overlaps docs + the content pack) + a house-style suppression layer (the dogfood showed Intent's `--` style trips the trope catalogue's dash-overuse regex).

**Standing backlog:**

1. `/in-review` Elixir fleet sweep -- Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
2. Conflab pre-existing test findings (`IN-EX-TEST-001` / `005` / `007`).
3. Homebrew tap; `scripts/release` v2 polish (config.json bump is still a manual post-tag wrap); CI per-platform-leg surfacing (macOS-green has masked Linux `set -e` breaks); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot.
4. ST0040 + ST0041 deferred items (revisit on field evidence).

## Recent

- **2026-07-03**: v2.15.0 shipped (tag `33e5d57`) -- ST0052 author project-type pack. See `intent/history/v2.15.0.md`.
- **2026-07-02**: v2.14.0 shipped (tag `c7842f1`) -- ST0050 `intent todo` + ST0051 width. See `intent/history/v2.14.0.md`.
- **2026-06-29**: v2.13.1 shipped (tag `d01a1b2`) -- ST0048 close-gate fail-by-default + ST0049 release notes. See `intent/done.md`.
- Earlier releases: `intent/done.md` ledger + `intent/history/`.

## Parked

_(None.)_
