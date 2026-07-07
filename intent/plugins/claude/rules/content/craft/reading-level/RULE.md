---
id: IN-CO-CRAFT-003
language: content
category: craft
severity: recommendation
title: Reading level matches the audience
summary: >
  Content is pitched at the reading level its audience actually has, not the
  author's. General-web copy reads at roughly a grade 7-9 level; a specialist
  audience tolerates more, a broad consumer audience less. Sentences run short,
  jargon is defined or dropped, and the register fits who is reading -- a
  judgement, not a formula.
principles:
  - reader-first
  - clarity
applies_when:
  - "Any web content page with a defined audience"
  - "Copy aimed at a general or consumer audience, where accessibility of language matters most"
does_not_apply_when:
  - "Deliberately technical content for a specialist audience that shares the vocabulary"
  - "Legal or regulatory text whose precise wording is fixed by requirement"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
tags:
  - content
  - web
  - readability
  - clarity
references: []
related_rules: []
aliases: []
status: active
version: 1
---

# Reading level matches the audience

Pitch the language to the reader you have, not the vocabulary you enjoy.

## Problem

Copy written at the author's reading level rather than the audience's loses readers who bounce off dense sentences, undefined jargon, and a register that does not fit them. A consumer landing page written in the sentence structure of an academic paper reads as cold and hard work; a specialist reference dumbed down to grade 5 reads as condescending. A readability score is a signal, not a verdict -- the real question is whether _this_ audience reads _this_ copy easily, which depends on their domain knowledge, why they came, and what they need to do next. That is a judgement.

## Detection

A judgement read against the stated or evident audience. The critic asks:

- Who is this for, and does the language fit them -- neither over their head nor beneath them?
- Are sentences short enough to parse on first read, or do they stack clauses?
- Is domain jargon defined on first use, or assumed?
- Does a long or nominalised phrase ("utilisation of the functionality") have a plainer form ("using it") that the audience would prefer?

## Bad

```markdown
Our solution facilitates the optimisation of cross-functional operational
workflows through the utilisation of an integrated platform architecture that
leverages synergistic capabilities to maximise stakeholder value realisation.
```

## Good

```markdown
Our tool helps teams work together in one place. Fewer tools to juggle, less
time lost switching between them, more done.
```

## When This Applies

- Consumer and general-audience pages, where plain language widens reach.
- Any page where the audience is broader or less specialist than the author.

## When This Does Not Apply

- Technical content for a specialist audience that shares the vocabulary.
- Legal or regulatory copy whose exact wording is fixed.

## Further Reading

- IN-CO-CRAFT-001 -- scannability; short sentences and a low reading level reinforce each other.
- IN-PR-STYLE-001 -- banned filler; plain language and no filler are the same instinct.
