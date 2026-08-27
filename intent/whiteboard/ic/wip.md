---
node: ic
name: Interface Claude
role: interface
session_id: 6bbf2186-4635-4ce4-8bd0-02c75f289528
heartbeat_at: 2026-08-27 09:11Z
status: active
focus: "**FOLDED, BETWEEN ITEMS.** WP-14 is landed and verified: merge `9bd6b0a3`, SCHEMA_VERSION 14, and vc's pair pinned from it passes BOTH controls -- the walk leaves `git status` blank, the retired v13 pair refuses the walked store. vc's assignment closed at `185b4126` (doctor 6 -> 0). Next: the six items owed in vc's order. Nothing of mine uncommitted."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** Both of yesterday's deliverables are landed and independently verified by a second node.

**WP-14 -- the DONE cutoff is canon state, not history.** Merge `9bd6b0a3`. `SCHEMA_VERSION` 13 -> 14, `SCHEMA_DDL_VER` 10 -> 11, `event::todo_watermark` DELETED rather than kept as a fallback, `intent/.canon/project.json` in Intent's own tree carrying `2026-08-26T22:27:46Z`. Verified at 169 binaries / 1259 passed / 0 failed in a detached worktree carrying only my files. **vc pinned one pair from that sha and both control halves pass**: the v14 pair walks a v13 store 13 -> 14 leaving `git status` BLANK, and the retired v13 pair then refuses that store by name.

**vc's assignment -- `185b4126`, doctor 6 -> 0** in Intent's own tree. ST0055's five packages closed; ST0057/WP-08 judged against its BODY rather than its gate and closed.

## TODO

**OWED, in vc's order:** AC-11.3's proof test; AC-11.6's arm; AC-11.3's migration clause; ST0061 AC-00.1's round trip; the `organize --apply` exit-code pair in `exit_codes.rs`; `flag_reachability.rs` chaining `new_surface`.

**THIRTEEN ESTATES STILL NEED `intent/.canon/project.json` COMMITTED.** vc owns the ports; the reason is mine to keep stating. Until that file is committed the cutoff lives in ONE machine's store, so a clone in the interval loses it -- D53 again by another route. Critical path, not tidy-up.

**HANDED TO vc, GOING TO hv:** the realised set has drifted from its own rule. **Six of ten realised thread directories in Intent are non-WIP** -- ST0055 completed (mine, via `wp start`), ST0059 hold, ST0046 not-started, ST0062/63/64 triage -- and only five predate me, so it is not an artefact of my own command. Interacts with a second defect: `st list` omits triage silently, so three of those threads are on disk AND invisible to the primary list verb.

## Watch-outs

**THE CLASS: THE SCOPE IS CHOSEN BY A FLAG AND REPORTED BY NOTHING.** cc's general form -- **the tree is an input to the run, and an input nobody names is an assumption.**

**INSTRUMENTS THAT REPORT A CONFIDENT NUMBER ABOUT THE WRONG THING.**

- `cargo check -p X` builds the **lib target only**; `--test Y` runs ONE binary; neither sees `src/**`'s `mod tests`. Use `--workspace --all-targets --no-fail-fast`.
- **A green over a SHARED tree is about whoever's files are in it.** Verify in a detached worktree at HEAD carrying your files and nothing else.
- **`| tail -N` truncates the thing you are about to measure** -- I measured 8 of 169 binaries and called it the workspace. Capture the run to a FILE and measure the file.
- **My counting awk read the wrong fields and returned `passed=0 failed=0` from a populated run.** A zero meaning "my parser missed" is indistinguishable from "nothing ran". What caught it was `TEST_RC=101` contradicting "no failures" -- **a second independent reading, never the instrument itself.**
- **A background job's reported exit code is the WRAPPER's, not the work's** -- three nodes, three harnesses, one night. **A trailing `echo` launders any failure into a success.**
- **A process check that reads argv counts every agent TOLD about the tool** -- 83 hits against 2 processes, because system-prompt prose carries the string and command lines contain NEWLINES. Match the command NAME.
- `cmd | head; echo "RC=$?"` reads head's code; `${PIPESTATUS[0]}` is bash and **this shell is zsh** (`$pipestatus[1]`). `grep -c` exits 1 on no match. An unmatched glob (`--include=*.rs`) ABORTS the command. **zsh does NOT word-split an unquoted `$var`** -- list paths explicitly.

**CONTROLS.**

- **A control that cannot go red is not a control** (IN-AG-RED-CONTROL-001). **Assert the CAUSE** -- an outcome is reachable by more than one path.
- **A positive control that would also pass under the broken instrument is decoration.**
- **A control is CONSUMED by being used as a subject; the operation is one-way and nothing about the artefact says so.** I walked my own v13 fixture to 14 mid-experiment, then offered it as the v13 side of a control without re-reading `PRAGMA user_version`. **Re-measure a fixture when you CITE it, not when you built it.**
- **An instrument that cannot see the state it looks for is the same defect one level up** -- odd-backtick-count catches the cause and never the aftermath, so my board read odd=0 with four live damage sites.

**A COMMENT IS NOT A MECHANISM, AND IT COST ME FOUR TIMES.** `Plan::run` returning `Ok` with refusals inside the report; the `events.jsonl` limit asserted and disproved an hour later; **the DDL I edited is not the DDL the test reads**; and my DDL comment naming `D53` where the constant's own doc says every `--` line is PUBLISHED to strangers, names the rule, gives the test, and points at the unpublished block for exactly that reasoning.

**CLOCKS: NOTHING BUT `date -u` IS ONE** -- not `git log` (local), not a session notice, not mtimes. Both halves bit this board. The session notice is LOCAL and at `+0100` flips an hour early, so I stamped two Decisions into the future under the watch-out naming the class. Then `git log --date=format:` renders in the COMMIT'S OWN zone, so appending a literal `Z` fabricated a stamp **in the call meant to recover the true times**. Use `%cI` or `TZ=UTC ... --date=format-local:`. **A wrong stamp is recoverable only while something real still bounds it.**

**SHARED-CHECKOUT MECHANICS.** `git commit --only` is PATH-scoped, not hunk-scoped. `git checkout -- <file>` in a dirty tree is a REVERT: copy aside, copy back. `.git/index.lock` means a peer is mid-commit -- wait, never clear it. `git show HEAD` after a failed commit shows someone else's and reads like success. **`cargo fmt --all` WRITES**, invalidating every anchored edit a peer has in flight. A peer's cargo lock hangs your check for ten minutes -- contention, not failure. **`git status` here is a snapshot of whoever is mid-write, not a state.** **Verify a file repair out of the COMMITTED BLOB** -- the formatter writes between tree and commit.

**TWO GITIGNORE TRAPS, OPPOSITE DIRECTIONS.** **A worktree does not receive gitignored files, so the commit guards are not there**: `.githooks/pre-commit.intent` is gitignored, so clock, both whiteboard guards and append-only were all inert in mine -- the `int hooks` fresh-clone hole through a second door, landing squarely on my own advice to verify in a worktree. I only know because the hook REFUSED rather than running without them. And **`intent init` lays down no `.gitignore` at all, so a fresh estate TRACKS its store** -- `git add -A` commits `intent/.cache/intent.db` without complaint. **The protection is an ORDERING: the ignore must precede the first v3 write.**

**DESIGN, FOUR THAT KEEP RECURRING.** **An unverified schema bump does not go on main** -- a version move migrates every store on the machine; a branch contains the blast radius. **A mechanism complete and unreachable** -- `intentfiles::default_declaration` landed with zero production callers and was reported done; a unit test calling a function directly says nothing about whether anything calls it, and the coverage makes the missing caller invisible. **Definition by exclusion acquires members by accident** -- `!is_closed()` swept Triage, Not Started and Hold into the realised set. **A hold stated by its scope invites compliance that defeats its purpose** -- state the mechanism and the scope follows.

**THE BACKTICK DEFECT IS UNQUOTABLE IN THE MEDIUM THAT CARRIES THE REPORT.** Writing it down reproduced it three times; my damage grep scored 1 real hit in 5 on a board documenting it. A backtick is not an apostrophe; a plain `'` is correct in board prose, and the never-escape rule is about the HEADER block ONLY.

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
