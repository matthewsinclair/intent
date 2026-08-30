---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 20:25Z
status: active
focus: "Guard landed 9fb602a1 (three homes, only the third load-bearing). Binary-resolution fix landed 238b7007 -- 20 arms were spawning a binary cargo did not build for them. backup.enabled is BUILT, GREEN and CONTROLLED AT BOTH LAYERS, waiting on vc's in-flight absent_at_check.sh pair. WP-08 AT column measured: four closures, one cause."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**`backup.enabled` IS BUILT AND GREEN AND CANNOT COMMIT YET.** Field on `BackupConfig` defaulting true, one reader in `backup::due`, new `Due::Disabled`, six arms. **281 pass in intentsvcs+intentd.** Blocked on vc's `absent_at_check.sh` being staged-but-not-in-HEAD, which `runner_roster_check.sh` correctly reds as an in-flight pair.

- **THE THREE ARMS THAT MATTER ASSERT WHAT IT DOES NOT DO.** A gate is easy to write and easy to over-apply, and an over-applied one fails where nobody looks: `intent backup` silently declining, doctor going quiet about a year-old backup. `cycle` and doctor are both ungated, deliberately.
- **ABSENCE-IS-ENABLED IS DRIVEN TWICE**, from a `backup` block omitting the key AND from a config with no block at all. Two different serde paths, and this estate has already shipped a version where typing one field silently emptied the reader of another in the same struct.
- **CONTROLLED AT BOTH LAYERS.** Removing the gate in `due` reds exactly one arm and leaves eighteen green -- which is also the evidence it reaches nothing else. Making `consider_backup`'s arm fall through to `cycle` reds the end-to-end arm through a real `intentd`: **exhaustiveness makes the compiler force you to HANDLE a variant, never to handle it correctly.**
- **THE FLAG HAD TO BE RENAMED.** `said_it_cannot_be_scheduled` was accurate for the only case that existed and would have been quietly wrong for the second; it is `said_why_it_is_not_backing_up`.

## TODO

- **`AT-08.4` / `AT-08.7` / `AT-08.9` citations are SENT to vc and are vc's to apply.** All three re-cite to `intent-cli/tests/`. 08.4 and 08.7 will cite the SAME file, so they need arm-level naming or greening one greens the other by inspection.
- **WP-13 (search, XL)** stays claimed and unbuilt; hv sequenced it post-tag.
- **`AC-08.9` follow-on is ic's** and they have taken it -- `AC-17.1` moved from blocked to buildable on the `/op` door.

## Watch-outs

**A PRIVATE `GIT_INDEX_FILE` CLOSES THE SHARED-INDEX CLASS THE BOARD HAD CALLED UNCLOSABLE, AND IT IS NOT FREE.** `read-tree HEAD`, `apply --cached` a filtered patch, `add`, commit. **`--only` gives path granularity and forces the whole file; `apply --cached` gives hunk granularity and forces the whole index; a private index gives BOTH.** Proved live: dc committed `1e19bdbb` between my reading the ambient index and my own commit, and it was a non-event.

**AND IT NEEDS A HEAD PIN OR IT CONVERTS A LOUD FAILURE INTO A SILENT ONE** (vc's ruling, on ic's near-miss: they built an index from `read-tree HEAD`, two commits landed before theirs, and it would have **silently reverted both** -- the roster gate's unrelated red is the only thing that stopped it. ic's words: _luck wearing a green's clothes_). **My own two are verified clean** -- `9fb602a1` on `1e19bdbb` and `238b7007` on `4640eab9`, each diffing to only my files -- **and that was ordering luck, not design.**

**THE PIN HAS TO BE ON BOTH SIDES OF THE GATE, AND THE RULED HALF IS THE LESS IMPORTANT ONE.** Checking HEAD before `git commit` closes the window between `read-tree` and staging, which is milliseconds. **The pre-commit gate runs between that check and git writing the commit object, and git resolves the parent AFTER the hooks** -- so a peer landing during the machine table, the 301 view comparisons and the 15-arm census gets a new HEAD as my parent while my tree is the old one. **A pre-check alone makes the technique feel safe across the window where it is least safe.** So: `base=$(git rev-parse HEAD)` before `read-tree`, refuse if HEAD moved before staging, and **after committing assert `HEAD^ == base`.** The post-verify does not prevent the reversion; it makes it LOUD -- which is the property `--only` had for free via `cannot lock ref 'HEAD'` and which a private index removes along with the contention.

**THE CLASS HAS THREE MEMBERS AND ONE IS CLOSED.** Contention (closed by `--only`, or by a pinned private index); COHERENCE (not closeable by the committer -- a half-landed pair is incoherent however carefully you scope); and REVERSION, which neither addresses. vc measured the third: `at.set` then `sync_from_disk` 1.2s later took their green AND the file-path correction, **and the row looked untouched afterwards.** D01 says the store is SSOT and `sync_from_disk` inverts it for one call. **Announce any disk->store sync before running it.**

**ITS COST, WHICH I CAUSED AND HAD TO CLEAN: THE COMMIT MOVES HEAD WHILE THE AMBIENT INDEX KEEPS THE PRE-COMMIT ENTRIES**, so `git status` reads `MM` and **the ambient index becomes a silent REVERSION of your own commit, waiting for the next plain `git commit` by anyone.** `git reset -q HEAD -- <paths>` clears it, no worktree bytes touched. It converts a contention hazard into a staleness hazard and the second one is quieter.

**AND IT DOES NOT ESCAPE THE OTHER HALF, WHICH IS THE MORE IMPORTANT FINDING.** Isolation is _do not take a peer's bytes_ and is solvable by tooling. **COHERENCE is _the tree I commit must make sense_, and a half-landed pair is incoherent however carefully I scoped my paths.** `--only` has the identical property and `runner_roster_check.sh` says so in its own text. **Only one half of the class is closable by the committer.**

**A GUARD'S AUTHORITY IS ITS MEMBERSHIP RULE, NEVER ITS NAME -- THIRD INSTANCE TODAY, THREE DIFFERENT ARTEFACTS.** vc's `populations.self_loop`; my arm 6b's hardcoded `case`; dc reading `table_driven_tests_fixture_their_home` as the guard over binary resolution when its rule is _does this spawn the binary at all_, for HOME fixturing. **In all three the name was a plausible description of the wrong set, and in all three the reader was careful.**

**AN UNTRACKED FILE CAN CHANGE WHAT A SHARED GUARD SAYS ABOUT EVERY NODE, WITH NO SIGNAL TO ITS AUTHOR.** My untracked `web.rs` embedded from `docs/design/`; arm 6b refused and **the refusal landed on vc**, because the guard runs at commit and my last commit predated the file.

**THE BINARY ON PATH IS NOT THE BINARY YOU BUILT, IN BOTH DIRECTIONS.** `~/.local/bin/intentd` has no web face; only `target/cc/debug/intentd` does, until `bin/devbin build all`. And 20 test arms were spawning `target/debug/intent` while cargo built `target/cc/debug/intent` -- **a private target dir INSIDE `target/` converts a crash into a wrong answer.**

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`.** A control that cannot fail certifies a test that cannot fail. `cargo build -p intentd` first.

**A UNIX SOCKET PATH HAS A LENGTH LIMIT AND THE SCRATCHPAD EXCEEDS IT** -- `SUN_LEN` at 143 bytes. `RealDaemon` uses `short_dir` for exactly this.

**rustfmt NEEDS `--edition 2024` HERE.** `--edition 2021` fails on let-chains with an error that looks like a code defect and formats nothing.

## Decisions

- (2026-08-30) **`backup.enabled` gates the sweep and nothing else** -- `cycle` ungated so `intent backup` still works, doctor ungated so staleness is still reported. vc's homonym ruling.
- (2026-08-30) **`Due::Disabled` is checked BEFORE `schedule`**, so an inert `backup.schedule` is not announced as a defect; doctor still reports it on its own path.
- (2026-08-30) **The binary-resolution guard is a NEW file, not an arm of the HOME guard** -- a binary-resolution arm inside `..._fixture_their_home` would be a home whose name does not describe its contents.
- (2026-08-30) **Attachments are AUTHORED and no sync direction rewrites them.** `--to-store` takes the EXTRACT, is destructive, and bare replaces the whole store: scope it to the thread.
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0**; the HTTP body is `wire::frame`'s bytes; `Op::Shutdown` refused over HTTP.
- (2026-08-30) **51737 is a preference, never a promise** -- ask for it, fall back to a kernel port, publish what was bound.
