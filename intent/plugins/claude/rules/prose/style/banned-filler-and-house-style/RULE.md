---
id: IN-PR-STYLE-001
language: prose
category: style
severity: warning
title: No banned filler; keep to house style
summary: >
  Prose in a project with a prose discipline (author or content) holds to
  the house style: no banned filler
  (`absolutely`), `eg` never `e.g.`, and no sycophantic openers
  ("You're right"). These are mechanical tells -- cheap in any prose, and in
  LLM-assisted drafts a marker that the text was machine-smoothed.
principles:
  - house-style
  - no-tells
applies_when:
  - "Any prose file in a prose project -- author (chapters, courseware modules) or content (pages, blog drafts, essays)"
  - "LLM-assisted drafts, where machine-smoothing introduces filler and reflexive agreement"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Source material quoted verbatim -- a citation may legitimately contain the banned words"
  - "Text about the words themselves (a style guide that lists what to avoid)"
tags:
  - prose
  - house-style
  - mechanical
references: []
related_rules:
  - IN-PR-STYLE-004
aliases:
  - IN-AU-STYLE-001
status: active
version: 2
---

# No banned filler; keep to house style

Prose holds to the house style: banned filler is cut, `eg` is never `e.g.`, and no sentence opens by agreeing with the reader.

## Problem

A filler word like `absolutely` adds nothing -- delete it and the sentence is stronger. It accumulates in LLM-assisted drafts because the model reaches for it as connective tissue. (`overall` is deliberately NOT banned: an autopsy of Intent's own corpus found its hits were legitimate analytical usage -- "overall progress", "the overall objective" -- not the filler sense.) The dotted `e.g.` is needless punctuation where `eg` reads cleaner. And the reflexive "You're right" (or worse, "You're absolutely right") opener is a sycophantic tell: it front-loads agreement instead of leading with the substance. Individually small; together they mark prose as unedited machine output and cost the reader's trust.

## Detection

Case-insensitive greps over prose (skip fenced code blocks and verbatim quotes):

- `\babsolutely\b` -- banned filler.
- `\be\.g\.` -- the dotted form; the house style is `eg`.
- A sentence or paragraph opening with `You'?re (absolutely )?right` -- the sycophantic opener.

Each hit is a candidate, not a certainty -- confirm the context is prose, not a quotation or a discussion of the words themselves.

## Bad

```markdown
The results were absolutely conclusive (e.g. the second trial).

You're absolutely right that the earlier framing was off.
```

## Good

```markdown
The results were conclusive (eg the second trial).

The earlier framing was off -- here is the correction.
```

## When This Applies

- Every prose file in a project that declares `author` -- chapters, courseware modules, blog drafts, essays, long-form.
- Especially LLM-assisted drafts, where filler and reflexive agreement are introduced by the model rather than the author.

## When This Does Not Apply

- Verbatim quotations: a cited source may contain `absolutely` or `e.g.` and must not be altered.
- Meta-text about the words: a style guide that lists the banned words to explain the rule.

## Further Reading

- The project's `CLAUDE.md` house-style section -- the authoritative banned-words list for the project.
- IN-PR-STYLE-004 -- the mechanical trope pass, the sibling mechanical-tell rule.
