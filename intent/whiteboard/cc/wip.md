---
node: cc
name: Control Claude
role: control
session_id: 1b8817f9-d0d6-4b3e-b2f5-d09c0a9a0577
heartbeat_at: 2026-10-26 19:58Z
status: paused
focus: "CLOSED FOR THE NIGHT on hv's end-of-day word (relayed by molt-vc 2026-10-26), holding until hv says go on: 0648's exit-0 revert landed at ddb749ca7 and upstream has it; ST0086 WP-04 Part B's six-bank train (doing 96) waits for Intent's slot behind Utilz's end-point; issue 0657 (todo 42) is banked after it; hold 33 stands until ST0087, now WIP, has gone first; tmp/wt-cc-0648r and tmp/wt-cc-0086b stay until molt-vc's Spotlight word; no push, no release"
claims: [ST0081, ST0086/04]
---

# Control Claude (cc)

## DOING

- ST0086 WP-04 Part B, a six-bank train in vc's order: refs/bank/cc/0086/wp04b-status-v2 (4ae2585f7479), AC-04.6's CLI share; refs/bank/cc/0086/wp04b-wait-v2 (2421b3d9719d), AC-04.4, with slots wait, slots watch and the hook's slots pending; ic's refs/bank/ic/0086/wp04b-page-v2 (8126a568631b), the page's half; refs/bank/cc/0086/wp04b-init-v1 (e8aee9ac86d6), AC-04.5; refs/bank/cc/0086/wp04b-ats-v1 (f649be3fa3ea), on vc's ruling of 2026-10-26: the AT markers on Part A's tests, a test for AC-04.2's lane re-open and LaneBusy close, and the NamedMismatch text asserted; and ic's pipeline-cell-width-v4 (e058514c0734), hv's ask on ic, CSS only. On main 00eb61a2e the tree after five is 2ca7692e71da and after six c45a889677a8 (ic's and vc's measurement, 17:17Z by their clocks); fmt, gen and critic were green over the five. The heavy judge was granted at 15:01:09Z and revoked at 15:01:34Z by vc's clock for load (hv: one project at a time), and runs when molt-vc grants Intent's slot, behind Devbin and Utilz, straight through with build all and the kovacs preflight. A red in cc's banks comes back to cc in tmp/wt-cc-0086b, detached on scratch tips dfb14c98c (four banks) and ddc43caba (with ats); a fix-forward bank carries no CHANGELOG hunk, its line goes to vc as text, and a bank that moves is told to ic. Owed after a green landing, in one chain: AT rows for AC-04.1 to AC-04.6 and AC-04.8 to AC-04.13 (slots.rs for all but 04.5, coordinator.rs for 04.5), with `intent at lint ST0086` before the commit; design.md through `intent st attach` with controller_unreadable, lanes[].queue and line 67's UNREADABLE, plus pending's cost measured with the judge's binary under `--lane none`, as dated figures, never as a bound; ic's todo 58 owes the pipeline cells' sentence in the same file, so the two attaches go in sequence. No node removes a worktree until molt-vc's Spotlight word; then cc removes tmp/wt-cc-0086b at the landing's END or the next fold. Light work only under decision 92. No push, no release. (edited)

## TODO

- ISSUE 0657 (filed by vc at 6d852eb04 on hv's "A: file the fix now"): install's HEAD check refuses only when commits past the built sha touch DIRT_SCOPE or a support path. Size S, cc's to bank when Intent's queue reaches it in the next train, after Part B's train (doing 96). Until it lands, a commit to main between a build and its install makes the fresh pair refuse, so main holds through every build all. Light work only under decision 92.

## Holds

- HOLD ST0081 AC-02.1 and WP-02 (Linux assets on each tag's release, one writer per release object) UNTIL ST0087 (Intent conforms to the dev, opt and brew contract, citing devbin's docs/dev-opt-brew.md) has gone first, per hv's order, with vc to specify it; ST0087 was started at 6d852eb04 on hv's yes (vc's decision 96) and reads WIP, so the condition, that it has gone first, is still unmet. AC-02.1 is proven at the first cut after that. D55 is ruled (hv, 2026-10-23: devbin's release core attaches the Linux assets, SHA256SUMS riding it), and devbin-vc's shape is WP-02's plan, not yet built: Intent declares `commands.release.enabled: true` with `release.verbs: [check, attach]`, `release.repo` and the assets keys; linux-release.yml, which today is workflow_dispatch only, gains a trigger on tags matching `v[0-9]*`; the cut runs build release, macos prepare, macos publish, `bin/devbin release attach vX.Y.Z`, then brew formula --push. Every other ST0081 row is settled and the gate reads BLOCKED on AC-02.1 alone. The release.remotes block is retired (vc), not owed. (edited)

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc's bank refs stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on how many there are. A spent bank ref costs nothing where it sits. (edited)

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
