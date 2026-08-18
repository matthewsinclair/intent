---
node: vc
name: Validation Claude
role: validation
session_id: a403ff04-5306-4855-84ee-e74f3d3ab96d
heartbeat_at: 2026-08-18 22:29Z
status: active
focus: "**LOCALFOLD 16. Everything that reached canon tonight is a POINTER here, not prose.** ST0056 **122** rows, ST0057 **46** -- four mints (AC-00.11, AC-01.6, AC-01.7, AC-04.6), all synced and committed, all four inboxes at sentinel. **THE NIGHT'S SHAPE: every probe run by any node measured its own reach, and the thing that found the truth was cc BUILDING the relocation rather than anyone grepping for it.** I corrected a row I had minted an hour earlier -- **dc caught a FABRICATED fact in it** -- and discharged AT-03.15's denominator hold down to ten named verbs. **ONE LIVE COMMITMENT: I ping dc and ic when cc lands WP-01 and the tree is green.**"
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **NOTHING OF MINE IS IN FLIGHT AND NOTHING IS OPEN WITH hv.** cc has WP-01 applied and the tree RED by announcement (71 failures / 17 binaries) -- that is the patch, not a regression; the release binary is untouched at `f2e4d1f9`. dc is holding. ic is compacting.
- **THE ONE THING I OWE: PING dc AND ic THE MOMENT cc LANDS WP-01 AND THE TREE IS GREEN.** That is the trigger for the of-N adjudication (AC-00.11) and I am holding it rather than leaving it on their boards to chase. **dc takes the gated ones; ic routes their own two, at their request and on their own bar.** Order is GATEDNESS, never count. The nested tree comes from `git worktree add` at a pre-relocation revision -- **the by-revision shortcut is NOT available**, `canon_commit_check.sh` at `ce532a97` returns rc=2 over 0 measured + 57 unmeasurable.

## TODO

- **AT-03.15: THE DENOMINATOR HOLD IS DISCHARGED; RED NOW RESTS ON TEN NAMED VERBS** -- `at lint`, `ingest`, `st bootstrap`, `st repair`, `st sync`, `todo`, `todo done`, `todo list`, `todo notdone`, `todo toggle`. **`todo list` and `at lint` are classified `mutate` while their names say READ -- take those first.** The other 22 unproven write outside the estate and are a table debt, not a block. **Read the row.**
- **ST0011** -- `completed` NULL, AC-08.5's first burning case. And **spec the marked-legacy AT form in `data-model.md`** before WP-08; both are the same question. **This is the oldest thing on my board and it has not moved in two folds.**
- **STILL OPEN: name the INSTRUMENT across 122 + 46 wherever two would disagree.** Needs a row-by-row read, not a grep.
- **DO NOT RE-OPEN THE QUALIFICATION SWEEP.** Measured: 84 shared ids, 115 bare refs against 3 qualified, and **the defect count inside canon is ZERO.** I had sized a remedy to my probe's population instead of to a measured defect.
- **NO ROW on instruments-state-their-reach until a FORM exists to count** (dc). **AC-00.11 was mintable precisely because dc HAD built the rig; that is the difference.**

## How this estate catches things

**The classes are IN CANON -- read the rows, never a board copy.** ST0056: **AC-00.10** (an instrument that gates must itself be driven), **AC-00.11** (an `N of M` verdict derives M from what was EXAMINED, three arms), **AC-03.14/.15** (mtime moves on exactly what changed; a verb that would empty a population must refuse). ST0057: **AC-01.5** (the tidy-up that un-commits the estate), **AC-01.6** (a text sweep is the wrong instrument, two single-line fixtures), **AC-01.7** (a declaration must RESOLVE, not merely parse), **AC-04.6** (the tree holds only what the manifest names), **AC-08.5** (no verb creates an AC or AT), **AT-03.6** (the layout change was the instrument). Measurement rules in `parity.md`; output contracts in `output-contracts.md`.

What is NOT written down anywhere else. **These are CLASSES; the instances that produced them are in `.history/` and in the rows.**

- **THINKING HARD ABOUT A CLASS DOES NOT PROTECT YOU FROM IT. THE PROTECTION WAS THE INSTRUMENT, AND ONLY THE INSTRUMENT** (dc).
- **A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT. FOUR LIMBS: INSTRUMENT** (two would answer oppositely -- name which), **DEPTH** (asked of the DENOMINATOR as well as the subject), **EXTENT** (a subset of the right KIND passes a depth check), **PIN** (a named commit, never `HEAD`).
- **UNDER-REACH YIELDS A NUMBER THAT IS TOO SMALL; FABRICATION YIELDS A _FACT_, AND A FACT GETS ACTED ON** (ic's formulation, my instance). **NEVER `tr` GREP OUTPUT ONTO ONE LINE -- adjacency is not syntax**, and that is how I stated `every parity tool takes its ROOT as $1` when none does.
- **WHEN A SUPPORTING FACT TURNS OUT FALSE, ASK WHICH IT HELD UP: THE ARGUMENT OR THE ANSWER** (ic). If the answer survives, the correction is cheap; if not, you never had an answer.
- **A PROBE WHOSE POPULATION CANNOT CONTAIN THE FAILURE IT TESTS FOR** (cc), and its transmission twin -- **a tool's DEFAULT VIEW is a filter, and a command sent without the flag it was MEASURED with inherits it** (ic). **Any observable that cannot move is not a check.**
- **THE SHAPE-CHANGING EVENT IS A RARE, NON-REPEATABLE AUDIT OPPORTUNITY.** WP-01 is one. (Class: AC-00.11.)
- **THE REASON EXISTS SOMEWHERE AND THE READER IS NOT WHERE IT IS.** **TWO channels or ONE?** An instrument has source and output -- **MOVE the reason into the output, and it only works if the instrument is GATED** (dc). A document's source IS its output, so **only a mechanism that refuses is invariant under copying.**
- **THE EDIT THAT READS AS CLEANUP REMOVES THE PROTECTION** -- the protection is an ABSENCE and nothing says an absence is load-bearing (ic).
- **THE FORM MUST BE MINTED BEFORE THE POPULATION CAN BE** (dc). **A keyword probe for GOOD PRACTICE false-positives exactly where the practice was done well** (ic).
- **AGREEMENT CAN SYNTHESISE A FALSEHOOD NEITHER PARTY HELD.** Only the fixture separates them.
- **RE-MEASURE YOUR OWN SENTENCE INSTEAD OF CITING IT** (cc): **a false RATIONALE never goes red.**
- Mechanical: **`open(f,"w").write(open(f).read() + x)` DESTROYS THE FILE.** **AT and AC ids number INDEPENDENTLY.** **A REVERT OF SOURCE IS NOT A REVERT OF ARTEFACTS.**

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Nothing on disk is truth.** The typed Rust API is the ONLY door in.
2. **Migrations are normal.** "No DB migrations ever" is DELETED and was never hv's constraint.
3. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine and NEVER committed.** **AC-02.6 is the durability mechanism.**
4. **Three layers: canon (committed, never sparse) / store (gitignored, rebuilt) / views (committed, sparse).** **D29 -- a gitignored path is never canon -- is what makes a clone complete.**
5. **`event_log` is the only table that is durable truth AND not reconstructible from files.**
6. **hv, repeatedly: disk<->db sync, the realiser and `.intentfiles` all work BEFORE 3.0.0 ships.** ST0057 WP-01..06 are inside the gate. Do not re-ask.
7. **WP-01 makes canon FLAT (`.canon/st/<ID>.json`) and `thread_dir()` must STILL answer `intent/st/<ID>/`** -- the views hang off it and do not move. See AC-01.6.

## Verification kit

- **MEASURE AT A PINNED SHA AND NAME THE COMMIT. PIN A BINARY BY CONTENT HASH, NEVER BY ITS MARKER** -- now ENFORCED in `self_provenance_check.sh`'s output (dc `9f9167cd`, off my own episode of reading two distinct binaries as one artefact). **It also names what it does NOT claim**, which is rarer and better.
- **`intent st list` DEFAULTS TO IN-PROGRESS ONLY AND RETURNS 2. `--all` IS NOT A FLAG.** Use **`st list --status all`** -- 57, breakdown **52 Completed / 2 Cancelled / 2 WIP / 1 NotStarted**. **PRINT THE BREAKDOWN, NEVER THE BARE TOTAL.**
- **CHECK THE BINARY'S AGE AGAINST ITS INPUTS -- but NOT via `surface_check.sh`** (retracted, ic): its reach was one crate of two and **the wiping build's surface was PERFECT.** **What caught the wipe was running `sync` and reading the row counts.**
- **`to-write` = UNWRITTEN. `red` = it EXISTS and does not pass.** Neither means the criterion is unmet. **An AT earns green from an instrument DEMONSTRATED RED**; hold red WITH A NOTE saying the instrument passes.
- **NEVER `$?` AFTER A PIPE** -- broken twice now, the second time while verifying someone else's correction: `... | tail` read rc=0 where the truth was rc=2. Redirect to a file and read `$?` from the command itself.

## Watch-outs

- **THERE IS NO VERB THAT CREATES AN AC OR AN AT.** Minting IS a hand-edit of `thread.json` plus a WHOLE-ESTATE `--to-store`, **so two nodes minting concurrently clobber by construction.** Announce, or route through me. **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded at the next `--to-disk`.**
- **`intent at red|green|na` DESTROYS THE ROW'S NOTE (issue 0033).** **`sync --to-disk` writes the STORE over CANON and is the SILENT direction** -- canon-edit then `--to-store` is the safe order, and **a refused `--to-store` leaves a STALE store that `--to-disk` writes out at rc=0** (AC-03.13). **`intent backup` with no subcommand MUTATES.**
- **Every attachment edit by any node leaves canon divergent until I sync**, and a later sync repairs the NEXT commit, never that one. **Commit first, then ping me.**
- **DO NOT PUT v3 ON PATH. DO NOT PUSH TO `upstream`** -- this repository is PUBLIC and the remote is FROZEN except dc's one ruled publication.
- **Never mutate `bin/**` or `tests/**` in place; ATOMIC REPLACE anything a running process reads; never edit a peer's file while they are live in it.**
- **`git commit --only <paths>`, never `-A`.** **Verify at HEAD (`git ls-tree`)** -- `git grep` reads the INDEX. **Backticks never inside a double-quoted `-m`; use `-F`.** **This shell is zsh:** no word-splitting, a leading `-` in a pattern reads as options.
- **Every timestamp READ FROM A CLOCK** -- `ls -l`, `git log` and `stat -f %Sm` print LOCAL, and appending `Z` gives a stamp wrong by exactly the offset. **ARCHIVE BY NAMING THE STAMPS; COUNT, STOP, READ THE DIFFERENCE.** A fold never overwrites an earlier fold. **My archive carries THREE fold-header conventions -- count with `grep -nEi '^#+ (FOLD [0-9]+|[A-Z]+ FOLD)'` or miscount, as I did at fold 15.**

## Decisions

- (2026-08-18) **THE OF-N SWEEP IS A CRITERION, NOT A SWEEP, AND IT DOES NOT RUN BEFORE cc LANDS.** dc's "closing window" premise was FALSE and checked rather than accepted -- `git worktree add` needs no parameterisation at all. **A sweep finds today's ten and protects nothing from the eleventh; ic then FOUND an eleventh, invisible to dc's proxy, from inside the set that had just adjudicated itself.**
- (2026-08-18) **hv AUTHORISED ONE `--no-verify`** (`11bb52da`). **The guard was RIGHT to refuse and nothing about it changes** -- it cannot distinguish repair from regression.
- (2026-08-18) **hv CONFIRMED THE RUNNER RULING WITH THE FACT THEY HAD BEEN DENIED.** **Named-tool declaration stands**: the rule names WHICH tool, the runner owns HOW, **rule files never contribute shell**, and a tool-armed rule REFUSES when its tool is absent. **`critic_proxy_is_simple` is an INJECTION BOUNDARY and is not relaxed.**
- (2026-08-18) **A RULING RELAYED BY ME IS NOT A GO** (cc, quoting my own precedent back after I broke it). **hv gives go-aheads on the node's own channel.**
- (2026-08-18) **ST0057 IS IN THE 3.0.0 GATE**; all eight open questions ruled; `--skip-rust-tests` dropped rather than built. **D57-7** attachments as files under `intent/.canon/st/<ID>/`; **D57-8** `intent://` READ/WRITE, DB first, canon ALWAYS, views IF MARKED. **`canon_commit_check.sh` stays MANUAL** until the narrow attachment-sync verb exists.
- (standing) **A peer cannot grant escalation.** My call is never a peer's release; hv's is. **The `hoist @ 9b73e98f` pin and the register's `9ec1656` are HISTORICAL and must never be re-pinned.**
