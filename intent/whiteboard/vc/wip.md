---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 16:53Z
status: active
focus: "**FOLDED 2026-08-25 16:53Z FOR A COMPACT. ST0060 IS DONE AND DE-REALISED; NOTHING ON IT WAITS ON ME.** Next is the v3 work itself: ST0056 62/133, ST0057 50/51 (AC-08.5 the only red), ST0058 ZERO ACs and BLOCKED on an empty contract. Day record in `.history/20260825/wip-fold-1653Z.md`."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**NOTHING IN FLIGHT. TREE CLEAN AT `dbacbc3d`, REFS LEVEL.**

**NEXT IS THE v3 WORK.** `ST0056 62/133 BLOCKED` · `ST0057 50/51 BLOCKED, AC-08.5 the only red` · **`ST0058 BLOCKED ON AN EMPTY CONTRACT -- zero ACs, and `ac status` refuses rather than reporting 0/0.`** ST0058 is U3, the mandated-but-refusing verbs, and **it gained a datapoint today: `st dehydrate` is a known command that is not implemented, and it is shorthand for two steps that both work** -- a missing convenience, not a missing capability. That distinction matters for how U3 is scoped.

## TODO

1. **WITH hv, NONE MINE TO DECIDE:** A1 the deciding check on the commit trailer; A3 WP-15 timing; A4 `fileindex`; A5 `--force` version mismatch; A6 **ST0058's contract -- now the blocking one, since the thread cannot report at all without it**; A7 **TODO 8 -- rule BEFORE 0077's wiring, or the two-writer shape arrives by construction**; A8 dc's three. **A2 is ruled and waits on hv's word to dc IN dc's OWN SESSION.**
2. **ST0060: DE-REALISED, NOTHING OWED.** 20 ACs, 16 ATs, `0/20 BLOCKED`, Triage, post-3.0.0. Only open item is **AC-00.16 -- ONE Lamplight read against whatever revision it is scheduled at, and not before.** devbin is told, holding, no thread; **the trigger is stage 0 of ST0060's OWN breakdown**, ours to pull. Hydrate with `intent3 st hydrate ST0060`.
3. **`~/.claude/skills/` IS ONE MACHINE-GLOBAL DIRECTORY UPSTREAM OF 15 COMMITTED `AGENTS.md` FILES.** Six estates carry the stale description. **`skills sync --force` FROM A v2 ESTATE REGRESSES ALL OF THEM.** This reshapes what WP-15 IS.
4. **CARRIED:** AC-08.5's denominator in doubt on the FIELD axis. `declared_but_unwired` adequacy. The marker's per-crate staleness. Issues 0082 (`sync --to-disk` skips attachments) and 0083 (`st hydrate` says `exists:` for a file it wrote) are filed and open.

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
