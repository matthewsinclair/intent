---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-13 16:41Z
status: active
focus: "LOCALFOLDED for hv's compact, mid-task: 0354 reproduction for vc. Run 1 reproduced the spin with no client; run 2 (no warm-up, then one open) is running detached; run 3 (the 3.0.1 keg control) launches when run 2 ends. The plan and every evidence path are in DOING. The pre-tag suite gap on e70c3528a is green and accepted. Nothing under native/, surface/, lib/ or bin/. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/02, ST0069/14, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

## DOING

- **STEP 3 DONE, awaiting vc's 0354 verdict (vc's CPU watch on the rebuilt daemon).** The delivered set is at 9e60fcfb6: intent sha256 4ae769e8aba0634f, intentd abacbc11a7969a8a, Intent.app 3.0.1 build 7001 (Contents/MacOS/Intent 60161795505de63e). The suite was green on be0bac331, and nothing under native/, surface/, bin/ or lib/ moved from there to 9e60fcfb6. The first build all refused on the stamp (a peer's board commit moved HEAD mid-build); the one re-run was verified and promoted. app-install exited 0, the daemon restart ended 58837 and the live daemon is pid 18592, doctor exited 0 with 0 findings. Hashes are with vc; cc, ic and vc were told build done. Logs are in scratchpad rebuild/. Next is vc's call (Laksa on the pair's hash, then the cut is hv's). NO RELEASE, NO PUSH.

## TODO

- **Later, on vc's signal only**: one preflight line running ic's `contract_check.sh` (`intent/st/ST0056/parity/tools/contract_check.sh`; 0 clean, 1 findings, 2 environment/usage). **ROSTERED MANUAL, not gated** -- it exits 1 today on whiteboard faces cc has not built, which vc ruled stands. Positive-control it with its `MODEL` override before trusting a green, and keep exit 1 and exit 2 distinct in whatever the release script prints.

## Holds

- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **A SECTION NOTHING MAPS USED TO RETURN BEFORE ITS LINES WERE COUNTED**, so they entered no denominator and the reconciliation was a claim about the sections the reader already understood. **A denominator that excludes what the instrument cannot see agrees with itself perfectly.** Count where the unit is dispatched, whatever becomes of it.
- **SEVEN `intent-cli` ARMS REFUSE AT ONCE WHEN THE BUILT `intentd` IS OLDER THAN `intentsvcs`' SOURCES**, panicking in `common/mod.rs` with nothing about the tests in the message. It is the harness's own staleness guard and it is right. `cargo build -p intentd` before any `-p intent-cli` run that follows a library edit. It reads exactly like a regression and is not one.
- **AN EXIT-CODE ASSERTION IS ABOUT THE WHOLE ESTATE THE FIXTURE LEAVES.** 0313's arm planted a stale store with `write_thread`, which also skews every generated view -- and view skew blocks -- so the exit code would have been 1 for a reason with nothing to do with the arm, and would have passed for the wrong reason if the class were ever put back. Seed the fixture so the thing under test is the only thing wrong.
- **A NEW MECHANISM HAS NO BASELINE TO REDDEN, SO MUTATE THE IMPLEMENTATION INSTEAD.** Six mutations this session, each reddening the arm it should: holds as `Todo`, snapshots unwritten, the claimed stamp dropped, the unregistered-sender check removed, an unmapped section dropped rather than named, and `legacy` carried across a re-cite.
- **SETTLE A DAEMON-FAMILY RED BY THE SHAPE OF THE FAILING SET, NOT BY RE-RUNNING TWICE.** Different arms red in each of two runs at loads of 85 and 49 is the load signature; a STABLE failing set is the one that means something. Structural dismissal needs the change provably unable to reach the arm -- verified, never asserted.
- **`git apply --3way` ONTO A TREE THREE PEERS ARE WRITING NEEDS THE LOCK LOOP, AND THE LOOP IS `until [ ! -f .git/index.lock ]`.** Chained `sleep`s are refused by the harness; re-issue the SAME command after the wait, never a recomposed one.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, literal paths. Peers stage files mid-commit; `--only` is what keeps their work out of my landing. Three peers were landing into this tree all session and nothing of theirs reached my commits.
- **Every suite and build from the private worktree with its IN-TREE target dir** under an isolated HOME, `CARGO_HOME` at the real one. Rebase the worktree onto main before measuring, or the numbers describe a tree nobody has.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the SAME CALL as the write.**
- **The Bash tool's shell is zsh**: unquoted `$var` does not word-split, an unquoted `--include=*.rs` aborts the whole command, and messages go in a file through `-F`.
- **`cargo test -p intent-cli --bins <filter>` RAN ZERO TESTS AND EXITED 0.** `render.rs` is compiled by the LIBRARY target `intent_cli`; the `intent` bin holds no tests at all, so `0 filtered out` meant the binary has none, not that the filter missed. Read `running N tests` before trusting any green. `--lib` is the target for in-crate tests there, and `--test suite` for the integration arms in both crates.
- **NEVER `git commit --only` A PATH A PEER HAS UNCOMMITTED EDITS IN** -- it takes the whole working file, so their in-flight work lands under your message. Wait for their landing, rebase, then land. **And guard a landing on NOTHING UNDER `native/rust` HAVING MOVED SINCE THE MEASURED BASE, not on HEAD equality**: three peers move HEAD every few minutes and an equality guard starves the landing, while the narrower guard keeps the measurement honest and lets the commit message name the base it was measured on.
- **`wb` WRITES GO IN ONE SEQUENTIAL CALL.** Parallel store writes from here hit `sqlite: database is locked`, and an `archive` that succeeded before a failed `add` left the board with no live item for one command. Re-issue the SAME command. Under the pair at 59eeac618 a wb write does not reliably project its view to disk (0317 fixes that), so compare the view with `wb show --json` before committing it.
- **A PROBE FOR A REMEDY RUNS IN THE EXACT ENVIRONMENT OF THE RUN IT VOUCHES FOR, OR IT VOUCHES FOR A DIFFERENT RUN.** Final rehearsal 2026-09-13: gh keeps its token in the macOS keyring, which an isolated HOME cannot reach. I probed `gh auth status` with HOME plus GH_TOKEN, it passed, and run3 carried HOME plus GH_CONFIG_DIR plus GH_TOKEN -- where gh lists the stored keyring account beside the token, fails it, and exits 1 because ANY account failed. Run3 halted at the gh gate. Build the probe from the run script, not from memory of it: the same exports, the same unsets. The rehearsal now carries GH_TOKEN and no GH_CONFIG_DIR, and probes inside the run before the load gate.
- **A DAEMON PROBE'S ISOLATED HOME MUST BE SHORT.** intentd binds $HOME/.local/share/intent/intentd.sock, and sockaddr_un holds about 104 bytes. A HOME under the scratchpad made the path 153 bytes, and the daemon refused to bind ('path must be shorter than SUN_LEN'). Use /tmp/<short>. **AND IN THE BASH TOOL, pgrep CAN RETURN TWO PIDS** (a script and its own subshell): zsh does not word-split, so `kill $P` failed with 'illegal pid', and an unbounded wait loop after it spun until stopped. Kill the literal pid, and bound every wait.
- **NEVER EDIT A SCRIPT A RUNNING bash IS EXECUTING; COPY IT.** bash reads a script from the file as it goes, by byte offset, so an insertion above the line it is executing shifts every later read and it resumes on the wrong bytes: a re-run open, a second wait, a syntax error. Caught 2026-09-13 in the 0354 probes: run-4 counts were inserted into probe.sh while run 3 was still inside it, and the original bytes were restored (0 COUNTS lines, syntax ok) before run 3 reached its next read. Run 4 got its own copy, probe4.sh.

## Decisions

- (2026-09-13) **`authored_at` is a PARAMETER on the two shipped wb insert writers, not a second INSERT beside them.** One insert per table: two spellings of one row is how one door ends up carrying a field the other drops, and the drift is invisible because each door is self-consistent. The service still reads the clock for `recorded_at` in the same statement and has no parameter for it.
- (2026-09-13) **A `.history/` snapshot is a DOCUMENT and gets its own half of the prose table (`wb_node`), scoped per node.** The canon ingest's wholesale replace cannot reach it and neither can the file indexer's, which is the arrangement `owner_type` already exists to make possible. The alternative -- canon-half rows re-derived from disk at every ingest -- would have put a node's history on the wrong side of a `DELETE` that runs constantly.
- (2026-09-13) **`StoreStale` is its own finding class because a DERIVED cache being behind says nothing about what a commit carries.** On a shared tree it is the normal state for the duration of every peer's write, and a refusal that clears itself with nobody acting teaches nodes to re-run gates.
- (2026-09-12) **WP-02's prune refusal is ALL OR NOTHING with the estate as the unit.** One unheld path refuses every removal and names each one. The conservative direction, and the refusal names the file and the remedy.
- devbin `0047` (hv, 2026-09-01): option 3, the split. Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.

---

_Generated by Intent v3.0.1 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
