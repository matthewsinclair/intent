---
verblock: "20 Aug 2026:v1.12: vc - globalfold; gate 62/67 computed not typed, D57-8 stopped contradicting itself"
intent_version: 2.19.0
---

# Work In Progress

## Current State (as at `69a5db5e`, 2026-08-20)

**This heading names a COMMIT, not a date.** A wip file is read as current and written as a snapshot; if you cannot say what it is current as at, that is the finding.

**THE GATE IS 62 OF 67, AND IT IS COMPUTED RATHER THAN TYPED.** `intent ac status <STID>` prints `N/M satisfied, K withdrawn` and is the authority. **The previous two figures in this file were hand-tallied and both were wrong** -- 50-of-64 was stale by a day, and a later 57-of-67 counted "live" by two different rules in its two halves (ST0057's denominator excluded withdrawn rows, ST0056's counted one). **Do not re-derive this number by hand. Run the verb.**

**THE DISK MODEL IS RUNNING, NOT DESIGNED.** `intent organize --apply` removed 423 files at `e7f00e65`; `intent/st/` holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database. **Proven reversible by measurement**: ST0001 rehydrated to five files byte-identical to git, a fence-heavy pair to fifteen, and all 282 attachments verify against their own `sha256`.

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2. **DO NOT PUT v3 ON PATH** -- the pre-commit gate works _because_ it runs v2, whose version guard is scoped to writes. On PATH, `intent critic` answers 2 in all five declared languages, which is the code the gate fails open on, here and in the other 15 Intent projects on this machine.

### The two threads

**ST0056 -- the v3.0.0 rewrite.** Architecture in `design.md`. **The intentdb is the DURABLE SSOT; nothing on disk is truth.** D01 was REVERSED by hv 2026-08-15 -- do not reason from it. **132 criteria / 134 tests, 56 of 131 satisfied, 1 withdrawn.** WPs 01/02 Done; 03/04/05/06/07/10/11 WIP; 08/09/12-16 Not Started.

**ST0057 -- disk as a sparse projection.** **Sparseness applies to VIEWS; canon is NEVER sparse.** **53 criteria / 53 tests, 47 of 51 satisfied, 2 withdrawn.** WPs 02/04/06/07/09/10 Done; 01/03/05/08 WIP.

### THE GATE: 62 OF 67, AND THE FIVE THAT ARE LEFT

All of ST0057's live rows (47/51) plus all of ST0056 WP-03's (15/16). **Outstanding, with owners:**

| row      | thread       | owner | why it is not green                                           |
| -------- | ------------ | ----- | ------------------------------------------------------------- |
| AC-01.5  | ST0057       | cc    | red                                                           |
| AC-03.6  | ST0057       | cc    | red                                                           |
| AC-03.14 | ST0056 WP-03 | cc    | AT-03.15 red                                                  |
| AC-07.7  | ST0057       | ic    | **newly minted 2026-08-20, unbuilt**                          |
| AC-08.5  | ST0057       | ic    | red -- the pin is a measurement and its measured set is empty |

**dc holds none of the five.**

### THE ARCHITECTURE hv RULED, replacing the two-region manifest design

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds an id, `st done` removes it, a human may edit it. **Nothing derives it from status.** **ABSENT IS NOT EMPTY** -- a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

## What changed on 2026-08-20

**D57-8 STOPPED CONTRADICTING ITSELF** (`c5320329`). Its fenced list enumerated nine forms, every one an entity; its READ/WRITE clause required three COLLECTION addresses by name; its under-addressing clause wrote out a fourth in full. Four collection forms were mandated in prose, implemented in `address.rs`, and absent from the only place a reader -- or a test -- goes to enumerate the grammar. **The cost was paid by the node who sourced it correctly**: `d57_8_forms()` was built from the DESIGN rather than from `address.rs`, on the sound ground that a denominator read out of the implementation agrees with it by construction, and it came back four short. **AC-07.1 is NOT reopened** -- its population is _every ENTITY form_ and against nine it is faithful. **AC-07.7 / AT-07.7 minted** for collection resolution.

**FOUR ROWS GREENED AND ONE CRITERION ATTESTED BY EVIDENCE** (`8d20dc49`), driven at `28b3610b` in a clean detached worktree: 140 targets, 985 passed, 0 failed. **ST0057/WP-04 closed on 7/7 verified rather than accepted.** WP-06, WP-09 and WP-10 also closed this cycle.

**AT-11.6's RE-CITATION WITHDRAWN AND THE DELIVERABLE RESTORED.** `prepush` clones HEAD to a temp dir and builds there -- it never touches the shared release path, refuses nobody, names no paths. AC-11.6 weighed both mitigations and ruled FOR refusal, and the row had been re-cited onto the one it declined. Separately, `bin/int` is a symlink to the dispatcher and contains no `prepush` at all.

## Next

1. **cc** -- AC-01.5, AC-03.6, AC-03.14; AC-10.4 built over `migrate::plan`'s write set with a non-empty control; AT-10.2's second citation onto `intent-cli/tests/ingest_command.rs`; AT-10.12 still held on the unexplained trim asymmetry.
2. **ic** -- AC-08.5; AT-07.7, whose red-first arm **must be `AcCollection` specifically**, because the other three come from the POST clause and any test sourced from that paragraph reaches them.
3. **dc** -- holds none of the gate. AT-11.6's deliverable is theirs and stays unbuilt.
4. **vc** -- `declared_but_unwired` adequacy; the heartbeat-currency note for hv; the eleven-copies filing (one v2-estate builder now in `common/mod.rs`, nine pre-existing named and left).
5. **Everyone, hv's standing question:** 250 files under `intent/` are not in the store at all -- `docs/`, `llm/`, `history/`, `eng/`, `plugins/`, and the project-level `done.md` / `wip.md` / `restart.md` / `todo.md`. hv: _not all of that should be in the db, but certainly some of it should._
