---
node: vc
name: Validation Claude
role: validation
session_id: 590c4fbc-ea99-41b3-9c10-75344a715f96
heartbeat_at: 2026-08-19 21:41Z
status: paused
focus: "**THE DEHYDRATION SHIPPED. 423 files out, `intent/st` holds ST0046, ST0056, ST0057, and the round trip is byte-identical against git.** Gate 50 of 64. The precondition block is 14 and reads 0 unmet -- hv cleared four on one word, and NONE of them is withdrawn: they are still owed as ordinary work. Tomorrow is the 250 files under `intent/` the store has never heard of."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## ON RESUME -- PLAIN WORDS, READ THIS FIRST

**v3 keeps the project in a database and keeps a chosen subset of it on disk as real files. `.intentfiles` is the list of what is on disk. `intent organize` makes disk match the list.** That is the whole architecture and it is now RUNNING, not designed.

**WHAT SHIPPED TONIGHT.** `organize --apply` removed **423 files** at `e7f00e65`. `intent/st/` now holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database.

**AND IT IS PROVEN REVERSIBLE, MEASURED RATHER THAN ARGUED.** ST0001 back onto the list -> five files returned, **all five byte-identical to what git holds**. A fence-heavy pair (ST0016 at 48 fence markers, ST0034 at 180 non-ASCII chars) -> **fifteen files, all byte-identical**. All 282 attachments in canon verify against their own `sha256` and byte count. **Dehydration is not a loss.**

**GATE: 50 of 64.** Green tonight: AT-00.1, AT-00.4, AT-02.2, AT-02.3, AT-03.1--03.5, AT-04.1--04.5, AT-06.1, AT-06.2, AT-07.5. It was 33 this morning.

## THE PRECONDITION BLOCK IS 14 AND READS 0 UNMET -- READ THIS BEFORE ASSUMING ANYTHING IS DONE

**hv cleared four preconditions out of AC-00.1's declared block on one word (_Word_), and AC-00.3 earlier on the git ruling.** The block went 19 -> 18 -> 14.

**NOT ONE OF THEM IS WITHDRAWN EXCEPT AC-00.3. AC-03.6, AC-06.3, AC-06.4 and AC-07.5 REMAIN LIVE CRITERIA AND ARE STILL OWED.** The distinction is dc's and it is the useful half of the whole exercise: **the question was never whether the work is wanted, it is whether a GATE should hold on it.** A board that records "they came off the gate" reads as done to whoever opens it next, which is why this section exists.

    withdrawn   AC-00.3   migration-conservation verdict -- a safety proof git substitutes for exactly
    still owed  AC-03.6   cc -- DETECTION of canon/disk divergence. Git preserves files; it does not
                          stop canon and disk disagreeing
    still owed  AC-06.3   dc -- CAPABILITY. the third `Projection` variant, `md` sayable not refused
    still owed  AC-06.4   dc -- `intent init` from an empty dir. Reversibility proof AS A GATE, but
                          independently required as HOSTING work: a tool that cannot create a project
                          is not one anyone can host on
    still owed  AC-07.5   ic -- MET. green tonight

**THE TEST FOR WITHDRAWING A PRECONDITION, AND ic's HALF IS THE BETTER HALF:** hv's git grounds retire a precondition **only where git can SUBSTITUTE for the proof**. ic: _restoring the estate from git RE-HYDRATES it, which destroys the precondition under test_ -- reaching for git falsifies the subject, so an ACCESSIBILITY claim is never withdrawable on those grounds and a SAFETY claim usually is.

## FOR TOMORROW, hv's OWN QUESTION

**250 files under `intent/` are not in the store at all** -- `docs/` (12), `llm/` (14), `history/` (18), `eng/` (10), `plugins/` (191), `autopsy/`, `analysis/`, and the project-level `done.md`, `wip.md`, `restart.md`, `todo.md`. The store holds **threads, work packages and issues, and nothing else** (`doc_sections.owner_type` is exactly those three).

**hv: _not all of that should be in the db, but certainly some of it should. A job for tomorrow._**

**THE TRAP IN MEASURING IT: there IS a `done.md` in the store -- ten of them -- and they are `intent/st/ST0019/done.md` and siblings, per-thread attachments.** A grep for `done.md` returns hits and the project-level file is not among them.

**The same gap from the command side is dc's measurement: 16 of 32 top-level families dispatch, 14 answer exit 2, `intent claude` implements 1 of its 8 verbs, against 230 `intent claude <verb>` call sites in this repo's own machinery.** Everything that manages STEEL THREADS is done; everything that manages INTENT ITSELF is not. **Two faces of one gap, found from opposite ends on the same evening.**

## WHAT IS OWED, BY NODE

- **cc** -- AC-03.6 (built, waiting on dc's roster ruling). Carrying: the skew guard's real scope; `intent doctor` sees view skew and is not wired to the gate; the unclaimed-report digest.
- **dc** -- AC-06.3, AC-06.4, `st hydrate`'s render arm, the `organize` summary/stderr split, and the hosting sweep hv redirected them onto.
- **ic** -- `Report.pruned` is not rendered, so a destructive act is invisible at the CLI. The `st edit` fork is unruled and `edit_writes_pinned_region.rs` still asserts the retired architecture behind a red AT-05.2.
- **vc** -- hold the ledger. **ST0057/WP-09 is filed and unstarted: the event log records the MODEL and not the DISK.**

## WP-09, BECAUSE IT IS MINE AND IT IS NEW

**`Facade::apply` (`facade.rs:3504`) is a real chokepoint and a good one** -- it DIFFS `next` against loaded canon rather than trusting a caller's list, because the declared-list version made _the mutation did not persist_ reachable by naming the wrong id while the DB and the return value both said success.

**`grep -c 'apply(' organize.rs` returns 0.** `organize`, `sync_to_disk`, `sync_from_disk` and `hydrate` all write the filesystem without passing it. **So 423 files left this estate tonight and the log recorded nothing.** The only act all evening that destroyed anything is the only class absent from the one table that cannot be re-derived from anything else on disk.

**Two non-fixes are written into the WP.** Do not route `organize` through `apply` -- it diffs canon and organize changes no canon, so the event would be a lie about its own mechanism. Do not add a pruner -- 55 events in 35 hours at ~229 bytes is under 4MB a year. **The thing worth deciding while there are 55 rows rather than 17,000 is concurrent append**, a tracked append-only file written by four sessions minutes apart.

## WATCH-OUTS THAT COST REAL WORK

- **EACH SYNC DIRECTION DESTROYS WHATEVER EXISTS ONLY ON THE OTHER SIDE, AND vc HAD THE LABEL BACKWARDS.** `--to-disk` destroys unsynced DISK edits; `--to-store` destroys unprojected STORE state, and it is `sync_from_disk` the code itself calls _the DESTRUCTIVE direction_. **`intent sync --to-store <ID>` before any verb, and know which side your change is on.**
- **`at green` MOVES A STATUS AND NO VERB MOVES A NOTE.** Twice tonight a row read green above prose saying the test does not exist. Canon prose routes through vc; the builder commits the contradiction and names it rather than hand-editing canon.
- **THE REVISION IS PART OF THE FINDING, NOT CONTEXT FOR IT** (ic). Two of my reads tonight were true of a tree one rebuild or one mid-write file out of date. **Name revision, clock and dirty count on every measurement.**
- **NEVER `$?` AFTER A PIPE. `grep` here is ugrep. The Bash tool's shell is zsh, so an unquoted glob in `--include=*.rs` is a hard error and PATH can collapse inside a `while read` loop** -- that last one reported three fake sha256 mismatches before I noticed `jq: command not found` in the same output.
- **COULD THIS MEASUREMENT HAVE COME BACK THE OTHER WAY?** I reported ST0056's generator fine because its 132 rows agreed; `doctor` showed the file stale by 1813 bytes. **Rows agreeing said nothing about bytes.**

## THE ONE CLASS, AND IT GOT SHARPER TONIGHT

**TWO ARTEFACTS DISAGREE AND NO THIRD THING READS BOTH** -- and the harder sub-shape, **a third thing exists and compares the WRONG PAIR**, because a reader checking for the class finds an instrument already there and stops.

**Four instances tonight, three of them in instruments that were reporting green.** The skew guard printed _nothing to check_ for hours while ST0057's committed view contradicted canon on 7 of 46 rows. cc's roster check went green gated on a wiring that judged the wrong commit. ic's floor filter was protected by `steel_threads.md` happening to exist rather than by the bound the code claims. And my own gate count read `n-a` as passing for hours while `contract.rs` says in as many words that it never is.

**THE PROGRESSION WORTH KEEPING IS THE FOURTH: cc PREDICTED an instance instead of finding one.** _A report whose first two hundred lines are byte-identical on every run trains its reader to stop looking._ **I drove it in thirty seconds and it is false for ADD (the count moves) and true for SWAP -- the real boundary is CONSTANT CARDINALITY**, which is a thing dc can build against and the loose sentence was not. **A named class earns its keep when it makes the next instance cheap to pin, and the difference was falsifiability rather than luck.** cc's proposed remedy -- count plus distinct directories -- **failed the property cc themselves stated**, since a same-directory swap leaves both untouched. The digest closes it.

## QUEUED AND DELIBERATELY NOT BUILT

Nothing outside the 64 gets started before the release. **Best candidate remains ic's: nothing cross-reads a row's EVIDENCE against another row's STATUS, and both inputs are already committed text.** Also: the critic scans whole staged files rather than added lines; no edge exists from _a capability landed_ to _a decision waiting on it_; `organize`'s stdout still says `0 to remove` beside a stderr saying 423; a manifest PARSE ERROR is still indistinguishable from an ABSENT one; AT-03.15's debt is a second CLI-level instrument, not 31 more cases.
