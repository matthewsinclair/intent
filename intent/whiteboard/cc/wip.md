---
node: cc
name: Control Claude
role: control
session_id: ff217fb1-894b-458c-b98d-f1967721af37
heartbeat_at: 2026-07-03T15:24Z
status: active
focus: "ST0052: WP01 done + e930cad (LANG_SUBDIRS discovery miss caught while authoring); WP02 style tier done (ea54dca -- 5 IN-AU-STYLE rules canon-valid). Aside: fixed the matts-flagged `intent help` Other: dump bug (1d4f255). Next: WP02 craft tier (4 judgment rules) + index regen + close. hv docs-lean chore parked for wrap."
claims: [ST0052]
---

# Control Claude (cc)

## DOING

**Active (build): ST0052 -- author project-type pack.** hv RATIFIED (2026-07-03): `AU` schema bump YES; v1 on-demand-only YES; +2 refinements folded -- two-form detrope (D5: mechanical greppable pass by default, full `/in-detrope` only under direct instruction) and `languages`-scoped critic dispatch (D7: code critics off in an author-only project -- already free from `/in-review`'s per-language dispatch; just wire `author -> critic-author`). Scope docs (info/design/tasks) + 6 WPs created. Grounding findings: validator regex duplicated across 4 files; `/in-review` already languages-scoped; the mechanical detrope surface is NET-NEW in Intent (only the LLM skill + `trope-catalog.md` exist -- indicators live in Utilz), so v1 builds it Highlander-sourced from `llm-tropes` (D5). **WP01 DONE** (`14fe6c3`) + **discovery correction** (`e930cad`): admitting `AU` needed a 5th site I first missed -- `LANG_SUBDIRS` in `rules_lib.sh` (the canon enumerator; `id-scheme.md` had pointed at the wrong file). Without it an `IN-AU-*` rule validated by path but was invisible to `list`/`index`. Found while authoring the first WP02 rule; fixed + guard extended (4/4). **WP02 style tier DONE** (`ea54dca`): 5 `IN-AU-STYLE-*` rules, all canon-valid -- banned-filler-and-house-style, no-vanity-metrics, front-matter-and-objectives, heading-hygiene, mechanical-trope-pass. The trope pass references the `in-detrope` catalogue's automated-trope regexes (Highlander -- no duplicated indicators; D5 resolved). **Aside (matts-flagged BUG, fixed `1d4f255`):** `intent help` Other: dumped `lang`'s whole doc (inline `@short:` over-read) + showed `critic`'s raw `--` header (single-dash strip); both fixed in `bin/intent_help`, 12/12 guards. **Next: WP02 craft tier** -- 4 `IN-AU-CRAFT-*` judgment rules (voice, continuity, full-detrope handoff, citation) + regenerate `index.json` + WP02 close.

## TODO

- hv CHORE (deferred -- "at the end"): `intent/wip.md` is massively out of date -- lean out ALL the intent docs (`wip.md`, `restart.md`, `.claude/restart.md`, `done.md`) before the next wrap.
- Build order: WP01 DONE -> **WP02 next** (rule library + mechanical trope surface) -> WP03 (critic-author) -> WP04 (lang canon) / WP05 (skill + dispatch wiring) -> WP06 (dogfood + docs + close).
- `content` pack deferred -- WP02/03/04 author the shareable pattern (heading-hygiene, front-matter) it will copy.
- Deferred follow-up: headless `intent critic author` gate once D4 path-based file-selection is designed.
- Parked: v2.14.1 follow-ups (from vc's audit, in `intent/wip.md`): AC-01.8 enumeration Highlander, AT-name traceability, `intent upgrade` false-no-op + `scripts/release` `confirm()` hardening.

## Watch-outs

- Everything pushed -- main (`3f4452a`+) and the `v2.14.0` tag on both remotes; nothing outstanding.

## Decisions (ratified)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (DONE = "completed since the last flush") -- the acceptance-verify flag is closed.
- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
