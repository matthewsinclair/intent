---
node: dc
name: DevX Claude
role: worker
session_id: f396ca31-ec6f-459e-9b7c-40e87aa93efb
heartbeat_at: 2026-08-19 21:41Z
status: active
focus: "**THE ARMING BLOCK IS GONE -- the estate dehydrated at `e7f00e65`, 423 files out, block reads 0 unmet. TOMORROW IS THE HOSTING GAP, ruled by hv.** Measured on the built binary: **32 top-level families, 16 dispatch, 14 answer 2**, and `intent claude` implements 2 of 8. Against that, this repo's own machinery makes **230 `intent claude <verb>` calls of which 9 are `hook`** -- 125 were `claude rules`, which landed tonight. **Everything that manages steel threads is done; everything that manages Intent itself is not.**"
claims: [ST0056/11, ST0057/04, ST0057/06]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap two writers interleave in.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION HOLDS.** A time-typed input parameter is a defect by inspection.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it is just a label".
- **A board stamp is a label, not data.** The ordering that exists and cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The db is the durable SSOT, files are re-creatable; the typed API is the only door in; migrations are normal.** As of tonight that is not a plan -- `intent/st` holds ST0046, ST0056, ST0057 and nothing else.

## DOING

Nothing in flight. Everything of mine is committed; `render.rs` is clear for ic.

## TODO

1. **THE HOSTING GAP IS TOMORROW'S JOB (hv).** Fourteen families answer 2: `version` `config` `init` `bootstrap` `agents` `critic` `lang` `learn` `modules` `plugin` `ext` `fileindex` `daemon` `mcp`. **`intent claude` implements `hook` and `rules`; `skills` (27 calls), `subagents` (29), `upgrade` (19), `ws` (12), `start` (5), `prime` (4) do not.** Highest leverage after `rules`: **`critic` (36 calls, and it IS the pre-commit gate)**, then `agents` (40).
2. **THE GATE WORKS TODAY ONLY BECAUSE IT RUNS v2, AND THIS IS THE TRAP IN THE WHOLE HOSTING PLAN.** v2's version guard is scoped to WRITES, so `intent critic shell --files ...` runs clean at rc=0 on this v3 tree while `intent st list` refuses at 2. **The day v3 goes on PATH, `intent critic` answers 2 -- the gate own fail-open code -- in all five declared languages, here and in the other 15 projects reached through one symlink. Build `critic` BEFORE anything touches PATH.**
3. **TWO RULINGS HELD FOR hv, both deliberately not taken:**
   - **Ext rule packs.** v2 serves `~/.intent/ext`; v3 does not. I wrote the resolver, `no_intent_home::the_shipped_surface_reads_exactly_one_environment_variable` refused it (`ALLOWED` is `["COLUMNS"]`; mine read three), and **I deleted the module rather than allowlist myself through** -- that test says in its own failure message that a further read needs an hv ruling and not a quiet addition, because every machine here has the variable set so nothing else would fail. The seam is a parameter: ext support is one argument.
   - **`claude rules validate` / `index`** stay at 2. The row is `pending-hv` on whether `index` retires with the on-disk rules root. `list`/`show` carry no part of that question, which is exactly why they shipped.
4. **AC-06.3 AND AC-06.4 ARE STILL OWED AS CAPABILITY, NO LONGER GATING.** AC-06.3 is the third `Projection` variant -- `md` stops being REFUSED and stays non-authoritative, **the `because` clause survives verbatim and only `instead` goes.** AC-06.4 is `intent init`, which is item 1 work under a different name.
5. **`issues dehydrate is in 0 buckets` UNDERSTATES BY FOUR, AND THE ROWS ARE MINE.** `st hydrate`, `st dehydrate` and `issues hydrate` are unbucketed too; the assert stops at the first. Adding all four trips the UNPROVEN ratchet at 36 against a cap of 32 -- **the roster correctly declining to absorb four rows that are ahead of their mechanism.** Left as found; ic owns the roster and is landing `st hydrate`.
6. **Standing, unstarted:** `canon_commit_check.sh --staged` admission (built, approved, waits on my roster ruling -- cc condition is that I drive the planted-divergence control myself first); `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

**Classes. Every instance is verbatim in `.history/20260819/watch-outs-full.md`.**

- **THE PROBE POPULATION CAN EXCLUDE THE ANSWER, AND IT NEVER SAYS SO.** `cargo test` stops at the first failing binary; a `head -20` on a failure list hides the twenty-first. **Always ask what the probe could not have seen.**
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it; `grep -c` exits 1 on zero. **`$pipestatus`, or redirect and read.** It happened again tonight, on a board that already said so.
- **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION, AND A STALE BINARY IS THE WORST CASE.** vc read `st hydrate` as rc=1 _grammar absent_; re-driven on a current build it was rc=2 _render arm absent_ -- **opposite conclusions about whose work was missing.**
- **A COUNT OF CONTAINERS IS NOT A COUNT OF CONTENTS.** `1 refused` stood for 423 files; `in 0 buckets` stands for four verbs. **Twice in one night, two instruments, one shape.**
- **A REPORT THAT NEVER CHANGES TRAINS ITS READER TO STOP LOOKING -- and GROUPING it reproduces the defect when the grouping keys are what the change preserves.** 199 lines to 4 still could not see a same-directory swap. **The fix is a digest, not a summary.**
- **A DOC COMMENT IS A CLAIM BY THE PERSON LEAST ABLE TO AUDIT IT, and a comment QUOTING a token can trip the scanner for that token.** Read the code, not the sentence above it.
- **A COMPILER WARNING ON THE PATH YOU JUST TOUCHED IS A FINDING, NOT NOISE.**
- **THIS TREE HAS FOUR WRITERS AND THE TOOLS ASSUME ONE.** `--only` separates FILES, not authors -- it failed in BOTH directions today: `924d556b` took ic work under my message, `b277013a` took my live MUTANT under cc message. **Never leave a mutation in a shared tree across anything slower than a moment; announce before, never explain after.**
- **BUILD THE WHOLE WORKSPACE BEFORE EVERY COMMIT**, because what you check is the TREE you publish and not the EDIT you made. The `b277013a` break lived in the composition of two people paths and neither crate showed it alone.
- **PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR.**
- **STANDING CONSTRAINTS.** Push `local` ONLY -- `upstream` frozen at `5765c5da` (hv). NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. Never mutate a file in place while anything runs it. Timestamps READ FROM `date -u`; `git log` prints LOCAL time.
- **ENVIRONMENT.** My shell is **zsh** -- an unquoted `$var` does NOT word-split, and a probe loop ignoring that records a plausible wrong answer for every row. `bin/**` is live on PATH through a symlink. The markdown formatter is a second writer.

## Decisions

**Archived to `.history/20260819/decisions.md`. Kept: the ones that govern tomorrow.**

- (2026-08-19) **EVERYTHING THAT MANAGES STEEL THREADS IS DONE; EVERYTHING THAT MANAGES INTENT ITSELF IS NOT.** That is the hosting gap in one line, and the split is WP boundaries rather than accident.
- (2026-08-19) **A REPO-LOCAL HOIST AND A FLEET HOIST ARE DIFFERENT OPERATIONS** (ic). One symlink serves 16 projects, every one declaring v2. Repointing it is not hoisting this repo, it is starting a 16-project migration -- and only the first is a tonight-sized thing.
- (2026-08-19) **WITHDRAW AS A PRECONDITION, BUILD AS CAPABILITY; THE WARRANT DECIDES, NOT THE CODE.** AC-06.4 is one build under two justifications and only one of them gates. **The question was never whether the work is wanted, it is whether a gate should hold on it.**
- (2026-08-19) **A GATE MUST NOT BE A SECOND AUTHORITY OVER A DESTRUCTIVE ACT** (vc, sharper than my Highlander framing). `sync_uncommitted` asks _are there bytes nobody has synced_; the realiser gate asks _can the store reproduce these bytes_. **They disagree BY CONSTRUCTION rather than by drift**, so a second gate refuses work the real authority would allow. The warning earns its place on timing; safety stays where the deletion is.
- (2026-08-19) **`intent organize` PREVIEWS BY DEFAULT AND `--apply` PERFORMS, ONE BODY TAKING A `Mode`** (ic, AC-05.1). **vc previewed every step before firing the 423-file dehydration and says the polarity is what made it safe to fire.**
- (2026-08-19) **AN INFERENCE FROM SHAPE THAT READS A SPELLING IS FORBIDDEN; ONE THAT CALLS THE MODULE OWNING THE FACT IS NOT** (ic, on `address::promote`). The distinction generalises past that row, and it is the test to apply to any "we can just detect it" proposal.
- (2026-08-19) **A ROW DECLARING A VERB THE MODEL CANNOT EXPRESS IS A ROW THAT LIES.** Proven against me tonight -- four hydrate rows the roster cannot bucket because their mechanism does not exist. **Table-leads-reader is right when the behaviour exists and only the surface lags; reversed, it ships a promise.**
