---
verblock: "25 Nov 2026:v4.25: vc - closed out for the reboot of rhadamanth; DOING names the resume steps"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99 and 100). It carries the Highlander residue thread (ST1c2db93), the Linux `intent app` fix (issue I3c24d2a) and the pre-commit gate's count of an on-demand discipline (issue Ic5f63aa). cc, ic and dc build the thread's packages, each in its own worktree, and vc lands each bank on its builder's proof. Each package's state is in `intent wp list ST1c2db93` and on its seat's board.
- Now: closed out on 2026-11-25 for the reboot of rhadamanth (hv's word through devbin-vc). dc's second bank is saved and not banked: `tmp/wt-dc-st1c2` at 984ee6021 (`refs/bank/dc/st1c2db93/wip-v2b-on-010c0396d`, base 010c0396d). It carries d0fd's fixture literals and record paths, the hand-written counts in tests, `id::head_number_at` for efe0's critic finding R2 with cc's zero rule, the critics' fixes and Ic5f63aa. After the reboot, each Desk ask is made again: dc runs critic-shell on Ic5f63aa again, squashes and rebases onto main, runs one clippy on the Desk, banks v2 and reports to vc.
- Next: vc lands dc's bank on its proof, after ic reads `head_number_at` again (cc's zero rule changed it after ic's read). Then vc's close sweep (AC-00.1, a read). Its brief, the pending design.md rows and the landing checks are at `refs/bank/vc/st1c2db93/close-notes`. Then the pre-cut documents (the reference set, known-defects, the explorer's shots and the CHANGELOG), because `gate all` refuses a reference set that does not describe HEAD. Then one `bin/devbin gate all` and one `bin/devbin test shell` at main's HEAD on Desk slots, which decide the whole batch, and the dry run. hv pre-approved the cut and the tap on a green dry run (hv decision 101). hv types the cut's confirm, which reads `/dev/tty`. I3c24d2a and Ic5f63aa stay open until the release ships them.

## TODO

- After the 3.5.4 cut: devbin-vc sweeps devbin 0.1.33 into Intent (vc's FREEZE of 2026-11-25 keeps Intent on 0.1.32 until then).
- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 (ST0084), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
