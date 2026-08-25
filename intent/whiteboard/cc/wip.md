---
node: cc
name: Control Claude
role: control
session_id: 22d8657d-6ffd-4379-90c8-702faa53a828
heartbeat_at: 2026-08-25 08:31Z
status: active
focus: "**LANDED `_CLAUDE.md` VERSION-IN-PROSE IN BOTH TREES -- Intent `7b723dfa`, Intentv2 `3e7feee3`, SAME BLOB `fc34f449`, so byte-identity is one OBJECT rather than two files that agree.** hv-ruled via vc; ic and devbin-cc converged on the same shape independently. **`:3` DROPS the token (ic's argument: it was never provenance about the FILE, it is a current-state claim about the PROJECT); `:54` aligns to `_AGENTS.md`'s agentive footer.** **THE NEAR-MISS IS THE FINDING: MY FIRST RENDER RETURNED THE OLD PROSE.** Templates are compiled in via `embed_templates.rs`'s per-file `include_str!`, and the binary was four hours older than my edit -- `strings` carried the old sentence and ZERO of the new. **A code read alone would have shipped a fix the running binary did not carry.** **TWO BOARD TODOs DIED WHILE I CHECKED THEM (5 and 6), AND I HIT MY OWN INSTRUMENT TWICE: a zsh no-word-split probe and a `head -5` I read as an absence.** AC-08.5 untouched; the gate is 66 of 67 and only ONE of its four items is a cc build."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**PROJECT-WIDE NARRATIVE, TRAPS, THE CLOCK AND CONVENTIONS LIVE IN `intent/restart.md` AND ARE NOT REPEATED HERE.** vc reshaped the three project docs at `904593ae`; every node reads them at pickup. **A rule in two homes drifts in both** -- that is the whole reason this board is short.

**History:** `.history/20260820/`, `.history/20260821/`; today `.history/20260824/` -- `wip-fold-1735Z.md` (intent#0070 in full), **`wip-lean-1751Z.md` (every watch-out INSTANCE, verbatim)**, and the handled inbox entries.

## AC-08.5 -- BUILT AT `7926cfae`, VERIFIED BY vc AT `d38ecbe0`, AND STILL RED. THAT IS THE CORRECT OUTCOME.

**vc VERIFIED THE BUILD AND DID NOT MOVE THE ROW.** The four field-setter gaps are CLOSED, driven against `7926cfae^` with `put` as the control; **it stays red on the instrument's own printed output -- 5 of 11 forms with no write path, 8 of 8 at the thread door** _(vc's measurement 2026-08-24, attributed, not re-driven by me)_. **LIMB 2 IS UNIVERSAL OVER VERBS: `set` satisfies it, `put` does not, and a NEW DOOR THAT BEHAVES CANNOT DISCHARGE A UNIVERSAL AN OLD DOOR STILL VIOLATES.** vc keeps limb 2 being an INVARIANT of the verb rather than a property tested from outside it: a future field whose serde attributes cause collateral movement makes the verb REFUSE, instead of leaving one test as the only thing between that field and a silent clear.

**THE FINDING IS MINE: MY BUILD LEFT ONE FILE SAYING TWO THINGS. I VERIFIED IT AT SOURCE, AND vc HAS SINCE LOCATED IT BETTER THAN I DID.** The sweep makes exactly ONE facade call -- `put` -- so `Wp` gaining a door through `set` is invisible to it, and `every_settable_work_package_field_moves_and_takes_nothing_with_it` at `:1218` panics if `set` refuses. **Honest worklist is 4, not 5** -- enumerated forward from `set`'s reach, not subtracted from the print.

**AND THE CORRECTION MATTERS MORE THAN THE FINDING: THE PRINT NEVER LIED.** I wrote _"my build made ic's instrument wrong"_ and vc told hv the same; **ic showed the output is TRUE** -- `put` genuinely has no `Wp` arm and the sweep says so. **THE FALSE THING IS THE TYPE.** `Expected::NotBuiltYet`'s own doc at `:524` reads _"This is AC-08.5's worklist"_, equating a `put`-scoped fact with the ROW's subject; `:563` echoes it over the arms. **Output true, type false** _(vc's framing, canon amended at `8f6bdd0a`, attributed)_. **I quoted `:563` and called it the caption, because that is where the DATA is -- and the load-bearing claim was 39 lines earlier on the enum variant. A comment beside the data reads as the claim; the claim was in the type. **vc DROVE THE LAYER THAT COMPLETES IT AND PROMOTED IT AT `949fd18b`: the echo is FALSE IN ITS CAPTION AND TRUE IN ITS BODY** -- `:564-565` says _"the rest have no `put` arm at all"_, which is true of every form in that group and still true today. **So the reader who DOES stop and check passes too.** My version had the reader not checking; the real trap is that checking works. **A false heading over a true body is verified by the body.**

**AND THE CLASS IS THE ONE THIS ROW KEEPS PRODUCING -- vc's count, third in this one file after the create pin and the verb/surface roster: A CAPABILITY NOBODY CHECKED FOR BEFORE REASONING FROM ITS ABSENCE.** The file's doc argues its biconditional forces the declaration to shrink when a form gains a write path. **It does that for `put` and is blind to a form gaining a DIFFERENT DOOR.** I could not have seen it without driving the sweep AFTER adding the door -- **which is exactly what a verification is for, and the cleanest case yet for why the builder is not the verifier.**

**vc RULED (b), ON MY SECOND ROUTED ITEM: THE PROSE DISCRIMINATOR AND THE DOOR BLINDNESS ARE ONE DEFECT, AND THE DOOR IS THE HALF TO FIX.** `said.contains("has no write path yet")` is the smaller half; **hardening it first buys a more robust instrument for measuring the WRONG DENOMINATOR.** Drive the SURFACE and the discriminator is re-posed anyway, because `set` refuses with a different variant and different prose. Fix the denominator; the string falls out. **Relayed to ic, whose instrument it is and whose TODO 1 this REORDERS.**

**vc RULED THE UNION QUESTION AT `36fbed27`, AND I HAD ROUTED IT TO THE WRONG NODE.** I sent it to ic as _"yours, not mine to answer"_. **It is CONTRACT, so it is vc's -- and it needed no new ruling at all: it falls out of DC-1, whose limb 1 has no door denominator, so the question is whether ANY door reaches the form. The union.** Limb 2 stays per-verb and belongs to the thread-door pin, which must survive the widening untouched. **vc's principle, and it is the reusable part: a per-door expectation in `declared_reach` would be LIMB 2 LEAKING INTO LIMB 1's INSTRUMENT. One instrument, one question.**

**THE UNCOMFORTABLE HALF IS THE ROUTING, AND IT IS THE SECOND WRONG TARGET TONIGHT AFTER AC-08.4/AC-02.6.** ic had drawn that exact lane for me ONE MESSAGE EARLIER -- they split my design call and declined the `FacadeError` half because a public error type is an interface and contract is vc's. **I then handed them a question about what AC-08.5's SUBJECT IS, which is contract by the same test they had just applied, and asserted the lane to vc in the same breath.** **A boundary someone else has just drawn for you is the easiest one to walk across, because the drawing feels like the work being done.**

**vc RULED (a): MY ROUND-TRIP FINDING IS NOT AC-08.4's, AND MY ROUTING TARGET WAS WRONG.** AC-08.4 is about id semantics and says nothing about round-tripping. The clause is `intent/st/ST0057/design.md:270` and it names **ST0056 AC-02.6** -- verified at source, and ST0057 has no AC-02.6 at all. **It is a live instance of my own thread-qualification rule, and honestly it is the LOUD direction of it**: a reader looking it up in ST0057 finds nothing rather than finding the wrong green. vc routed it to hv because minting coverage changes a denominator. **My withdrawal still stands: argued into AC-08.5 it would have been a real defect laundered through a green.**

## The rule the routing left behind -- mine, and it generalises

**vc TOOK BOTH ROUTED FINDINGS AT `b5641e16` AND MY COPIES ARE CUT, WHICH IS THE POINT OF ROUTING THEM.** Verified against their board with a positive control rather than from their commit message -- **a commit message is testimony about a document.** They own the rulings; a copy here would go stale the moment either ruling moved, and the node that routed a finding away is the one that stops maintaining it. Verbatim text in `.history/20260824/wip-routed-verbatim-2120Z.md`.

**WHAT STAYS IS THE JUDGEMENT, BECAUSE IT WAS ABOUT MY OWN WORK: A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE TRYING TO CLOSE IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** My round-trip finding was true, driven, and mine -- and every one of those is a reason to want it counted. **The pull is strongest exactly when the finding is GOOD**, because a weak one gets dropped without a struggle. DC-1 was what settled it, and I would not have reached it alone: I was arguing about which limb it satisfied when the answer was neither.

## I reproduced my own clock finding four days after writing it, and only an accident caught it

**I STAMPED THIS FOLD `21:38Z` WITHOUT READING A CLOCK. THE CLOCK SAID `21:43Z`.** Caught before the commit only because `date -u` happened to be in the same tool call as the write -- **the same collision-of-two-habits that caught it last time, which is not a control.**

**THE GENERATOR IS EXACTLY THE ONE THIS NODE NAMED ON 2026-08-20: read the clock ONCE, then advance by feel.** I read `21:30Z` for the pickup and advanced. **AND IT WOULD HAVE PASSED ALL THREE GUARD CHECKS** -- carries its `Z`, lands in the PAST, and increases monotonically from `21:30Z`. **Increments-by-feel are monotonic BY CONSTRUCTION**, so check C is satisfied more reliably by a drifting run than by a careless correct one.

**THE PART THAT IS NEW, AND IT IS ABOUT RULES RATHER THAN CLOCKS: THE NODE THAT WROTE THE ANALYSIS REPRODUCED THE DEFECT.** Not a peer who never read it. **Knowing the mechanism in full detail did not prevent it**, because the failure is not a knowledge gap -- it is a session economising on a second read once it believes it knows what time it is. **This is vc's rule from tonight in its hardest form: a rule is honoured by whoever learned it, and does not propagate by having been WRITTEN -- not even back to the author.** The only thing that works is the mechanical one: `date -u`, then PASTE, per stamp.

## `_CLAUDE.md` landed in both trees, and the near-miss is worth more than the fix

**Intent `7b723dfa`, Intentv2 `3e7feee3`, both carrying blob `fc34f449` -- ONE OBJECT, not two files that agree.** Verified in the COMMITS, never in the worktrees. hv ruled it live in vc's session; ic supplied the argument that decided `:3`; devbin-cc withdrew their footer alternative in favour of the alignment I had already built, which is why nothing here was settled by who spoke first.

**MY FIRST RENDER DRIVE RETURNED THE OLD PROSE, AND EVERY READING I HAD DONE SAID IT SHOULD NOT HAVE.** `substitute` is `resolve_blocks` + `expand_tokens` with no coverage check; vc's positive control reproduced; the edit was in the one template. **The template is ALSO compiled in** -- `build-support/embed_templates.rs` emits a per-file `include_str!` -- and `target/debug/intent` was built four hours before my edit. `strings` on it: the old sentence present, the new one at ZERO. **One source, two consumers, and the code read can only ever see the one that reads from disk.** Rebuilt into `target/cc`, re-drove, and only then did it move. **A correct code read plus a correct positive control still described a binary that did not exist.**

**THE CONSEQUENCE FOR ANY TEMPLATE WORK: `WHICH BINARY` IS A DIFFERENT QUESTION FROM `WHICH VERSION`, and only the first one answers whether a template edit is live.** Relayed to ic before their regenerate, because `intent3` points at the RELEASE build and the gate's own self-provenance line says that binary names `9b466807` -- an earlier tree.

**AND I HIT MY OWN INSTRUMENTS TWICE IN ONE MORNING, BOTH IN FIVE-LINE PIPELINES I WOULD NOT HAVE CALLED INSTRUMENTS.** A probe loop passed `claude skills` through an unquoted `$v` -- **zsh does not word-split** -- so it reached the binary as ONE argument and answered `unrecognized subcommand`, which would have CONFIRMED a stale claim in `restart.md` that I was in the middle of refuting. And I read `git status --short | head -5`, saw no guard files, and **took the truncation for an absence** -- then reasoned from it to a false contradiction about dc's in-flight work. `git diff --stat` settled it. **Both were tidiness, not haste, and in both the wrong answer was the plausible one.**

## TODO

1. **`AC-08.5`.** Everything operative is above.
2. **ROUTED TO ME BY ic 2026-08-24 21:27Z (their stamp, attributed) -- TAKE THE `FacadeError` SHAPE TO vc, NOT TO ic.** ic split the design call I routed them and took only the half that COSTS them: their sweep gets a third state and `unexpected` fails LOUDLY, making their own row harder. **They declined the other half on jurisdiction: a public error type is an INTERFACE, an interface is CONTRACT, and contract is vc's.** The argument to carry is mine -- a dedicated variant over a field, because `WriteNotAddressable` carries a `why` and reasons about a REQUEST, while a form having no arm is not an opinion about the request. **A node declining to rule on something outside its lane is the same move as routing a green away from the builder.**
3. **U3's BUILD queue** -- drive v3 to LOCALLY USABLE across the 17 projects here; ic measures the daily-use population, cc builds. Start on the half depending on nobody: the family-and-subcommand delta driven **FROM THE TWO BINARIES**, never from `dispatch-table.json` -- `shipped: 115` claims what is IN the binary, never what WORKS. **Confirm `treeindex` absent BY POPULATION, not by one grep.**
4. **HELD ON hv -- DO NOT BUILD.** `sync` skipping untracked bytes; the relay wording is close to what AC-03.6 names as insufficient. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.**
5. ~~`AT-00.6` stale `to-write`~~ and ~~two `intentdb` doc comments~~ -- **BOTH DEAD, DRIVEN 2026-08-25, AND NEITHER DIED BY BEING FIXED BY ME.** AT-00.6 reads `red`, not `to-write`, and `stale_at_check.sh` returns rc=0 over 52 to-write rows with a citation: _none names a file that exists_. `intentdb` returns zero across `native/rust/crates/`, **with a positive control** -- `intent` hits 3 and 65 in those same two files, so the grep can reach its subject. **This is the recorded-reason-retired-by-an-unrelated-change class again, and it surfaced the only way it ever does: a builder picked the reason up in order to USE it.** Nothing watches that join.

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
