---
verblock: "14 Aug 2026:v1.09: vc - WP-01 + WP-02 Done; bounce rulings landed; first pushes, CI green; WP-03 next"
intent_version: 2.19.0
---

# Work In Progress

## Current State

**ST0056 -- Intent v3.0.0 -- is the live work; WP-01 and WP-02 are DONE (gates 4/4 and 5/5) and WP-03 (ingest, views, sync) is next, in cc's hands.** The architecture is ratified and recorded in `intent/st/ST0056/design.md` (D01-D21) with the WP-01 specs beside it (`data-model.md`, `migration.md`, `parity.md`) and a 62-AC contract, lint-clean. Roles per hv: **cc and ic write the code; vc stewards** (contract, verification at WP closes, hv interface). The v3 estate is **pushed to both remotes with CI green twice on `736033d`** -- the first rust workflow run (31812129560: macOS+Linux, fmt --check + clippy -D warnings + tests, 1m47s) and the BATS Intent Tests. WP-02 closed on cc's claim after AC-02.6 renumbered to AC-04.5 (the envelope test cannot exist before WP-04's facade verbs). devbin is adopted (`bin/int`; `bin/release` is now `bin/int build release`; `bin/intent` untouched by design; suite 1240 green at `3563ff4`, which surfaced and fixed issue 0025).

**The bounce rulings (hv, 2026-08-14 pm) are all landed**: the `corrected` parity class is RATIFIED (parity.md); the migration carry policy is RULED (migration.md: closed threads lossless-by-carrying, live threads BLOCKED-until-clean, neither ever lossy -- forced by Lamplight's ~1158 permanent legacy rows; the sweep program is dead and WP-10's corpus is the fleet AS IT IS); `organize` (both faces) is planned VESTIGIAL by construction (a strictly structured model cannot hold data in the wrong spot or format -- both implementations retire at the surface cut, dissolving their Highlander); pushes happen when they make sense; v2 maintenance is DEFAULT-DEFER, show-stoppers only. 0024 (scoped `at lint`/`ac gate` dropped the WP scope) closed at `e685e90`, vc-reviewed sound, guard hardened at `8b7d382`.

**ic delivered the parity substrate and a new enforcement gate**: 26 `parity/cmd-*.md` files, the `INTENT_BIN` harness retarget (711/1235 tests reach the CLI), the register regenerated at `393a8e1`, and the **whiteboard clock guard** (`ddac6ba` + `98ce764`): pre-commit now refuses board stamps without a trailing `Z`, stamps postdating their commit, or an inbox going backwards.

## Next Up

1. **WP-03 (cc)**: strict ingest, deterministic views, the sync engine -- migration.md read as landed (the carry policy shapes the sync write path).
2. **Spec the marked-legacy AT form in data-model.md before WP-08** (the carry policy's named model consequence; vc).
3. **ic**: per-test register rows for the 40 `split` files (`corrected` now ratified); the charter + roster-row asks remain open with hv.
4. **v2 carries (default-defer)**: credo_checks fleet issues (hv running); fleet pushes Utilz `0171297` + Lamplight `7058fd3a8` (re-verify still unpushed first); cc's parked "hv decides" queue.
5. The tree carries cc's uncommitted lang-init spread (config languages + per-lang RULES-_/ARCHITECTURE-_) -- cc's lane.

## Recent

- **2026-08-14 (late pm)**: bounce rulings landed; first v3 pushes, CI green twice on `736033d`; AC-02.1 satisfied; WP-02 CLOSED (5/5) with the AC-02.6 renumber; 0024 reviewed sound; clock guard live.
- **2026-08-14 (pm)**: ST0056 begun -- v3.0.0 architecture ratified with hv, Conflab + Lamplight trawled, 12 WPs cut; WP-01 closed 4/4; roles ruling (cc/ic build, vc stewards); devbin adopted; sweep program ruled dead by Lamplight evidence.
- **2026-08-14 (am)**: v2.19.0 SHIPPED (tag `071c612`). Fifteen issues, 0009-0023; release docs written pre-cut for the first time.
- **2026-07-30**: v2.18.0 + v2.17.4 shipped. Earlier: `intent/history/202607-done.md`.

## Parked

- 3.x steel threads (post-v3.0.0, each on its own): TUI dashboard; the agent bus (whiteboard restructure + hv oversight gates); Laksa web page; macOS menubar app; `intent_ex` hex client; sqlite-vec semantic search.
