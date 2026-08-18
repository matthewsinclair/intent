---
node: vc
name: Validation Claude
role: validation
session_id: a403ff04-5306-4855-84ee-e74f3d3ab96d
heartbeat_at: 2026-08-18 18:09Z
status: active
focus: "**hv RULED ST0057 INTO THE 3.0.0 GATE: feature complete BEFORE we release.** It is 0 of 8 WPs with no objective, no context, and NO ACCEPTANCE CONTRACT AT ALL -- and `.intentfiles` and `intent/.canon/` do not exist, so 468 of the 797 files under `intent/st/` belong to threads nobody is working on. **WRITING ST0057'S CONTRACT NOW** so it reaches cc ratified rather than as my prose. **hv found it by looking at their own file tree, minutes after I reported nothing of mine was outstanding -- I scoped "outstanding" to my INBOX and said it in the grammar of a claim about the ESTATE.** Found on the way: **the v3 view generator collapsed v2's five-valued status glyph to a constant `[ ]`** (`views.rs:838-846` vs `intent_todo:63-73`); sent to cc, and to ic as a class. Upstream FROZEN; push `local` only; v3 NOT on PATH."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **ST0057's acceptance contract.** hv ruled it into the 3.0.0 gate; it has `objective: ""`, `context: ""`, 0 ACs and 0 ATs, so there is nothing for cc to build against but eight hand-written WP bodies. Objective + context + the AC/AT boundary, in that order.

## TODO

- **ST0057 WP-01/WP-02 ordering** against cc's ST0056 queue -- asked, not answered. WP-01 (canon relocation) and WP-02 (`.intentfiles`) unblock the other six.
- **The interruption-property AC.** The moratorium is LIFTED so it is finally writable. **`same_end_state_check.sh` and `interrupt_rig.sh` are built, committed, self-tested and cited by NOTHING** -- hv gated the cutover on a property whose instrument exists and whose contract does not know it. **AT row held `red` with an explicit note** until ic's run at a named commit: the instrument exists, so `to-write` is the wrong state by my own rule. **AC-10.8 already exists -- my backlog was wrong about that half.**
- **ST0011** -- `completed` NULL on the estate's one genuinely wrong row. Needs a field-setter verb that does not exist (ST0057 WP-08), or a canon hand-edit.
- **`deferred.md`: cc's section is the last un-re-driven one.** dc's and ic's are walked. **The file is NOT deleted until all three are re-driven**, and no pickup is copied to a board without re-driving its premise first.
- **WP-07/WP-08 acceptance criteria** for ST0057, now that ACs can be minted.

## Open with hv

- **The `critic` exit-code discriminator.** dc showed the naive fix rebuilds the dark gate with hv's own ruling as the cause. **ic's rc=2 confusion while verifying Half A is the sharpest evidence: if a human cannot tell two rc=2s apart, the hook cannot.**
- **May the runner shell out to `shellcheck`/`clippy`?** Gates dc's nine written declarations -- writing them on the premise that grep is the only instrument means redoing them all if the answer is yes.
- **D50** -- `Cancelled` as ruled (fixes 10 of 34) vs cc's `status_legacy` (covers 34, no machine change, reverses nothing ratified). Full context delivered.
- **`--skip-rust-tests`** shape, and **`doctor` printing `intent v2.19.0`** on a 3.0.0-dev project.

## Rules that keep paying

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
