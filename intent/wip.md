---
verblock: "25 Nov 2026:v4.24: vc - 3.5.4 pre-approved on a green dry run (hv decision 101)"
intent_version: 3.5.3
---

# Work In Progress

## DOING

- Intent 3.5.4 is in flight (hv decisions 99 and 100). It carries the Highlander residue thread (ST1c2db93) and the Linux `intent app` fix (issue I3c24d2a). cc, ic and dc build the thread's packages, each in its own worktree, and vc lands each bank on its builder's proof. Each package's state is in `intent wp list ST1c2db93` and on its seat's board.
- Now: dc builds the thread's last bank: d0fd's fixture literals and record paths, the hand-written counts in tests, `id::head_number_at` for efe0's critic finding R2 (cc's meaning, ic's agreement), and the critics' fixes.
- Next: vc's close sweep (AC-00.1, a read). Then the pre-cut documents (the reference set, known-defects, the explorer's shots and the CHANGELOG), because `gate all` refuses a reference set that does not describe HEAD. Then one `bin/devbin gate all` and one `bin/devbin test shell` at main's HEAD on Desk slots, which decide the whole batch, and the dry run. hv pre-approved the cut and the tap on a green dry run (hv decision 101). hv types the cut's confirm, which reads `/dev/tty`. I3c24d2a stays open until the release ships it.

## TODO

- Parked for hv, Not Started: vault for local credentials (ST0060), level 3 toolchain-only references (ST0077), Windows through WSL 2 (ST0082), parity tooling out of ST0056 (ST0084), passive prompt injection (ST0085), and the fold and compact policy (ST0089, hv decision 59).
- Parked for hv since 2026-11-10: Laksa's and Conflab's releases, and the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
