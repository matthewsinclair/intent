---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 07:23Z
status: paused
focus: "FOLDED 2026-08-31 07:23Z, pre-fold at .history/20260831/wip-fold-0721Z.md. Four landed: 49d8b24e AT-06.6, 4a3ca7a9 SERVED_BY_DAEMON membership, ccbb4afd AT-10.8, a3b8aa60 the stale-sibling-intentd refusal. Nothing of mine is uncommitted and nothing is owed to me. ON THE BOUNCE: AT-10.12 migrator_determinism.rs, ISOLATED FIXTURE ONLY. Waiting on vc for ONE thing -- which output carries AC-10.8's residue naming."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** Tree is clean of me.

## TODO

- **AT-10.12 `migrator_determinism.rs`** -- migrate a fixture, migrate again, require identical canon bytes, **and separately** require the output to match canon from the same binary. **The two are different assertions and the second is the one that failed.** **ISOLATED FIXTURE, NEVER THE LIVE TREE**: this is the verb that zeroed the estate's event log, and I have stated I will not run it here again for any reason, including verifying my own fix. Positive control is the measured divergence itself.
- **AC-10.8's naming half -- BLOCKED ON vc, and it is a specification call not a build one.** `sync --to-disk` prints `ok: extract written for {n} thread(s)` and names no residue. I put three readings to vc and recommended (c) -- `doctor`/`export` rather than the egest -- holding it loosely. **Do not build to the literal wording without the ruling**: rewording a criterion to match what is easy to build is the move to avoid, and so is printing a MODEL constant on every run of a routine verb.
- **AT-10.5 `fleet_corpus_ingest.rs`** -- the `migrate.rs` blocker expired 2026-08-27, **but vc's second and load-bearing argument stands: AC-10.5 asks the MIGRATION to name its residue and it does not.** Same shape as AC-10.8's second half; both move together or neither does.
- **AT-06.11 `remedies_are_reachable.rs`** -- walk the emitted REMEDY STRINGS, never the declared verbs; a test asserting every declared verb exists passes trivially. Held red by design once written.
- **AC-06.8 is ic's file, reported not fixed.** `INHERITED_UNREAD` carries four live entries and **three are `st bootstrap` flags on a verb that refuses at rc=2** -- graded as violations because `unwired_families()` keys the deferral on the FAMILY and `st` is wired.
- **A `bin/devbin build all` is owed** before anyone can browse the web face from the delivered binary.

## Watch-outs

**A ROW'S STATED BLOCKER IS A FALSIFIER TO DRIVE, NOT A NOTE TO READ. FOUR OF MINE HAD EXPIRED UNANNOUNCED.** AT-06.8 held red _until the arm runs_ (ic un-`#[ignore]`d it 2026-08-27); AT-10.5 held _until `migrate.rs` exists_ (it does); AC-06.6 awaiting a mechanism already built; and my own fold sent me at WP-13, whose nine criteria were descoped to ST0069. **Re-drive the register at pickup before believing your own handover.**

**AND THE DANGEROUS SHAPE IS A STALE REASON ON A CORRECT VERDICT.** AT-06.8's red is right and its stated reason is dead, so **a reader honouring the note's own expiry would move the row to green.** The verdict gets re-checked; the reason does not.

**A GUARD'S AUTHORITY IS ITS MEMBERSHIP RULE, NEVER ITS NAME.** `export_round_trip.rs`'s anti-vacuity arm -- the one whose whole job is refusing to assert over an empty set -- counted TWO kinds of a THREE-kind type under the name `..._carries_both_kinds_...`, blind to a variant hv had ruled in months-of-commits earlier. **Its name stated its membership rule and the rule went stale.** `export.rs` reasons about that exact hazard for that exact type in its own comment; the test never inherited it. **Where a population must not silently shrink, use an exhaustive `match` so a new variant fails to COMPILE**, never `matches!`.

**A STALE SIBLING BINARY REPORTS ITSELF AS A DEFECT IN SOMEONE ELSE'S KEY.** ic hit a red backup arm on clean HEAD; the daemon predated the gate. **Measured in the binaries, never the source** -- `strings | grep -c 'backup.enabled = false'` gives 1 gated, 0 stale. `render.rs:5620` resolves the daemon as `current_exe().parent().join("intentd")`, and `cargo test -p intent-cli` builds this package's binaries and not another's. **Both prior protections missed in opposite directions**: the panic covers ABSENT, the version check covers CROSS-version, and both binaries say `3.0.0`. Closed by a refusal in `RealDaemon::start` (`a3b8aa60`) -- **and a refusal, never a rebuild**, because a harness that quietly builds hides the class from the node whose binary was stale.

**WRITING A LIMIT DOWN DOES NOT CLOSE IT.** That sibling hazard is stated verbatim in `the_binary_under_test_is_the_one_cargo_built.rs`'s header, written 2026-08-30, and **it fired within a day on a node who had never read it.** A recorded limit is a note to the author, not a guard on the estate.

**A CHECK THAT SUPPLIES ITS OWN DENOMINATOR CERTIFIES ITSELF.** ic's `--out-of-model` attack: the migrator zeroes a counter by naming everything. **Where the denominator is PROSE and cannot be parsed, pin the mapping to its SOURCE PHRASE** -- `egest_estate.rs` plants the pair `data-model.md` names and reds if the document stops naming it. Neither a parse nor a hardcode.

**AND POSITIVE-CONTROL THE GREP BEFORE CALLING A CITATION WRONG.** `out-of-model` appears ZERO times in the file AC-10.8 cites it from; the set is there under another heading. I nearly filed that as a defect. **A citation is not wrong because your search term is** -- test the instrument against a term that must be present.

**COMMITTING IN A SHARED CHECKOUT IS THREE PROBLEMS AND ONLY ONE IS CLOSABLE BY THE COMMITTER.**

- **CONTENTION** -- taking a peer's bytes. Closed by a HEAD-pinned private index.
- **COHERENCE** -- **NOT closeable by you**: a half-landed pair is incoherent however carefully you scope. Land the pair, or wait.
- **REVERSION** -- the nastiest, because it leaves a correct-looking artefact. **Announce any disk->store sync before running it.**

**A RULE THAT NAMES _WHEN_ AND NOT _IN WHICH ENVIRONMENT_ IS SATISFIABLE BY THE BROKEN FORM** (dc's generalisation of my `GIT_INDEX_FILE` slip, 2026-08-31, and it is better than the instance). Mine said _reset in the same turn as the commit_ -- and the same turn is exactly where the variable is still exported, so the rule as written endorsed the defect. **dc's own note was worse and they said so: it named a pre-commit check and nothing about teardown, so it had no clause that could even be read wrong.** Before writing a procedural rule, ask which environment it runs in.

**THE PINNED PRIVATE INDEX, AND THREE WAYS I GOT IT WRONG TODAY.**

- **Pin on BOTH sides.** Git resolves the parent AFTER the hooks, so the gate's whole runtime sits between the pre-check and the commit object.
- **`unset GIT_INDEX_FILE` BEFORE the reset, never after.** I reset with it still exported, so the reset hit the PRIVATE index and **the ambient index was left holding 30 deletions of the commit I had just landed** -- a staged reversion of my own work that any node's plain `commit` would have taken.
- **Gate the post-verify on the commit's EXIT STATUS.** On a refusal HEAD never moved, so `HEAD^` is the base's parent and the check screams DANGER at a gate working correctly. A check that cannot tell _nothing happened_ from _something bad happened_ trains you to ignore it.

**`cd` PERSISTS BETWEEN TOOL CALLS AND IT COST ME THREE REFUSALS.** A `cargo` call leaves the shell in `native/rust`; the next `git add -- native/rust/...` then resolves to `native/rust/native/rust/...` and matches nothing. **Start every git turn with an absolute `cd`.**

**`... | head; echo "rc=$?"` REPORTS `head`'s STATUS.** I printed `rc=0` under a command that exits 2. Use `out=$(cmd 2>&1); rc=$?`.

**A MEASUREMENT THROUGH PATH HAS A SHELF LIFE.** I read `upgrade` and `export` through `~/.local/bin/intent`, got clean output, and the file did not exist minutes later -- a release build cleans and rebuilds in place. **Build your own and drive that.**

**A TRUE CLAIM CAN HAVE A SHELF LIFE SHORTER THAN THE MESSAGE CARRYING IT.** Re-measure on receipt -- and **resetting another node's index entries is the same offence as taking their bytes.**

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`.** Now enforced rather than remembered, but the fact is unchanged.

**A UNIX SOCKET PATH HAS A LENGTH LIMIT AND THE SESSION SCRATCHPAD EXCEEDS IT** (`SUN_LEN`, 143 bytes). `RealDaemon` uses `short_dir`.

**NEVER START `intentd` UNDER THE REAL `$HOME` WHILE PEERS ARE LIVE** -- it takes the store exclusively and refuses every peer's store verbs at once.

**rustfmt NEEDS `--edition 2024` HERE**, and **it reformats what you just wrote** -- a patch anchor built from your own source text stops matching after it runs. Format, then re-read, then patch.

**EXHAUSTIVENESS MAKES THE COMPILER FORCE YOU TO HANDLE A VARIANT, NEVER TO HANDLE IT CORRECTLY.** Drive the effect, not the verdict.

**DO NOT `assert_eq!` ON FILE BYTES.** Mine dumped 757KB of sqlite on its first red. Collect the differing PATHS and name those.

## Decisions

- (2026-08-31) **`RealDaemon` refuses a stale sibling daemon rather than rebuilding one.** Source mtime is the comparison, deliberately not the sibling `intent`: cargo does not relink an unchanged binary, so both would be old together and the check would pass on exactly the tree it exists to refuse.
- (2026-08-31) **AC-10.8's naming half is vc's specification call, not my build.** Recorded rather than resolved by inventing surface.
- (2026-08-30) **`SERVED_BY_DAEMON` is complete at one entry**: a path belongs when its answer is an existing project-scoped, request-response `Op`. It grows when `Op` gains such a variant, not before.
- (2026-08-30) **`backup.enabled` gates the daemon sweep and NOTHING else** -- `cycle` ungated so `intent backup` still works, doctor ungated so staleness is still reported.
- (2026-08-30) **A new guard gets a file named for its contract**, never an arm inside one whose name describes something else.
- (2026-08-30) **Attachments are AUTHORED; no sync direction rewrites them.**
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0.** `Op::Shutdown` is refused over HTTP.
- (2026-08-30) **51737 is a preference, never a promise** -- publish what was bound. D6 intact.
