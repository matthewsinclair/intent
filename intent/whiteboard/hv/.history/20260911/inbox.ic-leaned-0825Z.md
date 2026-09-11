# ARCHIVE: hv/inbox.ic.md as it stood before the 2026-09-11 08:25Z lean

**LEANED BY ic ON hv's INSTRUCTION** (_"lean out any inbox/whiteboard content for HV after sending it to VC"_), **AFTER the full disposition below was routed to vc** so the pen had every item before any of it moved. **The inbox below is reproduced VERBATIM, byte for byte, after this header.** Nothing in it was summarised away; each entry's reason for leaving is stated here, against state driven 2026-09-11 08:22Z. **The first draft of this table carried `0218` forward as unruled, and so did ic's message to vc -- it was ruled two days earlier and ic's own board said so. Caught reading the board before commit, corrected in both places.**

| Entry                                | Disposition                                   | Why                                                                                                                                                                                                                                                  |
| ------------------------------------ | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-08-30 16:55Z ST0065 rulings     | DISCHARGED                                    | Ruling 3 executed at `0b5d46c9` (Option 2: drift-tested copy, generator measured and rejected). Rulings 1 and 2 moot -- CLAUDE.md documents AGENTS.md's absence at init; ruling 2 only preconditioned Option 3, not chosen. `ST0065` gate 8/8 PASS. |
| 2026-09-04 10:04Z announce           | SPENT                                         | Retracted six minutes later by the next entry.                                                                                                                                                                                                       |
| 2026-09-04 10:10Z retraction         | SPENT                                         | Its job was to reach peers.                                                                                                                                                                                                                          |
| 2026-09-08 19:02Z six items          | TWO CARRIED FORWARD, FOUR DISCHARGED          | Carried: `AC-01.7`, palette flip. Discharged: ST0065 (above); `0281` as an ask (`AC-01.4` satisfied at `6eb230653`); WP-15/WP-02 two homes (`ST0065/WP-02` Cancelled); `0218` remedy 1 -- **hv RULED it 2026-09-09** (uninstall promises the files it wrote, not the directory) and cc delivered the reason at `9f65beca`. The `restart.md` rev-list note is moot -- no longer there. |
| 2026-09-10 09:20Z census question    | QUESTION CARRIED FORWARD, REST SUPERSEDED     | The question is unanswered and no surface records a hold. The reap/durability half is superseded by `ST0073` Completed and by `0301`/`0302`.                                                                                                        |
| 2026-09-10 09:24Z WAL correction     | ARCHIVED; SUBSTANCE IN `0301`                 | The correction must not be lost, which is why it is here verbatim rather than summarised. vc asked to confirm hv saw the CORRECTION and not only the entry above it -- top-down reading delivers the wrong claim first.                              |

---

# inbox: ic -> hv

**LEANED 2026-09-03 17:17Z BY ic, ON hv's INSTRUCTION. THREE OF FOUR ENTRIES WERE DISCHARGED AND ARE ARCHIVED VERBATIM AT `hv/.history/20260903/inbox.ic-discharged-1717Z.md`, EACH WITH THE REASON IT WAS DISCHARGED.** Two were spent work announcements; the third was resolved by hv's own 2026-09-01 ruling reWORDING `AC-12.4`. **ONE ENTRY IS LIVE AND IT IS BELOW.**

---

## (2026-08-30 16:55Z)

**MY ST0065 COSTED PROPOSAL HAS BEEN SITTING UNROUTED SINCE 2026-08-28 17:48 AND YOUR BOARD IS RIGHT ABOUT WHY. ROUTING IT NOW.** `intent/st/ST0065/_proposal-agents-md.md`. Its own section 7 says _this goes to hv via vc_ -- and nothing did. **Not a write that failed: a write that never happened because I believed the routing had.** That is the hv-inbox class in its worse form and the author is me, so it is recorded here rather than summarised away.

**IT NEEDS NO READING TO BE ACTED ON -- THE THREE RULINGS ARE BELOW.** The document is design-first: no template, root file, skill or crate was touched for it, per your 2026-08-28 ruling.

=== THE PROBLEM, WHICH IS NOT THE ONE IT LOOKS LIKE ===

**AGENTS.md is not broken and the rules are not missing. Two correct local decisions compose into a wrong global outcome.** CLAUDE.md deliberately does NOT state the four principles, on Highlander grounds, and points at AGENTS.md. AGENTS.md deliberately DOES state them, ratified deviation (c) in `parity.md`. **And AGENTS.md is the one file the Claude Code agent never receives** -- observed directly in a session's own context, not inferred -- **and does not exist at all in a fresh project until `intent agents sync` runs.**

**THE HALF NOBODY HAD COUNTED:** the four-rule index has THREE live homes (AGENTS.md, usage-rules.md, in-standards/SKILL.md) and **CLAUDE.md has ZERO, deliberately. The only document that reasoned about Highlander is the only one honouring it, and the rule it honours was already violated three ways.**

=== THREE RULINGS, AND THEY ARE YOURS RATHER THAN vc's ===

1. **DOES `AGENTS.md` EXIST AT FRESH INIT?** Today it does NOT -- measured with a real `intent init` in a clean dir, with CLAUDE.md and config.json present in the same run as the control -- and CLAUDE.md references it four times including its opening paragraph. **Either init lays it down or those references stop assuming it. Both answers are coherent and they lead to different documents.**
2. **WHAT IS `AGENTS.md` A MIRROR OF?** Option 3 is unwritable until this is answered, **because a ledger of permitted divergences presupposes a thing to diverge from.**
3. **GENERATOR OR COPY FOR THE INDEX** -- and if generator, what to do about `usage-rules.md`, which **cannot join**: `canon.rs:316`, user-owned files are seeded, never synced.

=== THE RECOMMENDATION, COSTED ===

**Option 2 -- the 575-byte index duplicated in source and DRIFT-TESTED. S. Recommended.** Delivered by `claude upgrade --apply`, which already writes both files; one new arm on the existing invariant test asserts the block is byte-identical in every root template carrying it. **The mechanism is a test rather than a generator, deliberately** -- the same argument `agents_sync_parity.rs` already makes: you do not have to eliminate duplication, you have to make drift DETECTABLE.

**Option 1 (fix the pointer only, XS)** adds a FOURTH hand-kept home. **Option 3 (declare the contract, M)** is the ruling Option 2 sets up, and its M is an estimate by analogy rather than a measurement -- stated as unmeasured rather than left to surface in front of you. **Option 4 (inject AGENTS.md into Claude Code) is recorded ONLY so the rejection is visible**: it doubles the injected budget to deliver content the agent already has.

**Option 2 does NOT reach zero homes and the document says so:** three uncounted homes become two tested ones plus one declared exception, because `usage-rules.md` stays hand-kept and outside the test.

=== TWO CORRECTIONS THE DOCUMENT IS BUILT ON, BECAUSE BOTH POINTED AT A WORSE ANSWER ===

**The four principles are NOT unreachable** -- `/in-standards` carries all four ids with glosses and `/in-session` auto-loads it every session. **The uncorrected version would have recommended Option 4**, the most expensive one, to deliver content the agent already has. And **`intent agents sync` does NOT render all three root files** -- it hardcodes `AGENTS.md` at `render.rs:5529`; `render_all` over `ROOT_FILES` is a capability with no verb driving it. **The uncorrected version costed Option 2 against the wrong mechanism.**

**NOTHING HERE IS MINE TO EXECUTE AND I HAVE EXECUTED NONE OF IT.** `rootfiles.rs` and `canon.rs` are cc's; the build assignment is vc's to make. **ST0065 is out of the 3.0.1 cut, and the routing was owed regardless of the cut** -- which is the part I got wrong.

**CORRECTION APPENDED 2026-09-03 17:17Z BY ic WHILE LEANING THIS QUEUE: THE SENTENCE ABOVE IS STALE AND I AM NOT DELETING IT, BECAUSE IT IS WHAT YOU WERE TOLD AT THE TIME.** `ST0065 is out of the 3.0.1 cut` was true when written. **hv then ruled 2026-09-01 (via vc): RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED -- everything outstanding goes in**, and told me directly on 2026-09-03 _EVERYTHING IS IN 3.0.1_. **hv's own board still carries the out-of-cut line**, so two live surfaces disagree. **THE THREE RULINGS ABOVE ARE THEREFORE EITHER LIVE NOW OR EXPLICITLY EXCEPTED, AND WHICH ONE IS ITSELF A DECISION** -- routed to vc as Q7 of my outstanding-work report. Nothing here changes without hv's word.

## (2026-09-04 10:04Z) FYI only -- no response needed.

**ANNOUNCE -- I AM TOUCHING A SHARED SURFACE: `native/rust/crates/intent-cli/tests/common/`.**

Taking A1 on vc's direction: a composed drive-and-observe helper for interactive menu selection (Explorer/Lotus). Both primitives already ship in this crate and nobody has composed them -- `crate::common::pty_pair` (a real kernel PTY), `organize_default_force_applies.rs:85` (writes input into the master), `st_edit_opens_or_prints.rs:300` (drains the terminal on a concurrent thread while the child runs). The real unknown is **paint sync**: knowing a full-screen frame has landed before sending the next key. Type-ahead through the line discipline is enough for a line prompt and is not obviously enough for raw mode.

**WHY THIS IS AN ANNOUNCE AND NOT A NOTE: `tests/common/` rebuilds every test binary in `intent-cli`.** If you are mid-run in that crate you will see a rebuild; if you are about to commit there, my edits are in the shared worktree.

I will keep it additive -- a new module rather than edits to the existing helpers -- so nothing you already depend on moves. If that turns out to be impossible I will announce again before changing an existing signature.

**And vc's standing instruction on this, which I am adopting as mine: if paint sync turns out to be deep, I stop and report rather than absorbing it.** _We cannot observe paint deterministically_ is a real answer and worth having early.

## (2026-09-04 10:10Z) Re: 2026-09-04 10:04Z FYI only -- no response needed.

**RETRACTING THE ANNOUNCE: I DID NOT TOUCH `native/rust/crates/intent-cli/tests/common/`, AND WILL NOT ON A1.** Nothing of mine is in that directory; treat the surface as untouched by me.

**A1 STOPPED BECAUSE ITS PREMISE IS REFUTED, NOT BECAUSE PAINT SYNC WAS DEEP.** The harness A1 existed to build already exists in-process and is green: `src/tui/run.rs` drives the caret key by key in both vocabularies against concrete screen line numbers (16 green), and `src/tui/app.rs` drives the full open-filter-move-commit path including the argv that reaches the realiser, exhaustively over the command vocabulary (52 green).

**An announce that stands unretracted is a shared surface everyone avoids for nothing**, which is why this is going to all of you rather than only to vc.

## (2026-09-08 19:02Z)

**SIX ITEMS OF MINE NEED YOUR WORD AND ONLY ONE OF THEM WAS EVER FILED HERE. ROUTING THE OTHER FIVE, AT YOUR INSTRUCTION, SO THEY ARE IN ONE PLACE RATHER THAN ON MY BOARD WHERE YOU DO NOT READ THEM.** My own four inboxes are EMPTY -- there was nothing to clear out, and I am saying so rather than inventing an action to report.

Ordered by what unblocks a node, not by age.

=== 1. `ST0065`'s THREE RULINGS -- FILED 2026-08-30 16:55Z, STILL THE ONLY LIVE ENTRY ABOVE ===

**Unchanged and unanswered. `ic/inbox.hv.md` is empty, so nothing has come back.** This is the one that matters: **ic takes `WP-01` the moment they land, and I have had nothing startable for most of three days.** The three questions are in the entry above (does `AGENTS.md` exist at fresh init; what is it a mirror of; generator or drift-tested copy for the four-rule index) with a costed recommendation of **Option 2, S**.

=== 2. `0281` -- A DESIGN DISPOSITION, AND THE REMEDY IS ALREADY WRITTEN DOWN ===

**`ST0064`'s `AC-01.4` cannot close without it, and there is no engineering left to do first.** `IntentCLI.swift:131` assigns the console tail's lifecycle to _the verb's job, not the app's_ -- and no `log`, `tail` or `console` verb exists anywhere in the surface (positive-controlled: the same probe returns 2 for `daemon`).

**THE FIX IS IN OUR OWN TREE, FROM GEODICA, AND NOTHING LINKED IT UNTIL I RE-SYNCED `ST0064`'s DOCS:** `design-menubar-app.md` records the identical failure already diagnosed and already fixed in their estate -- _the verb runs its pipeline under a shell that reads its own stdin and takes the process group down when that pipe closes; verified against SIGTERM, SIGINT and SIGKILL, zero orphans._ **The lesson crossed estates and the implementation did not.**

**WHAT I NEED IS ONE WORD: does Intent adopt that shape?** If yes it is cc's build and I write the fixture. **The fixture has a precondition that must not be lost** -- `kill(-pgid)` works only because Foundation makes the child a group LEADER, which I measured on this machine on this day and is not a documented guarantee; if a future Foundation stops doing it, `-pgid` names the app and the remedy becomes worse than the leak. So the test asserts `child.pgid == child.pid` FIRST.

**RELATED FROM THE OTHER SIDE: `0284`** -- 27 leaked `intentd` processes, 249 minutes of CPU, none answering. **Two spawn paths in one estate, neither cleaning up after itself.**

=== 3. `AC-01.7` -- APPLE DEVELOPER CREDENTIALS, ONLY YOU ===

Not a decision, a dependency. **Recording it here so it is visible next to the others rather than looking like work someone is withholding.**

=== 4. NEW TONIGHT: `ST0056/WP-15` AND `ST0065/WP-02` ARE TWO HOMES FOR ONE CONCERN, BOTH WIP ===

- **`WP-15`** _Skills catalogue triage_, scope **L**, WIP, fully specified, **vc's and being worked right now**.
- **`WP-02`** _Audit and cleanup of the /in-\* skills_, scope **S**, WIP, **Objective `_(not yet written)_`**. Mine, via the `ST0065` claim.

**WP-15's deliverable list already contains WP-02's entire subject, verbatim** -- _a check that the catalogue's cross-references still resolve: `chains_to:` frontmatter, the `/in-*` names skills cite in each other's prose, and the rule IDs they name._

**THE DISPOSITION LOOKS OBVIOUS AND IS DELIBERATELY NOT BEING TAKEN BY ANYONE HERE.** Collapsing a work package is a scope call. vc has declined to resolve it under the pen -- their words: two homes for one concern across two open threads is the same defect they had just cut three instances of on their own board. **What is already done is the safe half: I am not starting WP-02**, so nothing duplicates while this waits.

=== 5. `0218` REMEDY 1 -- WHAT `uninstall` PROMISES ===

**The files it wrote, or the directory it emptied.** Not _implement pruning_ -- `prune_empty_dirs` IS called and refuses `dir == root`, which is the skip. **Both answers are coherent and they produce different behaviour**, so it is yours and not a bug to fix.

=== 6. THE PALETTE `Home`/`End` FLIP ===

**A product-feel call, which is why neither I nor the pen may take it.** The `/` sigil means the palette's buffer is never empty, so the empty-composer guard cannot fire the way it does elsewhere.

=== AND ONE THING THAT IS NOT AN ASK, BECAUSE IT IS ALREADY MOVING ===

**`restart.md`'s three-measurements ritual hands every node a BOUND dressed as a MEASUREMENT.** `git rev-list --count <marker>..HEAD` counts commits already compiled in, so drift is overstated; and the scope it ranges over is missing `lib/templates/{llm,prj}`, which compiles into the binary. **Three nodes quoted figures over that denominator tonight and every one was caught by a peer rather than by any instrument.** vc has it as the global fold's first job and cc's wording is the fix -- **label the number an upper bound rather than remove the command.** Raising it here only because the ritual is the first thing every node runs and you are the reader who would otherwise meet it cold.

## (2026-09-10 09:20Z)

**ONE ASK, AND IT IS SMALL: DO YOU STILL HOLD ANYTHING ON THE `v2:`-LABEL CENSUS?** I am asking because I cannot answer it from any surface, which is the actual finding.

**THE HOLD ON MY BOARD READ _RELEASES WHEN hv RELEASES THEIR HOLD_ AND THAT IS A PARAPHRASE OF WORDS NOTHING RECORDS.** Driven across every `hv` surface: no condition for that census is written anywhere but my own board. **The search took two passes and the first one lied** -- `grep -rl 'v2:' hv/` returns zero, and positive-controlling it showed the zero belonged to the pattern rather than to the corpus (`census` matches three hv files, `v2` matches four). Read, all of them are a different subject: dc's `AC-02.3` fixture census, the fleet census, the elixir-pack census, the rate census, dc's citation census. **So the claim is narrower than my first draft of it and better evidenced: the word is there and the subject is not.**

**I HAVE RE-WORDED IT AGAINST A STATE I CAN DRIVE RATHER THAN LEAVE IT POINTING AT YOU** -- `0215` reads `closed`, so the census's only live subject is vc's candidate `AC-00.16` amendment, and the hold now releases when that is ruled. **Please do not read that as your hold being lifted. It is not. It is that no record of your hold survives**, and if you do still hold something here it needs re-establishing, because nothing on the board would show it.

**NOT AN ASK, BUT IT IS YOURS TO DECIDE ON AND I MEASURED IT FIRST-HAND: THE REAP IS A CLEANUP, NOT A FIX.** Zero `intentd` after it, WAL fully reclaimed into a 22 MB db, load 499 to 15, and the store came through the recovery with every gate figure intact. **Within four minutes a `PPID 1` holder was back with the WAL at 19.9 MB from zero**, and the population has churned since. The spawner is a peer's live test run rather than any verb of mine. **vc carries the full picture and the ordered ask; this is only the durability half, which nobody had measured because the reap had not happened yet.**

## (2026-09-10 09:24Z) CORRECTION: THE WAL DID NOT RECLAIM ITSELF. I SAID IT DID, ONE ENTRY ABOVE, AND THAT READING WOULD COST YOU THE FIX

**MY PREVIOUS ENTRY SAYS _WAL fully reclaimed into a 22 MB db_ IN A SENTENCE ABOUT THE REAP, WHICH READS AS THE REAP RECLAIMING IT. IT DID NOT. vc TRUNCATED IT BY HAND AND I MEASURED THE STATE THEY LEFT** (vc's correction, 2026-09-10 09:24Z window):

    after the reap, 0 daemons, 0 holding the store
    intent.db-wal   559,479,552 bytes   -- STILL THERE, which is the finding
    PRAGMA wal_checkpoint(TRUNCATE)  ->  0|0|0
    intent.db-wal   0 bytes, then removed

**THE FINDING THAT SURVIVES IS THE OPPOSITE OF THE ONE I SENT YOU, AND IT IS ACTIONABLE WHERE MINE WAS NOT: A WAL THAT OUTLIVES ITS LAST CONNECTION MEANS THAT CONNECTION DID NOT CLOSE CLEANLY.** SIGTERM'd daemons are not closing their sqlite handle. **That is a defect with a fix. _It self-healed_ has no fix, and believing it is what would stop one being written.**

**AND THE FIRST ITEM OF vc's ORDERED ASK IS NOT DISCHARGED BY THE REAP.** It was discharged by a command a person typed. **Nothing will type it next time**, so the recovery of 559 MB is not evidence that the next accumulation recovers.

**WHAT STANDS FROM THAT ENTRY, UNCHANGED AND RE-DRIVEN:** zero `intentd` after the reap by three instruments with the instrument positive-controlled; load 499 to 15; the store intact across the recovery with all four gate figures matching what I last drove them to; and **the durability half -- holders reappear within minutes and the WAL regrows**, which is the half that made the entry worth sending.

**THE SHAPE OF MY ERROR, BECAUSE IT IS THE ONE THIS BOARD KEEPS CATALOGUING: I MEASURED A STATE AND ATTRIBUTED A MECHANISM TO IT.** _No `-wal` present_ is the observable. _It checkpointed on last close_ is a story about how it got that way, and I had no evidence for the story -- I had a peer's framing that predicted it and I read my measurement as confirming their prediction. **A measurement that agrees with a prediction is not evidence for the prediction's mechanism**, and I sent it to you as though it were.
