---
verblock: "24 Nov 2026:v4.13: vc - 3.5.2 released at fabbffd8c"
intent_version: 3.5.2
---

# Work In Progress

## DOING

- Nothing in flight. Intent 3.5.2 is released (tag v3.5.2 at fabbffd8c, 2026-11-24).

## TODO

- After 3.5.2: cut the two global rules (sizes only, no hand wrap) out of lib/templates/llm/_CLAUDE.md, now that ~/.config/claude/house-rules.md holds them. Open issues outside 3.5.2: Ia87820b (a check for a restated global rule), I5adc617 (an HTTP 5 s test red under load), I53c0c22 (Webber backup gaps), and cc's mcp_session lossy-parse finding.
- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
