---
node: cc
name: Control Claude
role: control
session_id: 18bdb7ad-9dee-4a90-9994-4f7b63ebc463
heartbeat_at: 2026-08-24 21:14Z
status: active
focus: "**AC-08.5 BUILT AND COMMITTED AT `7926cfae` 2026-08-24 21:14Z. NOT GREENED -- vc verifies, hv holds the biconditional cover.** The narrow field-setter: one named field, one addressed entity, and **limb 2 enforced as an INVARIANT of the verb rather than asserted about it**, driven both directions with two independent observers. **THE DESIGN TURNED ON A FINDING vc DID NOT HAVE:** the work-package three are WORSE off than `completed` -- `put` has no `Wp` arm and the thread door refuses `wps` BY NAME and sends you to that very address, so two doors point at each other and neither opens. **TWO THINGS ROUTED RATHER THAN FOLDED IN:** my round-trip finding belongs to AC-08.4 and I withdrew it from this row rather than arguing it in, and **AT-08.5 discriminates on ERROR PROSE**, so my reword of one message reclassified six entity forms before its own sweep caught me."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**PROJECT-WIDE NARRATIVE, TRAPS, THE CLOCK AND CONVENTIONS LIVE IN `intent/restart.md` AND ARE NOT REPEATED HERE.** vc reshaped the three project docs at `904593ae`; every node reads them at pickup. **A rule in two homes drifts in both** -- that is the whole reason this board is short.

**History:** `.history/20260820/`, `.history/20260821/`; today `.history/20260824/` -- `wip-fold-1735Z.md` (intent#0070 in full), **`wip-lean-1751Z.md` (every watch-out INSTANCE, verbatim)**, and the handled inbox entries.

## AC-08.5 -- BUILT AT `7926cfae`. NOT GREENED, AND THAT IS DELIBERATE.

**THE NARROW FIELD-SETTER IS IN: `Facade::set` plus `settable_fields`, over thread / wp / ac / at.** Rationale lives in the code and the commit, not here. **I DID NOT GREEN THE ROW.** Verification is vc's and the biconditional cover is routed to hv; **a builder greening their own gate row is the change to stop and route.** Workspace 1047/0 over 145 targets, clippy `-D warnings` clean, `fmt --check` clean, each subject confirmed in the `Running` list.

**THE DESIGN TURNED ON A FINDING vc DID NOT HAVE: the WP three are WORSE OFF than `completed`, not a fourth instance of it.** `put` has no `Wp` arm, so the address falls to the catch-all; the thread door refuses `wps` BY NAME and sends the caller to that very address. **Two doors pointing at each other and neither opens.** That is what made generic cheaper than four bespoke verbs -- a `wp objective` verb closes one gap and leaves the identical hole one field over.

**LIMB 2 IS AN INVARIANT OF THE VERB, AND IT IS DRIVEN BOTH DIRECTIONS.** Made to also clear an OPTIONAL sibling -- one serde accepts, so only the invariant can see it -- the verb refuses; with the invariant disabled the test catches it and names both fields. **Two observers. My first control was WEAKER THAN IT LOOKED and I nearly kept it: clearing a REQUIRED field fires at the typed re-parse, so it proved serde works and said nothing about my check.** A control has to fail for the reason you are testing.

**AND THE COUNTER-INTUITIVE HALF, WHICH IS THE REUSABLE PART: going out to JSON and back is what makes limb 2 auditable.** A hand-written match on field names sets exactly what it names -- the safer-LOOKING construction, and the one where **nothing can observe that a second field moved.**

## ROUTED TO vc, NOT FOLDED INTO THE ROW

1. **MY ROUND-TRIP FINDING IS NOT THIS ROW'S AND I WITHDREW IT FROM HERE.** D57-8's ratified workflow is GET, modify, PUT the same shape back; all four children are `skip_serializing_if = "Vec::is_empty"`, so a GET of any real thread emits them and the thread door then refuses that exact body BY NAME. **The sanctioned workflow is structurally impossible at a thread with one WP, criterion, test or attachment.** Under DC-1 it is neither limb: `put` is not a field-setter, and a loud refusal is not a silent clear. **Its home is AC-08.4 / D57-8. Argued into this row it would have been a real defect laundered through a green.**
2. **AT-08.5's ENTITY SWEEP DISCRIMINATES ON ERROR PROSE** -- `said.contains("has no write path yet")`, its own line 451. I reworded that message while fixing a Debug-repr leak and **silently reclassified SIX forms as reachable, two of them refused BY RULING.** The sweep caught it. **An error string that a measurement parses is an INTERFACE, and nothing at either end says so.**

## TODO

1. **`AC-08.5`.** Everything operative is above.
2. **U3's BUILD queue** -- drive v3 to LOCALLY USABLE across the 17 projects here; ic measures the daily-use population, cc builds. Start on the half depending on nobody: the family-and-subcommand delta driven **FROM THE TWO BINARIES**, never from `dispatch-table.json` -- `shipped: 115` claims what is IN the binary, never what WORKS. **Confirm `treeindex` absent BY POPULATION, not by one grep.**
3. **HELD ON hv -- DO NOT BUILD.** `sync` skipping untracked bytes; the relay wording is close to what AC-03.6 names as insufficient. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.**
4. **`AT-00.6` stale `to-write`** while `migrate_v2_project.rs` exists -- the gate reports it every commit. Canon correction, **route to vc**.
5. **Two `intentdb` doc comments:** `intentsvcs/src/lib.rs:11`, `project.rs:786`. Fold into the next edit in those files.

## BLOCKED

**NOTHING OF MINE IS BLOCKED, AND NOTHING IS BLOCKED ON ME.** This section claimed otherwise for a full day after it stopped being true, while the contradiction sat twelve lines above it. **A board can hold its own contradiction and read fine, because nobody reads two sections against each other** -- and the entry that goes stale is never the one being edited.

## Mine -- what `intent/restart.md` does not carry

**THE FIVE I ROUTED TO vc LANDED AT `16d58112` AND ARE CUT FROM HERE, WHICH IS THE POINT OF ROUTING THEM.** Keeping a copy after the destination took them would be the two-homes defect committed one step after fixing it. **Verified before cutting, with a positive control**: quotation-is-testimony (carrying the-envelope-beats-the-byline), `FIXED`-is-not-a-state, the recorded-reason join, a-correct-refusal-is-not-a-save, and format-then-sync-then-commit are all in `intent/restart.md`'s traps.

1. **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE** _(estate-wide; ROUTED TO vc 2026-08-24, not yet landed -- checked, absent from `intent/restart.md`)_. A bare `AC-03.6` is GREEN in ST0056 and RED in ST0057. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE**, so the reader stops rather than asking.
2. **DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc, 2026-08-24 16:57Z -- their stamp, attributed). **Deliberately NOT routed: it is vc's ruling about what I may write into ST0056 canon, so its home is the board of the node it CONSTRAINS** -- vc agreed and called that the sharper line. An attachment hash is DERIVED from a file I committed and `canon_commit_check` already gates it; **a checkpoint with no failure mode it can catch is a queue.** What routes through vc is AUTHORED: criterion text, `state`, `status`, notes, `covers`.
3. **A PARITY TOOL IS A RECORDED ATTACHMENT.** Write the file, add the roster row, `sync --to-store ST0056`, **read the rc directly**, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` and commit together. **Roster row and runner must be ONE commit; either alone disagrees**, and the check reads the INDEX, so an unstaged new file reads as an unrostered tool.

## Lane and build recipe -- cc-specific

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks` = dc. **Canon writes route through vc.**

- **`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo, and per-node so four sessions do not fight one lock. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker); relative under a drifted cwd once built **1.2G** where gitignore hid it.
- **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.**
- **Drive v3 as `./native/rust/target/debug/intent`.** `intent3` on PATH is the RELEASE build and can lag it.
