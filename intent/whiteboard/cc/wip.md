---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 09:30Z
status: active
focus: "AT-06.11 GREEN at 93eed26e (vc drove it), ST0056 102/135. AC-10.5's LIVE RESIDUE IS 2, NOT 114: the migration emits 168 dispositions and `conservation_check.sh --dispositions` consumes exactly that output -- nobody had joined them. The row's load-bearing sentence (`the migration does not name it`) is FALSE at HEAD. Surviving 2 are one defect: issue 0059 has TWO `## Related` sections and canon holds one. ONE CORPUS ONLY -- not extrapolated to the fleet."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** Tree is clean of me; ambient index verified clean on every path I landed.

## TODO

- **AT-10.5 IMMEDIATELY AFTER AT-10.8 -- NOT PULLED FORWARD, AND NOT A QUEUE PREFERENCE.** vc's reason: **the two rows are ONE design question asked of two verbs.** AC-10.5 asks the migration to name its residue; AC-10.8 asks the egest to name what it cannot reproduce. **Ruling (d) applies to both, and doing them adjacently is one ruling applied twice rather than two rulings that drift.**
- **`intent daemon status --format` IS DECLARED, PROJECTED NOWHERE, AND REFUSES NOTHING** (vc, 2026-08-31 07:54Z; my WP-08 surface). `--format zzz` is ACCEPTED at rc=0 and byte-identical to `json` and `terminal`; `render.rs:5555`'s `daemon_status()` takes no format argument at all. **Two defects and only one is a choice.** D2 is shipped, not scope: the table's own `disposition_basis` states MY rule -- an unknown value refuses in the renderer at exit 1, never exit 2 -- and this is the one verb of eleven that never reaches it. D1 vc RULED: narrow to `terminal` now, on the `07ad9876` precedent where five verbs were narrowed under my own correction ending _widen it again when a projection is built_. **AND THE INSTRUMENT COVERING IT IS GREEN**: `format_roster_is_honoured.rs` declares `daemon status` rather than skipping it, named its own expiry (_becomes a real drive the moment cc wires the arm_), and the wiring landed at `e6aba646` -- so the arm went vacuous-because-UNWIRED to vacuous-because-UNREAD with the same green and no signal, because its pass condition is _matches neither refusal pattern_ and a wired verb ignoring the flag matches neither either.
- **AT-10.5 -- THE ROW'S LOAD-BEARING SENTENCE IS FALSE AT HEAD, AND THE RESIDUE IS 2.** The row says _AC-10.5 asks the MIGRATION to name its residue, and the migration does not name it._ **It does**: `intent upgrade` emits 168 dispositions (115 declared drops + 53 deferrals), and `conservation_check.sh` has a `--dispositions` flag that consumes exactly that stdout. **Nobody had ever joined them.** Without it: ALTERED 114, without-a-destination 3, 658 findings. With it: **ALTERED 2, without-a-destination 0, 543 findings, DECLARED-DROP 115 matched of 115.** The 114 were one thing -- every one carried canon hash `e3b0c442...`, the SHA-256 of the EMPTY STRING, over Acceptance 52 / Dependencies 20 / Deliverables 20 / Context-for-LLM 20: template scaffolding the migrator drops on purpose with a per-section reason. **AND IT IS NOT THE DENOMINATOR ATTACK, because ic's tool does not take the migrator's word**: each declared drop is `named per-section AND VERIFIED EMPTY IN CANON`, so a migrator naming everything would FAIL the verification rather than zero the counter. **THE SURVIVING 2 ARE ONE REAL DEFECT**: `issue CLOSED/0059 'Related'`, two census rows with different estate hashes and one non-empty canon value -- the file carries **two `## Related` sections** with different authored bodies and canon holds one, so the other is gone. That is the `DOUBLED-SECTION` shape arriving on the INGEST side. **ONE CORPUS: the canary is one of the four members AC-10.5 names, and I have not run Lamplight/Utilz/Baize. Not extrapolated. Row is vc's.**
- **THE PREDICATE FIX FOR AT-06.11 IS SEQUENCED (vc, ruled): BEFORE OR WITH hv's `--help` NARROWING, NEVER AFTER.** Narrowing first makes _for the verbs that are_ point at an empty list -- ic's original zero-verbs defect by another road, strictly worse than today. Predicate-first is safe alone. **And the spec question I held open was not one**: `unwired()`'s else-arm already reads _nothing in this build provides it -- `intent --help` lists what does_, which is exactly right for config/ext, so a correct predicate routes them into an existing branch. **I mistook a consequence of the bug for a design gap the bug had created.** Wiredness must be DERIVED or declared by a test that DRIVES every declared-unwired path -- **never a hand-maintained flag**, on vc's `exposed_on_mcp` evidence from today.
- **AC-06.8 is ic's file, reported not fixed.** `INHERITED_UNREAD` carries four live entries and **three are `st bootstrap` flags on a verb that refuses at rc=2** -- graded as violations because `unwired_families()` keys the deferral on the FAMILY and `st` is wired.
- **A `bin/devbin build all` is owed** before anyone can browse the web face from the delivered binary.

## Watch-outs

**A CONTROL DRAWN FROM THE SAME ENUMERATION AS THE INSTRUMENT CAN ONLY CONFIRM THE SHAPES THAT ENUMERATION ALREADY HAS** (`flag_reachability.rs`'s hardest-won line, built to rather than admired). In `remedies_are_reachable.rs` the pinned instance is one the ESTATE asserts elsewhere -- `unmigrated_project.rs` requires the unmigrated refusal to name `intent upgrade` -- and **it red on its first run, on me.** My extractor stripped backticks then ate lowercase words, reading `upgrade to migrate this project to` out of English prose: matches no declared path, silently dropped, **so the remedy reads as carrying no command and the property arm passes by having nothing to check.** A hole exactly the size of every prose-carrying remedy.

**AND A NARROWING IS ONLY SOUND WHILE ITS PRECONDITION IS TRUE, SO MEASURE THE PRECONDITION.** The fix is backticked spans only -- which is blind to a command named outside backticks. `every_command_reference_is_backticked` walks the same corpus and refuses any remedy naming `intent ` outside a span, so the extractor's population and the real one are the same set **by measurement rather than by convention.**

**A PREDICATE CAN ANSWER THE QUESTION NEXT TO THE ONE THE SENTENCE ASKS, AND THE TWO AGREE UNTIL THEY DO NOT.** `unwired()` selects _run `intent <x> --help` for the verbs that ARE_ on `is_shipped()`, which is `disposition != "retire"` -- DECLARED. Fine for as long as declared implied built. **ic fixed the zero-DECLARED-verbs case in August; the zero-WIRED case is the same defect, same symptom, same harm, and the predicate ic fixed it with cannot see it.**

**A BAD NEEDLE THAT AGREES WITH THE NUMBER YOU ARE CHECKING IS THE MOST DANGEROUS KIND.** Probing whether a bucketed file's content reached canon, I first grabbed a `verblock:` line, got NOT IN CANON, and **would have filed it as confirmation the file was stranded** -- agreeing with the tool. It is frontmatter: the migrator parses it into fields rather than carrying it as prose, so its absence proves nothing at all. The body-line probe reversed the answer completely. **Fourth instrument slip of the day, and the fourth caught by a control rather than by care.**

**A FLAG THAT EXISTS AND IS NEVER PASSED IS A JOIN NOBODY HAS MADE, AND IT READS AS A FINDING ABOUT THE ESTATE.** `conservation_check.sh --dispositions` consumes `intent upgrade`'s own stdout, the migrator has been emitting 168 dispositions per run all along, and every measurement of AC-10.5's residue had been taken WITHOUT it -- so 114 template-scaffolding drops the migration had named per-section were counted as loss. **The row then recorded, as its blocker, that the migration does not name its residue.** Look for the unexercised flag before believing a figure that indicts someone.

**AND THE JOIN IS ONLY LEGITIMATE BECAUSE THE TOOL DOES NOT TAKE THE MIGRATOR'S WORD**: `named per-section AND VERIFIED EMPTY IN CANON`. A migrator that named everything would fail the verification rather than zero the counter -- ic's design, closing the denominator attack at the point where it would actually be attempted, which is the same property AC-10.8 needed from the other end.

**A COUNTER THAT CANNOT FIRE READS EXACTLY LIKE A COUNTER THAT FOUND NOTHING, AND THE PARTITION IS WHERE THE DIFFERENCE SHOWS.** `DOUBLED 0` was not a wrong count; the predicate asked a tree that holds one of the things it was counting. **I could not have found that from the 386 alone** -- I found it because 386's partner was impossible, and ic found the cause because I said so. The number I came for was reproducible, deterministic and agreed by two nodes; **none of that was ever evidence about the predicate.**

**A DRIVEN NUMBER IS NOT A TRUSTED NUMBER UNTIL ITS COMPANION MAKES SENSE.** `STRANDED 386` is reproducible, deterministic and agreed by two nodes -- and its sibling `DOUBLED 0` is impossible for an estate that migrated 54 bucketed threads without emptying the buckets. **The reproducibility was never the question.** Check the partition, not just the figure you came for.

**A DELEGATED REMEDY PUTS NO LITERAL WHERE A GREP FOR `remedy:` CAN SEE IT.** My harvest found 24 command references, all valid -- and could not have found the one the estate already has a test for, because `FacadeError::Unmigrated(pending) => pending.remedy()` produces the string one call away. **THE CONTROL IS WHAT CAUGHT IT: I went looking for a reference I KNEW existed and the instrument did not have it.** A corpus that cannot contain the known instance cannot support a claim about the population.

**AND `${=c}` IS THE ZSH SPELLING; PLAIN `$c` DOES NOT SPLIT** -- third firing today, and this one manufactured a 15-row table of `reaches the verb` verdicts in which **not one multi-word verb was ever invoked**: `lang list` went to clap as a single argument and came back `unrecognized subcommand`. Paired with an `init --name P` that silently created no project, so the whole probe also ran outside a project. **Two independent breakages, one plausible-looking table, no error anywhere.** The tell was uniformity -- every row rc=1 -- and uniformity across a set that should differ is the thing to distrust.

**A GUARD WHOSE SCOPE WAS NEVER STATED IS NOT A STALE GUARD, AND THE REMEDY IS DIFFERENT.** `upgrade_command.rs`'s `running_it_twice_leaves_the_tree_byte_identical` is green, correct, and its fixture holds NO issues -- while AC-10.12's measured defect is 40 issue bodies rewritten. **It would have passed through the whole episode.** Nothing had gone stale; nobody had written down where its reach stopped, so a green there read as evidence about somewhere else. **Driven rather than argued: dropping the issues from my own fixture leaves the property arm passing**, so the blind spot is demonstrated inside the file that closes it.

**A GUARD'S POPULATION IS THE WORKTREE AND EVERY ISOLATION DEVICE I OWN IS ABOUT THE INDEX** (dc, 2026-08-31, and it is the FOURTH member of the shared-checkout class rather than an instance of the three). Contention, coherence and reversion are all about WHAT ENTERS A COMMIT. This is about WHAT A GUARD CAN SEE: an untracked file nobody staged red-lined the gate for every node in the tree, and neither a private `GIT_INDEX_FILE` nor `--only` touches it. **The corollary is the nasty half: the AUTHOR is the one node whose gate may not fire on it**, because they may not be committing yet, so it runs for a while before anyone reports it. dc's mirror framing is the keeper -- a delete can hollow a survivor, a create can ARM a guard against one.

**rc=1 FROM CLAP IS NOT EVIDENCE OF A WIRED VERB** (dc, 2026-08-31). Bare `config` is rc=2 unwired, but `config get` with no argument is rc=1 _required arguments were not provided: <KEY>_, which reads exactly like a wired subverb demanding input. **Argument validation runs BEFORE the unwired dispatch**, so the real refusal is only reachable by supplying the arguments. Same family as `--help` rendering for a declared command: part of the declaration, not of the implementation.

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

**A PRIVATE-INDEX COMMIT THAT ADDS A NEW FILE LEAVES THE AMBIENT INDEX WITH NO ENTRY FOR IT, AND THE ONLY WITNESS IS A CLONE NOBODY HAS MADE** (caught by vc AND ic independently, 2026-08-31; mine, and not deliberate). `read-tree HEAD` into a private `GIT_INDEX_FILE`, `add` there, commit -- HEAD gains the file, **the ambient index is never told**, and against the new HEAD it reads as a STAGED DELETION. **MODIFIED FILES ARE IMMUNE, WHICH IS WHY IT TOOK THIS LONG TO SURFACE**: measured on all five paths I touched today, the four modified each held an ambient entry and the one I CREATED held none. **Creation is the entire population.**

**AND THE CONSEQUENCE IS WHY IT OUTRANKS AN ORDINARY DIRTY FILE** (vc's wording, kept): the next commit that is not path-scoped **deletes the file FROM THE REPOSITORY while leaving it on every local disk.** `absent_at_check` finds it, cargo builds it, the citation resolves -- **and the estate has no instrument that would see it, because every instrument reads a worktree that still has the file.**

**IT IS THE HOOKS TRAP'S CLASS FROM A DIFFERENT MECHANISM.** `restart.md` opens with it: hooks are tracked and `core.hooksPath` is repo-local config a clone does not inherit, so a fresh clone has every hook body and runs none, and _nothing triggers this automatically and nothing can_. **Local tree fine, clone broken, nothing local able to tell.** Two instances, two mechanisms, one property; handed to vc as a property rather than as a row.

**PROCEDURE, NOT MEMORY: after any private-index commit that CREATES a file, `git restore --staged` the created paths.** Idempotent, syncs the entry from HEAD, loses nothing because disk already equals HEAD.

**AND THE RULE-WRITING FAILURE UNDERNEATH IT IS THE PART TO KEEP.** This board already said _`unset GIT_INDEX_FILE` BEFORE the reset_, written the day the ambient index was left holding 30 deletions of a commit I had just landed. **I wrote that rule about a `git reset` and the class has nothing to do with resets** -- no reset was involved here. **My rule named an ACT where the class is a MECHANISM**, which is dc's _a rule that names WHEN and not IN WHICH ENVIRONMENT is satisfiable by the broken form_ -- arriving again, in a rule I wrote AFTER recording that generalisation two entries below.

**TWICE IN ONE DAY A DECLARATION OF MINE PROVED NARROWER THAN ITS CLASS.** The other was `SERVED_BY_DAEMON`'s membership rule, which said _project-scoped, request-response `Op`_ full stop -- true of every member because every `Op` had an in-process twin when I wrote it, and **wrong the moment ic added one that did not.** A declaration true of every current member is not the same as a correct one, and nothing separates them until someone adds a member.

**A REPO LOCK IS A RETRY, NOT A REJECTION** -- `git restore --staged` died on `index.lock: File exists` with a peer mid-commit, and succeeded on the next attempt. Same family as the `cannot lock ref 'HEAD'` refusal.

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
