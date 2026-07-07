---
id: IN-AU-CRAFT-004
language: author
category: craft
severity: recommendation
title: Citation and attribution discipline
summary: >
  Quotations, data, named studies, and borrowed ideas carry attribution;
  nothing sourced is presented as original, and no fact is asserted without a
  traceable source. LLM-drafted prose is especially prone to confident but
  fabricated or unsourced citations.
principles:
  - honest-prose
  - attribution-integrity
  - reader-trust
applies_when:
  - "Non-fiction, courseware, or technical writing that states facts or quotes sources"
  - "Any LLM-assisted draft, where hallucinated citations are a known failure mode"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
does_not_apply_when:
  - "Fiction and opinion or editorial writing, where claims are not asserted as sourced fact"
  - "Common knowledge that conventionally needs no citation"
tags:
  - author
  - citation
  - attribution
  - craft
  - honesty
references: []
related_rules:
  - IN-AU-CRAFT-003
aliases: []
status: active
version: 1
---

# Citation and attribution discipline

Every quotation, statistic, named source, and borrowed idea is attributed, and every asserted fact is traceable to a real source.

## Problem

Non-fiction earns its authority by being checkable. When a quotation has no source, when a statistic or named study is cited with no reference, when a borrowed idea is presented as the author's own, or -- the distinctively modern failure -- when an LLM-drafted passage asserts a confident, plausible-sounding citation that simply does not exist, the reader has no way to verify the claim and every reason to doubt the rest. A single fabricated citation, once a reader catches it, poisons trust in the whole work. LLM assistance makes this worse, not better: models produce fluent, authoritative-looking author-title-year citations that are entirely invented, and they read as more credible than a hedged honest claim.

## Detection

This is a critic-as-reader judgement backed by verification; grep can find a bare URL but cannot tell a real citation from a fabricated one. Read for:

- Quotations with no attributed speaker or source.
- Statistics, dates, or named studies asserted with no citation.
- Confident factual claims that read like an LLM hallucination -- a plausible author, title, and year that cannot be verified against any real source.
- Ideas or framings borrowed from a named source but presented as original.

Where a citation is given, spot-check that it resolves to a real, correctly-attributed source. The violation is not "no link present" -- it is a claim the reader cannot trace, or a citation that does not survive verification.

## Bad

```markdown
Studies show that 73% of developers prefer this approach (Henderson, 2019), and as Dijkstra famously said, "recursion is the root of all clarity."
```

## Good

```markdown
In a 2019 survey of roughly 90,000 developers, a majority reported preferring this approach [1]. (The line about recursion is often attributed to Dijkstra but is hard to source, so it is left out rather than misattributed.)
```

## When This Applies

- Non-fiction, courseware, and technical writing that makes factual claims or quotes sources.
- Any LLM-assisted draft -- fabricated citations are a known and frequent failure mode.

## When This Does Not Apply

- Fiction, opinion, and editorial writing, where claims are not offered as sourced fact.
- Common knowledge that conventionally needs no citation ("water boils at 100 degrees C at sea level").

## Further Reading

- IN-AU-CRAFT-003 -- the full trope diagnosis; fabricated citations often travel with other LLM tells.
- IN-PR-STYLE-002 -- no vanity metrics, a related discipline about not dressing prose in unearned numbers.
