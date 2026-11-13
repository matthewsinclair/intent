---
verblock: "2026-11-13:v3.86: vc - Intent 3.3.1 released; follow-ups and the devvm"
intent_version: 3.3.1
---

# Work In Progress

## DOING

- After Intent 3.3.1 (released 2026-11-13, tag v3.3.1 at c64a42e29, cut by devbin-vc under hv decisions 49, 54 and 55): agentic-engineering-vc builds the devvm from both taps, on devbin-vc's word that Intent's version is out and the desk is free. vc's todo 79 carries the follow-ups: ic brings intent.laksa.io to 3.3.1, its push hv's word (ic todo 70); dc banks the stale config.yaml comment on CI's toolchain and the actions/checkout@v4 bump; releasing.md's tag check reads `vX.Y.Z^{commit}`; main's CI at the tag and the first run under the fleet-Rust pin (4e6fb844a) are read.
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on 3.3.1's sweep.

## TODO

- For hv: whether the devvm takes Molt from a pushed repo, so that devbin-vc's 0.1.23 sweep commit f06f164 must be pushed (vc todo 80).
- After Intent's cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- The plan-usage card blanks on a 429 after an intentd restart (ic files it, on hv's word, for after 3.3.1).
- ST0094, the estate's last two boards (moved out of ST0092), is not part of this release (hv decision 42).
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- The memory prune.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
