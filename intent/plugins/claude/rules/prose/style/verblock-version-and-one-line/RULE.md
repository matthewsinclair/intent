---
id: IN-PR-STYLE-005
language: prose
category: style
severity: warning
title: A verblock is the version and one short line, never history
summary: >
  A verblock records which version a document is at and says, in one short
  line, what that version is. Its history belongs to version control: an
  earlier version kept beside the current one, or a narrative of how the
  document got here, turns the frontmatter into a second log that drifts from
  the real one.
principles:
  - house-style
applies_when:
  - "Any hand-authored Markdown document that carries verblock frontmatter: a work-in-progress file, restart notes, design and as-built notes, docs"
  - "Every edit that moves a document's version, an agent's end-of-session fold included"
applies_to:
  - "**/*.md"
does_not_apply_when:
  - "Records kept verbatim, such as archived copies of earlier documents: the rule is going-forward only, so a record keeps the verblock it was written with"
  - "Generated views, which carry no verblock"
tags:
  - prose
  - house-style
  - mechanical
  - frontmatter
references: []
related_rules:
  - IN-PR-STYLE-001
aliases: []
status: active
version: 1
---

# A verblock is the version and one short line, never history

A verblock carries the document's date, its version and one short line about that version, and nothing else. What came before is in version control.

## Problem

Intent's maintainer, hv, stated the rule on 2026-10-12:

> The verblock just has 1-line (or less) text plus the version and NEVER has history in it. That is what version control is for!

And again on 2026-10-26:

> THE VERBLOCK IS NOT A REPLACEMENT FOR VERSION CONTROL. DO NOT STORE PROSE IN THE VERBLOCK. Just the docs version and a short -- no more than 1-line -- text about the current version.

A verblock that grows is a changelog in the wrong place. Every earlier version or narrative appended to it gives the document's history a second home beside the commit log, which already holds that history exactly. The two drift, and only the commit log is ever checked. The growth is mechanical rather than deliberate: an agent ending a session rewrites a hand-authored document, and the verblock is the one line that changes on every edit, so the session's account lands there and stays.

## Detection

Read the `verblock:` key at the top level of the document's leading frontmatter, with any indented or listed continuation lines joined to it:

- **A second version stamp.** A stamp is a version token -- `v`, digits and at least one dotted group -- that a colon precedes or follows: `2026-10-25:v3.24:`, `19 Sep 2026:v0.4:`, `2026-10-25: v1.2 -`. Two stamps in one verblock, or across two `verblock:` keys, is an earlier version kept beside the current one. A date alone is not a stamp: a line about this version may name one.
- **A value longer than 100 characters**, counted in characters rather than bytes, with one pair of surrounding quotes removed.

The shipped pre-commit guard `verblock-guard.sh` applies both checks to the Markdown a commit stages and refuses the commit on either. A critic reading a document applies the same two.

## Bad

```markdown
---
verblock: "2026-10-25:v3.24: vc - globalfold at hv's EOD: the desk is closed, every in-flight piece is banked and verified, and tomorrow starts by re-confirming each bank's bytes at molt-vc's desk"
---
```

```markdown
---
verblock: "19 Sep 2026:v0.4: citations re-synced to the tree at 6d761cf69 and the declared languages completed. 15 Sep 2026:v0.3: citations and the install-root pointer re-synced to the tree at a0c7300eb"
---
```

The first is one version carrying a session's account. The second keeps the previous version beside the current one.

## Good

```markdown
---
verblock: "2026-10-25:v3.24: vc - globalfold at hv's EOD"
---
```

```markdown
---
verblock: "19 Sep 2026:v0.4: citations re-synced to the tree at 6d761cf69"
---
```

A line that will not fit is elided, never kept whole. hv, 2026-10-26: "Just elide them at 97 chars if you need to." Ninety-seven characters and `...` make the hundred:

```markdown
---
verblock: "2026-10-25:v3.24: vc - globalfold at hv's EOD: the desk is closed, every in-flight piece is banke..."
---
```

## When This Applies

- Every hand-authored Markdown document that carries verblock frontmatter: a work-in-progress file, restart notes, design and as-built notes, docs.
- Every edit that moves the version. The line is rewritten to say what the new version is, and the old line is replaced rather than kept.

## When This Does Not Apply

- Records kept verbatim, such as archived copies of earlier documents. The rule is going-forward only: a record keeps the verblock it was written with, and the guard reads only what a commit stages.
- Generated views, which carry no verblock.

## Further Reading

- `verblock-guard.sh`, among the hooks Intent ships, which refuses a commit that stages a verblock breaking either Detection check.
- IN-PR-STYLE-001 -- the house-style rule this one sits beside.
