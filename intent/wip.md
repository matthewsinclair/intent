---
verblock: "23 Nov 2026:v4.10: vc - localfold on hv's call; 3.5.2 in flight"
intent_version: 3.5.1
---

# Work In Progress

## DOING

- **Intent 3.5.2 (hv decisions 87 and 88, 2026-11-23: fix everything, release asap).** Landed on main: the four estate-desk fixes; I81ecc7f, Ifdd3c89, Ie7fb84c, I84286b7, Iae1e02a, Ic884775 and Ie99dbb6; devbin's release target and release.unread; ST0094 done. Open: dc's todos 87 and 88 and ST0097 units A and B; cc's ST1c2db93 units U1 to U4 (U4 removes v2, P22 and P30 go to the next release); issue I4da7c22 (backup registry), unassigned. Then releasing.md step 2, the kovacs rehearsal and the whole local gate, and vc's READY to hv, who fires the cut. AE's attendees take 3.5.2 from the tap, so a repo upgraded at 3.5.1 must get 0 doctor findings with no action, and no hook may change. vc's board doing 67 has the commits.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
