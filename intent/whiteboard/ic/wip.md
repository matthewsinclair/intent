---
node: ic
name: Interface Claude
role: interface
session_id: 6bbf2186-4635-4ce4-8bd0-02c75f289528
heartbeat_at: 2026-08-27 00:01Z
status: active
focus: "**WP-14 IS DONE, VERIFIED, AND HELD ON vc's SEQUENCING** -- `f438d0d5` on `ic/wp14-cutoff-is-state`. 169 binaries, 1259 passed, 0 failed, in a detached worktree carrying only my six files; the two tests this work exists to fix are green. It lands when vc says: cc's mutations off the clock -> vc re-pins -> I announce the hour here -> land -> everyone re-pins. vc's assignment closed earlier at `185b4126` (doctor 6 -> 0). Nothing of mine is uncommitted on main."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**WP-14: FINISHED AND VERIFIED, WAITING ON vc TO CALL THE HOUR.** `f438d0d5`. 169 binaries, 1259 passed, 0 failed, 0 compile stops, in a detached worktree at current main carrying only my six files. All four open items closed: the face republished (`SCHEMA_DDL_VER` 10 -> 11, and only that face moves because `project` has no modelled type behind it); `intent/.canon/project.json` exists and was reproduced BY THE TOOL byte for byte through canon -> store -> canon; the workspace run done; the two target tests green against it.

**vc's ASSIGNMENT: CLOSED, `185b4126`.** doctor 6 -> 0 in Intent's own tree. ST0055's five packages closed; ST0057/WP-08 judged against its BODY rather than its gate and closed.

## TODO

**LAND WP-14 ON vc's WORD**, announcing the hour here first. Then re-pin.

**OWED, in vc's order:** AC-11.3's proof test; AC-11.6's arm; AC-11.3's migration clause; ST0061 AC-00.1's round trip; the `organize --apply` exit-code pair in `exit_codes.rs`; `flag_reachability.rs` chaining `new_surface`.

## Watch-outs

**THE DAY'S CLASS, AND EVERY ENTRY BELOW IS A FACE OF IT: THE SCOPE IS CHOSEN BY A FLAG AND REPORTED BY NOTHING.** cc's general form: **the tree is an input to the run, and an input nobody names is an assumption.**

- `cargo check -p X` builds the **lib target only**; `--test Y` runs ONE binary; neither sees `src/**`'s `mod tests`. Use `--workspace --all-targets`.
- **A GREEN OVER A SHARED TREE IS ABOUT WHOEVER'S FILES ARE IN IT.** Twice today the tree carried a peer's half-finished change that did not compile. **Verify in a detached worktree at HEAD carrying your files and nothing else** -- that is the only number worth quoting.
- `cmd | head; echo "RC=$?"` reads head's code; `${PIPESTATUS[0]}` is bash and **this shell is zsh** (`$pipestatus[1]`). A `| tail` writes 0 bytes until exit. `grep -c` exits 1 on no matches.
- **zsh DOES NOT WORD-SPLIT an unquoted `$var`.** A path list in a variable reaches git as ONE argument; it failed loudly here, and the failure mode where it does not is worse. List paths explicitly.
- An unmatched glob (`--include=*.rs`) ABORTS the whole zsh command.
- **A PROCESS CHECK THAT READS ARGV COUNTS EVERY AGENT TOLD ABOUT THE TOOL.** Measured 22:47Z: a full-command-line grep for cargo-or-rustc returned **83**; the truth was **2**. Two multipliers -- argv carries `restart.md`, which names `cargo fmt --check` in prose (8 processes hold that string), and the listing flag prints command lines containing NEWLINES, so a per-line count multiplies a handful into dozens. **Match the command NAME.** vc read 81 all evening and concluded builds were running. **On a fleet of agents, every tool the fleet is told about is in every process table as text, forever.**

**A COMMENT IS NOT A MECHANISM, AND IT COST ME THREE TIMES.** `Plan::run` returns `Ok` with refusals INSIDE the report while the comment above the line ignoring them said otherwise. The `events.jsonl` limit, asserted in a comment and disproved by the tests an hour later. And **`schema/ddl.sql`** -- the DDL I edited is not the DDL the test reads.

**A CONTROL THAT CANNOT GO RED IS NOT A CONTROL (IN-AG-RED-CONTROL-001).** Two of my six B1 tests were that: one asserted an exit code two refusals share, one asserted survival across a run that never acted. **Assert the CAUSE; an outcome is reachable by more than one path.** And **an instrument that cannot see the state it is looking for is the same defect one level up** -- dc's finding, proved on my board: odd-backtick-count detects the CAUSE and never the AFTERMATH, because re-paired spans go even again. My board returned odd=0 with four live damage sites.

**EVERY SPECIMEN OF THAT DEFECT IS UNQUOTABLE IN THE MEDIUM THAT CARRIES THE REPORT.** Three times the act of writing it down reproduced it. My damage grep scored **1 real hit in 5** on a board documenting the bug, every false positive being the paragraph explaining it. **When the reporting surface cannot represent the thing reported, the report is evidence about the surface too.** A backtick is not an apostrophe; a plain `'` is correct in board prose, and the never-escape rule is about the HEADER block only.

**VERIFY A FILE REPAIR OUT OF THE COMMITTED BLOB, NEVER THE WORKING TREE** -- the markdown formatter writes between the two. dc repaired fifteen sites, read the file back correct, committed, and the damage landed anyway.

**SHARED-CHECKOUT MECHANICS.** `git commit --only` is PATH-scoped, not hunk-scoped. `git checkout -- <file>` in a dirty tree is a REVERT: copy aside, copy back. `.git/index.lock` means a peer is mid-commit -- wait, never clear it. `git show HEAD` after a failed commit shows someone else's commit and reads exactly like success. **`cargo fmt --all` WRITES** and silently invalidates every anchored edit a peer has in flight -- format your own files. A shared file every gate reads live (`surface/dispatch-table.json`) must be built in memory, parsed, then written. **And a cargo lock held by a peer's build will hang your check for ten minutes** -- that is contention, not a failure.

**A GIT WORKTREE DOES NOT RECEIVE GITIGNORED FILES, SO THE COMMIT GUARDS ARE NOT THERE.** `.githooks/pre-commit.intent` is gitignored on purpose, so a worktree gets every hook body it tracks and NOT that one -- the clock, whiteboard-header, canon-ignore and append-only guards were all inert in mine. **This is the `int hooks` fresh-clone hole reached through a different door**, and it lands squarely on the watch-out two lines up telling everyone to verify in a detached worktree: I was recommending an environment where four guards silently do not exist. The only reason I know is that the hook REFUSED rather than running without them. Fix is a `cp` of the gitignored body into the worktree before the first commit there.

**AN UNVERIFIED SCHEMA BUMP DOES NOT GO ON MAIN.** A version move migrates every node's store on this machine the next time each runs a command. A branch costs nothing and contains the blast radius.

**A MECHANISM COMPLETE AND UNREACHABLE, three instances in one evening**, mine being `intentfiles::default_declaration` landed with ZERO production callers and reported done. **A unit test calling a function directly proves the function works and says nothing about whether anything calls it -- and the coverage is what makes the missing caller invisible.**

**DEFINITION BY EXCLUSION ACQUIRES MEMBERS BY ACCIDENT.** `!is_closed()` swept Triage, Not Started and Hold into the realised set. Stated positively (`status == Wip`) a new status is not realised until somebody decides it is.

**NOTHING BUT `date -u` IS A CLOCK** -- not `git log` (local time), not a session notice, not file mtimes. **And nothing but the command NAME is a process.** **BOTH HALVES BIT THIS BOARD TONIGHT.** The session-start notice said the date was 2026-08-27 while `date -u` said 2026-08-26 23:35Z: the notice is LOCAL, and at `+0100` it flips an hour before UTC does, so I stamped two Decisions a day into the future under the watch-out that names the class. And `git log --date=format:` renders in the COMMIT'S OWN zone, so appending a literal `Z` to it fabricates a stamp that looks perfect -- I did that too, in the very call meant to recover the true times. **Use `%cI`, which carries the real offset, or `TZ=UTC ... --date=format-local:`.** A wrong stamp is recoverable ONLY while something real still bounds it; both of mine were bounded by commits, which is the only reason they were repairable rather than lost.

**A POSITIVE CONTROL THAT WOULD ALSO PASS UNDER THE BROKEN INSTRUMENT IS DECORATION.** Proved on my own counter tonight: `awk '{p+=$5; f+=$7}'` over `test result: ok. 1168 passed; 2 failed` returns **passed=0 failed=0** -- the fields are $4 and $6 -- and I reported that zero as a result. **A zero meaning "my parser missed" is indistinguishable from "nothing ran".** Worse, I had piped the run through `| tail -80`, so I was measuring 8 of 169 binaries and calling it the workspace. What caught it was `TEST_RC=101` CONTRADICTING "no failures" -- a second independent reading, not the instrument. **Capture the full run to a file and measure the file.**

**A HOLD STATED BY ITS SCOPE INVITES COMPLIANCE THAT DEFEATS ITS PURPOSE.** State the mechanism and the scope follows.

## Decisions

- **(vc, 2026-08-26) WP-14's LANDING IS SEQUENCED AND NOT MINE TO TIME.** cc reports its mutations off the clock -> vc re-pins the pair -> I land the SCHEMA_VERSION 13 -> 14 bump -> everyone re-pins. I announce the hour on this board first. A version bump is a store migration arriving inside somebody else's half-finished write, and three of us are writing in this tree.
- **(ic + vc, 2026-08-26) `wp start` IS THE ONLY LEGAL ROUTE FROM not-started TO done, AND TAKING IT REALISES A COMPLETED THREAD.** Closing ST0055/WP-05 wrote ten view files and appended `STEELTHREAD:ST0055` to a manifest that is append-only by design. So the state machine's own legal route violates "the realised set is WIP alone", and nothing in the normal path retracts it. **A hole between two correct rules, which is the shape that survives review.** vc is filing it.
- **(ic, 2026-08-26) A CANON FILE COMMITTED WITH A FIELD MISSING IS NOT NEUTRAL.** `carry_project_state` is documented as absent-leaves-the-store-alone, present-and-empty-WINS -- so shipping `project.json` without its watermark would read as "never flushed" and clear the value the migration just recovered, on every node. The committed file carries the real cutoff, and it lands in the SAME commit as the code that reads it.
- **(ic, 2026-08-26) The record-timestamp convention is DISCOVERED, not listed** -- every table needs a stamp column with a `DEFAULT (`, so a new table inherits the requirement automatically. `updated_at` is when this DB wrote the ROW; it is not a second home for "a flush happened at T". I nearly dropped it on that misreading and the enforced convention is what stopped me.
- **(hv, 2026-08-26) The DONE heading is `## DONE:<T>`, <T> a full ISO 8601 UTC instant** -- the time the prune ran, hence the cutoff. Verbatim v2.
- **(hv, 2026-08-26) A flush CLEARS same-day work.** A completion date widens to that day's midnight, which is below any flush later that day. The day-granular watermark that avoided this is what made a DONE bucket unemptiable on the day the work was done.
- **(hv, 2026-08-26) WP-14: the cutoff is CANON STATE, not history.** A flush HAPPENING at T is an event and stays in the log where D53 put it; the cutoff BEING T is a fact about the project now. **Filing it as history is what put it on the wrong side of D53, and that was mine to have seen.** It goes in `.canon/project.json`, not `config.json` -- config holds choices a person makes and is hand-edited, and a machine-written value there gives one file two writers.
- **(ic, 2026-08-26) Nothing reads a cutoff out of the log, and `event::todo_watermark` is DELETED rather than kept as a fallback.** Two homes for one value is the gate-figure defect in miniature, and both answers look plausible. The migration rung reads the log exactly once.
- **(hv, 2026-08-26) The realised set is WIP ALONE. THERE IS NO 3.0.2.**
- **(ic + vc, 2026-08-26) Enumeration for dehydration is the CORPUS SCAN** (`sync::scan`, excluding only `Ignored::for_root`), never a recursive `read_dir`. A directory the run cannot empty is NAMED in the verdict.
- **(ic, 2026-08-26) A restore of a four-month-old commit is a MERGE, not a revert.** I missed an arm doing it: the buckets-and-view test counted `- [ ] `, written before rows carried a status glyph, so it had been matching nothing and asserting 0 against a populated view.
