---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-15 11:57Z
status: active
focus: "D01 REVERSED -- intentdb is the durable SSOT. State machines ratified. Rolled out to all nodes with an ack due on their pickup. 30/97, four WPs open."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Localfold before a compact, mid-session.** `/compact` does not end a session (protocol invariant 6), so status stays `active`. Session detail is in `.history/20260815/wip.md`.
- **Two things are owed to me on every peer's pickup**, both broadcast and both durable in their inboxes: the **db-is-SSOT ack** (model in their own words, what in their lane still assumes the old one, how they checked) and **anything the ratified state machines invalidate**. Chase these before accepting any new work from a node.

## THE MODEL, in case everything else is lost

1. **The intentdb is the durable SSOT. Everything else is a secondary artefact. Nothing on disk is truth.**
2. All of `intentsvcs` works FROM the db; sync runs both ways.
3. The typed Rust API is the ONLY door in, so conformance is **by construction**.
4. Re-creation from an extract is a **capability**, not a licence to treat the db as disposable.
5. Ingest passes the **hard gate** of the intentsvcs API; the gate does the work, not the file format.
6. **Migrations are normal.** "No DB migrations, ever" is DELETED and was never hv's constraint.
7. The real standing requirement is **platform and data-model openness** (AC-02.6): always a 1-1 db-entity-to-`.json`/`.md` mapping, lossless, usable elsewhere. **That is what bidirectional sync is FOR.**

## TODO

- **`wp reopen` does not exist and three of five WPs disagree with their own gate** (WP-02 and WP-04 say `Done` against BLOCKED; WP-05 says `WIP` against PASS). I caused two by adding ACs to closed WPs. Cannot be repaired through the tool until the verb exists -- do NOT hand-edit the status field to paper over it.
- **Open WPs**: 02 (AC-02.6 openness), 03 (AC-03.9 sync directions), 04 (AC-04.1 TornRollback, AC-04.6 graph conformance), 06 (AC-06.1, AC-06.3, AC-06.6).
- **AC-00.1 carries the 28 deferred non-core `pending` rows.** ic's to name, gated here, not forgiven.
- **Issues 0026 and 0027 are filed and are cc's to fix under hv's DEFAULT-DEFER.** 0026 was corrected from my own false high-severity version; read the Correction notice before acting on it.
- **hv rulings not yet built**: devbin moves `bin/.devbin` -> `devbin/` (dc's lane); `intent doctor` flags an unwired hook and `--fix` repairs it (the flag is `--fix`, NOT `--repair` -- it already exists, do not add a second); auth model for WP-08 (local password at first install, Conflab-style cli-to-daemon).
- **WP-10 precondition, from cc**: measure L2/L3 failures per fleet member at its named revision before ruling whether a broken reference in a CLOSED thread carries or blocks.

## Verification kit

- `$CLAUDE_JOB_DIR/tmp/v3fix` is a **migrated** v3 fixture with its own `git init`. Recipe: `config.json` at 3.0.0, `st new`, prose into `thread.json` (**never** a generated view), `sync`.
- **Falsify before flipping.** Perturb the artefact the test asserts against and watch the right subset go red.
- **Refuse at partial coverage, and hold the AT at `to-write` rather than `red`** -- `red` is a false statement about a passing suite. AT-00.5 is the precedent; applied three times today.

## Watch-outs

Measurement rules live in `intent/st/ST0056/parity.md`. What follows is operational to this node.

- **THE CLOCK -- in force at full strength, and on death row.** D33 deletes these rules the moment WP-14's API is the only writer; until then the class is constructible. **Five fabrications now**, the last one _inside the message where I was owning a different error_. Run `date -u +'%Y-%m-%d %H:%MZ'` in its own step and paste it. A stamp 2 minutes out passes the guard (issue 0027).
- **Refusing to settle by inference is NOT a resting state.** It obliges you to go and get the answer. A question parked across three rulings is a decision made by default -- that is how D01 stayed wrong through four hv statements of it.
- **When you inherit a rule with a rationale attached, the rationale is the part most likely to be wrong** -- nobody re-derives it. "No DB migrations, ever" was a consequence I defended as a requirement while the real one sat beside it doing the work.
- **Absence of a mechanism's NAME is not absence of the mechanism.** I grepped for `hooksPath`, found it only in dc's files, and filed a false `high` defect on a public repo. The correct API never needs to name it.
- **A rule true in its own scope is the easiest to over-apply**, because it keeps being true wherever you check it. Check the SHAPE matches the shape the rule was measured on.
- **Verify at HEAD (`git ls-tree`), never on disk -- better, clone fresh and build.** **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.**
- **`git commit --only <paths>`, never `-A`** -- and **a move is TWO facts**: naming only the new paths commits half a move, silently, with a green working tree.
- **THIS REPOSITORY IS PUBLIC** and hv has ratified that the whiteboards are part of the public record, intentionally. The `-A` hazard is therefore a _publication_ hazard into a history nobody can rewrite. Check `git status` for untracked strays before any commit.
- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks here and the BATS suite reads the live tree. Sacrificial worktrees only.
- **Never `head` a list you are counting**; a frequency-sorted list puts the RARE value last. **Read `$?` before anything else touches it** -- but the pipe trap is the pipe trap; `A || echo "$?"` reports A correctly.
- **This shell is zsh**: no word-splitting of unquoted parameters.
- **The live channel does not survive a peer's restart; the inbox does.** Durable copy first, live ping as accelerant.

## Decisions

Archived once a committed artefact carries them -- see `.history/`.

- (2026-08-15) **The number going DOWN is the contract working.** 31/94 -> 30/97 after the reversal, because three WPs were resting on sentences it invalidated. A rising satisfied-count during a spec change is the thing to distrust.
- (2026-08-15) **Correct in place with a named Correction notice; never a quiet edit.** An issue that silently becomes a different issue is the same class of lie as the ones this board exists to catch.
- (2026-08-15) **A control refuses; documentation reminds; only one is load-bearing** (cc). Treat a rule you can obey only by concentrating as an unfixed defect.
- (2026-08-15) **Necessary is not sufficient.** Exercise the mechanism; presence of a mechanism is not evidence it works.
- (2026-08-15) **File a defect under its own noun, even when that reopens a closed WP.** Done twice today, once on a PASSING WP.
- (2026-08-15) **Report what you MEASURED, never what you INTENDED** (ic and dc, independently). A designed coverage figure rots; a measured one cannot.
- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.**
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.**
- (2026-08-14) **hv standing authorisation is not review, and does not reach a ratified decision.**
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc holds the ST0056 claim as steward and does not build.
- (2026-07-02) vc fires on a close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv.
