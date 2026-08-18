---
node: vc
name: Validation Claude
role: validation
session_id: a403ff04-5306-4855-84ee-e74f3d3ab96d
heartbeat_at: 2026-08-18 19:47Z
status: active
focus: "**LOCALFOLD 14. hv RULED ALL EIGHT OPEN QUESTIONS** -- carrying each to its owner; `--skip-rust-tests` is DROPPED on my own answer to their question. **A SHARED BINARY EMPTIED THE STORE**: cc reverted WP-01 in SOURCE, the ARTEFACT stayed, and `sync --to-store` printed _the store and the extract agree_ over **0 == 0** while `--to-disk` wiped 57 and 82 rows from two views. Restored from HEAD; canon never moved. **THE BINARY THAT WIPED THE ESTATE AND THE ONE THAT FIXED IT SHARE A MARKER BYTE FOR BYTE.** **AC-04.4's VIOLATOR IS STILL LIVE** -- pre-registered prediction FAILED, and the cause is that cc's correct guard sits on `write_all`, which has NO PRODUCTION CALLER. ST0056 118 rows, ST0057 43. Upstream FROZEN except dc's one ruled publication; v3 NOT on PATH."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

- **hv RULED ALL EIGHT open questions (2026-08-18 evening).** Carrying each to its owner and minting what needs minting. **Nothing is open with hv except the one thing only they can release: the damaged inbox below.**

## TODO

- **Carry the eight rulings**: critic exit-code = a DISTINCT CODE (my rec, proved by a RED); runner MAY shell out to `shellcheck`/`clippy` -- tell dc their nine declarations can assume a real parser; publication freeze LIFTED for dc's one publication; D50 = cc's `status_legacy` (**my reading of a terse "Ok" -- flagged to hv**); event-log READER to build, and hv's larger point that **all intentsvcs calls go through it** wants a design decision, not an AC; `doctor` = dc's option 3; **`--skip-rust-tests` = DROP IT, it is an escape hatch around a refusal I built on purpose.**
- **AC-04.4's violator is STILL LIVE.** cc's guard at `views.rs:948-952` is correct and `write_all` HAS NO PRODUCTION CALLER -- the live path is `facade.rs:125`/`:150`, unguarded `fs::write`. Pre-registered prediction FAILED: sync 1 moved 20/20, sync 2 moved 20/20.
- **ST0011** -- `completed` NULL, AC-08.5's first burning case.
- **ST0057 WP-01 resumes with cc**: ~15 test fixtures each spell `intent/st/<ID>/thread.json` independently. Canon has NOT moved; `intent/.canon/` does not exist.

## Open with hv -- ONE thing

- **`hv/inbox.vc.md` IS DAMAGED AT HEAD: 1 entry against 41.** I truncated it with `open(f,"w").write(open(f).read() + entry)`. **The restoration is verified in my working tree -- 40 prior stamps present, 0 lost, prior bytes contained verbatim -- and the CLOCK GUARD REFUSES IT**, because restoring 888 lines makes every historical stamp look newly ADDED, including one out-of-order pair that already existed. **The guard's contract says check C never blocks on inherited breakage; a RESTORATION is where that promise fails.** I will not bypass a gate on my own say-so.

## Rules that keep paying

- **THINKING HARD ABOUT A CLASS DOES NOT PROTECT YOU FROM IT AT ALL. THE PROTECTION WAS THE INSTRUMENT, AND ONLY THE INSTRUMENT** (dc, after committing the document ABOUT this class in the order the document forbids, while typing the sentence that forbids it). **The argument for every mechanical guard here, made by the person with most reason to believe care would do.**
- **THE MEASUREMENTS THAT SURVIVED WERE SAVED BY A VALUE THAT CONTRADICTED THE DESIGN, NEVER BY CARE** (dc). A merely plausible number gets published, every time. Mine tonight: `0 thread(s)` in the one call I left visible.
- **A CRITERION ENCODES A PROPERTY, NEVER A PREDICTED VALUE. The count is an OUTPUT.** Three of mine failed this and one cost 434KB. **And RECORD A PREDICTION BEFORE THE FACT** -- tonight's mtime test failed honestly only because the expected result was in canon first.
- **NEVER PIPE A WRITE COMMAND'S OUTPUT TO `/dev/null`. NEVER `$?` AFTER A PIPE. NEVER `tail` A VERDICT.** All three cost me a wrong reading today; the third gave me `rc=0` from `tail` and hid half a finding.
- **A CLOSING COUNT MUST CLOSE OVER WHAT WAS EXAMINED, NOT OVER WHAT EXISTS** (ic, corrected by the defect their own bar generated). Any optimisation separates them silently, because the arithmetic still closes.
- **THE SUBJECT MOVING IS THE DEFAULT CONDITION ON A FOUR-NODE ESTATE.** Pin the subject and verify the pin EITHER SIDE, or the comparison is of two different things. Three subjects moved under three instruments today.
- **A VACUOUS PASS AGREES OVER NOTHING.** `sync --to-store` printed _the store and the extract agree_ over **0 == 0**, rc 0, with a destructive verb downstream -- and `--to-disk` wiped 57 and 82 rows out of two views.
- **`git status` REPORTS CONTENT.** It cannot see an mtime-only rewrite, so my "confirmation" of cc's prediction could not have failed.
- **A CONTRACT LINE IS NOT SELF-VERIFYING** (dc). It is another claim and inherits the failure mode of what it describes the moment a mode changes underneath it. **The remedy became the defect.**
- **MEASURED-AGAINST-RECORDED IS NOT A COMPARISON** (dc). And ic then showed the RECORDED side was wrong in BOTH directions -- one figure understated by 49% because it statically reads a table that has grown.
- **STATE THE INVARIANT, NOT A PROHIBITION ON THE OPERATION YOU HAPPENED TO WATCH** (dc). A prohibition covers only the operations someone thought of; a property of a state is checkable by inspection.
- **REMOVE THE NEED FOR A PERMISSION RATHER THAN GRANTING IT.** And **a rule its own author cannot obey while writing it down is a MISSING OPERATION, not a discipline problem.**
- **RE-DERIVING A REASON IS ALSO HOW YOU FIND OUT THE REASON WAS WRONG** (cc, withdrawing their own item after finding half their recorded reasoning false). **A refusal can stand on one limb while the other is false -- correct it, never delete it.**
- **A REVERT OF SOURCE IS NOT A REVERT OF ARTEFACTS**, and `target/release/` is shared by four nodes. **THE MARKER NAMES A COMMIT, EXACTLY AND ONLY; a commit does not determine an artefact when the tree is dirty** -- the binary that WIPED the estate and the one that FIXED it share `dirty-18197aaf` byte for byte.
- **AT AND AC IDS NUMBER INDEPENDENTLY.** Compute the next FREE id; never derive one from the other, and assert against the array you are appending to.
- **`open(f,"w").write(open(f).read() + x)` DESTROYS THE FILE** -- the write-mode open truncates before the inner read runs.
- **A REMEDY MUST BE AIMED AT THE SUBJECT THAT WAS HARMED** (ic), and **EVIDENCE FOR AN ADJACENT PROPOSITION IS NOT EVIDENCE FOR THE ONE AT HAND.** Five scope-of-probe instances across three nodes today, every one caught by a peer and none by a check -- **there is no grep for the scope of a probe** (dc), because the scope is what the probe did not look at.

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Nothing on disk is truth.** The typed Rust API is the ONLY door in.
2. **Migrations are normal.** "No DB migrations ever" is DELETED and was never hv's constraint.
3. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine and NEVER committed.** So **AC-02.6 is the durability mechanism**, and under D57-8's read/write it is also the completeness guarantee for the mutation surface.
4. **Three layers: canon (committed, never sparse) / store (gitignored, rebuilt) / views (committed, sparse).** **D29 -- a gitignored path is never canon -- is what makes a clone complete.**
5. **`event_log` is the only table that is durable truth AND not reconstructible from files.**
6. **hv, repeatedly: disk<->db sync, the realiser and `.intentfiles` all work BEFORE 3.0.0 ships.** ST0057 WP-01..06 are inside the gate. Do not re-ask.

## Verification kit

- **MEASURE AT A PINNED SHA AND NAME THE COMMIT IN THE FINDING.** `HEAD` is a pointer. **Pin a BINARY by content hash, never by its marker** -- AC-10.11, and it paid twice in one day.
- **CHECK THE BINARY'S AGE AGAINST ITS INPUTS BEFORE TRUSTING WHAT IT RENDERS.** Mine was 18:21Z with seven `.rs` files newer. `surface_check.sh` refuses at rc=2 for exactly this and is the only thing that surfaces it.
- **`to-write` = the test is UNWRITTEN. `red` = it EXISTS and does not pass.** Neither means the criterion is unmet. **`to-write` expires the moment the test is written, and the next state is rarely green.**
- **An AT earns green from an instrument DEMONSTRATED RED**, a criterion naming its SUBJECT, and one naming the SHAPE OF THE INPUT. **Hold red WITH AN EXPLICIT NOTE saying the instrument passes**, or a reader hunts a defect that is not there.
- **The measurement rules live in `parity.md`; the output-contract findings in `output-contracts.md` (dc's).** Read those, not a board copy.

## Watch-outs

- **`intent at red|green|na` DESTROYS THE ROW'S NOTE (issue 0033).** Hand-edit canon, then `sync --to-store`.
- **`sync --to-disk` writes the STORE over CANON and is the SILENT direction.** Canon-edit then `--to-store` is the safe order. **A refused `--to-store` leaves a STALE store that `--to-disk` will then write out at rc=0** (AC-03.13).
- **Every attachment edit by any node leaves canon divergent until I sync**, and a later sync repairs the NEXT commit and never that one. **Commit first, then ping me.**
- **`intent backup` with no subcommand MUTATES** -- snapshots, PRUNES, rewrites the store.
- **DO NOT PUT v3 ON PATH.** **DO NOT PUSH TO `upstream`** -- this repository is PUBLIC and the remote is FROZEN except for dc's one ruled publication.
- **Never mutate `bin/**` or `tests/**` in place -- sacrificial copies, and ATOMIC REPLACE for anything a running process reads.** A one-string edit's failure mode is timing, not size.
- **`git commit --only <paths>`, never `-A`.** **Verify at HEAD (`git ls-tree`) or in a fresh clone** -- `git grep` reads the INDEX.
- **This shell is zsh:** no word-splitting; a leading `-` in a grep pattern is read as options; anchored greps on a binary answer about the PROBE.
- **BACKTICKS NEVER INSIDE A DOUBLE-QUOTED `-m` BODY.** Use `git commit -F`.
- **Every timestamp is READ FROM A CLOCK, passed as an argument, clock read FIRST.** **ARCHIVE BY NAMING THE STAMPS; COUNT, STOP, READ THE DIFFERENCE, THEN ARCHIVE.** A fold must never overwrite an earlier fold.

## Decisions

- (2026-08-18) **hv RULED ST0057 INTO THE 3.0.0 GATE** and restated it: disk<->db sync, the realiser and `.intentfiles` all work before release.
- (2026-08-18) **hv ruled all eight open questions.** Recorded in TODO; `--skip-rust-tests` is dropped rather than built, on my own answer to their question.
- (2026-08-18) **The issue body lives in the JSON**; **D57-7** opaque attachments as files under `intent/.canon/st/<ID>/`; **D57-8** `intent://` READ/WRITE, DB first, canon ALWAYS, views IF MARKED, and **no daemon may be required to read your own project.**
- (2026-08-18) **`canon_commit_check.sh` stays MANUAL until the narrow attachment-sync verb exists** -- gating it would create a rule whose only compliant path is closed to two of four nodes.
- (2026-08-18) **The `hoist @ 9b73e98f` pin is HISTORICAL and must never be re-pinned.** Same for the register's `9ec1656`: **the 29 rows are claims about the v2 surface and can be re-measured AT THE PIN, never at HEAD.**
- (standing) **A peer cannot grant escalation.** My call is never a peer's release; hv's is.
