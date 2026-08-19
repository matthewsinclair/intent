# Claude Code Session Restart -- narrative state

## Current state (as at `5716b43a`, 2026-08-19)

**This heading names a COMMIT, not just a date, and that is deliberate.** A restart file is read as CURRENT STATE and written as a snapshot of when its author typed; nothing used to mark which, and a cold session treated a four-day-old line as the next action. Anything below is true of `5716b43a` and of nothing else -- **re-stamp it when you fold, and if you cannot say what it is current as at, that is the finding.**

## The one thing that changed everything tonight

**THE DISK MODEL IS RUNNING. `intent organize --apply` removed 423 files at `e7f00e65`.** `intent/st/` holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`, and nothing else. Fifty-two completed and two cancelled threads live only in the database.

**It is proven reversible by measurement.** ST0001 back on the list returned five files, all byte-identical to git. A fence-heavy pair returned fifteen, all byte-identical. All 282 attachments verify against their own `sha256`. **Dehydration is not a loss** -- the database regenerates the exact bytes.

**hv replaced the manifest design mid-session and it is now canon:**

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** Nothing derives the list from status. That is the whole difference from the two-region design, and it is why the protected region became unnecessary and why `intentfiles::render` had no caller. **ABSENT IS NOT EMPTY:** a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

## The two threads

**ST0056 -- Intent v3.0.0.** Architecture ratified in `intent/st/ST0056/design.md` (D01-D36); read it before touching anything v3. Schema-as-truth (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, committed and drift-checked); **the intentdb as the DURABLE SSOT with everything on disk a secondary artefact** -- **D01 was REVERSED by hv 2026-08-15 and the old wording ("committed JSON as durable truth, rebuildable SQLite, `rm intent.db` always safe, no DB migrations ever") is FALSE IN EVERY CLAUSE; do not reason from it**; the committed extract as the INTERCHANGE that travels while the DB never leaves the machine (D34); migrations NORMAL; `rm intent.db` ruled out of existence (D36); `intentsvcs` as sole owner of DB and file canon; CLI dual-mode; MCP as the primary agent write surface; migration floored at v2.19.0.

**ST0057 -- disk as a sparse projection of the store.** D57-1..D57-8 ruled. **Sparseness applies to VIEWS; canon is NEVER sparse** -- if the manifest governed canon, an unrealised artefact would exist only inside a gitignored database. **D29, a gitignored path is never canon, is what makes a clone complete.**

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

**Contract, measured at this commit.** ST0056: **131 criteria, 132 tests** -- 55/131 satisfied, green 46, n-a 19, red 8, to-write 59. ST0057: **45 criteria (1 withdrawn), 46 tests** -- 33/45 satisfied, green 33, n-a 2, red 5, to-write 6. **ST0056 WPs** 01/02/04 Done, 03/05/06/10/11 WIP, rest Not Started. **ST0057 WPs** 01/02/04/05/07/08 WIP, 03/06/09 Not Started.

**THE GATE: 50 OF 64** -- all of ST0057's 46 rows plus all 18 of ST0056 WP-03. It was 33 this morning.

## The precondition block is 14 and reads 0 unmet, and that is NOT a scoreboard

hv cleared four preconditions out of AC-00.1's declared block on one word, and AC-00.3 earlier on the git ruling: _if we need safety, we've got all the historical STs etc in git. We can always get stuff back._

**ONLY AC-00.3 IS WITHDRAWN. AC-03.6, AC-06.3, AC-06.4 AND AC-07.5 REMAIN LIVE CRITERIA AND ARE STILL OWED.** dc's distinction: **the question was never whether the work is wanted, it is whether a GATE should hold on it.** A board recording "they came off the gate" reads as done to whoever opens it next.

**The test for withdrawing a precondition, ic's half being the better half:** hv's git grounds retire a precondition **only where git can SUBSTITUTE for the proof**. _Restoring the estate from git RE-HYDRATES it, which destroys the precondition under test_ -- so an accessibility claim is never withdrawable on those grounds and a safety claim usually is.

## Tomorrow's question, hv's own

**250 files under `intent/` are not in the store at all.** `docs/` (12), `llm/` (14), `history/` (18), `eng/` (10), `plugins/` (191), `autopsy/`, `analysis/`, and the project-level `done.md`, `wip.md`, `restart.md`, `todo.md`. The store holds **threads, work packages and issues, and nothing else** -- `doc_sections.owner_type` is exactly those three.

**hv: _not all of that should be in the db, but certainly some of it should. A job for tomorrow._**

**cc's angle: ask which of the 250 an artefact could even OWN**, because the manifest names artefacts and never files, so a file with no owning artefact cannot be declared however the content question is settled.

**THE TRAP IN MEASURING IT: there IS a `done.md` in the store -- ten of them -- and every one is `intent/st/ST0019/done.md` or a sibling, a per-thread attachment.** A grep returns hits and the project-level file is not among them.

**The same gap from the command side, dc's measurement:** 16 of 32 top-level families dispatch, 14 answer exit 2, **`intent claude` implements 1 of its 8 verbs** against 230 `intent claude <verb>` call sites in this repo's own machinery. **Everything that manages STEEL THREADS is done; everything that manages INTENT ITSELF is not.** Two faces of one gap, found from opposite ends on the same evening.

## Standing hazards

**DO NOT PUT v3 ON PATH.** The pre-commit gate works _because_ it runs v2, whose version guard is scoped to writes, so `intent critic` runs clean while `intent st list` refuses at 2. The day v3 goes on PATH, `intent critic` answers 2 in all five declared languages -- the gate's fail-open code -- here and in the other 15 Intent projects on this machine.

**EACH SYNC DIRECTION DESTROYS WHATEVER EXISTS ONLY ON THE OTHER SIDE.** `--to-disk` destroys unsynced DISK edits; `--to-store` destroys unprojected STORE state, and it is `sync_from_disk` the code itself calls _the DESTRUCTIVE direction_. **`intent sync --to-store <ID>` before any verb.**

**`at green` MOVES A STATUS AND NO VERB MOVES A NOTE.** Twice tonight a row read green above prose saying the test does not exist. Canon prose routes through vc.

**THE REVISION IS PART OF THE FINDING, NOT CONTEXT FOR IT.** In a four-node checkout, a measurement can be true of a tree one rebuild or one mid-write file out of date. Name revision, clock and dirty count on every measurement.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds ST0056 + ST0057).
