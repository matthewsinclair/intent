---
node: cc
name: Control Claude
role: control
session_id: 499621e2-fc3c-4e68-95a3-009e5e8b84c4
heartbeat_at: 2026-10-15 14:08Z
status: active
focus: "ST0083: WP-05 banked and passed, landing only on hv's own word in cc's session; ST0081: WP-03 landed, 0601's fix shape banked for vc after the restart, hv owes (c) and (d); no push, no release"
claims: [ST0081, ST0083]
---

# Control Claude (cc)

## DOING

- ST0081, HOMEBREW ON LINUX (cc's lane; vc directs, and kovacs runs vc's orders). LANDED 2026-10-15 on vc's judgement, each with the bank's patch-id: issue 0600's brew smoke fix at 163c8b58c (2f1c1e4c4a94), with 0600 closed at c79197089 and proven on kovacs by ORDER 6a's green re-run; WP-03's docs (docs/install.md#linux, README, working-in-a-team, and two CHANGELOG 3.2.4 lines composed with dc's refs/bank/dc/324/changelog in both orders) at 8c6e1cd39 (8c6b1307dc1c), with WP-03 started at cbc3aa079. OPEN, each on its condition: WP-03 is WIP until AC-03.3, the post-cut run of the page, verbatim, from the published tap on Linux. WP-06 is WIP until AC-06.1, the clean install from matthewsinclair/intent after 3.2.4's cut on a machine with no checkout; 6a's two runs and 6b are on its note (no Linux step needs brew trust, and the published 3.2.3 formula requires macOS). WP-07: AC-07.1's evidence is complete (o7-b, then hv's real session, where the gate blocked and released and Stop fired, with SessionStart unconfirmed), and its satisfy waits on hv's word, since it was observed with issue 0601 attached. WP-01 and WP-02 wait on the push (the workflow_dispatch run). ISSUE 0601 (CRITICAL, fea7dd410: upgrade --apply wires the prompt gate without the skill that releases it, and nothing inside the session can release it): hv rules its lane, and vc recommends cc. cc's fix SHAPE is banked at refs/bank/cc/0081/0601-shape.md (blob 720f71d2417e), with the hooks-doc check at refs/bank/cc/0081/evidence/0601-hooks-doc.md, and goes to vc after the restart. (b) comes first and alone: the gate fails open with a JSON systemMessage where ~/.claude/skills/in-session/scripts/release-gate.sh is absent, and its tests and both smokes are driven under a temp HOME. Then, by hv's (d), (a1) upgrade --apply installs in-session or (a2) its report names the verb, with a doctor advisory and one docs line. hv owes (c), whether the gate passes ! input and peer messages, and (d), whether a hard gate is on by default for a fresh user. NOTHING IS BUILT. STANDING: defects kovacs finds come to cc as issues, fixed in banks vc judges; one heavy run at a time, on vc's GO after a census. NO PUSH, NO RELEASE. (edited)
- ST0083, THE AGENT CONTRACTS LANE (vc's order of 2026-10-15 on hv's "Ok, proceed"). LANDED on vc's judgement with the bank's patch-id: WP-01 at 16f351a90 (the @AGENTS.md import), WP-02 at a85dfbdf0 (the contracts key, both layouts) and WP-03 at 2d9b4de1a (init --contracts and agents layout). WP-01 to WP-03 are DONE, and 6 of 14 criteria are satisfied. Issue 0598 (CLAUDE.md's copy of the four-rule index) is filed for the next line. WAITING ON hv'S OWN WORD IN cc'S SESSION, asked 2026-10-15 and not yet answered: WP-05, the deny (Edit(/AGENTS.md) and Edit(/.claude/AGENTS.md) in the settings template and, byte for byte, in this repo's own .claude/settings.json, with AT-05.1's bats arm). It is BANKED at refs/bank/cc/0083/wp05 (blob 3fd6377f1c12, base ba3babaea, patch-id edcada5fa2eb), ran green (3108 passed, bats 782 of 782) and was PASSED by vc. It still applies cleanly to main, since none of its files have moved. It changes the settings every session here runs under, so it lands only on hv's word in cc's session, never on a relay. Then the records (AT-05.1, AC-05.1, WP-05 done). THEN, each shape first and one bank each: WP-07 (Dropbox mark, whose shape vc accepted), WP-04 (doctor) and WP-06 (deinit). WP-08 comes LAST, after dc's train lands, with its CHANGELOG lines composed on dc's refs/bank/dc/324/changelog in both orders. The worktree tmp/wt-cc-0083 is KEPT: detached at ba3babaea, with WP-05 staged and its own cargo target cloned from main's; WP-07 builds there after WP-05 lands. RECORDED: init renders the llm/ and prj/ templates embedded at build, so the key and the import reach init on this machine only after build all, which is owed before the push. STANDING: every Rust bank owes the workspace suites after cargo build -p intentd, CI's clippy, doc and fmt lines, and the shell half; questions go to vc. FIRST STEPS AT THE NEXT START: /in-session and the pickup, then ask hv by name for WP-05's word, and send vc 0601's shape. NO PUSH, NO RELEASE. (edited)

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
