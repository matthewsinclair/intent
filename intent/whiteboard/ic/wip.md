---
node: ic
name: Interface Claude
role: interface
session_id: 6bbf2186-4635-4ce4-8bd0-02c75f289528
heartbeat_at: 2026-08-26 23:28Z
status: active
focus: "**FOLDED, HOLDING FOR vc's INSTRUCTIONS.** Today's two deliverables are on main: `c14aa9bf` (the DONE cutoff is an ISO instant and the heading is `## DONE:<T>`, v2 parity, hv drove it and it works) and `2d0387f7` (WP-14 designed). **WP-14's CODE IS ON A BRANCH AND NOT ON MAIN, DELIBERATELY: `ic/wp14-cutoff-is-state` at `225b9c88`.** It carries SCHEMA_VERSION 13 -> 14, and a version bump migrates every node's store on this machine -- unverified, so it does not go where it would migrate anybody. Four things are NOT done and the commit message names all four; the first is that `schema/ddl.sql` is a PUBLISHED FACE and `openness.rs` reads THAT, not `store.rs`. My paths on main are clean."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**HOLDING FOR vc.** Nothing of mine is uncommitted on main.

**SHIPPED TODAY.** `c14aa9bf` -- the cutoff is a full ISO instant, the heading is `## DONE:<T>`, a completion date is widened to midnight before the compare (v2's `normalize_completed`), and a flush now clears work finished TODAY. Three arms, each mutation-proven to red alone. `7211cf5c` regenerated `todo.md`; hv drove `--prune` against it and it works. `2d0387f7` -- WP-14's design, written before its code.

**PARKED ON A BRANCH: `ic/wp14-cutoff-is-state` @ `225b9c88`.** The cutoff moves from the event log to `intent/.canon/project.json` as STATE. New `project` singleton table + rung 14 seeding it once from the log; `ProjectState`; `carry_project_state` on both disk-to-store paths; `todo_flush` records history AND state; `doctor` reads the FILE, which is what lets it answer with no store; **`event::todo_watermark` DELETED rather than left as a fallback.**

## TODO

**WP-14's FOUR OPEN ITEMS, in the commit message verbatim.** (1) **`schema/ddl.sql` is a published face and it is stale** -- `openness.rs` reads that file, so the new table's declaration is currently untested and `schema_versioning.rs` will disagree on the version. (2) `init` writes no canon files, so a fresh project has no `project.json` and the `carried by` path may dangle for AC-01.7. (3) The full workspace run had not finished when I folded; the last completed `cargo check --workspace --all-targets` was clean. (4) The two tests this exists to fix have not been re-run against it.

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

**AN UNVERIFIED SCHEMA BUMP DOES NOT GO ON MAIN.** A version move migrates every node's store on this machine the next time each runs a command. A branch costs nothing and contains the blast radius.

**A MECHANISM COMPLETE AND UNREACHABLE, three instances in one evening**, mine being `intentfiles::default_declaration` landed with ZERO production callers and reported done. **A unit test calling a function directly proves the function works and says nothing about whether anything calls it -- and the coverage is what makes the missing caller invisible.**

**DEFINITION BY EXCLUSION ACQUIRES MEMBERS BY ACCIDENT.** `!is_closed()` swept Triage, Not Started and Hold into the realised set. Stated positively (`status == Wip`) a new status is not realised until somebody decides it is.

**NOTHING BUT `date -u` IS A CLOCK** -- not `git log` (local time), not a session notice, not file mtimes. **And nothing but the command NAME is a process.**

**A POSITIVE CONTROL THAT WOULD ALSO PASS UNDER THE BROKEN INSTRUMENT IS DECORATION.**

**A HOLD STATED BY ITS SCOPE INVITES COMPLIANCE THAT DEFEATS ITS PURPOSE.** State the mechanism and the scope follows.

## Decisions

- **(hv, 2026-08-26) The DONE heading is `## DONE:<T>`, <T> a full ISO 8601 UTC instant** -- the time the prune ran, hence the cutoff. Verbatim v2.
- **(hv, 2026-08-26) A flush CLEARS same-day work.** A completion date widens to that day's midnight, which is below any flush later that day. The day-granular watermark that avoided this is what made a DONE bucket unemptiable on the day the work was done.
- **(hv, 2026-08-27) WP-14: the cutoff is CANON STATE, not history.** A flush HAPPENING at T is an event and stays in the log where D53 put it; the cutoff BEING T is a fact about the project now. **Filing it as history is what put it on the wrong side of D53, and that was mine to have seen.** It goes in `.canon/project.json`, not `config.json` -- config holds choices a person makes and is hand-edited, and a machine-written value there gives one file two writers.
- **(ic, 2026-08-27) Nothing reads a cutoff out of the log, and `event::todo_watermark` is DELETED rather than kept as a fallback.** Two homes for one value is the gate-figure defect in miniature, and both answers look plausible. The migration rung reads the log exactly once.
- **(hv, 2026-08-26) The realised set is WIP ALONE. THERE IS NO 3.0.2.**
- **(ic + vc, 2026-08-26) Enumeration for dehydration is the CORPUS SCAN** (`sync::scan`, excluding only `Ignored::for_root`), never a recursive `read_dir`. A directory the run cannot empty is NAMED in the verdict.
- **(ic, 2026-08-26) A restore of a four-month-old commit is a MERGE, not a revert.** I missed an arm doing it: the buckets-and-view test counted `- [ ] `, written before rows carried a status glyph, so it had been matching nothing and asserting 0 against a populated view.
