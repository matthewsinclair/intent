---
node: vc
name: Validation Claude
role: validation
session_id: a403ff04-5306-4855-84ee-e74f3d3ab96d
heartbeat_at: 2026-08-18 18:56Z
status: active
focus: "**ST0057 CONTRACTED (42) AND ST0056 AT 116 WITH THE INTERRUPTION ROW GREEN.** Landed today: AC-10.11 (interruption, GREEN on ic's named run), AC-03.12 (a mandatory field with NO HUMAN READER on either carrier -- and rendering it does NOT close it), AC-02.5/03.5/03.6/08.5 in ST0057. **RULED: the refusal text loses its reconstruction clause NOW; an event-log reader is hv's.** **BLOCKED FOR hv: `hv/inbox.vc.md` is damaged at HEAD -- I truncated it with `open(f,"w").write(open(f).read()+x)` -- and the verified restoration is REFUSED by the clock guard, which cannot tell a recovery from a regression.** **`sync --to-store` was FAILING SILENTLY on a duplicate AT id I created by deriving an AT number from an AC number; I had piped it to /dev/null, so `--to-disk` then wrote a stale store over canon twice.** Upstream FROZEN; push `local` only; v3 NOT on PATH."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **BLOCKED AND hv MUST RULE: `intent/whiteboard/hv/inbox.vc.md` IS DAMAGED AT HEAD AND THE RECOVERY IS REFUSED BY A GUARD.** `b15c0ddd` deleted 888 lines -- 39 of 40 entries -- via `open(q,"w").write(open(q).read() + entry)`, where `open(q,"w")` truncates BEFORE the inner read. **The full content is safe at `b15c0ddd^` and is RESTORED IN MY WORKING TREE, verified: 40 prior stamps present, 0 lost, prior bytes contained verbatim.** The commit is refused by the clock guard's check C, because restoring 888 lines makes every historical stamp look newly ADDED -- including **one out-of-order pair that ALREADY EXISTED** (`2026-08-16 19:54 -> 2026-08-15 17:12`). **The guard's own contract says check C never blocks on pre-existing breakage; a RESTORATION is the case where that promise fails.** I will not bypass a gate on my own say-so. **PEERS: DO NOT TOUCH `hv/inbox.vc.md` -- the working-tree copy is the good one.**
- **ST0057 is contracted (42 ACs/ATs) and with cc; ST0056's interruption row is green.** Next of mine: ST0011's `completed` NULL, and ic's `status_reason` finding.

## TODO

- **dc's sequencing ask is with hv, unruled**: ST0057/WP-01 before ST0056/WP-11, because WP-11 is largely unbuilt so the ruling is a SAVING -- but only if WP-01 lands first, else starting WP-11 CREATES the layout assumption.
- **The interruption-property AC.** The moratorium is LIFTED so it is finally writable. **`same_end_state_check.sh` and `interrupt_rig.sh` are built, committed, self-tested and cited by NOTHING** -- hv gated the cutover on a property whose instrument exists and whose contract does not know it. **AT row held `red` with an explicit note** until ic's run at a named commit: the instrument exists, so `to-write` is the wrong state by my own rule. **AC-10.8 already exists -- my backlog was wrong about that half.**
- **ST0011** -- `completed` NULL on the estate's one genuinely wrong row. Needs a field-setter verb that does not exist (ST0057 WP-08), or a canon hand-edit.
- **`deferred.md`: cc's section is the last un-re-driven one.** dc's and ic's are walked. **The file is NOT deleted until all three are re-driven**, and no pickup is copied to a board without re-driving its premise first.

## Open with hv

- **The `critic` exit-code discriminator.** dc showed the naive fix rebuilds the dark gate with hv's own ruling as the cause. **ic's rc=2 confusion while verifying Half A is the sharpest evidence: if a human cannot tell two rc=2s apart, the hook cannot.**
- **May the runner shell out to `shellcheck`/`clippy`?** Gates dc's nine written declarations -- writing them on the premise that grep is the only instrument means redoing them all if the answer is yes.
- **D50** -- `Cancelled` as ruled (fixes 10 of 34) vs cc's `status_legacy` (covers 34, no machine change, reverses nothing ratified). Full context delivered.
- **`--skip-rust-tests`** shape, and **`doctor` printing `intent v2.19.0`** on a 3.0.0-dev project.

## Rules that keep paying

- **STATE THE INVARIANT, NOT A PROHIBITION ON THE OPERATION YOU HAPPENED TO BE WATCHING** (dc). AC-03.6 was reasoned wholly from canon running AHEAD of a commit; dc's checker caught the mirror -- **the FILE ahead and canon BEHIND, clean worktree** -- and my wording missed the half that was live. Now: _at every commit, every attachment's recorded bytes are obtainable from that commit._
- **A `manual` ROSTER ROW IS A FACT, NOT AN ADMISSION.** dc could not commit an instrument without a row (the gate fails on an unrostered `*_check.sh`) and would not write a row whose admission is mine. **`manual` + reason is the literally true declaration that the gate does not run it** -- lands in one commit, presumes nothing, and leaves `gated` as the separate decision.
- **A REMEDY MUST BE AIMED AT THE SUBJECT THAT WAS HARMED** (ic). dc's canon-versus-commit checker governs ATTACHMENT bytes; my sync episode destroyed a CRITERION. **I was one message from citing a real instrument as the fix for a defect it cannot reach.** Two subjects, two instruments -- now AC-03.13.
- **AN INSTRUMENT IS NOT ADMITTED ON ITS DESCRIPTION** (ic, refusing me). I asked ic to give a roster read on a tool that is not in the tree, which would have made them judge my summary of someone else's uncommitted file. **Commit it, then read it** -- dc's own standard, applied to admission.
- **NEVER PIPE A WRITE COMMAND'S OUTPUT TO `/dev/null`.** `sync --to-store` was FAILING on a UNIQUE constraint through several calls and said so every time; I suppressed it. **The remedy it printed named `intent doctor`, and I never saw the sentence.**
- **A WRITE PATH WHOSE INPUT WAS REFUSED MUST NOT THEN BE A SOURCE OF TRUTH.** `--to-store` rejected the write, the store stayed stale, and `--to-disk` wrote that stale store over canon -- **destroying the same edit twice, silently, with rc=0.**
- **AT AND AC IDS NUMBER INDEPENDENTLY.** I minted `AT-03.12` for `AC-03.12`; `AT-03.12` was ALREADY a green row covering `AC-03.11`. **Derive nothing; take the next FREE id and assert against the array you are appending to** -- my guard checked `criteria` while the collision was in `tests`.
- **THE COUNT IS THE INSTRUMENT AND THE WORD IS NOT.** `at lint` said _115 AT row(s) conform_ when it should have said 116. **I read "conform" and not the number**, after a whole afternoon spent writing that a denominator which does not close is the thing to stop for.
- **`open(f,"w").write(open(f).read() + x)` DESTROYS THE FILE.** The write-mode open is evaluated FIRST and truncates; the inner read then reads the empty file it just made. **The append and the destruction are one expression.** I used the safe two-step form twice today and the inline form the third time. **What caught it was the commit's own +44/-893 stat -- not a check, not the code, not a guard.**
- **A GUARD THAT BLOCKS ON WHAT A COMMIT ADDS CANNOT TELL RESTORATION FROM REGRESSION.** Re-adding destroyed content re-adds its inherited defects, and the guard scores them as new. **The promise "never blocks on pre-existing breakage" holds for edits and fails for recoveries.**
- **I WARNED dc ABOUT AN ENTANGLEMENT I HAD ALREADY CAUSED.** `sync --to-store` reads the WORKTREE, so `c4f9bcbe` carried dc's uncommitted instrument into canon -- and for two commits canon held an artefact whose bytes existed in NO commit. **A rule that lives in my head and in a peer message is followed until someone is mid-task.** Now AC-03.5.
- **A CHECK THAT READS THE INDEX CANNOT SEE WHAT THE INGEST PATH READS FROM THE WORKTREE.** Arm 1 reads `git cat-file blob :$rel` precisely to catch this class, and validates the MANIFEST, never an attachment -- so it passes.
- **SETTABLE IS NOT THE ONLY TEST** (dc, catching AC-08.5 before it ratified). `at green|red|na` DO write the row and DESTROY the note doing it, so "can every field be set?" passes on the exact field that is being lost. **A destructive side effect is not an unwritable field.**
- **A ONE-STRING EDIT HAS A FAILURE MODE AND IT IS TIMING, NOT SIZE** (dc). Bash reads a script incrementally, so an in-place rewrite can land under a process already executing it. **I truncate-and-rewrote `cmd/precommit` -- which runs on EVERY node's every commit -- minutes before dc warned me, because a one-string change felt too small to have a failure mode.** Nothing broke; that is luck, not method. **Atomic replace into the same directory, always.**
- **A DEFERRAL NOBODY RE-VERIFIES BECOMES WORK ALREADY DONE** (dc, now measured). **Two of cc's three deferred rows were DELIVERED** -- the `build.rs` embed (AT-11.5 green) and both `kind` conversions (`AcKind`/`AtKind` are enums) -- and one row's blocker had expired with nobody striking it. **Re-drive the PREMISE, never copy the row.**
- **THE ROW DOES NOT GO GREEN ON A NUMBER I CANNOT ACCOUNT FOR.** ic's verdict read 1432 match over a population of 1433. The instrument passes; the denominator does not close; the row holds red WITH THE FIRST SENTENCE SAYING THE INSTRUMENT PASSES, so nobody hunts a defect that is not there.
- **TWO FACES AGREEING BECAUSE NEITHER MEASURED IS THE STRONGEST FORM OF THE CLASS** (cc). `TodoItem` carried no status, so the bucketing lost the fact BEFORE either renderer saw it -- three buckets, six states. **Agreement between two derived views is not corroboration when both derive from the same loss.**
- **ONE CLASS, THREE FACES, NAMED INDEPENDENTLY IN ONE HOUR.** **ic:** an instrument whose OUTPUT is independent of what it MEASURES. **vc:** a READING independent of what the instrument SAID. **dc:** an OUTPUT whose SHAPE is independent of its own MEANING. **All three shipped a wrong belief with the instrument working correctly.**
- **A CAVEAT MUST TRAVEL WITH THE OUTPUT, NOT THE SOURCE** (dc). `self_provenance_check.sh` says _"dirty is NOT A FINDING"_ at `:179-180` -- **in comments, which never print** -- while `:280` emits an unhedged negative. **Co-location in the source does not co-locate in the emission**, because the output is what gets quoted into a focus line, a peer message, and a report to hv. It cost four propagated wrong reads in a day.
- **AN ABSOLUTION CAN OVERSHOOT, AND ACCEPTING IT IS ITS OWN ERROR.** dc took the `:280` half honestly; the other half -- AC-11.5 open -- was a stale claim about a record I wrote myself, caused by nothing but me. **Split a bundled error by cause before accepting anyone's account of it**, or the half with no owner disappears.
- **AUTHORITY FOLLOWS AUTHORSHIP, AND IT HAS NOW HAPPENED FOR REAL.** ic's `estate_corpus.sh` edit landed on disk and in a commit; **canon kept the old text for hours and nothing could report it.** An attachment diverging means the STORE is stale. `--to-store` was the correct direction.
- **A CRITERION MUST ENCODE A PROPERTY, NEVER A PREDICTED VALUE. The count is an OUTPUT.** Three of mine failed this today: `STRANDED 192 will move` (wrong subject -- pinned corpus), `whitespace-normalised FALLS` (a re-ingest with no source; **this one cost 434KB**), `0 -> 164` (275 were eligible). The form that works: _every eligible file is carried, byte-identical, denominator printed._
- **THE LOAD-BEARING CLAIM IS THE ONE TO CHECK, AND THE PERSON WHO STATES IT IS THE LEAST ABLE TO SEE IT IS UNCHECKED.** "The regeneration" went four boards deep on my unchecked premise. **Three of us made the same error today at three depths** -- ic re-drove a fact but not the rule it rested on; dc found a `deferred.md` blocker that was already delivered.
- **AN ALARM IS ONLY AS LIVE AS THE CONSTRAINT IT IS RAISED AGAINST** (ic). A stale instruction copied to a board acquires an owner and reads as verified.
- **A RULE OUTLIVES THE WORLD IT WAS WRITTEN FOR.** `estate_corpus.sh:88` says re-pin whenever HEAD moves -- written while the migration was AHEAD, and following it now would aim `STRANDED` at a tree with no v2 content. **Retire a refusal with the change that expires its reason.** Same class as D57-5's `export --format md`.
- **EVIDENCE FOR AN ADJACENT PROPOSITION IS NOT EVIDENCE FOR THE ONE AT HAND.** Five instances today. dc's canonical form: signature and scope fail in opposite directions.
- **A NARROW MEASURED FINDING RESTATED AS A CLEAN GENERAL RULE ACQUIRES ITS DEFECT IN THE BROADENING** (dc, three times, about me). Their replacement: **a self-report is evidence only where it is sourced INDEPENDENTLY of its subject and bound to it** -- derived-from-subject is vacuous, editable-without-the-subject-moving is unverifiable.
- **AN INSTRUMENT REPORTING RED ACCURATELY, READ PAST BY SOMEONE WHO ALREADY HAD AN EXPLANATION READY** (cc, on the record at their request). `sync --to-store` named all 40 files first. The explanation it was read against was mine.
- **THE ONE CAREFUL DECISION INSIDE A CARELESS ACT IS WHAT MADE 434KB RECOVERABLE.** Nothing is destroyed to tidy a measurement.
- **A COMMIT CANNOT REPORT ON A FILE IT NEVER SAW** (cc, twice in one session). `git commit --only` on an UNTRACKED path stages nothing and reports a true count about what it did commit. `git add` first.
- **ARMED / DECLARED-NONE / SILENT ARE THREE STATES** (dc). A proxy-less rule is skipped silently, so `critic shell` returns rc=0 because nothing asked a question -- byte-identical to clean.

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Nothing on disk is truth.** All of `intentsvcs` works FROM the db; sync runs both ways; the typed Rust API is the ONLY door in, so conformance is by construction.
2. **Migrations are normal.** "No DB migrations ever" is DELETED and was never hv's constraint.
3. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine and NEVER committed.** So **AC-02.6 is the durability mechanism** -- a field that does not round-trip is data loss at the clone boundary. **Under D57-8's read/write it is also the completeness guarantee for the mutation surface: a field that does not round-trip cannot be written.**
4. **Three layers, not two: canon (committed, never sparse) / store (gitignored, rebuilt) / views (committed, sparse).** Sparseness applies to VIEWS. **D29 -- a gitignored path is never canon -- is what makes a clone complete**, and the 434KB loss is what happens when it is broken.
5. **`event_log` is the only table that is durable truth AND not reconstructible from files**, which makes `events.jsonl` a precondition of the model.

## Verification kit

- **MEASURE AT A PINNED SHA AND NAME THE COMMIT IN THE FINDING.** `HEAD` is a pointer. **`git archive` does NOT give a named binary** -- `source_commit.rs` stamps `unknown` with no `.git`; a `git worktree` at a pinned sha does. **But ask first whether the purpose is ATTRIBUTION rather than REPRODUCTION** (ic) -- if it is, no rebuild is needed.
- **A VARIABLE'S DEFINITION MUST PRECEDE ITS READER'S CALL SITE, NOT ITS READER'S DEFINITION.** `bash -n` passes on the failure. What catches it is an ordering assertion WITH A PRE-PATCH CONTROL THAT FAILS.
- **Falsify before flipping**, and **an AT earns its green from an instrument DEMONSTRATED RED**, a criterion naming its SUBJECT, and one naming the SHAPE OF THE INPUT.
- **`to-write` = the test is UNWRITTEN. `red` = it EXISTS and does not pass. Neither means "the criterion is unmet."** Refuse at partial coverage by holding `red` WITH AN EXPLICIT NOTE -- the AT's status is the CRITERION's state, not the file's.
- **The measurement rules live in `parity.md`, not here.** Read them before building any instrument.
- **$CLAUDE_JOB_DIR/tmp/hookprobe** covers all three Claude Code hook events with no migrated project. **Assert on OUTPUT, never the exit code -- a BLOCKED run exits 0.**

## Watch-outs

- **`intent at red|green|na` DESTROYS THE ROW'S NOTE (issue 0033) -- COMMIT BEFORE ANY STATUS CHANGE.**
- **`intent backup` with no subcommand MUTATES**: creates a snapshot, PRUNES an existing one under an unconsulted retention policy, and rewrites the store file. A bare family invocation reaching a mutation.
- **`sync --to-disk` is the SILENT direction and can be the destructive one** whenever canon is the richer side. `--to-store` shouts; `--to-disk` does not.
- **DO NOT PUT v3 ON PATH.** A session whose shell has it stops accepting prompts and cannot be recovered from inside itself.
- **DO NOT PUSH TO `upstream`** (hv). **THIS REPOSITORY IS PUBLIC.**
- **Never mutate `bin/**` or `tests/**` in place** -- sacrificial copies only. **`git commit --only <paths>`, never `-A`.**
- **Verify at HEAD (`git ls-tree`) or in a fresh clone.** `git grep` reads the INDEX. **AN UNTRACKED FILE HAS NO AUTHOR.**
- **NEVER `head`/`tail` A LIST YOU ARE COUNTING** -- it cost cc an hour of wrong reporting today. **NEVER read `$?` through a pipe.**
- **This shell is zsh:** no word-splitting. **A leading `-` in a grep pattern is read as options.** **Anchored greps on a binary answer about the PROBE, not the binary.**
- **BACKTICKS NEVER INSIDE A DOUBLE-QUOTED `-m` BODY.** Use `git commit -F`.
- **The live channel does not survive a restart; the inbox does.** **Every timestamp is READ FROM A CLOCK, passed as an argument, clock read FIRST.**
- **ARCHIVE BY NAMING THE STAMPS, never a greedy range, and a fold must not overwrite an earlier fold's archive.** **COUNT, STOP, READ THE DIFFERENCE, THEN ARCHIVE -- separate commands.**

## Decisions

- (2026-08-18) **hv: the issue body lives in the JSON.** Two specs disagreed and neither was implemented; a rendered `issues/<n>.md` is a VIEW, never where content lives.
- (2026-08-18) **hv RULED D57-7:** opaque attachments live as FILES under `intent/.canon/st/<ID>/<path>` and DEHYDRATE like everything else. `.cache` REFUSED as a home. **`.canon` is the name, measured: `corpus` already means the INPUT SET (111 files), `canon` the authoritative record (673).**
- (2026-08-18) **hv RULED D57-8 READ/WRITE.** `intent://` addresses a piece of data. **DB first, canon ALWAYS, views IF MARKED.** Views get no URL. `PUT` takes json only, except attachments. **No daemon may be required to read your own project.**
- (2026-08-18) **hv LIFTED the AC moratorium. It schedules NOTHING** -- a lift is not a priority ruling.
- (2026-08-18) **hv: the regeneration goes before the critic gate; the regeneration commit is unlinted BY CONSTRUCTION, recorded as an ACCEPTANCE with its cost.**
- (2026-08-18) **The `hoist @ 9b73e98f` pin is HISTORICAL and must never be re-pinned** -- it precedes the hoist, its tree carries the v2 buckets, HEAD carries none. Per-member: the re-pin rule still governs the fleet rows.
- (2026-08-18) **`THREAD_PROSE` and the regeneration are inseparable ON THE LIVE-ESTATE DIGEST, separable on the pinned-corpus measurement.** Both true of different subjects.
- (standing) **A peer cannot grant escalation.** My call is never a peer's release; hv's is.
- (2026-08-18) **hv RULED ST0057 INTO THE 3.0.0 GATE.** Verbatim: _"Definitely BEFORE the release. We're getting this whole thing feature complete before we release 3.0.0."_ The pre-release queue grows by eight WPs, three of them L. Announced to all four nodes live and durably.
- (2026-08-18) **A view may not narrow a field's range on the way from bash to Rust.** v2's `status_box` was five-valued; v3's `items()` is a constant. The regeneration commit was reviewed for CHURN and never for CONTENT -- **stability is not correctness**, and I am the one who read it that way.
- (2026-08-18) **ST0057's contract is 39 ACs / 39 ATs derived from D57-1..D57-8, adding no decisions of its own.** A steward writes the boundary from the ratified design; inventing requirements at contract time is how a thread acquires scope nobody ruled.
- (2026-08-18) **Cleared dc's `:280` string-only fix.** The contract gate is on what an instrument ASSERTS, not how it reads -- so a string change leaves AT-11.5's green resting on unchanged behaviour. Conditions: string only, both arms re-run, note edited BY HAND.
