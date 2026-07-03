---
node: cc
name: Control Claude
role: control
session_id: ff217fb1-894b-458c-b98d-f1967721af37
heartbeat_at: 2026-07-03T15:24Z
status: active
focus: "ST0052 (author project-type pack) RATIFIED by hv: AU bump yes, v1 on-demand-only yes, +2 refinements (two-form detrope, languages-scoped dispatch). Scope docs + 6 WPs written. Build starting at WP01 (AU schema bump)."
claims: [ST0052]
---

# Control Claude (cc)

## DOING

**Active (build): ST0052 -- author project-type pack.** hv RATIFIED (2026-07-03): `AU` schema bump YES; v1 on-demand-only YES; +2 refinements folded -- two-form detrope (D5: mechanical greppable pass by default, full `/in-detrope` only under direct instruction) and `languages`-scoped critic dispatch (D7: code critics off in an author-only project -- already free from `/in-review`'s per-language dispatch; just wire `author -> critic-author`). Scope docs (info/design/tasks) + 6 WPs created. Grounding findings: validator regex duplicated across 4 files; `/in-review` already languages-scoped; the mechanical detrope surface is NET-NEW in Intent (only the LLM skill + `trope-catalog.md` exist -- indicators live in Utilz), so v1 builds it Highlander-sourced from `llm-tropes` (D5). **Next: WP01** -- add `author`/`AU` to the 4 ID/validation sites + guard; `bin/intent_critic` untouched in v1 (headless prose gate deferred, D4).

## TODO

- Build order: WP01 (`AU` bump) -> WP02 (rule library + mechanical trope surface) -> WP03 (critic-author) -> WP04 (lang canon) / WP05 (skill + dispatch wiring) -> WP06 (dogfood + docs + close).
- `content` pack deferred -- WP02/03/04 author the shareable pattern (heading-hygiene, front-matter) it will copy.
- Deferred follow-up: headless `intent critic author` gate once D4 path-based file-selection is designed.
- Parked: v2.14.1 follow-ups (from vc's audit, in `intent/wip.md`): AC-01.8 enumeration Highlander, AT-name traceability, `intent upgrade` false-no-op + `scripts/release` `confirm()` hardening.

## Watch-outs

- Everything pushed -- main (`3f4452a`+) and the `v2.14.0` tag on both remotes; nothing outstanding.

## Decisions (ratified)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (DONE = "completed since the last flush") -- the acceptance-verify flag is closed.
- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
