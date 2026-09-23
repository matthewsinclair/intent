---
node: dc
name: DevX Claude
role: worker
session_id: 1a8e4fe7-0650-45f6-a619-84fc0c380145
heartbeat_at: 2026-09-23 12:56Z
status: active
focus: "LOCALFOLD for hv's compact, 2026-09-23. RESUME at the RESUME HERE doing item: 0532 banked (60e20ebfc, patch-id c10efbc0d), its judging run after ic's review, then landing and close; then the known-defects re-drive. Hold 29. NO PUSH, NO RELEASE."
claims: [ISSUE:0532]
---

# DevX Claude (dc)

## DOING

- **RESUME HERE (dc localfold for hv's compact, 2026-09-23 12:57Z by date -u; on hv's word, continue on the bounce). NO PUSH, NO RELEASE.**
- 0532 IS BANKED, NOT JUDGED, NOT LANDED: refs/bank/dc/0532/v1, blob 60e20ebfc, patch-id c10efbc0d, 8 files +613/-12. It applies clean on the landed pair 084b154b9 (0525 at d3b2250b6, 0530 at 084b154b9), and vc READ it. tmp/wt-0532 holds it staged on the stack (index patch-id c10efbc0d) with its own warm target. The scripts are banked beside it under refs/bank/dc/0532/: setup, warm, dev, compose, prep_judge, judge, surface, close_draft.
- ic's SURFACE REVIEW PASSED, driven by ic on the worktree binary: pandoc reads every heading, sub-bullets nest, pickup's hv standing block is covered, doctor's --verbose note is as sent, and sync --to-disk restores the board. It carried three notes, none blocking:
  - (a) A fifth spelling, unindented with the mark. vc had already ruled it unneeded, because 0525 and 0532 deploy in one build all; it matters only if 0532 misses that build.
  - (b) The doctor note reads "differs, with any text the renderer owns". Make it "differs, and any text the renderer owns".
  - (c) For hold 29: prettier 3.9.8 rewrites board views in both shapes, so the recipe's "declare formatters" step must carry the prettier-ignore lines for the generated board and inbox views, as Intent's .prettierignore does.
- The judging run was NOT started: vc said not before the compact.
- NEXT 1: apply (b) in tmp/wt-0532, run cargo fmt, re-cut the bank as refs/bank/dc/0532/v2, and give vc the new blob and patch-id. Then run the judging run on vc's approved terms. Baseline HEAD (084b154b9, or the HEAD then if only canon moved); bank HEAD + 0532 v2. cargo build -p intentd first; the whole intentsvcs, intent-cli and intentd suites, then the whole bats suite, both sides, red sets diffed by name both ways. Run cargo clean -p for the four members in any cloned target, then the rlib grep for the other worktree's path with a control that can see it (judge.sh's rlib_check). The START names the base, the blob with its patch-id and git apply --stat, the tree and the census. END goes to cc, ic and vc; cc asked to be told. Update prep_judge.sh's bank to v2, run it with that HEAD, then judge.sh.
- NEXT 2: land on vc's word, reading the judged patch-id back off the landed commit, then close 0532 from close_draft.md. It names the 0525 re-edit gap the fourth spelling closes, and intent sync --to-disk as the driven verb.
- NEXT 3: the known-defects re-drive (todo 44) after the last landing and the second build all.
- The heavy order after dc's END: ic's 0533, then cc's 0534. After the landing, remove tmp/wt-0532 and tmp/wt-0532-base (cd out of them first).
- Hold 29 stands, with ic's note (c) as its rider.

## TODO

- KNOWN-DEFECTS RE-DRIVE (vc's pre-cut split, 2026-09-23; ic's brief banked verbatim at refs/bank/dc/kd/brief-ic.md). WHEN the last 3.2.1 landing is in, the second bin/devbin build all has ended, and intent --version prints intent 3.2.0 (<that HEAD>): re-drive docs/known-defects.md WHOLE on the installed pair only, each entry in a fresh scratch project under an isolated HOME (the daemon entry with its own scratch intentd, 0442 on a copy of its evidence store), starting from cc's banked drive at refs/bank/cc/kd/drive-log. Replace every transcript verbatim. An entry that no longer reproduces leaves the page, its commit naming the release that fixed it (0063's entry is driven against 0526's remedy, which ships in 3.2.1). Title v3.2.1; the pin sentence keeps the PRE-bump string exactly as the pair prints it. Commit with or straight after ic's reference-set commit and BEFORE step 3; nothing commits during a build all. Ask ic before bending the method. NO PUSH, NO RELEASE.

## Holds

- **THE GUARDS ADOPTION PASS FOR THE REMAINING ESTATES IS HELD UNTIL THE 3.2.1 CUT'S FLEET SWEEP (hv's ruling via vc, 2026-09-23).** The recipe runs intent claude upgrade --apply, so each estate's hooks wiring rides that sweep, one commit per estate for both. The pilot, MicroGPTEx 80d4c13, passed vc's check.
THE RECIPE: git config core.hooksPath .githooks; then intent claude upgrade --apply, which writes the four chain blocks (tracked) and the four .intent carriers; add .githooks/*.intent to .gitignore; declare formatters wherever the old hook checked any; red-control with staged probes; commit by path; run doctor.
THE ESTATES: Molt and Prolix take the recipe unchanged, because their hook is byte-identical to MicroGPTEx's. Anvil, Baize, Cdtempl, Courses and Riffle share a chain-block-only hook and need no formatter declaration. Prodinfra, Conflab and Lamplight each get their own diff first. vc sequences Devbin, Laksa and Gtools.
KNOWN LIMIT: core.hooksPath is per clone, so a fresh clone runs no hook until it is set, and only doctor --verbose's uncounted advisory says so.

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
