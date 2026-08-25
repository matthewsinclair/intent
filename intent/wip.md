---
verblock: "25 Aug 2026:v1.19: vc - globalfold; the gate closed, the cut moved further away, and the issue list was eight short"
intent_version: 2.19.0
---

# Work In Progress

**Current as at `97cc09ca`, 2026-08-25. This heading names a COMMIT, not a date** -- a wip file is read as current and written as a snapshot, and if you cannot say what it is current as at, that is the finding.

## The gate: CLOSED, 67 of 67

**RUN THESE. DO NOT TRANSCRIBE THE NUMBER.** It has had three homes carrying three values, and one document held it twice disagreeing with itself.

    intent ac status ST0057      -> 51/51 satisfied, 2 withdrawn -- PASS
    intent ac status ST0056/03   -> 16/16 satisfied, 1 withdrawn -- PASS
    intent ac gate ST0057        -> PASS

**The scope is all of ST0057's live rows plus all of ST0056 WP-03's.** `ac status ST0056` answers 63/134 and is **NOT** this number's denominator. `ST0056/03` is a WP-scoped STID and the verb accepts it -- the third call is the one nobody wrote down, and omitting it is how the second wrong figure was produced.

**CLOSED IS NOT RELEASED, AND THE GATE WAS THE SMALLER HALF.** ST0057's closure gate is not the 3.0.0 release gate. Nothing is tagged, pushed or published; the tree is 62 commits ahead of `upstream/main` and that is hv's hand alone. All four nodes hold this independently.

## The cut is further away tonight than it was this morning

**Two rulings from hv on 2026-08-25, first-hand in vc's session. Both menus are recorded in full on `intent/whiteboard/hv/wip.md`, not just the choices** -- that board carries an entry about a ruling whose options survive nowhere, where an option never on the menu cannot be told apart from one declined.

1. **THE CUT BLOCKER IS RULED: the `claude ws` family SURVIVES the cut, with an expiry the gate enforces.** `AC-12.1` prunes `bin/` at the cut while the whiteboard PROVISIONER is contracted in WP-14, Not Started -- so at the cut it did not degrade, it **disappeared**. The obvious fix is wrong: `AC-14.7` serves every `/in-whiteboard` verb FROM THE STORE, so porting the file-based implementation builds what WP-14 exists to replace. **`AC-14.12` is new and is a CLOSING CONDITION of WP-14**, so the exception is retired by the gate rather than by memory -- a workaround outlives its bug because a defect announces itself and a defect's DISAPPEARANCE does not. **The exception has TWO discharge routes and they retire different things:** WP-07 (WIP, dc) porting `intent/plugins/claude/bin/intent_claude_cwi` off its single `bin/` dependency at `:31`, or WP-14 replacing the four `cmd_ws_*` verbs from the store. Neither substitutes for the other.
2. **ST0058's SIX CRITERIA ALL BIND THE 3.0.0 CUT** -- the strongest of three options. `ST0058` reports `0/6` and **now gates the release**. The broad one is U3 DAILY-COMPLETE; `st dehydrate` is unbuilt and is itself a U3 datapoint.

## The two threads plus the third, driven 2026-08-25

**ST0056 -- the v3.0.0 rewrite. 63 of 134 satisfied**, 1 withdrawn. WPs 01/02/03 Done; 04/05/06/07/10/11 WIP; 08/09/12/13/14/15/16 Not Started. **The denominator moved 133 -> 134 tonight** because `AC-14.12` was added; a moving denominator is why this file drives figures instead of holding them.

**ST0057 -- disk as a sparse projection. 51 of 51 satisfied**, 2 withdrawn, gate PASS. WPs 02/03/04/06/07/09/10 Done; **01/05/08 still WIP while the gate PASSes** -- that is WP-close verification, and it is vc's. **Sparseness applies to VIEWS; canon is NEVER sparse.**

**ST0058 -- local cutover. 0 of 6, and it now gates the cut.** It had ZERO criteria until `562d48d`; the contract exists now, so `ac status` reports instead of refusing.

## The absence class: still live, and demonstrated again tonight

**No CLI verb CREATES an AC or an AT.** The only create arms are `st new`, `wp new` and `issues add` -- established by finding which command OWNS each arm, because counting names is the exact trap this row's history records. **Demonstrated again tonight: `AC-14.12` reached canon by a hand-edit plus `sync --to-store`, which is precisely the route `AC-08.5` existed to retire.**

**The lesson outlived the cases that taught it.** Six absence claims on the AC-08.5 row were refuted the moment somebody checked. The class is not a wrong measurement: **it is reasoning from an absence nobody looked for.** Re-drive before building against any absence.

## Next, per node

1. **cc** -- WP-06, the CLI parity long tail: the largest single lever on 63/134. WP-04 is XL, WIP and unclaimed if WP-06 blocks.
2. **dc** -- `hook_compat.rs` (AC-07.2), shape settled before a line is written. Then WP-07's port of `intent_claude_cwi` off `bin/intent_helpers`, which is now a named discharge route for `AC-12.1`'s exception.
3. **ic** -- TODO 1's second half: the door-denominator widening, and three stale claims in `mutation_every_writable_field.rs`. Then ST0058's two surface rows: **`AC-00.5`** (a RETIRED refusal must be distinguishable from an UNBUILT one BY EXIT CODE -- `treeindex bin` and `lang list` both exit 2 with different meanings) and **`AC-00.6`** (`intent3 --version` answers, `intent3 version` refuses -- one capability, present by one spelling and absent by the other).
4. **vc** -- ST0057 WP-01/05/08 close-verification; the instrument-defect sweep opened tonight; ST0058's remaining four.
5. **hv's standing question:** **199** files under a thread are not carried by the store -- _"not all of that should be in the db, but certainly some of it should."_ It was ~250; the number moves, so drive it.

## Sitting with hv

- **Twelve queued, none vc's to decide:** the commit trailer; WP-15 timing; `fileindex`; `--force` version mismatch; TODO 8 ordering; dc's three including **AT-11.7, which had never actually been put to hv**; the roster check reading two populations from two trees; two estate-wide commit blocks; limb 2's denominator; the marker's `DIRT_SCOPE`. **Plus** the `st attach` **SPELLING** (vc authorised the build, not the name) and ic's **`## Holds`** protocol change, which edits the shipped skill and carries fleet blast radius.
- **A2 needs no ruling, only a route.** It is already ruled and waits on hv's word to dc **first-hand in dc's own session**; vc will not carry a ruling second-hand.
- **AC-02.6's SECOND JOB is uncovered.** `intent/st/ST0057/design.md:270` assigns the GET/PUT round trip to AC-02.6, which lives in **ST0056** and is green on its FIRST job only. **It unsatisfies nothing, and that is the defect.** Minting coverage changes a denominator.
- **What re-reads a criterion's instruments when a facade grows an arm?** Nothing does. **Re-running proves nothing when an instrument and its expectation drift together, in step, away from the thing they are about.**
- **`0073`** six swift rule dispositions and **`0071`** a CHANGELOG entry against a v2 heading that does not exist: both shipped surface or release policy, so both need hv before an editor.

## Open issues

**TWENTY, and this file said twelve until tonight** -- `0077` through `0084` were absent and `0075` was listed after closing. **SEVEN are high: `0063` `0068` `0071` `0073` `0076` `0079` `0082`** -- and vc wrote _six_ here first, having carried the figure from a summary rather than counting the column, which is this section's own rule failing in the sentence that states it. **Driven, not transcribed:** `intent issues list`.
