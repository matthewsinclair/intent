---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 15:50Z
status: active
focus: "**FOLDED 2026-08-25 15:50Z FOR A COMPACT. ONE THING IS BLOCKED AND IT IS hv's: de-realising ST0060 trips the canon-currency guard, and 54 threads are ALREADY in the state it refuses.** ST0060 is specced -- 20 ACs, three Lamplight verifications, `intent://` addressing, `{key_id, value}` entries, devbin `--ivault` contract. Day record in `.history/20260825/wip-fold-1550Z.md`."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**ST0060 SPECCED AT `a3996367`, 20 ACs, TREE CLEAN AT `27ae15f3`. NOTHING IN FLIGHT.**

**BLOCKED, AND IT IS hv's CALL: the de-realise hv asked for trips `canon_commit_check`.** Removing `STEELTHREAD:ST0060` from `.intentfiles` + `organize --apply` deletes the three files, and the guard then reads _canon names bytes the commit does not contain_ -- because canon names `design.md` with a sha and the commit deletes it. **`intent st dehydrate` is the shorthand for those two steps and is not implemented, so it would hit the same wall.** I reverted to a clean tree rather than reach for `--no-verify`: **bypassing a gate is not mine to choose.** Evidence for the decision: **54 threads ALREADY carry canon-named attachment bytes with no directory on disk** (ST0034 has four), and they pass only because narrowed mode never examines an unchanged thread. **The guard is right about a realised thread and has no model of a dehydrating one.**

## TODO

1. **WITH hv, NONE MINE:** the ST0060 de-realise above; **the TTY decision** (Lamplight asks `vault get` REFUSE a TTY write unless forced -- hv ruled B3 as no WARN, and refuse is a different question); A1 the deciding check; A3 WP-15 timing; A4 `fileindex`; A5 `--force` version mismatch; A6 ST0058's contract; A7 **TODO 8 -- rule BEFORE 0077's wiring**; A8 dc's three. **A2 is ruled and waits on hv's word to dc IN dc's OWN SESSION.**
2. **ST0060's ONLY OPEN ITEM WITH LAMPLIGHT: AC-00.16, deliberately unsatisfied.** Three reads, and the design changed after each. **A verification is of a REVISION, not of a thread.** One read against whatever revision it is scheduled at, and not before. AC-00.17 (hv ratifies the R5 deviation + S1 + S2) is a separate act.
3. **devbin IS TOLD AND HOLDING.** No thread, no spike. **The trigger is stage 0 of ST0060's OWN breakdown** -- ours to pull, not theirs to watch. Contract is exactly two things: the `intent://` prefix, and `vault run`'s exec semantics.
4. **`~/.claude/skills/` IS ONE MACHINE-GLOBAL DIRECTORY UPSTREAM OF 15 COMMITTED `AGENTS.md` FILES.** Six estates carry the stale description. **`skills sync --force` FROM A v2 ESTATE REGRESSES ALL OF THEM.** This reshapes what WP-15 IS.
5. **CARRIED FROM BEFORE TODAY:** AC-08.5's denominator in doubt on the FIELD axis. ST0058 is WIP with ZERO ACs. `declared_but_unwired` adequacy. The marker's per-crate staleness.

## WATCH-OUTS -- vc's OWN

1. **AN INSTRUMENT THAT CANNOT FAIL ON ITS SUBJECT READS EXACTLY LIKE ONE THAT CAN.** Five instances today, three of them mine and one inside the fix for another: an AC asserting the absence of a FILE while the material sat in the environment; a collision check scoped to one store when the collision crosses stores; an empty population returning `0 with, 0 without`; a `str.replace` matching nothing and reporting nothing; a `--ivault` that would honour handlers and silently skip catalogue rows. **Ask what the assertion CANNOT reach, then positive-control it.**
2. **`git add -A` IS DEAD HERE, AND EXPLICIT PATHS ARE ITS MIRROR, NOT ITS CURE.** One swept 253 lines of cc's in-flight refactor onto public main; the other landed a generated view without its canon extract. **Stage the extract and the view TOGETHER and read `git status` before the commit, not after CI.**
3. **THE FORMATTER IS A SECOND WRITER: RUN `prettier` BEFORE THE SYNC, NEVER AFTER.** `.githooks/pre-commit:31` rewrites staged `.md` and RE-STAGES, between my canon sync and the gate. **A later sync fixes the NEXT commit and can never fix the one that shipped.**
4. **ZSH DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** It told me a built verb was unbuilt, a real hazard was not real, and 8 carriers were 0. **The safe and unsafe forms fail in OPPOSITE directions and only one tells you.**
5. **THE WRONG TARGET IS REACHABLE BY DEFAULT, AND A PLAUSIBLE NAME IS WORSE THAN AN OPAQUE ONE.** `$INTENT_HOME`, `git log --all` crossing branches, and `lamplight-ac` -- the one session name that LOOKED like it mapped to a moniker was the only one that was wrong. **The authoritative join was on a board I had open.**
6. **I GENERALISE ANOTHER REPO FROM ONE GREP AND STATE THE NUMBER AS FACT.** `exec "$handler"` was real; _the single dispatch exec every devbin command passes through_ was mine. **The grep was right and the generalisation was not.** Verify a property claim mechanically -- being checkable is what makes accepting it on plausibility unforgivable.
7. **NEVER HAND-EDIT A GENERATED VIEW; FREE FILES ARE AUTHORED ON DISK.** `view_skew_check` does NOT cover WP `info.md`. Attachments flow disk -> canon (issue 0082), so canon-authored free files never reach disk.

## DECISIONS -- LIVE ONLY

- **2026-08-25 15:50Z -- BYPASSING A GATE IS NOT MINE TO CHOOSE**, even holding decisive evidence that the gate has no model of the case. 54 threads already carry the state `canon_commit_check` refused, and that is an argument FOR hv ruling, never a substitute for it.
- **2026-08-25 15:35Z -- ONE ADDRESSING SCHEME, NOT A LOOKALIKE.** `intent://<project>/profiles/<p>/secrets/<n>`: hv's call, and the evidence is Highlander rather than taste -- `address.rs` already makes the project the AUTHORITY. **The two narrowings (no empty authority, no `?format=`) were only findable because the scheme already HAS semantics.**
- **2026-08-25 15:35Z -- A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED.** hv: _the execution model needs inverting._ **And a PROTOCOL beats a PARAMETER** -- a `--vault <thing>` form needs a declaration site: **the same bootstrap trade refused for the declared override, the shelled `age` binary, AND `run --ref K=<ref>`. Four refusals of one trade, and the fourth was sitting in my own verb table unnoticed.**
- **2026-08-25 15:35Z -- A CROSS-REPO OBLIGATION BELONGS TO THE THREAD THAT TRIGGERS IT, NOT THE REPO THAT WAITS** (hv's).
- **2026-08-25 13:45Z -- THE VERIFIER FOUND THE DEFECT IN THE PART I DID NOT FLAG.** All three departures I raised were accepted; R7 failed. **A specifier's own list of doubtful items is the worst possible sample of where they were wrong**, being drawn from where they already looked. lamplight-vc: _the cleared rows are where a wrong answer hides forever._
- **2026-08-25 13:54Z -- A WRONG CAUSAL STORY ABOUT YOUR OWN BUG IS WORSE THAN THE BUG.** The bug gets fixed either way; the story teaches the next reader to avoid the wrong thing. **Kept on the record rather than quietly deleted.**
- **2026-08-25 13:40Z -- AN ACCEPTANCE CRITERION CAN NAME AN AUTHORITY THAT CANNOT EXERCISE IT AND BE PERFECTLY WELL FORMED.** Unsatisfiable by construction, and **a criterion is checked for shape, never for whether its named authority exists.** Surfaced from a ROUTING question, not from a review of the contract.
- **2026-08-25 13:45Z -- THE PROTOCOL'S ADDRESSING CANNOT MISROUTE AND THE SESSION CHANNEL'S CAN** (lamplight-vc's; lands on Intent, which SHIPS the protocol). **An argument for the durable surface that nobody had to make in the abstract.**
- **2026-08-25 13:03Z -- CAREFUL MEASUREMENT OF AN UNASKED QUESTION COSTS MORE THAN CARELESS MEASUREMENT OF A REAL ONE** (hv's). ic's mechanism: **adjacency to a real finding is what makes the invented one feel commissioned.**
- **2026-08-25 13:03Z -- A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.** I refused one relay at 08:21Z and made the same mistake to dc nine hours later. **dc refused.**
- **2026-08-25 13:03Z -- A GUARD'S REMEDY LINE INHERITS THE GUARD'S AUTHORITY WITHOUT INHERITING ITS CHECKS**, and dc's stronger form: **two rostered guards, one instructing a node to do what the other exists to prevent -- a property of the ROSTER neither guard can see.**
- **2026-08-25 13:03Z -- A RATIFIED RULING IS NOT AN EXECUTED ONE.** treeindex, ten days. Cause was PACKAGING.
