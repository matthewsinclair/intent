---
node: cc
name: Control Claude
role: control
session_id: 66e84138-c919-405e-bed0-8057e34a7ac2
heartbeat_at: 2026-10-24 18:29Z
status: active
focus: "ST0086 WP-02 READY at 0dcf5c0ba on 138c09ca6 (doing 89): light rerun at molt-vc's desk beside Laksa (2), then the wp02 bank for vc; then ic's heavy, build all, 0632; devbin v0.1.14 re-vendor pre-empts when hv cuts it (doing 88, hold 33); no push, no release"
claims: [ST0081, ST0086/01, ST0086/04]
---

# Control Claude (cc)

## DOING

- RESUME STATE (cc, 2026-10-24 14:21Z by date -u), localfold before a compact. 3.2.6 is cut and pushed (tag v3.2.6 at 88633fe93) with cc's landings in it. WAITING ON: hv cutting devbin v0.1.14 privately (devbin 0213 landed at 76b3d790). NEXT, in 3.2.7 and after dc's 0635: re-vendor devbin v0.1.14 into Intent, with the macOS bats run a vendored devbin upgrade owes, booked through vc with molt-vc; it pre-empts ST0086 (doing 89) when it comes; then ST0081 WP-02's Intent half after the dev, opt, brew conformance thread (hold 33). vc's kovacs preflight ended green at 13:47Z, so the board-paths-only rule is lifted. tmp/wt-cc-0616t and tmp/wt-cc-0630 hold nothing unbanked and can be removed. No push, no release. (edited)
- ST0086, RESUME STATE (cc, 2026-10-24 18:26Z by date -u), localfold on molt-vc's fold call for hv. WP-01 LANDED on main: 84b680035 from cc's two banks (refs/bank/cc/0086/wp01, then wp01-fix blob fae018c86, patch-id 473cbdfed, judged green in the light slot 17:33Z-17:39Z), then vc's design.md line-13 correction at 138c09ca6. WP-02 is READY at 0dcf5c0ba in tmp/wt-cc-0086 (detached, hooks off, own in-tree target), rebased with `git rebase --onto 138c09ca6 6919f6e47`: Op::EstateStatus answered unbound in intentd dispatch(), the census and its in-memory ring on a 60s tick, plan usage by /usr/bin/curl -K - with the Keychain service keyed on CLAUDE_CONFIG_DIR being set and no fallback, the endpoint read only once a page has asked, the statusline fallback, the tick reaping gone sessions' WIP records, no watcher (vc decision 73); the light slot's two reds are fixed in 5999f4a2e (the report test's epoch now 1792857600 with its Z form; utc_ms folded into resets_at(row), vc approved, SEAMS untouched); design.md recorded as built with its canon in 0dcf5c0ba; critic clean on every changed src file, fmt clean, prettier clean. NEXT: the light rerun is booked beside Laksa (2) at molt-vc's desk; on GO send START and END by name to molt-vc and intent-vc and run scratchpad judge-0086-wp02b.sh (pass-2 legs at 0dcf5c0ba); on green bank refs/bank/cc/0086/wp02 = git diff --binary 138c09ca6 0dcf5c0ba ("" as the old value) and send vc blob, sha256 and patch-id; on red send vc the cause before any fix. vc lands it after ic's bank over the same base; then ic's heavy run, Intent build all right after ic by hv's ruling (restart announced first), then dc's 0632. Still to come: the Op::Form View::Estate arm after ic's nav hunk, the statusline step calling usage::report (its wiring is hv's), the census watch/unwatch verbs, then 04a. No push, no release. (edited)

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
