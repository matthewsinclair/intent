---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 07:04Z
status: active
focus: "**`st hydrate` IS WIRED AND THE ROSTER IS GREEN (`21c0ccf2`); the FORMAT leg is closed (`0a9a7341`).** The find of the morning is a LAYER one: `Facade::hydrate` resolves a thread's realisation home to the ESTATE and an issue's to **CANON**, because an issue has no realised form at all -- driving it wrote `ISSUE:0001` into the live manifest and printed `ok` over 0 files. Backed out, escalated to hv. **NEXT: AC-05.2, the lifecycle verbs edit the list -- and `--dehydrate` / `--keep` are DOCUMENTED NO-OPS today.**"
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** `21c0ccf2` and `0a9a7341` are in; my three files are committed and the tree's dirt is peers'.

## ON RESUME -- read this first

1. **AC-05.2 IS THE NEXT UNIT AND IT IS BIGGER THAN THE ROW READS, BECAUSE TWO FLAGS ARE ALREADY LYING.** `intent st new --dehydrate` says _"Create the thread without listing it in .intentfiles, so no files are written"_ and `intent st done --keep` says _"Close the thread but leave its entry in .intentfiles, so its files stay on disk"_. **NOTHING READS EITHER FLAG** -- `grep -rn 'dehydrate|flag(a, "keep")' crates/intent-cli/src/` returns only `report.dehydrated` in the organize renderer, and no lifecycle verb touches `.intentfiles` at all (`st_new` at `facade.rs:2193`, `st_done` a one-line `set_thread_status`). **EACH FLAG'S PROMISED EFFECT IS IDENTICAL TO THE CURRENT DEFAULT**, so an operator gets what the help says for the wrong reason and no outcome-only test can see the difference. Building AC-05.2 retires both; it does not merely add a feature.
2. **`intentfiles::pin` EXISTS AND ITS INVERSE DOES NOT.** AC-05.2 needs an `unpin` before `st done`/`st cancel` can remove an entry.
3. **DO NOT REBUILD THIS MORNING'S WORK.** `st hydrate` (both doors written, issues door deliberately shut), the roster report + assertion + the new `DECLARED_BUT_UNWIRED` bucket, and `declared_but_unwired.rs` are landed and mutation-proven in both directions.

## TODO -- LIVE ONLY

1. **AC-05.2: the lifecycle verbs edit the list** (see ON RESUME 1 and 2). Build the WARNING, not a gate -- vc retracted the refuse clause at `9b887765`, `organize.rs:695` is the only line in the tool that removes an estate file, and `Facade::sync_uncommitted` answers the unsynced-bytes question exactly.
2. **`Facade::dehydrate`, then `st dehydrate`.** Constraint that must hold: it unlists and goes through `Plan::run`. It must NOT grow a second deletion site -- that is the argument that withdrew the `refuse` clause, applied to the verb itself.
3. **AC-05.1 / AT-05.1: `intent edit <ID>`.** Blocked on hv's `st edit` fork ruling, and `edit_writes_pinned_region.rs` (AT-05.2's file) still asserts the retired two-region design behind its red row.
4. **WAITING ON hv: does an issue have a realised form?** Three components disagree today -- the grammar accepts `ISSUE:`, `Facade::hydrate` would write it, and `intentfiles::realised()` silently drops it (vc). Until ruled, `issues hydrate`/`issues dehydrate` stay refusing at rc=2.
5. **AC-08.4 / AC-08.5** (WP-08) are still red and untouched today.

## Watch-outs

- **`--only <paths>` SEPARATES FILES, NOT AUTHORS** (vc's rule, and I proved it the hard way this morning). `0a9a7341` was scoped to 20 paths and landed 21 files: `tests/unit/devbin_rust_gates.bats`, dc's hand-written #348 fix, which was uncommitted under a path my sweep named. **My commit body says "no semantics" over it.** Third instance in this estate. **Name the paths AND read `git diff -U0` before committing; a hunk you did not write is a stop.**
- **NEVER DRIVE A MUTATOR ON THE LIVE ESTATE TO SEE WHAT IT DOES.** `intent issues hydrate 0001` wrote `ISSUE:0001` into `intent/.intentfiles`. I reverted it, but a probe is not a test and the estate is not a fixture. The finding was real; the method was wrong and only luck made it recoverable.
- **A WORKSPACE-WIDE `cargo fmt` IS SAFE ONLY WHILE NOTHING UNDER `native/` IS DIRTY, AND THAT PROPERTY IS PERISHABLE.** It held at `483fbcfe` and expired within the hour when vc opened `views.rs`. When a peer IS mid-edit, format your own files with `rustfmt <file>` and never `cargo fmt`.
- **`intentsvcs` IS THE DEPENDENCY ROOT: A PEER MID-EDIT THERE STOPS ALL THREE OF US.** vc's `views::skew` gaining a parameter left `doctor.rs:753` behind and nothing in the workspace compiled for tens of minutes. Not a defect -- but announce the BLAST RADIUS, not just the files (vc's own conclusion).
- **A MUTATION BATTERY IN A SHARED CHECKOUT GIVES A TREE THAT COMPILES AND LIES.** Isolation needs a consistent snapshot and there is not one while four nodes hold interleaved work. Restore via `trap ... EXIT` with absolute paths, and **measure the exit status WITHOUT A PIPE** -- `cmd | head; echo $?` reports head's, which reported `rc=0` on four verbs that were exiting 2.
- **THE BASH TOOL'S SHELL IS ZSH.** Unquoted `$var` does NOT word-split; use `bash -c` for probes. Backticks inside a double-quoted `-m` are command substitution -- feed commit messages from a quoted heredoc through `-F -`. **And the tool's working directory PERSISTS between calls**: a `cd` in one call silently relocated three later ones and produced `rc=127` and `ugrep: No such file`, both of which read exactly like real failures. Use absolute paths.
- **A GREEN CAN BE A FACT ABOUT THE ESTATE RATHER THAN ABOUT THE PROPERTY**, and **AN ASSERTION CAN PASS ON ITS OWN INPUT ECHOED BACK**. Every new bucket and every new refusal needs a positive control that is not the thing under test.
- **A TRUE MEASUREMENT FILED WHERE NOTHING READS IT DOES NO WORK.** The roster printed 65 under a denominator of 69 and asserted ONE unbucketed verb while four were. Neither line was false; nothing read both.
- **STILL TRUE: run the workspace not the crate; a mutant that does not compile is not a red; a mutant that changes no behaviour is not a survivor; `ls`/`stat` without `-u` print LOCAL; `grep -c` exits 1 on zero so a `||` fallback fires on a true zero; and an `&&` chain after a zero-match grep silently skips the next command.**

## Decisions

- (2026-08-20) **AN ISSUE HAS NO REALISED FORM, SO `issues hydrate` STAYS REFUSING UNTIL hv RULES.** `Facade::hydrate` resolves a thread's home to `thread_dir(id)` (ESTATE) and an issue's to `issues_dir()` = `canon_dir().join("issues")` (**CANON**) -- two arms of one match addressing two layers -- and it resolves that way because canon is the only issue path that exists. `views.rs` renders no issue view; every `Project` issue accessor is canon-side. **Inert today only because `organize::plan` emits no step under `intent/.canon/`, which is a property of the plan and not a bound `hydrate` states.** Fail closed: a verb that pins an unrealisable artefact and prints `ok` over 0 files is worse than one that says it is unbuilt.
- (2026-08-20) **`COVERED_ELSEWHERE` FOR `st hydrate` NAMES `facade_hydrate.rs`, NOT `organize_idempotent_mtime.rs`.** Hydrate runs a plan FILTERED to one artefact; the estate-wide file covers the mechanism and not the scoping, so citing it would name a file that cannot fail when this verb regresses. A citation that cannot go red is not a cover.
- (2026-08-20) **A DECLARED-BUT-UNIMPLEMENTED VERB IS ITS OWN BUCKET, AND THE BUCKET IS DRIVEN.** Its writes are not UNPROVEN, they are provably EMPTY -- the binary refuses at rc=2 -- and filing a known zero as debt inflates the debt and hides the fact. `declared_but_unwired.rs` runs each one and requires the refusal, so implementing one turns it RED and forces the re-bucket. **It measures the CAPABILITY, never a name**, which is the distinction AC-08.5 paid for.
- (2026-08-19) **`intent hydrate <address>`, WITH A BARE ARTEFACT ID PROMOTED TO ONE.** Landed and driven `21c0ccf2`. The promotion DELEGATES rather than guesses: `is_thread_id` / `is_issue_id` own the fact. A malformed argument is a USAGE error naming both forms, never a not-found.
- (2026-08-19) **A REALISED ARTEFACT IS ONE WHOSE COVER VIEW EXISTS, NEVER ONE WHOSE DIRECTORY DOES** (vc's rule). Dehydration removes files; it now also removes the directories it emptied, `rmdir` semantics only.
- (2026-08-19) **FOUR CRITERIA LEFT THE DECLARED PRECONDITION BLOCK WITHOUT BEING WITHDRAWN.** AC-03.6, AC-06.3, AC-06.4, AC-07.5. **The block is about what GATES, not about what is wanted** -- every one is still owed as ordinary work.
- (2026-08-19) **hv's GIT GROUNDS RETIRE A PRECONDITION ONLY WHERE GIT CAN SUBSTITUTE FOR THE PROOF.** AC-00.3 was a reversibility proof and git substitutes exactly. AC-07.5 is an ACCESSIBILITY claim and **reaching for git falsifies its subject**.
