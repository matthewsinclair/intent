---
id: IN-AU-STYLE-003
language: author
category: style
severity: recommendation
title: Chapters and modules carry front-matter and objectives
summary: >
  Each chapter or courseware module opens with YAML front-matter (at least
  title and an ordering key) and, for courseware, an explicit learning-
  objectives section. Structural metadata is what lets the work be sequenced,
  navigated, and assessed.
principles:
  - structural-integrity
applies_when:
  - "A manuscript chapter or courseware module maintained as an individual file"
  - "Courseware, where each module should state what the learner will be able to do"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Front-matter-free formats where a manifest carries the ordering instead (documented)"
  - "Narrative fiction, where learning objectives are not a concept"
tags:
  - author
  - courseware
  - structure
  - front-matter
references: []
related_rules:
  - IN-AU-STYLE-004
aliases: []
status: active
version: 1
---

# Chapters and modules carry front-matter and objectives

A chapter or module declares its metadata; a courseware module also declares what the learner will be able to do.

## Problem

A chapter file with no front-matter cannot be reliably ordered, titled in a generated table of contents, or cross-linked -- the ordering ends up encoded in filenames and drifts. A courseware module with no stated learning objectives cannot be assessed against its own goal: the reader (and the author) has no contract for what the module delivers. The metadata is scaffolding; without it the structure is implicit and fragile.

## Detection

- Chapter/module files whose first non-blank content is not a YAML front-matter block (`---`) carrying at least `title` and an ordering key (`order` / `slug` / a numeric filename prefix).
- Courseware module files with no "Objectives" / "Learning objectives" section near the top.

## Bad

```markdown
# Recursion

Recursion is when a function calls itself...
```

## Good

```markdown
---
title: Recursion
order: 7
---

# Recursion

## Learning objectives

By the end of this module you can: trace a recursive call, identify a base case, and rewrite a simple loop as recursion.

Recursion is when a function calls itself...
```

## When This Applies

- Every chapter or module maintained as its own file.
- Courseware especially -- objectives are the module's contract with the learner.

## When This Does Not Apply

- Projects where a separate manifest file carries ordering and titles (document the choice).
- Narrative fiction, where "learning objectives" do not apply (front-matter may still help).

## Further Reading

- IN-AU-STYLE-004 -- heading hygiene, the sibling structural rule.
