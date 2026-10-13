---
node: dc
name: DevX Claude
role: worker
session_id: 7dc8d33f-f427-4eb3-8abb-d33189c975ae
heartbeat_at: 2026-10-13 08:53Z
status: active
focus: "LOCALFOLDED for hv's demo of 2026-10-13: holding on hold 32 until vc relays that the demo is over; the first act after it is the read-only M2 sizing (hold 30); no claim, no worktree, no unlanded bank"
claims: []
---

# DevX Claude (dc)

## DOING

- **RESUME (dc, localfold for hv's demo, 2026-10-13): dc IS HOLDING ON HOLD 32 UNTIL vc RELAYS THAT THE DEMO IS OVER. NO CODE, NO BUILD, NO RECORD WRITES, NO PUSH, NO RELEASE.** dc booted on hv's "boot, then hold" and has written nothing but this fold. No claims, no worktrees, no unlanded bank. AFTER THE DEMO, in order: (1) size M2 read-only and send vc its size and mechanism (hold 30); (2) on vc's word on hv's 3.2.4 scope ruling, hold 31's lane, landing before cc's kovacs proof as hold 31 records; (3) prestage the known-defects re-drive. dc's PLAN NOTES, not yet ruled and not yet on the issue records: 0591 takes the draft its body names (18,299 bytes; the skill is unchanged since f9b3419a2, so its patch should still apply) as input rather than a judged result, re-probes rule survival with a planted-deletion control, and adds one arm holding the startup skills under 20,000 characters, red on today's 51,366 bytes; 0593 lands before 0592, stacked, because both edit in-session; 0592's post-compact rule fails toward reloading when the compact's note is absent, and its pickup runs as the intent wb pickup verb rather than reloading the skill. Hold 29 was re-checked at pickup and its condition is unmet.

## TODO

_(none)_

## Holds

- **THE GUARDS ADOPTION PASS FOR THE REMAINING ESTATES IS HELD UNTIL THE DEVBIN FLEET SWEEP REACHES THEM (hv's ruling via vc; re-checked 2026-09-26 at vc's order).** Devbin 0.1.8 is in for Intent only (f85f2663f). A read-only census of the 15 named estates found core.hooksPath set to .githooks in Intent, MicroGPTEx (the pilot) and Gtools, to bin/hooks in Laksa, and unset in the other 11, so the sweep has not reached the estates this hold is about. The recipe runs intent claude upgrade --apply, so each estate's hooks wiring rides that sweep, one commit per estate. Pilot: MicroGPTEx 80d4c13, which passed vc's check. (edited)
  THE RECIPE: git config core.hooksPath .githooks; intent claude upgrade --apply (writes the four chain blocks, tracked, and the four .intent carriers); add .githooks/*.intent to .gitignore; declare formatters wherever the old hook checked any; red-control with staged probes; commit by path; run doctor.
  THE ESTATES: Molt and Prolix take the recipe unchanged, because their hook is byte-identical to MicroGPTEx's. Anvil, Baize, Cdtempl, Courses and Riffle share a hook with only the chain block and need no formatter declaration. Prodinfra, Conflab and Lamplight each get their own diff first. vc sequences Devbin, Laksa and Gtools. vc rules the seven unnamed estates on a measurement of which carry Intent.
  RIDER: wherever the recipe declares a markdown formatter in an estate with a whiteboard, the same commit adds prettier-ignore lines for intent/whiteboard/*/wip.md and intent/whiteboard/*/inbox.*.md (prettier 3.9.8 rewrites the generated views).
  KNOWN LIMIT: core.hooksPath is per clone, so a fresh clone runs no hook until it is set.
- **THE M2 FIX IS HELD UNTIL hv RULES IT INTO 3.2.4; vc's RECOMMENDATION IS IN IF dc SIZES IT AT S OR LESS, AND THE SIZING WAITS ON HOLD 32 (the demo).** M2 is the spurious "not migrated" line a first v2 conversion prints. 3.2.2 shipped it documented, as option (b), and put the removable-v2-source fix on the list for the next release. M2 IS UNSIZED, AND ITS ONLY RECORD IS a935fd25c's BOARD DIFF. SIZING IT, read-only, IS dc's FIRST ACT AFTER THE DEMO (vc, 2026-10-13). It is the only 3.2.4 candidate under native/rust: if it is in, it owes the whole crate suites and the shell half, and it lands before cc's kovacs proof and before vc's build all (hold 31's sequencing). (edited)
- **0591, 0592, 0593 AND 0594, AND THE KNOWN-DEFECTS RE-DRIVE AT THE 3.2.4 CUT, ARE HELD UNTIL vc's WORD ON hv's 3.2.4 SCOPE RULING (vc's orders, 2026-10-12), AND UNTIL HOLD 32 (the demo) LIFTS.** vc recommends 0591 to 0594 in, as dc's lane. 0594 is the XS test fix vc filed at d7cd1ea7c: armed_tool_preconditions.bats:116 reds in any checkout whose path names an armed tool. THE SEQUENCING (vc in the live channel, 2026-10-13, agreeing with dc's reading, and put to hv with the scope question): the package ships intent/plugins/claude (bin/.devbin/cmd/shared/support.lib:76), so 0591, 0592 and 0593, which are packaged skills, and M2 if it is in, which is native/rust, land BEFORE cc's kovacs proof, and the proof then runs once, on the tree that gets pushed. 0594 (tests/unit only) and the docs (WP-03's pages, the CHANGELOG) can land on either side of it. The re-drive starts from refs/bank/dc/323/drive.sh (a00da3f2e), diffs every entry against refs/bank/dc/323/drive-3.2.3.log (e469fa33b), and runs only on the pair from the final pre-cut build all. The M2 fix stays under hold 30. (edited)
- **ALL dc WORK IS HELD UNTIL vc RELAYS THAT hv's DEMO OF 2026-10-13 IS OVER (hv's word, given in vc's session before vc's clock read of 08:47Z, relayed by vc).** Nothing is touched before the demo: no code, no build, no VM work, no push and no record writes, and the M2 sizing waits as well. This localfold is the one write it allowed. dc's FIRST ACT AFTER THE DEMO is the read-only sizing of M2 (hold 30), reported to vc, because hv's "in if S or less" needs that number.

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
