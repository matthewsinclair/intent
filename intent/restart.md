# Claude Code Session Restart -- narrative state

## Current state (as at `58397c5a`, 2026-08-19)

**This heading names a COMMIT, not just a date, and that is deliberate.** A restart file is read as CURRENT STATE and written as a snapshot of when its author typed; nothing used to mark which, and a cold session treated a four-day-old line as the next action. Anything below is true of `58397c5a` and of nothing else -- **re-stamp it when you fold, and if you cannot say what it is current as at, that is the finding.**

**TWO LIVE THREADS, BOTH INSIDE THE 3.0.0 GATE.**

**ST0056 -- Intent v3.0.0.** Architecture ratified in `intent/st/ST0056/design.md` (D01-D36); read it before touching anything v3. The shape: schema-as-truth (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, committed and drift-checked); **the intentdb as the DURABLE SSOT with everything on disk a secondary artefact** -- **D01 was REVERSED by hv on 2026-08-15 and the old wording ("committed JSON as durable truth, rebuildable SQLite, `rm intent.db` always safe, no DB migrations ever") is FALSE IN EVERY CLAUSE; do not reason from it**; the committed extract as the INTERCHANGE that travels while the DB never leaves the machine (D34); migrations NORMAL; `rm intent.db` ruled out of existence (D36); a 1-1 lossless db-entity-to-file mapping as the standing openness requirement (AC-02.6), which is what bidirectional sync is FOR; markdown demoted to generated views + authored prose; strict validate-or-refuse ingest; `intentsvcs` as sole owner of DB and file canon; CLI dual-mode (in-process facade, or GraphQL to one machine-level intentd); MCP as the primary agent write surface; migration floored at v2.19.0; intentd IN the gate. Prior art: Lamplight `native/cli`, Conflab `native/daemon`.

**ST0057 -- disk as a sparse projection of the store.** D57-1..D57-8 ruled; all eight open questions answered 2026-08-18. **Sparseness applies to VIEWS; canon is NEVER sparse** -- if the manifest governed canon, an unrealised artefact would exist only inside a gitignored database. **D29, a gitignored path is never canon, is what makes a clone complete.** hv put the thread in the gate verbatim: _"I need to be able to sit in a project, work with disk versions of the relevant artefacts, and have the db kept in sync as things change. So all of that has to happen before we do the 3.0.0 release."_

**INTENT IS SELF-HOSTED ON v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2.

**Contract, measured at this commit.** ST0056: **123 criteria, 124 tests** -- 7 red, 58 to-write, 40 green, 19 n-a (reconciles to 124). ST0057: **46 and 46** -- 1 red, 43 to-write, 2 n-a. **ST0056 WPs** 01/02/04 Done, 03/05/06/10/11 WIP, rest Not Started. **ST0057 WPs** 01 WIP, 02-08 Not Started -- **the WP-01 start is made BY this commit, not by the pin.**

**ST0057 WP-01 IS THE STATE THAT GOVERNS TOMORROW.** cc committed the CODE at `f41d6760` -- canon resolves at `intent/.canon/`, workspace 647 passed / 0 failed across 88 suites. **THE 57 + 40 FILES HAVE NOT MOVED AND `intent/.canon/` DOES NOT EXIST.** The live move is next and **it happens once**. AC-01.6 carries the classifier for it: **a half-migration is silent exactly when its unmigrated end can still produce a value** -- a missing directory is not a plausible wrong answer, a wrongly-derived filename is.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface).

## Next (as at `58397c5a`)

1. **cc -- the live move of 57 + 40 files**, then the 88-binary test consolidation with dc (it changes one spelling for everyone: `--test <name>` becomes `--test suite <name>`, including the `INTENT_BLESS=1` re-pin, so it wants announcing rather than discovering).
2. **vc -- ping dc and ic the moment the move lands and the tree is green.** The of-N adjudication trigger (AC-00.11). **Gated on the FILE move, not the code.** Order is gatedness, never count; both trees as worktrees at named revisions.
3. **dc -- Half B** against AC-07.4 as elaborated: six declarations, two cost-bearing grep arms, and the RED. Measured start: **13 of 13 shell and rust rules carry no proxy and no declaration, and `critic_runner.sh` skips a proxy-less rule silently, so `critic shell` and `critic rust` return 0 because nothing was ever asked.**
4. **ic -- `of_n_labels_its_derivation.sh`** (AT-00.12, red, file exists at 229 lines) and `of_n_closes_over_examined.sh` (AT-00.11, to-write, gated on the move). **AT-00.11 must run against a confirmed positive before any clean result is banked.**
5. **vc -- ST0011** (`completed` NULL; `intent doctor` names it), and **AC-03.16's fix** queued to cc as not-now.
6. **Standing:** `organize` (both faces) planned vestigial by construction; v2 maintenance DEFAULT-DEFER, show-stoppers only.

## Standing lessons (carry forward)

- **A LESSON WRITTEN DOWN IS NOT A MECHANISM.** This file already said _a line number in a durable record is a fact with an expiry date_ -- and AC-00.10 still shipped four rotted line numbers, found by a peer. **Thinking hard about a class does not protect you from it; the protection was the instrument, and only the instrument.**
- **A CITATION IS A FIGURE** -- it travels with a subject and a revision. **Name the revision or name the text, never the position.**
- **RUN THE SWEEP AGAINST THE CONFIRMED POSITIVES BEFORE BANKING A CLEAN RESULT.** A test that would not have caught the instance you already know about says nothing about the rest.
- **AGREEMENT: same observation, opposite value.** The discriminator is _could each instrument have produced the other's finding?_ No -- complementary coverage. Yes -- one method used twice, evidence of nothing.
- **AN INDEX THAT RESOLVES AN AMBIGUOUS NAME TO AN ARBITRARY MEMBER FABRICATES CONFIDENTLY AND SILENTLY** -- not under-reach; a specific, actionable, wrong number.
- **Mutation-test every guard**, and the canary must come from the same fixture and branch the test drives. Twice in one day a test written for a bug did not catch it, both found by mutating and neither by reading.
- **A migrator must not do half of a two-ended migration.** Refuse and name beats guessing.
- **Diagnose by running, not reading; run the real path in a sacrificial copy.** Never mutate `bin/**` in place -- and **a rig assembled by SYMLINKING into the real tree is not isolated: `cp` follows the symlink and writes through to production.**
- **A measured figure names its subject and revision**, or it is a rumour with a decimal point.
- **Read the clock, then PASTE -- never read, then type.** A stamp in the past clears the guard's future-check and its `Z`-check both.

## Release checklist (carry forward)

1. Tree clean, suite green. **Clean means ALL of it** -- the release aborts on anything dirty outside its five sidecars, including another node's whiteboard board.
2. **Write the release docs BEFORE the cut** (`intent/history/<v>.md` + `docs/releases/<v>/RELEASE_NOTES.md`) so the tag carries them. Adopted at v2.19.0.
3. `bin/int build release --minor` -- interactive, **NEVER `--no-confirm`**. Pre-flight re-runs doctor + the full suite, stamps five sidecars, dates the CHANGELOG, tags, pushes.
   3a. If it aborts after the sidecar commit, the documented recovery is `--skip-tests` -- which skips the ONE gate certifying HEAD. **A recorded green is cheap while redundant and expensive at the single moment it matters.**
4. Post-cut: flip `intent/done.md`, verify sidecars/tag/release body, globalfold.

## Where detail lives

- **ST0056**: `design.md` (D01-D36), `tasks.md`, `acceptance.md` (generated view -- **canon is `thread.json`**), `parity.md` (measurement rules), `output-contracts.md`, `critic-gate.md`, WP info files.
- **ST0057**: `design.md` (D57-1..D57-8), `data-model.md`, `migration.md`.
- **v2.19.0**: `intent/history/v2.19.0.md`; CHANGELOG `[2.19.0]`; `docs/releases/2.19.0/`.
- **Whiteboard**: `intent/whiteboard/<node>/wip.md` live; `.history/<YYYYMMDD>/` folded.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when matts asks; **always `git commit --only <paths>`** (a bare commit sweeps a peer's staged index); whiteboard stamps carry a trailing `Z` read from `date -u`; matts runs the full suite externally and is the acceptance verifier; **DO NOT PUT v3 ON PATH; DO NOT PUSH TO `upstream`** (public, frozen).
