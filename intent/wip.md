---
verblock: "26 Nov 2026:v4.31: vc - 3.5.4 attendee fixes landed; A3(b), the 0.1.34 sweep and the batch to go"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99, 100, 101 and 104 to 113). It carries the Highlander residue thread (ST1c2db93), the Linux `intent app` fix (issue I3c24d2a) and the pre-commit gate's count of an on-demand discipline (issue Ic5f63aa). The team works from the estate plan, `~/Devel/scratch/estate-plan/PLAN.md`, which hv approved as written. devbin-vc holds the pen and the Desk (hv decision 103).
- Now: 3.5.4 is not cut, and nothing is pushed (local and upstream hold bd1aa3c6c). On 2026-11-26 it took the attendee fixes (hv decisions 109 to 113): the autopsy skill-text fix, the WIP: line fallback, /in-finish, the guide texts and arca_swift 0.3.0 are on main (4ff2c0a13 to bed2fdc44). The gate following a moved test line (A3(b), hv decision 112) is cc's bank refs/bank/cc/i354c/v1, to land. The batch at 5d58e6b6e no longer stands for the tip.
- Next: land A3(b); on devbin-vc's word, the devbin 0.1.34 sweep; the reference set at the final tip; dc's one gate all and test shell there. Then, with hv at the Mac, unlocked (vc watchout 38): `intent daemon stop`, `bin/devbin preflight daemon`, `bin/devbin release v3.5.4 --dry-run` on a Desk slot. At a green dry run, hv types `bin/devbin release v3.5.4` and its `/dev/tty` confirm. The cut pushes main and the tag to local and upstream. After the ship: `intent daemon start`, and I3c24d2a and Ic5f63aa close.

## TODO

- Parked for hv as design questions from the 2026-11-26 attendee findings: /in-autopsy finds too little (recall); how an AT whose test passes on the old code reaches green; refusing `wp done` and `st done` on an unmerged branch; doctor naming the binary whose templates it compared; adopting arca_swift's ArcaLaunch in Intent.app.
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 with the three parity-tool fixes hv parked there (ST0084; hv decisions 104, 105, 106 and 110), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
