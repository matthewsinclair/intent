---
verblock: "19 Aug 2026:v1.10: vc - ST0057 in the gate, WP-01 code committed, files not moved; contract at 123/124 + 46/46"
intent_version: 2.19.0
---

# Work In Progress

## Current State (as at `58397c5a`, 2026-08-19)

**This heading names a COMMIT, not a date.** A wip file is read as current and written as a snapshot; if you cannot say what it is current as at, that is the finding.

**Two live threads, both inside the 3.0.0 gate.** **ST0056** is the v3.0.0 rewrite -- architecture ratified in `design.md` (D01-D36), contract at **123 criteria / 124 tests** (7 red, 58 to-write, 40 green, 19 n-a). **ST0057** is the disk model -- disk as a sparse projection of the store, D57-1..D57-8 ruled, contract at **46/46** (1 red, 43 to-write, 2 n-a). hv put ST0057 in the gate verbatim: _"all of that has to happen before we do the 3.0.0 release."_

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2.

**ST0056 WPs:** 01/02/04 Done; 03/05/06/10/11 WIP; 07/08/09/12/13/14/15/16 Not Started.
**ST0057 WPs:** 01 WIP; 02-08 Not Started. (**The WP-01 start is made BY this commit, not by `58397c5a`** -- the pin names what was measured, not what this fold changes.)

**ST0057 WP-01 -- THE STATE THAT MATTERS FOR TOMORROW.** cc committed the **code** at `f41d6760`: canon resolves at `intent/.canon/`, workspace 647 passed / 0 failed across 88 suites, fmt clean. **THE 57 + 40 FILES HAVE NOT MOVED AND `intent/.canon/` DOES NOT EXIST.** The live move is the next action and it happens once. Landed with it: AC-01.7's openness declarations (7 moved in `store.rs`, `ddl.sql` re-blessed, drift clean) and `canon_resolver_singularity.rs` for AT-01.6.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds both claims).

## Next Up

1. **cc -- the live move of 57 + 40 files.** AC-01.6 carries the classifier it needs BEFORE it runs: **a half-migration is silent exactly when its unmigrated end can still produce a value.** Sites whose unmigrated end still yields a value need a driven test; the rest announce themselves.
2. **vc -- ping dc and ic the moment the move lands and the tree is green.** That is the of-N adjudication trigger (AC-00.11). Gated on the FILE move, not the code. Order is gatedness, never count.
3. **dc -- Half B** (six declarations, two cost-bearing grep arms, the RED) against **AC-07.4** as elaborated. Also blocked on cc for the 88-binary test consolidation, which changes one spelling for everyone and wants announcing.
4. **ic -- `of_n_labels_its_derivation.sh`** (AT-00.12, red, file exists) and `of_n_closes_over_examined.sh` (AT-00.11, to-write, gated on the move).
5. **vc -- ST0011** (`completed` NULL, AC-08.5's first burning case) and **AC-03.16's fix**, queued to cc as not-now.
6. **v2 carries (default-defer):** credo_checks fleet issues; fleet pushes Utilz `0171297` + Lamplight `7058fd3a8`.

## Recent

- **2026-08-18/19**: ST0057 ruled into the gate, all eight questions answered. Contract rulings into canon: **AC-07.4** (the critic runner's silent skip is the defect, not the default -- 13 of 13 rules were unaskable and the runner returned 0), **AT-00.12** (two drive modes with different populations get two rows), **AC-03.16** (a generated cover naming `acceptance.md` as _the single source of truth_ is a work-loss instruction, 206 of 207 covers), **AC-01.6** extended with the half-migration classifier. **AC-00.10's `18 of 24` RETRACTED -- measured 12 of 45, wrong in both halves.**
- **2026-08-14**: v2.19.0 SHIPPED (tag `071c612`, fifteen issues 0009-0023). ST0056 opened the same afternoon.
- Earlier: `intent/history/`.

## Parked

3.x steel threads, post-3.0.0, each on its own: TUI dashboard; the agent bus; Laksa web page; macOS menubar app; `intent_ex` hex client; sqlite-vec semantic search.
