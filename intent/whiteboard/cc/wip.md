---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 08:27Z
status: active
focus: "WP-06 is the only blocked gate (4/7) and it is mine. Three pieces left: the marked-legacy scope field, `intent export` (AC-06.6), and the surface tail (AC-06.1) -- now minus treeindex, retired by hv."
claims: []
---

# Control Claude (cc)

## DOING

- **Estate green and pushed to BOTH remotes** (`local`, and `upstream` at `c8aee92`). 234 tests, fmt and clippy clean. Gates 01-05 PASS; **WP-06 is the only blocked one and it is mine** (4/7 -- AC-06.1, AC-06.3, AC-06.6 outstanding, and AC-06.3 is vc's and ic's).
- The overnight session's detail, watch-outs and lessons are in `.history/20260815/`.

## TODO -- in this order

1. **The marked-legacy `scope` field.** Shape is DECIDED, so this is a build rather than a decision: keep `scope` a **unit-only, non-optional** enum and carry the out-of-enum spelling in a **sibling optional field**. Both halves matter -- unit-only because `TShirt` derives async-graphql's `Enum` and a data-carrying variant breaks the SDL face; non-optional because `Option<TShirt>` would make it nullable for all 129 well-formed work packages and admit an invalid both-none state. Requirement to hold: **the value is neither guessed nor dropped**. Driven by `Medium-Large` (1 of 129, at `intent/st/COMPLETED/ST0020/WP/09/info.md`), which maps to nothing in `XS · S · M · L · XL · XXL` and sits in a CLOSED thread where hv's policy is lossless-by-carrying. Ruling + measurement: `data-model.md:83-89`. Touches the model, all three faces, the store DDL, ingest and the renderer.
2. **AC-06.6 -- `intent export --format <fmt>`.** D03's mechanism: "YAML/md/anything else are `intent export --format` projections", which is how v3 refuses YAML canon without refusing YAML users. **Round-trip -- what it emits re-ingests to byte-identical canon -- OR refuse the format BY NAME rather than emitting lossily.** Settle first: whether `md` can round-trip at all, or must be refused by name despite D03 naming it.
3. **AC-06.1 -- the surface tail.** `st edit`, `st repair`, `st zero`; the `issues` and `todo` families; `info`, `version`, `config`, `init`, `bootstrap`; then `claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, **`fileindex`**. The AC's own clause: **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable by construction. Also: `bin/intent_st:1231` is `[0-9]+)` and `+` is literal in a `case` glob, so only the 4-digit form of `st repair` has ever worked -- a forced fix, unconstructible in clap.

## hv rulings -- 2026-08-15 morning

- **`treeindex` RETIRES**, and so does **handover**. The source tree index in the DB obviates `treeindex`; the DB model obviates handover entirely. **State moves out of per-session `.md`s shared between workstreams and into durable state in the intentdb** -- the same direction D30/WP-14 takes the whiteboard. Its dispatch row still says `disposition: keep, target: pending-hv`, so the row needs changing, and **`surface/dispatch-table.json` is ic's lane**. Its open `pending-hv` question (INV-07, `--help` exits non-zero) is now moot.
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
