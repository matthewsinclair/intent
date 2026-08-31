---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 22:28Z
status: active
focus: "AT-06.6 (49d8b24e), SERVED_BY_DAEMON membership (4a3ca7a9), AT-10.8 (ccbb4afd) landed. ics RED backup test was a STALE SIBLING intentd, not my key -- RealDaemon now refuses one. AC-10.8s naming half stays unbuilt pending vcs design call. Next: AT-10.12 migrator_determinism.rs, ISOLATED FIXTURE ONLY."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** AT-10.8 is built and landed. **AC-10.8 stays RED on its second half and that is the honest state**: the criterion wants the out-of-model set named in the output, `sync --to-disk` prints `ok: extract written for {n} thread(s)`, and the arm for it is `#[ignore]`d with its expiry rather than relaxed. **Waiting on vc for WHICH output carries the naming** -- I recommended `doctor`/`export` over the egest, held loosely, because the set is a property of the MODEL and a constant restated every run is where suppression starts.

## TODO

- **AT-10.12 `migrator_determinism.rs`** -- migrate a fixture, migrate again, require identical canon bytes, AND separately require the output to match canon from the same binary. **The two are different assertions and the second is the one that failed.** **ISOLATED FIXTURE, NEVER THE LIVE TREE** -- this is the verb that zeroed the estate's event log and I have stated I will not run it here again for any reason, including verifying my own fix.
- **AT-10.5 `fleet_corpus_ingest.rs`** -- its named blocker (`migrate.rs` absent) EXPIRED 2026-08-27. **But vc's second argument still stands and is the load-bearing one: AC-10.5 asks the MIGRATION to name its residue and the migration does not.** Same shape as AC-10.8's second half. Asked vc whether to pull it forward.
- **AT-06.11 `remedies_are_reachable.rs`** -- walk the emitted REMEDY STRINGS, never the declared verbs. Held red by design once written.
- **AC-06.8** -- reported to vc, not mine to fix: `INHERITED_UNREAD` has four live entries, three of them `st bootstrap` flags on a verb that refuses at rc=2. ic owns the file.
- **A `bin/devbin build all` is owed** before anyone can browse the web face from the delivered binary.

## Watch-outs

**A STALE SIBLING BINARY REPORTS ITSELF AS A DEFECT IN SOMEONE ELSE'S KEY, AND IT DID.** ic hit `a_project_with_backups_turned_off_...` RED on a clean HEAD and it was neither their code nor my key: the daemon was built before `42402762`. **Measured in the binaries, never the source** -- `strings | grep -c 'backup.enabled = false'` gives 1 for a gated daemon and 0 for `target/debug/intentd`. `render.rs:5620` resolves the daemon as `current_exe().parent().join("intentd")`, and **`cargo test -p intent-cli` builds this package's binaries and not another package's**, so the sibling is whatever an earlier build left.

**BOTH EXISTING PROTECTIONS MISS IT BY DESIGN, IN OPPOSITE DIRECTIONS.** The `wait_until_it_answers` panic covers ABSENT; the version check covers a CROSS-version mismatch and both binaries say `3.0.0`. **Same version, different build, nothing to compare.** `RealDaemon::start` now refuses a sibling older than the newest `intentd`/`intentsvcs` source, and REFUSES rather than rebuilding -- a harness that quietly builds hides the class from the node whose binary was stale.

**AND IT IS THE RESIDUAL LIMIT I WROTE INTO `the_binary_under_test_is_the_one_cargo_built.rs` YESTERDAY, FIRED WITHIN A DAY.** That header says the guard proves the SPELLING and not that a spawn reached the intended inode. **Writing a limit down does not close it**, and the person who hit it was not the person who recorded it.

**MY BOARD'S NEXT-ITEM WAS STALE IN THE DIRECTION THAT COSTS A SESSION.** It sent me to ask hv about WP-13; WP-13 was descoped to ST0069 and there was nothing to ask. **A fold records what you were thinking, and what you were thinking is the first thing to go out of date.** Re-drive the register at pickup before believing your own handover.

**A GUARD'S AUTHORITY IS ITS MEMBERSHIP RULE, NEVER ITS NAME. SIXTH INSTANCE 2026-08-30 21:45Z:** `flag_reachability`'s `unwired_families()` keys its deferral on the FAMILY -- the first token of the path -- so `st bootstrap`, which refuses at rc=2 as unimplemented, has its three flags graded as if wired and grandfathered as debt. **Three of the four entries in `INHERITED_UNREAD` are not debt at all.**

**AND THE FIVE FROM 2026-08-30 STAND:** vc's `populations.self_loop`; my arm 6b's hardcoded `case`; dc reading `table_driven_tests_fixture_their_home` as the guard over binary resolution; dc's `facade.rs` kind rule; vc catching my arm undercount. **In three of the five the CONCLUSION WAS STILL CORRECT**, which is why they survived. **My own was worst-shaped: I read every arm of the file I was attacking and truncated my grep on the file I was defending.**

**A BLOCKER'S EXPIRY IS SILENT AND NOTHING ANNOUNCES IT. THREE TODAY, ALL MINE.** AT-06.8 held red _until the arm runs_ (ic un-`#[ignore]`d it 2026-08-27); AT-10.5 held to-write _until `migrate.rs` exists_ (it does, 2026-08-27); AC-06.6 awaiting a mechanism that is fully built. **A row states its own falsifier and then nobody drives it.** Drive the falsifier, do not read the note.

**AND THE DANGEROUS HALF: AT-06.8's NOTE WOULD NOW MOVE THE ROW TO GREEN IF HONOURED.** Its stated expiry has been met and the row is still correctly red for a reason the note does not give. **A stale reason attached to a correct verdict is worse than a stale verdict**, because the verdict gets re-checked and the reason does not.

**A DECLARED, HELPED, UNREAD FLAG IS HARMLESS ONLY WHILE ITS VERB REFUSES.** `st bootstrap --dry-run` and `--audit-only` promise no-writes and are read by nothing. rc=2 is the only thing making that safe. **Wiring the verb without wiring the flags is what converts it into the defect**, and that is a one-commit distance.

**THE BINARY ON PATH IS NOT THE BINARY YOU BUILT, AND IT CAN VANISH MID-MEASUREMENT.** I read `upgrade` and `export` through `~/.local/bin/intent`, got clean output, and **the file does not exist now** -- a release build cleans and rebuilds in place, dangling the symlink for ~66s. **A measurement through PATH has a shelf life.** Build your own and drive that.

**`... | head; echo "rc=$?"` REPORTS `head`'s STATUS.** I printed `rc=0` under a command that exits 2. Capture with `out=$(cmd 2>&1); rc=$?` or `${PIPESTATUS[0]}`. **The failure returns a plausible number rather than an error.**

**COMMITTING IN A SHARED CHECKOUT IS THREE PROBLEMS AND ONLY ONE IS CLOSABLE BY THE COMMITTER.**

- **CONTENTION** -- taking a peer's bytes. Closed by a HEAD-pinned private index (below), or `--only` where whole files suffice.
- **COHERENCE** -- the tree you commit must make sense. **NOT closeable by you**: a half-landed pair is incoherent however carefully you scope, and on 2026-08-30 one blocked EVERY node in the estate for ~20 minutes. The remedy is the guard's own: land the pair, or wait.
- **REVERSION** -- vc's find. `at.set` then `sync_from_disk` 1.2s later took their green AND a path correction, **and the row looked untouched afterwards.** D01 makes the store SSOT and `sync_from_disk` inverts it for one call. **Announce any disk->store sync before running it.**

**THE PINNED PRIVATE INDEX, AND IT IS ONLY SAFE PINNED ON BOTH SIDES.** `base=$(git rev-parse HEAD)`; `read-tree $base`; stage; refuse if HEAD moved; commit; **then assert `HEAD^ == base`.**

- **Why both sides:** git resolves the parent AFTER the hooks, so **the pre-commit gate's entire runtime sits between the pre-check and the commit object.** ic nearly lost two of vc's commits to the unpinned form.
- **Why pin at all:** `--only` and plain `commit` lose LOUDLY on `cannot lock ref 'HEAD'`. **A private index removes the contention and therefore the noise.** The post-verify buys the noise back.
- **Its other cost:** the commit moves HEAD while the AMBIENT index keeps pre-commit entries, so **the ambient index becomes a silent reversion of your own commit.** `git reset -q HEAD -- <paths>` in the SAME turn as the commit.

**A TRUE CLAIM CAN HAVE A SHELF LIFE SHORTER THAN THE MESSAGE CARRYING IT.** dc warned me five of my paths were staged as pending reversions; true when sent, false when it arrived. Re-measure on receipt -- and **resetting another node's index entries is the same offence as taking their bytes.**

**AN UNTRACKED FILE CAN CHANGE WHAT A SHARED GUARD SAYS ABOUT EVERY NODE, WITH NO SIGNAL TO ITS AUTHOR.** The guard runs at commit; if your last commit predates the file, the refusal lands on whoever commits next.

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`.** A control that cannot fail certifies a test that cannot fail. `cargo build -p intentd` first.

**A UNIX SOCKET PATH HAS A LENGTH LIMIT AND THE SESSION SCRATCHPAD EXCEEDS IT** (`SUN_LEN`, 143 bytes). `RealDaemon` uses `short_dir`.

**NEVER START `intentd` UNDER THE REAL `$HOME` WHILE PEERS ARE LIVE** -- it takes the store exclusively and refuses every peer's store verbs at once.

**rustfmt NEEDS `--edition 2024` HERE.** `--edition 2021` fails on let-chains with an error that reads like a code defect and formats nothing. Format BEFORE staging.

**EXHAUSTIVENESS MAKES THE COMPILER FORCE YOU TO HANDLE A VARIANT, NEVER TO HANDLE IT CORRECTLY.** Drive the effect, not the verdict.

## Decisions

- (2026-08-30 21:45Z) **A row's stated blocker is a falsifier to DRIVE, not a note to read.** Three of mine had expired unannounced. Re-drive at pickup.
- (2026-08-30) **`backup.enabled` gates the daemon sweep and NOTHING else** -- `cycle` ungated so `intent backup` still works, doctor ungated so staleness is still reported. vc's homonym ruling, quoted at the field itself.
- (2026-08-30) **`Due::Disabled` is checked BEFORE `schedule`**, so an inert value is not announced as a defect.
- (2026-08-30) **A new guard gets a file named for its contract**, never an arm inside one whose name describes something else.
- (2026-08-30) **Attachments are AUTHORED; no sync direction rewrites them.** `st attach <ID> <rel> --from <file>` is the narrow disk->store door; `--to-store` is destructive and bare replaces the WHOLE store -- scope it to the thread.
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0.** `Op::Shutdown` is refused over HTTP; `/op` binds per REQUEST where the socket binds per CONNECTION.
- (2026-08-30) **51737 is a preference, never a promise** -- ask for it, fall back to a kernel port, publish what was bound. D6 intact.
