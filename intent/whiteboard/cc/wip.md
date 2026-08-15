---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 10:57Z
status: active
focus: "HOLDING at hv's instruction while hv+vc settle the D01 canon. Four WPs reopened today; AC-04.6, AC-04.1 and AC-03.9's facade half are landed. AC-02.6 (openness) is next, not started."
claims: []
---

# Control Claude (cc)

## DOING

- **HOLDING.** hv: _"Just hold for a moment while VC and I sort this out. We'll come back with something definitive and canonical shortly."_ Folded and waiting. **Do not start AC-02.6 until they land it.**
- **D01 IS REVERSED** (hv, 2026-08-15): the intentdb is the durable SSOT, everything on disk is a secondary artefact, and **DB migrations are normal** -- "no migrations, ever" was never hv's constraint. The real requirement is **platform and data-model openness**: a 1-1 mapping between every DB entity and a `.json`/`.md` form, so the data comes out LOSSLESSLY and is usable without Intent. That is why bidirectional sync exists -- not backup, not disposability.
- **Landed today** (all pushed, clone-checked, 257 tests green): AC-04.6 mutation completeness + `transitions.rs`; AC-04.1's rollback tests; AC-03.9's facade half (`sync_to_disk`/`sync_from_disk`/`sync_overwrite`); the D01 write-path reversal; `ac unsatisfy` end to end; `wp_rescope`. Detail in `.history/20260815/`.

## TODO -- after the hold

1. **AC-02.6 -- openness** (vc, WP-02 reopens 5/6). `AT-02.6` = `openness.rs`. **Enumerate the table list FROM THE GENERATED DDL FACE**, never a hand roster. Each table needs a file form OR a DECLARED exemption naming why it is derivable. Round-trip both directions. **The discriminating case is ADDING a table with no file form and no exemption and watching it go red** -- a test over the tables that already have forms passes on the defect, which is exactly how `event_log` survived. Two known gaps: `event_log` (now `events.jsonl`) and `file_index` (plausible exemption, must be declared).
2. **Five test files still narrate old D01** in prose while asserting properties that survive as CAPABILITIES rather than laws: `canon_round_trip`, `store_round_trip`, `store_rebuild`, `ignored_paths_corpus`, `sync_scan`. Left deliberately -- correcting them twice is worse than once.
3. **The marked-legacy `scope` field.** Shape decided (`data-model.md:83-89`): unit-only non-optional enum + sibling optional field. Driven by `Medium-Large`, 1 of 129.
4. **AC-06.6 export**, then **AC-06.1 surface tail**.

## Waiting

- **hv + vc**: the definitive D01 canon. Blocks nothing landed; shapes everything next.
- **ic**: the `sync` direction spelling (asked 10:16Z -- the facade has both directions, the CLI cannot select between them because `sync`'s dispatch row has no flags). And the `at` guard ruling -- **v3 has NONE of v2's four at set-time guards**; the gate recovers two, and _green-only-from-red CANNOT be recovered at gate time ever_, because it is a property of history. Taking them seriously breaks my per-field transition model: v2's `at` graph is conditional on `kind`, so the union view false-passes.

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
- **THREE REMEDIES IN THIS ESTATE INSTRUCTED DATA LOSS, all found the same day and all by checking a remedy's premise as it was written.** `ViewsNotWritten` said "run `intent sync`" (disk->db: overwrites the SSOT with the stale copy); `FacadeError::Store` said "delete `intent/.cache/intent.db`" (that IS the SSOT, and it is shown on every store error); `doctor`'s module doc said "`rm` is always safe". Each was TRUE under the model it was written against and became destructive when the model moved. Fact about the estate: **a remedy naming a command outlives the reasoning that made it correct.**
- **A report of N sites is a sample until someone counts.** vc named four false-canon sites; there were sixteen. Same shape as their `hooksPath` grep, my `| head`, my hyphen regex, my `type == "boolean"` probe. Four instances, one class.
- **`surface/dispatch-table.json` is ic's lane; `acceptance.md` is vc's.** Findings go to them, not into an edit -- except the one mechanical commit vc explicitly suspended it for.
