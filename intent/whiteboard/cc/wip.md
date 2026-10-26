---
node: cc
name: Control Claude
role: control
session_id: 1b8817f9-d0d6-4b3e-b2f5-d09c0a9a0577
heartbeat_at: 2026-10-26 13:55Z
status: active
focus: "HOLD on hv's word, relayed by vc at 13:43Z by vc's clock, and molt-vc's machine-wide round (slots desk hold 01M5W7E40CE071QMSE4JT8KWXQ): no task, no slot ask, no run until hv says go on. On hv's go-on, ST0086 WP-04 Part B's judge is the first run molt-vc books through the slot records, on vc's word, over the stack wp04b-status-v2 4ae2585f7479, wp04b-wait-v2 2421b3d9719d, ic's wp04b-page-v2 8126a568631b, wp04b-init-v1 e8aee9ac86d6; a red comes back to cc in tmp/wt-cc-0086b, and a fix-forward bank carries no CHANGELOG hunk; owed after the landing: AT rows for WP-04's uncovered criteria (AC-04.1 to 04.6, 04.8 to 04.13), design.md's key paragraph, pending's measured cost; hold 33 stands until ST0087 has gone first; no push, no release"
claims: [ST0081, ST0086/04]
---

# Control Claude (cc)

## DOING

- LOCALFOLD resume state (2026-10-26, on hv's word relayed by vc at 13:17Z by vc's clock). ST0086 WP-04 Part B, begun on vc's word at 10:07Z by vc's clock, is banked as a stack in vc's train order: refs/bank/cc/0086/wp04b-status-v2 (4ae2585f7479), AC-04.6's CLI share; refs/bank/cc/0086/wp04b-wait-v2 (2421b3d9719d), AC-04.4; ic's refs/bank/ic/0086/wp04b-page-v2 (8126a568631b), the page's half; and refs/bank/cc/0086/wp04b-init-v1 (e8aee9ac86d6), AC-04.5. wp04b-status-v1 and wp04b-wait-v1 are superseded. vc verified at 11:19Z by vc's clock that the stack applies on main 34c7046ad to tip tree b8c3025dd884. Main has since taken 0654, 0656 and A4 (0655), and the pair was built at 2d4c941fa05d. After each, cc measured outside the repo that the stack still applies: CHANGELOG.md is the only path Part B shares with them, B1's, B2's and B3's hunks apply after their lines, and no other Part B path has moved. The CHANGELOG hunks stand as written on vc's ruling, though decision 92 asked new banks for none, and any wording fix goes in vc's own CHANGELOG commit at landing. A fix-forward bank from here carries no CHANGELOG hunk, and its line goes to vc as text. Part B's judge runs after devbin-vc's push, as the first run molt-vc books through the slot records; hv ruled at 13:16Z, relayed by vc, that molt-vc takes control after the kovacs preflight's END. A red comes back to cc in tmp/wt-cc-0086b, detached over 629ce5ad2 with hooks-off scratch tips e52f97bb6 (B1 v2), 88ccdd4d6 (B2 v2) and 99a9ced05 (B3), and a bank that moves is told to ic, whose page bank sits on B1 v2 and B2 v2. The worktree stays live for Part B on vc's ruling after 0654: no node removes any worktree until molt-vc's Spotlight word, and if Part B lands after it, cc removes tmp/wt-cc-0086b at the chain END that lands it or at the next fold. Owed after the landing, on vc's word: an AT row for every WP-04 criterion with no covered-by, which `intent ac list ST0086` showed at this fold as AC-04.1 to AC-04.6 and AC-04.8 to AC-04.13 (AC-04.7 has AT-04.7), each citing one marked file; design.md's key paragraph (lanes[].queue and controller_unreadable) through the attachment door; and the measured cost of one `pending` call with the judge's binary. Light work only under decision 92. No push, no release. (edited)

## TODO

_(none)_

## Holds

- HOLD ST0081 AC-02.1 and WP-02 (Linux assets on each tag's release, one writer per release object) UNTIL the dev, opt, brew conformance thread has gone first, per hv's order (vc to specify it, citing devbin's docs/dev-opt-brew.md); AC-02.1 is proven at the first cut after that. MET, 2026-10-25: devbin v0.1.14 is re-vendored into Intent at 141dcf9ff (refs/bank/cc/devbin/v0.1.14), its macOS bats run at e255f734a green but for issue 0640's pre-existing worktree reds; the 3.2.6 tag condition (v3.2.6 at 88633fe93). D55 is ruled (hv, 2026-10-23: devbin's release core attaches the Linux assets, SHA256SUMS riding it), and devbin-vc's shape is the build: Intent declares `commands.release.enabled: true` with `release.verbs: [check, attach]`, `release.repo` and the assets keys; linux-release.yml triggers on tags matching `v[0-9]*`; the cut runs build release, macos prepare, macos publish, `bin/devbin release attach vX.Y.Z`, then brew formula --push. Every other ST0081 row is settled: AC-01.1, AC-01.2, AC-03.3 and AC-06.1 satisfied, WP-01, WP-03 and WP-06 done, and the gate reads BLOCKED on AC-02.1 alone. The release.remotes block is retired (vc), not owed. (edited)

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
