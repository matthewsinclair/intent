---
verblock: "24 Nov 2026:v4.15: vc - the batch adds ST1c2db93 P22 and P30"
intent_version: 3.5.2
---

# Work In Progress

## DOING

- **After the compact (hv, 2026-11-24): fix EVERY open issue in one batch, then ONE release.** Intent 3.5.2 is released (tag v3.5.2 at fabbffd8c). The open issues are I5adc617 (an HTTP 5 s test red under load), I53c0c22 (Webber daily backup gaps), I6f2e257 (the MCP test helper parses stdout lossily) and Ia87820b (a check for a restated global rule, design first). Also ST1c2db93 P22 (the single older-render predicate, L) and P30 (the canon-ignore-guard hook fix), which hv decision 88 moved to the next release; cc holds the claim. Also cut the two global rules (sizes only, no hand wrap) out of lib/templates/llm/_CLAUDE.md. Read `intent outstanding` again first. No build all, push, dry run or READY until every one is fixed and proven in one gate all. hv decides any deferral, from the full list.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
