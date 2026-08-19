---
node: dc
name: DevX Claude
role: worker
session_id: 4dda1c3e-7ae2-4786-8c14-bed8ad03287d
heartbeat_at: 2026-08-19 07:45Z
status: active
focus: "**HALF B IS BUILT: both packs report 0 UNDECLARED** (`e7674053` `35ab425e` `ca1d330c`), and the shell critic asks shellcheck for the first time. vc ruled two AXES not five values; the RED is 16 arms with both named mutants driven. **I retired my own IN-SH-CODE-005 arm by measurement -- 68% of shell files.** AT-07.4 status is vc's to adjudicate: it asserts less than the criterion because (b), the refusal, waits on the hook's distinct exit code. Also fixed `canon_commit_check.sh`, whose flat arm was a prediction the move falsified."
claims: [ST0056/11]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.** hv ruled it four times and it was reinterpreted after three of them, twice by me inside ten minutes.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap two writers interleave in.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **Direction is not symmetric -- IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION HOLDS.** A time-typed input parameter is a defect by inspection. Asking where a caller got a value is a discussion; asking whether a signature accepts one is a grep.
- **The defect is always one step earlier.** Reaching for a clock means you are about to write a time into something that is not a durable record.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it's just a label". The third fooled cc, vc and me independently.
- **SCOPE, hv: devbin is NOT Intent** -- external, vendored, no db. My whole D42 directive resolved to a no-op, and reporting that beat inventing work to look responsive.
- **A board stamp is a label, not data.** The ordering that exists and cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. Three points that change what I DO: **the db is the durable SSOT, files are re-creatable**; **the typed API is the only door in**; **migrations are normal.**

## DOING

Nothing in flight. **Owed to me: vc's adjudication of AT-07.4's status and its cited path.** Owed by me: nothing.

## TODO

1. **HALF B: DONE. The arming axis is closed -- shell 2 armed / 4 declared / 0 undeclared, rust 4 armed / 3 declared / 0 undeclared.** `e7674053` (two grep arms), `35ab425e` (five tool arms + the two-axis census), `ca1d330c` (AT-07.4, 16 arms, both mutants red). **The shell critic now genuinely asks shellcheck** -- SC2086/2046/2206/2068 and SC2012 -- and it blocked one of my own commits on its first day.
   **WHAT REMAINS OF AC-07.4 IS (b), THE REFUSAL, AND IT IS DELIBERATELY NOT BUILT.** A tool-armed rule on a project that armed it must REFUSE when its tool is absent. Refusing needs an exit code the hook can tell apart from an invocation error, and **emitting one before the hook can read it puts a THIRD condition in the bucket that already conflates two, on purpose this time.** It lands with TODO 5, both ends in one commit.
   **I REVERSED MY OWN ROW AND THE MEASUREMENT IS THE ARGUMENT: `IN-SH-CODE-005` IS DECLARE-NONE, NOT GREP-ARMED.** `2>/dev/null` appears **395 times in 87 of 128 shell files -- 68%**. Arming it refuses two thirds of the estate on a CRITICAL rule over correct code. The reason is structural: the rule's content IS its qualifier (_without an adjacent comment_) and a proxy may carry no `-A`/`-B`. `critic-gate.md`'s row needs the correction.
2. **THE RUST-TEST SLOWNESS IS UNDIAGNOSED AND I WAS WRONG FOUR TIMES.** Link fan-out, then "Elixir has no link step" (Lamplight's CLI is Rust), then "Lamplight uses worktrees" (both branches 0 ahead, one 305 behind -- **filed with hv and retracted the same hour**), then binary count (**Lamplight: 1 crate + 19 binaries = ~10min; Intent: 2 crates + 80 binaries = 1m56s -- we are the FASTER project**).
   **ESTABLISHED**: slow run compiled, fast run did not, same command (matts's two logs). Lamplight source untouched 4 days, manifests since 31 Jul, rustc since 21 Jul -- **cargo recompiled what nothing had changed**. **RULED OUT**: disk, memory, swap, devbin-invokes-cargo-per-suite (`cmd/test.d/rust:199` is one call per project), global cargo config, toolchain update.
   **THE INSTRUMENT, and it must run WHILE THE SYMPTOM IS PRESENT** -- I ran it warm and learned nothing: `CARGO_LOG=cargo::core::compiler::fingerprint=info bin/ll test rust 2>&1 | grep -i dirty`. Cargo names the unit and the reason.
3. **Per-node `CARGO_TARGET_DIR` -- announced, mine is `native/rust/target/dc`.** Verified 646 passed / 0 failed / 0.22s warm. **MUST be inside the workspace**: `/tmp` breaks four install-resolution tests because `install.rs:91` walks up from `current_exe()` for a marker dir. Stops us invalidating each other; does NOT stop source churn.
4. **Test-binary consolidation: STOOD DOWN, cc told not to reorder WP-01 for it.** 80 top-level `tests/*.rs` -> modules of ~2 binaries; no file deleted, none moved out of its own file; 42 `mod common;` become `use crate::common;`. **Justification is gone** -- Lamplight has 19 and is slower. Worth doing as a tidy-up only, and it changes a spelling four nodes use (`--test X` becomes `--test suite X`, including the `INTENT_BLESS=1` re-pin).
5. **The hook's fail-open branch -- ruled: a DISTINCT exit code, proved by a RED. IT NOW CARRIES THREE JOBS, NOT ONE, AND THEY ARE ONE RE-CUT OF ONE BLOCK.** (i) the distinct exit code; (ii) **Half B's refusal**, which is the second consumer that makes the code worth having; (iii) **generalising `WB_GUARDS`' presence test per-guard**, because cc's `canon-ignore-guard.sh` (`91736056`) is opt-in by `intent/.canon/` and the whole fail-open reasoning sits inside `if [ -d intent/whiteboard ]`. **cc offered to do (iii) to my spec and I took it instead: two edits to that file means re-reasoning it twice and risking two half-compatible semantics.** A generalisation with one member is a rename; Half B supplied the second. Both ends in one change, and ic's condition: **a distinct code is necessary and NOT sufficient** -- `surface_check.sh` returned rc=2 for the right reason and all four of us filed it as a chore, because triage happens on the PROSE. Build the code AND the register.
6. **BASH-4 SWEEP OF `lib/templates/` -- cc's find, taken.** `env bash` resolves to homebrew 5.3 here while macOS ships 3.2.57, so a template can pass every local test and do nothing on a default shell -- **and a hook that silently does nothing reports success having checked nothing**, which is Half B's own sentence one layer down. Sweep `mapfile`, `readarray`, `declare -A`, `${var^}`, `&>>`.
7. **`doctor` = option 3, ruled** (`intent v2.19.0 auditing a 3.0.0-dev project`). XS. Check whether v3's doctor needs the mirror.
8. **WP-11 dist wiring** -- no `dist-workspace.toml`, no formula, and confirmed independent of ST0057 WP-01. **Publication is policy-unblocked and sequence-blocked**: it needs a tag, `int build release` correctly refuses a dev version, and `staged_version` is `3.0.0-dev`. AC-11.1/11.4 stay open.
9. **`output-contracts.md` owes four additions**: suppression restated as TRANSMISSION (prose is a channel); the fifth mechanism (**the refuting datum was in the author's own output, uncollected when it became load-bearing**); ic's class name (**a control that appears to fire and doesn't**); and the sixth (**an aggregating formatter can manufacture evidence that appears nowhere in the source**).

**STRUCK, ruled and needing nothing: `--skip-rust-tests` (DROPPED). The shim (HOLD -- defeats Half A). WP-12's clean-tree rebuild (not bankable, AC-11.5 closed).**

## Watch-outs

- **THIS REPO IS A v3 PROJECT** (`3.0.0-dev`; store `intent/.cache/intent.db`, gitignored). v2 refuses PROJECT verbs at exit 2; `GLOBAL_COMMANDS` run fine, and **`critic` is now one of them (my change, `92a51134`)**. Invoke v3 by explicit path and use `target/debug/`, not `release/`.
- **THE BUILT BINARIES ARE AT `native/rust/target/release/`, NOT `release/`** -- two days of my own records named a directory that does not exist.
- **PUSH `local` ONLY.** `upstream` closed (hv, CI/CD budget), frozen at `5765c5da`.
- **NEVER `git pull --rebase` here; a peer `.git/index.lock` means WAIT. ALWAYS `git commit --only <paths>`** -- a bare commit sweeps a peer's staged index, and `--amend` ignores `--only`.
- **NEVER mutate a file in place while anything runs it -- USE AN ATOMIC REPLACE (write beside it, `mv` over it).** Size is irrelevant, only timing: the pre-commit roster runs on every node's every commit. Both vc and I nearly shipped an in-place edit to a live gate file today.
- **COMMIT BEFORE ANY `intent at` STATUS CHANGE** -- `intent at red|green|na` DESTROYS the row note (issue 0033). There is NO note verb; the whole family is done/green/lint/list/na/notdone/red.
- **`cargo test` IS A CONCURRENT WRITER ON THE ESTATE**, and `.gitignore:127` ignores the store, so `git status --porcelain` cannot see it.
- **A PEER CANNOT WAIVE AN hv RULE**, and being right on the substance is not the same as being entitled to say so.
- **MY SHELL IS zsh: it does NOT word-split an unquoted `$var`, and `~3` is a directory-stack reference, not a git suffix.** Both produced plausible wrong answers today. Quote, or use a bash array.
- **NEVER `$?` AFTER A PIPE.** I hit this in my own verification today after quoting it at two other people.
- **AN APOSTROPHE INSIDE THE SINGLE-QUOTED `ROSTER` STRING BREAKS `runner_roster_check.sh` AT PARSE.** Caught by running the consumer, not by `bash -n` on the tool.
- **`git log` prints LOCAL time**; reading one and appending `Z` is wrong by exactly the offset and looks perfect.
- **macOS: signing MUTATES the binary, notarisation does NOT -- checksum AFTER signing.** `target/release/` is shared mutable state; private `CARGO_TARGET_DIR`, never sign there.
- **A control refuses; documentation reminds; only one is load-bearing.** My own watch-outs are not controls -- which the day proved on me twice.
- **`cp` ONTO A SYMLINK FOLLOWS IT AND WRITES THROUGH TO THE TARGET.** My isolation rig symlinked every `bin/intent_*` into the real repo and then `cp`-ed over one -- so it wrote into the LIVE `bin/intent_critic` for two minutes. **A rig assembled by symlinking into the real tree is not isolated; it looks isolated right up until it writes to the file it existed to protect.** `rm` the symlink before writing, and only symlink what you READ.
- **`exit` INSIDE `$( )` EXITS ONLY THE SUBSHELL.** Two refusals printed and the tool carried on; one then closed its arithmetic over a population silently reduced by one. **A refusal that does not stop is a diagnostic, and a diagnostic upstream of arithmetic is worse than none, because the number after it looks measured.** Every caller checks.
- **`chmod --reference` IS GNU-ONLY.** On macOS it fails and a `||` fallback IMPOSES a mode rather than preserving one -- it flipped a sourced library to 755. A fallback that imposes is not a fallback.
- **A REPRODUCTION THAT IS NOT BYTE-FAITHFUL IS WORSE THAN USELESS.** Mine gave rc=2 where the live state gave rc=0, and I nearly announced the gate had been fail-opening -- a false alarm about the exact defect under investigation.
- **CLIPPY LISTS ITS LINTS HYPHENATED** (`clippy::needless-lifetimes`). My underscored grep reported 7 of 8 ABSENT from a list of 825. Third vocabulary error of the day.
- **THIS MACHINE'S TIMING NOISE FLOOR IS 6x.** Identical suite, back to back, warm: 16.12s then 100.10s. **I called it noise and moved on; it was the most informative number I had.** Any timing here needs repeats before it means anything.
- **MEASURE THE CASE THAT DISPROVES YOU, AND matts HANDED ME THE CONTROL GROUP TWICE BEFORE I USED IT.** Four wrong explanations on one question in one evening, each reasoned from a property I had not checked. **`.worktrees/cc` EXISTING told me nothing about whether anyone works in it** -- `git rev-list --count` did, in thirty seconds, and said zero.

## Decisions

- (2026-08-19) **A COUNT INSIDE A CRITERION IS A RECORD, AND ITS SUBJECT CAN DIE WITHOUT ANYTHING NOTICING.** AC-00.11 carries `dc nominated 10 of 41 parity tools by a two-grep proxy` and **that probe was never committed** -- no tool, no board history, and my reconstruction returns 14. `of_n_population.sh` (`f789ae48`) enumerates the population instead. **ic found the same shape in the same criterion within the hour** (`EXAMINED 86 of 278` names no revision, so nobody can check it), so AC-00.11 carried TWO unreproducible figures in the criterion that rules figures must derive from what was examined. vc's remedy is the right one and it is not a third number: **NAME THE REVISION, or MARK IT `RECORDED`.**
- (2026-08-19) **TWO POPULATIONS THAT COINCIDE ONLY IN A NUMERAL READ AS A CROSS-REFERENCE AND ARE DOING NO SUCH WORK.** `10 gated of 17 rostered` and `10 of 41` are different sets; **five of the ten gated carry no `intent/st` path shape at all**, so half a `drive the gated ten` plan would have returned clean under a two-tree differential and meant nothing. ic was one step from building on it.
- (2026-08-19) **A FIFTH VALUE IN A KEY IS HOW AN AXIS GETS SMUGGLED IN, and vc caught mine before it landed.** Arming state and run disposition are two axes: `armed` + `not-run:out-of-context` says both true things at once, where a fifth arming value would have made `armed` mean two things depending on which member you read. **The general form vc then minted from three same-day instances: a checker that verifies MEMBERSHIP in a vocabulary never verifies that the vocabulary can EXPRESS the states in use** -- and the forced-wrong value is always the one that reads as MORE finished, which is why review never catches it.
- (2026-08-19) **I ARMED A CRITICAL RULE AGAINST MY OWN WRITTEN WARNING AND ONLY THE MEASUREMENT STOPPED ME.** `critic-gate.md` said IN-SH-CODE-005 would fire on every documented correct use. I armed it anyway on "noisy beats silent", and it blocked my own commit within the hour: 13 findings on one file, 13 false positives, 68% of the estate. **A cost I had written down did not function as a constraint until it was a number.**
- (2026-08-19) **A STRIP THAT SUCCEEDS AND PRODUCES A WELL-FORMED PATH TO NOWHERE IS INVISIBLE TO EVERY ASSERTION ABOUT THE STRIP.** `canon_commit_check.sh` already guards the failed-strip case (pattern absent, string returned unchanged). The move broke the OTHER direction and the tool reported **ADDS 279 of 279 at exit 1** -- its own resolution failure, in the vocabulary of a finding. **The flat arm was a PREDICTION written before the move, and the move did something else.**
- (2026-08-19) **A PROBE THAT MEASURES THE WRONG NAME REPORTS A REAL CAPABILITY AS MISSING, AND THE OUTPUT READS EXACTLY LIKE A TRUE ABSENCE.** `command -v clippy` is false on a machine with clippy 0.1.97, because it ships as `cargo-clippy`. **Second clippy-vocabulary error in two days** after the hyphenated-lint-name one. And the ORDER compounded it: testing availability before context answered a question about the invocation with a fact about the host, and a reader would have installed something that still would not run.
- (2026-08-19) **SIX CONFIDENT REDS IN MY OWN NEW TEST, ALL THE INSTRUMENT'S FAULT.** `bash -c` spawns a shell that has not sourced the library, so six arms asserted against `command not found`. **Same shape as a sweep reporting `examined: 0` over an estate of 51 -- the instrument answered a different question and the output looked like a result.**

**Today's 21 decisions are archived to `.history/20260819/` and every one is carried by a committed artefact** -- `critic-gate.md`, `output-contracts.md`, AC-07.4, AC-00.11, or the instrument itself. **Restating them here is where the drift starts.** What follows is only what is NOT in one of those.

- (2026-08-19) **A DIRECTORY THAT EXISTS IS NOT A DIRECTORY ANYBODY USES.** I filed a structural proposal with hv off a path listing and retracted it the same hour when `rev-list` showed both branches carried zero commits. **Same shape as a marker that names a commit but identifies no artefact, a proxy that is not the parser, and an instrument that lives in a scratchpad: the record survives and its subject does not.**
- (2026-08-19) **A GUARD BUILT FROM ONE INSTANCE DEFENDS AGAINST THAT INSTANCE, NOT THE CLASS** (ic, on truncating my inbox with a guard they wrote the same day against the opposite failure). **Its freshness is what makes it feel like coverage.**
