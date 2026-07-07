---
id: IN-AU-CRAFT-003
language: author
category: craft
severity: recommendation
title: Full trope and stylometry diagnosis
summary: >
  Beyond the mechanical trope grep, a full contextual detrope pass -- trope
  density, the non-automated tells, and stylometric fingerprints -- runs on
  direct instruction via the `/in-detrope` skill. The critic recommends it as
  a handoff; it is never part of the default review.
principles:
  - no-tells
  - honest-prose
applies_when:
  - "LLM-assisted long-form approaching publication"
  - "When the mechanical trope pass is clean but the prose still reads like a machine wrote it"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Before the mechanical pass (IN-PR-STYLE-004) has been run -- do that first, it is cheaper"
  - "Content that is legitimately about AI, where the trope vocabulary is on-topic"
tags:
  - author
  - detrope
  - tropes
  - craft
  - judgment
references: []
related_rules:
  - IN-PR-STYLE-004
aliases: []
status: active
version: 1
---

# Full trope and stylometry diagnosis

The full, contextual detrope pass -- everything grep cannot see -- runs on direct instruction, recommended by the critic as a handoff.

## Problem

The mechanical trope pass (IN-PR-STYLE-004) catches the `detection: automated` tells with grep, but the tells that give LLM prose away are mostly not mechanical: the fractal summary that restates the paragraph it just wrote, the dead metaphor reached for on cue, the relentless rule-of-three, the flattened cadence where every sentence is the same length, the density of hedges. None of these is a single pattern; each is a judgement about context, frequency, and voice across a passage. A draft can pass the mechanical grep clean and still read, unmistakably, as machine output -- and shipping it that way costs the reader's trust just as surely as an "as an AI" leak would.

## Detection

This is the judgement-tier companion to IN-PR-STYLE-004, and it is handled by handoff, not by the default review. Following the critic contract, `critic-author` does NOT run this pass by default and does NOT invoke the skill itself. When the mechanical pass is clean but the prose warrants a deeper look, the critic emits a recommendation -- "run `/in-detrope` for full trope diagnosis" -- and the human or top-level agent runs the `/in-detrope` skill under direct instruction. The skill performs the contextual and stylometric diagnosis: trope density, the non-automated tropes, cadence and voice fingerprints -- the work no grep can do.

The signal that triggers the recommendation is qualitative: a clean mechanical pass on prose that still reads as generated. The diagnosis itself is the skill's job, not the critic's.

## Bad

```markdown
The draft passed grep for the automated tropes, so it shipped -- fractal summaries, rule-of-three on every page, and a flat cadence all intact, reading like exactly what it was.
```

## Good

```markdown
The mechanical pass was clean, so the critic recommended a full pass; the author ran `/in-detrope`, it flagged the density of rule-of-three and the flattened cadence, and the draft was revised before it shipped.
```

## When This Applies

- LLM-assisted long-form nearing publication, after the mechanical pass is clean.
- Any draft that "reads like AI" without tripping the automated tells.

## When This Does Not Apply

- Before the mechanical pass has run -- IN-PR-STYLE-004 is cheaper, so do it first.
- Content that is legitimately about AI, where the trope vocabulary is on-topic and expected.

## Further Reading

- IN-PR-STYLE-004 -- the mechanical trope pass; this rule is its on-instruction, full-diagnosis companion.
- `intent/plugins/claude/skills/in-detrope/SKILL.md` -- the `/in-detrope` skill the handoff invokes.
