---
verblock: "24 Nov 2026:v4.18: vc - 3.5.3 released; b353 issue closes and ci353 v2 follow"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- **After Intent 3.5.3 (tag v3.5.3 at 8475de825).** cc closes the b353 issues. cc runs the proof of ci353 v2 (refs/bank/cc/ci353/v2 = 63751ae7b, test only), then vc lands it on main by its one path, tests/unit/release_candidate_trigger.bats.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
