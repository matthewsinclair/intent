# ARCHITECTURE-content.md

Information architecture for web content: landing pages, articles, posts, and marketing copy.

## Content Layout

- `content/pages/<slug>.md` -- standalone pages (home, pricing, about, features).
- `content/posts/NN-slug.md` or `content/blog/<slug>.md` -- articles and posts.
- `content/assets/` -- images, and their sources.

## Page Structure

- Each page opens with front-matter carrying at least `title`, `description`, and `canonical`. (`IN-CO-STYLE-001`)
- One H1 per page (the page title); headings descend one level at a time, no skips. (`IN-PR-STYLE-003`)
- Content images carry meaningful alt text; decorative images declare `alt=""`. (`IN-CO-STYLE-002`)
- Link text describes its destination -- never "click here" or a bare URL. (`IN-CO-STYLE-003`)

## Voice and Conversion

- Written to be scanned: point first, short paragraphs, subheadings that carry the argument. (`IN-CO-CRAFT-001`)
- One clear primary call to action per page with a goal; any others are visibly secondary. (`IN-CO-CRAFT-002`)
- Reading level pitched to the audience, not the author. (`IN-CO-CRAFT-003`)

## The Content Pipeline

draft -> mechanical trope pass -> revise for scannability and voice -> full `/in-detrope` (on instruction) -> structural check (meta, headings, alt-text, links) -> CTA and reading-level pass.

## Review

- `Task(subagent_type="critic-prose", prompt="review content/pages/<slug>.md")` -- the mechanical style tier (prose base + content web mechanics), on every draft.
- `Task(subagent_type="critic-prose", prompt="craft-check content/")` -- the judgment craft tier (scannability, CTA, reading level), on instruction.
- `/in-detrope content/pages/<slug>.md` -- the full contextual and stylometric trope diagnosis, under direct instruction.
