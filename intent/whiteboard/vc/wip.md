---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-15 13:02Z
status: active
focus: "D34 (extract is the interchange) and D35 (rolling backup, never a file copy) RULED by hv on measurement, contracted at AC-03.10/AC-08.8, announced to all nodes. 30/99."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Localfold before a compact, mid-session.** `/compact` does not end a session (protocol invariant 6), so status stays `active`. Session detail is in `.history/20260815/wip.md`.
- ~~**Two things are owed to me on every peer's pickup.**~~ **DISCHARGED 2026-08-15 12:23Z. All three acked with method, and every one of them found something in its own lane it had previously reported CLEAN** -- cc nine sites after reporting sixteen fixed, dc a false claim in `.gitignore` after auditing the lane clean, ic three user-facing help strings its first structured pass could not reach. The ack mechanism earned itself; do not let the next broadcast go out without one.
- ~~**BLOCKED ON hv -- D21 gitignores the SSOT.**~~ **RULED 2026-08-15 as D34 + D35, on measurement, because hv required the size question be grounded BEFORE it was answered.** The extract is the interchange; the DB is per-machine truth, never committed; a rolling local backup covers what that costs. Contracted at **AC-03.10** (backup) and **AC-08.8** (daemon+CLI identity), 97 rows -> 99. Announced with the numbers attached at `503e0b9`.
- **The numbers, so I never re-derive them.** FTS5 expansion is LINEAR: Intent 1.97x, Lamplight 1.95x. GitHub hard-blocks at 100 MB; Lamplight's markdown-only DB is already 82.49 MB and its whole-corpus projection is ~163 MB. **Git deltas SQLite WELL** -- 82 MB packs to 29.5 MiB, a commit costs 219 KiB -- so **cite the CEILING, never the dirtiness**; we had a correct conclusion resting on a reason that does not hold. `dbstat`: 98.6% of the DB is `doc_sections_*`, and 69.5% is a verbatim second copy of text already on disk (issue 0029, 64% saving, cc's call).
- **Two things promoted from detail to precondition by D34**: AC-02.6 IS the durability mechanism (a field that does not round-trip is data loss, not a gap), and **`event_log` is the only table that is both durable truth AND not reconstructible from the files**, so `events.jsonl` gates the truth model.

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
- **Issues 0026, 0027 and 0028 are filed under hv's DEFAULT-DEFER.** 0026 was corrected from my own false high-severity version; read the Correction notice before acting on it. 0028 is a one-sentence addition to the shipped whiteboard SKILL.md and touches every node's commit habit.
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
- **`git commit --only <paths>`, never `-A`** -- and **a move is TWO facts**: naming only the new paths commits half a move, silently, with a green working tree. **`--only` also never CLEARS the index** (measured, issue 0028): a stale entry survives every subsequent commit, is invisible to `git diff HEAD`, and shows only as the left `M` of `MM` in `git status --short`. Read the short status, not the diff.
- **THIS REPOSITORY IS PUBLIC** and hv has ratified that the whiteboards are part of the public record, intentionally. The `-A` hazard is therefore a _publication_ hazard into a history nobody can rewrite. Check `git status` for untracked strays before any commit.
- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks here and the BATS suite reads the live tree. Sacrificial worktrees only.
- **Never `head` a list you are counting**; a frequency-sorted list puts the RARE value last. **Read `$?` before anything else touches it** -- but the pipe trap is the pipe trap; `A || echo "$?"` reports A correctly.
- **This shell is zsh**: no word-splitting of unquoted parameters.
- **The live channel does not survive a peer's restart; the inbox does.** Durable copy first, live ping as accelerant.

## Decisions

Archived once a committed artefact carries them -- see `.history/`.

- (2026-08-15) **GROUND THE NUMBER BEFORE RULING ON IT -- hv's instruction, and it changed the answer's REASONING.** Everyone (me included) assumed binary-merge dirtiness would kill DB-in-git. Measured, it does not: git deltas SQLite well. The ceiling kills it instead. **A correct conclusion resting on a false reason is the same defect as a wrong conclusion** -- it just fails later, when someone leans on the reason. Two instances in one day: D29's void derivation, and this.
- (2026-08-15) **A backup that fails silently is worse than one that fails loudly, and the plausible implementation is the silent one.** `cp` of a WAL-mode SQLite DB captured 0 rows against a live 50 -- and OPENED CLEANLY. **My first attempt to demonstrate it FAILED to reproduce it**, because the probe read the DB first and a lone clean close checkpoints the WAL. So the hand-check passes and production fails. **When a test cannot reproduce a hazard, suspect the test's setup before concluding the hazard is not real.**
- (2026-08-15) **Three nodes reaching one question from three unrelated entry points is the strongest evidence available that the gap is real.** cc from `rm intent.db`, dc from an ignore-rule premise that inverted underneath it, ic from two people on one project -- all landing on D21's transport question. One node asking is a node's confusion; three converging is a hole in the design.
- (2026-08-15) **A board field is a claim about a session's last fold, not a fact about the present.** I inferred authorship of a live edit from `status: active` and was wrong -- the editing node was `paused`. We call heartbeats advisory and I then used one as evidence.
- (2026-08-15) **A clean result is only as current as the needle set that produced it, and a needle set has a timestamp** (dc's, better than my "absence of the NAME is not absence of the mechanism" because it explains rather than names). Corollary from ic: **a structured query is a needle like any other and reports on the subtree it TRAVERSED** -- and it feels exhaustive in a way a grep never does, so a clean structured result is trusted harder than it has earned.
- (2026-08-15) **`corrected` PRESERVES a ratified guard; "aspirational" reinterprets it.** When a ratified requirement and a measured v2 behaviour disagree, the requirement governs v3 and the gap is a parity class -- never a reason to widen the guard to fit what the code already does, which is invisible because the result is green.
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
