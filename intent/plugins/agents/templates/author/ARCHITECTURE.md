# ARCHITECTURE-author.md

Information architecture for authored long-form: books, courseware, and long articles.

## Work Layout

One directory per book or course:

- `<work>/outline.md` -- the spine: parts, then chapters (book) or modules (course), in reading order.
- `<work>/chapters/NN-slug.md` (book) or `<work>/modules/NN-slug.md` (course) -- one file per unit, ordered by a numeric filename prefix.
- `<work>/assets/` -- figures, data, and reference material.

## Unit Structure

- Each chapter or module opens with YAML front-matter carrying at least `title` and an ordering key (`order`, or the numeric filename prefix). (`IN-AU-STYLE-003`)
- Courseware modules add a `## Learning objectives` section near the top -- the module's contract with the learner. (`IN-AU-STYLE-003`)
- One H1 per file (the unit title); headings descend one level at a time, no skips. (`IN-PR-STYLE-003`)

## Voice and Continuity

- One voice and register across the whole work; no drift into marketing copy. (`IN-AU-CRAFT-001`)
- A term is defined once and used consistently; later units never silently redefine earlier ones. For multi-part works, keep a term and notation ledger. (`IN-AU-CRAFT-002`)

## The Authoring Pipeline

outline -> draft -> mechanical trope pass -> revise -> full `/in-detrope` (on instruction) -> structural check (front-matter, headings, objectives) -> citation pass.

## Review

- `Task(subagent_type="critic-author", prompt="review <unit>.md")` -- the mechanical style tier, on every draft.
- `Task(subagent_type="critic-author", prompt="craft-check <work>/")` -- the judgment craft tier, on instruction.
- `/in-detrope <unit>.md` -- the full contextual and stylometric trope diagnosis, under direct instruction.
