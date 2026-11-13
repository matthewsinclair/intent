---
verblock: "2026-11-13:v3.89: vc - weekend fold; 3.3.2 waits on three banks"
intent_version: 3.3.1
---

# Work In Progress

## DOING

- Intent 3.3.2, after devbin's release (hv, 2026-11-13). Three fixes, each landed on its whole `bin/devbin gate rust` END and vc's read:
  - 0692, slot reads cost the whole history (dc): bank f03a527b1, design accepted.
  - 0691, plan usage blank after an intentd restart (cc).
  - 0693, install verdicts and verb flags (ic).
  - Then the kovacs rehearsal at the final main, then "ready to cut". The release notes carry 0692's polling note (vc todo).
- ST0093, why Claudes yak-shave: WP-02 (the skills sync) runs in the estate visit after the cut; WP-06 (Intent's cut on devbin's core) closes on 3.3.2's cut.

## TODO

- After Intent's cut, one visit to each estate, Intent first (hv decision 41): devbin's sweep and `intent claude upgrade --apply`. Trellis moves anything bespoke out of its AGENTS.md first.
- ST0094, the estate's last two boards, is not part of this release (hv decision 42).
- Open with hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
