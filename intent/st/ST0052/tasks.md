# Tasks - ST0052: Author project-type pack

## Work packages (created via `intent wp new` once scope is blessed)

- [x] **WP01 -- `AU` language-code schema bump.** Add `author`/`AU` to the four v1 ID/validation sites (design.md D2): `rule-schema.md` enum, `id-scheme.md` codes table + regex doc (:155), `index-generator.md` regex doc (:128), `intent_claude_rules` validator regex (:161-162, both the check and the error string). Guard test asserting `IN-AU-*` now validates and a bad id still fails. `bin/intent_critic` NOT touched (deferred with D4). Size: S.
- [x] **WP02 -- Author rule library seed.** Author the starter `rules/author/<style|craft>/<slug>/RULE.md` set (design.md rule table), mechanical (`style`) tier first. Textual Bad/Good examples (prose has no runtime -- CI-LIMITATIONS textual-only). Regenerate `index.json`. Harvest from global `CLAUDE.md`. Includes the mechanical trope surface (D5) -- resolve its form here (⚙ rule Detection vs vendored `data/trope-indicators`, sourced Highlander from `llm-tropes`). Size: M.
- [x] **WP03 -- `critic-author` subagent.** `agent.md` (own `style`/`craft` categories + modes -- D6; Read/Grep/Bash) + `metadata.json`; register in `.manifest/global-agents.json`. Two-tier (D3): mechanical by default incl. the mechanical trope pass; emit a `/in-detrope` handoff recommendation for the full LLM diagnosis under direct instruction (D5, diogenes-handoff pattern). Size: M.
- [ ] **WP04 -- `intent lang init author` canon.** `templates/author/RULES.md` (+ `ARCHITECTURE.md` -- book/course IA: parts/chapters/modules/objectives). Verify `intent lang init author` installs `RULES-author.md`, appends the Language Packs entry, and adds `author` to config `languages`. Size: S.
- [ ] **WP05 -- Skill + dispatch wiring.** `/in-author-essentials` skill (authoring pipeline: outline -> draft -> detrope -> revise -> structural check; detrope-every-step). Wire `author -> critic-author` into the `/in-review` dispatch list + a Task example, and add the `author` row to the `/in-session` fan-out (D7 -- the code-critic exclusion is already free; this is just registration). Size: M.
- [ ] **WP06 -- Dogfood + docs + close.** Run the pack on a real authoring artifact (agentic course / a blog draft); README + usage-rules + CHANGELOG + release notes; write + satisfy `acceptance.md`; close through the gate. Size: M.

Optional follow-up (own WP or fast-follow ST): headless `intent critic author` pre-commit gate once D4 path-based file-selection is designed (the deferred `bin/intent_critic` edit).

## Task Notes

- Document-before-code: scope is ratified (design.md "Decisions ratified"). Build proceeds WP01 first (the `AU` code is the foundation the rule IDs + critic validate against).
- `content` pack is deliberately out of scope -- WP02/WP03/WP04 author the shareable pattern (heading-hygiene, front-matter, the mechanical trope surface) it will copy.
- The mechanical trope surface (D5) is net-new in Intent; do NOT re-author the trope catalogue -- add only the mechanical detection layer, sourced from `llm-tropes`.

## Dependencies

- WP01 precedes WP02 and WP03 (rule IDs and the critic need the `AU` code to validate).
- WP02 precedes WP03 (the critic loads the rule library, incl. the mechanical trope surface).
- WP04 and WP05 can run alongside WP02/WP03.
- WP06 is last (dogfood needs the pack assembled).
