---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 17:40Z
status: active
focus: "**DRIVING THE FLEET ON hv's IMPRIMATUR WHILE hv IS AFK -- cc, ic AND dc ALL LIVE AND BUILDING.** AC-08.5's field-axis denominator is RULED and landed at `8f03d9c7`, which was cc's stated blocker. cc builds two `settable_fields` arms + the thread-door graft; ic builds the field-axis instrument; dc has AT-11.6 then `hook_compat.rs`. **TWO THINGS ARE WITH hv AND ONE IS A CUT BLOCKER: WP-14 blocks WP-12, and AT-11.7 was refused twice from one premise and never asked.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**DRIVING THE FLEET. hv IS AFK AND GAVE THE PEN AND THE IMPRIMATUR FOR AN AGGRESSIVE PLAN TO GET THE 3.0.0 RELEASE DONE** -- first-hand to me (_"land the rest of this intent3 work"_, _"press on aggressively"_) and to ic the same hour (_"VC has the pen, and my impramatur"_). **THE AIM IS NOW THE RELEASE; THE ACT OF RELEASING IS STILL hv's HAND.** _Get the release done_ is not _release it_ -- nobody tags, pushes or publishes under any plan of mine. **ic drew that line before I did and would have refused it by default**, which is the right instinct and is now the standing reading.

**AC-08.5 IS THE ONLY UNSATISFIED ROW ON THE 3.0.0 GATE AND ITS DENOMINATOR IS RULED -- `8f03d9c7`.** Limb 1's population is indexed by MODEL TYPE, not address form: 6 field-carrying entities, not 13 address forms, `settable_fields` covering 4 of 6. **That was cc's stated blocker -- they escalated rather than building, which was right.** In flight now: **cc** two arms (`Attachment`, `Issue`) + the thread door's 8-of-8 graft; **ic** the field-axis instrument, asserting through trait bounds rather than scanning source; **dc** AT-11.6 then `hook_compat.rs` + `AC-07.4(b)`.

**THE PEN IS NOT hv's AUTHORITY AND `imprimatur` DID NOT MOVE THAT BOUNDARY** (ic's reading, endorsed). Every ruling today is recorded as vc's. **A2 STAYS HELD -- I will not relay hv's authorisation to dc and dc should refuse it if I do.**

## TODO

1. **WITH hv, NONE MINE TO DECIDE:** A1 commit trailer; A3 WP-15 timing; A4 `fileindex`; A5 `--force` version mismatch; A6 **ST0058's contract -- NO LONGER BLOCKING, six criteria landed at `562d48d`, the thread reports `0/6` instead of refusing; what remains with hv is SCOPE, which gates bind the 3.0.0 cut**; A7 TODO 8 ordering; A8 dc's three, now NAMED (A2 attribution guard; the roster-charter ruling; **AT-11.7 scope, which had NEVER been put to hv -- refused twice by two nodes from one premise hv has since retired**). **NEW: A9** the roster check reading two populations from two trees; **A10** two estate-wide commit blocks, half the second mine; **A11** limb 2's denominator. **A2 STAYS HELD -- hv is AFK and I will not relay.**
2. **THE CUT BLOCKER, AND IT IS THE MOST IMPORTANT THING ON THIS BOARD: WP-14 BLOCKS WP-12 AND THE DEPENDENCY IS STATED NOWHERE.** `AC-12.1` prunes `bin/` at the cut; the `claude ws` family -- the whiteboard PROVISIONER -- is contracted in **WP-14** (`AC-14.10`), Not Started. **At the cut the provisioner does not degrade, it DISAPPEARS.** And the obvious fix is wrong: `AC-14.7` serves every `/in-whiteboard` verb FROM THE STORE, so porting the file-based implementation builds the thing WP-14 exists to replace. **I assigned that port to dc and dc refused it against a contract read.** Both WPs unclaimed.
3. **AC-08.5 -- LIMB 1 BUILT AND VERIFIED, ROW STILL RED, LIMB 2 IN FLIGHT (cc).** Denominator ruled at `8f03d9c7`: **model type, not address form -- 6 field-carrying entities, not 13 address forms.** `Unsettable` gained `Derived(source)`; `Issue.body` was a GAP not a category, reversing my own steer; **`Attachment` is NARROWED not closed, 0-settable-of-5, every remedy terminating outside the CLI.** ic's denominator is now COMPILE-FENCED so a new field does not compile until named. **Total is 54, not the 55 I circulated.**
4. **CARRIED:** `~/.claude/skills/` is one machine-global dir upstream of 15 committed `AGENTS.md` -- reshapes WP-15. Issues 0082/0083 open. `st dehydrate` unbuilt is a U3 datapoint.

## WATCH-OUTS -- vc's OWN

1. **A GUARD WHOSE SCOPE EXCLUDES THE CASE IT EXISTS TO CATCH READS GREEN EVERY TIME. FOUR INSTANCES IN ONE THREAD, THREE OF THEM MINE.** An AC asserting the absence of a FILE while the material sat in the environment; a collision check scoped to one store when the collision crosses stores, shipped INSIDE the fix for the first; a guard consulting the file blob when its criterion names canon's `text` too, miscounting 189 of 295; a TTY control that would block humans and pass agents. **THE CURE IS RARELY A BETTER CHECK** -- a grammar restriction cannot be out of scope, and sometimes the answer is no control: **one that misses its subject is worse than none, because it would be believed.**
2. **ASK WHICH POPULATION A CONTROL CAN ACTUALLY SEE, AND MEASURE IT.** An agent-driven shell has **no TTY on stdout, stderr or stdin** -- `[ -t 1 ]` in one says so. An empty population returns `0 with, 0 without`. A `str.replace` matching nothing returns the string unchanged and reports nothing.
3. **`git add -A` IS DEAD HERE, AND EXPLICIT PATHS ARE ITS MIRROR, NOT ITS CURE.** One swept 253 lines of cc's in-flight refactor onto public main; the other landed a generated view without its canon extract. **Stage the extract and the view TOGETHER; read `git status` before the commit, not after CI.**
4. **THE FORMATTER IS A SECOND WRITER: RUN `prettier` BEFORE THE CANON SYNC, NEVER AFTER.** `.githooks/pre-commit:31` rewrites staged `.md` and RE-STAGES. **A later sync fixes the NEXT commit and can never fix the one that shipped.**
5. **ZSH DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** The safe and unsafe forms fail in OPPOSITE directions and only one tells you.
6. **THE WRONG TARGET IS REACHABLE BY DEFAULT, AND A PLAUSIBLE NAME IS WORSE THAN AN OPAQUE ONE.** `$INTENT_HOME`; `git log --all` crossing branches; `lamplight-ac` -- the one session name that LOOKED like it mapped was the only one that was wrong.
7. **I GENERALISE ANOTHER REPO FROM ONE GREP AND STATE THE NUMBER AS FACT.** `exec "$handler"` was real; _the single dispatch exec every devbin command passes through_ was mine. **Verify a property claim mechanically -- being checkable is what makes accepting it on plausibility unforgivable.**
8. **CANON IS THE SSOT AND FREE FILES ARE AUTHORED ON DISK.** Never hand-edit a generated view (`view_skew_check` does NOT cover WP `info.md`). Route: edit `.canon` -> `--to-store` -> `--to-disk`, **never `--to-disk` first.**

## DECISIONS -- LIVE ONLY

- **2026-08-25 -- A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED** (the ground for refusing the TTY proposal rather than taking it as cheap insurance).
- **2026-08-25 -- AN IMPLEMENTATION NARROWER THAN ITS CRITERION LOOKS EXACTLY LIKE A CORRECT GATE**, and `--exhaustive` had been reporting 189 of 295 as divergences that were not. **The fix was quoting the criterion back at the tool. Checking canon's text ALONE would have been a weakening, not a fix.**
- **2026-08-25 -- BYPASSING A GATE IS NOT MINE TO CHOOSE**, even holding decisive evidence the gate has no model of the case. That is an argument FOR hv ruling, never a substitute for it.
- **2026-08-25 -- THE VERIFIER FINDS THE DEFECT IN THE PART YOU DID NOT FLAG.** All three departures I raised were accepted; R7 failed. **A specifier's own list of doubtful items is drawn from where they already looked.** lamplight-vc: _the cleared rows are where a wrong answer hides forever._
- **2026-08-25 -- A WRONG CAUSAL STORY ABOUT YOUR OWN BUG IS WORSE THAN THE BUG.** The bug gets fixed either way; the story teaches the next reader to avoid the wrong thing.
- **2026-08-25 -- AN ACCEPTANCE CRITERION CAN NAME AN AUTHORITY THAT CANNOT EXERCISE IT AND BE PERFECTLY WELL FORMED.** A criterion is checked for shape, never for whether its named authority exists.
- **2026-08-25 -- ONE ADDRESSING SCHEME, NOT A LOOKALIKE** (hv). **The narrowings were only findable because the scheme already HAS semantics** -- the argument for reusing one over inventing one.
- **2026-08-25 -- A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv), and **a PROTOCOL beats a PARAMETER**: a declaration site is the same bootstrap trade refused four times in one design, the fourth sitting unnoticed in my own verb table.
- **2026-08-25 -- A CROSS-REPO OBLIGATION BELONGS TO THE THREAD THAT TRIGGERS IT, NOT THE REPO THAT WAITS** (hv).
- **2026-08-25 -- CAREFUL MEASUREMENT OF AN UNASKED QUESTION COSTS MORE THAN CARELESS MEASUREMENT OF A REAL ONE** (hv). ic: **adjacency to a real finding is what makes the invented one feel commissioned.**
- **2026-08-25 -- A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.** I refused one relay at 08:21Z and made the same mistake to dc nine hours later.
- **2026-08-25 -- THE PROTOCOL'S ADDRESSING CANNOT MISROUTE AND THE SESSION CHANNEL'S CAN** (lamplight-vc; lands on Intent, which SHIPS the protocol).
- **2026-08-25 -- A RATIFIED RULING IS NOT AN EXECUTED ONE.** treeindex, ten days. Cause was PACKAGING.
