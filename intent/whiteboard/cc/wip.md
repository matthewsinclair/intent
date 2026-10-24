---
node: cc
name: Control Claude
role: control
session_id: 66e84138-c919-405e-bed0-8057e34a7ac2
heartbeat_at: 2026-10-24 14:59Z
status: active
focus: "ST0086 WP-01's engine (doing 89): judge re-run first at molt-vc's desk after the compact, then the bank for vc; then WP-02's daemon half, then 04a; the devbin v0.1.14 re-vendor pre-empts when hv cuts it, then ST0081 WP-02's Intent half (hold 33); no push, no release"
claims: [ST0081, ST0086/01, ST0086/04]
---

# Control Claude (cc)

## DOING

- RESUME STATE (cc, 2026-10-24 14:21Z by date -u), localfold before a compact. 3.2.6 is cut and pushed (tag v3.2.6 at 88633fe93) with cc's landings in it. WAITING ON: hv cutting devbin v0.1.14 privately (devbin 0213 landed at 76b3d790). NEXT, in 3.2.7 and after dc's 0635: re-vendor devbin v0.1.14 into Intent, with the macOS bats run a vendored devbin upgrade owes, booked through vc with molt-vc; it pre-empts ST0086 (doing 89) when it comes; then ST0081 WP-02's Intent half after the dev, opt, brew conformance thread (hold 33). vc's kovacs preflight ended green at 13:47Z, so the board-paths-only rule is lifted. tmp/wt-cc-0616t and tmp/wt-cc-0630 hold nothing unbanked and can be removed. No push, no release. (edited)
- ST0086 WP-01, the engine: RESUME STATE (cc, 2026-10-24 14:59Z by date -u), localfold for molt-vc's estate-wide fold drill. The work is the LOCAL commit a4ef63178 in tmp/wt-cc-0086 (hooks off, own in-tree target), not for landing, on main 935092cc2 (main has since moved by board events only). The first full judge, slot 321 14:35Z-14:39Z, was RED on three causes, all fixed. (1) The clippy-lib expect is now EstateError::Unwritable via WriteError::Io, no new variant (vc). (2) Attachment drift is fixed: `intent sync --to-store ST0086` and `ST0056` in the worktree carry design.md into .canon/st/*.json; no event is expected (vc); the views that sync re-rendered (todo.md, the four boards) were restored to HEAD. (3) D42 is resolved by hv's ruling, vc decision 72 at daede1c49: the one estate clock is src/estate/clock.rs, one_clock EXEMPT names two files, no_function_takes_a_time SEAMS exempts (estate.rs, join, now) and pins it, D42 records the reason in ST0056's design.md, and the WIP record's stamp is its file's mtime (wip_at serde-skipped, filled by stamped_at). The @1 boundary in design.md now carries the controller, lanes and queue shapes plus the census and usage inner keys (vc: no bump); ic has them and the shared fixture intentsvcs/tests/fixtures/estate-status-1.json in tmp/wt-ic-0086, with slots as queue ids. NEXT: the re-run holds FIRST place at molt-vc's desk and gets its GO after the compact; send START and END to molt-vc and intent-vc; on green, bank refs/bank/cc/0086/wp01 with blob and patch-id for vc to land before ic's surface bank. THEN, by hv's priority: WP-02's daemon half (Op::EstateStatus plus Op::Form for View::Estate answered unbound in intentd dispatch(), the watchers, the census and its ring through estate::clock, plan usage with Keychain primary and statusline fallback, an HTTPS client still to be decided, proposed /usr/bin/curl -K - so the token stays out of argv, to be measured first); then 04a, the slots engine, approved shape. (edited)

## TODO

_(none)_

## Holds

- HOLD ST0081 AC-02.1 and WP-02 (Linux assets on each tag's release, one writer per release object) UNTIL devbin v0.1.14 is re-vendored into Intent (hv cuts it privately; devbin 0213 at 76b3d790), with the macOS bats run a vendored devbin upgrade owes before the cut that carries it, AND the dev, opt, brew conformance thread has gone first, per hv's order; AC-02.1 is proven at the first cut after the re-vendor. The 3.2.6 tag condition is met (v3.2.6 at 88633fe93). D55 is ruled (hv, 2026-10-23: devbin's release core attaches the Linux assets, SHA256SUMS riding it), and devbin-vc's shape is the build: Intent declares `commands.release.enabled: true` with `release.verbs: [check, attach]`, `release.repo` and the assets keys; linux-release.yml triggers on tags matching `v[0-9]*`; the cut runs build release, macos prepare, macos publish, `bin/devbin release attach vX.Y.Z`, then brew formula --push. Every other ST0081 row is settled: AC-01.1, AC-01.2, AC-03.3 and AC-06.1 satisfied, WP-01, WP-03 and WP-06 done, and the gate reads BLOCKED on AC-02.1 alone. The release.remotes block is retired (vc), not owed. (edited)

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
