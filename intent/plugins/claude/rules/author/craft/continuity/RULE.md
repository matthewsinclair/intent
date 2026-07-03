---
id: IN-AU-CRAFT-002
language: author
category: craft
severity: recommendation
title: Cross-chapter continuity
summary: >
  Terms, names, notation, and claims stay consistent across the whole work. A
  concept defined in an early chapter is not silently redefined later, a term
  is not used before it is introduced, and no chapter contradicts what another
  established.
principles:
  - continuity
  - reader-trust
applies_when:
  - "Multi-chapter books or multi-module courseware where later material builds on earlier"
  - "Any long-form work with a defined vocabulary or notation"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Standalone essays or articles with no cross-references"
  - "A deliberate reintroduction or redefinition that the text explicitly flags"
tags:
  - author
  - continuity
  - craft
  - consistency
  - structure
references: []
related_rules:
  - IN-AU-CRAFT-001
aliases: []
status: active
version: 1
---

# Cross-chapter continuity

Across the whole work a term means one thing, is introduced before it is used, and no chapter contradicts another.

## Problem

A book or course is a single argument told in order, and it depends on continuity the reader can rely on. When a term defined in chapter 2 is quietly redefined in chapter 9, when notation drifts (a value is `x` here and `n` there for the same thing), when a term is used pages before it is introduced, or when one chapter asserts what another denies, the reader cannot build a stable model of the material. These faults rarely show in any single file -- each chapter is internally fine -- so they survive a per-file review and only surface to a reader holding the whole work in mind. They erode trust faster than a local typo because they make the reader doubt they have understood correctly.

## Detection

This is a whole-corpus critic-as-reader judgement; no single-file grep suffices. Read across chapters and keep a ledger:

- Build a term-and-notation ledger: where each key term or symbol is first defined, and every later use. Flag a use before its definition, and a definition that shifts meaning between chapters.
- Flag forward references to chapters, sections, or examples that do not exist or are misnumbered.
- Flag factual or logical contradictions between chapters -- a claim in one that another denies.
- Flag inconsistent names for the same entity (a library, a character, a running example) across the work.

The unit of review is the whole corpus, not the file -- the violation lives in the relationship between chapters.

## Bad

```markdown
## Chapter 2

A _steel thread_ is a thin end-to-end slice of working functionality.

## Chapter 9

A _steel thread_ -- meaning any unit of work you track -- should be closed when its tasks are done.
```

## Good

```markdown
## Chapter 2

A _steel thread_ is a thin end-to-end slice of working functionality.

## Chapter 9

Close a _steel thread_ -- the thin end-to-end slice defined in Chapter 2 -- once its tasks are done.
```

## When This Applies

- Multi-chapter books and multi-module courses, where later material assumes earlier material.
- Any long-form work that defines its own vocabulary, notation, or running examples.

## When This Does Not Apply

- Standalone essays or articles with no cross-references to maintain.
- A deliberate redefinition the text calls out ("we now generalise the earlier definition ...").

## Further Reading

- IN-AU-CRAFT-001 -- voice and register consistency, the sibling whole-corpus judgement rule.
- IN-AU-STYLE-003 -- front-matter and objectives, which gives each chapter the metadata a continuity review reads.
