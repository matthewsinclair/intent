---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-18 10:44Z
status: paused
focus: "FOLD 12, PAUSED ON hv'S INSTRUCTION -- hv is rebooting. **ST0057 EXISTS AND CARRIES THE DISK-MODEL DESIGN** (`442ef27d`, `e7c11f14`), created BY hv THROUGH THE v3 CLI one command at a time: **seven commands, four defects, one design reversal, and the 620-leg suite could express none of the four.** **hv RULED EVERYTHING OPEN:** the 165 are ATTACHMENTS and `THREAD_PROSE` is deleted; canon relocates to `intent/.canon/` still per-artefact; `intent edit` not `wip`; **D57-5, NEW -- a full text realisation into `.backup/text/<UTC>/` as a HUMAN fallback, which is a different assurance from the dehydration gate.** **cc is RELEASED and the regeneration window is APPROVED, sequence pinned so it happens once: `Triage->Wip`, `has_end_date()`, `THREAD_PROSE`, THEN regenerate.** **Critic gate BOTH HALVES approved and primed at `intent/st/ST0056/critic-gate.md` (`ce0ac764`) -- unbuilt, to be done on hv's return.** **STRANDED 192 is the only number gating any deletion.** Upstream FROZEN at `5765c5da`; v3 NOT on PATH; push `local` only."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **HOLDING on hv's instruction.** hv approved both critic halves and the regeneration window, then had to reboot. **Everything approved is PRIMED and nothing is built.** On resume, the first question is whether cc has taken their queue.

- **ST0057 -- the disk model, DESIGNED AND RULED.** `intent/st/ST0057/design.md`, six numbered decisions each with a measurement under it. **D57-1** canon to `intent/.canon/`, per-artefact (one consolidated file is a merge-conflict generator with four writers). **D57-2** `.intentfiles`, committed, generated + pinned regions, grammar REFUSES rather than skips. **D57-3** `organize` with four answers plus **UNCLAIMED: report, never remove**; the per-file dehydration gate; the attachment/view authorship asymmetry; idempotence by content; a lock against ANY process. **D57-4** `intent edit`, because `wip` already has four meanings. **D57-5** the full text realisation. **D57-6** the 165 as attachments.

- **D57-5 IS THE HALF THE DESIGN WAS MISSING AND hv SUPPLIED IT.** The dehydration gate proves **THE STORE holds it**; a complete text export proves **A HUMAN can get it back without the tool.** Different assurances; only the first was covered. **It lands on `export --format md`, DELIBERATELY REFUSED today because _"the views are already in the tree"_ -- correct now, false the moment the disk model lands, so RULED withdrawn as PART of that thread and not before.** Precondition cc found: **`intent init` is NOT IMPLEMENTED**, and you cannot demonstrate a fallback from a clean directory you cannot create.

- **THE REGENERATION IS cc's, APPROVED, AND THE SEQUENCE IS THE POINT: `THREAD_PROSE` CHANGES WHAT THE MIGRATION WRITES.** Regenerate before it lands and it happens twice. Asked cc for: one owner, digest before and after, **report exactly what moved rather than that it worked**, `views::info`'s blank line fixed BEFORE the run so the two effects land as one intended change, and a check that **`whitespace-normalised` FALLS and `byte-identical` RISES** -- if not, something is reading the store rather than the source. **ST0011 is MINE and goes AFTER the regeneration**, never before: one row into a store about to be rebuilt is a write into the thing being replaced.

- **THE CRITIC GATE, BOTH HALVES APPROVED, PRIMED, UNBUILT -- `intent/st/ST0056/critic-gate.md`.** **A:** one word, `critic` into `GLOBAL_COMMANDS` at `bin/intent:55`; 8 bats failures -> 0; `st list`/`wp list` still refuse at rc=2 so the guard is not weakened. **B:** repaired, it still only reports Elixir -- **0 of 6 shell rules and 0 of 7 rust rules carry a greppable proxy** on a project that is 114 `.rs` + 57 `.sh` + 71 `bin/` + 108 `.bats`; Elixir 19/19 is the positive control. **Re-arming rust and shell is REAL WORK, unscoped, and must not be quoted as a one-liner.** **The shim DEFEATS A** -- take the fix, hold the shim. Adjacent and NOT yet put to hv: **`release:373-383` never runs `cargo test` at all.**

- **STRANDED 192 IS THE ONLY NUMBER GATING ANY DELETION, and `LOST-PROSE 0` is the trap beside it.** It is TRUE and means only _every section that HAS a destination reached it_. **The two-counter shorthand would have authorised deleting 192 files holding 748 authored sections while printing a zero** -- it was in the SPEC, not just on boards, and is gone. **The gate is the VERDICT at full scope**: exit 0, a printed `0 finding(s)` line whose ABSENCE is a refusal, the denominator, a pinned subject, and a demonstrated red. Full text in `conservation_check.sh`'s header; do not restate it here.

## Rules that keep paying

- **THE DAY'S RULE, in cc's narrowing rather than mine, because mine was uncosted.** Not _"check your premises"_. **_"When someone states a claim as the LOAD-BEARING reason for a decision, THAT claim is the one to check."_** One grep each time. **The social half is the MECHANISM, not a caveat: the person who states the reason is not usually the person who can see it is unchecked** -- which is why the peer structure is the instrument rather than overhead. It read three times on 2026-08-18.
- **A SUBJECT THAT CANNOT EXHIBIT THE DEFECT CANNOT CLEAR IT.** The gitignored store no clone could hold; the attachment ordering a path-sorting migrator makes unobservable. **Both found by something that differed from the subject BY ACCIDENT** -- the argument for fixtures that differ on purpose.
- **A FIXTURE THAT HOLDS THE STATE BUT SKIPS THE TRANSITION lets BOTH ARMS of a contradiction test green while the path between them is broken.** Distinct from a fixture that cannot hold the defect at all. `st cancel` -> `doctor` is the live instance.
- **THE COUNTER-PATTERN, the only thing that has worked by design rather than by luck: an instrument that names its own blind spot in the same breath as its verdict.** `conservation_check.sh:793`, the SCOPE line, and `sync`'s refusal enumerating all 40 issues it would overwrite before declining to.
- **AN AT ROW EARNS ITS GREEN from an instrument DEMONSTRATED RED, a criterion naming the SUBJECT it was handed, and one naming the SHAPE OF THE INPUT** (mine + ic's two sharpenings).
- **ic: a field a test EXECUTES gets a different grade from one a test merely READS.**
- **A prediction being right is not evidence the run was sound** (ic). They applied it to their OWN correct prediction, which is the moment nobody does.
- **A source-text grep measures CLAIMS. Only EXECUTION measures behaviour.** ST0039 already paid for this; a probe whose POSITIVE is generated by the DOCUMENTATION OF ITS OWN NEGATIVE is the sharpest form.
- **A ruling given over the live channel is NOT LANDED until it is in the artefact it governs.**
- **The push result carries NO information about the remote in either direction.** Only `git ls-remote` + `git merge-base --is-ancestor`.

## TODO

- **ST0011** -- `completed` NULL on the estate's one genuinely wrong row. Mine, AFTER cc's regeneration.
- **AC-10.8 (egest symmetry)** and **the WP-10 interruption property** are contract items still not in the AC set. **0 of 114 cover "a second migration over an interrupted estate reaches the same end state as a clean one"**, which hv gated the cutover on.
- **D50** -- asked hv to re-rule; my framing produced the right answer to the wrong question.
- **The board format** -- a board is read as CURRENT STATE and written as a SNAPSHOT OF WHEN ITS AUTHOR TYPED, and nothing marks which. `as at <commit>` in the header block is the shape. Mine.

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Everything else is a secondary artefact. Nothing on disk is truth.**
2. All of `intentsvcs` works FROM the db; sync runs both ways.
3. The typed Rust API is the ONLY door in, so conformance is **by construction**.
4. Re-creation from an extract is a **capability**, not a licence to treat the db as disposable.
5. Ingest passes the **hard gate** of the intentsvcs API; the gate does the work, not the file format.
6. **Migrations are normal.** "No DB migrations, ever" is DELETED and was never hv's constraint.
7. The real standing requirement is **platform and data-model openness** (AC-02.6): always a 1-1 db-entity-to-`.json`/`.md` mapping, lossless, usable elsewhere. **That is what bidirectional sync is FOR.**
8. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine truth and is NEVER committed.** Authority is not bidirectional just because transport is. **So AC-02.6 is the durability mechanism** -- a field that does not round-trip is data loss at the clone boundary, not a gap -- and **`event_log` is the only table that is both durable truth AND not reconstructible from the files**, which makes `events.jsonl` a precondition of the model rather than a WP-04 detail.

## Verification kit

- `$CLAUDE_JOB_DIR/tmp/v3fix` is a **migrated** v3 fixture with its own `git init`. Recipe: `config.json` at 3.0.0, `st new`, prose into `thread.json` (**never** a generated view), `sync`.
- **`$CLAUDE_JOB_DIR/tmp/hookprobe` is the 0043 rig; it covers ALL THREE Claude Code hook events and needs NO migrated project and NO interactive session.** Per arm: a dir, a `settings.json` wiring `UserPromptSubmit` (matcher `""`) to one command, then `claude -p "<prompt>" --settings ./settings.json`. Arms are hook scripts exiting 0/1/2 plus one wired to the real v3 binary. **Assert on the OUTPUT, never the exit code -- a BLOCKED run exits 0**, which is the same silent shape the probe was built to find. **And keep the exit-1 arm**: an assertion that `2` blocks passes equally on a build where every code blocks, so only the arm that does NOT block ties the failure to the change that caused it.
- **MEASURE AT A PINNED SHA, AND NAME THE COMMIT IN THE FINDING.** `HEAD` is a POINTER, so a claim about it is a claim about whatever it points at WHEN READ -- pinning protects the measurement, only naming the commit protects the report. **And `rust` is a pointer at a tree the same way, which is the whole of 0049.** `git archive` the sha, build in a separate `CARGO_TARGET_DIR`. Incidents in `.history/`.
- **ASK HOW MANY WRITERS AN ARTEFACT HAS BEFORE RULING ON IT** (cc). _"A ruling enforced on one of two writers is enforced on neither RELIABLY"_ -- **the uncovered writer wins whenever it runs LAST, so it is a RACE, which passes in the suite and fails in the estate.** `todo.md` has two doors and all five tests reached it through the other one. Incident in `.history/`.
- **Falsify before flipping.** Perturb the artefact the test asserts against and watch the right subset go red.
- **COUNT, STOP, READ THE DIFFERENCE, THEN ARCHIVE -- IN THAT ORDER, IN SEPARATE COMMANDS.** I ran the both-sided count check and the archive in ONE command and archived an unread ic entry; it printed `ic live=5` against my four named stamps AFTER the archive had run. **A check whose result arrives after the act it was meant to authorise is not a check.** Recovered from `.history/` and nothing was lost. The check caught unread entries FIVE times today and failed only when I let it run too late.
- **SIGNATURE AND SCOPE FAIL IN OPPOSITE DIRECTIONS AND NEITHER IMPLIES THE OTHER** (dc, 2026-08-16, correcting my verification of their header guard). Signatures answer _does it fire on the right SHAPE_; scope answers _does it fire on the right FILES_. **I cleared that guard on signatures alone and reported it as verified**: `intent/whiteboard/*/wip.md` matches 21 files, SIXTEEN of them archived boards, so it would have refused the protocol's own housekeeping. Both `case` globs and git pathspecs cross `/`. **The identical trap had already bitten the clock guard's port**, which makes it a class.
- **THE DAY'S MEASUREMENT CLASS AND ITS INSTRUMENT RULES NOW LIVE IN `parity.md`, NOT HERE** -- nine variants, three properties, the calibration and chokepoint rules, the annotated-artefact bias, and the sibling class. Four entries that sat here (guard-proven-only-by-its-positive-case, a-check's-error-message, the-ok-line-must-be-conditional, put-the-population-on-the-ok-line) are folded into it verbatim and archived. **Read `parity.md` -> Measurement rules before building any instrument.** Keeping a second copy here is the divergent-copy defect the canon itself warns about.
- **SHARPENED 2026-08-16, by L2 refusing a row I wrote: the three states answer TWO questions, not one. `to-write` = the test is UNWRITTEN. `red` = the test EXISTS and does not pass. NEITHER means "the criterion is unmet" -- a criterion can be false with no instrument pointed at it, which is exactly AC-10.9.** I set that row `red` because issue 0038 makes its criterion measurably false, and L2 refused it because the cited file does not exist. **The refusal was right.** My partial-coverage rule below was always about a test that EXISTS and under-covers, and it does not generalise to any unmet criterion; reading it that way produced a red row citing a file that is not there. The exempt-state hazard on such a row is covered by `stale_at_check.sh` firing the moment the file appears.
- **Refuse at partial coverage. CORRECTED 2026-08-15: hold the AT at `red` WITH AN EXPLICIT NOTE, not at `to-write`.** I had this the other way round on the grounds that `red` lies about a passing suite. It does not: **the AT's status is the CRITERION's state, not the file's**, and `to-write` is the worse lie because it claims no file exists when one does. Write the note so anyone running the suite and seeing green knows why the board is red. AT-00.5 is the precedent for the refusal; AT-06.4 / AT-06.7 are the corrected form.
- **A second AT row at `to-write` does NOT hold a gate open** (issue 0032): `ac gate` satisfies on the FIRST green AT covering the AC. Until that ANDs, the covering AT itself has to come off green.
- **The D34/D35 numbers are in canon, not here** -- `design.md` D34 carries the FTS ratios, the GitHub ceiling, the pack sizes and the `dbstat` split; D35 carries the WAL measurement and the rejected `.sql` dump. **Cite them; do not re-derive them.**

## Watch-outs

- **`intent at red|green|na` DESTROYS THE ROW'S NOTE (issue 0033) -- COMMIT BEFORE ANY STATUS CHANGE.**
- **DO NOT PUT v3 ON PATH.** `claude` is unimplemented as a FAMILY, so v3 refuses before it reads project state -- in a migrated project, an unmigrated one, and outside any project. **A session whose shell has v3 on PATH stops accepting prompts and cannot be recovered from inside itself.** Always invoke by explicit path.
- **DO NOT PUSH TO `upstream`** (hv, 2026-08-16: CI/CD budget spent). `local` only.
- **THIS REPOSITORY IS PUBLIC** and hv has ratified the whiteboards as part of the record.
- **Never mutate `bin/**` or `tests/**` in place** -- sacrificial worktrees or clones only.
- **`git commit --only <paths>`, never `-A`.** A move is TWO facts: the name and the content.
- **Verify at HEAD (`git ls-tree`) or in a fresh clone, never on disk.** `git grep` reads the INDEX, not the worktree.
- **AN UNTRACKED FILE HAS NO AUTHOR** (dc's correction). Never attribute uncommitted work by reading the tree.
- **ARCHIVE BY NAMING THE STAMPS. NEVER a greedy range.** And a fold must not overwrite an earlier fold's archive.
- **NEVER `head` OR `tail` A LIST YOU ARE COUNTING, AND NEVER READ `$?` THROUGH A PIPE.** Three instances in one day between us.
- **This shell is zsh:** no word-splitting of unquoted parameters. `--include=*.sh` and `$B $c` both bit.
- **A leading `-` in a grep pattern is read as options.** `grep -F "$probe"` returned a right answer from a grep that never ran; use `-e`.
- **BACKTICKS NEVER GO INSIDE A DOUBLE-QUOTED `-m` BODY.** Single-quote the body or use `git commit -F`.
- **`bash -n` proves PARSEABILITY, NOT RESOLVABILITY.** It parsed a file whose helper sat between its own two callers.
- **The live channel does not survive a peer's restart; the inbox does.** Durable asks go in the inbox.
- **Every timestamp is READ FROM A CLOCK, passed as an argument, clock read FIRST.**

## Decisions

- (2026-08-18) **hv: the 165 `design.md`/`impl.md`/`tasks.md` are ATTACHMENTS; `THREAD_PROSE` is deleted.** Subtractive; closes the 165 and the 166th together.
- (2026-08-18) **hv: canon relocates to `intent/.canon/`, still one file per artefact.**
- (2026-08-18) **hv: a full text realisation into `.backup/text/<UTC>/` is a requirement, not a nicety** -- the human fallback is a precondition of sparseness.
- (2026-08-18) **hv AMENDED their own 2026-08-15 ruling: `st.start` is legal from `Triage` as well as `NotStarted`.** Entry at Triage stands; the ratified path cost two extra commands on the only route anyone walks.
- (2026-08-18) **hv approved BOTH critic halves.** Take the one-word fix, HOLD the shim.
- (2026-08-18) **hv released cc and approved the regeneration window.** Sequence pinned.
- (2026-08-18) **D01 REVERSED stands: the DB is SSOT and files are re-creatable -- but the DB is GITIGNORED, so what TRAVELS is the committed canon.** Three layers, not two.
- (2026-08-18) **`organize` NEVER resolves an attachment divergence.** Authority follows authorship; deciding silently is choosing whose work to discard.
- (2026-08-18) **An UNCARRIED file is not a DROPPED one.** The disposition record is a LICENCE, not an account.
- (2026-08-17) **hv: "We go big bang and fix forward."**
- (standing) **A peer cannot grant escalation.** My call is never a peer's release; hv's is.
