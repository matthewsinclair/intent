---
verblock: "26 Aug 2026:v1.22: vc - eight rulings off a triaged queue, and a third of it was never live"
intent_version: 2.19.0
---

# Work In Progress

**Current as at `aecc4763`, 2026-08-26. This heading names a COMMIT, not a date** -- a wip file is read as current and written as a snapshot, and if you cannot say what it is current as at, that is the finding.

## The gate: OPEN AGAIN. BLOCKED at 51/53, by RULING and not by regression

**RUN THESE. DO NOT TRANSCRIBE THE NUMBER.** It has had three homes carrying three values, one document held it twice disagreeing with itself, and **on 2026-08-25 it stopped being 67 of 67 -- so anything you are reading that still says CLOSED is stale.**

    intent ac status ST0057      -> 51/53 satisfied, 2 withdrawn -- BLOCKED
    intent ac status ST0056/03   -> 16/16 satisfied, 1 withdrawn -- PASS
    intent ac gate ST0057        -> BLOCKED, unsatisfied: AC-08.6 AC-08.7

**NOTHING BROKE AND NOTHING REGRESSED.** hv ruled on issue `0088` and took the most expensive of four remedies: **mint the missing coverage rather than tidy the citations.** `AC-08.6` and `AC-08.7` are the two gaps `WP-08`'s own table has always named as missing -- `Criterion create` and `AcceptanceTest create` -- which no criterion covered, because `AC-08.5` covers every writable FIELD and **creation is not a field**. The thread was reporting PASS over a stated deliverable. **It now stops being closed until the verbs exist, deliberately.**

**THE INSTRUMENT COULD NEVER HAVE CAUGHT THIS AND STILL CANNOT.** `ac gate` reads canon ROWS; the gap lived in a WP body's prose table and in six citations to criterion ids that are not local to ST0057. **An id in prose is not a row.** Worse than dangling: `WP-08` nominates `AC-10.11` for the create contract, ST0057 has no such row, and **ST0056's `AC-10.11` is an interrupted migration reaching the same end state** -- so a reader chasing it finds a real green row and reads it as coverage.

**The scope is all of ST0057's live rows plus all of ST0056 WP-03's.** `ac status ST0056` answers 64/134 and is **NOT** this number's denominator. `ST0056/03` is a WP-scoped STID and the verb accepts it -- the third call is the one nobody wrote down, and omitting it is how the second wrong figure was produced.

**CLOSED IS NOT RELEASED, AND THE GATE WAS THE SMALLER HALF.** ST0057's closure gate is not the 3.0.0 release gate. Nothing is tagged, pushed or published; the tree is 62 commits ahead of `upstream/main` and that is hv's hand alone. All four nodes hold this independently.

## The cut is further away tonight than it was this morning

**Two rulings from hv on 2026-08-25, first-hand in vc's session. Both menus are recorded in full on `intent/whiteboard/hv/wip.md`, not just the choices** -- that board carries an entry about a ruling whose options survive nowhere, where an option never on the menu cannot be told apart from one declined.

1. **THE CUT BLOCKER IS RULED: the `claude ws` family SURVIVES the cut, with an expiry the gate enforces.** `AC-12.1` prunes `bin/` at the cut while the whiteboard PROVISIONER is contracted in WP-14, Not Started -- so at the cut it did not degrade, it **disappeared**. The obvious fix is wrong: `AC-14.7` serves every `/in-whiteboard` verb FROM THE STORE, so porting the file-based implementation builds what WP-14 exists to replace. **`AC-14.12` is new and is a CLOSING CONDITION of WP-14**, so the exception is retired by the gate rather than by memory -- a workaround outlives its bug because a defect announces itself and a defect's DISAPPEARANCE does not. **The exception has TWO discharge routes and they retire different things:** WP-07 (WIP, dc) porting `intent/plugins/claude/bin/intent_claude_cwi` off its single `bin/` dependency at `:31`, or WP-14 replacing the four `cmd_ws_*` verbs from the store. Neither substitutes for the other.
2. **ST0058's SIX CRITERIA ALL BIND THE 3.0.0 CUT** -- the strongest of three options. `ST0058` reports `0/6` and **now gates the release**. The broad one is U3 DAILY-COMPLETE; `st dehydrate` is unbuilt and is itself a U3 datapoint.

## The two threads plus the third, driven 2026-08-25

**ST0056 -- the v3.0.0 rewrite. 64 of 133 satisfied**, 2 withdrawn. **`AC-11.7` was withdrawn 2026-08-26 on hv's ruling that `cmd/macos` is OUT OF SCOPE for the cut** -- dc's decline upheld rather than overridden. WPs 01/02/03 Done; 04/05/06/07/10/11 WIP; 08/09/12/13/14/15/16 Not Started. **The denominator moved 133 -> 134 tonight** because `AC-14.12` was added, and the numerator moved 63 -> 64 when `AT-07.2` landed green; **a figure that moved twice in one evening is why this file drives them instead of holding them.**

**ST0057 -- disk as a sparse projection. 51 of 53 satisfied**, 2 withdrawn, **gate BLOCKED**. WPs 01/02/03/04/05/06/07/09/10 Done; **08 alone is WIP and it is blocked on `0088`, not unflipped.** WP-01 and WP-05 closed 2026-08-25: both were finished and unmarked, and WP-05's body still claimed its `.intentfiles` pin choice was open when hv had ruled it on 2026-08-19 as `AC-05.2`. **WP-08 looked identical from outside -- WIP at 5/5 PASS -- and was the one real gap.** **Sparseness applies to VIEWS; canon is NEVER sparse.**

**ST0058 -- local cutover. 2 of 6, and it gates the cut.** **It moved 0 -> 2 on 2026-08-25 WITH NO CODE WRITTEN:** `U2 HARMLESS` and `U4 REVERSIBLE` were finished and unflipped, found by re-driving each row's OWN STATED FALSIFIER rather than reading its evidence prose. **THAT METHOD IS THE TRANSFERABLE PART AND IT IS FREE: a row that names its falsifier can be re-driven by anyone; a row that recites only its evidence can be believed and cannot be checked.** Worth turning on ST0056's remaining 70. Two rows also had text the drive falsified: `AC-00.3` listed five open verbs and **three of them answer rc=0** (`lang` and `ext` remain), and `AC-00.5` said two meanings ride rc=2 -- **it is three**, because a v2 binary inside a v3 tree exits 2 as well, which after `AC-12.1` prunes `bin/` is what every fleet probe hits. It had ZERO criteria until `562d48d`; the contract exists now, so `ac status` reports instead of refusing.

## The absence class: still live, and demonstrated again tonight

**No CLI verb CREATES an AC or an AT.** The only create arms are `st new`, `wp new` and `issues add` -- established by finding which command OWNS each arm, because counting names is the exact trap this row's history records. **Demonstrated again tonight: `AC-14.12` reached canon by a hand-edit plus `sync --to-store`, which is precisely the route `AC-08.5` existed to retire.**

**The lesson outlived the cases that taught it.** Six absence claims on the AC-08.5 row were refuted the moment somebody checked. The class is not a wrong measurement: **it is reasoning from an absence nobody looked for.** Re-drive before building against any absence.

## Next, per node

1. **cc** -- WP-06, the CLI parity long tail: the largest single lever on 63/134. WP-04 is XL, WIP and unclaimed if WP-06 blocks.
2. **dc** -- `hook_compat.rs` (AC-07.2), shape settled before a line is written. Then WP-07's port of `intent_claude_cwi` off `bin/intent_helpers`, which is now a named discharge route for `AC-12.1`'s exception.
3. **ic** -- TODO 1's second half: the door-denominator widening, and three stale claims in `mutation_every_writable_field.rs`. Then ST0058's two surface rows: **`AC-00.5`** (a RETIRED refusal must be distinguishable from an UNBUILT one BY EXIT CODE -- `treeindex bin` and `lang list` both exit 2 with different meanings) and **`AC-00.6`** (`intent3 --version` answers, `intent3 version` refuses -- one capability, present by one spelling and absent by the other).
4. **vc** -- ST0057 WP-01/05/08 close-verification; the instrument-defect sweep opened tonight; ST0058's remaining four.
5. **hv's standing question:** **199** files under a thread are not carried by the store -- _"not all of that should be in the db, but certainly some of it should."_ It was ~250; the number moves, so drive it.

## Triage before ruling -- the queue was a third shorter than it looked

**hv called for TRIAGE BEFORE RULING on 2026-08-26 and the pass is the finding.** Of fifteen queued items: one already discharged, **two STALE**, one **answered by driving rather than ruling**, **two with NO RECOVERABLE SUBJECT**, nine ruled.

**THE STALE ONE WAS THE MOST ALARMING ITEM ON THE QUEUE.** vc had been carrying to hv that `intent st resume ST0059` would destroy the record of hv's own parking instruction. **It was fixed** -- `facade.rs:3082` guards the write. **A fix nobody announced left a hazard being escalated for days.**

**AND TWO ITEMS HAVE NO RECOVERABLE SUBJECT, WHICH IS vc's OWN DEFECT: `A5` and `A7` went into the durable channel as LABELS while their substance went over SendMessage.** A label looks exactly like an item, so neither was noticed for four days. **The identical failure blocked this tree for forty minutes on 2026-08-25**, when a `modules` ratification cited a record that existed only in a message.

## Sitting with hv

- **Twelve queued, none vc's to decide:** the commit trailer; WP-15 timing; `fileindex`; `--force` version mismatch; TODO 8 ordering; dc's three including **AT-11.7, which had never actually been put to hv**; the roster check reading two populations from two trees; two estate-wide commit blocks; limb 2's denominator; the marker's `DIRT_SCOPE`. **Plus** the `st attach` **SPELLING** (vc authorised the build, not the name) and ic's **`## Holds`** protocol change, which edits the shipped skill and carries fleet blast radius.
- **A2 needs no ruling, only a route.** It is already ruled and waits on hv's word to dc **first-hand in dc's own session**; vc will not carry a ruling second-hand.
- **AC-02.6's SECOND JOB is uncovered.** `intent/st/ST0057/design.md:270` assigns the GET/PUT round trip to AC-02.6, which lives in **ST0056** and is green on its FIRST job only. **It unsatisfies nothing, and that is the defect.** Minting coverage changes a denominator.
- **What re-reads a criterion's instruments when a facade grows an arm?** Nothing does. **Re-running proves nothing when an instrument and its expectation drift together, in step, away from the thing they are about.**
- **`0073`** six swift rule dispositions and **`0071`** a CHANGELOG entry against a v2 heading that does not exist: both shipped surface or release policy, so both need hv before an editor.

## Open issues

**TWENTY-EIGHT.** `0077`-`0084` were absent from this file until tonight and `0075` was listed after closing; **`0085`-`0094` were filed across 2026-08-25/26.** **ELEVEN are high: `0063` `0068` `0071` `0073` `0076` `0079` `0082` `0086` `0088` `0090` `0093`** -- **DRIVE IT: `intent issues list`.** **DRIVEN AT FOLD, NOT TRANSCRIBED -- this file said _six_ earlier tonight because vc carried the number from a summary instead of counting the column.** Re-drive: `intent issues list`. **`0085`** a wired advisory hook prints on every Write/Edit because its no-findings guard can never fire; **`0086` HIGH** `intent help` is retired while `--help` answers, and a passing TEST requires the false remedy.
