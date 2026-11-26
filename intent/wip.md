---
verblock: "26 Nov 2026:v4.30: vc - 3.5.4 waits on devbin 0.1.34, then the dry run and hv's cut"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99, 100, 101 and 104 to 110). It carries the Highlander residue thread (ST1c2db93), the Linux `intent app` fix (issue I3c24d2a) and the pre-commit gate's count of an on-demand discipline (issue Ic5f63aa). The team works from the estate plan, `~/Devel/scratch/estate-plan/PLAN.md`, which hv approved as written. devbin-vc holds the pen and the Desk (hv decision 103).
- Now: 3.5.4 is built and proved locally and not cut. The batch is green at 5d58e6b6e (gate all and test shell), ST1c2db93 is done (cfed458e9), and nothing is pushed: local and upstream hold bd1aa3c6c. hv decision 110: 3.5.4 vendors devbin 0.1.34 before its cut. devbin-vc cuts 0.1.34 and sweeps it into Intent's main on vc's word; gate all and test shell are not run again. Every Intent run holds until then.
- Next, after the 0.1.34 sweep and with hv at the Mac, unlocked (vc watchout 38): `intent daemon stop`, `bin/devbin preflight daemon`, `bin/devbin release v3.5.4 --dry-run` on a Desk slot. At a green dry run, hv types `bin/devbin release v3.5.4` and its `/dev/tty` confirm. The cut pushes main and the tag to local and upstream. After the ship: `intent daemon start`, and I3c24d2a and Ic5f63aa close.

## TODO

- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 with the three parity-tool fixes hv parked there (ST0084; hv decisions 104, 105, 106 and 110), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
