---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 09:27Z
status: active
focus: "AC-04.6 service half landed acf8491; surface blocked on ic's row. Closure is necessary and NOT sufficient -- my own fix disarmed the test, mutation-testing caught it. Next: marked-legacy scope."
claims: []
---

# Control Claude (cc)

## DOING

- **AC-04.6's SERVICE half is landed at `acf8491`** -- `transitions.rs` (the declared graph) + `AT-04.6` green, 245 tests, fmt and clippy clean. **Surface half is blocked on ic** and correctly so: the spine is built from the dispatch table, so `intent ac unsatisfy` needs ic's row first. Told ic at 09:25Z with the addition recorded BEFORE the wiring (AC-06.3). vc has the gate row and the one judgement call: whether five `Unbuilt` fields owing mutations leave AC-04.6 short of closing.
- **CLOSURE IS NECESSARY AND NOT SUFFICIENT** -- the morning's real finding, and it is against my own instrument. Mutation-testing showed that once scope changes cleared satisfaction, deleting `ac.unsatisfy` STILL left `satisfied: true` formally leavable via descope-then-rescope, so the closure check went green over the exact defect hv ruled on. **My own fix is what disarmed the test.** Edges are now Direct or Incidental; incidental counts for reachability and never discharges a trap. The rule: _a state you can only leave by changing a different field is still a state you cannot leave._ Six mutations, all caught.
- Gates: 01, 02, 03, 05 PASS. WP-04 pending vc on AC-04.6; WP-06 4/7 (AC-06.1, AC-06.6 mine; AC-06.3 is vc's and ic's).
- Session detail is in `.history/20260815/`.

## TODO -- in this order

1. **The marked-legacy `scope` field.** Shape DECIDED, so this is a build: keep `scope` a **unit-only, non-optional** enum, carry the out-of-enum spelling in a **sibling optional field**. Unit-only because `TShirt` derives async-graphql's `Enum`; non-optional because `Option<TShirt>` would make it nullable for all 129 well-formed work packages and admit an invalid both-none state. Requirement: **the value is neither guessed nor dropped**. Driven by `Medium-Large` (1 of 129, `intent/st/COMPLETED/ST0020/WP/09/info.md`). Ruling: `data-model.md:83-89`.
2. **AC-06.6 -- `intent export --format <fmt>`.** Round-trip to byte-identical canon, OR refuse the format BY NAME rather than emit lossily. Settle first: whether `md` can round-trip at all, or must be refused despite D03 naming it.
3. **AC-06.1 -- the surface tail.** `st edit`, `st repair`, **`st bootstrap`** (hv RULED the verb at `c1cca8c` -- not `initzero`, not the incumbent `st zero`; `zero` was never a verb, it is the NAME of the thing, so the real verb was `install` hiding a level down. `install` is COLLAPSED into the bare form, flags `--audit-only`/`--dry-run`/`--deliverable`, root face DELETED. **Watch when wiring**: `st_zero`'s row is `corrected`, so `is_shipped()` is true for a deliberately deleted face and it is today indistinguishable from a merely-unbuilt one); `issues`, `todo`; `info`, `version`, `config`, `init`, `bootstrap`; then `claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `fileindex`. **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable. And `bin/intent_st:1231` is `[0-9]+)` -- `+` is literal in a `case` glob, so only the 4-digit form of `st repair` has ever worked.

## Lane boundary, from 2026-08-15

`dc` (DevX Claude) owns dev-x, build, CI, release mechanics, git workflow and the install story -- including the devbin handlers I wrote this morning and hv's Conflab flavour-switch ask. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` and `design.md` are vc's. Full handover with the measurements is in `dc/inbox.cc.md` and `.history/20260815/wip.md`.

**The boundary is PROPOSED (vc), not ruled, and `bin/` is the one real collision** -- the v2 bash CLI is mine, `bin/int` is dc's, same directory. vc's own test settles it cleanly and I have proposed this to dc: **`bin/intent*` is cc's** because it is the INCUMBENT whose behaviour WP-06 ports -- a parity reference, not a build tool, and changing it changes what the tool DOES. **`bin/int`, `bin/devbin`, `bin/.devbin/**` are dc's** -- changing them changes only how it gets built and run. **dc ACCEPTED it at 09:00Z**, still proposed-pending-hv, and corrected the argument: the load-bearing half is not ownership but the FREEZE -- `bin/intent*` is the baseline ic's burn figures and register rows are all measured from, so if it moves they measure a moving target and silently stop meaning what they say. "cc's" really means "cc is the one who has to refuse", which dc notes is frozen-by-contract rather than by convention and may deserve a control rather than an agreement.

## Waiting on hv

- **Three MODEL questions, all recorded as declared orphans in `transitions.rs` with their evidence** so they cannot be forgotten: `ThreadStatus::tbc` (v2 treats `TBC` as the DISPLAY of `Not Started`, `bin/intent_st:120` -- likely a display alias reified into the model), `ThreadStatus::hold` (real v2 vocabulary, no v2 command sets it), `satisfied: false` (nothing produces it; `None` and `Some(false)` render identically at `views.rs:443`, so three values and two meanings). None is a mutation gap.
- **D01 is now LOAD-BEARING, not merely queued.** Every mutation writes committed canon and lets the DB rebuild from it. If "durable state is in the db" reverses D01, `apply()` changes shape underneath all of WP-04. Proceeding on D01 as written and flagging the assumption rather than inferring the reversal. Third node to stop on it.

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
- **TWO symlinks point INTO this repo**, not one: `which -a intent` returns three reachable copies, with `~/.local/bin/intent` and `~/bin/intent` both landing on `Intent/bin/intent` (dc measured; I had said one). So mutating `bin/intent*` in place changes the tool every live session runs. Sacrificial `git worktree` only. (`bin/.devbin/**` is exposed through neither, which is where the lane line falls.)
- **`git stash` is unsafe here** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **`git commit --only <paths>` takes whatever is in the working tree at those paths** -- no protection on a file a peer is also editing. Read the diff first. The index has carried staged-only content from before a formatter run more than once.
- **v3 REFUSES in this repository**, correctly -- unmigrated 2.19.0. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
- **`surface/dispatch-table.json` is ic's lane; `acceptance.md` is vc's.** Findings go to them, not into an edit -- except the one mechanical commit vc explicitly suspended it for.
