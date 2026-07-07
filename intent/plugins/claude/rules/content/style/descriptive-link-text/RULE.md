---
id: IN-CO-STYLE-003
language: content
category: style
severity: recommendation
title: Link text describes its destination
summary: >
  Link text says where it goes: no "click here", "read more", or a bare URL as
  the visible text. A screen-reader user tabbing through a list of links, and a
  search engine reading anchor text as a ranking signal, both get the
  destination from the words in the link, not the surrounding sentence.
principles:
  - accessibility
  - discoverability
applies_when:
  - "Any web content page with links"
  - "Markdown, MDX, or HTML anchors"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
does_not_apply_when:
  - "A URL shown deliberately as text (a citation, a command to copy, documentation of the address itself)"
  - "In-product UI microcopy where a button label is constrained by design, not content"
tags:
  - content
  - web
  - accessibility
  - links
  - mechanical
references: []
related_rules: []
aliases: []
status: active
version: 1
---

# Link text describes its destination

The words inside the link say where it goes; the reader should not need the sentence around it.

## Problem

Assistive technology can present all of a page's links as a standalone list. A list of "click here, click here, read more, here" is useless -- none of them says where it goes. The same anchor text is a ranking signal: a search engine reads "click here" and learns nothing about the destination, while descriptive anchor text tells it (and the reader) exactly what is on the other side. A bare URL as visible text is read out character by character by a screen reader -- punishing to listen to.

## Detection

- Link text matching `click here`, `here`, `read more`, `learn more`, `this link`, `link` (case-insensitive) as the whole anchor.
- A bare URL (`https?://...`) used as the visible link text rather than a described destination.
- Empty or placeholder `href` (`#`, `TODO`, `javascript:void(0)`) on a content link.

## Bad

```markdown
For pricing details, [click here](/pricing).

See https://example.com/docs/setup for setup.

[Read more](#)
```

## Good

```markdown
See our [pricing details](/pricing).

Follow the [setup guide](https://example.com/docs/setup).

[Read the full case study](/case-studies/acme).
```

## When This Applies

- Every content link on an indexable page.
- Especially navigational and call-to-action links, where the anchor text carries intent.

## When This Does Not Apply

- A URL shown on purpose as text (a citation, a copy-paste command, documenting the address).
- Design-constrained UI button labels (content rules govern content, not the app chrome).

## Further Reading

- IN-CO-STYLE-002 -- alt text, the sibling accessibility rule.
