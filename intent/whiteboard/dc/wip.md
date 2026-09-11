---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-11 21:26Z
status: active
focus: "HOLDING. The doc audit is done and pushed on hv approval: main at 1ebd57700 on both remotes, the tap at c0e6ed9, the pair delivered at 2c3a7d2d4. No new work until hv rules on the defect list in intent/wip.md. NO FIGURE HERE IS EVIDENCE; RUN THE VERBS."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058]
---

# DevX Claude (dc)

**COLD-SESSION MINIMUM. A rule is never dropped here, only its narrative.** Today's reasoning: `.history/20260831/` -- `census-narrative-0931Z.md`, `day-narrative-1049Z.md`, `ac0201-narrative-1300Z.md`, `decisions-fold-0723Z.md`. **Every Watch-out and lesson below is its HEADLINE ONLY since the 2026-09-11 17:17Z fold; the full text of each, by number, is verbatim in `.history/20260911/wip-prefold-1717Z.md`. Read it there before citing a rule's detail.**

## D42 -- TIME. Read before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **not the database.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES: NO cli or intentsvcs function TAKES a time.** They may RETURN times.
- **A board stamp is a label, not data** -- read from `date -u` and PASTE, **per stamp, never per session.**
- **SAME TURN IS NOT ENOUGH -- THE VALUE MUST COME FROM THE READ.** 2026-09-04 19:48Z: I put `date -u` and a hand-typed `19:12Z` in the SAME tool call, so the clock output and the fabricated stamp printed side by side, 36 minutes apart. **The read was present and the write did not come from it.** D42 says paste; the failure mode is composing the write BEFORE the read and never closing the loop. It reached the worktree and no commit, and nothing but reading the two lines together would have caught it.
- **`stat`, `git log`, `ls -la` ALL PRINT LOCAL.** Appending `Z` to a local read is an ASSERTION, not a format.
- **A ULID is an IDENTITY and the `ts` is the STAMP** -- why `Envelope::minted` may generate the id in Rust without breaching D42.

## The truth model and the environment

- `design.md` (D01 reversed) + `data-model.md`. **The db is the durable SSOT; the typed API is the only door in.** Crates: `intent-cli`, `intentd`, `intentsvcs`. **`intentdb` IS RETIRED.**
- **`intent` ON PATH IS v3 AND RESOLVES INTO THIS TREE** -- `~/.local/bin/intent` and `~/bin/intent` both symlink `native/rust/target/release/intent`. **A release build in progress makes it transiently ABSENT**, which reads as "not installed".
- **THE STORE IS PROJECT-RELATIVE**, `intent_dir()/.cache/intent.db`, found by walking up from CWD. **THE SCHEMA NUMBER IS NOT PRINTED HERE ANY MORE -- IT SAT AT 16 WHILE THE STORE WAS AT 17**, which is my own standing rule catching me in my own environment section. Read it: `sqlite3 intent/.cache/intent.db 'PRAGMA user_version;'`, and the source's value with `grep -m1 'pub const SCHEMA_VERSION' native/rust/crates/intentsvcs/src/store.rs`.
- **A TEST BINARY BUILT FROM THIS TREE OPENS THIS TREE'S LIVE STORE, WHATEVER CWD OR HOME YOU GIVE IT** (2026-09-11: cc's intentsvcs run migrated the live store 17 -> 18 and every 17 binary refused the project until hv's rebuild). `testkit::repo_root()` walks up from the COMPILE-TIME `CARGO_MANIFEST_DIR` (`testkit/src/lib.rs:61`, `:128`; verified), so the root is the source tree the binary was compiled from. `attachment_drift_detected.rs` runs doctor on it and `Store::open` migrates. **Compile and run tests only from a worktree's own sources; never invoke a test binary built here.** A private `CARGO_TARGET_DIR` does not help, because the path baked in is the source's. Before the first suite run in a new worktree, confirm the store it creates is `<worktree>/intent/.cache/intent.db`.
- **THE INDEX IS SHARED.** `add` + `commit --only <paths>` in ONE call is the only safe write.
- **CURRENCY IS A COMMAND HERE, NEVER A VALUE, BECAUSE THE VALUE ON THIS LINE WENT STALE AND I QUOTED IT TO hv.** The line used to say the pair was behind HEAD as of 2026-08-31 16:54Z. It was rebuilt at `4be902e1` and I reported the stale claim anyway, from memory, two days later. **THE ARM THAT DECIDES IS NOT THE PIN-VERSUS-HEAD DIFFERENCE** -- that difference is normal and says nothing, because the pin names the last commit touching BUILD INPUTS and HEAD runs ahead over commits that compile nothing. Run this and read its last line: `bash intent/st/ST0056/parity/tools/self_provenance_check.sh`. It prints `currency ok` or `currency REFUSING` and names the files that moved. **`intentd --version` prints NO commit where `intent --version` does, so the pair cannot be compared through `--version` at all**; the embedded marker is the only route, and the sha256 on each line is what distinguishes one build from another because the marker does not. **Do not rebuild into the shared path with sessions live -- `0196`.** **CORRECTED 2026-09-02: this line used to end _A NODE MUST NOT TAKE A REBUILD WINDOW; it is hv's_, and that was FALSE.** I originated it and vc repeated it to hv all evening. Driven: **`0196` is a DEFECT** -- `guarded_release_build` deletes the shared pair BEFORE it builds and no failure path restores it -- **not an authority constraint**, and ic has rebuilt twice today without hv. **The real constraint is the HAZARD: a failed build leaves every live session on this machine without a binary, so a rebuild wants a QUIET TREE, not a permission.**
- **CANON ORDER: `intent st attach <ST> <THREAD-RELATIVE path> --from <file>` FIRST, then commit the file and canon together.** Read the existing spelling out of canon first: `jq -r '.attachments[]?|.path' intent/.canon/st/<ST>.json`. **AND `sync --to-store` IS NOT THE ROUTE WHILE intentd WATCHES THIS TREE** -- it refuses, correctly, and the daemon has already ingested a canon edit and re-rendered the views by the time you look. Verify PAST the ingest, never at the `ok:`.
- **SEVERAL BEHAVIOURS NOW DIFFER BETWEEN THE TREE AND THE SHIPPED PAIR, SO WHICH ONE YOU GET DEPENDS ON WHICH BINARY YOU ARE STANDING ON.** Measure it, do not read this list: `strings native/rust/target/release/intent | grep -o '\[intent-source-commit:[^]]*\]'` against `git rev-parse HEAD`. At the 2026-09-05 cut the pair held `da5919e8` while HEAD was `09bc0eb4`. Fixed in the tree and NOT in that pair: `0262` (a repo-relative attach is refused with the corrected spelling computed for you -- this line used to say it was ACCEPTED, which was true for weeks), `0242`, `0270` (`at green/red` refuses a verdict citing an absent file), `0273` (`at lint` reports rows EXAMINED, not walked) and `0267`'s narrow half (an id on a line naming another thread is not coverage). **Kept as corrections rather than deleted: the old readings are the shape that misleads on restart.**
- **`INTENT_HOME` IS v2's VARIABLE AND v3 DOES NOT READ IT** (`0277`). The install root is resolved by walking up from the executable's own canonicalised path to a `lib/templates` marker; `resolve` takes an executable and nothing else, and a second guard refuses the `env::var` that would feed it. **v3 reports its resolved root AS `INTENT_HOME` in `intent info` -- output, not input** -- which is why it reads like a knob. Setting it will not repair a keg and will not point a binary at a different tree. `install.md` told readers twice that it would.

- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND NEVER TRANSCRIBED.**
- **THE PROVENANCE PIN IS NOT HEAD AND THAT IS BY DESIGN.** `emit_source_commit` runs `rev-list -1 HEAD` over `:(top)native/rust :(top)surface :(top)docs/design`, so it names the last commit touching the BUILD'S INPUTS. **`intentd --version` prints NO commit where `intent --version` does, so the pair cannot be compared through `--version` at all** -- the embedded marker is the only route. `dvb build all` forces the embeds and verifies the SET, which is the check cargo cannot make.
- **THE BASH TOOL'S SHELL IS zsh.** **BACKTICKS INSIDE A DOUBLE-QUOTED COMMIT MESSAGE RUN AS COMMAND SUBSTITUTION AND SILENTLY EAT THE WORD** -- 2026-09-03, `stated` vanished from a landed commit message and the only symptom was one `command not found` line among 40 lines of green guard output. **A message goes in a FILE and through `-F`.** No word-split on unquoted `$var`; an unmatched glob aborts the command; **`mapfile` DOES NOT EXIST**, and in an `&&` chain its failure silently skips every later step, leaving a previous run's file to be read as this run's answer. **Write anything non-trivial as a `#!/usr/bin/env bash` script file.**

## DOING

**Nothing in flight.** The doc audit is done and pushed on hv's approval: main is at `1ebd57700` on both remotes, and the tap is at `c0e6ed9` (origin/main read back 2026-09-11 21:26Z). The pair was delivered at `2c3a7d2d4` through the guard fix `bc696da63`. What dc did in the audit is in those commits and in vc's globalfold.

**With vc to close:** `0150` (`c9960b90`) and `0065` (`5493dd28`). Re-drive `intent issues show` before quoting either.

## TODO

**Nothing assigned.** Do not invent work.

## Holds

- **HOLD everything until hv rules on the defect list in `intent/wip.md`.** Condition: a ruling from hv, or a task from vc under hv's word.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD** (`W69`). Re-drive a hold's condition at the moment you quote it; never re-read it off this line.

## Watch-outs

**A MAP BY FAMILY, then every rule by number, HEADLINE ONLY.** Each rule's full text, its worked instances and the notes on merging (refused twice) and on the absent `W104` are verbatim in `.history/20260911/wip-prefold-1717Z.md`.

- **INSTRUMENTS AND CONTROLS -- the dominant class. An instrument that cannot exhibit the failure returns a clean answer.** -- W1, W15, W16, W20, W28, W33, W44, W46, W51, W60, W69, W75, W78, W82, W85, W90, W91, W92, W103, W120, W125, W111, W112, W128, W129, W130, W131, W132, W134, W135
- **POPULATION, SUBJECT, DENOMINATOR -- what was measured is not what was claimed.** -- W2, W22, W23, W27, W49, W52, W61, W65, W80, W87, W94, W95, W99, W113, W119, W121, W123, W126, W127, W140, W142, W143
- **SECOND HOMES AND DRIFT -- one fact, two copies, and nothing binding them.** -- W6, W10, W17, W34, W42, W47, W55, W58, W86, W110, W116, W117, W133
- **THE SHARED CHECKOUT AND PEERS -- five sessions, one tree, no authorship.** -- W3, W4, W45, W66, W68, W70, W74, W79, W81, W89, W96, W97, W98, W100, W101, W137, W139, W141, W144
- **CLASS VERSUS INSTANCE -- fixing the one you hit leaves the rest, and the fix can be the trap.** -- W5, W12, W21, W37, W40, W41, W50, W71, W72, W73, W105, W114, W115, W118
- **READING, REPORTING AND REMEDIES -- what the output said versus what it meant.** -- W8, W9, W19, W24, W25, W29, W35, W36, W38, W54, W56, W67, W83, W84, W88, W93, W102, W106, W107, W108, W109, W124
- **ENVIRONMENT AND MECHANICS -- things that return a plausible wrong answer here.** -- W7, W11, W13, W14, W18, W26, W30, W31, W32, W39, W43, W48, W53, W57, W59, W62, W63, W64, W76, W77, W122, W136, W138

- **W1 -- THE INSTRUMENT ANSWERED A DIFFERENT QUESTION THAN THE ONE ASKED, AND ITS OUTPUT LOOKED LIKE AN ANSWER.**
- **W2 -- THE MEASUREMENT'S SUBJECT WAS NOT THE ONE I NAMED.**
- **W3 -- OWNERSHIP IS MEASURED, NEVER INFERRED, AND IT HAS THREE TELLS.**
- **W4a -- THE INDEX CLASS HAS THREE MEMBERS AND ONLY ONE IS CLOSABLE BY THE COMMITTER**
- **W4 -- THE SHARED CHECKOUT PUNISHES EVERY ASSUMPTION ABOUT WHO ELSE IS WRITING.**
- **W5 -- A RESTRICTION THAT REDS NOTHING HAS NOT BEEN ADDED.**
- **W6 -- A SECOND HOME FOR A FACT DRIFTS, AND THE PATTERN IS PROXIMITY, NOT DISTANCE**
- **W7 -- MECHANICS THAT RETURN A PLAUSIBLE WRONG ANSWER.**
- **W8 -- HOW TO REPORT.**
- **W9 -- A REMEDY WHOSE STATED REASON DOES NOT HOLD IN YOUR CASE IS NOT PERMISSION**
- **W10 -- I DOCUMENTED THE CLASS AND FIXED THE INSTANCE, AND THE COMMENT MADE IT LOOK CLOSED.**
- **W11 -- I ARGUED FROM HOW THE SYSTEM BEING REPLACED WAS BUILT**
- **W12 -- I PROVED SOMETHING TRUE AND BESIDE THE POINT, AND EVERY STEP WAS SOUND.**
- **W13 -- A RULE OR CITATION TAKING ITS AUTHORITY FROM A NAME RATHER THAN FROM ITS MEMBERSHIP RULE.**
- **W14 -- A CENSUS THAT ENUMERATES MEMBERS CANNOT SEE A DEPENDENCY HELD BY THE CONTAINER, AND THE CONTAINER IS WHERE THE FATAL ONE LIVES.**
- **W15 -- AN INSTRUMENT'S NEGATIVE DIRECTION, ONCE SHOWN UNTRUSTWORTHY ANYWHERE, WITHDRAWS EVERY ZERO IT EVER RETURNED**
- **W16 -- I CONTROLLED THE NEW INSTRUMENT ONLY IN THE DIRECTION THE OLD ONE FAILED, AND MY NEGATIVE CONTROL WAS ONE THAT COULD NOT FAIL.**
- **W17 -- A FACT WITH TWO HOMES, WHERE EITHER CAN OUTLIVE THE OTHER AND THE GUARD IS THE ONLY THING THAT NOTICES**
- **W18 -- AN ARTEFACT REWRITTEN UNDER THE CHECK THAT READS IT LEAVES THE OPERATOR WITH NO PROGRESS AND NO DIAGNOSIS**
- **W19 -- A DRIVE THAT STOPS AT ITS FIRST FAILURE REPORTS ONE DEFECT AND HIDES EVERY ONE BEHIND IT.**
- **W20 -- A BOOLEAN PROBE HIDES ITS EVIDENCE.**
- **W21 -- WHEN EACH FIX FINDS ANOTHER DEFECT, THE METHOD IS THE DEFECT, NOT THE ENTRIES.**
- **W22 -- A COUNT OF DIFFERENCES IS NOT A COUNT OF DAMAGE**
- **W23 -- NAME THE TREE / THE BINARY / THE DENOMINATOR, OR THE CLAIM CANNOT BE DISAGREED WITH.**
- **W24 -- A GREP OVER A SERIALISATION ANSWERS A QUESTION ABOUT TEXT WHEN THE QUESTION WAS ABOUT STRUCTURE.**
- **W25 -- AN EDIT THAT REPLACES A LINE RANGE DELETES WHAT IT DID NOT READ.**
- **W26 -- A PARTITION BUILT FROM THE DIRECTION YOU EXPECT THE DRIFT TO RUN HAS NO BUCKET FOR THE OPPOSITE DIRECTION, AND THE UNNAMED CASE SORTS TO THE WRONG ONE IN SILENCE**
- **W27 -- A FIGURE WITH NO HOME THAT DERIVES IT SURVIVES ON RESTATEMENT, AND EACH RESTATEMENT MAKES IT LOOK BETTER SOURCED.**
- **W28 -- THE INSTRUMENT ITSELF CAN GO ABSENT, AND ITS ABSENCE READS AS AN EMPTY ANSWER**
- **W29 -- A WITHDRAWAL IS A CLAIM, AND IT NEEDS THE SAME EVIDENCE AS THE FINDING IT RETRACTS.**
- **W30 -- A PARKED ARTEFACT IS A MEASUREMENT WITH A TIMESTAMP ON IT, AND THE TREE MOVES UNDERNEATH IT**
- **W31 -- A REWRITTEN ARCHIVE LOOKS CORRECT AFTERWARDS, WHICH IS WHY ONLY A GUARD CAN SEE IT**
- **W32 -- THE ESTATE MAY ALREADY OWN YOUR FINDING, AND A REDISCOVERY IS INVISIBLE TO YOU BECAUSE YOU SEARCHED YOUR MEMORY INSTEAD OF THE REGISTER.**
- **W33 -- A BROKEN INSTRUMENT THAT RETURNS THE SAME ANSWER ON BOTH ARMS READS AS A CLEAN TWO-SIDED CONTROL.**
- **W34 -- DOING THE WORK IS NOT REGISTERING IT, AND THE INSTRUMENT READS THE REGISTER.**
- **W35 -- A BYTE CHECK VERIFIES THE QUOTE, NEVER THE CLAIM, AND A PAGE CAN CARRY YOUR STRING WHILE SAYING THE OPPOSITE.**
- **W36 -- A GUARD ARM'S NAME DESCRIBES THE REFUSAL, NOT WHAT FOLLOWS IT. READ THE BODY.**
- **W37 -- STATING THE TRUE HALF OF A DEFECT IS NOT STATING THE DEFECT, AND IT IS WORSE THAN SILENCE.**
- **W38 -- WHEN YOU PROPOSE A MECHANISM, CHECK WHETHER ANYTHING YOU ALREADY RAN BEARS ON IT. YOU OFTEN HOLD THE DISPROOF, FILED UNDER A DIFFERENT QUESTION**
- **W39 -- TWO INDIVIDUALLY-CORRECT RULES CAN COMPOSE INTO A DEADLOCK WHOSE ONLY SYMPTOM IS THAT NOBODY COMMITS.**
- **W40 -- A CLASSIFICATION DERIVED FROM ONE AXIS CANNOT EXPRESS A FACT ON ANOTHER, AND A COMPLETE-LOOKING REGISTER IS WHAT STOPS ANYONE LOOKING.**
- **W41 -- ONE WORD NAMING TWO ARTEFACTS SENDS TWO NODES HUNTING A DISCRIMINATOR THAT DOES NOT EXIST, AND THE EXPERIMENT COMES BACK CLEAN.**
- **W42 -- AN ERROR MESSAGE'S STATED CAUSE IS A SECOND HOME FOR THE REFUSAL LOGIC, AND IT DRIFTS LIKE ANY OTHER COPY.**
- **W43 -- A DEFECT THAT ONLY BITES THE OUTERMOST LAYER IS INVISIBLE IN EVERY TEST THAT NESTS IT.**
- **W44 -- A FAILING TEST HERE CARRIES ITS OWN DIAGNOSIS, AND I BUILT A MECHANISM INSTEAD OF READING IT.**
- **W45 -- I CORRECTED A PEER OFF MY LIVE BOARD WHILE THE CLAIM SAT IN MY OWN ARCHIVE.**
- **W46 -- A METER'S ONLY SELF-CHECK FIRES IN THE DIRECTION THAT MEANS "MY MODEL BROKE" AND IS SILENT IN THE DIRECTION THAT MEANS "ALARM".**
- **W47 -- A PEER'S NUMBER THAT I RECORD FOR DURABILITY GAINS A SECOND WITNESS, AND THE SECOND WITNESS IS WHAT SURVIVES THE AUTHOR'S RETRACTION.**
- **W48 -- MY FOLD CONTROL PROVES NOTHING WAS LOST AND IS STRUCTURALLY BLIND TO WHAT WAS ADDED.**
- **W49 -- A COUNT AND A CHANGE DRAWN FROM DIFFERENT POPULATIONS IS THE SAME DEFECT AS A COUNT NOBODY DROVE, AND IT MUTATES.**
- **W50 -- A DEFECT NAMED AFTER THE SPELLING OF ONE INSTANCE GETS A FIX SHAPED LIKE THAT SPELLING, AND EVERY CENSUS DOWNSTREAM INHERITS THE BLINDNESS RATHER THAN REPORTING IT.**
- **W51 -- AN INSTRUMENT THAT CANNOT SEE A CLASS AT ANY LEVEL OF CARE, AND A CURE THAT NAMES IT WITHOUT MAKING ANYONE RUN IT.**
- **W52 -- THE INSTRUMENT'S POPULATION IS NOT THE CLAIM'S.**
- **W53 -- A RED ABOUT THE ENVIRONMENT IS INDISTINGUISHABLE FROM A RED ABOUT THE SUBJECT.**
- **W54 -- A LANDING'S HALVES ARE NOT ENUMERABLE BY READING.**
- **W55 -- AN ASSERTION WHOSE COMPLIANCE LEAVES NO TRACE CANNOT BE REVIEWED, SO NOTHING EVER DISCOVERS THAT IT STOPPED BEING TRUE.**
- **W56 -- A VERIFICATION SENTINEL THAT IS NOT UNIQUE TO THE WRITE CANNOT FAIL, AND A READ-BACK KEYED ON A DURATION IS A SMUGGLED CONSTANT.**
- **W57 -- A SHARED-ARTEFACT REBUILD IS A COORDINATION PROBLEM WITH AN UNBOUNDED FAILURE MODE, AND THE RESTORE MUST EXIST BEFORE THE START.**
- **W58 -- A REMEDY FALSIFIES ITS OWN ROW'S PREMISE, AND THE NEXT RULING IS MADE OFF THE ROW.**
- **W59 -- ONE DIRECTORY CAN HAVE TWO OWNERSHIPS, AND A RULE APPLIED TO THE PATH IS WRONG FOR HALF OF IT.**
- **W60 -- A CHECK I WRITE FOR A READER IS AN INSTRUMENT WHOSE FAILURE I WILL NEVER SEE.**

- **W61 -- AN INSTRUMENT MEASURES A FIELD; A CLAIM IS ABOUT A SUBJECT, AND NOBODY NOTICES WHEN THEY COINCIDE ON MOST MEMBERS.**

- **W62 -- A FALSE PREMISE SITTING ON TOP OF A REAL DEFECT IS THE ONE SHAPE NO READING CAN RESOLVE, AND IT INVITES OPPOSITE DESTRUCTIONS.**

- **W69 -- AN INSTRUMENT WHOSE CONSTRUCTION EXCLUDES THE FAILURE IT IS AIMED AT RETURNS A CLEAN VERDICT THAT READS AS AN ANSWER.**
- **W67 -- A CORRECT ACCOUNT OF A CLASS, SITTING BESIDE A SURVIVING INSTANCE OF IT, SUPPRESSES THE SEARCH THAT WOULD FIND THE INSTANCE.**
- **W68 -- AN IN-FLIGHT TOOL-AND-ROSTER PAIR FREEZES EVERY PATH-SCOPED COMMIT IN THE REPOSITORY, AND THE POLITE READING OF THE QUEUE HIDES IT.**
- **W70 -- `--only` IS PATH-SCOPED, NOT HUNK-SCOPED: IT PROTECTS AGAINST AN UNNAMED FILE AND GIVES ZERO PROTECTION AGAINST AN UNNAMED HUNK INSIDE A NAMED ONE**
- **W64 -- TWO INDEPENDENTLY SUFFICIENT GUARDS ARE MUTUALLY UNKILLABLE, SO REDUNDANCY MAKES AN ABSENCE UNDETECTABLE**
- **W65 -- A POPULATION HANDED TO YOU BY A PEER IS A FOSSIL, AND RE-DRIVING IT FAITHFULLY IS NOT RE-MEASURING**
- **W66 -- A SHARED INDEX DESTROYS AUTHORSHIP AND THEN OFFERS ADJACENCY IN ITS PLACE, WHICH IS WORSE THAN OFFERING NOTHING**
- **W63 -- ORIGIN DOES NOT DETERMINE DISPOSITION, AND A DEBRIS SWEEP IS WHERE THAT BITES.**

- **W71 -- FIXING THE NARROW ARM OF A CLASS IS WHAT CREATES THE HAZARD: A GUARD THAT REPORTS A CLASS FOR ONE POPULATION, WHILE THE SAME CLASS RUNS UNREPORTED EVERYWHERE ELSE, MAKES ITS GREEN EVIDENCE FOR SOMETHING IT NEVER MEASURED**
- **W72 -- A PREDICATE NARROWED CORRECTLY, TO KILL A REAL FALSE POSITIVE, IS STRUCTURALLY BLIND TO THE NEIGHBOURING CLASS, AND THE NARROWING IS INVISIBLE AT THE CALL SITE.**
- **W73 -- A REMEDY KEYED TO _WHEN_ MISSES A DEFECT WHOSE MECHANISM IS _WHETHER_, AND CONTIGUOUS IDS ARE THE TRAP THAT INVITES THE TEMPORAL STORY**
- **W74 -- A VALUE YOU ARE PASSING ON IS ONE YOU HAVE NOT MEASURED, AND RELAYING IS THE ACT THAT LAUNDERS IT**
- **W75 -- A CONTROL THAT DOES NOT TRAVERSE THE CODE PATH PRODUCING THE VERDICT IS DECORATION, AND IT READS EXACTLY LIKE A CONTROL.**
- **W76 -- A PARK IS ONLY CLEAN IF IT PARKS THE ASSERTIONS WITH THE CODE, AND A COMPLETENESS CHECK SCOPED TO WHAT YOU TOUCHED CANNOT SEE WHAT YOU STRANDED**

- **W77 -- A SEPARATOR THAT IS A LITERAL BETWEEN TWO EXPANSIONS IS NOT A SPLIT POINT, EVEN WHEN IT IS IN `IFS`.**
- **W78 -- A CONTROL ASSERTS A DIFFERENCE, SO IT MUST BE DRIVEN AGAINST THE STATE IT CLAIMS TO DISTINGUISH FROM**
- **W79 -- A CONTROL THAT DEPENDS ON A LIVE FILE IS RETIRED BY ANOTHER NODE'S ORDINARY WORK**
- **W80 -- A DENOMINATOR COUNTING THE HITS SAYS NOTHING ABOUT THE WALK, AND THE WALK IS THE COST SIGNAL**
- **W81 -- A REPO-WIDE CONDITION ARRIVES AT EACH NODE AS A PERSONAL ONE, AND THE PERSONAL READING IS THE ONE THAT SUPPRESSES THE REPORT**
- **W82 -- AN INSTRUMENT THAT NARRATES WHAT IT DID PRODUCES ERROR-DETECTION IT WAS NEVER DESIGNED FOR**
- **W83 -- A "SAFE DIRECTION" ARGUMENT MUST NAME ITS BLAST RADIUS**
- **W84 -- DECLARING A REACH IS NOT WIDENING IT, AND THEY ARE TWO ACTS**

- **W85 -- CHECK THE HARNESS BASELINE AGAINST THE REAL RUN BEFORE YOU BELIEVE ANY MUTATION**
- **W86 -- WHEN A DOC COMMENT AND THE CODE'S OWN OUTPUT DISAGREE ABOUT REACH, THE OUTPUT IS THE SURVIVING HOME**
- **W87 -- A COMPLEMENT MUST NOT CLAIM MORE THAN THE EXAMINER**
- **W88 -- ON A LONG SESSION THE INSTRUMENTS DEGRADE FASTER THAN THE REASONING**

- **W89 -- A HANDOVER WRITTEN INTO A SHARED ARTEFACT WITH NO ADDRESSING IS A NOTE, NOT A HANDOVER, AND IT IS MINE.**
- **W90 -- AN ASSERTION OF ABSENCE CANNOT DISTINGUISH A CLEAN SUBJECT FROM A DEAD INSTRUMENT**
- **W91 -- A DISCRIMINATION CONTROL MUST BE DRIVEN IN BOTH DIRECTIONS OR IT IS DECORATION.**
- **W92 -- RUN THE DIRECTORY'S OWN INSTRUMENTS AGAINST THE INSTRUMENT YOU JUST WROTE, BEFORE THE COMMIT.**
- **W93 -- WHEN TWO DEFENSIBLE READINGS OF ONE FIELD DISAGREE BY AN ORDER OF MAGNITUDE, THE FIELD HAS NO GRAMMAR**
- **W94 -- A WORKED INSTANCE IS EVIDENCE THAT AN INSTRUMENT CAN FIRE. IT IS NOT EVIDENCE ABOUT THE POPULATION, AND IT PRICES NOTHING**
- **W95 -- CITING AN ISSUE IS NOT CLAIMING IT IS OPEN, AND A COUNT BUILT ON THAT PREDICATE MEASURES NOTHING**
- **W96 -- A CLAIM REPEATED BACK TO YOU APPROVINGLY HAS NOT BEEN CHECKED, IT HAS BEEN AMPLIFIED**
- **W97 -- W-NUMBERS ARE NODE-LOCAL AND ARE BEING CITED ACROSS NODES AS IF SHARED.**
- **W98 -- AN `AC-nn.n` CITED WITHOUT ITS THREAD IS THE SAME DEFECT AS W97, IN THE VOCABULARY I USE EVERY DAY.**
- **W99 -- A SAMPLE THAT NOBODY LABELLED AS A SAMPLE BECOMES THE POPULATION.**
- **W100 -- A PEER CHANNEL HAS NO TERMINATING CONDITION, AND AN ACKNOWLEDGEMENT IS AN INVITATION**
- **W101 -- A HANDOVER DOES NOT DIE WITH THE NODE THAT WAS GOING TO MAKE IT.**
- **W102 -- I CONTRADICTED MY OWN FIRST MEASUREMENT OF THE SESSION AND CALLED IT A FINDING**
- **W103 -- A SANDBOX FAILURE READS EXACTLY LIKE A BUILD DEFECT, AND I NEARLY FILED TWO.**

- **W105 -- A CONDITION WRITTEN FROM A MECHANISM I HAD NOT DRIVEN NAMES A PROXY FOR THE PROPERTY, AND READS AS PERFECTLY CHECKABLE UNTIL IT IS CHECKED.**
- **W106 -- A CONVENTION FOLLOWED BY ITS NEIGHBOURS MAKES THE LINE THAT BREAKS IT READ AS INTENTIONAL**
- **W107 -- A GENERATED VIEW SELF-HEALS ON THE NEXT RENDER; AN ATTACHMENT DOES NOT; AND IN THE TREE THEY LOOK IDENTICAL.**
- **W108 -- DEMONSTRATE THE DEFECT NOT OCCURRING, NOT THE FIX WORKING.**
- **W109 -- A FIELD WITH TWO READERS ASKING DIFFERENT QUESTIONS IS CORRECT ONLY WHERE THEIR SUBJECTS COINCIDE, WHICH IS THE MOMENT NOBODY IS STRESSING IT.**
- **W110 -- A REMEDY STRING IS THE MOST-COPIED TEXT IN THE SYSTEM AND IS AUTHORED UNDER NONE OF THE CONSTRAINTS THAT IMPLIES**

- **W111 -- AN INSTRUMENT THAT VOLUNTEERS ITS OWN DENOMINATOR IS WHAT CATCHES A RIG ERROR YOU CANNOT SEE.**

- **W112 -- A NEGATIVE NEEDS A POSITIVE CONTROL ON THE RIG, NOT ONLY ON THE PATTERN.**

- **W113 -- A WELL-FORMED MEASUREMENT OF A SUBJECT WITH NOTHING TO LOSE RETURNS A CLEAN ZERO.**

- **W114 -- DRIVING A FILED DEFECT CAN FIND THE LARGER ONE HIDING IT.**

- **W115 -- A PROXY DOES NOT MERELY UNDER-SPECIFY THE PROPERTY; IT CAN OVERRIDE THE CORRECT SIGNAL.**

- **W116 -- A DUPLICATE REGISTER ROW RIDES ON THE ENTRY, IT DOES NOT GET A SECOND HOME.**

- **W117 -- A FIX CAN SHIP A FRESH INSTANCE OF THE CLASS IT IS FIXING, AND THE DOC THAT FORBIDS IT WILL ALREADY BE THERE.**

- **W118 -- AN ISSUE IS WRITTEN ABOUT THE INSTANCE SOMEBODY HAPPENED TO HIT; PROBE THE DOOR BEFORE FIXING THE ROW.**

- **W119 -- MEASURE THE BLAST RADIUS BEFORE CHOOSING REFUSE OVER NORMALISE, NOT AFTER.**

- **W120 -- EVERY CONTROL I WROTE DROVE A FUNCTION, AND THE DEFECT WAS IN THE SENTENCE.**

- **W121 -- THE MACHINERY A CONTROL NEEDS FOR ISOLATION CAN BE AN UNREPORTED BLIND SPOT IN THE FIELD.**

- **W122 -- `sqlite3 <path>` CREATES THE FILE, SO A READ CAN WRITE, AND A FALLBACK LADDER THEN ANSWERS FOR IT.**

- **W123 -- A PARTITION CAN BURY THE MEMBER IT WAS BUILT TO EXPOSE, AND IT LOOKS COMPLETE WHILE DOING IT.**

- **W124 -- A QUOTE IS A COUPLING TO PROSE SOMEBODY ELSE OWNS.**

- **W125 -- A REMEDY IS THE ONLY CLAIM ON A DEFECTS PAGE A READER ACTS ON, AND NOTHING VERIFIES ONE.**

- **W127 -- A PAIR CANNOT DISTINGUISH INTERMITTENT FROM STOPPED, AND TWO NODES AGREEING IS NOT CORROBORATION WHEN BOTH READ THE SAME OBJECT.**

- **W128 -- AN UNANCHORED GREP FOR A FIELD NAME COUNTS THE COMMITS THAT DISCUSS THE FIELD.**

- **W129 -- A GUARD THAT SCANS PROSE MAKES REPORTING THE DEFECT AN OFFENCE, AND THIS ESTATE HAD ALREADY RULED THAT AND WENT THE OTHER WAY.**

- **W130 -- KNOWING THE MENTION-VERSUS-USE CLASS DOES NOT PREVENT IT, AND THE CURE IS NOT A BETTER PATTERN. IT IS PRINTING THE LINE.**

- **W131 -- A LAWFUL GUARD REFUSAL AND A ONE-WAY DOOR ARE THE SAME BYTES AT THE PROMPT, AND CALLING ONE THE OTHER FABRICATES A DEFECT IN A ROW ABOUT DEFECTS.**

- **W132 -- THIS ESTATE'S STANDING LISTS DECAY FASTER THAN ANYONE RE-READS THEM, AND THE ONLY THING THAT FINDS IT IS GOING TO TOUCH THE ITEM.**

- **W133 -- WHEN A FIGURE IS TYPED BESIDE THE CODE THAT COMPUTES IT, LOOK FOR THE CONSTRUCT THAT FORCED THE LITERAL, NOT FOR THE PERSON WHO TYPED IT.**

- **W134 -- A PATH PREFIX CANNOT DISTINGUISH A TOOL FROM THE TOOL NAMED AFTER IT, AND BOTH CURES ARE WRONG.**

- **W135 -- WHEN YOUR VERIFIER REPORTS SOMETHING ALARMING, VERIFY THE VERIFIER BEFORE YOU REPORT THE ALARM. THREE INDEPENDENT DEFECTS IN ONE CHECK ALL POINTED THE SAME WAY, AND THE WAY THEY POINTED WAS AT A SERIOUS DEFECT THAT HAD NOT HAPPENED.**

- **W136 -- AN INSTRUMENT WHOSE FAILURE IS CONFINED TO THE CONDITION UNDER TEST IS THE WORST SHAPE THERE IS, BECAUSE EVERY CELL YOU DO NOT CARE ABOUT WORKS PERFECTLY.**

- **W137 -- A STATE IS NOT A MECHANISM, AND THE QUESTION THAT SEPARATES THEM IS _WHO PRODUCED THIS STATE_ RATHER THAN _WHAT WOULD PRODUCE IT_.**

- **W138 -- COUNTING PROCESSES BY COMMAND-LINE TEXT CANNOT WORK FROM INSIDE A SHELL THAT QUOTES THE TEXT, AND THE BRACKET TRICK DOES NOT SAVE YOU.**

- **W139 -- A CHANGE THAT FIXES A REAL DEFECT AND SILENTLY RE-BASES SOMEONE ELSE'S MEASUREMENT IS THE WORST SHAPE A CORRECT-LOOKING COMMIT CAN TAKE, BECAUSE THE DEFECT IT FIXES IS THE ALIBI.**

- **W140 -- REPLACING AN ENUMERATION OF PATHS WITH AN ENUMERATION OF SPELLINGS IS STILL AN ENUMERATION, AND IT TRADES FALSE NEGATIVES FOR FALSE POSITIVES.**

- **W141 -- `intent sync --to-store` REPLACES THE STORE FROM THE EXTRACT, SO A HAND-EDIT OF CANON SILENTLY REVERTS EVERY STATE VERB RUN SINCE.**

- **W126 -- WHEN AN OUTCOME DEPENDS ON AN AMBIENT THE READER CANNOT SEE, DRIVING ONE ARM ESTABLISHES ONE ARM. ~~AND THE SPECIFIC CLAIM BELOW IS WRONG~~ -- CORRECTED 2026-09-09 BY RE-DRIVING IT: the no-daemon arm INGESTS. A committed, clean, diverged attachment IS taken into canon by `sync --to-store`, on the release pair AND on a debug build at HEAD, with a negative control (canon does not move without the sync, 15s) and a positive control (the uncommitted case warns in full). **

## Decisions

- **2026-09-01, hv, first-hand in this session -- devbin `0047` RULED: option 3, the split.** `fullcycle`'s clean phase forces **only** the blocked-binaries arm (`clean:486`); `_clean_confirm`'s removal prompt **stays**. Ground: `--force` merges two consents and `fullcycle` discharges one of them. It answers for _the PATH binaries going dark_ -- it rebuilds them and verifies the rebuild fails RED -- and answers not at all for _deleting the build tree_. **The against-case survives and is not ceremony: the damage detector DETECTS a clean-succeeded-build-failed run, it does not PREVENT one, and Conflab hit exactly that.** Relayed to devbin-vc (they own `bin/.devbin/lib/`; the vendored copy here is clobbered on upgrade, so this is NEVER mine to implement). **I put the options to hv with a recommendation attached, so the provenance carries that.**
- **AND THE RULING SETS THE TRAP IT WAS JUST FIXED FOR, WHICH I FLAGGED WITH IT.** Option 3 changes WHICH refusal a non-interactive run reaches: today headless `fullcycle` dies at the blocked arm, after the split it sails past and dies at `_clean_confirm`'s no-tty refusal. **The notice fixed at `bd323ea` names both refusals and predicts neither -- correct then, and the REACHABLE SET changes under this ruling.** Same class arriving through the fix for it.

**hv PROVENANCE ENTRIES ARE VERBATIM AND ARE NEVER COMPRESSED** -- a ruling's own words cannot be re-derived.

- **AN AC REWORD THAT LEAVES ITS AT BEHIND CONVERTS A STRENGTHENED CRITERION INTO A WEAKER ONE**
- **A FIGURE TRANSCRIBED INTO A CRITERION IS A CLAIM ABOUT A CORPUS AT A MOMENT.**
- **D-DELETE -- A DELETE HAS THREE POPULATIONS AND I HAD ONE.**
- **D-POP -- FOLDED INTO W52 2026-09-05.**
- **D-SHARED -- FOLDED INTO W4/W4a 2026-09-05.**
- **D-FACES -- A DECLARATION HAS CONSUMERS YOU DID NOT WRITE, AND THE COMPILER GUARDS ONLY THE ONES IT CAN SEE.**
- **D-ADVICE -- A DEFECT THAT CORRUPTS NO NUMBER HAS NO INSTRUMENT WATCHING IT.**
- **A TEST WHOSE SUBJECT IS GONE IS DELETED WITH ITS REASON, NEVER PATCHED TO KEEP PASSING**
- **A NEW GUARD IS DRIVEN TWO-SIDED ON A PLANTED FIXTURE BEFORE IT IS BELIEVED, AND ITS LIMIT GOES IN ITS OWN HEADER.**
- **VERIFY A PEER'S CLAIM ABOUT YOUR OWN INSTRUMENT, EVEN WHEN THEY ARE RIGHT.**
- **A VERB CAN BE SEMANTICALLY CORRECT AND STILL BE A SILENT FAILURE, BECAUSE CORRECTNESS IS ABOUT WHAT IT DID AND SILENCE IS ABOUT WHAT IT SAID**
- **A PERMISSIVE READER MAKES A CONTRACT VIOLATION AND A CONFORMING IMPLEMENTATION INDISTINGUISHABLE.**
- **hv HAD ALREADY RULED THE THING I WAS INSTRUCTED TO BUILD, THE OTHER WAY.**
- **A RULING'S PREMISE CAN BE CORRECTLY MEASURED AND STILL NOT COVER WHAT YOU BUILD.**
- **A DECISION THAT REACHES A QUESTION IT WAS NOT ABOUT IS THE STRONGEST EVIDENCE FOR IT**
- **ic's WORDING FOR WHY THE INNOCENT EXPLANATION WINS: _a correct general model is what made me stop measuring._**
- **A FOLD IS A WRITE, SO IT NEEDS THE SAME READ-BACK ANY OTHER WRITE NEEDS, AND THE WRITER IS THE WORST READER OF IT IN THE SAME SESSION.**

- **TWO hv RULINGS, 2026-08-30, ARRIVING BETWEEN THE 14:59Z AND 15:02Z CLOCK READS OF THIS SESSION -- THE MINUTE IS NOT RECOVERABLE AND IS NOT INVENTED. FIRST-HAND IN THIS dc SESSION. THE WORDING IS hv's RATHER THAN A PARAPHRASE.**

  **(a) EVERYTHING GOES IN 3.0.1.** hv, verbatim: _"Sure. Everything is in 3.0.1. That is what Intent v3 is supposed to be. Everything we can get into 3.0.1."_

  **(b) `shipped_surface_drift` IS AN EXPIRED TEST AND THE v2 GATES ARE DEAD.** hv, verbatim: _"Expired test. We don't care about gates for v2 any more. Don't waste any time on it, other than to remove it."_ **DONE, and the second half was found at fold time**: the test was deleted but the CI step existing only for it was not, and it FAILED the job. **A removal ruled in one sentence had a second site nobody looked for.**

- **THE CUT GOES AHEAD AGAINST AN OPEN GATE. PROVENANCE: hv, 2026-08-30 11:41Z, FIRST-HAND IN THIS dc SESSION, hv's own selection from four options dc authored.** (A) **cut against an open gate -- CHOSEN**, tag 3.0.1 with ST0056 BLOCKED and the tag-gated rows named as such; (B) re-scope the five, declined -- it buys a green by changing the question; (C) two-phase RC-then-real tag, declined -- unless the code is byte-identical the second artefact is not the one the criteria were driven against; (D) fiat-close the five, declined -- **they are not over-cooked, merely not-yet-possible, which is the case `fc` was explicitly not built for.** **Five ST0056 criteria require the artefact only the cut produces**, so the gate could never read green before the release existed. **CONSEQUENCE: WP-07 and WP-11 not closing before the cut is the DECIDED shape, not something waiting on hv's hand.**

- **THE STORE-MIGRATION GUARD LANDS AFTER THE CUT. PROVENANCE: hv, 2026-08-30 11:51Z, FIRST-HAND IN THIS dc SESSION.** (A) extend the guard after the cut -- **CHOSEN**; (B) now, declined; (C) fix the two proven members and defer the guard, declined -- **instance-shaped fix under a class-shaped problem, W10 done knowingly**; (D) file and do nothing, declined. **THE GROUND IS THAT IT IS A DEV-TREE HAZARD WITH NO SHIPPED CONSEQUENCE**, not that it is small.
