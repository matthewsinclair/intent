---
node: dc
name: DevX Claude
role: worker
session_id: 45688c36-a10d-4ad6-b27b-9f7011eb1956
heartbeat_at: 2026-10-16 12:23Z
status: active
focus: "LOCALFOLDED 2026-10-16 on hv's word via vc, resuming on vc's instruction: doing 125 is the resume state; the 3.2.4 lane is 8 banks on refs/bank/dc/324/ (0605 at patch-id ac55f52517dd); slot 1 runs slot1.sh on vc's GO after devbin-dc's WP-03 END; no push, no release"
claims: [ISSUE:0591, ISSUE:0592, ISSUE:0593, ISSUE:0594, ISSUE:0596, ISSUE:0597, ISSUE:0599, ISSUE:0605]
---

# DevX Claude (dc)

## DOING

- **THE 3.2.4 TRAIN UNDER hv's RULING (vc decision 63) AND vc's ORDERS IS THE RESUME STATE, AS LOCALFOLDED ON hv's WORD VIA vc (dc, 2026-10-16).** hv ruled 0591, 0592, 0593, 0594, 0596 and 0597 into 3.2.4; 0590 waits on its design question; 0599 (intent --version migrates ~/.intent) is dc's in 3.2.4 after the train is judged; 0600's CHANGELOG line is already on main at 92d5a77ce. vc assigned 0605 (critical, the scoped-sync doubling of the prose index) to dc under hv's delegated pen (decision 4), first in the train after 0594. The rulings are recorded in 0596's and 0597's issues and the claims taken at eeabc0df7. THE LANE: tmp/wt-dc-324, detached at base 1c84752b4 with every change staged, equals this stack, which applies onto main at bb437bc86: refs/bank/dc/324/0593 (patch-id 5e3381fa4f55), 0592 (267a20d62f3f), 0591 (484cb0f6e47b), 0594 (c5eab1feaf88), 0605 (blob 864722699383, patch-id ac55f52517dd: the canon-half guard in Store::write_doc_sections, compose_scoped's read-back filter, rung 31 DOC_SECTIONS_DEDUPE, doctor's uncounted prose-index-duplicated finding whose remedy names intent index rebuild, and a three-arm test file), 0596 (0e9574949b4a), 0597 (8c35467adc86) and changelog (83826c3c1cb3); refs/bank/dc/tools/verify-lane.sh repeats the check read-only. NEXT, IN ORDER: (1) SLOT 1 on vc's GO, which follows devbin-dc's WP-03 END: START to vc and cc, then refs/bank/dc/324/evidence/slot1.sh (8566e6bcf) in the background, which runs a census, seeds the worktree's own target by a cp -c clone of main's with cargo clean -p of the three crates (vc's OK), builds intentd, then runs the arms RED with ingest.rs, facade.rs and legacy.rs at T0 and the store.rs guard disabled, then GREEN on the whole train. Expected red: the doubling arm, 0596's arm, 0597's unit and CLI arms; green both phases: the rung, doctor and 0597-control arms. SLOT 1 IS DONE (dc, 2026-10-16; the log ends "done at 12:26:01Z", exit=0; END sent to vc and cc): every arm went as expected, red with the fixes out (the doubling arm left file 4 and wb_node 4 against 1 and 1; 0596's arm printed the not-examined line; 0597's unit arm found no removable views; 0597's CLI arm found the held thread's v2 info.md surviving) and all green on the train (svcs 13 of 13, cli 9 of 9); schema_faces_drift green in both phases, so no published-schema update rides in 0605; no arm needed a change, so no bank moved; log at refs/bank/dc/324/evidence/slot1.log (blob 2528ed45deb7). vc judges the whole train next in a worktree of its own, so slot 3 is vc's and steps (2) and (4) are done or vc's. (2) The schema-faces verdict to vc: schema_faces_drift green, or the published schema updated in the 0605 bank with its reason. Any arm fix is re-banked and its blob, sha256 and patch-id sent to vc. (3) Logs banked under refs/bank/dc/324/evidence/, ISSUE:0605 claimed, one chain by literal path. (4) SLOT 3, the whole-train judging run on vc's GO: a short isolated HOME, an in-tree CARGO_TARGET_DIR, cargo build -p intentd first, intentsvcs's and intent-cli's whole suites with --no-fail-fast, clippy -W, fmt, the rust.yml doc line, tests/run_tests.sh, logs banked, verdicts to vc verbatim. (5) Land one bank at a time on vc's hash, 0593, 0592, 0591, 0594, 0605, 0596, 0597, changelog, each patch-id checked and each issue closed with its commit. (6) Then 0599, size S: migrate_legacy moves out of main before dispatch into the verbs that use user state; an arm shows --version leaves a legacy ~/.intent byte for byte while bootstrap migrates it. STILL WITH hv: who ports 0589's quote classes to llm-tropes, and when intent claude skills sync runs. NO PUSH, NO RELEASE. (edited)

## TODO

_(none)_

## Holds

- **THE GUARDS ADOPTION PASS FOR THE REMAINING ESTATES IS HELD UNTIL THE DEVBIN FLEET SWEEP REACHES THEM (hv's ruling via vc; re-checked 2026-09-26 at vc's order).** Devbin 0.1.8 is in for Intent only (f85f2663f). A read-only census of the 15 named estates found core.hooksPath set to .githooks in Intent, MicroGPTEx (the pilot) and Gtools, to bin/hooks in Laksa, and unset in the other 11, so the sweep has not reached the estates this hold is about. The recipe runs intent claude upgrade --apply, so each estate's hooks wiring rides that sweep, one commit per estate. Pilot: MicroGPTEx 80d4c13, which passed vc's check. (edited)
  THE RECIPE: git config core.hooksPath .githooks; intent claude upgrade --apply (writes the four chain blocks, tracked, and the four .intent carriers); add .githooks/*.intent to .gitignore; declare formatters wherever the old hook checked any; red-control with staged probes; commit by path; run doctor.
  THE ESTATES: Molt and Prolix take the recipe unchanged, because their hook is byte-identical to MicroGPTEx's. Anvil, Baize, Cdtempl, Courses and Riffle share a hook with only the chain block and need no formatter declaration. Prodinfra, Conflab and Lamplight each get their own diff first. vc sequences Devbin, Laksa and Gtools. vc rules the seven unnamed estates on a measurement of which carry Intent.
  RIDER: wherever the recipe declares a markdown formatter in an estate with a whiteboard, the same commit adds prettier-ignore lines for intent/whiteboard/*/wip.md and intent/whiteboard/*/inbox.*.md (prettier 3.9.8 rewrites the generated views).
  KNOWN LIMIT: core.hooksPath is per clone, so a fresh clone runs no hook until it is set.
- **THE KNOWN-DEFECTS RE-DRIVE IS HELD UNTIL THE 3.2.4 CUT'S FINAL PRE-CUT build all PAIR EXISTS.** It starts from refs/bank/dc/323/drive.sh (a00da3f2e), diffs every entry against refs/bank/dc/323/drive-3.2.3.log (e469fa33b), and runs only on that pair. (Re-scoped 2026-10-16: the scope half of this hold was met by hv's ruling, vc decision 63, and the train's judging and landing are doing 125's.) (edited)

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
