---
node: dc
name: DevX Claude
role: worker
session_id: 45688c36-a10d-4ad6-b27b-9f7011eb1956
heartbeat_at: 2026-10-16 13:58Z
status: paused
focus: "LOCALFOLDED 2026-10-16 on hv's word via vc, holding for hv's compact: the 3.2.4 train is LANDED on main (8 banks, 15 commits, 87fdc6bcf..dfe4ff8eb, every patch-id as judged; issues 0591 0592 0593 0594 0596 0597 0605 closed); hv's build all is still owed; 0599 is next, not started; holds 29 and 31; no push, no release"
claims: [ISSUE:0599]
---

# DevX Claude (dc)

## DOING

- **RESUME STATE (dc, localfold 2026-10-16 on hv's word via vc: hv is moving, pick up later). THE 3.2.4 TRAIN IS LANDED ON MAIN AND NOTHING OF dc's IS IN FLIGHT.** vc's first judging run (judge-324) was HOLD: four reds were the train's own, five were cc's 0601 (b). dc re-banked three: 0591 (the claim-forms sentence restored byte-equal to model::CLAIM_ADDRESS_FORMS) to patch-id 3a1bff190ae1; 0605 (the remedy literal no longer names the issue) to 0015e32c9e97; 0597 (two migration arms now assert that an undeclared thread's v2 cover is removed and an attachment beside it stays byte for byte) to f7a6758e169b. The earlier blobs are kept as refs/bank/dc/324/{0591-v1,0605-v2,0597-v1}. dc's eight plus cc's a1-v2 and fixup compose in both orders to tree f37ce22d2. vc's re-judge of that tree PASSED at 13:20:29Z (refs/bank/vc/324/evidence/judge-324b.log). dc landed the eight on vc's word, two commits each (the bank by literal path behind census-ok, with the patch-id checked before and after; then the issue's Resolution and close), with the script at scratchpad/land/land.sh:
  - 0593: ce873474a, closed at 15cbac92c
  - 0592: d6a372833, closed at 51c60abc3
  - 0591: be387b038, closed at 6d4df7a2a
  - 0594: 086e98b71, closed at eafc07c40
  - 0605: 915245eb2, closed at d3c1436b4
  - 0596: e2d8519ce, closed at 6c1c571b1
  - 0597: 58f3e7a9b, closed at 2bbfa6dc7
  - changelog: dfe4ff8eb
  cc lands a1-v2 and the fixup after dc's END. STILL OWED, not dc's: hv's build all and the push. tmp/wt-dc-324 is kept: its staged tree is the landed train, and its target is seeded. No push, no release.

## TODO

- **ISSUE 0599 (S), NEXT IN dc's LANE, NOT STARTED: `intent --version` must not migrate ~/.intent.** No code is written, by hv's word via vc on 2026-10-16. Shape read so far: main.rs:17 runs userstate::migrate_legacy before dispatch. spine::parse returns DisplayHelp and DisplayVersion as Err(EXIT_OK) before any verb runs (spine.rs:869), so running the migration at the head of render::run, after parse, keeps --version and --help out of it with no list; `info` is skipped there by name, and the note is printed from render. intentd's main has no migration call. The arm: under a HOME carrying a legacy ~/.intent, `intent --version`, `--help` and `info` leave it byte for byte, and `intent bootstrap` migrates it and prints the note. Build in a fresh worktree off main, with its own target, on vc's GO. It starts in the NEXT session, on vc's GO after hv's build all and daemon restart (vc, 2026-10-16 13:5xZ, after its globalfold at b96a14c78). (edited)

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
