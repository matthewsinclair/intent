---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 21:32Z
status: paused
focus: "FOLDED 2026-08-30 21:31Z, pre-fold at .history/20260830/wip-fold-2131Z.md. WP-08 is 12/12 and backup.enabled was its last unimplemented ratified key (42402762). Nothing of mine is uncommitted and nothing is owed to me. On the bounce: WP-13 (search, XL) is the only thing I hold that is unbuilt, and hv sequenced it post-tag -- so ASK before starting it."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** WP-08 closed 12/12 today; `backup.enabled` landed at `42402762` and was the last ratified key with no reader. My worktree is clean, no commit is parked, and no peer is waiting on me.

## TODO

- **WP-13 (search, XL) is the only claim I hold that is unbuilt, and hv sequenced it POST-TAG.** Do not start it on the bounce without asking -- taking it early would reorder a human's sequencing by the quiet route, which is a node accepting work rather than disagreeing with anyone.
- **`AC-17.1` and the MCP tier are ic's**, and both consume doors I built (`/op`, `dispatch`). Answer questions; do not build into them.
- **A `bin/devbin build all` is owed before anyone can browse the web face from the delivered binary.** `~/.local/bin/intentd` has no web face; only a dev-tree build does.

## Watch-outs

**A GUARD'S AUTHORITY IS ITS MEMBERSHIP RULE, NEVER ITS NAME. FIVE INSTANCES ON 2026-08-30 ACROSS FOUR NODES.** vc's `populations.self_loop`; my arm 6b's hardcoded `case`; dc reading `table_driven_tests_fixture_their_home` as the guard over binary resolution; dc's `facade.rs` kind rule; vc catching my arm undercount. **Every one of us was being careful, and in three of the five the CONCLUSION WAS STILL CORRECT** -- which is exactly why they survived. Before citing a rule, read what its set is defined by, not what it is called.

**AND MY OWN WAS THE WORST-SHAPED: I APPLIED THE DISCIPLINE TO THE FILE I WAS ATTACKING AND NOT TO THE FILE I WAS DEFENDING.** I read all six arms of the file I was arguing against and truncated my grep at eight lines on the file I was arguing for. **An undercount that still supports your conclusion does not feel like something to re-check.**

**COMMITTING IN A SHARED CHECKOUT IS THREE PROBLEMS AND ONLY ONE IS CLOSABLE BY THE COMMITTER.**

- **CONTENTION** -- taking a peer's bytes. Closed by a HEAD-pinned private index (below), or `--only` where whole files suffice.
- **COHERENCE** -- the tree you commit must make sense. **NOT closeable by you**: a half-landed pair is incoherent however carefully you scope, and on 2026-08-30 one blocked EVERY node in the estate for ~20 minutes. The remedy is the guard's own: land the pair, or wait.
- **REVERSION** -- vc's find. `at.set` then `sync_from_disk` 1.2s later took their green AND a path correction, **and the row looked untouched afterwards.** D01 makes the store SSOT and `sync_from_disk` inverts it for one call. **Announce any disk->store sync before running it.**

**THE PINNED PRIVATE INDEX, AND IT IS ONLY SAFE PINNED ON BOTH SIDES.** `base=$(git rev-parse HEAD)`; `read-tree $base`; stage; refuse if HEAD moved; commit; **then assert `HEAD^ == base`.** Script kept at the session scratchpad; procedure is what matters.

- **Why both sides:** git resolves the parent AFTER the hooks, so **the pre-commit gate's entire runtime sits between the pre-check and the commit object.** A pre-check alone makes it feel safe across the window where it is least safe. ic nearly lost two of vc's commits to the unpinned form.
- **Why pin at all:** `--only` and plain `commit` lose LOUDLY on `cannot lock ref 'HEAD'`, and that noise is why no contention incident lost data. **A private index removes the contention and therefore the noise.** The post-verify buys the noise back.
- **Its other cost:** the commit moves HEAD while the AMBIENT index keeps pre-commit entries, so **the ambient index becomes a silent reversion of your own commit** awaiting anyone's next plain commit. `git reset -q HEAD -- <paths>` in the SAME turn as the commit, not as a cleanup step.

**A TRUE CLAIM CAN HAVE A SHELF LIFE SHORTER THAN THE MESSAGE CARRYING IT.** dc warned me five of my paths were staged as pending reversions; true when sent, false when it arrived, because I had already reset them. Re-measure on receipt -- and dc was right to tell me rather than fix it, because **resetting another node's index entries is the same offence as taking their bytes.**

**AN UNTRACKED FILE CAN CHANGE WHAT A SHARED GUARD SAYS ABOUT EVERY NODE, WITH NO SIGNAL TO ITS AUTHOR.** The guard runs at commit; if your last commit predates the file, the refusal lands on whoever commits next.

**THE BINARY ON PATH IS NOT THE BINARY YOU BUILT, IN BOTH DIRECTIONS.** `~/.local/bin/intentd` has no web face. And test files resolving `target/debug/intent` by hand ignore `CARGO_TARGET_DIR` entirely -- **a private target dir INSIDE `target/` turns that from a crash into a wrong answer.** Guarded now by `the_binary_under_test_is_the_one_cargo_built.rs`; `env!("CARGO_BIN_EXE_intent")` is the only correct spelling.

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`.** A control that cannot fail certifies a test that cannot fail. `cargo build -p intentd` first.

**A UNIX SOCKET PATH HAS A LENGTH LIMIT AND THE SESSION SCRATCHPAD EXCEEDS IT** (`SUN_LEN`, 143 bytes). `RealDaemon` uses `short_dir` for exactly this.

**NEVER START `intentd` UNDER THE REAL `$HOME` WHILE PEERS ARE LIVE** -- it takes the store exclusively and refuses every peer's store verbs at once. An isolated short `HOME` costs nothing.

**rustfmt NEEDS `--edition 2024` HERE.** `--edition 2021` fails on let-chains with an error that reads like a code defect and formats nothing. And format BEFORE staging: each gate refusal re-opens the index window it is protecting.

**EXHAUSTIVENESS MAKES THE COMPILER FORCE YOU TO HANDLE A VARIANT, NEVER TO HANDLE IT CORRECTLY.** A match arm falling through satisfies every unit test of the decision function. Drive the effect, not the verdict.

## Decisions

- (2026-08-30) **`backup.enabled` gates the daemon sweep and NOTHING else** -- `cycle` ungated so `intent backup` still works, doctor ungated so staleness is still reported. vc's homonym ruling, quoted at the field itself.
- (2026-08-30) **`Due::Disabled` is checked BEFORE `schedule`**, so an inert value is not announced as a defect; doctor still reports it on its own path.
- (2026-08-30) **A new guard gets a file named for its contract**, never an arm inside one whose name describes something else.
- (2026-08-30) **Attachments are AUTHORED; no sync direction rewrites them.** `st attach <ID> <rel> --from <file>` is the narrow disk->store door; `--to-store` is the recovery one, destructive, and bare replaces the WHOLE store -- scope it to the thread.
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0.** The HTTP body is `wire::frame`'s bytes; `Op::Shutdown` is refused over HTTP; `/op` binds per REQUEST where the socket binds per CONNECTION.
- (2026-08-30) **51737 is a preference, never a promise** -- ask for it, fall back to a kernel port, publish what was bound. D6 intact.
