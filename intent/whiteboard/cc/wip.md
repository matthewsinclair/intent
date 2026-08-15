---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 08:56Z
status: active
focus: "cc is now services + app functionality; dev-x/build/git went to dc. AC-04.6 FIRST -- mutation completeness reopened WP-04 and is costing vc hand-edits. Then marked-legacy scope, export (AC-06.6), surface tail (AC-06.1)."
claims: []
---

# Control Claude (cc)

## DOING

- **The native/ move is landed, verified and pushed** (`a1a949c`, both remotes). All Rust lives at `native/rust/{Cargo.toml,Cargo.lock,rustfmt.toml,target/,crates/}`; `native/macos/` is reserved for a Swift app. **Run cargo from `native/rust`.** 234 tests, fmt and clippy clean. vc re-verified the CONTRACT survived: gate results byte-identical pre- and post-move, L2 resolves all 24 green rows, L3 passed (which independently proves `git mv` preserved bytes).
- Gates: 01, 02, 03, 05 PASS. **Two blocked, both mine**: WP-04 5/6 (AC-04.6) and WP-06 4/7 (AC-06.1, AC-06.6 mine; AC-06.3 is vc's and ic's).
- Session detail is in `.history/20260815/`.

## TODO -- in this order

1. **AC-04.6 -- MUTATION COMPLETENESS (D32).** hv ruled it and it reopened WP-04 from PASS, so it is a gate regression rather than new scope -- and the defect is costing vc hand-edits to `acceptance.md`, the file the CLI exists to own. **Every state an entity can enter, it can leave, by a service call reachable from every surface.** The instance: `intent ac satisfy` is a one-way door, and `rescope`/`reinstate` only undo a descope. **Held to a MECHANICAL test** (`AT-04.6` = `native/rust/crates/intentsvcs/tests/mutation_completeness.rs`, to-write): enumerate each modelled state field, assert its transition set is CLOSED, fail naming the state and the missing inverse. Discriminating case is satisfy-then-unsatisfy. Build the enumeration GENERAL -- the rule also binds D30's whiteboard operations, so special-casing `ac` would satisfy the AC and miss the rule.
2. **The marked-legacy `scope` field.** Shape DECIDED, so this is a build: keep `scope` a **unit-only, non-optional** enum, carry the out-of-enum spelling in a **sibling optional field**. Unit-only because `TShirt` derives async-graphql's `Enum`; non-optional because `Option<TShirt>` would make it nullable for all 129 well-formed work packages and admit an invalid both-none state. Requirement: **the value is neither guessed nor dropped**. Driven by `Medium-Large` (1 of 129, `intent/st/COMPLETED/ST0020/WP/09/info.md`). Ruling: `data-model.md:83-89`.
3. **AC-06.6 -- `intent export --format <fmt>`.** Round-trip to byte-identical canon, OR refuse the format BY NAME rather than emit lossily. Settle first: whether `md` can round-trip at all, or must be refused despite D03 naming it.
4. **AC-06.1 -- the surface tail.** `st edit`, `st repair`, `st zero`; `issues`, `todo`; `info`, `version`, `config`, `init`, `bootstrap`; then `claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `fileindex`. **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable. And `bin/intent_st:1231` is `[0-9]+)` -- `+` is literal in a `case` glob, so only the 4-digit form of `st repair` has ever worked.

## Lane boundary, from 2026-08-15

`dc` (DevX Claude) owns dev-x, build, CI, release mechanics, git workflow and the install story -- including the devbin handlers I wrote this morning and hv's Conflab flavour-switch ask. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` and `design.md` are vc's. Full handover with the measurements is in `dc/inbox.cc.md` and `.history/20260815/wip.md`.

**The boundary is PROPOSED (vc), not ruled, and `bin/` is the one real collision** -- the v2 bash CLI is mine, `bin/int` is dc's, same directory. vc's own test settles it cleanly and I have proposed this to dc: **`bin/intent*` is cc's** because it is the INCUMBENT whose behaviour WP-06 ports -- a parity reference, not a build tool, and changing it changes what the tool DOES. **`bin/int`, `bin/devbin`, `bin/.devbin/**` are dc's** -- changing them changes only how it gets built and run. Awaiting dc's or hv's confirmation.

## Waiting on hv

- **`st_zero`'s root face.** hv floated `intent st initzero`; ic then found the fact that changes the question -- **`intent st zero` ALREADY EXISTS** (`bin/intent_st:1610-1612`) and its own usage documents only that spelling. So this is a DELETION of an undocumented root face, not a rename, and it costs ZERO divergence. Two-way for hv: keep `st zero` (survives, self-documented, no divergence row) vs `st initzero` (reads better -- `st`'s subcommands are verbs and `zero` is a noun -- but buys a divergence). **The underscore dies either way, which is what hv actually ruled on.** Both rows held at `pending` with the evidence written in; I wire whichever lands.

## Standing rulings

- **`treeindex` and handover RETIRE.** DB source-tree index obviates treeindex; the DB model obviates handover. State moves out of per-session `.md`s into durable state in the intentdb -- the D30/WP-14 direction. Row landed `retire` by ic at `0434223`. **A retired command is PRESENT AND REFUSING, not absent** (my ruling): it carries no functionality, only an explanation, and AC-04.4 says an error names its cause. Scoped to the v3 line. **The build consequence is mine and it breaks a guard**: `dispatch::is_shipped()` excludes `retire` rows, so a retired-but-refusing command is absent from `shipped_entries()` and present in the spine, and `dispatch_ssot.rs` asserts both directions. Fix the guard with the feature.
- **`fileindex` is NOT covered** -- different mechanism (checkbox indexes, not directory summaries). Its `pending-hv` INV-07 question stands.
- **Push to all remotes when needed.**

## Watch-outs -- mechanical only

Everything amounting to "remember to" is archived; it failed twice on entries this board already carried. These are facts about the estate, not reminders.

- **`--only` commits what you NAME, and a move is TWO facts** (vc). The add and the delete are separate index entries; naming the new path commits an addition and leaves the deletion staged. `a1a949c` did exactly this -- 58 additions committed, 55 files under `crates/` plus three root build files left at HEAD, on both remotes, where a fresh clone would have built the OLD tree from five DIVERGENT files. **Every working-tree check passed** -- 234 tests, fmt, clippy, lint, six gates -- because the working tree was right and only the repository was wrong. `--only` stays: it is what stopped that same commit sweeping a peer's inbox. Name the deletion side too, and **verify at HEAD (`git ls-tree`), never on disk**.
- **After any move, clone fresh and build.** It is the only check that sees the class above. A green suite is evidence about the tree you HAVE, never about the tree you PUSHED.
- **Cargo runs from `native/rust`.** A repo-root `cargo` finds no manifest.
- **A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE; cargo compares timestamps and inputs, not the manifest ROOT, so a path move bakes into artefacts invisibly. Tell: passes in isolation, fails in the suite -- that is a conclusion (something is shared and one run is lying), not flakiness. Cost 1.2G and an hour.
- **`~/.local/bin/intent` symlinks INTO this repo**, so mutating `bin/intent*` in place changes the tool every live session runs. Sacrificial `git worktree` only. (`bin/.devbin/**` is not exposed that way.)
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **`git commit --only <paths>` takes whatever is in the working tree at those paths** -- no protection on a file a peer is also editing. Read the diff first. The index has carried staged-only content from before a formatter run more than once.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
- **`surface/dispatch-table.json` is ic's lane; `acceptance.md` is vc's.** Findings go to them, not into an edit -- except the one mechanical commit vc explicitly suspended it for.
