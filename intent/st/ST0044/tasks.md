# Tasks - ST0044: Add in acceptance.md and supporting process

## Tasks

Work packages, T-shirt sized. WP-01/WP-02 collapsed once it was clear the doc-set glob stamps any `*.md` in the template dir -- adding the template is the whole mechanism, default on.

- [x] WP-01 (S) acceptance.md in the default doc-set -- template at `lib/templates/prj/st/ST####/acceptance.md`, stamped into every new ST by the existing `*.md` glob in `bin/intent_st`. GREEN (`tests/unit/st_commands.bats`).
- [x] WP-02 (S) acceptance.md template content -- contract preamble + AC (ST-level + per-WP) + AT sections. Provisional, refinable in place (no flip gate). Registered in MODULES.md.
- [x] WP-03 (L) `intent ac` / `intent at` instrumentation -- `list` / `status` / `green` / `red` / `na` / `satisfy`, `done`+`notdone` aliases, red-first transition guard, AC/AT grammar parser, Rust-style output.
- [x] WP-04 (M) Close-gate -- `intent st done` / `intent wp done` refuse to close until coverage is complete (green AT or non-test evidence) and sign-off is recorded; verdict computed, not hand-ticked. GREEN (`tests/unit/acceptance_close_gate.bats`, 4/4): `intent ac gate` verb + both done-handlers consult it + template re-indent so freshly stamped STs carry no live col-0 ACs (opt-in / legacy-safe). [depends on WP-03]
- [x] WP-05 (S) Template references + show/edit -- update `info.md` and `WP/info.md` templates to reference `acceptance.md` and restate no ACs (Highlander); extend `st show` / `st edit` to know the `acceptance` file type. GREEN (`tests/unit/st_new_acceptance.bats`, 2/2): ST + WP info.md templates point at acceptance.md (no restated ACs); `st show` / `st edit` / `st show all` learn the `acceptance` type; `st edit` reworked to pure emit-path (global, no editor launch) per matts.
- [ ] WP-06 (M) Skill / process integration -- map the five-step onto `/in-plan`, `/in-verify`, `/in-review`, `/in-finish`; decide `/in-acceptance` vs thread-through; update `intent/docs/working-with-llms.md` (or a dedicated process doc).
- [ ] WP-07 (S) Dogfood -- `acceptance.md` for ST0043 + ST0044 (done); run ST0044's own build through the five-step with matts as verifier (in progress -- WP-01/03/04 laps complete).
- [x] WP-08 (XS) MODULES.md registration -- register the parser + gate module before its code exists (precedes WP-03 / WP-04). Template row already added.

## Task Notes

The mechanism collapsed to "drop the template file in the dir": the `for template in "$TEMPLATE_DIR"/*.md` loop in `bin/intent_st` (~line 394) stamps every `.md`. No seam, no gate -- the template's presence is the switch, default on. WP-01 + WP-02 are the proof of that, GREEN via the dogfood lap (red witnessed by matts, then green).

Remaining substance is WP-03 (CLI instrumentation) and WP-04 (close-gate), which need the AC/AT record grammar to be stable + greppable under bash 3.2. WP-05/06 are docs/refs; WP-07 is the ongoing dogfood; WP-08 registers the WP-03/04 module.

## Dependencies

- WP-08 -> WP-03, WP-04 (register before code).
- WP-03 -> WP-04 (gate computes from parsed coverage).
- ST0044 ACs ratified (open-gate) -> WP-03 / WP-04 code-build. [satisfied: ratified]
