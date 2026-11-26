---
verblock: "26 Nov 2026:v4.35: vc - 3.5.5 landed; folded for cycle 5, the gate and cut to go"
intent_version: 3.5.4
---

# Work In Progress

## DOING

- Intent 3.5.5 (hv decisions 114, 115 and 116): every unit is on main, and nothing is pushed. It holds the /in-autopsy rewrite (STee6564b), doctor naming its binary, the warning from `wp done` and `st done` on an unmerged branch, and the guide text for a test that passes on the old code. The plan and the Desk are devbin-vc's (hv decision 103), from `~/Devel/scratch/estate-plan/PLAN.md`.
- Next, after hv's cycle-5 compact: the reference set at the tip after the seats' folds, committed as the final tip; dc's one gate all and test shell there, with main held; then, with hv at the Mac and unlocked, `intent daemon stop`, `bin/devbin preflight daemon` and `bin/devbin release v3.5.5 --dry-run` on a Desk booking. At a green dry run, hv types `bin/devbin release v3.5.5` and its `/dev/tty` confirm.

## TODO

- Intent 3.5.6: Intent.app adopts arca_swift's ArcaLaunch (hv decision 114).
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 with the three parity-tool fixes hv parked there (ST0084; hv decisions 104, 105, 106 and 110), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Both take Intent 3.5.4 and nothing else (hv decision 114). Their text is `git show f16dce4a7:intent/wip.md`.
