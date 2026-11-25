---
verblock: "25 Nov 2026:v4.28: vc - 3.5.4 ready for hv at the Mac"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99, 100, 101 and 104 to 108). It carries the Highlander residue thread (ST1c2db93), the Linux `intent app` fix (issue I3c24d2a) and the pre-commit gate's count of an on-demand discipline (issue Ic5f63aa). The team works from the estate plan, `~/Devel/scratch/estate-plan/PLAN.md`, which hv approved as written. devbin-vc holds the pen and the Desk (hv decision 103).
- Now: everything before the dry run is done. The batch is green at 5d58e6b6e (gate all and test shell), ST1c2db93 is done (cfed458e9), and `bin/devbin preflight` clean, faces, spill and schema pass. The dry run and the cut wait for hv at the Mac, unlocked: the cut's pre-flight runs `macos doctor`, and the notary profile reads only in an unlocked session (vc watchout 38). The cut waits for hv's keystroke (hv decision 108).
- Next, with hv at the Mac: vc runs `intent daemon stop`, `bin/devbin preflight daemon` and `bin/devbin release v3.5.4 --dry-run` on a Desk slot. At a green dry run, hv types `bin/devbin release v3.5.4` and its `/dev/tty` confirm. The cut pushes main and the tag to local and upstream itself. After the ship: `intent daemon start`, and I3c24d2a and Ic5f63aa close.

## TODO

- After the 3.5.4 cut: devbin-vc sweeps devbin 0.1.33 into Intent (vc's FREEZE of 2026-11-25 keeps Intent on 0.1.32 until then).
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 with the three parity-tool fixes hv parked there (ST0084; hv decisions 104, 105 and 106), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
