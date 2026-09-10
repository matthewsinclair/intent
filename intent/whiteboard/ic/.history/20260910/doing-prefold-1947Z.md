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

### AC-02.1 -- DRIVEN BEFORE BUILT, AND I MISATTRIBUTED A FLAKE TO MYSELF ON THE WAY

**`AC-02.1` SATISFIED at `dd9782764`; `ST0073` 5/7 -> 6/7, only `AC-05.1` left.** vc offered a descope rather than have me build to a row written before the shape was understood, so the subject was constructed FIRST: pre-fix build **SURVIVED 8s** with its home gone; post-fix build **EXITED 3200ms**. Same probe, opposite verdicts.

**THE CONFOUND IS REMOVED RATHER THAN MANAGED, and it is the one the next person will walk into.** A lifeline FIFO must not live inside the home -- the obvious placement -- because removing the home closes the pipe and the daemon exits ON THE LIFELINE, reading as a clean pass while proving nothing. The arms start the daemon **SUPERVISED**: no owner, no pipe, nothing for an EOF to arrive on, so the lifeline cannot be the explanation and no assertion is needed to keep that true.

**AN INTERVAL HERE WHERE `AC-01.3` REFUSES ONE NEXT DOOR, with the reason at the function** so nobody concludes the lifeline could have been polled too. **The lifeline had an EXACT alternative and a directory has none:** a pipe's EOF is kernel-delivered on a descriptor that cannot be recycled while open; `notify` REFUSES A PATH THAT DOES NOT EXIST, so a watcher must be re-registered to notice the very event it exists for -- an interval wearing a watcher's name, plus a second watcher in a process already running a debouncer.

**`0302` FILED at `32ed04cb3`:** a live daemon outliving its own socket path -- unreachable, still holding the store, and **indistinguishable from a clean machine to `daemon status`**, which is `0301`'s population defect by a third route. `AC-02.1` closes the state-directory route only; anything unlinking the socket alone reproduces it.

**AND THE CORRECTION IS MINE AND IT IS THE THIRD OF ITS CLASS TODAY.** A peer's arm failed in the same run as my change, I ran it once without my change, it passed, and **I told vc the break was mine.** It was not: 6 runs with my change gave 5 pass / 1 fail, 6 runs at HEAD gave 5 pass / 1 fail, **failing on a DIFFERENT lifeline arm each time.** Pre-existing, same rate either side. **n=1 each side of a stochastic outcome is exactly the trap I warned vc about this morning**, and I committed it hours later while holding the warning.

**AND THE BASELINE RUN BEFORE THAT ONE WAS VOID AND I NEARLY REPORTED IT.** The `git checkout` hit a peer's `index.lock` and did not happen, so the "baseline" ran against my own files; and my pass-check grepped for `ok. 4 passed`, which a 6-arm file can never emit, so it reported six failures that belonged to the pattern rather than the tests. **Two instrument defects in one measurement, both returning a confident wrong answer.**

## TODO
