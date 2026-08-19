---
verblock: "19 Aug 2026:v1.11: vc - the dehydration shipped; 423 files out, gate 50/64, the block is 14 and reads 0 unmet"
intent_version: 2.19.0
---

# Work In Progress

## Current State (as at `5716b43a`, 2026-08-19)

**This heading names a COMMIT, not a date.** A wip file is read as current and written as a snapshot; if you cannot say what it is current as at, that is the finding.

**THE DISK MODEL IS RUNNING, NOT DESIGNED. `intent organize --apply` removed 423 files from this estate tonight at `e7f00e65`.** `intent/st/` now holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database.

**AND IT IS PROVEN REVERSIBLE BY MEASUREMENT RATHER THAN BY ARGUMENT.** ST0001 put back on the list returned five files, **all five byte-identical to what git holds**. A fence-heavy pair (ST0016 at 48 fence markers, ST0034 at 180 non-ASCII characters) returned fifteen files, **all byte-identical**. All **282 attachments** in canon verify against their own `sha256` and byte count. **Dehydration is not a loss.**

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2. **DO NOT PUT v3 ON PATH** -- the pre-commit gate works _because_ it runs v2, whose version guard is scoped to writes, so `intent critic` still runs clean while `intent st list` refuses. The day v3 goes on PATH, `intent critic` answers 2 in all five declared languages, which is the code the gate fails open on, here and in the other 15 Intent projects on this machine.

### The two threads

**ST0056 -- the v3.0.0 rewrite.** Architecture in `design.md` (D01-D36). **The intentdb is the DURABLE SSOT; nothing on disk is truth.** D01 was REVERSED by hv 2026-08-15 -- do not reason from it. Contract: **131 criteria / 132 tests**, 55/131 satisfied (green 46, n-a 19, red 8, to-write 59). WPs 01/02/04 Done; 03/05/06/10/11 WIP; 07/08/09/12-16 Not Started.

**ST0057 -- disk as a sparse projection.** Contract: **45 criteria (1 withdrawn) / 46 tests**, 33/45 satisfied (green 33, n-a 2, red 5, to-write 6). WPs 01/02/04/05/07/08 WIP; 03/06/09 Not Started.

### THE GATE: 50 OF 64

**All of ST0057's 46 AT rows plus all of ST0056 WP-03's 18. It was 33 this morning.** Green tonight: AT-00.1, AT-00.4, AT-02.2, AT-02.3, AT-03.1--03.5, AT-04.1--04.5, AT-06.1, AT-06.2, AT-07.5.

### THE ARCHITECTURE hv RULED, 2026-08-19, REPLACING THE TWO-REGION DESIGN

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds the id, `st done` removes it, `st hydrate` / `st dehydrate` and the ISSUE equivalents do it directly, and a human may edit it by hand. **Nothing derives it from status.** That is why the protected region became unnecessary -- a write is a change to state, never a regeneration of it -- and why `intentfiles::render` had no caller: the thing it does is not needed.

**ABSENT IS NOT EMPTY.** A missing manifest means nobody has said, and everything stays. A manifest present and declaring nothing means keep nothing -- which is why 545 files sat on the removal path by omission until the list was populated.

### The precondition block is 14 and reads 0 unmet -- READ THIS BEFORE ASSUMING ANYTHING IS DONE

hv cleared four preconditions out of AC-00.1's declared block on one word, and AC-00.3 earlier on the git ruling (_if we need safety, we've got all the historical STs etc in git_). The block went 19 -> 18 -> 14.

**ONLY AC-00.3 IS WITHDRAWN. AC-03.6, AC-06.3, AC-06.4 AND AC-07.5 REMAIN LIVE CRITERIA AND ARE STILL OWED.** dc's distinction is the one to keep: **the question was never whether the work is wanted, it is whether a GATE should hold on it.**

**The test for withdrawing a precondition, and ic's half is the better half:** hv's git grounds retire a precondition **only where git can SUBSTITUTE for the proof**. _Restoring the estate from git RE-HYDRATES it, which destroys the precondition under test_ -- so an accessibility claim is never withdrawable on those grounds and a safety claim usually is.

## Next Up

1. **hv's own question, and it is item 1 for everyone: 250 files under `intent/` are not in the store at all.** `docs/` (12), `llm/` (14), `history/` (18), `eng/` (10), `plugins/` (191), `autopsy/`, `analysis/`, and the project-level `done.md`, `wip.md`, `restart.md`, `todo.md`. The store holds **threads, work packages and issues, and nothing else**. hv: _not all of that should be in the db, but certainly some of it should._ **cc's angle: ask which of the 250 an artefact could even OWN, since the manifest names artefacts and never files.** **The trap: there IS a `done.md` in the store -- ten of them -- and they are `intent/st/ST0019/done.md` and siblings.**
2. **The same gap from the command side, dc's measurement: 16 of 32 top-level families dispatch, 14 answer exit 2, `intent claude` implements 1 of its 8 verbs** against 230 `intent claude <verb>` call sites in this repo's own machinery. **Everything that manages STEEL THREADS is done; everything that manages INTENT ITSELF is not.** `intent claude rules` landed tonight (`2e512d2e`) -- the 125-call-site verb.
3. **dc** -- AC-06.3 (the third `Projection` variant), AC-06.4 (`intent init` from an empty directory, now hosting work rather than gate work), and the hosting sweep.
4. **ic** -- `st hydrate`'s render arm (two lines now `address::promote` landed at `8a6ae532`); the `st edit` fork, unruled, with `edit_writes_pinned_region.rs` still asserting the retired architecture behind a red AT-05.2; the `issues dehydrate` bucket ruling that understates by four.
5. **cc** -- AC-03.6; wiring `intent doctor`'s view-skew detection into the gate, since the detection exists and only the wiring is missing.
6. **vc** -- ST0057/WP-09, filed tonight and unstarted: **the event log records the MODEL and not the DISK.** `Facade::apply` is a real chokepoint, `grep -c 'apply(' organize.rs` returns 0, and 423 files left this estate with the log recording nothing.

## Recent

- **2026-08-19 (evening)**: **THE DEHYDRATION SHIPPED.** hv replaced the two-region manifest design mid-session; ic rebuilt the mechanism, dc the dispatch rows, cc the root-file generator. Gate 33 -> 50. Also fixed tonight: `organize` removes the directories it emptied (`dd06342a`, driven live -- `1 pruned`); the unclaimed report carries a **digest over the sorted set**, because a count plus directories is byte-identical across a same-directory swap (`8e544de4`); and `0 to remove (423 blocked)` folds the blocked figure into the summary line rather than a note beneath it (`456e72a4`).
- **2026-08-19 (morning)**: canon relocated to `intent/.canon/`; ST0057 WP-01 code and the live file move.
- **2026-08-14**: v2.19.0 SHIPPED (tag `071c612`, issues 0009-0023). ST0056 opened the same afternoon.
- Earlier: `intent/history/`.

## Parked

3.x steel threads, post-3.0.0, each on its own: TUI dashboard; the agent bus; Laksa web page; macOS menubar app; `intent_ex` hex client; sqlite-vec semantic search.
