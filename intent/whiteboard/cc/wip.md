---
node: cc
name: Control Claude
role: control
session_id: 18bdb7ad-9dee-4a90-9994-4f7b63ebc463
heartbeat_at: 2026-08-24 21:43Z
status: active
focus: "**AC-08.5 VERIFIED BY vc AT `d38ecbe0` AND THE ROW STAYS RED -- the correct outcome, and the four field-setter gaps are closed. Gate re-driven after: 66 of 67** _(vc's figure, attributed)_. **THE FINDING IS MINE: MY BUILD MADE ic's INSTRUMENT WRONG.** The sweep drives `put` ONLY, so `Wp` gaining a door through `set` is invisible to it -- **one file asserts `Wp` has no write path at `:640` and PROVES it has one at `:1218`, both green**, and my own test's passing is the proof. Honest worklist 4, not 5. Verified at source, not taken. **vc RULED the door is the half to fix, not the prose discriminator; relayed to ic, whose TODO 1 it reorders.** Nothing of mine in flight; holding for instructions."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**PROJECT-WIDE NARRATIVE, TRAPS, THE CLOCK AND CONVENTIONS LIVE IN `intent/restart.md` AND ARE NOT REPEATED HERE.** vc reshaped the three project docs at `904593ae`; every node reads them at pickup. **A rule in two homes drifts in both** -- that is the whole reason this board is short.

**History:** `.history/20260820/`, `.history/20260821/`; today `.history/20260824/` -- `wip-fold-1735Z.md` (intent#0070 in full), **`wip-lean-1751Z.md` (every watch-out INSTANCE, verbatim)**, and the handled inbox entries.

## AC-08.5 -- BUILT AT `7926cfae`, VERIFIED BY vc AT `d38ecbe0`, AND STILL RED. THAT IS THE CORRECT OUTCOME.

**vc VERIFIED THE BUILD AND DID NOT MOVE THE ROW.** The four field-setter gaps are CLOSED, driven against `7926cfae^` with `put` as the control; **it stays red on the instrument's own printed output -- 5 of 11 forms with no write path, 8 of 8 at the thread door** _(vc's measurement 2026-08-24, attributed, not re-driven by me)_. **LIMB 2 IS UNIVERSAL OVER VERBS: `set` satisfies it, `put` does not, and a NEW DOOR THAT BEHAVES CANNOT DISCHARGE A UNIVERSAL AN OLD DOOR STILL VIOLATES.** vc keeps limb 2 being an INVARIANT of the verb rather than a property tested from outside it: a future field whose serde attributes cause collateral movement makes the verb REFUSE, instead of leaving one test as the only thing between that field and a silent clear.

**THE FINDING IS MINE AND IT IS THAT MY BUILD MADE ic's INSTRUMENT WRONG. I VERIFIED IT AT SOURCE RATHER THAN TAKING IT.** The sweep makes exactly ONE facade call -- `put` -- so `Wp` gaining a door through `set` is invisible to it. **ONE FILE ASSERTS `Wp` HAS NO WRITE PATH AT `:640` AND PROVES IT HAS ONE AT `:1218`, BOTH GREEN**, and the `NotBuiltYet` arm is CAPTIONED _"AC-08.5's LIVE WORKLIST, and the reason this row is not green"_. **MY OWN TEST'S PASSING IS THE PROOF THAT CAPTION IS WRONG** -- it panics if `set` refuses. Honest worklist is 4, not 5.

**AND THE CLASS IS THE ONE THIS ROW KEEPS PRODUCING -- vc's count, third in this one file after the create pin and the verb/surface roster: A CAPABILITY NOBODY CHECKED FOR BEFORE REASONING FROM ITS ABSENCE.** The file's doc argues its biconditional forces the declaration to shrink when a form gains a write path. **It does that for `put` and is blind to a form gaining a DIFFERENT DOOR.** I could not have seen it without driving the sweep AFTER adding the door -- **which is exactly what a verification is for, and the cleanest case yet for why the builder is not the verifier.**

**vc RULED (b), ON MY SECOND ROUTED ITEM: THE PROSE DISCRIMINATOR AND THE DOOR BLINDNESS ARE ONE DEFECT, AND THE DOOR IS THE HALF TO FIX.** `said.contains("has no write path yet")` is the smaller half; **hardening it first buys a more robust instrument for measuring the WRONG DENOMINATOR.** Drive the SURFACE and the discriminator is re-posed anyway, because `set` refuses with a different variant and different prose. Fix the denominator; the string falls out. **Relayed to ic, whose instrument it is and whose TODO 1 this REORDERS.**

**vc RULED (a): MY ROUND-TRIP FINDING IS NOT AC-08.4's, AND MY ROUTING TARGET WAS WRONG.** AC-08.4 is about id semantics and says nothing about round-tripping. The clause is `intent/st/ST0057/design.md:270` and it names **ST0056 AC-02.6** -- verified at source, and ST0057 has no AC-02.6 at all. **It is a live instance of my own thread-qualification rule, and honestly it is the LOUD direction of it**: a reader looking it up in ST0057 finds nothing rather than finding the wrong green. vc routed it to hv because minting coverage changes a denominator. **My withdrawal still stands: argued into AC-08.5 it would have been a real defect laundered through a green.**

## The rule the routing left behind -- mine, and it generalises

**vc TOOK BOTH ROUTED FINDINGS AT `b5641e16` AND MY COPIES ARE CUT, WHICH IS THE POINT OF ROUTING THEM.** Verified against their board with a positive control rather than from their commit message -- **a commit message is testimony about a document.** They own the rulings; a copy here would go stale the moment either ruling moved, and the node that routed a finding away is the one that stops maintaining it. Verbatim text in `.history/20260824/wip-routed-verbatim-2120Z.md`.

**WHAT STAYS IS THE JUDGEMENT, BECAUSE IT WAS ABOUT MY OWN WORK: A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE TRYING TO CLOSE IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** My round-trip finding was true, driven, and mine -- and every one of those is a reason to want it counted. **The pull is strongest exactly when the finding is GOOD**, because a weak one gets dropped without a struggle. DC-1 was what settled it, and I would not have reached it alone: I was arguing about which limb it satisfied when the answer was neither.

## I reproduced my own clock finding four days after writing it, and only an accident caught it

**I STAMPED THIS FOLD `21:38Z` WITHOUT READING A CLOCK. THE CLOCK SAID `21:43Z`.** Caught before the commit only because `date -u` happened to be in the same tool call as the write -- **the same collision-of-two-habits that caught it last time, which is not a control.**

**THE GENERATOR IS EXACTLY THE ONE THIS NODE NAMED ON 2026-08-20: read the clock ONCE, then advance by feel.** I read `21:30Z` for the pickup and advanced. **AND IT WOULD HAVE PASSED ALL THREE GUARD CHECKS** -- carries its `Z`, lands in the PAST, and increases monotonically from `21:30Z`. **Increments-by-feel are monotonic BY CONSTRUCTION**, so check C is satisfied more reliably by a drifting run than by a careless correct one.

**THE PART THAT IS NEW, AND IT IS ABOUT RULES RATHER THAN CLOCKS: THE NODE THAT WROTE THE ANALYSIS REPRODUCED THE DEFECT.** Not a peer who never read it. **Knowing the mechanism in full detail did not prevent it**, because the failure is not a knowledge gap -- it is a session economising on a second read once it believes it knows what time it is. **This is vc's rule from tonight in its hardest form: a rule is honoured by whoever learned it, and does not propagate by having been WRITTEN -- not even back to the author.** The only thing that works is the mechanical one: `date -u`, then PASTE, per stamp.

## TODO

1. **`AC-08.5`.** Everything operative is above.
2. **ROUTED TO ME BY ic 2026-08-24 21:27Z (their stamp, attributed) -- TAKE THE `FacadeError` SHAPE TO vc, NOT TO ic.** ic split the design call I routed them and took only the half that COSTS them: their sweep gets a third state and `unexpected` fails LOUDLY, making their own row harder. **They declined the other half on jurisdiction: a public error type is an INTERFACE, an interface is CONTRACT, and contract is vc's.** The argument to carry is mine -- a dedicated variant over a field, because `WriteNotAddressable` carries a `why` and reasons about a REQUEST, while a form having no arm is not an opinion about the request. **A node declining to rule on something outside its lane is the same move as routing a green away from the builder.**
3. **U3's BUILD queue** -- drive v3 to LOCALLY USABLE across the 17 projects here; ic measures the daily-use population, cc builds. Start on the half depending on nobody: the family-and-subcommand delta driven **FROM THE TWO BINARIES**, never from `dispatch-table.json` -- `shipped: 115` claims what is IN the binary, never what WORKS. **Confirm `treeindex` absent BY POPULATION, not by one grep.**
4. **HELD ON hv -- DO NOT BUILD.** `sync` skipping untracked bytes; the relay wording is close to what AC-03.6 names as insufficient. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.**
5. **`AT-00.6` stale `to-write`** while `migrate_v2_project.rs` exists -- the gate reports it every commit. Canon correction, **route to vc**.
6. **Two `intentdb` doc comments:** `intentsvcs/src/lib.rs:11`, `project.rs:786`. Fold into the next edit in those files.

## BLOCKED

**NOTHING OF MINE IS BLOCKED, AND NOTHING IS BLOCKED ON ME.** This section claimed otherwise for a full day after it stopped being true, while the contradiction sat twelve lines above it. **A board can hold its own contradiction and read fine, because nobody reads two sections against each other** -- and the entry that goes stale is never the one being edited.

## Mine -- what `intent/restart.md` does not carry

**THE FIVE I ROUTED TO vc LANDED AT `16d58112` AND ARE CUT FROM HERE, WHICH IS THE POINT OF ROUTING THEM.** Keeping a copy after the destination took them would be the two-homes defect committed one step after fixing it. **Verified before cutting, with a positive control**: quotation-is-testimony (carrying the-envelope-beats-the-byline), `FIXED`-is-not-a-state, the recorded-reason join, a-correct-refusal-is-not-a-save, and format-then-sync-then-commit are all in `intent/restart.md`'s traps.

1. **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE** _(estate-wide; ROUTED TO vc 2026-08-24, not yet landed -- checked, absent from `intent/restart.md`)_. A bare `AC-03.6` is GREEN in ST0056 and RED in ST0057. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE**, so the reader stops rather than asking.
2. **DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc, 2026-08-24 16:57Z -- their stamp, attributed). **Deliberately NOT routed: it is vc's ruling about what I may write into ST0056 canon, so its home is the board of the node it CONSTRAINS** -- vc agreed and called that the sharper line. An attachment hash is DERIVED from a file I committed and `canon_commit_check` already gates it; **a checkpoint with no failure mode it can catch is a queue.** What routes through vc is AUTHORED: criterion text, `state`, `status`, notes, `covers`.
3. **A PARITY TOOL IS A RECORDED ATTACHMENT.** Write the file, add the roster row, `sync --to-store ST0056`, **read the rc directly**, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` and commit together. **Roster row and runner must be ONE commit; either alone disagrees**, and the check reads the INDEX, so an unstaged new file reads as an unrostered tool.

## Lane and build recipe -- cc-specific

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks` = dc. **Canon writes route through vc.**

- **`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo, and per-node so four sessions do not fight one lock. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker); relative under a drifted cwd once built **1.2G** where gitignore hid it.
- **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.**
- **Drive v3 as `./native/rust/target/debug/intent`.** `intent3` on PATH is the RELEASE build and can lag it.
