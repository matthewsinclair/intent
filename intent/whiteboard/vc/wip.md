---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 21:25Z
status: active
focus: "**BOOTED AFTER COMPACT 2026-08-25 21:09Z. A REPORTED SHELL REGRESSION WAS THE INSTRUMENT, NOT THE CODE: the full shell suite is 1480/1480 at a clean HEAD, and the two red tests were the only two of 1480 that read a bare `intent` off the OPERATOR'S PATH -- a different tree. Fixed and committed at `058c0400` with a mutation control.** The gate stays CLOSED at 67 of 67 and CLOSED IS NOT RELEASED. **WP-14 BLOCKS WP-12, unruled since 17:17Z, still the top blocker.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**A REPORTED TEST REGRESSION WAS A BROKEN INSTRUMENT AND IS FIXED (`058c0400`).** The red verdict carried its own warning -- `measured: 78a44edf +1 dirty -- THIS VERDICT DESCRIBES NO COMMIT` -- and it was right: **the full shell suite is 1480/1480 on a clean HEAD, runner exit 0.** The two failures were the ONLY two call sites in 1480 tests invoking a bare `intent` off PATH, which on this machine symlinks into the FROZEN Intentv2 tree, while the other eight in the same file drive `$INTENT_BIN`. **`0` is a value neither a duplicate (2) nor a correct dedup (1) can produce -- so the one reading impossible for a live subject was reported as the subject's answer, and the failure ACCUSED THE FEATURE IT GUARDS.** Reproduced byte-identically by removing `intent` from PATH; fixed file green with it present AND absent; mutation control red at `Actual 1`.

**NOTHING ELSE IN FLIGHT. GATE CLOSED, ALL FOUR INBOXES AT THE SENTINEL.**

**THE 3.0.0 CLOSURE GATE IS DONE AND IT IS THE SMALLER HALF OF THE RELEASE.** `ST0057` PASS 51/51; with `ST0056/03` at 16/16 that is 67 of 67. **`ST0056` IS 63/133 AND WP-12 IS NOT STARTED** -- the rewrite is the work that remains, not the gate.

**NOTHING IS TAGGED, PUSHED OR PUBLISHED AND ALL THREE NODES HOLD THAT INDEPENDENTLY.** ~54 commits ahead of `upstream/main`. **hv named the release as the AIM first-hand; the ACT is still hv's hand.**

## TODO

1. **THE CUT BLOCKER, AND IT OUTRANKS EVERYTHING ELSE HERE: WP-14 BLOCKS WP-12 AND THE DEPENDENCY IS STATED NOWHERE.** `AC-12.1` prunes `bin/` at the cut; the `claude ws` family -- the whiteboard PROVISIONER -- is contracted in **WP-14** (`AC-14.10`), Not Started. **At the cut it does not degrade, it DISAPPEARS.** The obvious fix is wrong: `AC-14.7` serves every `/in-whiteboard` verb FROM THE STORE, so porting the file-based implementation builds what WP-14 exists to replace. **I assigned that port to dc; dc refused it against a contract read and was right.** Both WPs unclaimed. **Unruled since 17:17Z.**
2. **TWELVE WITH hv, NONE MINE TO DECIDE.** A1 commit trailer; **A2 attribution guard -- HELD, I have not relayed and will not**; A3 WP-15 timing; A4 `fileindex`; A5 `--force` version mismatch; A6 **ST0058 SCOPE** (contract exists now, reports `0/6`); A7 TODO 8 ordering; A8 dc's three, named, one of which (**AT-11.7**) had never been asked; A9 the roster reading two populations from two trees; A10 two estate-wide blocks; A11 limb 2's denominator; A12 the marker's `DIRT_SCOPE` disagreeing with the guard's. **PLUS:** the `st attach` **SPELLING** (I authorised the build, not the name) and ic's **`## Holds`** protocol change (edits the shipped skill; fleet blast radius).
3. **ISSUES OPEN: 0082 (HIGH)** -- `sync --to-disk` reports `ok` whenever store and disk disagree about an attachment, **either direction**; five occurrences, two operators, one evening. **0084** -- one retirement left TWO survivors, a refusal in `ingest` and a class remedy at `finding.rs:267` emitted from **7 sites**; **editing the string closes the instance and leaves the class.** **0083** low.
4. **CARRIED:** `~/.claude/skills/` is one machine-global dir upstream of 15 committed `AGENTS.md` -- reshapes WP-15. `st dehydrate` unbuilt is a U3 datapoint. **ST0060 de-realised; only AC-00.16 open, one Lamplight read when scheduled.**

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AGREEMENT AND DISAGREEMENT ARE BOTH UNINFORMATIVE UNTIL YOU KNOW WHETHER THE TWO INSTRUMENTS ASKED THE SAME QUESTION** (ic). Same figure by the same method read as corroboration -- **one measurement run twice.** Different figures by different methods read as contradiction -- **two questions.** **MATCHING NUMBERS ARE NOT EVIDENCE AND DIFFERING NUMBERS ARE NOT A DEFECT; the only thing that carries information is the QUESTION, and a number does not carry its question with it.**
2. **A GUARD WHOSE SCOPE EXCLUDES THE CASE IT EXISTS TO CATCH READS GREEN EVERY TIME.** Five in one thread. **Siblings:** an instrument comparing **two populations from two different trees** (dc); a **write whose report does not describe what it wrote** (`git stash pop`, ic -- a stash is a pointer, a copy is bytes); an **empty population returning `0 with, 0 without`**. **THE CURE IS RARELY A BETTER CHECK -- and the only DURABLE fix found in eight tries was a SHAPE: the wrong sentence made unsayable by the type** (cc). **NEW SIBLING, AND IT IS A CHEAP TEST TO RUN ON ANY RED: ASK WHETHER THE REPORTED VALUE IS ONE A LIVE SUBJECT COULD PRODUCE AT ALL.** st_enumeration reported `0` where a duplicate gives 2 and a correct dedup gives 1 -- **the single value that RULES OUT the hypothesis the test is about**, so the failure accused the feature it guarded. **A NUMBER OUTSIDE THE SUBJECT'S RANGE IS NEVER THE SUBJECT'S ANSWER**, and `2>/dev/null | grep -c` is the shape that manufactures one.
3. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A workaround outlives its bug because **a defect announces itself and a defect's disappearance does not** (dc). **A remedy that was never true is caught by review; one that EXPIRES is caught by nobody** -- two of eight today. **A restatement inherits the trust earned by admitting the error** (ic) -- my closing condition was corrected twice and neither correction was more driven than the sentence it replaced. **A driven wrong answer outranks an undriven right one** (ic).
4. **DRIVE WHAT YOU RELAY, AND I AM THE FASTEST PATH.** dc measured _one staged file_, truly; **staged was never the predicate.** I relayed the framing inside a minute and routed the wrong node. **A CORRECT MEASUREMENT DESCRIBED IN THE WRONG TERMS TRAVELS AS THE WRONG RULE** (dc).
5. **A FALSE-POSITIVE GUARD IS WORSE THAN A NARROW ONE: the first fails closed against a reader who checks, the SECOND CORRUPTS THE READER** (cc). **`pgrep -f` IS STRUCTURALLY UNUSABLE IN AN AGENT FLEET.** **AND A GUARD'S VALUE IS IN THE PART IT PRINTS BEFORE THE RESULT** -- I `tail`ed one and read the private pair's hash as the shared one. **CAPTURE THE `rc`, NOT THE PIPELINE'S.**
6. **SHARED CHECKOUT: `--only <paths>` IS NOT A NICER SPELLING OF _CHECK THE INDEX FIRST_ -- IT IS THE ONLY FORM ATOMIC WITH RESPECT TO PEERS** (dc). Reading `git diff --cached` measures a MOMENT; the commit happens at another. **I printed `staged before: 3` and let an `&&` chain proceed.** **Two individually-correct moves compose into an outage.** **ANNOUNCING PROTECTS AGAINST A NODE WHO HAS NOT ANNOUNCED AND DOES NOTHING AGAINST A SIMULTANEOUS ONE.**
7. **A CORRECT FIX AND NO FIX PRODUCE IDENTICAL OUTPUT WHEN THE READER IS EMBEDDED** (cc). `include_str!` means a shared build captures a peer's mid-edit file and the binary outlives the edit -- **a DIAGNOSTIC hazard, and the feedback lies in the direction that makes a correct engineer undo correct work.** **A green is a claim about a TREE.**
8. **CANON IS THE SSOT AND FREE FILES ARE AUTHORED ON DISK.** Route: edit `.canon` -> `--to-store` -> `--to-disk`, **never `--to-disk` first.** No CLI verb CREATES an AC or an AT.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED.**
- **AN IMPLEMENTATION NARROWER THAN ITS CRITERION LOOKS EXACTLY LIKE A CORRECT GATE.** The fix is quoting the criterion back at the tool.
- **BYPASSING A GATE IS NOT MINE TO CHOOSE**, even holding decisive evidence the gate has no model of the case.
- **THE VERIFIER FINDS THE DEFECT IN THE PART YOU DID NOT FLAG.** A specifier's own list of doubtful items is drawn from where they already looked.
- **A WRONG CAUSAL STORY ABOUT YOUR OWN BUG IS WORSE THAN THE BUG.**
- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv). **CAPABILITY EXISTS != ROUTE EXISTS ON THE SURFACE**, which is what AC-08.5 turned on twice.
- **A CROSS-REPO OBLIGATION BELONGS TO THE THREAD THAT TRIGGERS IT, NOT THE REPO THAT WAITS** (hv).
- **CAREFUL MEASUREMENT OF AN UNASKED QUESTION COSTS MORE THAN CARELESS MEASUREMENT OF A REAL ONE** (hv).
- **A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.**
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM; `imprimatur` DID NOT MOVE THE ATTRIBUTION BOUNDARY** (ic's reading, endorsed). **_Get the release done_ is not _release it_.**
- **A NAME PICKED UNDER THE PEN BECOMES THE RULING BY DEFAULT** -- the ground for authorising the `st attach` BUILD and routing its SPELLING to hv.
- **A RATIFIED RULING IS NOT AN EXECUTED ONE.** treeindex, ten days. Cause was PACKAGING.
