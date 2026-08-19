---
node: ic
name: Interface Claude
role: interface
session_id: 7c9b8dad-5c1f-49af-a9fd-9dbd287fc26d
heartbeat_at: 2026-08-19 21:41Z
status: paused
focus: "**AGGRESSIVELY FOLDED FOR EOD ON hv's INSTRUCTION. NOTHING OF MINE UNCOMMITTED.** Four green today, all mutation-proven: AT-02.2, AT-02.3, AT-07.5, plus the directory prune as a code fix. **NEXT: the `st hydrate` render arm -- render.rs is clear at `8e544de4` and the spec is landed at `8a6ae532`; it is a two-line call.**"
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** Everything of mine is committed; the tree's dirt is generated files and peers'.

## ON RESUME -- read this first

1. **TAKE THE `st hydrate` RENDER ARM FIRST. IT IS TWO LINES AND EVERYTHING UNDER IT EXISTS.** `address::promote(arg)?` then `facade.hydrate(&addr)`. `render.rs` is clear (dc, `8e544de4`); `promote` is landed with 6 tests and 3 red arms (`8a6ae532`); `Facade::hydrate` is built and pinned to `Mode::Apply`. **Pass the WHOLE STRING to `promote`, never an id extracted from it** -- rebuilding the URL from its id is the spelling that reads fine and silently turns a cross-project reference into a local one, and hydrate's refusal never fires because the authority is gone before it is called.
2. **THEN RULE THE ROSTER, BECAUSE IT IS MINE AND IT IS FOUR ROWS NOT ONE.** dc found `issues dehydrate in 0 buckets` UNDERSTATES: the assert fires on the first unbucketed verb and stops. **`st hydrate`, `st dehydrate` and `issues hydrate` are unbucketed too**, the ratchet refused at 36 against a cap of 32, and dc reverted rather than re-bucket on my roster at midnight. **COVERED_ELSEWHERE is defensible for `st hydrate` ONCE IT LANDS** -- they write through `Plan::run`, which `organize_idempotent_mtime.rs` measures -- **and dishonest for the three that do not exist.**
3. **DO NOT REBUILD TODAY'S WORK.** AT-02.2, AT-02.3, AT-07.5 and the prune are landed, verified end-to-end by vc on the live estate, and mutation-proven.

## TODO -- LIVE ONLY

1. **`st hydrate` render arm** (see ON RESUME 1).
2. **Roster ruling on the four unbucketed hydrate/dehydrate verbs** (see ON RESUME 2).
3. **AC-05.2: the lifecycle verbs edit the list.** Build the WARNING, not a gate -- vc retracted the refuse clause at `9b887765`, and `organize.rs:695` is the only line in the tool that removes an estate file. `Facade::sync_uncommitted` answers the unsynced-bytes question exactly.
4. **`intent init` HAS A DESIGN DECISION INSIDE IT AND THE COMMAND DOES NOT EXIST YET, WHICH IS THE CHEAPEST MOMENT TO MAKE IT.** A project declaring NO preconditions cannot dehydrate at all -- `organize --apply` answers _0 checked of 0 declared, so nothing is proved and nothing may be removed_. Fail-closed; absence is not permission. **So a freshly initialised project is born unable to dehydrate until someone declares one.**
5. **THE `st edit` FORK IS UNRULED AND `edit_writes_pinned_region.rs` STILL ASSERTS THE RETIRED TWO-REGION ARCHITECTURE** behind a red row. vc's ruling: tolerable while a red row names it, **not tolerable the moment someone greens that row without deleting the file.** hv's, not mine.
6. **hv, FOR TOMORROW: 250 files under `intent/` are not in the store at all** -- `docs/`, `llm/`, `history/`, `eng/`, `plugins/`, project-level `done.md` / `wip.md`. _"Not all of that should be in the db, but certainly some of it should."_

## Watch-outs

- **A MUTATION BATTERY IN A SHARED CHECKOUT GIVES A TREE THAT COMPILES AND LIES.** A peer mid-edit gives you one that will not compile or fails honestly; a battery gives you a green that is false. dc's `852/1` measured a tree my arm had broken, and dc later swept a live mutant into HEAD under cc's commit message. **Isolation needs a consistent snapshot and there is not one while four nodes hold interleaved work.** Restore on SIGTERM/atexit, check the baseline compiles AND passes before injecting, and **measure the exit status WITHOUT A PIPE** -- `bash "$F" | tail -3; echo $?` reports tail's.
- **THE BASH TOOL'S SHELL IS ZSH AND IT BIT ME TWICE TONIGHT IN TWO DIFFERENT WAYS.** Unquoted `$var` does NOT word-split, so a probe loop passes `st show ST0001` as ONE argument and records `unrecognized subcommand` -- which reads exactly like a real failure. And **backticks inside a double-quoted `-m` are COMMAND SUBSTITUTION**: a commit landed at exit 0 with a sentence missing its subject. Use arrays or `bash -c` for probes; feed commit messages from a quoted heredoc through `-F -`.
- **A GREEN CAN BE A FACT ABOUT THE ESTATE RATHER THAN ABOUT THE PROPERTY.** Deleting the prune's floor left all five integration tests green, including the one asserting the estate root survives -- the root was protected by `steel_threads.md` happening to live in it, not by the bound the code claims. **A bound that is never reached is not a bound that was tested.** Same class as AT-07.5's behavioural arm being green because the daemon does not exist, and vc's skew guard reporting `nothing to check` all evening.
- **AN ASSERTION CAN PASS ON ITS OWN INPUT ECHOED BACK**, and a FILTER WITH NOTHING TO EXCLUDE IS AN UNTESTED BRANCH. `said.contains("ac")` was satisfied by the URL in the error; my sigil filter survived a mutation because the fixture held no `ISSUE:` line. **Fix the fixture, not the note.**
- **A TRUE MEASUREMENT FILED WHERE NOTHING READS IT DOES NO WORK.** vc's general form: two artefacts disagree and no third thing reads both. **Live instance tonight: vc told me `Report.pruned` is unrendered while dc had already rendered it inside the 211 lines vc could not see.** Check the file, not the report.
- **`git commit --only <path>` COMMITS THE WORKING-TREE STATE OF THAT PATH** -- it defends against the INDEX, not against a peer editing the same file.
- **STILL TRUE: run the workspace not the crate; every restore path absolute; a mutant that does not compile is not a red; a mutant that changes no behaviour is not a survivor; `stat` without `-u` prints LOCAL.**

## Decisions

- (2026-08-19) **`intent hydrate <address>`, WITH A BARE ARTEFACT ID PROMOTED TO ONE.** The argument is an address because the SERVICE refuses in address terms -- two of `Facade::hydrate`'s three refusal arms are unreachable from a bare id. The promotion is safe because it **DELEGATES rather than guesses**: `is_thread_id` / `is_issue_id` own the fact and `Sigil::accepts` already calls them. **An inference from SHAPE that reads a spelling is the class `intrinsic` exists to kill; one that calls the module owning the fact is not.** A malformed argument is a USAGE error naming both forms, never a not-found.
- (2026-08-19) **A REALISED ARTEFACT IS ONE WHOSE COVER VIEW EXISTS, NEVER ONE WHOSE DIRECTORY DOES** (vc's rule, from my wrong predicate). Dehydration removes files; it now also removes the directories it emptied, `rmdir` semantics only.
- (2026-08-19) **FOUR CRITERIA LEFT THE DECLARED PRECONDITION BLOCK WITHOUT BEING WITHDRAWN.** AC-03.6, AC-06.3, AC-06.4, AC-07.5. **The block is about what GATES, not about what is wanted** -- every one is still owed as ordinary work. A note saying "they came off the gate" reads as done.
- (2026-08-19) **hv's GIT GROUNDS RETIRE A PRECONDITION ONLY WHERE GIT CAN SUBSTITUTE FOR THE PROOF.** AC-00.3 was a reversibility proof and git substitutes exactly. AC-07.5 is an ACCESSIBILITY claim and **reaching for git falsifies its subject** -- restoring the estate re-hydrates it. Withdrawability is not a property of being a precondition.
