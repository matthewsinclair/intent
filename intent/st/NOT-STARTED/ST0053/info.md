---
verblock: "07 Jul 2026:v0.1: matts - Initial version"
intent_version: 2.15.0
status: Not Started
slug: content-web-content-project-type-pack
created: 20260707
completed:
---

# ST0053: Content (web-content) project-type pack

## Objective

Add a `content` (web-content) project-type pack: a second non-code discipline on Intent's `languages` axis, after the `author` pack (ST0052). A project declaring `languages: [content]` (or `[elixir, content]`, `[author, content]`, ...) gets a web-content authoring rule pack, a prose critic, canon templates, and an essentials skill -- activated the same way a code language or the author pack is. Target surface: marketing pages, product docs, landing copy, blog posts, and site content, as distinct from the author pack's book / course focus.

## Context

ST0052 stood up the `author` pack and, in doing so, built the first mechanical prose surface in Intent (banned-filler-and-house-style, no-vanity-metrics, heading-hygiene, front-matter-and-objectives, the mechanical trope pass) plus the `critic-author` two-form detrope. The web-content pack shares most of that mechanical surface; the differences are in the craft tier (web voice / SEO-adjacent structure / scannability vs long-form book continuity) and the information architecture (page + section + front-matter vs part + chapter + learning-objective).

The central design question -- to be ratified before any WP is authored -- is **reuse vs copy**. The restart framing was that content "copies the author-pack shape". Copying `IN-AU-STYLE-*` into `IN-WC-STYLE-*` would be a direct Highlander violation (two divergent copies of banned-filler / trope-pass / heading-hygiene). The Highlander-correct move is to **extract the shared mechanical surface into a common base** that both `author` and `content` reference, and let each pack own only its genuinely distinct rules. `design.md` frames this decision plus the open scope questions (language code, base-pack shape, critic reuse) for hv to rule on -- exactly as ST0052's D1-D7 were ratified before build.

This thread is a minor (new project-type surface; opt-in, zero behaviour change for non-adopting projects), so it ships as its own release (2.16.0), not folded into the 2.15.x patch line.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- ST0052 -- Author project-type pack (the pattern this thread reuses; source of the shared mechanical prose surface).
- ST0037 -- Languages config field (the axis both packs activate on).

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
