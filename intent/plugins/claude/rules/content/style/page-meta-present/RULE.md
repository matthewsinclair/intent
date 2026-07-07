---
id: IN-CO-STYLE-001
language: content
category: style
severity: warning
title: Every page carries title, description, and canonical meta
summary: >
  A web content page opens with front-matter carrying at least `title`, a
  `description` (the search / social snippet), and a `canonical` URL. Missing
  meta means the search engine and the social card invent their own, and
  duplicate-content pages compete with each other.
principles:
  - structural-integrity
  - discoverability
applies_when:
  - "Any web content page (landing page, article, post) maintained as a file"
  - "Pages that will be indexed or shared on social platforms"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
does_not_apply_when:
  - "Fragments or partials included into a parent page that owns the meta"
  - "Pages explicitly excluded from indexing (a documented noindex utility page)"
tags:
  - content
  - web
  - meta
  - seo
  - mechanical
references: []
related_rules: []
aliases: []
status: active
version: 1
---

# Every page carries title, description, and canonical meta

A page declares the metadata search engines and social cards read; without it they guess.

## Problem

A page with no `description` gets a snippet auto-extracted from its first paragraph -- often a nav label or a cookie notice. A page with no `canonical` competes with its own variants (trailing slash, query params, print view) for ranking, splitting authority. The `title` is the single most-weighted on-page signal; leaving it to the H1 alone forfeits the chance to write a distinct search title. The metadata is cheap to add and expensive to omit.

## Detection

- Front-matter (the `---` block, or `<head>` for HTML) missing any of `title`, `description`, `canonical`.
- A `description` that is empty, a placeholder (`TODO`, `Lorem`), or longer than ~160 characters (it will be truncated).

## Bad

```markdown
# Our Pricing

We have three plans...
```

## Good

```markdown
---
title: Pricing -- Three plans for every team size
description: Compare Starter, Team, and Enterprise plans. Transparent per-seat pricing, no setup fees, cancel anytime.
canonical: https://example.com/pricing
---

# Our Pricing

We have three plans...
```

## When This Applies

- Every indexable page maintained as its own file.
- Especially pages that earn traffic from search or social sharing.

## When This Does Not Apply

- Partials/fragments composed into a parent page that owns the meta.
- Deliberately non-indexed utility pages (document the choice).

## Further Reading

- IN-CO-CRAFT-002 -- a clear primary call to action, the conversion companion to being found.
