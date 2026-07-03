---
id: IN-AU-STYLE-005
language: author
category: style
severity: warning
title: No unremediated mechanical AI tropes
summary: >
  The mechanically-detectable LLM tropes -- AI identity leaks, fourth-wall
  breaks, and the rest of the `detection: automated` set -- are caught by
  grep before a human read. This is the default, mechanical form of detrope;
  the full contextual diagnosis is a separate, on-instruction pass.
principles:
  - no-tells
  - reader-trust
applies_when:
  - "Any prose in an author project, especially LLM-assisted drafts"
  - "Before a human review pass -- the mechanical tells are cheap to catch first"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Text explicitly about AI (a chapter on language models contains the AI-identity vocabulary legitimately)"
  - "Quoted material reproduced verbatim"
tags:
  - author
  - detrope
  - tropes
  - mechanical
references: []
related_rules:
  - IN-AU-CRAFT-003
aliases: []
status: active
version: 1
---

# No unremediated mechanical AI tropes

The greppable AI tropes are caught mechanically before a human reads the draft.

## Problem

LLM-assisted prose carries mechanical tells: direct AI-identity references ("as an AI"), fourth-wall breaks ("in this story"), dead metaphors, fractal summaries, and the rest of the trope catalogue. The subset marked `detection: automated` in the catalogue each carry a machine-readable regex -- they can and should be caught by grep before anyone spends a human read on the draft. Leaving them in marks the text as unedited machine output and costs the reader's trust.

## Detection

Highlander: the trope knowledge lives in one place -- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md`. This rule does not restate it. For each trope whose frontmatter says `detection: automated`, the catalogue supplies a `**Regex**:` line; apply those patterns to the target prose with `grep -iE` (drop the PCRE `(?i)` prefix -- `-i` covers it). A hit is a candidate; confirm the document is not itself about AI and the text is not a verbatim quote.

This is the mechanical, default form of detrope. The full contextual / stylometric diagnosis (the non-automated tropes, density judgement, voice) is IN-AU-CRAFT-003 -- an on-instruction `/in-detrope` pass, not part of the default review.

## Bad

```markdown
As an AI, I find this topic fascinating. In this story, the player enters a dark forest.
```

## Good

```markdown
The topic rewards a closer look. She stepped into the dark forest, listening.
```

## When This Applies

- Every prose file in an author project, particularly LLM-assisted drafts.
- As the first, cheap pass before a human review.

## When This Does Not Apply

- Content that is legitimately about AI (the AI-identity vocabulary is on-topic there).
- Verbatim quotations.

## Further Reading

- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md` -- the single home for trope knowledge; the automated tropes carry the regexes this rule applies.
- IN-AU-CRAFT-003 -- the full `/in-detrope` diagnosis, the on-instruction companion to this mechanical pass.
