---
verblock: "25 Nov 2026:v4.26: vc - localfold for the estate-wide compact"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99, 100, 101, 104 and 105). It carries the Highlander residue thread (ST1c2db93), the Linux `intent app` fix (issue I3c24d2a) and the pre-commit gate's count of an on-demand discipline (issue Ic5f63aa). The team works from the estate plan, `~/Devel/scratch/estate-plan/PLAN.md`, which hv approved as written. devbin-vc holds the pen and the Desk (hv decision 103).
- Now: folded on 2026-11-25 for an estate-wide compact, on hv's word through devbin-vc. The close sweep's findings go into this release as fix banks. cc's (intentsvcs, intentd and `schema/`) and dc's (the tests and the gate tools) are open, each saved at its seat's bank ref. What is open, the rows for design.md and the steps after the banks are in `close-state.md` at `refs/bank/vc/st1c2db93/close-notes`.
- Next: vc lands cc's and dc's fix banks on their proof. Then releasing.md step 2 at the final HEAD: ic runs build all and takes the shots, cc re-drives the known-defects page, and vc regenerates the reference set and checks CHANGELOG, in one commit. Then one `bin/devbin gate all` and one `bin/devbin test shell` at main's HEAD on Desk slots, the thread's close with design.md, the push on devbin-vc's word after githubstatus, and the dry run. hv types the cut's confirm, which reads `/dev/tty` (hv decision 101). I3c24d2a and Ic5f63aa stay open until the release ships them.

## TODO

- Put to hv: the install pointer `~/.local/share/intent/home` names Intent's dev checkout, so every estate's pre-commit gate on rhadamanth runs the hook from Intent's main while the `intent` binary is brew 3.5.3 (hv decision 80). The finding and its fixes are in `close-state.md` at `refs/bank/vc/st1c2db93/close-notes`.
- After the 3.5.4 cut: devbin-vc sweeps devbin 0.1.33 into Intent (vc's FREEZE of 2026-11-25 keeps Intent on 0.1.32 until then).
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 (ST0084), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
