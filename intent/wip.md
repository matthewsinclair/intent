---
verblock: "2026-11-19:v4.03: vc - ST0095 and ST0096 in flight, folded before hv's 13:00Z compact"
intent_version: 3.4.0
---

# Work In Progress

## DOING

- **ST0095, git-style ids for threads, work packages and issues (P0, hv decisions 61 to 71).** Under way since 2026-11-19. Each package's seat and order are in `intent/st/ST0095/design.md`, `intent wp list ST0095` says where each stands, and vc's board (`intent wb show vc`) carries the routing. cc builds WP-05 and WP-06 as one unit, then WP-07; ic builds WP-10, WP-11 and WP-13; dc builds WP-08, WP-09, WP-12 and WP-14 and finishes WP-03's screenshots. Until the release is installed from brew, no dev build runs against the main checkout's store (vc decision 132).
- **ST0096, a forge merges every open merge request with no hand step (issue 0705, hv decisions 66, 70 and 71).** Under way since 2026-11-19. `intent/st/ST0096/design.md` carries cc's canon split, one canon file per record. Open: ic's shape call on the thread views, which comes to hv if a reader on the forge would lose a thread's criteria and tests.
- **Every fix found in this work rides this release (hv decision 69).** `intent issues list` gives which are still open, and each issue names its owner.

## TODO

- The release that carries ST0095 and ST0096: its number is hv's at the cut, and the tap's formula holds it until hv's word (hv decision 68). When a build of it exists, vc tells agentic-engineering-vc how its seats install it without the tap.
- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16, and issue 0703 made the shell suite independent of it.
- ST0089 stays parked (hv decision 59): its policy needs hv's call first, since "fold at a task boundary, not at a token count" conflicts with hv's compact-at-200-250k rule.
- The rest of the estate visit (hv decision 41): Intent's is done; the other estates follow, with the fleet's root files upgraded after this cut. Trellis moves anything bespoke out of its AGENTS.md first.
- ST0094, the estate's last two boards.
- Open with hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own. devbin-cc holds 0326 now.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
