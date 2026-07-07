---
id: IN-PR-STYLE-003
language: prose
category: style
severity: recommendation
title: One H1, no skipped heading levels
summary: >
  Each document has exactly one H1 (its title) and never skips a heading
  level on the way down (no H2 straight to H4). A clean heading tree is what
  generated tables of contents, navigation, and screen readers depend on.
principles:
  - structural-integrity
applies_when:
  - "Any prose document with headings (chapters, modules, articles, long-form)"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Fragments included into a parent document, where the parent owns the H1"
  - "Formats where the H1 is supplied by front-matter/template rather than the body"
tags:
  - prose
  - structure
  - headings
  - accessibility
  - mechanical
references: []
related_rules: []
aliases:
  - IN-AU-STYLE-004
status: active
version: 1
---

# One H1, no skipped heading levels

A document has one title-level heading and a heading tree that descends one level at a time.

## Problem

Two H1s in one document mean two competing titles: a generated table of contents cannot tell which is the page title, and screen readers announce a confusing structure. Skipping a level -- an H2 followed directly by an H4 -- breaks the outline: the H4 has no H3 parent, so the document tree is malformed and navigation tools mis-nest it. The reader may not notice, but every tool that consumes the structure does.

## Detection

- More than one line matching `^# ` in a single document (multiple H1s).
- A heading whose level is more than one deeper than the previous heading (eg a `##` followed later by a `####` with no `###` between).

## Bad

```markdown
# Setup

# Configuration

#### Advanced flags
```

## Good

```markdown
# Setup

## Configuration

### Advanced flags
```

## When This Applies

- Every standalone prose document with headings.
- Especially content feeding a generated table of contents or navigation sidebar.

## When This Does Not Apply

- Partial files included into a parent document, where the parent supplies the single H1.
- Systems where the H1 comes from front-matter or a template, and the body starts at H2 by design.

## Further Reading

- Each prose discipline's front-matter rule (eg `IN-AU-STYLE-003` in the author pack) -- headings and front-matter together define a document's structure.
- A shared `IN-PR-*` prose-base rule: it applies to author and content alike.
