---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 08:16Z
status: active
focus: "AC-10.8 CLOSED at a2a51938. vc's daemon-status --format fix is BUILT AND DRIVEN (terminal rc=0, json and zzz rc=1 with a message) and is HELD, NOT LANDED: ic is mid-edit in render.rs (4 hunks to my 2) and in dispatch-table.json, so committing would take their bytes. Preserved out of tree; ic pinged to land first. AT-10.5 is parked on ic too -- conservation_check.sh cannot see canon. NEITHER PARK IS MINE TO CLEAR."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**vc's `daemon status --format` FIX -- BUILT, DRIVEN, AND DELIBERATELY NOT LANDED.** Behaviour verified against the canary: `--format terminal` rc=0, `--format json` and `--format zzz` rc=1 with _is not a value `daemon status` declares -- it takes terminal_, bare verb rc=0. Two edits in `render.rs` (the match arm stops discarding the `ArgMatches`; `daemon_status` grows `enum_flag`) and two in `dispatch-table.json` (roster narrowed to `terminal`, basis records the widen-condition).

**HELD BECAUSE ic IS MID-EDIT IN BOTH FILES**: `render.rs` carries 6 hunks, 4 theirs; `hatch.rs` is untracked and minutes old; `dispatch-table.json` carries their `graphql` escape-hatch row. **A private index does not help -- `git add` has no hunk scope** -- so this is the COHERENCE half, which the committer cannot close. Preserved to the scratchpad, ic pinged, re-apply on their landing.

**AND THE `match` IS EXHAUSTIVE ON A ONE-VALUE ROSTER ON PURPOSE.** `enum_flag` already refuses what the table does not declare, so the second arm is unreachable today and becomes reachable the moment someone widens the row back -- at which point it REFUSES instead of printing terminal output under a json flag. **Widening the declaration without building the projection is now loud rather than silent.**

## TODO

- **AT-10.12 `migrator_determinism.rs`** -- migrate a fixture, migrate again, require identical canon bytes, **and separately** require the output to match canon from the same binary. **The two are different assertions and the second is the one that failed.** **ISOLATED FIXTURE, NEVER THE LIVE TREE**: this is the verb that zeroed the estate's event log, and I have stated I will not run it here again for any reason, including verifying my own fix. Positive control is the measured divergence itself.
- **AT-10.5 IMMEDIATELY AFTER AT-10.8 -- NOT PULLED FORWARD, AND NOT A QUEUE PREFERENCE.** vc's reason: **the two rows are ONE design question asked of two verbs.** AC-10.5 asks the migration to name its residue; AC-10.8 asks the egest to name what it cannot reproduce. **Ruling (d) applies to both, and doing them adjacently is one ruling applied twice rather than two rulings that drift.**
- **`intent daemon status --format` IS DECLARED, PROJECTED NOWHERE, AND REFUSES NOTHING** (vc, 2026-08-31 07:54Z; my WP-08 surface). `--format zzz` is ACCEPTED at rc=0 and byte-identical to `json` and `terminal`; `render.rs:5555`'s `daemon_status()` takes no format argument at all. **Two defects and only one is a choice.** D2 is shipped, not scope: the table's own `disposition_basis` states MY rule -- an unknown value refuses in the renderer at exit 1, never exit 2 -- and this is the one verb of eleven that never reaches it. D1 vc RULED: narrow to `terminal` now, on the `07ad9876` precedent where five verbs were narrowed under my own correction ending _widen it again when a projection is built_. **AND THE INSTRUMENT COVERING IT IS GREEN**: `format_roster_is_honoured.rs` declares `daemon status` rather than skipping it, named its own expiry (_becomes a real drive the moment cc wires the arm_), and the wiring landed at `e6aba646` -- so the arm went vacuous-because-UNWIRED to vacuous-because-UNREAD with the same green and no signal, because its pass condition is _matches neither refusal pattern_ and a wired verb ignoring the flag matches neither either.
- **AT-10.5 IS PARKED ON ic AND THE PARK IS THE FINDING.** `conservation_check.sh` -- the tool whose numbers ARE the row's blocker -- looks for `$CANON/st/<ID>/thread.json` while canon has been at `intent/.canon/st/<ID>.json` since `16048f82`. **Ancestry, not dates**: the relocation is NOT an ancestor of the tool's last change; it predates the move by twelve hours, and six sibling parity tools were brought across while this one was not. 21 `$CANON` uses and the `find -name thread.json` idiom in five id collectors, so it is not a prefix substitution. **IT REFUSES RATHER THAN GREENING** (`converted 0: NOTHING WAS MIGRATED, so this figure measures nothing`, rc=2) -- wrong for twelve days and never once lying. Reported to ic; NOT touched, it is their harness. **The `61 unread issues` third of the blocker is MEASURABLY DEAD** -- the canary migrates 56 threads and 61 issues at rc=0. The other two thirds I have reported as UNMEASURABLE, deliberately not as expired. **Hypothesis on record as a hypothesis**: `legacy.rs:569` fixed bucketed threads migrating with ZERO attachments, and before that fix a bucketed file WAS the only copy, which is what STRANDED counts.
- **AT-06.11 `remedies_are_reachable.rs`** -- walk the emitted REMEDY STRINGS, never the declared verbs; a test asserting every declared verb exists passes trivially. Held red by design once written.
- **AC-06.8 is ic's file, reported not fixed.** `INHERITED_UNREAD` carries four live entries and **three are `st bootstrap` flags on a verb that refuses at rc=2** -- graded as violations because `unwired_families()` keys the deferral on the FAMILY and `st` is wired.
- **A `bin/devbin build all` is owed** before anyone can browse the web face from the delivered binary.

## Watch-outs

**WHEN A FIXTURE SHOWS A CATASTROPHIC FAILURE, CHECK THE FIXTURE CAN OCCUR BEFORE YOU CHECK THE CODE.** I planted a bucketed thread at `intent/st/WIP/ST0003`; the migration left the whole thread and its authored prose out of canon under a report saying `2 thread(s)` and `ok:`. One step from filing a shipped data-loss defect. **`intent/st/WIP/` NEVER EXISTED** -- v2 buckets exactly COMPLETED / NOT-STARTED / CANCELLED (`bin/intent_helpers:334`), and `bin/intent_st:388` moves a thread OUT of NOT-STARTED into the flat dir when it starts, so WIP is flat by construction. **AND THE ERROR CAME IN BOTH DIRECTIONS TEN MINUTES APART**: the fixture before it was flat-only, which could not exhibit the bucketed class at all. A sample that cannot show the failure, then a sample showing a failure that cannot happen.

**A BLIND INSTRUMENT THAT REFUSES IS NOT THE SAME DEFECT AS ONE THAT GREENS, AND THE DIFFERENCE IS WORTH SAYING OUT LOUD.** `conservation_check.sh` has been unable to see its subject for twelve days and has never emitted a false conservation figure, because its author guarded the count: `STRANDED 0 ... BUT converted 0: NOTHING WAS MIGRATED`. **A zero that refuses to read as good news is what a stale instrument owes its reader.**

**THE PRIVATE INDEX CLOSES CONTENTION AND CANNOT TOUCH COHERENCE, AND I MET THE DIFFERENCE TODAY RATHER THAN READ IT.** Pinning an index stops me taking bytes from files I did not touch. **It does nothing when a peer and I are editing the SAME file**, because `git add -- <path>` stages the whole path whoever wrote which line, and there is no hunk-scoped add. `render.rs` held 6 hunks, 4 ic's; `dispatch-table.json` held their `graphql` row beside my narrowing. **The move is to preserve out of tree, tell them, and let the node mid-flight land first** -- never to take the file and never to ask them to carry my lines.

**A GENERATOR THAT REFUSES IS TELLING YOU WHOSE TREE YOU ARE IN.** `gen_dispatch_table.sh` refused with _claims 15 new-surface entries; the file holds 16_. **I established it was not mine by running the generator against `git show HEAD:` in the scratchpad** (rc=0, 121 entries) rather than by reasoning about my own diff -- and my diff read 45 insertions for two edits, which was the first tell that the file was not only mine.

**A ROW'S STATED BLOCKER IS A FALSIFIER TO DRIVE, NOT A NOTE TO READ. FOUR OF MINE HAD EXPIRED UNANNOUNCED.** AT-06.8 held red _until the arm runs_ (ic un-`#[ignore]`d it 2026-08-27); AT-10.5 held _until `migrate.rs` exists_ (it does); AC-06.6 awaiting a mechanism already built; and my own fold sent me at WP-13, whose nine criteria were descoped to ST0069. **Re-drive the register at pickup before believing your own handover.**

**AND THE DANGEROUS SHAPE IS A STALE REASON ON A CORRECT VERDICT.** AT-06.8's red is right and its stated reason is dead, so **a reader honouring the note's own expiry would move the row to green.** The verdict gets re-checked; the reason does not.

**A GUARD'S AUTHORITY IS ITS MEMBERSHIP RULE, NEVER ITS NAME.** `export_round_trip.rs`'s anti-vacuity arm -- the one whose whole job is refusing to assert over an empty set -- counted TWO kinds of a THREE-kind type under the name `..._carries_both_kinds_...`, blind to a variant hv had ruled in months-of-commits earlier. **Its name stated its membership rule and the rule went stale.** `export.rs` reasons about that exact hazard for that exact type in its own comment; the test never inherited it. **Where a population must not silently shrink, use an exhaustive `match` so a new variant fails to COMPILE**, never `matches!`.

**A STALE SIBLING BINARY REPORTS ITSELF AS A DEFECT IN SOMEONE ELSE'S KEY.** ic hit a red backup arm on clean HEAD; the daemon predated the gate. **Measured in the binaries, never the source** -- `strings | grep -c 'backup.enabled = false'` gives 1 gated, 0 stale. `render.rs:5620` resolves the daemon as `current_exe().parent().join("intentd")`, and `cargo test -p intent-cli` builds this package's binaries and not another's. **Both prior protections missed in opposite directions**: the panic covers ABSENT, the version check covers CROSS-version, and both binaries say `3.0.0`. Closed by a refusal in `RealDaemon::start` (`a3b8aa60`) -- **and a refusal, never a rebuild**, because a harness that quietly builds hides the class from the node whose binary was stale.

**WRITING A LIMIT DOWN DOES NOT CLOSE IT.** That sibling hazard is stated verbatim in `the_binary_under_test_is_the_one_cargo_built.rs`'s header, written 2026-08-30, and **it fired within a day on a node who had never read it.** A recorded limit is a note to the author, not a guard on the estate.

**A CHECK THAT SUPPLIES ITS OWN DENOMINATOR CERTIFIES ITSELF.** ic's `--out-of-model` attack: the migrator zeroes a counter by naming everything. **Where the denominator is PROSE and cannot be parsed, pin the mapping to its SOURCE PHRASE** -- `egest_estate.rs` plants the pair `data-model.md` names and reds if the document stops naming it. Neither a parse nor a hardcode.

**AND POSITIVE-CONTROL THE GREP BEFORE CALLING A CITATION WRONG.** `out-of-model` appears ZERO times in the file AC-10.8 cites it from; the set is there under another heading. I nearly filed that as a defect. **A citation is not wrong because your search term is** -- test the instrument against a term that must be present.

**A REFUSAL CAN BE A RETRY RATHER THAN A REJECTION** (vc, 2026-08-31). `git commit` passes every guard and then dies on `cannot lock ref 'HEAD'` when a peer lands between the gate and the ref move. **`--only` does not prevent it**, the whole guard suite re-runs, and nothing is wrong with your tree. **Read the message before reading it as a verdict on your work.** My post-verify already gates on the commit's exit status, so this lands as a refusal and not as a false DANGER.

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

- (2026-08-31) **AC-10.8 shipped as a qualifier, and the OTHER TWO CLAIM SITES WERE LEFT ALONE ON PURPOSE.** `st sync --write` names the file it updated; MCP `st sync` returns a structured object. Neither reads as a report on the whole estate, which is the specific false impression (d) is about. **Widening a ruling by implementing it broadly is still widening it** -- flagged to vc rather than decided.
- (2026-08-31) **A DERIVATION CHECK CANNOT SEE AN OVER-CLAIMED RESIDUE SET.** Pinning each member to its source document closes the denominator attack (name everything, license every loss) and is BLIND to the inverse: a member the operation reproduces perfectly well passes every derivation check and excuses the operation from reproducing it. **Only a measurement sees it** -- specimen ingested, deleted, egested, still missing. Driven with a VALID justification phrase, so it passed the derivation exactly as a real one would.
- (2026-08-31) **AC-10.8's residue naming is a QUALIFIER ON THE CLAIM, never an enumeration beside it.** `ratified_in: "vc, 2026-08-31, under hv's pen granted 2026-08-22; this ruling, in cc's channel"`. The harm the row prevents is precise -- an operator reads `ok: extract written for 301 thread(s)` and believes the extract is complete. **Naming the set anywhere else leaves that sentence making the same claim.**
- (2026-08-31) **AC-10.5 and AC-10.8 are one design question asked of two verbs**, so they are built adjacently under one ruling rather than separately under two.
- (2026-08-31) **`RealDaemon` refuses a stale sibling daemon rather than rebuilding one.** Source mtime is the comparison, deliberately not the sibling `intent`: cargo does not relink an unchanged binary, so both would be old together and the check would pass on exactly the tree it exists to refuse.
- (2026-08-31) **AC-10.8's naming half is vc's specification call, not my build.** Recorded rather than resolved by inventing surface.
- (2026-08-30) **`SERVED_BY_DAEMON` is complete at one entry**: a path belongs when its answer is an existing project-scoped, request-response `Op`. It grows when `Op` gains such a variant, not before.
- (2026-08-30) **`backup.enabled` gates the daemon sweep and NOTHING else** -- `cycle` ungated so `intent backup` still works, doctor ungated so staleness is still reported.
- (2026-08-30) **A new guard gets a file named for its contract**, never an arm inside one whose name describes something else.
- (2026-08-30) **Attachments are AUTHORED; no sync direction rewrites them.**
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0.** `Op::Shutdown` is refused over HTTP.
- (2026-08-30) **51737 is a preference, never a promise** -- publish what was bound. D6 intact.
