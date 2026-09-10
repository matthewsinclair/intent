---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-10 17:24Z
status: active
focus: "0216 SIGHTING 4 LANDED at 9639527f5 -- sighting 3's asymmetry reproduced with the CONTENTION COUNTED, six cells, and the row says what the table does NOT show. WP-04 landed at 3a82fae27: /tmp fixture population 902 -> 10 and HOLDING across other nodes' runs. ST0064 8/9, ST0065 8/8 PASS, ST0073 4/7. FOUND AND ROUTED, NOT TAKEN: AC-05.1's own evidence note says pgrep -f UNDER-reports; driven at 17:2xZ it OVER-reports 7 against a true 4, counting a peer's shell and my own measuring pipeline. It is unsound in BOTH directions, and on a ZERO row a reader told only one direction treats the count as a floor. That is vc's row and I have not touched it. SUITES STILL FROZEN -- vc froze them and has not lifted it; a quiet machine is not permission. NO FIGURE ON THIS BOARD IS EVIDENCE; RUN THE VERBS."
claims: [ST0065, ST0056/17, ST0064, ST0073/04]
---

# Interface Claude (ic)

**Aggressive localfold 2026-09-03 14:46Z (hv: localfold + compact, then reconvene). Pre-fold board verbatim + sha-verified in this day's history dir as `wip-prefold-1446Z.md` (`09fe527766ff4402`, 27582 bytes, prettier-stable before the sha was taken). STATUS STAYS `active` -- a fold is not the end of a session.**

**FOLDED ON EXECUTION, NEVER ON DATE.** Everything cut was DONE: today's three hv asks are landed, built and delivered, and the three expired figures I opened the day correcting were each discharged by the node that owned them. **Every unexecuted ruling is still here**, and gated work sits under Holds with the condition that releases it rather than in TODO looking like work I am declining to start.

## DOING

**EVERYTHING I HELD UNDER THE FREEZE IS LANDED AND THE FREEZE'S OWN CONDITION -- vc's push -- IS DISCHARGED.** `AC-01.4` satisfied at `6eb230653`, `0301` filed at `2ca41641`, filing B withdrawn rather than filed. **What is left in this section is one measurement hv needs and nobody currently owns.**

### THE REAP HAPPENED AND THE LEAK DID NOT STOP, WHICH ARE TWO DIFFERENT FACTS

**MEASURED 2026-09-10 09:0xZ, INSTRUMENT POSITIVE-CONTROLLED FIRST.** Zero `intentd` by three instruments (`pgrep -x`, `pgrep -f`, raw `ps | grep`), with `pgrep -x zsh` returning 51 so the zero is a measurement rather than a pattern that cannot match its subject. Load averages 15 / 16 / 22 against vc's 499. **THE WAL WAS GONE WHEN I MEASURED AND I DREW EXACTLY THE WRONG CONCLUSION FROM IT (corrected by vc, 2026-09-10 09:03Z).** I wrote that the 534 MB _checkpointed back into the database on last close, which is vc's ratchet framing arriving exactly as written_. **INVERTED. vc TRUNCATED IT BY HAND AND I MEASURED THE STATE THEY LEFT.** After the reap, with zero daemons and zero holders, `intent.db-wal` was **still there at 559,479,552 bytes**; `PRAGMA wal_checkpoint(TRUNCATE)` returned `0|0|0` and only then did it go to zero and get removed.

**SO THE FINDING IS THE OPPOSITE OF SELF-HEALING AND IT IS THE ONE THAT MATTERS: A WAL THAT OUTLIVES ITS LAST CONNECTION MEANS THAT CONNECTION DID NOT CLOSE CLEANLY.** SIGTERM'd daemons are not closing their sqlite handle. **_It self-healed_ is not merely wrong, it is the reading that would stop the fix being written** (vc's words, and they are right). And vc's ordered first item is NOT discharged by the reap -- it was discharged by a command a person ran, **which is a different thing, because nothing will run it next time.**

**THE SHAPE OF MY ERROR: I MEASURED A STATE AND ATTRIBUTED A MECHANISM TO IT.** `no -wal present` is the observable; _it checkpointed on last close_ is a story about how it got that way, and I had no evidence for the story. **It is the second time in one morning I inferred a cause from a state** -- the other was crediting my own `app-test` with spawning daemons, which the timestamps refuted. That one I caught myself; this one a peer caught, and it had already reached hv's inbox.

**THE STORE SURVIVED THE RECOVERY WITH NOTHING LOST**, checked against the four gates rather than assumed: `ST0064` 7/9 at the time, `ST0065` 8/8, `ST0056` 124/142 (25 descoped, 2 withdrawn), `ST0068` 6/9 -- every figure exactly what I last drove it to.

**AND READS DO NOT RESPAWN THEM, WHICH IS THE DURABILITY QUESTION NOBODY HAD ASKED.** I was the first `intent` invocation after the reap and instrumented it rather than spending it: 0 before, 0 after `--version`, 0 after `daemon status`, 0 after `st list`. **`daemon status` answers `no intentd is answering; commands run in-process` and `st list` returns real data, so the store serves in-process with no daemon at all.**

**BUT HOLDERS REAPPEAR AND THE WAL REGROWS, SO THE REAP IS A CLEANUP AND NOT A FIX.** Within four minutes a `target/release/intentd` at `PPID 1` held `intent.db`, `-wal` and `-shm` with the WAL already back to **19.9 MB from zero**, and the population has churned since (a different pid holds it now). **The proximate spawner is a peer's live test run, not mine** -- `cargo test -p intentd --test suite a_daemon_outlives_nobody`, a file created at 10:07 today. **I nearly attributed it to my own `app-test` and the timestamps refuted that: my run ended 10:06:32, the daemons started 10:07:17 and 10:07:22.** That is W124's discipline applied before the claim rather than after it.

**NOT MINE TO REAP AND I HAVE NOT TOUCHED THEM.** They may be in-flight fixtures for a test that is being written right now, and `0284` reserves the reap to hv anyway.

### WP-04 (ST0073): THE FIXTURE-HOME LEAK -- BUILT, GREEN, AND HELD ON A SHARED FILE

**`AC-04.1` SATISFIED; `ST0073` 3/7 -> 4/7.** Sweep lives in `testkit` -- one home, std only, no new dependency. `sweep_abandoned_fixtures()` is the assertable worker; `sweep_once()` is the idempotent at-START hook, separate because collapsing them makes the worker unassertable after its first call.

**THE DISCRIMINATOR IS STRUCTURAL AND THE DISK FORCED THAT.** Reading the six creation sites found four families; reading `/tmp` found twelve name shapes. A candidate begins `intent` AND ends `-<pid>-<counter>`. **The suffix clause is what saves the machine:** `/tmp/intent` is the in-session gate's sentinel DIRECTORY, one file per live Claude Code session, and `/tmp/intentfiles.new` is a stray file -- both begin `intent`, and the prefix-only sweep anyone writes first deletes the sentinel out from under every running session. Both exclusions are asserted arms.

**BURNED IN THREE DIRECTIONS, each firing on exactly its own arms:** liveness removed -> the live-fixture arm alone; removal disabled -> the dead arm and the population arm; one of six sites unwired -> the source arm alone. **Live effect: 902 dirs / 133.7 MB -> 19 / 3.4 MB.**

**LANDED at `3a82fae27` on vc's option (b) -- both hunks, theirs named as theirs. THE HELD CONDITION IS DISCHARGED:** `a_daemon_outlives_nobody.rs` carries my sweep hunk and vc's `LIFELINE-EXEMPT` hunk, and `git commit --only` is path-scoped rather than hunk-scoped. **Skipping that file was not available** -- my own source arm would have been RED in CI, which is shipping a guard broken. vc ruled: take both rather than hunk-split a shared file under load. **AND IT LANDED WITHOUT A VERIFICATION RUN, DELIBERATELY:** vc froze suite runs at load 502, so it rests on the green taken before the freeze plus a doc-comment edit that cannot change behaviour. That is stated in the commit rather than left for someone to discover.

**THREE intentd REDS ARE PRE-EXISTING AND I PROVED IT RATHER THAN ASSUMED IT.** Detached worktree at clean HEAD, none of my changes and none of vc's uncommitted work: both `daemon_subscriptions` arms and `daemon_watch::a_change_to_a_path_outside_the_sync_scope_drives_no_ingest`. **The third is a POSITIVE-CONTROL failure rather than the property failing** -- its own precondition, the daemon ingesting at all, did not happen in 500 attempts, so reading that panic as _the daemon wrongly ingested_ sends the next reader at the opposite defect.

**AND MY FIRST ATTEMPT AT THAT WORKTREE MEASURED NOTHING.** `--test daemon_watch` returned `no test target` because at HEAD it is a MODULE of `suite`, and the run exited 0 with no test lines. **I nearly read that silence as a pass.**

### 0216 SIGHTING 4, AND THE INSTRUMENT NOTE IT TURNED UP ON SOMEBODY ELSE'S ROW

**LANDED at `9639527f5`, held on the first attempt at ZERO holders** -- which is the reproduction's `contenders=0` arm on the live tree and is itself a seventh cell. Six cells in the row, each with a holder count taken by `lsof` in the same minute as the write. **The last pair is sighting 3 exactly:** `at edit --file --note` then `at green`, one row, minutes apart, citation and its 4012-byte note kept, green lost, both `ok:` at rc=0.

**THE ROW STATES WHAT THE TABLE DOES NOT SHOW, because the obvious reading of it is wrong.** The two four-holder cells are one loss and one hold, same verb and same row, which is a stochastic process sampled twice rather than a scaling law. **I made that scaling claim to vc in a message and withdrew it on the third cell, before it reached hv.** The withdrawn orphans-are-the-engine hypothesis is retired inside the row so the next reader does not re-derive it.

**AND DRIVING IT TURNED UP A DIRECTION ERROR ON `AC-05.1`'s OWN EVIDENCE NOTE.** That note says to count with `ps` and never `pgrep -f`, _which UNDER-reports_. Driven 2026-09-10 17:2xZ: true population **4** by executable path, `pgrep -f` reports **7**. The three extras read back one by one are a peer session's zsh wrapper whose COMMAND LINE contains the word, and two empty-command pids that were **my own measuring pipeline's subshells** -- `pgrep -f` counted the observer.

**BOTH DIRECTIONS CAN BE TRUE AND I HAVE DRIVEN ONLY ONE, WHICH IS HOW IT IS STATED.** It matches command-line text, so it gains anything that merely mentions the name; macOS truncates the string it matches against, so it can also lose a long one. **The direction is load-bearing on THAT row specifically because `AC-05.1` is a ZERO claim:** a reader told only that it under-reports treats its count as a FLOOR, and at a true zero it can read nonzero. **Routed to vc; it is their row and I have not touched it.**

## TODO

**ONE ITEM IS MINE AND STARTABLE; THE REST IS GATED OR SOMEBODY ELSE'S. DRIVEN 2026-09-10, NOT RECALLED.**

1. **HOLD 2's RE-WORD IS DONE (2026-09-10), AND IT DID NOT GO THE WAY THIS ENTRY PREDICTED.** I wrote that it _needs nothing from anyone_. **Driving it refuted that in both directions:** option (a), record hv's actual sentence, is UNRECOVERABLE -- the phrase is in no `hv` surface and my board is its only home; option (b) was restated against `0215`, which `issues show` reads **closed**, so the condition now names vc's `AC-00.16` amendment and is checkable by me at any pickup. **The residue is a question routed to vc under the pen, so the item did need someone after all.** Left here rather than deleted because _this needs nobody_ is the claim that was wrong, and it was mine.

**`ST0064` IS 8/9 AND THE LAST ROW IS NOT ENGINEERING.** `AC-01.7` is Apple Developer credentials -- hv's, an ACTION rather than a decision. **DO NOT PICK IT UP**, and do not read a lifted freeze as closing that thread.

**`ST0065` IS 8/8 PASS AND `WP-01` IS FINISHED AS A REVIEW, PAUSED ON THREE DISPOSITIONS AND NONE IS MINE:** `_DECISION_TREE.md`'s unconditional laydown (routed to hv by vc), whether the two dead-reason templates get dropped at the cut, and `0224`'s open design question.

**WHAT I DELIBERATELY DID NOT ADD ANYWHERE, BECAUSE THE HOME ALREADY EXISTS.** I lost a harness verdict this morning to `${PIPESTATUS[0]}` in a zsh tool shell, and `intent/restart.md:126` already says _"Never `$?` after a pipe -- zsh has `pipestatus`, not `PIPESTATUS`."_ **The rule is in a boot-read doc and I did not apply it; that is mine to own and not a second home to create.** The recovery is the durable part and it is in `AC-01.4`'s evidence: `cmd_app_test` calls `die` before its final printf, so `IntentTests passed` is emitted ONLY on the rc==0 branch and the terminal line is a stronger verdict than the rc I failed to capture.

## Holds -- work I am NOT doing, each with the condition that releases it

**A hold without a condition is an abandonment wearing a hold's name, so every line names what must become true, not who I am waiting on.** **THE AXIS IS ACT VERSUS STATE (vc's refinement of W23), NOT MINE VERSUS SOMEONE ELSE'S.**

**AND A NEW LIMB, LEARNED THIS MORNING AT MY OWN COST: A HOLD MUST ALSO NAME WHOSE THE CONDITION IS.** Two entries below sat for days in a column that read as hv's while hv owed nothing on either. **A durable surface with no expiry lets an item's presence read as its own justification (vc); a durable surface with no OWNERSHIP CHECK manufactures load on someone who was never asked.** Both are struck from the hv column and kept here, where they belong.

1. **THE ESTATE-WIDE BURN** -- **RELEASES WHEN THE TREE IS QUIET ENOUGH TO REBUILD, because the subject must be a CURRENT binary.** **THIS ENTRY EXISTS BECAUSE THE FOLD NEARLY BURIED IT: the burn lived under `## DOING` all day and its condition is UNMET, so archiving that section without moving it here would have been exactly the disappearance this section is for.**

   **THE CONDITION IT USED TO CARRY IS SPENT AND THE ONE THAT REPLACED IT IS A STATE, NOT A PERSON.** cc's provenance line landed at `75f1c0a76`: `burn.sh` now refuses an unset `INTENT_BIN` and stamps `SUBJECT=` and `VERSION=` on the TSV header, so a second subject-less artefact is no longer possible. **THE STALENESS LIMB MOVED ON 2026-09-09 AND THE HOLD STANDS ON THE OTHER ONE.** vc rebuilt the pair at `ec55b3ba`, mtime 23:14, so _the binary is stale_ is no longer the binding reason -- and **I am not calling it released on that**: `git rev-list --count <marker>..HEAD -- native/rust surface` reads **3**, which `restart.md` says is an UPPER BOUND that proves nothing when non-zero. **What binds now is hv's sequencing, untouched:** `0196`'s staging-plus-atomic-mv fix FIRST, the rebuild through it, no separate supervised window. **vc UPHELD THE HOLD 2026-09-09 rather than releasing it** -- hv's sequencing puts the staging-plus-atomic-mv fix for `0196` FIRST and the rebuild through it, with no separate supervised window, and cc and dc each independently declined to rebuild on the same grounds. **`coverage_map.sh` stays correctly refusing until then (98 baseline rows against 116 on-disk `.bats`).** When it runs, the baseline write goes through vc's ST0056 window (W2).

2. **The `v2:`-label census** -- **RELEASES WHEN vc's CANDIDATE `AC-00.16` AMENDMENT IS RULED.** It is ratified and it is dc's; **do not ask them to break it.** The census is load-bearing only as evidence for that amendment: _an instrument's reach is bounded by what the ESTATE CAN EXHIBIT, and that bound is a property of the ENVIRONMENT rather than of any tool._

   **THE RE-WORD OWED SINCE 2026-09-09 IS DONE, AND DRIVING IT FOUND THAT ONE OF ITS TWO OPTIONS DOES NOT EXIST.** The owed fix was _record hv's actual sentence, or restate the condition against dc's work_. **hv's actual sentence is UNRECOVERABLE, AND SAYING SO TOOK A SECOND PASS BECAUSE MY FIRST INSTRUMENT GAVE A CONFIDENT ZERO.** Grepping `v2:` across `hv/` returned nothing, which I nearly wrote down as _the phrase appears nowhere_. **Positive-controlled, that zero was the pattern's and not the corpus's:** `census` matches THREE hv files and `v2` matches four. **Read, they are all a different subject** -- dc's `AC-02.3` fixture-coverage census, the fleet census, the elixir-pack census, the rate census, dc's citation census -- and `v2` at `hv/wip.md:268` is _this is NOT a v2-parity gap_. **So the finding survives in a narrower and better-evidenced form: no hv surface records a condition for the `v2:`-LABEL census specifically, and my own board is its only home.** The word matches and the subject does not, which is the mention-versus-use split, committed inside the re-word that exists because a condition had no checkable home. The original condition read _RELEASES WHEN hv releases their hold_ (`.history/20260903/wip-prefold-1446Z.md`), which is a paraphrase of words no surface records. **A condition whose only witness is the person waiting on it cannot be checked by anyone, including them.**

   **SO IT IS RESTATED AGAINST A STATE I CAN DRIVE AT EVERY PICKUP, WHICH IS WHAT W23 ASKS FOR.** `intent issues show 0215` reads **closed**, so the worked case is discharged and the census's remaining subject is the amendment alone -- vc's, named, and checkable without asking anyone. **THE PREVIOUS INTERIM WORDING WAS WORSE THAN EITHER: _releases when dc's work makes it checkable_ silently substituted a different condition** and made the item look like it was waiting on dc, who owed nothing on it.

   **ONE THING IS ROUTED RATHER THAN RESOLVED: if hv does still hold something here, it needs re-establishing, because nothing on the board would show it.** Put to vc under the pen 2026-09-10. **Do not read this hold's re-word as evidence that hv's hold was lifted -- it is evidence that no record of it survives.**

3. **`ST0064` WP-01's REMAINING ONE, AND IT IS NOT ENGINEERING WORK (re-driven 2026-09-10: `intent ac gate ST0064` reads 8/9).** **`AC-01.7` -- RELEASES WHEN APPLE DEVELOPER CREDENTIALS EXIST.** hv's to supply, and it is an ACTION rather than a decision. The `int macos` lane question is CLOSED (option A).

   **`AC-01.4` LEFT THIS SECTION ON EXECUTION, WHICH IS THE ONLY GROUND ON WHICH A HOLD MAY GO.** Its condition was the freeze; vc's push discharged it; the row is satisfied at `6eb230653` on a green re-driven to name HEAD rather than on dc's greens, which named two commits behind. **`0281` STAYS OPEN AND THAT IS NOT AN OVERSIGHT: the ruling is made, the remedy in the app is NOT built, and there is no console to build it into yet.** The evidence says so in its own body, because a gate shows only `satisfied` and the next reader of that row is whoever builds the console.

4. **The palette `Home`/`End` flip** -- RELEASES WHEN IT IS RULED, **and with hv AFK that is vc under the pen rather than nobody. STILL UNRULED at this cut.** A product-feel call, which is why neither I nor the pen may take it. The `/` sigil means the palette's buffer is never empty, so the empty-composer guard cannot fire there -- which also makes _only when empty_ a no-op rather than a compromise. Flipping costs the query line its caret keys.

5. **`0222` -- the explorer thread list's `Status | Created | Completed` columns** -- RELEASES WHEN `intent wp show ST0056/17` READS `done`. **NOT AN hv ITEM AND NEVER WAS; struck from my hv column 2026-09-09** -- the condition is a STATE I check myself and hv is nowhere in it. **Re-driven 2026-09-09: still `WIP`, so the hold STANDS.** **SIZE UNMEASURED; do not quote one.** The blocker is the row type, not the columns: `Row` carries one name and one value, and the thread-collection arm is the one view that does not go through `form::triples`.

**ONE ENTRY LEFT THIS SECTION IN THIS FOLD ON EXECUTION, WHICH IS THE ONLY GROUND ON WHICH A HOLD MAY GO** -- `0218`, which hv released this morning and cc DELIVERED at `9f65beca`. **AND ONE ENTERED IT: the burn, which had been sitting in `## DOING` with an unmet condition and would have been archived with the rest.** `ST0065/WP-01` left at the previous fold and is in TODO. **A fold keyed on a DATE rather than on execution is how an unexecuted ruling gets buried, and this section is where that would happen.**

## Watch-outs -- mechanisms only

**RULES ONLY AS OF 2026-09-10 06:49Z. EVERY WORKED EXAMPLE IS IN `.history/20260910/watch-outs-full-0015Z.md`** (`5a51ba5b3f9ee291`), archived verbatim before this cut. **The section was 145 KB of a 408-line board -- 92% of it -- and a watch-out nobody can finish reading is a watch-out nobody reads.** What survives is each entry's first bold span, which is the mechanism; what left is the case that taught it.

**THE COMPRESSION WAS DRIVEN BEFORE IT WAS APPLIED, AND THE DRY RUN CAUGHT ITS OWN INSTRUMENT.** My first pass reported entries 100-106 as a _pre-existing gap in the numbering_. **They were not a gap: those seven lines use TWO spaces after the period and my regex required one.** Verified by asking the original for each number rather than believing the report -- the same shape as everything else on this board. Second pass: **123 entries, 1..119, NO missing numbers**, 42 sub-rules kept, and 5 lines lost entirely, all of them continuation lines inside one quoted tool-output block.

1. **Non-test AC closes via `intent ac satisfy --evidence`, NEVER an AT; test AT rows close via `at green`, non-test ones via `at na`.**

2. **Canon on SHARED threads: SERIALISE ST0056 writes through vc; gather evidence OUTSIDE the window, make the window ONE coherent commit.**

3. **The macOS app is `native/macos/Intent/` (Swift, xcodegen, folder-GLOBBED).**

4. **A value/claim carried ONE STEP past what it supports is the recurring class, and it is MINE too.**

5. **`git commit --only <explicit paths>` is the only safe write**

6. **`intent --version` names the commit the BINARY WAS BUILT FROM; CURRENCY is that against `git diff --name-only <that>..HEAD -- native/rust`.**

7. **A MARKDOWN ATTACHMENT HAS TWO WRITERS -- ORDER: prettier FIRST, let the store settle, verify, THEN commit.**

8. **A TEST WRITTEN FROM ONE OBSERVED FAILURE INHERITS THAT FAILURE'S INCIDENTAL SHAPE, AND THE INHERITANCE IS INVISIBLE BECAUSE THE TEST PASSES ON THE CASE THAT MOTIVATED IT.**

9. **A THING DECLARED, WRITABLE, VISIBLE AND READ BY NOTHING IS THE RECURRING DEFECT ON THIS SURFACE.**

10. **~~A shared file can carry a peer's hunk~~ -- FOLDED INTO W39**

11. **A DEFECT'S BLAST RADIUS IS NOT ITS PACKAGE.**

12. **THE SHARED-ARTEFACT GUARD REFUSING A REBUILD IS THE GUARD WORKING.**

13. **A PREMISE CARRIES A VERSION IN THIS ESTATE, AND A RE-DRIVE AIMED AT THE CITATION PASSES**

14. **DRIVE A DESTRUCTIVE QUESTION IN A SANDBOX OR DO NOT DRIVE IT.**

15. **`ok:` IS THE TOOL'S REPORT, NOT EVIDENCE THE ROW SURVIVES (`0216`, HIGH). DRIVEN AND CENSUSED BY vc ON cc's DESIGN, 2026-09-02 21:4xZ -- MY OWN READING OF IT WAS WRONG IN TWO PLACES AND BOTH CORRECTIONS MATTER AT THE KEYBOARD.**

16. **BACKTICKS INSIDE A DOUBLE-QUOTED SHELL ARGUMENT ARE COMMAND SUBSTITUTION, AND THE FAILURE IS SILENT AND PLAUSIBLE.**

17. **A PROBE WHOSE SHAPE IS GUESSED ANSWERS A DIFFERENT QUESTION, AND ITS ANSWER LOOKS EXACTLY LIKE A REAL ONE. THREE LIMBS, ONE MECHANISM (`W37` and `W60` folded in here 2026-09-08 under `W106`).**

**LIMB 1, THE FALSE NEGATIVE.**

**LIMB 2, THE FALSE POSITIVE (was `W37`).**

**LIMB 3, AND IT IS THE LIMB THAT SHIPS (was `W60`).**

**ONE CURE COVERS ALL THREE: POSITIVE-CONTROL THE INSTRUMENT AGAINST A RECORD KNOWN TO BE IN THE WANTED STATE BEFORE ASSERTING OVER ONE YOU ARE UNSURE OF**

18. **ONE LOCAL MEASUREMENT BECAME AN ESTATE-WIDE CLAIM BECAUSE I RECALLED A KNOWN FAILURE SHAPE AND REPORTED IT AS A READING (2026-09-02, corrected by dc).**

19. **TWO HALVES EACH CORRECT ABOUT ITS OWN DIRECTION, AND NOTHING OWNING THE RELATIONSHIP BETWEEN THEM (2026-09-03, hv found it).**

20. **~~A test can be green because of the bug~~ -- FOLDED INTO `W44`**

21. **MEASURE WHAT EXISTS BEFORE COSTING A REPLACEMENT: THE ASK NAMES A SYMPTOM AND THE CAUSE IS A GUESS UNTIL DRIVEN.**

22. **A MARKER THAT IS BOTH A STRUCTURAL ANCHOR AND ORDINARY PROSE WILL BE FOUND IN THE PROSE (mine, during this very fold).**

23. **I WROTE A HOLD WHOSE CONDITION ASSERTED A PROPERTY OF A PROGRAM I HAD NEVER READ, AND IT STOOD FOR A DAY (mine, 2026-09-03).**

24. **A PARTIAL VIEW OF A RECORD READS EXACTLY LIKE THE RECORD (mine, 2026-09-03, twice in one episode and in opposite directions).**

25. **A VERBATIM ARCHIVE AND A FORMATTER GATE ARE IN DIRECT TENSION, AND `.prettierignore` DECIDES IT WITHOUT SAYING SO (mine, 2026-09-03).**

26. **A STATED REASON FOR NOT DOING SOMETHING IS AN AUTHOR'S JUDGEMENT AT A MOMENT AND READS EXACTLY LIKE A PROPERTY OF THE SYSTEM (2026-09-03).**

27. **REPLICATION DOES NOT REPAIR A SHARED BLIND SPOT, AND A SECOND NODE AGREEING IS THE THING MOST LIKELY TO BE MISTAKEN FOR CONFIRMATION (vc aimed my own finding at their check, 2026-09-03).**

28. **A STALENESS BOUND THAT IS STATED BUT NOT ACTED ON IS DECORATION, AND STATING IT MAKES THE CLAIM TRAVEL MORE EASILY (mine, corrected by me inside the hour).**

29. **COSTING AN APPROACH IN DETAIL MAKES ITS PREMISE FEEL EXAMINED, AND MINE WAS NEVER TESTED AT ALL.**

30. **A SHARED INDEX DESTROYS AUTHORSHIP AND THEN PRESENTS ADJACENCY AS IF IT WERE PROVENANCE (vc's framing, on my error).**

31. **THE ASYMMETRY THAT MAKES `unstage to protect a peer` A DESTRUCTIVE RULE (vc, same episode, worth keeping because it is counter-intuitive).**

32. **CONTIGUOUS ROWIDS DO NOT DISCRIMINATE _NEVER DELETED_ FROM _DELETED AND REWRITTEN_, AND I FILED A HIGH DEFECT ON THAT EVIDENCE (mine, 2026-09-04).**

33. **A FIX CANDIDATE BANKED IN A FILING IS A GUESS WEARING A FINDING'S CLOTHES.**

34. **~~A vacuity guard found a second defect~~ -- FOLDED INTO `W44`**

35. **A GATE VERDICT IS A READING, NOT A STATE, AND I ACTED ON ONE FOR HOURS WITHOUT RE-RUNNING IT -- SO DID BOTH PEERS (the day-long freeze, 2026-09-04).**

36. **AN INSTRUMENT BUILT FROM THE WORKTREE CANNOT EXHIBIT A STALE-INDEX DEFECT, BY CONSTRUCTION (dc's finding, on their own error, and recorded here only as a pointer -- it is theirs to write up).**

37. **~~A grep whose pattern is an alternation returns a count that cannot say which branch matched~~ -- FOLDED INTO `W17`**

38. **I EXTRAPOLATED AN ESTATE-WIDE FIGURE FROM ONE SAMPLE, AND THE SAMPLE WAS THE ONE I HAD ALREADY MEASURED AS UNUSUAL.**

39. **`--only` IS PATH-SCOPED, AND EVERY WAY THAT BITES IS ONE MECHANISM. (mine and dc's; cc cites this number; dc banks it as their W70. W10 and W51 folded in here 2026-09-04 -- they were three copies of one rule.)**

    **THE RULE: `git commit --only <paths>` BUILDS HEAD PLUS THE NAMED PATHS, TAKING EACH NAMED FILE'S WORKING STATE ENTIRE.**

    - **BOTH DIRECTIONS BIT IN ONE DAY.**
    - **EXCLUDING A PATH KEEPS YOUR CHANGE OUT OF YOUR COMMIT AND NOT OUT OF THE FILE (dc, 2026-09-04).**
    - **THE FILE THAT BITES IS THE ONE YOU HAVE NO HABIT ABOUT.**
    - **CURE, HUNK-LEVEL AND NON-INTERACTIVE**
    - **DETECTION: before naming any path on a commit, read its diff for AUTHORSHIP, not only for collision.**

40. **I NEARLY ACTED ON AN ASSUMED RETENTION MODEL, AND TESTING IT COST ONE COMMAND (mine, 2026-09-04).**

41. **A BUILD THAT REMOVES ITS OUTPUTS BEFORE VERIFYING THEM CAN LEAVE THE TOOL UNUSABLE, AND I DID THAT TO hv's MACHINE.**

42. **FILE OWNERSHIP DOES NOT IMPLY DIFF AUTHORSHIP IN A SHARED CHECKOUT. cc's, and they proved it against my claim.**

43. **A MECHANISM BANKED IN MEMORY HAS NO TRIGGER, AND MINE FIRED ON ME TODAY.**

44. **A GREEN TEST IS NOT EVIDENCE UNTIL I HAVE SEEN IT GO RED, AND THE ARM THAT CLAIMS TO BE THE LOAD-BEARING ONE IS WHERE TO LOOK FIRST (`W20`, `W34` and `W112` folded in here 2026-09-08 under `W106`).**

**THE PRACTICE.**

**WHY IT COSTS MORE THAN NO TEST (was `W20`).**

**WRITE THE VACUITY GUARD EVEN WHEN THE ASSERTION LOOKS SUFFICIENT (was `W34`).**

**AND THE FOURTH FACE, WHERE THE PROSE ASSIGNS A JOB RATHER THAN CLAIMING A PROPERTY (mine, filed as `0281`, 2026-09-08; vc asked for it as its own limb).**

**WHERE TO AIM FIRST (was `W112`).**

113. **BATCHING THE `date -u` READ INTO THE SAME TOOL CALL AS THE WRITE MEANS THE STAMP IS CHOSEN BEFORE THE READ RETURNS -- WHICH IS FABRICATION WITH A CLOCK READ SITTING NEXT TO IT (mine, TWICE on 2026-09-08, caught both times before commit).**

114. **I CORRECTED THE INSTANCE AND NOT THE CLASS, IN A FILING WHERE I HAD SPENT THE DAY TELLING OTHER NODES TO SWEEP THE CLASS AND NAME THE PATHS (mine, caught by cc, 2026-09-08).**

**AND cc's ROUTE WAS STRUCTURALLY BETTER THAN A CAREFULLER RE-READ.**

**THIRD LIMB, AND IT IS THE ONE I WOULD HAVE STAYED WRONG ABOUT INDEFINITELY: I HEDGED THE CRITERION IN THE DIRECTION THAT WAS STRUCTURALLY CLOSED.**

115. **A REASON FELT LIKE EVIDENCE, AND THE HEDGE LANDED ON THE CLAIM THAT DID NOT NEED IT (mine, 2026-09-08, the FOURTH wrong attribution in an estate where I had written _three nodes, three wrong attributions_ onto this board the same morning).**

**THE TELL IS IN THE SAME MESSAGE, WHICH IS WHY THIS IS A MECHANISM AND NOT A LAPSE.**

**W114's THIRD LIMB RECURRING INSIDE AN HOUR: a caveat aimed at the wrong failure direction reads as diligence and buys confidence in the claim that is wrong.**

116. **A CRITERION ANSWERS _WHICH FILES MATCH_ AND NEVER _WHICH FILES THIS IS ABOUT_, AND NOTHING IN ITS OUTPUT DISTINGUISHES THOSE (mine and cc's, opposite errors on one population, 2026-09-08).**

117. **RE-MEASURE A BLOCKER BEFORE REPORTING IT, NOT ONLY BEFORE ACTING ON IT -- PESSIMISTIC-DIRECTION STALENESS IS THE KIND THAT DOES NOT SELF-CORRECT (vc's formulation, on my `CHANGELOG` hand-off, 2026-09-08).**

**THE ASYMMETRY IS THE WHOLE POINT AND IT IS NOT ABOUT CARE.**

**WHAT MADE IT CHEAP WAS LEAVING THE GAP VISIBLE RATHER THAN CLOSING IT WRONGLY**
**THE GENERAL FORM IS cc's AND IT SUBSUMES ALL THREE FACES OF THIS FLAG: `--only` IS AN INSTRUCTION, NEVER A FILTER. IT SUBTRACTS NOTHING YOU DID NOT THINK TO NAME, AND IT REPORTS SUCCESS EITHER WAY.**

**THIRD LIMB, AND IT IS THE ONE THAT EXPLAINS WHY WRITING THE ENTRY DID NOT PREVENT THE RECURRENCE (mine, corrected by vc, 2026-09-08, HOURS after banking the entry above).**

**FOURTH LIMB, AND IT IS cc's, AND IT HAS MORE REACH THAN THE THREE ABOVE (2026-09-08, within ten minutes of `W118` reaching them).**

**cc's formulation, which supersedes mine as the general form: AN INHERITED DENOMINATOR IS A CARRY-FORWARD ITEM THAT DOES NOT LOOK LIKE ONE, BECAUSE IT ARRIVES AS INFRASTRUCTURE RATHER THAN AS A CLAIM.**

**IT LANDS ON ME AND NOT ONLY ON THEM: my drift figure this boot came from `restart.md`'s prescribed command, which uses that same scope**

**AND THE CURE FOR THIS CLASS IS ALREADY BUILT IN THE PARITY TOOLKIT, WHICH IS THE FIND WORTH KEEPING**

119. **A GUARD DISCLOSED ITS OWN BLIND SPOT ON ONE LINE AND PRINTED `ok` ON THE NEXT, AND THE `ok` IS THE LINE A READER TAKES AWAY (cc's find via `0287`, DRIVEN BY ME RATHER THAN RELAYED, 2026-09-08).**

**THE HONESTY IS REAL AND IT DOES NOT SAVE THE READING.**

**BOTH CONTROLS FIRING IS THE PART THAT INVERTS.**

**WHY THE EMBED WAS INVISIBLE, WHICH IS THE REUSABLE HALF:**

**THE CURE ALREADY EXISTS ONE DIRECTORY OVER AND WAS NOT REACHED FOR: `lib_corpus.sh`'s `corpus_require` REFUSES rather than publishing over a corpus it cannot vouch for.**

**AND THE PROVENANCE MATTERS TO ME: cc went looking because of my own line that a bias has a direction and each caller must check their OWN use against it (`W118`).**

118. **A BIASED INSTRUMENT IS NOT UNIFORMLY WORSE, AND I RETIRED ONE FOR A PEER WHOSE USE OF IT THE BIAS MAKES SAFE (mine, corrected by cc within minutes, 2026-09-08).**

**I KNEW THE DIRECTION AND STILL GENERALISED PAST IT, WHICH IS THE PART WORTH KEEPING.**

**AND THE ESTATE FIX GOT SHARPER FOR BEING WRONG.**

45. **A PARAPHRASE IS NOT SUPPOSED TO CARRY EVERYTHING, WHICH IS EXACTLY WHY IT MUST NEVER BE THE THING YOU BUILD FROM.**

46. **THE DISPROOF WAS IN THE CODE TWICE TODAY, BOTH TIMES BEFORE I FILED.**

47. **RELAYING LAUNDERS A CLAIM. dc's, and it caught me within the hour they named it.**

48. **THE AVAILABLE CAUSE IS THE RECENT ONE, AND NOBODY ASKS WHAT ELSE WAS IN THE INTERVAL.**

49. **A CAPABILITY WITH TWO SPELLINGS HAS ONE EXISTENCE, AND THE INVARIANT CAN HOLD BY ABSENCE.**

50. **STORE CLAIMS SURVIVE A STALE BINARY AND BEHAVIOUR CLAIMS DO NOT (cc's formulation, via vc).**

51. **~~A shared file is not one you have a habit about~~ -- FOLDED INTO W39**

52. **A PARK IS ONLY CLEAN IF IT PARKS THE ASSERTIONS WITH THE CODE. dc's, and they claimed it.**

53. **A TRANSIENT BROKEN BUILD IS NOT OBSERVABLE AS TRANSIENT FROM OUTSIDE. dc's sharpening of my own note.**

54. **THE HEADLINE OF A CRITERION IS NOT ITS CONTENT -- THE GLOSS IS.**

55. **A RE-BUCKET IS THE ROSTER WORKING, AND THE RED IS THE MECHANISM RATHER THAN THE COST.**

56. **I CARRIED THE TWIN OF A DEFECT I WAS ABOUT TO CORRECT SOMEONE ELSE FOR.**

57. **A GUARD THAT IS PRESENT BUT AIMED AT THE WRONG SUBJECT IS INVISIBLE TO EVERY SEARCH FOR A MISSING GUARD (mine, 2026-09-04, and the issue itself had it wrong too).**

58. **AN ENUM BOUNDS ONE ARGUMENT GRAMMAR, NOT THE VERB -- AND I QUOTED A BLAST RADIUS FROM THE WRONG ONE (mine, 2026-09-04).**

59. **AN OUTPUT THAT IS INVARIANT OVER THE INPUT IS A DEFECT NO COMPARISON CAN DETECT, AND IT DECIDES WHAT THE FIX CAN BE.**

60. **~~My ad-hoc probe and the shipped client guessed the same field name wrong~~ -- FOLDED INTO `W17`**

61. **A NEGATIVE CONTROL RUN ONLY TO VALIDATE AN INSTRUMENT FOUND A DEFECT THE INSTRUMENT WAS NOT LOOKING FOR.**

62. **A PUBLISHED FIGURE IS NEVER COMPARED TO THE ARTEFACT IT DESCRIBES, AND THREE OF FOUR NODES DRIFTED THE SAME WAY (cc found it, on my commit).**

63. **A SHARED-RESOURCE FAILURE PRESENTS TO EVERY PARTICIPANT AS AN INDIVIDUAL PROBLEM, AND THE INDIVIDUAL READING IS THE ONE THAT SUPPRESSES THE REPORT (mine, 2026-09-04; vc took it onto their board).**

64. **THE TEST FOR A WORKAROUND IS NOT WHETHER A GUARD PROMPTED THE CHANGE -- IT IS WHETHER THE CHANGE WOULD STILL BE RIGHT WITH THE GUARD DELETED (vc's ruling, narrowing their own earlier one).**

65. **`grep -q` RETURNS 2 ON A MISSING FILE AND THAT OVERRIDES A MATCH, SO A RETRY LOOP MISCLASSIFIED A LOCK AS A REAL FAILURE (mine, 2026-09-04).**

66. **BOTH HALVES OF A PROMISE CAN EXIST, IN THE SAME FILE, WITH NOTHING JOINING THEM -- AND THE SUITE READS GREEN BECAUSE EACH HALF IS CORRECT (mine, 2026-09-04, answering `0244`).**

67. **A ZERO IS THE EASIEST NUMBER TO GET WRONG, SO A NULL RESULT NEEDS A CONTROL MORE THAN A POSITIVE ONE DOES (mine, 2026-09-04).**

68. **THE REASON TWO HALVES ARE UNJOINED MAY BE THE VERY PROPERTY THAT BLOCKS JOINING THEM -- SO SIZING _JOIN THESE TWO TESTS_ MEANS CHECKING BOTH HALVES ARE DRIVABLE, NOT BOTH PRESENT (mine, 2026-09-04, sizing `0244`).**

69. **A SIZE FOR ONE INSTANCE IS NOT A SIZE FOR THE CLASS, AND THE CLASS IS THE THING PEOPLE QUOTE.**

70. **A HANDOVER WRITTEN INTO AN ARTEFACT'S BODY IS NOT A HANDOVER, AND THIS ONE WAS INVISIBLE TO ME FOR FIVE DAYS (found 2026-09-04 when cc told me; verified against the tool, not assumed).**

71. **I MADE AN EXCUSE FOR A PEER WITHOUT DRIVING IT, IN THE SAME MESSAGE WHERE I CORRECTED THEM ON SOMETHING I HAD DRIVEN (mine, 2026-09-04, refuted by cc).**

72. **THE MUTATION THAT COUNTS PRODUCES A PLAUSIBLE WRONG ANSWER, NOT AN OBVIOUSLY BROKEN ONE (mine, 2026-09-05, building `0244`'s S).**

73. **A TRANSIENT EDIT TO A SHARED FILE IS A SHARED-STATE WINDOW, NOT A PERSONAL ONE (cc's, 2026-09-05, on my `render.rs` mutation).**

74. **AN IDENTIFIER TYPED FROM MEMORY IS THE SAME FAILURE AS A TIMESTAMP TYPED FROM MEMORY, AND THE ADDRESSING CHANNEL HAS NO GUARD ON IT (mine, 2026-09-05).**

75. **TWO FAILURES THAT SHARE A SYMPTOM CAN HAVE NON-CROSSING REMEDIES, AND THE SHARED SYMPTOM IS EXACTLY WHAT MAKES THEM READ AS ONE SEAM (mine, 2026-09-05, refuted by cc and re-driven here).**

76. **I ASSERTED A TOOL'S SURFACE WHILE WRITING ABOUT SOMETHING ELSE, IT WAS QUOTED BACK APPROVINGLY, AND THAT MADE IT LOOK CORROBORATED (mine, 2026-09-05, measuring `0257`).**

77. **A COINCIDENCE THAT REPRODUCES THE NUMBER YOU ARE TRYING TO EXPLAIN IS THE MOST DANGEROUS OBJECT IN A RECONCILIATION (vc's confirmation, dc's near-miss, driven by me 2026-09-05).**

78. **WRITING DOWN WHAT YOU ARE COUNTING FINDS THE DISAGREEMENT BEFORE ANY COUNT DOES (mine, 2026-09-05, and the protocol is vc's).**

79. **A NODE-LOCAL IDENTIFIER QUOTED ACROSS NODES DOES NOT DANGLE -- IT RESOLVES, TO SOMETHING ELSE (dc's finding, relayed by vc, made concrete here 2026-09-05).**

80. **A CLAIM ABOUT ORDER NEEDS AN ARTEFACT THAT ORDERS IT, AND A COMMIT TIMESTAMP ORDERS COMMITS RATHER THAN KNOWLEDGE (vc's separation, 2026-09-05, and it stands against me).**

81. **REACHING FOR THE SHARPEST AVAILABLE VERSION OF YOUR OWN FAULT IS STILL INACCURACY, AND IT DISTORTS THE RECORD IN THE DIRECTION THAT LOOKS HUMBLE (vc's, about themselves, 2026-09-05).**

82. **A COUNT OF A LIVE ARTEFACT IS A MEASUREMENT WITH A TIMESTAMP, AND THREE OF US PUBLISHED ONE AS A PROPERTY (vc's, 2026-09-05) -- AND THE DISCRIMINATOR BETWEEN _TIMING_ AND _METHOD_ IS FREE (mine, driven here).**

83. **WHEREVER A DEFECT HAS A LOUD MODE AND A QUIET MODE, THE LOUD MODE TRAINS YOU AND THE QUIET ONE ACCUMULATES (mine, generalised out of the W-number split with vc 2026-09-05).**

84. **BACKTICKS INSIDE A DOUBLE-QUOTED `echo` ARE COMMAND SUBSTITUTION, AND THE COMMAND THAT RUNS IS THE ONE YOU WERE TALKING ABOUT (mine, 2026-09-05).**

85. **TWO PATTERNS THAT SHARE A CLAUSE ARE ONE INSTRUMENT RUN TWICE, AND I CALLED MINE INDEPENDENT (mine, 2026-09-05, and it is why I concluded confidently in the WRONG direction).**

86. **A PEER CHANNEL HAS NO TERMINATING CONDITION, AND TWO NODES WHO ARE BOTH FINISHED WILL KEEP GOING BECAUSE EACH IS WAITING TO SEE WHETHER THE OTHER IS (vc's, 2026-09-05, closing an exchange in which we had each said _nothing further_ twice and continued).**

87. **_NEWER THAN X_ AND _CONTAINS Y_ ARE DIFFERENT RELATIONS, AND A COMPARISON AGAINST ONE LANDMARK CANNOT SEE A THIRD POINT LYING BETWEEN (dc's error, driven and refuted by me 2026-09-05).**

88. **A CLAIM RELAYED BETWEEN NODES CAN CHANGE SUBJECT IN TRANSIT WHILE STAYING TRUE AT BOTH ENDS, AND THE FALSEHOOD IS CREATED BY THE TRANSFER (dc's, self-reported 2026-09-05, after I refuted the result).**

89. **A REFUSAL'S TEXT IDENTIFIES WHICH LAYER REFUSED, AND THAT IS THE DISCRIMINATOR BETWEEN A DEFECT AND YOUR OWN ENVIRONMENT (dc's, 2026-09-05, a near-miss they did not file).**

90. **A CLEANUP GUARD MUST BE CONSTRUCTED BEFORE THE FIRST LINE THAT CAN FAIL, NOT BEFORE THE THING IT CLEANS UP (mine, 2026-09-05, found by driving a claim about my own work).**

91. **A FOLD COMPRESSES, AND COMPRESSION CAN CHANGE A CLAIM'S CATEGORY WHILE PRESERVING ITS ATTRIBUTION (vc's, self-reported and retracted with provenance 2026-09-05).**

92. **A CLAIM THAT STAYED TRUE OF SOMETHING AND FALSE OF WHAT IT NOW NAMES HAS NO LOCAL TEST, SO THE VERIFICATION MOVE IS TO RE-DERIVE ITS ORIGIN RATHER THAN TO CHECK ITS CONTENT (vc's generalisation of my W88/W91 pairing, 2026-09-05).**

93. **A GENERATED FILE HEALS ITSELF ON THE NEXT RENDER AND A HAND-AUTHORED ATTACHMENT CARRYING THE SAME VALUE DOES NOT, AND THE TWO ARE INDISTINGUISHABLE IN THE TREE (dc's, 2026-09-05).**

94. **A COUNT HANDED TO YOU AS A DEFECT COUNT MAY BE A STATUS COUNT, AND IT REPRODUCES EXACTLY EITHER WAY (mine, 2026-09-05).**

95. **SEARCHING BEFORE FILING FOUND THE OWNER OF A QUESTION I HAD ALREADY SPENT THE EFFORT ON (mine, 2026-09-05).**

96. **I COMMITTED THE DEFECT I HAD JUST CORRECTED IN A PEER, INSIDE THE SAME ROW, ONE SECTION LATER (mine, 2026-09-05).**

97. **A FOLD CUTS BY WHERE SOMETHING LIVES, AND A HOLD'S CONDITION IS ABOUT WHAT IT SAYS (mine, 2026-09-05, caught mid-fold).**

98. **A RULING THAT IS RIGHT DOES NOT GET RE-EXAMINED, SO ITS PRICE RIDES ALONG UNCHECKED (vc's, on themselves, 2026-09-05, and I want it here because I was the instrument twice).**

99. **A FILED ISSUE'S STATED CAUSE IS NOT ITS POPULATION, SO CHECKING YOUR EXPOSURE BY THE CAUSE RETURNS NIL ON AN ESTATE THAT HAS THE DEFECT (mine, 2026-09-05, on `0273`).**

100.  **A SCHEMA PROBE WITH THE WRONG KEY NAMES IS INDISTINGUISHABLE FROM A MALFORMED ROW (vc's, 2026-09-05, and I hit it at boot the same day).**

101.  **A COUNT AND THE SENTENCE NAMING IT DRIFT APART, AND THE SENTENCE IS WHAT TRAVELS (mine, 2026-09-05, third and fourth instances in one day).**

102.  **THE COST OF CHECKING SELECTS WHICH CLAIM GETS CHECKED, AND IT TRACKS WHAT INSTRUMENT YOU ARE ALREADY HOLDING RATHER THAN WHAT MATTERS (vc's self-report, sharpened here, 2026-09-05).**

103.  **AN UNBOUNDED ENDORSEMENT IS THE DEFECT; NAME WHICH FINDING IT COVERS (vc's, 2026-09-05, the operational form of W102).**

104.  **A SUBJECT WRITTEN DOWN WORKS ON A READER WHO IS NOT TRYING; A DISCIPLINE WORKS ONLY ON A READER WHO IS (vc's, 2026-09-05, and it is the day's conclusion rather than one more entry).**

105.  **`git commit --only` COMMITS EXACTLY WHAT IT WAS HANDED AND REPORTS SUCCESS, SO A RENAME COMMITTED BY ITS NEW PATH LANDS HALF (mine, 2026-09-05).**

106.  **A ROTTED RECORD IS ONE SOMETHING IS RESTING ON; A CLASS THAT CATCHES EVERY OUT-OF-DATE SENTENCE TRIAGES NOTHING (cc's, 2026-09-05, arrived at by REFUSING a correction I offered them).**

**cc MADE IT A FOLD OPERATION AND THAT IS THE PART THAT CHANGES PRACTICE (`2797abb2`): the boundary test runs FIRST, ahead of any archiving by date.**

107. **THE BLINDNESS THAT BELONGS TO NO AUTHOR, NO REVIEWER AND NO CHECK IS THE ONE THAT SURVIVES CARE (mechanism mine, ranking cc's, 2026-09-05 -- and cc asked for the split to be recorded exactly, having first had all seven as instruments).**

108. **EDITING ONE OF AN ADJACENT PAIR IS THE OPERATION THAT CREATES A CONTRADICTION, AND THE FIXER IS MORE EXPOSED THAN THE AUTHOR (mine, 2026-09-05, committed BY ME while removing this exact class from the same page).**

109. **`event_log` GIVES THE FILING INSTANT THAT AN mtime CANNOT, AND STILL DOES NOT GIVE THE ACTOR (mine, corrected by cc before it could be used, 2026-09-06).**

110. **BOTH RUST CRATES HAVE ONE `suite` TEST TARGET, AND THE HALF OF THIS ENTRY THAT SAID OTHERWISE WAS AN ASSUMPTION I WROTE AT 12:15Z AND DISPROVED AT 13:0xZ (mine, same session).**

111. **THE macOS APP'S `CFBundleShortVersionString` RESOLVES TO THE LITERAL STRING `dev` (cc's measurement, handed to me 2026-09-08 because `ST0064` is mine).**

112. **~~Check the arm that claims to be the load-bearing one first~~ -- FOLDED INTO `W44`**

113. **AN ABSENT COLUMN IS INVISIBLE TO EVERY CHECK THAT READS ROWS, AND ITS INVERSE IS A QUESTION NOBODY ASKS THE ARTEFACT (mine + cc + vc, 2026-09-09, and the pair is the durable half).**

114. **A GREEN OVER AN ENUMERATED AXIS IS EVIDENCE ABOUT THAT AXIS ONLY, AND A DOCSTRING SAYING _over the corpus_ DOES NOT WIDEN IT (mine, 2026-09-09, `nav_is_the_shared_path_contract.rs`).**

115. **A GUARD WHOSE _INPUT_ IS MISSING PRINTS ITS _SUBJECT IS BAD_ PROSE, AND THE TWO ARE INDISTINGUISHABLE AT THE TERMINAL (`0294`, mine, 2026-09-09).**

     **TWO CURES, AND THE SECOND IS THE DURABLE ONE.**

     **AND THE TELL THAT SCALES: THE PRESCRIBED REMEDY COULD NOT CLEAR THE REFUSAL.**

116. **ROUTING IS A MESSAGE I SEND, NOT A STATE I RECORD -- AND IF THE ONLY ARTEFACT IS ON MY OWN BOARD, NOTHING HAPPENED (cc's find, mine to own, 2026-09-09).**

     **THIS IS NOT THE hv-INBOX SHAPE AND CALLING IT THAT WOULD LET ME OFF.**

     **CURE, AND IT IS ONE LINE: a release is not written until a message is sent or an inbox entry is appended. The board records that it happened; it is never the thing that happened.**

- **W122 -- A TODO THAT NAMES THE REMAINING SCOPE WITHOUT NAMING THE ARTEFACT ALREADY PRODUCED SENDS THE NEXT PICKUP TO RE-DERIVE IT, AND THE NEXT PICKUP WAS ME (2026-09-09 20:48Z).**

- **W123 -- A SILENT `git add` IN A RETRY LOOP TURNS EVERY ITERATION INTO A NO-OP, AND THE LOOP REPORTS PATIENCE (2026-09-09 21:57Z). SECOND TIME TODAY, DIFFERENT MECHANISM, IDENTICAL SIGNATURE.**

- **W124 -- I ATTRIBUTED A FILE CHANGE TO A PEER WITHOUT DRIVING PROVENANCE. TWICE IN TWELVE HOURS, AND THE SECOND TIME COST A PEER A DEFENCE (2026-09-10 07:02Z).** First: credited dc with a `project.pbxproj` edit Xcode had written. Second: the gate refused my fold, I ran `git diff --cached`, saw `parity/tools/lib_currency.sh` staged IN THE SAME LISTING as cc's board, and told cc their file was refusing the tree. **`git log` on it returns ZERO commits -- brand new, never committed by anyone -- and cc had to write their own evidence to correct me.** **THEIR DIAGNOSIS THEN BEAT MINE: a PARTIAL STAGE.** The tool is staged `A `; the three modified tools including its roster row are ` M` and unstaged. **The decisive measurement I had not thought to take: the `lib_currency` row reads 1 ON DISK, 0 IN THE INDEX**, positive-controlled against `lib_staged` at 2/2. Mine was the symptom, theirs the cause. **RULE: a dirty file is not evidence of who dirtied it, and a `--cached` listing is a set of PATHS, not a set of AUTHORS -- adjacency inside it proves nothing.** `git log <path>` and `git diff --cached -- <one path>` are one command each and I ran neither before naming a node, having written the first half of this rule twelve hours earlier. **AND vc NAMED THE MECHANISM, WHICH IS BETTER THAN MY DIAGNOSIS OF MY OWN CARELESSNESS: the guard names an INSTRUMENT, an instrument has an obvious owner, and a STAGE does not.** So the message steers its reader toward _whose tool is this_ when the answer needed is _whose stage is this_. **That is a property of the MESSAGE, not of the reader** -- which is why the fix is a rule about what a guard's wording does to whoever reads it, and why _pay more attention_ would have been the useless version of this entry.

## Decisions

- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv 2026-09-01, via vc). Everything outstanding goes in; no external consumer; scarcity register retired as a class.
- **A DECISION TAKEN ON SOMEONE'S BEHALF AND RECORDED AS SUCH IS DELEGATION; THE SAME DECISION RECORDED AS A FACT IS A SUBSTITUTION** (vc, ratifying two of mine). What made Esc-enters-vi-normal legitimate was naming the alternative, writing it into the design _as_ a decision made on hv's behalf, and telling hv in chat -- not the reasoning behind it.
- **A ROW WITH TWO SUBJECTS CANNOT STATE ITS OWN REACH** (vc, four times in one day, and the tool caught the fourth). `AC-17.17` was minted test-backed while its own reach clause said no test could witness it; the contract REFUSED the pair. Split: `AC-17.17` the ORDER (test-backed, satisfied), `AC-17.18` the PAINTING (non-test, **SATISFIED -- hv drove it in a real terminal 2026-09-02 and the capture is on the row**; this entry said _open_ until 2026-09-03, see W24). **The discriminator is who holds the screen, not which variant it is** -- a borrower that blocks needs no signal, one that returns does.
- **ST0064 WP-01 = 5/9 (ic), RE-DERIVED 2026-09-05 AND STILL TRUE -- `intent ac list ST0064` is the authority, not this line.** 01.1/01.3/01.5/01.8/01.9 done; 01.2/01.4/01.6/01.7 cc/dc/hv-gated. project-CWD wiring; vc RULED per-app-instance root, D07 registry UNBUILT.
- **`AC-17.1` IS SATISFIED AND THE _UNSATISFIABLE IN 3.0.x_ READING IS RETIRED (2026-09-03, measured not argued).** This entry said 17.1 could not be met while the wire was reads-only. cc built `Op::Set` overnight and `AT-17.1` is green. **`EmptyMutation` staying is not a counter-argument -- it is `Op::Graphql`'s bound and it is deliberate; the mutating door is a DIFFERENT door.** Kept as a decision rather than deleted, because three boards and `intent/wip.md` carried the dead reading and a reader may still meet it. **`AC-17.6` is the one open row in WP-17 and is still gated on cc's WP-08** -- `browsed()` refuses unconditionally, and cc has taken the missing half, unstarted. **No figure here: run `intent ac list ST0056`.**
- **WP-14 (whiteboard + inboxes in the store, `L`) -- THE QUESTION IS LIVE AND IT IS NOT WHETHER IT RETURNS.** hv reinstated `AC-00.2`/`AC-10.5` into ST0056 at `a19d8b5c` (driven: ancestor of HEAD, both rows `computed`). **`ST0056/14` itself still reads `Not Started`/`L`**, and those rows are about the FLEET CORPUS, not the coordination model -- so _WP-14 is in the cut_ does not follow from their subject. vc accepted that as a defect in the menu they put to hv. **Bears on `0258`, whose ruling is gated behind exactly this.**
- **A SIZE IS A CLAIM ABOUT THE WORLD EXACTLY AS MUCH AS A COUNT IS** (mine, taken onto cc's board 2026-09-03). An evidence bar held for figures but not for t-shirt sizes is the asymmetry that hides itself, because only one of the two wears a number.
- **`init` LAYS `AGENTS.md` DOWN** (hv 2026-09-09, ruling on ST0065's costed proposal). Option 2's index landed in `CLAUDE.md` at `0b5d46c96`, so the content argument is spent -- but `AGENTS.md` is the TOOL-AGNOSTIC contract, and a fresh project had none until someone ran a verb nothing told them to run while `CLAUDE.md` said _read AGENTS.md first_ four times. **Ruling 2 (what `AGENTS.md` mirrors) is DEFERRED: it gates only Option 3 and blocks nothing.** Ruling 3 was already dead -- built and green since 2026-09-04.
- **`ST0065/WP-02` COLLAPSES INTO `ST0056/WP-15`** (hv 2026-09-09). Cancelled with reason via `intent wp cancel`. Two homes for one concern, both WIP, found before either party sank effort.
- **`uninstall` PROMISES THE FILES IT WROTE, NOT THE DIRECTORY IT EMPTIED** (hv 2026-09-09, `0218` remedy 1).
- **A RULING RECORDED ONLY IN CODE IS INVISIBLE TO EVERY BOARD-LEVEL RE-DRIVE** (mine, adopted by vc into the standing recipe 2026-09-09). ST0065's ruling 3 was answered, built and green on 2026-09-04 and survived the issue tracker, the boards AND the gates, because its only home is a doc comment on a test. **vc's expiry defect is an item outliving its REASON; this is an item outliving its ANSWER**, and it fails in the direction that costs most -- a DEAD item passing as live indefinitely. **Fourth line of the re-drive recipe: for any ruling whose answer would be BUILT, grep the estate for the thread id before believing the board.** **FIFTH LINE, DRIVEN AT THIS BOOT 2026-09-09 20:24Z, AND IT IS THE SAME DEFECT ONE RUNG UP: A RULING RECORDED ON ONE BOARD IS INVISIBLE TO EVERY HOME THAT SAYS IT IS UNMADE, AND FOUR OF THEM DO.** hv ruled `ST0065`'s set on 2026-09-09 -- `6d60bc516`, _hv ruled four of six_ -- and the POSITIVE CONTROL is the sibling ruling's artefact rather than the record of it: `intent wp list ST0065` reads WP-02 **Cancelled**, so the pass demonstrably happened and produced an executed outcome. **The live value has ONE home, this board's line above. FOUR carry the opposite**: `.claude/restart.md:18`, `intent/whiteboard/hv/wip.md:73`, `intent/wip.md:85` and `intent/whiteboard/hv/inbox.vc.md:1250`, every one still reading _ic takes WP-01 the moment they land_ while `WP-01` has been WIP since morning. **THE WORST OF THE FOUR IS THE ENTRY POINT, WHICH EVERY NODE READS AT EVERY BOOT, AND IT TOLD ME THIS MORNING THAT I WAS BLOCKED ON RULINGS I HAD RECORDED MYSELF** -- it cost nothing only because my own board carried the answer and I read both. `intent/wip.md:83` is stale a second way: it calls WP-02 WIP. **RULE: for a ruling whose answer would be RECORDED rather than BUILT, grep the estate for the thread id before believing the entry point. The direction that costs most is a LANDED ruling still reading as owed, because it parks a node that could be working, and nothing anywhere goes red.**
