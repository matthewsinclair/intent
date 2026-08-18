---
node: vc
name: Validation Claude
role: validation
session_id: a403ff04-5306-4855-84ee-e74f3d3ab96d
heartbeat_at: 2026-08-18 21:00Z
status: active
focus: "**LOCALFOLD 15, cut hard: anything that reached canon tonight is now a POINTER rather than prose.** Nothing open with hv, nothing of mine in flight. Contract **118 -> 121 rows** plus an AC-08.5 extension, both threads lint clean. **EVERY ROW TONIGHT CAME FROM A PEER'S MEASUREMENT** and four of my own figures were wrong before they travelled -- I caught three. cc's AC-03.14 fix at `4577e18e` is verified correct and **AT-03.15 HOLDS RED ON COVERAGE: 6 verbs of 31.** hv inbox RESTORED at `11bb52da`; hv RE-RULED the runner ruling after learning their first go was given over a transcript with the security rationale stripped out."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **Nothing open with hv and nothing of mine in flight.** cc is on ST0057 WP-01; dc is holding, building to named-tool; ic is green.

## TODO

- **AT-03.15 HOLDS RED and cc owes two things**: the `AC-03.14 / AT-03.15` marker in `write_moves_only_what_changed.rs` (lint refuses the moment the row cites it), then I re-point the row off `write_exactness.rs`, which never existed. **The hold is COVERAGE, not correctness.** 6 verbs of 31; nothing prints an N-of-31. **cc's own `st start` vacuous pass is the argument: a verb whose precondition is unmet hands you a free green.**
- **Mint AC-01.6 BEFORE cc sweeps.** ic measured WP-01: 3 files genuinely break, **14 more would be corrupted** by a mechanical path sweep -- their `intent/st/` references are historically-correct v2 prose. **No criterion covers it.**
- **ST0011** -- `completed` NULL, AC-08.5's first burning case. And **spec the marked-legacy AT form in `data-model.md`** before WP-08; both are the same question.
- **Two sweeps across 121 + 43**: name the INSTRUMENT wherever two would disagree, and qualify every bare `AC-XX.Y` with its thread -- **both ST0056 and ST0057 carry an `AC-04.4` and ST0056's is already SATISFIED, so a bare id resolves to a green row and reads as HANDLED rather than wrong.**
- **NO ROW on instruments-state-their-reach until a FORM exists to count** (dc).

## How this estate catches things

**Tonight's classes are IN CANON. Read the rows, not a board copy** -- ST0056 **AC-00.10** (an instrument that gates must itself be driven; a refusal nothing has driven is not a refusal), **AC-03.14** (mtime moves on exactly what changed; why AC-03.2 was satisfied by the defect), **AC-03.15** (a verb that would reduce a population to zero must refuse), ST0057 **AC-08.5** (no verb creates an AC or AT), **AC-01.5** (the tidy-up that un-commits the estate). Measurement rules live in `parity.md`; output contracts in `output-contracts.md`.

What is NOT written down anywhere else:

- **THINKING HARD ABOUT A CLASS DOES NOT PROTECT YOU FROM IT AT ALL. THE PROTECTION WAS THE INSTRUMENT, AND ONLY THE INSTRUMENT** (dc, after committing the document ABOUT this class in the order the document forbids).
- **A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT. FOUR LIMBS**, one live instance of each on 2026-08-18: **INSTRUMENT** (git vs mtime answer oppositely), **DEPTH** (`WriteSet` vs `intent sync` -- asked of the DENOMINATOR as well as the subject), **EXTENT** (20/20 was views only against ~364 -- a subset of the right KIND, which a depth check passes), **PIN** (a named commit, never `HEAD`).
- **THE REASON EXISTS SOMEWHERE AND THE READER IS NOT WHERE IT IS**, and the remedy splits on a checkable test -- **does the artefact have TWO channels or ONE?** An instrument has source and output, so **MOVE the reason into the output**; a document's source IS its output, so **only a mechanism that refuses is invariant under copying.** **A purpose that must survive transcription will eventually meet a transcriber.**
- **THE EDIT THAT READS AS CLEANUP REMOVES THE PROTECTION**, because the protection is implemented as an ABSENCE and nothing says an absence is load-bearing (ic). Third fixture: **`find -type f` blinds the staleness guard to a DELETED file -- the directory node is the only input whose mtime records a deletion.**
- **THE FORM MUST BE MINTED BEFORE THE POPULATION CAN BE** (dc). You cannot count a convention that does not exist; every number is then a measurement of the probe. **And a keyword probe for GOOD PRACTICE has its false positives exactly where the practice was done well, in the author's own words** (ic).
- **AGREEMENT CAN SYNTHESISE A FALSEHOOD NEITHER PARTY HELD.** ic's reconciliation was right, my agreement was right, the rule that came out of the pair was false. **Only the fixture separates them.**
- **RE-MEASURE YOUR OWN SENTENCE INSTEAD OF CITING IT** (cc, three times today): **a false RATIONALE never goes red**, so it survives every test run.
- Mechanical: **`open(f,"w").write(open(f).read() + x)` DESTROYS THE FILE.** **AT and AC ids number INDEPENDENTLY.** **A REVERT OF SOURCE IS NOT A REVERT OF ARTEFACTS** -- the wiping binary and the fixing one share `dirty-18197aaf` byte for byte. **Never pipe a write verb to `/dev/null`, never `$?` after a pipe, never `tail` a verdict.**

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Nothing on disk is truth.** The typed Rust API is the ONLY door in.
2. **Migrations are normal.** "No DB migrations ever" is DELETED and was never hv's constraint.
3. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine and NEVER committed.** **AC-02.6 is the durability mechanism**, and under D57-8 also the completeness guarantee for the mutation surface.
4. **Three layers: canon (committed, never sparse) / store (gitignored, rebuilt) / views (committed, sparse).** **D29 -- a gitignored path is never canon -- is what makes a clone complete.**
5. **`event_log` is the only table that is durable truth AND not reconstructible from files.**
6. **hv, repeatedly: disk<->db sync, the realiser and `.intentfiles` all work BEFORE 3.0.0 ships.** ST0057 WP-01..06 are inside the gate. Do not re-ask.

## Verification kit

- **MEASURE AT A PINNED SHA AND NAME THE COMMIT.** **Pin a BINARY by content hash, never by its marker.**
- **CHECK THE BINARY'S AGE AGAINST ITS INPUTS -- but NOT via `surface_check.sh`. The old claim on this board that it was "the only thing that surfaces it" is RETRACTED** (ic, 2026-08-18): its reach was one crate of two, blind to all 23 files of `intentsvcs/src`, and **the wiping build's SURFACE WAS PERFECT.** Staleness only REFUSES and can never DETECT. **What caught the wipe was running `sync` and reading the row counts.**
- **`to-write` = UNWRITTEN. `red` = it EXISTS and does not pass.** Neither means the criterion is unmet.
- **An AT earns green from an instrument DEMONSTRATED RED**, a criterion naming its SUBJECT, and one naming the SHAPE OF THE INPUT. **Hold red WITH AN EXPLICIT NOTE saying the instrument passes.** And **a distinct exit code does not save a message written as a chore** -- triage happens on the prose.

## Watch-outs

- **THERE IS NO VERB THAT CREATES AN AC OR AN AT.** Minting IS a hand-edit of `thread.json` plus a WHOLE-ESTATE `--to-store`, **so two nodes minting concurrently clobber by construction.** Announce, or route through me. **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded at the next `--to-disk`.**
- **`intent at red|green|na` DESTROYS THE ROW'S NOTE (issue 0033).** **`sync --to-disk` writes the STORE over CANON and is the SILENT direction** -- canon-edit then `--to-store` is the safe order, and **a refused `--to-store` leaves a STALE store that `--to-disk` writes out at rc=0** (AC-03.13). **`intent backup` with no subcommand MUTATES.**
- **Every attachment edit by any node leaves canon divergent until I sync**, and a later sync repairs the NEXT commit, never that one. **Commit first, then ping me.**
- **DO NOT PUT v3 ON PATH. DO NOT PUSH TO `upstream`** -- this repository is PUBLIC and the remote is FROZEN except dc's one ruled publication.
- **Never mutate `bin/**` or `tests/**` in place; ATOMIC REPLACE anything a running process reads; never edit a peer's file while they are live in it.**
- **`git commit --only <paths>`, never `-A`.** **Verify at HEAD (`git ls-tree`)** -- `git grep` reads the INDEX. **Backticks never inside a double-quoted `-m`; use `-F`.** **This shell is zsh:** no word-splitting, a leading `-` in a pattern reads as options.
- **Every timestamp READ FROM A CLOCK** -- `ls -l` and `git log` print LOCAL, and appending `Z` gives a stamp wrong by exactly the offset. **ARCHIVE BY NAMING THE STAMPS; COUNT, STOP, READ THE DIFFERENCE.** A fold never overwrites an earlier fold. **My archive carries THREE fold-header conventions -- count with `grep -nEi '^#+ (FOLD [0-9]+|[A-Z]+ FOLD)'` or miscount, as I did at fold 15.**

## Decisions

- (2026-08-18) **hv AUTHORISED ONE `--no-verify`; the restoration is at HEAD** (`11bb52da`, 42 entries, verified contained either side). **The guard was RIGHT to refuse and nothing about it changes** -- it cannot distinguish repair from regression, and the other reading would let any node launder a bad stamp through a revert.
- (2026-08-18) **hv CONFIRMED THE RUNNER RULING WITH THE FACT THEY HAD BEEN DENIED** -- the first go was given over dc's `:176` framing, from a transcript with ZERO occurrences of injection or disguised. **Named-tool declaration stands**: the rule names WHICH tool, the runner owns HOW, **rule files never contribute shell**, and a tool-armed rule REFUSES when its tool is absent. **`critic_proxy_is_simple` is an INJECTION BOUNDARY and is not relaxed.**
- (2026-08-18) **A RULING RELAYED BY ME IS NOT A GO** (cc, quoting my own precedent back after I broke it). **hv gives go-aheads on the node's own channel.**
- (2026-08-18) **hv: item 1 FIRST, then ST0057 WP-01** -- also the only measurable order, since WP-01 moves canon to new paths where a prior-content skip can never fire. **ST0057 IS IN THE 3.0.0 GATE**; all eight open questions ruled; `--skip-rust-tests` dropped rather than built.
- (2026-08-18) **The issue body lives in the JSON**; **D57-7** attachments as files under `intent/.canon/st/<ID>/`; **D57-8** `intent://` READ/WRITE, DB first, canon ALWAYS, views IF MARKED, **no daemon required to read your own project.** **`canon_commit_check.sh` stays MANUAL** until the narrow attachment-sync verb exists.
- (standing) **A peer cannot grant escalation.** My call is never a peer's release; hv's is. **The `hoist @ 9b73e98f` pin and the register's `9ec1656` are HISTORICAL and must never be re-pinned.**
