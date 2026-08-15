---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 08:27Z
status: active
focus: "AC-04.6 FIRST -- hv's mutation-completeness ruling reopened WP-04 from PASS, and the missing inverse is costing vc hand-edits right now. Then the marked-legacy scope field, export (AC-06.6), and the surface tail (AC-06.1)."
claims: []
---

# Control Claude (cc)

## DOING

- **Estate green and pushed to BOTH remotes.** 234 tests, fmt and clippy clean. Gates 01, 02, 03, 05 PASS. **TWO blocked and both are mine**: WP-04 reopened at 5/6 by hv's new AC-04.6, and WP-06 at 4/7 (AC-06.1 and AC-06.6 mine; AC-06.3 is vc's and ic's).
- The overnight session's detail, watch-outs and lessons are in `.history/20260815/`.

## TODO -- in this order

1. **AC-04.6 -- MUTATION COMPLETENESS (D32). Do this first.** hv ruled it this morning and it reopened WP-04 from PASS, so it is a gate regression rather than new scope -- and the defect is costing vc hand-edits to `acceptance.md`, the file the CLI exists to own, right now. **Every state an entity can enter, it can leave, by a service call reachable from every surface.** The instance: `intent ac satisfy` is a one-way door, and `rescope`/`reinstate` only undo a descope, so a verifier whose evidence proves incomplete has no way back. **Held to a MECHANICAL test, not a review** (`AT-04.6` = `crates/intentsvcs/tests/mutation_completeness.rs`, to-write): enumerate each modelled state field, assert its transition set is CLOSED, and fail naming the state and the missing inverse. The discriminating case is satisfy-then-unsatisfy. The rule also binds D30's whiteboard operations and anything else modelling state, so build the enumeration to be general rather than special-casing `ac`.
2. **The marked-legacy `scope` field.** Shape is DECIDED, so this is a build rather than a decision: keep `scope` a **unit-only, non-optional** enum and carry the out-of-enum spelling in a **sibling optional field**. Both halves matter -- unit-only because `TShirt` derives async-graphql's `Enum` and a data-carrying variant breaks the SDL face; non-optional because `Option<TShirt>` would make it nullable for all 129 well-formed work packages and admit an invalid both-none state. Requirement to hold: **the value is neither guessed nor dropped**. Driven by `Medium-Large` (1 of 129, at `intent/st/COMPLETED/ST0020/WP/09/info.md`), which maps to nothing in `XS · S · M · L · XL · XXL` and sits in a CLOSED thread where hv's policy is lossless-by-carrying. Ruling + measurement: `data-model.md:83-89`. Touches the model, all three faces, the store DDL, ingest and the renderer.
3. **AC-06.6 -- `intent export --format <fmt>`.** D03's mechanism: "YAML/md/anything else are `intent export --format` projections", which is how v3 refuses YAML canon without refusing YAML users. **Round-trip -- what it emits re-ingests to byte-identical canon -- OR refuse the format BY NAME rather than emitting lossily.** Settle first: whether `md` can round-trip at all, or must be refused by name despite D03 naming it.
4. **AC-06.1 -- the surface tail.** `st edit`, `st repair`, `st zero`; the `issues` and `todo` families; `info`, `version`, `config`, `init`, `bootstrap`; then `claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, **`fileindex`**. The AC's own clause: **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable by construction. Also: `bin/intent_st:1231` is `[0-9]+)` and `+` is literal in a `case` glob, so only the 4-digit form of `st repair` has ever worked -- a forced fix, unconstructible in clap.

## hv rulings -- 2026-08-15 morning

- **`treeindex` RETIRES**, and so does **handover**. The source tree index in the DB obviates `treeindex`; the DB model obviates handover entirely. **State moves out of per-session `.md`s shared between workstreams and into durable state in the intentdb** -- the same direction D30/WP-14 takes the whiteboard. ic landed the dispatch row as `disposition: retire` at `0434223`, took 762 lines of bash off the port list, and its open `pending-hv` question (INV-07, `--help` exits non-zero) is moot -- there is no v3 command left to correct.
- **A RETIRED COMMAND IS PRESENT AND REFUSING, not absent** (my ruling, ic asked). `intent treeindex` exits 1 naming the retirement and its replacement -- not a shim, because it carries no functionality, and AC-04.4 says an error names its cause. Scoped to the v3 line, which is the migration boundary. **The build consequence is mine and it breaks a guard**: `dispatch::is_shipped()` excludes `retire` rows, so a retired-but-refusing command is absent from `shipped_entries()` and present in the spine, and `dispatch_ssot.rs` asserts both directions. Fix the guard with the feature; do not work around it at the table end. ic landed the row at `0434223`.
- **`fileindex` is NOT covered by that ruling and I have deliberately not extended it.** Different mechanism -- "maintain checkbox file indexes", not directory summaries -- so its `pending-hv` INV-07 question stands and it stays in the port list above.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

Everything amounting to "remember to" is archived. It failed twice last night on entries this board already carried, one of them three lines from where I was looking, so that section is evidence for WP-14 rather than a defence. What remains are facts about the estate, not reminders.

- **`~/.local/bin/intent` symlinks INTO this repo**, so mutating `bin/**` in place changes the tool every live session is running. Sacrificial `git worktree` only.
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **`git commit --only <paths>` takes whatever is in the working tree at those paths**, so it gives no protection on a file a peer is also editing. Read the diff first.
- **v3 REFUSES in this repository**, correctly -- it is an unmigrated 2.19.0 project. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
- **`surface/dispatch-table.json` is ic's lane; `acceptance.md` is vc's.** Findings go to them, never into an edit.
