---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 12:13Z
status: paused
focus: "FOLDED 2026-08-31 12:12Z, pre-fold verbatim + sha-verified at .history/20260831/wip-fold-1212Z.md. NOTHING IN FLIGHT, nothing owed to any peer. On the bounce, in order: (1) the third endpoint state -- vc's WP-08 seam, release-blocking since hv put the menubar app in 3.0.1; (2) the write-path package's remaining items. Address-uniformity is mapped with ic and HELD for hv."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** Tree clean of me, no worktrees, no daemons of mine, ambient index blob-equal to HEAD on every path I landed.

## TODO

- **THE THIRD ENDPOINT STATE -- RELEASE-BLOCKING, AND THE FIRST THING ON THE BOUNCE.** hv put the menubar app into 3.0.1 (reversing `D3`: _"we need both"_), so ST0064 is now a 3.0.1 row. `AC-01.2` says the app's health predicate and the CLI's routing predicate are ONE predicate; `AC-01.6` says the app shows **LIVE / STALE / ABSENT**. **`daemon::route` has TWO variants (`daemon.rs:330`), so the shared predicate collapses STALE into ABSENT.** ic cannot invent the third in Swift without becoming the two-predicates-that-agree shape `AC-01.2` forbids, so it widens on my side. **vc has DELIBERATELY NOT RULED where it belongs** -- `Route` answers _where does this command run_, which is genuinely binary; the app asks _what do I show_, where STALE is not ABSENT. **Bring vc the shape, do not pick it.** `AC-08.3` governs either way: liveness is a completed round-trip under a bounded deadline, never a presence check, and both consumers must reach THAT rather than two implementations of it.
- **`daemon status` HAS NO MACHINE-READABLE FACE**, and the menubar app is a machine consumer. My own narrowing from this morning (`terminal` only) is now what stands between the app and the daemon -- correct when made, and the widening is the same `07ad9876` precedent I cited for narrowing it: **widen when a projection has a reader, and it now has one.**
- **THE WRITE-PATH PACKAGE, REMAINING** (hv, re-sequenced to me at `b1bf4cea`). `issues edit` is DONE. Left: **`close --note`** -- and it needs a ruling before code, because `Issue` has **no note field**, so it either appends to `body` (no schema change, but prose that is not the author's) or adds a field (canon schema change, reaching ic's published JSON Schema face). Then scoped issue sync, then **21 empty bodies** (hv's ruling says 20 -- the count moved, worth saying when I get there). `0122` is the FIRST DRIVE and `0118` the second; `0183` and `0179` are third and fourth, and their corrections come back OUT of the AT notes vc rehomed them into -- **the rehoming was a workaround for a missing verb and must not outlive it.**
- **ADDRESS UNIFORMITY -- MAPPED WITH ic, HELD FOR hv, NO CODE FROM EITHER OF US.** `promote()` is the declared command-line door and accepts a bare id OR an `intent://` address; 5 call sites use it and **37 use `thread_arg`**, which splits on `/` and reads `intent:` as the thread id. Two hv rulings are needed: does the nav SINGULAR grammar (`/thread/X`, AC-17.12) stay user-facing or go internal to the TUI view stack, and is the strict form the only one. **My measurement is the one that shapes it: NOTHING emits a canonical address -- zero across `st show`, `st list`, `issues show`, `search`, `doctor`, `--json`, `--format md`.** So strict refusal needs an EMIT partner in the same work, or the exact form is unlearnable and hv's `intent://threads/65` is what anyone reconstructs from memory. Split agreed: I take the ~40 call sites + a promote-then-narrow-then-RE-RENDER helper (the `ST0056/03` composite must survive a type that spells it `intent:///threads/ST0056/wp/3`) + a property test + a ratchet; ic takes explore, the address->view resolution, the grammar surface and the emit side.

## Watch-outs

**A DEFECT CAN HAVE A GUARD ASSERTING IT, AND THAT IS WHY IT SURVIVES.** `unwired()` asked the TABLE what the BINARY implements; `dispatch_ssot.rs` then asserted that same predicate from the other side. **The test had the RIGHT NAME, so every review looking for coverage of that defect found coverage of it.** A test whose name describes the defect and whose predicate cannot see it is worse than no test -- it consumes the search that would have found the hole. **When you fix a defect, grep for the test holding it in place before reading a red as your own breakage.**

**A MENTION IS NOT AN INSTANCE -- THREE TIMES IN ONE DAY, AND THE THIRD WAS IN THE INSTRUMENT BUILT TO PREVENT IT.** `llm guide` renders `UNWIRED_PHRASE` verbatim to explain exit 2, so a whole-output `contains` called a wired verb unwired. And `data-model.md` states the whiteboard's DEPARTURE inside the very section it departed from, so `section.contains(justified_by)` accepted a justification that meant the opposite. **Read the refusal as the PAIR it is; split a section at its exception prose; and drive the split to BOTH sides so it cannot be degenerate.**

**A DECLARATION TRUE OF EVERY CURRENT MEMBER IS NOT A CORRECT ONE, AND NOTHING SEPARATES THEM UNTIL SOMEONE ADDS A MEMBER.** Also: a TOKEN that spans the acceptable and the unacceptable case cannot discriminate -- `named` conflated _the check found the residue_ with _the migration named it_; `UNACCOUNTED` spans five verdicts; `UNAPPLIED` spanned a retired subject and an unapplied decision. **Two of those surfaced in one afternoon, in two tools, both found by driving to completion rather than by review.**

**FILLING IN MISSING DATA CAN TURN AN INSTRUMENT GREEN ON A CORPUS THAT FAILS.** `fleet_corpus_conservation.sh` reddened only on UNRUN or LOST, so recording the last three members would have flipped it to PASS while its own canary verdict said neither disjunct holds. **A counter that becomes able to fire exactly when someone finishes the work, and says PASS at that instant, is the most dangerous form of the class** -- driving the tool today cannot catch it; only driving it against the future it asks for.

**A STATED BLOCKER'S REASON EXPIRES WHILE ITS VERDICT STILL READS AS CURRENT.** `intentd` had no readable `SOURCE_COMMIT` because _"a const here is unreadable by anything, FOREVER"_ -- true when written, and the `forever` expired the moment the footer needed it. Same shape as five blockers that expired unannounced earlier in the day. **Drive the falsifier; never read the note.**

**PIN A DECLARATION TO ITS AUTHORISING DOCUMENT, BECAUSE THE AUTHOR OF A CHECK IS NOT A SAFE SOURCE FOR ITS OWN DENOMINATOR.** Issue `0183` as I filed it would have had the migrator declare 110 files out-of-model that D30 had moved INTO the model -- the denominator attack `sync.rs` warns about, by the person quoting the warning. **Care did not catch it; the citation did.**

**MY HEAD-PINNED PRIVATE INDEX DOES NOT CLOSE CONTENTION, IT INVERTS IT.** `read-tree HEAD` snapshots the WHOLE tree, so committing after HEAD moves reverts every path a peer touched in the window -- measured, the reverted set EQUALLED the peer's commit set exactly. **PROCEDURE: re-pin at COMMIT time until HEAD is unmoved across stage-and-commit, and post-verify `git show --name-only <commit>` EQUALS the intended set.** Checking my own staged list is structurally blind to a path I did not name. **And there is a window between the commit and `git restore --staged` where peers see my new files as staged deletions** (issue 0178) -- a peer caught it from outside while it was open.

**VERIFY THE CRATE WHOSE TESTS READ WHAT YOU CHANGED, NOT THE CRATE YOU CHANGED CODE IN.** A data change (the dispatch table) has a blast radius `cargo test -p <the-crate-I-edited>` cannot see. And **rebuild `intentd` after touching `intentsvcs`** or the daemon-backed arms fail as a stale sibling.

**A CONTROL CAN FAIL IN TWO PLACES AT ONCE AND STILL LOOK LIKE A CLEAN NEGATIVE.** My baize control moved nothing: `cp -R` left the `.CAPTURE` record behind AND the victim was not an attachment. **The tool said so in a line I had not read.** Later, my `UNAPPLIED` control could not fire because the unit I picked was in both sets. **Read the output, not the exit code.**

**A FORBIDDEN LIST LIVES IN AN INSTRUMENT AND DOES NOT TRAVEL TO THE NEXT ONE.** My driver excluded `daemon start`; I hand-wrote a shell probe for the same question and started a daemon. Temp `$HOME`, so the hard rule held. **`HOME=<temp> intent daemon stop` is the right undo** -- the store path is HOME-derived, so it CANNOT reach a peer's daemon.

**D37: OUR OWN THREAD AND WORK-PACKAGE NUMBERS DO NOT REACH A USER'S TERMINAL**, and `no_pm_state_in_output.rs` refuses the LITERAL, not just the render -- stricter than the ruling and right to be. **I made this mistake an hour after reading the comment that records it.**

**COMMITTING IN A SHARED CHECKOUT IS FOUR PROBLEMS.** CONTENTION (inverted, above); **COHERENCE -- not closeable, `git add` has no hunk scope, so a peer and you in ONE file is land-the-pair-or-wait**; REVERSION -- announce any disk->store sync, and **a sync realises whatever is in the STORE including peer writes nobody has committed**; WORKTREE VISIBILITY -- a guard's population is the worktree while every isolation device reads the index, and **a non-compiling file is a hazard for as long as it is on disk, tracked or not.** When the shared tree will not build, **verify in a detached worktree and control the result against pristine HEAD** -- that is how you say "not mine" with a measurement rather than a judgement.

**ATTACHMENTS ARE AUTHORED: `intent st attach` is the only writer, and NO sync direction rewrites one.** The canon guard's remedy (_sync canon FIRST_) is inert for an attachment and says so nowhere (issue 0184).

**zsh: `$var` does NOT word-split (use `${=var}`); an unmatched glob ABORTS the command; `... | head; echo $?` reports `head`'s status; and `||` after a pipeline binds to the LAST STAGE, so a `grep ... | head || echo "(none)"` fallback never fires.** All four cost a wrong answer rather than an error. **The tell is UNIFORMITY across a set that should have differed.**

**A REFUSAL CAN BE A RETRY** (`cannot lock ref 'HEAD'`, `index.lock`). **NEVER start `intentd` under the real `$HOME` while peers are live. NEVER invoke `intent fc`.** **rustfmt reformats what you just wrote** -- format, re-read, then patch. A UNIX socket path has a 143-byte limit and the session scratchpad exceeds it.

## Decisions

- (2026-08-31) **A build gap and an out-of-model declaration are DIFFERENT SENTENCES.** `NOT_CARRIED` closes by fiat and forever; `NOT_YET_BUILT` closes on its own when the work lands. **A gap that expires is worth more than a zero that never does** (vc). "still on disk" is load-bearing -- without it a build gap reads as data loss.
- (2026-08-31) **`refused` is AC-10.5's fourth verdict, and the denominator MOVES VISIBLY** -- a silent drop from four members to two is the denominator attack; a declared exclusion carrying its reason is the cure, and **the difference is entirely whether the reader is told.** A refusal with no reason exits 2.
- (2026-08-31) **DERIVED CENSUSES MULTIPLY FREELY; AUTHORITATIVE COPIES DO NOT** (vc). `legal_pairs` + three population lists are four CHECKS over one home, not four homes -- and reclassifying a row moved two counts in OPPOSITE directions, so only the per-pair census saw it. **The cure for a lossy projection is another projection, not another source of truth.**
- (2026-08-31) **Wiredness is DERIVED, or driven by a test that exercises every declared-unwired path -- never a hand-maintained flag** (vc). And a call site is still a declaration, so it is driven, not trusted.
- (2026-08-31) **`fileindex` is RETIRED** (hv's A4, 2026-08-26, executed at `c6515ad6`). A retired refusal and an unbuilt one say different things: _not implemented yet_ means LATER where hv ruled NEVER.
- (2026-08-31) **A ratification whose SUBJECT was retired is SUPERSEDED, not UNAPPLIED**, and the standing repair is to strike the id from the machine-read `covers:` clause and record the supersession beneath it -- **deleting it would make the record claim hv ratified something they did not.**
- (2026-08-31) **`issues edit` refuses an empty body, and that is NOT symmetric with `add`.** `add` leaves a body empty because an unwritten body is a STATE; editing one to empty is an ERASURE. And "no prose given" is refused separately from "prose that is empty" -- **AC-04.4 forbids one message for two causes.**
- (2026-08-31) **`issues edit` is withheld from MCP for AUTHORSHIP, not recoverability**, recorded in `recoverability_anomaly`: every other exposed issue mutator creates prose or moves a status field; this REPLACES prose a human wrote. **Withholding is the recoverable error.** Flagged to hv and ic, not settled by the builder.
- (2026-08-31) **The shell footer names the RELATIONSHIP between artefacts, not three shas** -- `intentsvcs` is linked into `intentd` and is the same build by construction; `intent` is a separate artefact read from the sibling. It showed a MISMATCHED PAIR on its first render.
- (2026-08-31) **`daemon status` narrowed to `terminal`, to be widened when a projection has a reader** -- and ST0064's menubar app is now that reader.
- (2026-08-30) **`RealDaemon` refuses a stale sibling rather than rebuilding one**; `restart(self)` reuses the home, and **the socket path is fixed per home so the PID is the witness, never the endpoint.**
- (2026-08-30) **A new guard gets a file named for its contract.** **One published port, both protocols, disambiguated at byte 0**; 51737 is a preference, never a promise.
