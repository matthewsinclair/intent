---
id: IN-PR-STYLE-002
language: prose
category: style
severity: warning
title: No vanity metrics; size in T-shirts, not clocks
summary: >
  Project meta-prose (module intros, progress notes, release notes) never
  reports counts as achievement -- words written, chapters drafted, tests
  passing -- and never estimates in clock-time. Size in T-shirts (XS-XXL).
  Counts inflate the work; clock estimates are false precision.
principles:
  - honest-prose
  - no-tells
applies_when:
  - "Project meta-prose: module/chapter intros, progress notes, release notes, changelogs"
  - "Any place the author is tempted to quantify effort or output as a mark of merit"
applies_to:
  - "**/*.md"
does_not_apply_when:
  - "Content where a count IS the subject (a data chapter reporting real figures, a statistics lesson)"
  - "Bibliographic or index counts that serve navigation, not self-congratulation"
tags:
  - prose
  - vanity-metrics
  - mechanical
references: []
related_rules:
  - IN-PR-STYLE-001
aliases:
  - IN-AU-STYLE-002
status: active
version: 1
---

# No vanity metrics; size in T-shirts, not clocks

Effort and output are not achievements to be counted, and estimates are T-shirt sizes, not hours.

## Problem

"Drafted 12,000 words across 6 chapters, 40 exercises, 3 review passes" reads as padding: it quantifies activity, not value, and it is the kind of self-congratulatory metric an LLM reaches for to signal diligence. Clock-time estimates ("this module takes 2 hours to write") are false precision -- the number is invented and immediately wrong. Sizing work in T-shirts (XS / S / M / L / XL / XXL) carries the useful signal (relative effort) without the fake accuracy.

## Detection

Over project meta-prose (not narrative or data content):

- Counts framed as merit: a number attached to `words|pages|chapters|modules|exercises|tests|lessons` presented as an accomplishment.
- Clock-time estimates where sizing is meant: a number attached to `min|minutes|hour|hours|day|days|week|weeks` for a unit of work.

Confirm the count is self-referential effort-reporting, not real subject-matter data.

## Bad

```markdown
This week: 8,000 words, 4 chapters done, roughly 6 hours of writing. Great progress.
```

## Good

```markdown
This week: chapters 3-6 drafted (size: L). Chapter 4 needs a second pass.
```

## When This Applies

- Progress notes, module and chapter intros, release notes, changelogs -- the meta-prose around the content.
- Anywhere effort or output is quantified to signal merit.

## When This Does Not Apply

- Content where the number is the subject: a chapter reporting real research figures, a statistics or data lesson.
- Navigational counts (a bibliography of N entries) that inform rather than boast.

## Further Reading

- The project's `CLAUDE.md` -- T-shirt sizing and the no-vanity-metrics rule.
- IN-PR-STYLE-001 -- the sibling house-style rule.
