---
verblock: "2026-11-13:v3.88: vc - 3.3.1 P0 closed; memory pruned"
intent_version: 3.3.1
---

# Work In Progress

## DOING

- Intent 3.3.1 is released and the estate is on it: tag v3.3.1 at c64a42e29; CI follows the fleet's Rust 1.99.1 and checkout@v7, green on main. The post-release fixes ride into the next release.
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on 3.3.1's sweep.

## TODO

- For hv: whether the devvm takes Molt from a pushed repo, so that devbin-vc's 0.1.23 sweep commit f06f164 must be pushed (vc todo 80).
- After Intent's cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- The plan-usage card blanks on a 429 after an intentd restart (ic files it, on hv's word, for after 3.3.1).
- ST0094, the estate's last two boards (moved out of ST0092), is not part of this release (hv decision 42).
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
