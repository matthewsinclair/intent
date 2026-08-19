---
node: vc
name: Validation Claude
role: validation
session_id: 590c4fbc-ea99-41b3-9c10-75344a715f96
heartbeat_at: 2026-08-19 20:31Z
status: active
focus: "**`intent at green`/`at red` REWRITE CANON AND LEAVE THE COMMITTED VIEW SAYING THE OPPOSITE, AT rc=0 -- clone-measured, and NOTHING REPAIRS IT TODAY.** ST0057's `acceptance.md` contradicts canon on 7 of 46 rows, both directions, and reads _0 of 8 WPs_. `st hydrate` is the only repair path and it is `unrecognized subcommand`. **GATE IS 46 OF 64, NOT 48 -- vc had been counting two `n-a` rows as passing that `contract.rs` counts as never green.** AC-05.2's `refuse` clause withdrawn on measurement; AT-02.2 re-pointed and one assertion short. Arming 6 unmet."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## ON RESUME -- PLAIN WORDS, READ THIS FIRST

**46 of 64.** Measured at HEAD, never in a working tree. It was 33 this morning and 51 at its highest; **it went DOWN because hv changed a design and two rows now test retired properties, one vc moved down on measurement, and TWO WERE NEVER PASSING AT ALL.**

**THE COUNTING CORRECTION, BECAUSE IT IS vc's OWN AND IT INFLATED THE HEADLINE FOR HOURS.** Two of the 64 rows are `kind: non-test` and carry status `n-a` -- AT-00.3 and AT-05.3. **vc's count treated `n-a` as satisfied. `contract.rs` says the opposite in as many words: _`to-write` and `red` are not coverage, and neither is `n-a` ... treating it as green is what let a contract look closed while nothing had been run_, and for a non-test row satisfaction lives on the AC's own line.** Both those lines read `unsatisfied` (AC-00.3 is vc's conservation verdict; AC-05.3 is the path-printing Highlander). **So the honest shape is 62 TEST ROWS THAT MUST GO GREEN -- 46 are -- PLUS 2 NON-TEST CRITERIA THAT MUST BE SATISFIED ON THEIR OWN LINE, AND NEITHER IS.** A row-count gate of "64 green" was never reachable: two of its rows cannot go green by construction. **This is vc's own class, in the harder shape -- an instrument existed and it compared the wrong pair.**

**What v3 does:** keeps the project in a SQLite database, and keeps a chosen subset of it on disk as real files. `intent organize` makes disk match the choice.

**THE DESIGN hv RULED TONIGHT, and it replaced the one that was there:**

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds the id, `st done` removes it, `st hydrate`/`st dehydrate` do it directly, a human may edit it. **Nothing derives it from status.** That is the only difference from the old two-region design, and it is why the protected region became unnecessary -- **a write is a change to state, never a regeneration of it.** It also explains why `intentfiles::render` had no caller: the thing it does is not needed.

**_AUTHORED_ WAS vc's WORD FOR THIS AND IT WAS WRONG.** hv corrected it before either builder committed to a shape. It does not mean untouched by commands; it means nothing recomputes it.

## THE ONE THING THAT STOPS THE PROJECT RUNNING ON v3 TONIGHT

**`intent at green` / `at red` REWRITE CANON AND LEAVE THE COMMITTED VIEW SAYING THE OPPOSITE, AT rc=0, ON EVERY USE.** Clone `8b81b871`, binary `b99789aff383f8bb`: `at red ST0057 AT-00.1` printed `ok: AT-00.1 -> red`, canon sha `092bdc09 -> 2dbfb769`, view sha `84527f99 -> 84527f99`, row still reading `status: green`.

**LIVE: `intent/st/ST0057/acceptance.md` DISAGREES WITH CANON ON 7 OF 46 ROWS, BOTH DIRECTIONS, FROZEN SINCE `35cfb080` (19:00 local), AND IT SAYS _ST0057 IS 0 OF 8 WPs_.** Three read green where canon says red (AT-02.2, AT-02.3, AT-05.2), four read `to-write` where canon says green (AT-02.4, AT-06.1, AT-06.2, AT-08.1). **ST0056's view agrees on all 132, so the generator is fine** -- ST0057 went nine status commits without a view write.

**NOTHING REPAIRS IT TODAY, AND EACH DOOR WAS DRIVEN RATHER THAN READ.** `sync --to-disk ST0057` writes the extract only -- **a marker line appended to `acceptance.md` SURVIVED the sync**, drift 7 before and 7 after. `organize --apply` writes views and is gate-blocked. **`st hydrate ST0057` is `unrecognized subcommand`** -- declared in dc's table, absent from the binary, and it is now the only repair path there is.

**THE SKEW GUARD (cc, AC-03.4) CANNOT FIRE ON IT:** it reports _no generated view was touched by this change -- nothing to check_, because it compares views IN THE COMMIT against canon and no verb puts a view in a commit. **vc stopped on that line twice today.**

**THE QUESTION FOR hv, AND IT MAY BE A DESIGN RATHER THAN A DEFECT:** under _the DB is the SSOT and disk is a projection_, a status change updating only the SSOT is correct and the projection refreshes at `organize`. **But the views are COMMITTED, so between organize runs git holds a face that contradicts the truth, and that face is what a human opens.** Either the status verbs project the one thread they touched, or `st hydrate` must exist before anyone runs the project on v3 for real.

**`intent st new` ALSO WRITES NOTHING TO DISK AND NOTHING TO `.intentfiles`** (clone: `created: ST0058`, no `intent/st/ST0058/`, manifest still 0 non-comment lines) -- the gap ic and dc are closing.

## WHO IS ON WHAT

- **ic** -- the mechanism. `organize` reads the list and makes disk match BOTH ways; the file loses its regions; **their three-state missing-vs-empty handling stays exactly as built and matters MORE now** (missing = keep everything, empty = keep nothing; it is the difference between a fresh clone working and a fresh clone deleting itself).
- **dc** -- six new dispatch rows (`st hydrate`, `st dehydrate`, `st new --dehydrate`, `st done --keep`, the two `issues` equivalents) plus the lifecycle verbs reclassified as writers of the file. **Separately: WP-06, which is four of the six arming preconditions.**
- **cc** -- AC-03.6 (needs `--staged`, built, awaiting dc's review) and AC-00.4 (`ROOT_FILES`).
- **vc** -- AC-00.3, the conservation verdict at a pinned revision. Hold the ledger. Verify every claim rather than take it.

**THE LIMB WITH TEETH, NEW TONIGHT: `st done` and `st cancel` now DELETE FILES, which they never did.** They must run the unsynced-attachment check and **REFUSE, NAMING THE PATHS**, when the artefact holds on-disk bytes the store has never seen. `Facade::sync_uncommitted` already answers it (cc's AC-03.5). **The database regenerates what it knows, never what somebody typed and did not sync.**

## THE ARMING LEDGER -- vc OWNS THIS NUMBER

**Measured from the shipped gate, never inferred: `intent organize --apply` in a disposable clone, refusal read directly.** Each node tracks its own preconditions correctly and **the subsets do not sum to nineteen** -- that is why one node has to own the union.

**At `8f9ba24a`, 2026-08-19 20:14Z: `19 checked of 19 declared, 6 unmet`, 545 removals planned.** Thirteen at 16:52Z, eight at 18:18Z, six now.

    dc   4   AC-06.1..06.4 -> now AC-06.3, AC-06.4   WP-06
    cc   2   AC-03.6, AC-00.4
    ic   1   AC-07.5
    vc   1   AC-00.3

**545 REMOVALS BECAUSE THE LIST IS STILL EMPTY.** Nothing has been declared yet; that is the work in flight. **When the last precondition lands the gate opens by itself, so dc stops and tells vc rather than letting it happen unattended.** What makes it survivable meanwhile: `organize` previews by default and `--apply` is a second deliberate act.

## WATCH-OUTS THAT COST REAL WORK TODAY

- **EACH SYNC DIRECTION DESTROYS WHATEVER EXISTS ONLY ON THE OTHER SIDE, AND vc HAD THE LABEL BACKWARDS.** `--to-disk` destroys unsynced DISK edits; `--to-store` destroys unprojected STORE state -- and it is `sync_from_disk` that the code itself calls _the DESTRUCTIVE direction_, under the reversed D01. vc edited a fix on disk, committed it, and was one routine sync from losing it. **`intent sync --to-store <ID>` before any verb, and know which side your change is on.**
- **`intent at green` REWRITES THE WHOLE DOCUMENT FROM THE STORE.** A hand edit to canon made at ANY time before the command is discarded at rc=0 -- `load_fresh` returns from the store and never reads the files, and `ingest.rs:301` promises the opposite in its own summary line.
- **COULD THIS MEASUREMENT HAVE COME BACK THE OTHER WAY?** `head -30` over 63 test binaries cannot report a red, so "fully green" was never a possible finding. A bare `render(` cannot tell `intentfiles::render` from `views::render`, so its 14 was never evidence.
- **NEVER `$?` AFTER A PIPE** -- vc did it three times today, twice on its own documented rule. **`grep` here is ugrep and a `{...}` pattern can silently match nothing.**
- **NAME REVISION, CLOCK AND DIRTY COUNT ON EVERY MEASUREMENT.** Four nodes write faster than the tree can be measured: three readings of one build inside two minutes, all correct.
- **A GREEN MEASURED IN A DIRTY TREE AND RECORDED AGAINST A SHA IS A CLAIM THAT REVISION CANNOT SUPPORT**, and nothing downstream can detect it.

## THE ONE CLASS EVERYTHING ELSE TODAY WAS AN INSTANCE OF

**TWO ARTEFACTS DISAGREE AND NO THIRD THING READS BOTH** (Lamplight's vc named it). Two sub-shapes: **between two artefacts that never meet**, and the harder one, **a third thing exists and compares the WRONG PAIR** -- harder because a reader checking for the class finds an instrument already there and stops.

**The sharpest instance was inside one node's own work: ic recorded _there is no `intentfiles::render` call anywhere in `organize.rs`_ in one row's note while another row sat green on the opposite claim, one screen away, for an hour** -- and it passed under two validation reads including vc quoting it back approvingly.

## QUEUED AND DELIBERATELY NOT BUILT

Nothing outside the 64 gets started before the release. **The best candidate is ic's: nothing cross-reads a row's EVIDENCE against another row's STATUS, and both inputs are already committed text.** Also: the critic scans whole staged files rather than added lines and taxes the person documenting a fix; no edge exists from _a capability landed_ to _a decision waiting on it_; `organize`'s stdout says `0 removed` on a run planning 545; a manifest PARSE ERROR is indistinguishable from an ABSENT one; AT-03.15's remaining debt is a second CLI-level instrument, not 31 more cases; `st edit`'s default target is a generated view its own ruling must refuse.
