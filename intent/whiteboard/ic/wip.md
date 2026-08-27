---
node: ic
name: Interface Claude
role: interface
session_id: 6bbf2186-4635-4ce4-8bd0-02c75f289528
heartbeat_at: 2026-08-27 12:01Z
status: active
focus: "**FOLDED, STOPPED WITH THE ESTATE.** hv halted all work on disk exhaustion; my 2.0 GB `target/ic` is cleared and the release product verified unmoved by hash. Landed before the stop: AC-11.3's proof test at `22b2e734`, `AT-11.3` red on purpose. The lesson worth keeping: a `default_declaration` that declares NOTHING satisfies the criterion as written -- only the controls catch it. Owed queue intact and unstarted."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT -- hv HAS STOPPED ALL WORK ESTATE-WIDE.** Disk exhaustion: ~2.9M files and ~146 GB across the estate's cargo target dirs, ~82 GB reclaimed. My lane stops with the rest.

**LANDED BEFORE THE STOP: AC-11.3's PROOF TEST, `22b2e734`, `AT-11.3` RED ON PURPOSE.** 170 binaries / 1263 passed / 0 failed, verified out of the committed blobs. Every expectation is the live function's output for that estate's own statuses; none is a literal, because three literals would be three homes for the definition. **Seven mutations proved each assertion can go red -- the table is in the test file, where a reader of the test meets it, and the rule it taught is under Watch-outs.** Red because the migration clause -- _realises only those threads_ -- is not asserted, and a green row would satisfy a criterion with an unmet clause.

**MY 2.0 GB IS CLEARED, AND THE COST TWO PEERS WEIGHED DOES NOT EXIST.** `native/rust/target/ic` was my private `CARGO_TARGET_DIR` from the shared-artefact guard's refusal path, which nothing collects. vc and dc both deferred to me reasoning it would cost a cold rebuild -- **but that dir serves REFUSED RELEASE BUILDS, not `cargo test`, and the shared `target/debug` they had already deleted is what my test runs use.** The rebuild was owed either way. 2039 MB back; release product verified unmoved by hash after (`04008c3f` / `588f34ab`, matching vc's record).

## TODO

**OWED, in vc's order** (all stopped, none started): AC-11.6's arm; AC-11.3's migration clause (what turns `AT-11.3` green); ST0061 AC-00.1's round trip; the `organize --apply` exit-code pair in `exit_codes.rs`; `flag_reachability.rs` chaining `new_surface`.

**AC-11.6 NEEDS A RULING BEFORE IT IS BUILT, AND BOTH SIDES ARE hv's.** `organize`'s source argues against folding declaration and reconciliation into one pass -- _the input to the preview would be produced by the run being previewed_ -- while AC-11.6 requires exactly that behind a tty confirm. Today's confirm text also promises _it removes no files_, which AC-11.6 makes false: **a confirm that understates what it will do is worse than no confirm.**

**WP-11's COVER CONTRADICTS hv's OWN RULING; SENT TO vc, AWAITING THEIR WORD.** It states the pre-ruling _OPEN = every status except Completed and Cancelled_ that hv overruled to WIP-only, plus the superseded _never removes a file_. It renders from canon, and rewording a cover to match a ruling is close enough to restating the ruling that vc or hv should hold the pen.

**AN AT ROW'S `note` HAS NO MUTATION VERB** -- six subcommands, none sets it, so the narrative every other row carries needs a canon hand-edit plus `sync --to-store`. Same shape as the create gap `AC-08.6`/`AC-08.7` closed, one field down, and it bites where a status needs a reason: `AT-11.3` is red on purpose and cannot say why on its own row.

**THIRTEEN ESTATES STILL NEED `intent/.canon/project.json` COMMITTED.** vc owns the ports; the reason is mine to keep stating -- until it is committed the cutoff lives in ONE machine's store and a clone in the interval loses it. **Seen working in the wild since:** a peer's flush moved Intent's own file to `2026-08-27T09:37:49Z` and it travelled as committed state.

**HANDED TO vc, GOING TO hv:** **six of ten realised thread directories in Intent are non-WIP**, and only five predate me, so it is not an artefact of my own `wp start`. Interacts with `st list` omitting triage silently, so three are on disk AND invisible to the primary list verb.

**HOUSEKEEPING FOR THE NEXT RUN:** `AT-11.3` cites a PATH and hv has ruled AT rows re-cited by test NAME (cc's job, 168 of 334 rows). **Cold rebuild owed** -- shared `target/debug` is gone, so the next run I time must not be set beside a warm figure. **Do not drive anything on `be13157a`** (an earlier cc commit was DELETING criteria; recovery is `f2477875`) -- checked rather than assumed at this pickup: ST0057 stands at 61 criteria / 56 AT rows, identical between `22b2e734` and HEAD.

## Watch-outs

**THE CLASS BEHIND MOST OF THESE: THE SCOPE IS CHOSEN BY A FLAG AND REPORTED BY NOTHING.** cc's general form -- **the tree is an input to the run, and an input nobody names is an assumption.**

**INSTRUMENTS THAT REPORT A CONFIDENT NUMBER ABOUT THE WRONG THING.**

- `cargo check -p X` builds the **lib target only**; `--test Y` runs ONE binary; neither sees `src/**`'s `mod tests`. Use `--workspace --all-targets --no-fail-fast`.
- **A green over a SHARED tree is about whoever's files are in it.** Verify in a detached worktree at HEAD carrying your files and nothing else.
- **Capture a run to a FILE and measure the file.** `| tail -N` truncates the thing you are about to measure -- 8 of 169 binaries, called the workspace.
- **A zero from your parser is indistinguishable from a zero from the world.** My awk read the wrong fields and returned `passed=0 failed=0` on a populated run; what caught it was `TEST_RC=101` contradicting "no failures" -- **a second independent reading, never the instrument itself.**
- **A background job's reported exit code is the WRAPPER's, not the work's**, and **a trailing `echo` launders any failure into a success.** Put the rc IN the log and read it from there.
- **A process check that reads argv counts every agent TOLD about the tool** -- 83 hits against 2 processes, because prose carries the string and command lines contain NEWLINES. Match the command NAME.
- **A `target/*/deps` BYTE FIGURE IS CRATE SIZE x GENERATIONS RETAINED, NOT CRATE WEIGHT.** cargo never collects them: 455 rlibs for 199 crates, 1.3 GB, beside 25.4 MB of product. dc retired a number for this, and **everyone had assumed it was debug-only.**
- `cmd | head; echo "RC=$?"` reads head's code; `${PIPESTATUS[0]}` is bash and **this shell is zsh** (`$pipestatus[1]`). `grep -c` exits 1 on no match. An unmatched glob (`--include=*.rs`) ABORTS the command. **zsh does NOT word-split an unquoted `$var`** -- list paths explicitly.

**CONTROLS.**

- **A control that cannot go red is not a control** (IN-AG-RED-CONTROL-001), and **a positive control that would also pass under the broken instrument is decoration.**
- **ASSERT THE CAUSE: an outcome is reachable by more than one path**, and a message naming one of them sends the next reader at the wrong fix.
- **A CRITERION CAN BE SATISFIED PERFECTLY BY A DEGENERATE IMPLEMENTATION, AND THE TEST THAT PROVES THE CRITERION WILL SAY SO.** `default_declaration` returning nothing passes _all three callers agree_ -- three writers agreeing on nothing is a perfect score. **What a criterion asks for is a floor and never a ceiling**; the controls beside the proof are what see past it.
- **A control is CONSUMED by being used as a subject, one-way, and nothing about the artefact says so.** I walked my own v13 fixture to 14 mid-experiment and then offered it as the v13 side of a control. **Re-measure a fixture when you CITE it, not when you built it.**
- **An instrument that cannot see the state it looks for is the same defect one level up** -- odd-backtick-count catches the cause and never the aftermath, so my board read odd=0 with four live damage sites.

**A COMMENT IS NOT A MECHANISM, AND IT HAS COST ME FOUR TIMES.** The worst two: **the DDL I edited is not the DDL the test reads**, and my DDL comment naming a decision where the constant's own doc says every `--` line is PUBLISHED to strangers, names the rule and gives the test.

**CLOCKS: NOTHING BUT `date -u` IS ONE** -- not `git log` (local), not a session notice, not mtimes. The session notice is LOCAL and at `+0100` flips an hour early, so I stamped two Decisions into the future under the watch-out naming the class. `git log --date=format:` renders in the COMMIT'S OWN zone, so appending a literal `Z` fabricated a stamp **in the call meant to recover the true times** -- use `%cI` or `TZ=UTC ... --date=format-local:`. **A wrong stamp is recoverable only while something real still bounds it.**

**SHARED-CHECKOUT MECHANICS.** `git commit --only` is PATH-scoped, not hunk-scoped. `git checkout -- <file>` in a dirty tree is a REVERT: copy aside, copy back. `.git/index.lock` means a peer is mid-commit -- wait, never clear it. `git show HEAD` after a failed commit shows someone else's and reads like success. **`cargo fmt --all` WRITES**, invalidating every anchored edit a peer has in flight. A peer's cargo lock hangs your check for ten minutes -- contention, not failure. **`git status` here is a snapshot of whoever is mid-write, not a state.** **Verify a file repair out of the COMMITTED BLOB** -- the formatter writes between tree and commit.

**TWO GITIGNORE TRAPS, OPPOSITE DIRECTIONS, AND BOTH ARE ABOUT WHAT A TREE DOES NOT RECEIVE.** A worktree does not receive gitignored files, so `.githooks/pre-commit.intent` is absent and clock, both whiteboard guards and append-only are all inert in it -- I only know because the hook REFUSED rather than running without them. And **`intent init` lays down no `.gitignore` at all, so a fresh estate TRACKS its store.** **The protection is an ORDERING: the ignore must precede the first v3 write.**

**DESIGN, FOUR THAT KEEP RECURRING.** **An unverified schema bump does not go on main** -- a version move migrates every store on the machine; a branch contains the blast radius. **A mechanism can be complete and unreachable**, and a unit test calling a function directly says nothing about whether anything calls it -- the coverage is what makes the missing caller invisible. **Definition by exclusion acquires members by accident** -- `!is_closed()` swept Triage, Not Started and Hold into the realised set. **A hold stated by its scope invites compliance that defeats its purpose** -- state the mechanism and the scope follows.

**THE BACKTICK DEFECT IS UNQUOTABLE IN THE MEDIUM THAT CARRIES THE REPORT.** Writing it down reproduced it three times. A backtick is not an apostrophe; a plain `'` is correct in board prose, and the never-escape rule is about the HEADER block ONLY.

## Decisions

- **(hv, 2026-08-26) The realised set is WIP ALONE. THERE IS NO 3.0.2.**
- **(hv, 2026-08-26) The DONE heading is `## DONE:<T>`, <T> a full ISO 8601 UTC instant** -- the time the prune ran, hence the cutoff. Verbatim v2. **And a flush CLEARS same-day work**: a completion date widens to that day's midnight, which is below any flush later that day.
- **(hv, 2026-08-26) WP-14: the cutoff is CANON STATE, not history.** A flush HAPPENING at T is an event and stays in the log where D53 put it; the cutoff BEING T is a fact about the project now. **Filing it as history is what put it on the wrong side of D53, and that was mine to have seen.** It goes in `.canon/project.json`, not `config.json` -- config holds choices a person makes, and a machine-written value there gives one file two writers.
- **(ic, 2026-08-26) Nothing reads a cutoff out of the log, and `event::todo_watermark` is DELETED rather than kept as a fallback.** Two homes for one value is the gate-figure defect in miniature, and both answers look plausible. The migration rung reads the log exactly once.
- **(ic, 2026-08-26) A canon file committed with a field MISSING is not neutral.** `carry_project_state` is absent-leaves-the-store-alone, present-and-empty-WINS -- so shipping `project.json` without its watermark would read as "never flushed" and clear the value the migration just recovered. It carries the real cutoff and lands in the SAME commit as the code that reads it.
- **(ic, 2026-08-27) The `project.json` commits are CRITICAL PATH, not tidy-up.** Measured: a storeless machine with no canon file renders `## DONE` and the value is gone; with the file, `## DONE:<T>`. vc rescheduled 14 estates on this.
- **(vc, 2026-08-27) WP-14 LANDS FIRST, THEN ONE PAIR IS PINNED FROM IT.** Pinning first would pin a v13 binary that then meets a store another binary already walked to 14 -- and that straddle is not hypothetical: the v13 pair REFUSES a walked estate outright. **A stamp is honest when the code trees match** -- `f438d0d5` vs `9bd6b0a3` differ in five whiteboard files and zero `.rs`/`.sql`/`.toml`, so the pair naming the branch tip is a correct label, not a stale one.
- **(ic + vc, 2026-08-26) `wp start` is the only legal route from not-started to done, AND TAKING IT REALISES A COMPLETED THREAD** -- ten view files plus a line in an append-only manifest, which nothing in the normal path retracts. **A hole between two correct rules, which is the shape that survives review.** vc filing it.
- **(ic, 2026-08-26) The record-timestamp convention is DISCOVERED, not listed** -- every table needs a stamp column with a `DEFAULT (`, so a new table inherits the requirement automatically. `updated_at` is when this DB wrote the ROW, not a second home for "a flush happened at T". I nearly dropped it on that misreading and the enforced convention stopped me.
- **(ic + vc, 2026-08-26) Enumeration for dehydration is the CORPUS SCAN** (`sync::scan`, excluding only `Ignored::for_root`), never a recursive `read_dir`. A directory the run cannot empty is NAMED in the verdict.
- **(ic, 2026-08-26) A restore of a four-month-old commit is a MERGE, not a revert.** The buckets-and-view test counted `- [ ] `, written before rows carried a status glyph, so it had been matching nothing and asserting 0 against a populated view.
