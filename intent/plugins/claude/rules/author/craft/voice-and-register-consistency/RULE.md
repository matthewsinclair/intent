---
id: IN-AU-CRAFT-001
language: author
category: craft
severity: recommendation
title: Consistent voice and register
summary: >
  A manuscript holds one voice and one register from first page to last. Drift
  into marketing copy, a lurch between formal and chatty, or a chapter that
  reads like a different author wrote it all break the reader's sense of a
  single guiding hand.
principles:
  - voice-consistency
  - reader-trust
applies_when:
  - "Long-form prose drafted across many files or many sessions"
  - "LLM-assisted or multi-author drafts, which are especially prone to register drift"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Deliberate register shifts (a quoted email, a character's dialogue, a call-out box with its own voice)"
  - "Anthologies where distinct authorial voices are the point"
tags:
  - author
  - voice
  - register
  - craft
  - consistency
references: []
related_rules:
  - IN-AU-CRAFT-002
aliases: []
status: active
version: 1
---

# Consistent voice and register

The whole work reads as though one person wrote it, holding a steady voice and register throughout.

## Problem

Long-form prose is drafted in pieces, over many sessions, sometimes by many hands or by an LLM. Each drafting session carries its own mood, and without a deliberate pass the seams show: a patient second-person tutorial suddenly turns into third-person academic prose; a plain-spoken chapter sprouts marketing intensifiers ("revolutionary", "seamless", "effortless"); the sentence rhythm shifts from short and declarative to long and subordinate at exactly the boundary where yesterday's writing stopped and today's began. Each seam is a small jolt, and the accumulated jolts tell the reader there was no single guiding hand -- which is the fastest way to lose their trust in the material.

## Detection

This is a critic-as-reader judgement, not a grep. Read consecutive sections and listen for tonal seams:

- A shift in person or stance (second-person "you" to impersonal "one"; tutorial to treatise) that the content does not motivate.
- Marketing register bleeding into exposition: intensifiers, superlatives, and product-launch verbs ("unlock", "supercharge") in prose that is otherwise plain.
- Abrupt changes in sentence length and rhythm at section boundaries -- often the fingerprint of a fresh drafting session or a different author.
- A chapter whose vocabulary or formality is visibly out of step with its neighbours.

No single pattern proves the violation; the signal is the seam between passages, which only a read across the boundary surfaces.

## Bad

```markdown
## Chapter 3

You'll set up the database in a few minutes -- it's easy, just follow along.

## Chapter 4

The persistence layer constitutes a foundational architectural concern whose ramifications the practitioner must rigorously interrogate before proceeding.
```

## Good

```markdown
## Chapter 3

You'll set up the database next. It takes a few minutes.

## Chapter 4

You'll design the persistence layer next. It's a foundational decision, so it is worth thinking through before you write any code.
```

## When This Applies

- Any book, course, or long-form document assembled from many files or many sessions.
- LLM-assisted drafts especially -- the model's register drifts with the prompt, so passages generated separately rarely match without an editing pass.

## When This Does Not Apply

- Deliberate register shifts: a reproduced email, a character's dialogue in fiction, a side-bar written in a knowingly different voice.
- Anthologies and edited collections where distinct authorial voices are the intended texture.

## Further Reading

- IN-AU-CRAFT-002 -- cross-chapter continuity, the sibling whole-corpus judgement rule.
- IN-AU-STYLE-001 -- banned filler and house style, the mechanical counterpart that catches the individual words register drift tends to introduce.
