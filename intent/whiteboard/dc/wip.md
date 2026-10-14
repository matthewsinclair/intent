---
node: dc
name: DevX Claude
role: worker
session_id: ecdcf3a6-63f3-4391-ad53-b521b03fdda0
heartbeat_at: 2026-10-14 10:14Z
status: active
focus: "LOCALFOLDED for hv's compact cycle of 2026-10-14: after the compact, /in-session and the pickup, then HOLD until hv's restart; resume is doing 123 at the M2 drive (its state on hold 30), then the lane; no worktree, no bank, no push, no release"
claims: []
---

# DevX Claude (dc)

## DOING

- **vc's ORDER OF 2026-10-14 10:00Z (a stamp vc itself marks unverifiable: vc's clock reads were 09:59Z and 10:06Z, and the order went out between them; vc END; records at 7d1d2b8d4, vc's board at b38bc26bc), IN ORDER: (1) SIZE M2 read-only (hold 30): a935fd25c, the source, and one drive on the installed pair in a scratch project; send vc the size, the mechanism and what the fix owes; hold 30 stays until hv rules M2 in. (2) THEN BUILD THE LANE stacked in one worktree named for a number: 0593, then 0592 on top, then 0591, then 0594, banking each piece as it goes; M2 joins only if hv rules it in. Nothing lands before hv's 3.2.4 scope ruling, and there is no judging run before vc's GO, because the box queue carries kovacs's runs (hold 31). (3) The known-defects re-drive stays for the cut. NO PUSH, NO RELEASE.** LOCALFOLDED FOR hv's COMPACT CYCLE OF 2026-10-14 (hv's word relayed by another estate's session and by vc; fold order cc, dc, vc): after the compact, /in-session and the pickup, then HOLD until hv gives the restart. RESUME AT (1): the M2 sizing paused after its source read and before its drive, and its state is on hold 30. Nothing of (2) is built: no worktree, no bank. dc's plan notes, carried from doing 122 and not yet ruled: 0591 takes the Laksa draft its body names (18,299 bytes, against f9b3419a2) as input rather than a judged result, re-probes rule survival with a planted-deletion control, and adds one arm holding the startup skills under 20,000 characters, red on today's 51,366 bytes; 0593 goes before 0592 because both edit in-session; 0592's post-compact rule fails toward reloading when the compact's note is absent, and its pickup runs as the intent wb pickup verb rather than reloading the skill. (edited)

## TODO

_(none)_

## Holds

- **THE GUARDS ADOPTION PASS FOR THE REMAINING ESTATES IS HELD UNTIL THE DEVBIN FLEET SWEEP REACHES THEM (hv's ruling via vc; re-checked 2026-09-26 at vc's order).** Devbin 0.1.8 is in for Intent only (f85f2663f). A read-only census of the 15 named estates found core.hooksPath set to .githooks in Intent, MicroGPTEx (the pilot) and Gtools, to bin/hooks in Laksa, and unset in the other 11, so the sweep has not reached the estates this hold is about. The recipe runs intent claude upgrade --apply, so each estate's hooks wiring rides that sweep, one commit per estate. Pilot: MicroGPTEx 80d4c13, which passed vc's check. (edited)
  THE RECIPE: git config core.hooksPath .githooks; intent claude upgrade --apply (writes the four chain blocks, tracked, and the four .intent carriers); add .githooks/*.intent to .gitignore; declare formatters wherever the old hook checked any; red-control with staged probes; commit by path; run doctor.
  THE ESTATES: Molt and Prolix take the recipe unchanged, because their hook is byte-identical to MicroGPTEx's. Anvil, Baize, Cdtempl, Courses and Riffle share a hook with only the chain block and need no formatter declaration. Prodinfra, Conflab and Lamplight each get their own diff first. vc sequences Devbin, Laksa and Gtools. vc rules the seven unnamed estates on a measurement of which carry Intent.
  RIDER: wherever the recipe declares a markdown formatter in an estate with a whiteboard, the same commit adds prettier-ignore lines for intent/whiteboard/*/wip.md and intent/whiteboard/*/inbox.*.md (prettier 3.9.8 rewrites the generated views).
  KNOWN LIMIT: core.hooksPath is per clone, so a fresh clone runs no hook until it is set.
- **THE M2 FIX IS HELD UNTIL hv RULES IT INTO 3.2.4; vc's RECOMMENDATION IS IN IF dc SIZES IT AT S OR LESS.** M2 is the line a first v2 conversion prints, "this project has not been migrated to Intent v3", on the line before its ok:. 3.2.2 shipped it documented as option (b), at docs/migrating-from-v2.md:18, which tells the reader the ok: line is the verdict, and a935fd25c's board diff put "the removable-v2-source fix" on the list for the next release. THE SIZING IS UNDER WAY ON vc's ORDER (doing 123), PAUSED FOR hv's COMPACT CYCLE AFTER ITS SOURCE READ AND BEFORE ITS DRIVE. FROM THE SOURCE, NOT YET DRIVEN: the text is Pending's Display (intentsvcs/src/project.rs:1448). Facade::upgrade (intentsvcs/src/facade.rs:4554), once the migration has landed and the v2 tree is pruned, opens a facade on project.clone(), the Project discovered BEFORE the conversion. On dc's reading its in-memory config still declares the v2 version, so project.migration() is Pending, Facade::open refuses Unmigrated, and the Err arm puts "the views of undeclared threads were not examined: <cause>" into dehydrate_refused, which upgrade() prints just before ok: (intent-cli/src/render.rs, fn upgrade). IF THE DRIVE BEARS THAT OUT, THE LINE IS NOT SPURIOUS: it truly reports that issue 0316's dehydration of undeclared threads' views never runs on a first conversion, and the page tells the reader to ignore it. That is a different mechanism from the one a935fd25c's name for the fix implies, and the drive decides between them. FIX SHAPE ON dc's READING: rediscover the project from its root once the conversion has landed and before Facade::open; one arm driving a first conversion of a v2 fixture, red at the base on the line and the unexamined dehydration and green with the fix; retire the page's sentence. OWED: intentsvcs's and intent-cli's whole suites and the shell half; no extra build all, because one is already owed before the push. PROVISIONAL SIZE, BEFORE THE DRIVE: S. NEXT: the drive, on the installed pair in a scratch project under a short /tmp HOME, then the size, the mechanism and the owed list to vc for hv's batch. It is the only 3.2.4 candidate under native/rust. (edited)
- **LANDING 0591, 0592, 0593 AND 0594 IS HELD UNTIL hv RULES ON 3.2.4's SCOPE, AND THE TRAIN'S ONE JUDGING RUN WAITS ON vc's GO (vc's order of 2026-10-14, stamped 10:00Z, a stamp vc itself marks unverifiable).** The same order released the building and the banking, which this board's DOING carries. vc recommends 0591 to 0594 in, as dc's lane; 0594 is the XS test fix vc filed at d7cd1ea7c. THE TRAIN: one worktree named for a number, holding 0593, then 0592 on top, then 0591, then 0594, with M2 only if hv rules it in (hold 30); one judging run stands for the whole train, and the banks land one at a time on vc's word. THE SEQUENCING against the Linux proof is vc's to set: hv's word of 2026-10-14 made ST0081 the Homebrew thread and moved the WSL work to ST0082 (7d1d2b8d4), and the package still ships intent/plugins/claude (bin/.devbin/cmd/shared/support.lib:76, read 2026-10-14). THE KNOWN-DEFECTS RE-DRIVE stays for the cut: it starts from refs/bank/dc/323/drive.sh (a00da3f2e), diffs every entry against refs/bank/dc/323/drive-3.2.3.log (e469fa33b), and runs only on the pair from the final pre-cut build all. (edited)

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
