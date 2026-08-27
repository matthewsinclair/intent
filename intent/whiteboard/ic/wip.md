---
node: ic
name: Interface Claude
role: interface
session_id: 6bbf2186-4635-4ce4-8bd0-02c75f289528
heartbeat_at: 2026-08-27 14:01Z
status: active
focus: "AC-11.3 SATISFIED and ST0061 AT-00.1 GREEN -- the migration realises what it declares, proven red against the pre-change source. The keeper: a dehydrate that removes NOTHING round-trips perfectly, so a round trip alone certifies an inverse that never ran. Holding: my queue is all under native/rust and the port is stopped behind cc building the pair."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. HOLDING** -- every remaining queue item is under `native/rust`, and vc has the port stopped behind a binary cc still has to build. Tree clean of me.

**TODAY: `e54bb968` (AC-11.3's migration clause + ST0061's round trip), `62070abb` (`AT-00.1` green), `fe1a0de8` (WP-11's cover, `AT-11.3` green, six recovered notes), plus two board commits.** Suite 177 binaries / 1284 passed / 0 failed; `doctor` 0; `at lint` clean at 56 and 7 rows. **AC-11.3 and AC-00.1 both compute satisfied.**

**THE ONE THING WORTH CARRYING FORWARD IS IN Watch-outs**, measured rather than argued: a do-nothing inverse round-trips perfectly, so AC-00.1's own instrument would have certified a verb that never ran.

## TODO

**OWED, in vc's order:** AC-11.6's arm (**BLOCKED -- hv's, and vc has carried it up; do not start**); ST0061's remaining ACs; the `organize --apply` exit-code pair in `exit_codes.rs`; `flag_reachability.rs` chaining `new_surface`. All under `native/rust`, so all wait on vc's word that the port has its binary.

**AC-11.6, for when it comes back:** `organize`'s source argues the preview's input would be produced by the run being previewed, and today's confirm promises _it removes no files_, which AC-11.6 makes false. **A confirm that understates what it will do is worse than no confirm.**

**AN AT ROW'S `note` HAS NO VERB WRITER** -- 249 of 308 file-citing rows carry one; the door is a canon edit plus `sync --to-store`. vc's to close. **Not a blocker on cc's 168-row re-cite** -- see the Watch-out; the re-cite done in canon keeps the note.

**THIRTEEN ESTATES STILL NEED `intent/.canon/project.json` COMMITTED.** vc owns the ports; until then the cutoff lives in ONE machine's store and a clone in the interval loses it.

**HANDED TO vc, GOING TO hv: six of ten realised thread directories here are non-WIP** -- the live symptom of what AC-11.3 just closed for NEW conversions. Intent's own tree still needs the one `organize` pass. Interacts with `st list` omitting triage silently.

**HOUSEKEEPING:** `AT-11.3` and `AT-00.1` cite PATHS and hv has ruled AT rows re-cited by test NAME (cc's, 168 rows). `tree()` is spelled in THREE `intent-cli/tests` files and 37 of 39 spell their own binary runner -- that crate has no `tests/common/mod.rs` while `intentsvcs` does; **reported in the test's own module doc rather than fixed in passing, because migrating the existing users mid-cut in a shared tree is a decision, not a side effect.**

## Watch-outs

**THE CLASS BEHIND MOST OF THESE: THE SCOPE IS CHOSEN BY A FLAG AND REPORTED BY NOTHING.** cc's general form -- **the tree is an input to the run, and an input nobody names is an assumption.**

**INSTRUMENTS THAT REPORT A CONFIDENT NUMBER ABOUT THE WRONG THING.**

- `cargo check -p X` builds the **lib target only**; `--test Y` runs ONE binary; neither sees `src/**`'s `mod tests`. Use `--workspace --all-targets --no-fail-fast`.
- **A green over a SHARED tree is about whoever's files are in it.** Verify in a detached worktree at HEAD carrying your files and nothing else.
- **Capture a run to a FILE and measure the file** -- `| tail -N` truncates the thing you are about to measure.
- **A zero from your parser is indistinguishable from a zero from the world.** Wrong awk fields returned `passed=0 failed=0` on a populated run; what caught it was the rc contradicting the count -- **a second independent reading, never the instrument itself.**
- **A background job's reported exit code is the WRAPPER's**, and a trailing `echo` launders failure into success: put the rc IN the log and read it there. **A trailing `&` inside a `run_in_background` call is the worst form** -- the wrapper returns instantly, the harness reports _completed, exit code 0_, and the build is still running. The `&` is redundant; drop it.
- **A process check that reads argv counts every agent TOLD about the tool** -- prose carries the string and command lines contain newlines. Match the command NAME.
- **A `target/*/deps` BYTE FIGURE IS CRATE SIZE x GENERATIONS RETAINED, NOT CRATE WEIGHT** -- cargo never collects them, and everyone had assumed it was debug-only.
- `cmd | head; echo "RC=$?"` reads head's code; `${PIPESTATUS[0]}` is bash and **this shell is zsh** (`$pipestatus[1]`). `grep -c` exits 1 on no match. An unmatched glob (`--include=*.rs`) ABORTS the command. **zsh does NOT word-split an unquoted `$var`.** **`cat -A` is GNU-only** and dies mid-pipeline; `sed 's/ /./g'` shows whitespace portably.

**CONTROLS.**

- **A control that cannot go red is not a control** (IN-AG-RED-CONTROL-001), and **a positive control that would also pass under the broken instrument is decoration.**
- **ASSERT THE CAUSE: an outcome is reachable by more than one path**, and a message naming one of them sends the next reader at the wrong fix.
- **A CRITERION CAN BE SATISFIED PERFECTLY BY A DEGENERATE IMPLEMENTATION, AND THE TEST THAT PROVES THE CRITERION WILL SAY SO.** Two measured cases: `default_declaration` returning NOTHING passes _all three callers agree_, because three writers agreeing on nothing is a perfect score; and a `dehydrate` that removes nothing passes _proven by round trip_, because a do-nothing inverse round-trips perfectly. **The general form: a property that holds VACUOUSLY is indistinguishable from one that holds substantively, and only an assertion on the MIDDLE STATE tells them apart.** What a criterion asks for is a floor, never a ceiling.
- **GREEN ON THE FIRST RUN IS NOT EVIDENCE.** Mutate the source, one change at a time, until every assertion has been SEEN to fail by name, and put the table where a reader of the test meets it. vc wants this on every proof test now.
- **A control is CONSUMED by being used as a subject, one-way, and nothing about the artefact says so.** **Re-measure a fixture when you CITE it, not when you built it.**
- **I ESCALATED A BLOCKER FROM THE SHAPE OF A DEFECT WHILE STANDING ON THE PATH THAT DEFEATS IT.** dc reported `at new` eats an AT row's `note` and no verb writes one, so a 168-row re-cite was "hard-blocked"; I sized it to vc as a prerequisite. **It is not** -- a re-cite done in CANON plus `sync --to-store` moves `file` and keeps `note`, driven -- **and I had restored six notes that exact way an hour earlier.** A defect's shape tells you what breaks, never what else works; **the door you just used does not announce itself while you reason about the door that failed.**
- **An instrument that cannot see the state it looks for is the same defect one level up** -- odd-backtick-count catches the cause and never the aftermath.
- **THE RECORD OF WHAT A TEST CANNOT DO IS NEVER RECOVERABLE FROM THE TEST** (dc). An AT row's `note` is where a control and a known limitation live; lose it and the row reads as sound. Which is why `at new` silently dropping six of them was worse than six strings.

**A COMMENT IS NOT A MECHANISM, AND IT HAS COST ME FOUR TIMES.** The worst: **the DDL I edited is not the DDL the test reads.** Name the rule and give the test.

**A STALE DOC IN THE ONE PLACE A READER ARRIVES IS WORSE THAN NONE**, because a corrected sentence reads exactly like one that was never wrong. Rewrite it AS superseded, quoting what it used to say -- vc has made that a standing convention. Same rule for a ruling: **transcribe it from its source with its provenance caveat attached; a quote that sheds its provenance is how a relay becomes a ruling.** Never take a peer's paste for the blob.

**CLOCKS: NOTHING BUT `date -u` IS ONE** -- not `git log` (local), not a session notice, not an mtime, and not a heartbeat already on the board. `git log --date=format:` renders in the COMMIT'S OWN zone, so appending a literal `Z` fabricates a stamp in the call meant to recover true times -- use `%cI`. **A wrong stamp is recoverable only while something real still bounds it.**

**SHARED-CHECKOUT MECHANICS.** `git commit --only` is PATH-scoped, not hunk-scoped. `git checkout -- <file>` in a dirty tree is a REVERT: copy aside, copy back. `.git/index.lock` means a peer is mid-commit -- wait, never clear it. `git show HEAD` after a failed commit shows someone else's and reads like success. **`cargo fmt --all` WRITES**, invalidating every anchored edit a peer has in flight. **`git status` here is a snapshot of whoever is mid-write, not a state.** **Verify a repair out of the COMMITTED BLOB** -- the formatter writes between tree and commit.

**TWO GITIGNORE TRAPS, OPPOSITE DIRECTIONS, BOTH ABOUT WHAT A TREE DOES NOT RECEIVE.** A worktree does not receive gitignored files, so `.githooks/pre-commit.intent` is absent and the clock, header and append-only guards are all inert in it. And **`intent init` lays down no `.gitignore`, so a fresh estate TRACKS its store.** **The protection is an ORDERING: the ignore must precede the first v3 write.**

**DESIGN, FOUR THAT KEEP RECURRING.** **An unverified schema bump does not go on main** -- a version move migrates every store on the machine. **A mechanism can be complete and unreachable**, and a unit test calling a function directly says nothing about whether anything calls it. **Definition by exclusion acquires members by accident** -- `!is_closed()` swept Triage, Not Started and Hold into the realised set. **A hold stated by its scope invites compliance that defeats its purpose** -- state the mechanism and the scope follows.

**THE BACKTICK DEFECT IS UNQUOTABLE IN THE MEDIUM THAT CARRIES THE REPORT** -- writing it down reproduced it three times. A backtick is not an apostrophe; a plain `'` is correct in board prose, and the never-escape rule is about the HEADER block ONLY. **AND THE ODD-COUNT CHECK I KEEP CITING HAS A FALSE-POSITIVE ARM TO MATCH ITS FALSE-NEGATIVE ONE:** splitting on the backtick and testing for an even field count flags every EMPTY line, because awk sets NF=0 on an empty record -- 26 phantom hits on a clean board. Guard it with a non-zero length test. **Both arms of one instrument were wrong in opposite directions and neither announced it.**

## Decisions

- **(hv, 2026-08-26) The realised set is WIP ALONE. THERE IS NO 3.0.2.** Ruling transcribed into WP-11's cover at `fe1a0de8` from `hv/wip.md` 19:48Z, provenance caveat attached.
- **(hv, 2026-08-26) The DONE heading is `## DONE:<T>`, <T> a full ISO 8601 UTC instant** -- the time the prune ran, hence the cutoff. Verbatim v2. **And a flush CLEARS same-day work**: a completion date widens to that day's midnight, which is below any flush later that day.
- **(hv, 2026-08-26) WP-14: the cutoff is CANON STATE, not history.** A flush HAPPENING at T is an event; the cutoff BEING T is a fact about the project now. It goes in `.canon/project.json`, not `config.json` -- config holds choices a person makes, and a machine-written value there gives one file two writers.
- **(hv, via vc 2026-08-27) In development, move to the latest build of any dependency and fix forward; favour consolidation into the simplest coherent structure.** This is what retired vc's defer-to-3.0.2 hold.
- **(ic, 2026-08-26) `event::todo_watermark` is DELETED rather than kept as a fallback.** Two homes for one value is the gate-figure defect in miniature, and both answers look plausible.
- **(ic, 2026-08-26) A canon file committed with a field MISSING is not neutral.** `carry_project_state` is absent-leaves-the-store-alone, present-and-empty-WINS -- so shipping `project.json` without its watermark reads as "never flushed" and clears the value the migration just recovered.
- **(ic, 2026-08-27) The `project.json` commits are CRITICAL PATH, not tidy-up.** Measured: a storeless machine with no canon file renders `## DONE` and the value is gone. vc rescheduled 14 estates on this.
- **(vc, 2026-08-27) WP-14 LANDS FIRST, THEN ONE PAIR IS PINNED FROM IT.** Pinning first pins a v13 binary that then meets a store another binary walked to 14 -- and the v13 pair REFUSES a walked estate outright. **A stamp is honest when the code trees match.**
- **(ic + vc, 2026-08-26) `wp start` is the only legal route from not-started to done, AND TAKING IT REALISES A COMPLETED THREAD** -- ten view files plus a line in an append-only manifest, which nothing in the normal path retracts. **A hole between two correct rules, which is the shape that survives review.** vc filing it.
- **(ic, 2026-08-26) The record-timestamp convention is DISCOVERED, not listed** -- every table needs a stamp column with a `DEFAULT (`, so a new table inherits the requirement automatically. `updated_at` is when this DB wrote the ROW, not a second home for "a flush happened at T".
- **(ic + vc, 2026-08-26) Enumeration for dehydration is the CORPUS SCAN** (`sync::scan`, excluding only `Ignored::for_root`), never a recursive `read_dir`. A directory the run cannot empty is NAMED in the verdict.
- **(ic, 2026-08-26) A restore of a four-month-old commit is a MERGE, not a revert.** The buckets-and-view test counted `- [ ] `, written before rows carried a status glyph, so it had been matching nothing and asserting 0 against a populated view.
