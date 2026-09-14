---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-14 21:58Z
status: paused
focus: "DAY CLOSED 2026-09-14T21:55Z on hv's word. Batch 2 (vc's seven) LANDED on main: C1 000521c0a (0393, index deletes as one statement per table) and C2 c4c4880c2 (0394 st detach, 0315 uncommitted report, 0323 init gitignore, 0316 organize footer gate + upgrade dehydration, 0320 closed-issue views, 0397 v2 id reader, ST0057 AC-04.2 reworded). Nothing in flight, nothing unlanded in wt-dc. Holds for hv: tap formula 9987a93 unpushed; the 3.0.2 hook-migration sweep (todo 8). NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/02, ST0069/14, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

## DOING

_(none)_

## TODO

- **Later, on vc's signal only**: one preflight line running ic's `contract_check.sh` (`intent/st/ST0056/parity/tools/contract_check.sh`; 0 clean, 1 findings, 2 environment/usage). **ROSTERED MANUAL, not gated** -- it exits 1 today on whiteboard faces cc has not built, which vc ruled stands. Positive-control it with its `MODEL` override before trusting a green, and keep exit 1 and exit 2 distinct in whatever the release script prints.
- **For the cut's install sequence, awaiting hv's ruling (raised by ic, relayed by vc):** with ST0074 WP-05, every estate's old .git/hooks/pre-commit.intent still reads ~/.intent/home, and the first 3.0.2 command moves that file. Those estates then refuse commits until intent claude upgrade --apply runs in each one; intent bootstrap does not fix it. The options are a sweep step in dc's install sequence, or ic's option (b). Nothing is built until hv rules.

## Holds

- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **SEVEN `intent-cli` ARMS REFUSE AT ONCE WHEN THE BUILT `intentd` IS OLDER THAN `intentsvcs`' SOURCES**, panicking in `common/mod.rs` with nothing about the tests in the message. It is the harness's own staleness guard and it is right. `cargo build -p intentd` before any `-p intent-cli` run that follows a library edit. It reads exactly like a regression and is not one.
- **SETTLE A DAEMON-FAMILY RED BY THE SHAPE OF THE FAILING SET, NOT BY RE-RUNNING TWICE.** Different arms red in each of two runs at loads of 85 and 49 is the load signature; a STABLE failing set is the one that means something. Structural dismissal needs the change provably unable to reach the arm -- verified, never asserted.
- **`git apply --3way` ONTO A TREE THREE PEERS ARE WRITING NEEDS THE LOCK LOOP, AND THE LOOP IS `until [ ! -f .git/index.lock ]`.** Chained `sleep`s are refused by the harness; re-issue the SAME command after the wait, never a recomposed one.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, literal paths. Peers stage files mid-commit; `--only` is what keeps their work out of my landing. Three peers were landing into this tree all session and nothing of theirs reached my commits.
- **Every suite and build from the private worktree with its IN-TREE target dir** under an isolated HOME, `CARGO_HOME` at the real one. Rebase the worktree onto main before measuring, or the numbers describe a tree nobody has.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the SAME CALL as the write.**
- **The Bash tool's shell is zsh**: unquoted `$var` does not word-split, an unquoted `--include=*.rs` aborts the whole command, and messages go in a file through `-F`.
- **`cargo test -p intent-cli --bins <filter>` RAN ZERO TESTS AND EXITED 0.** `render.rs` is compiled by the LIBRARY target `intent_cli`; the `intent` bin holds no tests at all, so `0 filtered out` meant the binary has none, not that the filter missed. Read `running N tests` before trusting any green. `--lib` is the target for in-crate tests there, and `--test suite` for the integration arms in both crates.
- **NEVER `git commit --only` A PATH A PEER HAS UNCOMMITTED EDITS IN** -- it takes the whole working file, so their in-flight work lands under your message. Wait for their landing, rebase, then land. **And guard a landing on NOTHING UNDER `native/rust` HAVING MOVED SINCE THE MEASURED BASE, not on HEAD equality**: three peers move HEAD every few minutes and an equality guard starves the landing, while the narrower guard keeps the measurement honest and lets the commit message name the base it was measured on.
- **`wb` WRITES GO IN ONE SEQUENTIAL CALL.** Parallel store writes from here hit `sqlite: database is locked`, and an `archive` that succeeded before a failed `add` left the board with no live item for one command. Re-issue the SAME command. Under the pair at 59eeac618 a wb write does not reliably project its view to disk (0317 fixes that), so compare the view with `wb show --json` before committing it.
- **A DAEMON PROBE'S ISOLATED HOME MUST BE SHORT.** intentd binds $HOME/.local/share/intent/intentd.sock, and sockaddr_un holds about 104 bytes. A HOME under the scratchpad made the path 153 bytes, and the daemon refused to bind ('path must be shorter than SUN_LEN'). Use /tmp/<short>. **AND IN THE BASH TOOL, pgrep CAN RETURN TWO PIDS** (a script and its own subshell): zsh does not word-split, so `kill $P` failed with 'illegal pid', and an unbounded wait loop after it spun until stopped. Kill the literal pid, and bound every wait.
- **NEVER run intent claude upgrade --apply anywhere from this tree with the installed 3.0.1 binary** (vc, 2026-09-13). The pre-commit hook migration for WP-05's XDG move is hv's to rule, and the step is part of the cut's sequence, not an ad-hoc fix.
- **ONE ATTEMPT PER STORE WRITE, NEVER A RETRY LOOP.** A wb or store write retried past 'database is locked' holds the lock for the loop's whole life: dc's focus loop refused ic's 0307 closure three times (2026-09-14, stopped by vc), and cc's scripted fold refused dc's batch-2 landing twice mid-chain. On a lock: lsof intent/.cache/intent.db, wait on THAT pid bounded, re-issue once. A landing chain that meets a peer's write stops, reports its exact partial state, and resumes from it.
- **A NEW DISPATCH ROW MOVES A DECLARED COUNT THE CARGO SUITES CANNOT SEE.** surface/dispatch-table.json's legal_pairs carries a per-pair n with a dated census_note; adding a v2 new-surface row (st detach, 2026-09-14) made it 73 against a corpus of 74, and only the commit gate's view_skew_check caught it, after a green full run. Bump n with its census_note entry and regenerate surface/dispatch-table.md with intent/st/ST0056/parity/tools/gen_dispatch_table.sh in the SAME change as the row.

## Decisions

- (2026-09-13) **`authored_at` is a PARAMETER on the two shipped wb insert writers, not a second INSERT beside them.** One insert per table: two spellings of one row is how one door ends up carrying a field the other drops, and the drift is invisible because each door is self-consistent. The service still reads the clock for `recorded_at` in the same statement and has no parameter for it.
- (2026-09-13) **A `.history/` snapshot is a DOCUMENT and gets its own half of the prose table (`wb_node`), scoped per node.** The canon ingest's wholesale replace cannot reach it and neither can the file indexer's, which is the arrangement `owner_type` already exists to make possible. The alternative -- canon-half rows re-derived from disk at every ingest -- would have put a node's history on the wrong side of a `DELETE` that runs constantly.
- (2026-09-13) **`StoreStale` is its own finding class because a DERIVED cache being behind says nothing about what a commit carries.** On a shared tree it is the normal state for the duration of every peer's write, and a refusal that clears itself with nobody acting teaches nodes to re-run gates.
- (2026-09-12) **WP-02's prune refusal is ALL OR NOTHING with the estate as the unit.** One unheld path refuses every removal and names each one. The conservative direction, and the refusal names the file and the remedy.
- devbin `0047` (hv, 2026-09-01): option 3, the split. Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
