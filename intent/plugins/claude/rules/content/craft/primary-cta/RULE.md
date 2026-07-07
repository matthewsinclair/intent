---
id: IN-CO-CRAFT-002
language: content
category: craft
severity: recommendation
title: Each page has one clear primary call to action
summary: >
  A content page whose job is to move the reader somewhere has one obvious next
  step, phrased as a specific action. Not zero (a dead end), not five competing
  ones (choice paralysis) -- one primary CTA, with any others clearly secondary.
principles:
  - reader-first
  - purpose-driven
applies_when:
  - "A web page with a conversion goal (sign up, buy, contact, subscribe, download)"
  - "Landing pages, product pages, and campaign pages"
does_not_apply_when:
  - "Purely informational pages with no next step (a reference article, an about page)"
  - "Index / hub pages whose job is navigation, where many equal links are correct"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
tags:
  - content
  - web
  - cta
  - conversion
references: []
related_rules: []
aliases: []
status: active
version: 1
---

# Each page has one clear primary call to action

A page with a goal names the single next step and makes it the obvious one.

## Problem

A page built to convert but ending in nothing wastes the attention it earned -- the reader finishes, nods, and leaves with no next step. The opposite failure is as costly: five equally-weighted buttons (Sign up, Book a demo, Read the docs, Contact sales, Download) force a decision the reader did not come to make, and a forced decision is often "none". Whether a page has the right call to action, phrased as a concrete action rather than a vague "Submit" or "Learn more", is a judgement about the page's purpose, not something grep can settle.

## Detection

A judgement read against the page's goal. The critic asks:

- Does the page have a next step at all, or does it dead-end?
- Is there one _primary_ action, visually and textually dominant, with others clearly secondary?
- Is the CTA phrased as a specific action ("Start your free trial") rather than a generic verb ("Submit", "Click", "Learn more")?
- Does the CTA match what the page has argued for, or ask for a bigger commitment than the reader is ready for?

## Bad

```markdown
## Ready?

[Submit](/x) [Learn more](/y) [Contact](/z) [Docs](/d) [Pricing](/p)
```

## Good

```markdown
## Start shipping faster today

[Start your free 14-day trial](/signup)

Prefer to talk first? [Book a 20-minute demo](/demo).
```

## When This Applies

- Any page with a conversion goal: landing, product, campaign, pricing.
- Pages at the end of a funnel, where the next step is the whole point.

## When This Does Not Apply

- Informational pages with no intended next step.
- Navigation hubs, where many equal-weight links are the correct design.

## Further Reading

- IN-CO-STYLE-003 -- descriptive link text; a CTA's words are its most important anchor text.
