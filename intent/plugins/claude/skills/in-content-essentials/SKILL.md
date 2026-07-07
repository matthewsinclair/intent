---
description: "Web content essentials: the draft -> detrope -> revise -> structural-check -> CTA/reading-level pipeline for pages, posts, and marketing copy, backed by the prose base + content rule pack and critic-prose"
chains_to: ["in-detrope"]
---

# Web Content Essentials

Load the content discipline for a project that writes web content -- landing pages, articles, posts, marketing copy. Chained from `/in-session` when `content` is a declared language. The rules are the shared `IN-PR-*` prose base plus the `content` pack (`intent claude rules list --lang prose` / `--lang content`); this skill is the pipeline that applies them, and `critic-prose` is the reviewer.

## When to invoke

- Chained from `/in-session` when `content` is in the project's `languages`.
- At the start of a content session, before drafting a page or post.
- When the user asks to write, edit, or review web content.

## The two tiers

The content discipline splits by how a rule is checked -- mirrored by `critic-prose`'s two modes:

- **style (mechanical, default)** -- greppable checks, cheap, run on every draft (`critic-prose review`): the prose base plus the web mechanics (page meta, alt-text, link text).
- **craft (judgment, on instruction)** -- a read: scannability and web voice, the primary call to action, reading level, and the full `/in-detrope` diagnosis (`critic-prose craft-check`, or `/in-detrope`).

## The content pipeline

Detrope at every step -- do not save it for the end:

1. **Draft** -- write the page for a scanning reader: the point first, short paragraphs, subheadings that carry the argument.
2. **Mechanical detrope** -- `Task(subagent_type="critic-prose", prompt="review content/pages/<slug>.md")`: the mechanical trope pass + house-style + web mechanics (meta, alt-text, links). Fix the greppable tells before a human read.
3. **Revise for craft** -- read for scannability, voice, and reading level. On instruction, `Task(subagent_type="critic-prose", prompt="craft-check content/")` and the full `/in-detrope <page>.md`.
4. **Structural check** -- front-matter meta present (title, description, canonical); one H1, no skipped heading levels; every content image has alt text; link text describes its destination.
5. **CTA and reading-level pass** -- one clear primary call to action; language pitched to the audience.

## Detrope: two forms, one home

detrope has one home -- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md` -- and two forms:

- **Mechanical (default)** -- `IN-PR-STYLE-004` greps the catalogue's `detection: automated` regexes. critic-prose runs it on every `review`.
- **Full LLM (on instruction)** -- `/in-detrope` does the contextual and stylometric diagnosis. Run it under direct instruction before publication; critic-prose only recommends it, never runs it.

Never fork the trope catalogue; both forms read the one home.

## Rules

The content discipline is the shared `IN-PR-*` prose base plus its own `IN-CO-*` rules -- read one with `intent claude rules show <id>` (`intent claude rules list --lang prose` / `--lang content`):

Prose base -- style (mechanical), shared with every prose discipline:

- `IN-PR-STYLE-001` -- banned filler and house style (`eg` not `e.g.`)
- `IN-PR-STYLE-002` -- no vanity metrics
- `IN-PR-STYLE-003` -- one H1, no skipped heading levels
- `IN-PR-STYLE-004` -- mechanical trope pass

Content-specific -- style (mechanical):

- `IN-CO-STYLE-001` -- title, description, and canonical meta present
- `IN-CO-STYLE-002` -- every content image has meaningful alt text
- `IN-CO-STYLE-003` -- link text describes its destination

Content-specific -- craft (judgment):

- `IN-CO-CRAFT-001` -- web copy is scannable and front-loaded
- `IN-CO-CRAFT-002` -- one clear primary call to action
- `IN-CO-CRAFT-003` -- reading level matches the audience

Per-project canon (`intent/llm/RULES-content.md`, `intent/llm/ARCHITECTURE-content.md`) lands via `intent lang init content`.

## Red Flags

| Rationalisation                                     | Reality                                                                               |
| --------------------------------------------------- | ------------------------------------------------------------------------------------- |
| "I'll add the meta and alt-text before launch."     | Meta and alt-text are the cheapest SEO and a11y wins; add them as you draft.          |
| "The mechanical pass was clean, so it reads human." | The grep misses the contextual tells. Run `/in-detrope` before publishing.            |
| "More buttons give the reader more options."        | Competing CTAs cause choice paralysis; one primary action converts (IN-CO-CRAFT-002). |
| "It reads fine to me."                              | You are not the audience. Pitch the reading level to who is actually reading.         |
