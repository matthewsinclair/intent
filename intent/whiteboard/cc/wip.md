---
node: cc
name: Control Claude
role: control
session_id: ff217fb1-894b-458c-b98d-f1967721af37
heartbeat_at: 2026-07-03T17:43Z
status: active
focus: "ST0052: WP01+WP02 DONE. WP02 craft tier + close (6c5bd19) -- 4 IN-AU-CRAFT judgment rules canon-valid, index regenerated, AT-02.1 green, WP02 closed through the gate. ST BLOCKED on AC-00.1 (WP-06 dogfood + hv sign-off), as designed. Next: WP03 critic-author subagent. hv docs-lean chore parked for wrap."
claims: [ST0052]
---

# Control Claude (cc)

## DOING

**Active (build): ST0052 -- author project-type pack.** hv RATIFIED (2026-07-03): `AU` schema bump YES; v1 on-demand-only YES; +2 refinements folded -- two-form detrope (D5: mechanical greppable pass by default, full `/in-detrope` only under direct instruction) and `languages`-scoped critic dispatch (D7: code critics off in an author-only project -- already free from `/in-review`'s per-language dispatch; just wire `author -> critic-author`). Scope docs (info/design/tasks) + 6 WPs created. Grounding findings: validator regex duplicated across 4 files; `/in-review` already languages-scoped; the mechanical detrope surface is NET-NEW in Intent (only the LLM skill + `trope-catalog.md` exist -- indicators live in Utilz), so v1 builds it Highlander-sourced from `llm-tropes` (D5). **WP01 DONE** (`14fe6c3`) + **discovery correction** (`e930cad`): admitting `AU` needed a 5th site I first missed -- `LANG_SUBDIRS` in `rules_lib.sh` (the canon enumerator; `id-scheme.md` had pointed at the wrong file). Without it an `IN-AU-*` rule validated by path but was invisible to `list`/`index`. Found while authoring the first WP02 rule; fixed + guard extended (4/4). **WP02 style tier DONE** (`ea54dca`): 5 `IN-AU-STYLE-*` rules, all canon-valid -- banned-filler-and-house-style, no-vanity-metrics, front-matter-and-objectives, heading-hygiene, mechanical-trope-pass. The trope pass references the `in-detrope` catalogue's automated-trope regexes (Highlander -- no duplicated indicators; D5 resolved). **Aside (matts-flagged BUG, fixed `1d4f255`):** `intent help` Other: dumped `lang`'s whole doc (inline `@short:` over-read) + showed `critic`'s raw `--` header (single-dash strip); both fixed in `bin/intent_help`, 12/12 guards. **WP02 craft tier DONE + WP02 CLOSED** (`6c5bd19`): 4 `IN-AU-CRAFT-*` judgment rules -- voice-and-register-consistency, continuity, full-trope-diagnosis (the on-instruction `/in-detrope` handoff, companion to STYLE-005), citation-and-attribution; all canon-valid, uniformly `recommendation` / critic-as-reader against the style tier's greppable Detection. `rules/index.json` regenerated (the full style+craft author set). AT-02.1 (`tests/unit/rule_pack_author.bats` -- presence, validator agreement, the style|craft tier split, the markdown textual-examples invariant, and the D5 catalogue-Highlander check) green; WP02 closed through the acceptance gate (AC-02.1 by the green AT; AC-02.2/.3/.4 by evidence). The ST stays BLOCKED on AC-00.1 (WP-06 dogfood + hv sign-off), as designed. **Next: WP03 -- `critic-author` subagent** (`agent.md` + `metadata.json` + `.manifest` registration; two-tier D3, mechanical-by-default incl. the trope pass; `/in-detrope` handoff recommendation D5).

## TODO

- hv CHORE (deferred -- "at the end"): `intent/wip.md` is massively out of date -- lean out ALL the intent docs (`wip.md`, `restart.md`, `.claude/restart.md`, `done.md`) before the next wrap.
- Build order: WP01+WP02 DONE -> **WP03 next** (critic-author subagent) -> WP04 (lang canon) / WP05 (skill + dispatch wiring) -> WP06 (dogfood + docs + close).
- `content` pack deferred -- WP02/03/04 author the shareable pattern (heading-hygiene, front-matter) it will copy.
- Deferred follow-up: headless `intent critic author` gate once D4 path-based file-selection is designed.
- Parked: v2.14.1 follow-ups (from vc's audit, in `intent/wip.md`): AC-01.8 enumeration Highlander, AT-name traceability, `intent upgrade` false-no-op + `scripts/release` `confirm()` hardening.

## Watch-outs

- Everything pushed -- main (`3f4452a`+) and the `v2.14.0` tag on both remotes; nothing outstanding.

## Decisions (ratified)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (DONE = "completed since the last flush") -- the acceptance-verify flag is closed.
- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
