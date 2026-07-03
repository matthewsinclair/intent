---
description: "Authoring essentials: the outline -> draft -> detrope -> revise -> structural-check pipeline for prose and courseware, backed by the author rule pack and critic-author"
chains_to: ["in-detrope"]
---

# Authoring Essentials

Load the authoring discipline for a project that writes prose or courseware -- books, courses, long-form articles. Chained from `/in-session` when `author` is a declared language. The rules live in the `author` rule pack (`intent claude rules list --lang author`); this skill is the pipeline that applies them, and `critic-author` is the reviewer.

## When to invoke

- Chained from `/in-session` when `author` is in the project's `languages`.
- At the start of an authoring session, before drafting.
- When the user asks to write, edit, or review a chapter, module, or article.

## The two tiers

The `author` pack splits by how a rule is checked -- mirrored by `critic-author`'s two modes:

- **style (mechanical, default)** -- greppable checks, cheap, run on every draft (`critic-author review`).
- **craft (judgment, on instruction)** -- a read: voice, continuity, citation, and the full `/in-detrope` diagnosis (`critic-author craft-check`, or `/in-detrope`).

## The authoring pipeline

Detrope at every step -- do not save it for the end:

1. **Outline** -- the spine first: parts, then chapters (book) or modules (course), in order. Each unit gets front-matter (`title` + an ordering key); courseware modules get learning objectives.
2. **Draft** -- write the unit in one voice and register throughout.
3. **Mechanical detrope** -- `Task(subagent_type="critic-author", prompt="review <unit>.md")`: the mechanical trope pass + house-style checks. Fix the greppable tells before a human read.
4. **Revise** -- read for craft: voice, continuity against earlier units, citations. On instruction, `Task(subagent_type="critic-author", prompt="craft-check <work>/")` and the full `/in-detrope <unit>.md`.
5. **Structural check** -- one H1, no skipped heading levels; front-matter and objectives present; a clean heading tree.

## Detrope: two forms, one home

detrope has one home -- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md` -- and two forms:

- **Mechanical (default)** -- `IN-AU-STYLE-005` greps the catalogue's `detection: automated` regexes. critic-author runs it on every `review`.
- **Full LLM (on instruction)** -- `/in-detrope` does the contextual and stylometric diagnosis (`IN-AU-CRAFT-003`). Run it under direct instruction before publication; critic-author only recommends it, never runs it.

Never fork the trope catalogue; both forms read the one home.

## Rules

The author pack, by tier -- read one with `intent claude rules show <id>`:

Style (mechanical):

- `IN-AU-STYLE-001` -- banned filler and house style (`eg` not `e.g.`)
- `IN-AU-STYLE-002` -- no vanity metrics
- `IN-AU-STYLE-003` -- front-matter and learning objectives present
- `IN-AU-STYLE-004` -- one H1, no skipped heading levels
- `IN-AU-STYLE-005` -- mechanical trope pass

Craft (judgment):

- `IN-AU-CRAFT-001` -- voice and register consistency
- `IN-AU-CRAFT-002` -- cross-chapter continuity
- `IN-AU-CRAFT-003` -- full `/in-detrope` diagnosis (on-instruction handoff)
- `IN-AU-CRAFT-004` -- citation and attribution

Per-project canon (`intent/llm/RULES-author.md`, `intent/llm/ARCHITECTURE-author.md`) lands via `intent lang init author`.

## Red Flags

| Rationalisation                                     | Reality                                                                         |
| --------------------------------------------------- | ------------------------------------------------------------------------------- |
| "I'll detrope at the very end."                     | Detrope every step -- tells compound and get baked in.                          |
| "The mechanical pass was clean, so it reads human." | The grep misses the contextual tells. Run `/in-detrope` before publishing.      |
| "It's just prose, no need to review."               | Voice drift, broken continuity, and fabricated citations are real defects.      |
| "One quote is fine without a source."               | A single fabricated citation poisons trust in the whole work (IN-AU-CRAFT-004). |
