---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-15 13:25Z
status: active
focus: "HOLD LIFTED, all three building; ic already closed six items and unblocked cc. vc trigger-driven again -- fire on a WP close, a green, or hv. cc is the critical path on all four open WPs. 30/99."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Localfold before a compact, mid-session.** `/compact` does not end a session (protocol invariant 6), so status stays `active`. Session arc is in `.history/20260815/wip.md`; all four inboxes archived and reset to `_(empty)_` at this fold, 97KB of live board down to 13KB.
- **vc is TRIGGER-DRIVEN again.** Fire on a WP close, a green, or an hv request. **Not on in-flight edits, and do not chase.** All three nodes were released to build at 13:14Z (`aea188f`) with per-node ordered lists.
- **The two cross-node blocks are the only things one node can stall another on**, and both are naming decisions rather than work: **dc names the `.backup/` namespace** (collides with `intent upgrade`'s `backup-<TIMESTAMP>/`, different retention) and **ic names the `intent config` backup keys**. cc was told explicitly not to invent either. **If cc's AC-03.10 is stalled, look here first** -- it will look like nothing.
- **cc is the critical path on ALL FOUR open WPs** (02 AC-02.6 | 03 AC-03.9 + AC-03.10 | 04 AC-04.1 + AC-04.6 | 06 three). Nothing ic or dc does can unblock a gate. **A WP that slips, slips here, and the boards will not show it.**

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Everything else is a secondary artefact. Nothing on disk is truth.**
2. All of `intentsvcs` works FROM the db; sync runs both ways.
3. The typed Rust API is the ONLY door in, so conformance is **by construction**.
4. Re-creation from an extract is a **capability**, not a licence to treat the db as disposable.
5. Ingest passes the **hard gate** of the intentsvcs API; the gate does the work, not the file format.
6. **Migrations are normal.** "No DB migrations, ever" is DELETED and was never hv's constraint.
7. The real standing requirement is **platform and data-model openness** (AC-02.6): always a 1-1 db-entity-to-`.json`/`.md` mapping, lossless, usable elsewhere. **That is what bidirectional sync is FOR.**
8. **(D34) The committed extract is the INTERCHANGE; the DB is per-machine truth and is NEVER committed.** Authority is not bidirectional just because transport is. **So AC-02.6 is the durability mechanism** -- a field that does not round-trip is data loss at the clone boundary, not a gap -- and **`event_log` is the only table that is both durable truth AND not reconstructible from the files**, which makes `events.jsonl` a precondition of the model rather than a WP-04 detail.

## TODO

- **`wp reopen` does not exist and three of five WPs disagree with their own gate** (WP-02 and WP-04 say `Done` against BLOCKED; WP-05 says `WIP` against PASS). I caused two by adding ACs to closed WPs. Cannot be repaired through the tool until the verb exists -- do NOT hand-edit the status field to paper over it.
- **AC-00.1 carries the 28 deferred non-core `pending` rows.** ic's to name, gated here, not forgiven.
- **Issues 0026-0029 under hv's DEFAULT-DEFER, none gating.** 0026 was corrected from my own false `high` version -- read the Correction notice first. 0028 is one sentence into the shipped whiteboard `SKILL.md` and touches every node's commit habit. **0029 is a DECISION before it is a fix, and cc must check AC-03.6 first** -- that AC may be green through the copy 0029 proposes to delete.
- **hv rulings not yet built**: devbin `bin/.devbin` -> `devbin/` (dc); `intent doctor` flags an unwired hook and **`--fix`** repairs it (**NOT `--repair`** -- `--fix` already exists, do not add a second); auth model for WP-08 (local password at first install, Conflab-style cli-to-daemon).
- **WP-10 precondition, from cc**: measure L2/L3 failures per fleet member at its named revision before ruling whether a broken reference in a CLOSED thread carries or blocks.

## Verification kit

- `$CLAUDE_JOB_DIR/tmp/v3fix` is a **migrated** v3 fixture with its own `git init`. Recipe: `config.json` at 3.0.0, `st new`, prose into `thread.json` (**never** a generated view), `sync`.
- **Falsify before flipping.** Perturb the artefact the test asserts against and watch the right subset go red.
- **Refuse at partial coverage, and hold the AT at `to-write` rather than `red`** -- `red` is a false statement about a passing suite. AT-00.5 is the precedent.
- **The D34/D35 numbers are in canon, not here** -- `design.md` D34 carries the FTS ratios, the GitHub ceiling, the pack sizes and the `dbstat` split; D35 carries the WAL measurement and the rejected `.sql` dump. **Cite them; do not re-derive them.**

## Watch-outs

Measurement rules live in `intent/st/ST0056/parity.md`. What follows is operational to this node.

- **THE CLOCK -- in force at full strength, and on death row.** D33 deletes these rules the moment WP-14's API is the only writer; until then the class is constructible. **SIX fabrications.** Run `date -u +'%Y-%m-%d %H:%MZ'` **in its own step** and paste it. A stamp 2 minutes out passes the guard (issue 0027). **`stat` and `git log` both print LOCAL time** -- reading one and appending a `Z` gives a stamp wrong by exactly the offset and looking perfect.
- **The sixth fabrication has a mechanism, and it is the useful part: I BATCHED `date` into the same command as the write.** The heredoc was composed before the command ran, so the stamp was generated and the clock read in the same breath -- and the generated one was a minute ahead. **Batching the clock read with the write looks exactly like compliance and defeats the rule entirely.** That is why "its own step" is the wording. **Correcting it to the reading I actually took is legitimate; inventing a plausible replacement is not** -- the ban is on invention, not on using a measurement you hold. Declare the correction in the entry.
- **Refusing to settle by inference is NOT a resting state.** It obliges you to go and get the answer. A question parked across three rulings is a decision made by default -- that is how D01 stayed wrong through four hv statements of it.
- **When you inherit a rule with a rationale attached, the rationale is the part most likely to be wrong** -- nobody re-derives it. Twice measured now: "no DB migrations ever" was a consequence defended as a requirement, and the binary-dirtiness case against committing the DB was true-sounding and not the reason.
- **Absence of a mechanism's NAME is not absence of the mechanism -- and it is not presence of it either.** I grepped `hooksPath` and filed a false `high`; I grepped `from: &[]` against a positional constructor, got zero, and nearly filed a refutation of a real finding.
- **A clean result is only as current as the needle set that produced it, and a needle set has a timestamp** (dc). **A structured query is a needle like any other and reports on the subtree it TRAVERSED** (ic) -- and it feels exhaustive in a way a grep never does, so it is trusted harder than it has earned.
- **A rule true in its own scope is the easiest to over-apply.** Check the SHAPE matches the shape the rule was measured on.
- **Verify at HEAD (`git ls-tree`), never on disk -- better, clone fresh and build.** **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.**
- **`git commit --only <paths>`, never `-A`** -- and **a move is TWO facts**: naming only the new paths commits half a move, silently, with a green working tree. **`--only` also never CLEARS the index** (measured, issue 0028): a stale entry survives every later commit, is invisible to `git diff HEAD`, and shows only as the left `M` of `MM`. **Read `git status --short`, not the diff.** When peers are live, scope any `git reset` to your own paths.
- **THIS REPOSITORY IS PUBLIC** and hv has ratified the whiteboards as part of the public record, intentionally. The `-A` hazard is therefore a _publication_ hazard into a history nobody can rewrite.
- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks here and the BATS suite reads the live tree. Sacrificial worktrees only.
- **Never `head` a list you are counting**; a frequency-sorted list puts the RARE value last. **Read `$?` before anything else touches it** -- but the pipe trap is the pipe trap; `A || echo "$?"` reports A correctly.
- **This shell is zsh**: no word-splitting of unquoted parameters.
- **The live channel does not survive a peer's restart; the inbox does.** Durable copy first, live ping as accelerant.

## Decisions

Archived once a committed artefact carries them -- see `.history/`.

- (2026-08-15) **A one-sided test against a failure event cannot see an event that never happened.** AC-03.10 said "a failed backup surfaces" and a schedule that never fires produces no failure to report -- so the clause written to prevent the nothing-is-wrong / nothing-ran ambiguity contained it. **Reach for the two-sided test**: staleness against a schedule, like the clock guard's check C comparing two stamps to each other and needing no clock. Found by ic asking for a read path I had not specified.
- (2026-08-15) **A hazard reachable through SUPPORTED CONFIGURATION is worse than one reachable by misuse** (ic). Hence the fixed snapshot directory, and no key that silences a backup failure -- a switch to turn a warning off manufactures the silent failure and gives it a supported name.
- (2026-08-15) **GROUND THE NUMBER BEFORE RULING ON IT.** hv's instruction, and it changed the ruling's REASONING rather than confirming it. Everyone assumed binary dirtiness would kill DB-in-git; measured, git deltas SQLite well and the size ceiling decides it. **A correct conclusion resting on a false reason is the same defect as a wrong conclusion -- it just fails later, when someone leans on the reason.**
- (2026-08-15) **When a test cannot reproduce a hazard, suspect the test's SETUP before concluding the hazard is not real.** The `cp`-of-WAL probe looked clean until I noticed it read the DB first, which checkpoints. The corollary is worse than the lesson: a `cp`-based backup passes every hand-check and fails only under production concurrency.
- (2026-08-15) **Three nodes reaching one question from three unrelated entry points is the strongest evidence available that the gap is real.** One node asking is a node's confusion; three converging is a hole in the design.
- (2026-08-15) **A board field is a claim about a session's last fold, not a fact about the present.** I inferred authorship of a live edit from `status: active` and was wrong -- the editing node was `paused`.
- (2026-08-15) **`corrected` PRESERVES a ratified guard; "aspirational" reinterprets it.** When a ratified requirement and a measured v2 behaviour disagree, the requirement governs v3 and the gap is a parity class -- never a reason to widen the guard to fit what the code already does, which is invisible because the result is green.
- (2026-08-15) **The satisfied-count going DOWN during a spec change is the contract working.** A rising count while the spec moves is the thing to distrust.
- (2026-08-15) **Correct in place with a named Correction notice; never a quiet edit.**
- (2026-08-15) **A control refuses; documentation reminds; only one is load-bearing** (cc). Treat a rule you can obey only by concentrating as an unfixed defect.
- (2026-08-15) **File a defect under its own noun, even when that reopens a closed WP.**
- (2026-08-15) **Report what you MEASURED, never what you INTENDED** (ic and dc, independently).
- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.**
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.**
- (2026-08-14) **hv standing authorisation is not review, and does not reach a ratified decision.**
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc holds the ST0056 claim as steward and does not build.
- (2026-07-02) vc fires on a close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv.
