---
node: dc
name: DevX Claude
role: worker
session_id: 4dda1c3e-7ae2-4786-8c14-bed8ad03287d
heartbeat_at: 2026-08-19 08:32Z
status: active
focus: "**HALF B DONE (0 undeclared, both packs) AND THE RUST SLOWNESS IS SOLVED: IT IS macOS GATEKEEPER, NOT CARGO.** 81 test binaries, ~17s of signature validation on FIRST exec each (20633ms -> 26ms), wrapped around **11.87s of actual test time in 22min40s** of wall clock. **Every timing figure I have ever published here is void** -- the hidden variable is validation-cache state, not the revision. Consolidation REVIVED: I killed it on a confounded control."
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

Nothing in flight. **THREE FIXES HELD, VERIFIED, UNCOMMITTED** -- waiting on matts's suite to finish rather than firing the pre-commit gate into a live acceptance run: `critic_report_format.bats` (2 json regressions, mine from `3a646965`) and `critic_arming_census.bats` (the retarget-guard catch). 126 assertions / 9 suites / 0 failures on the fixed tree.

## TODO

1. **THE RUST SLOWNESS IS SOLVED AND THE REMEDY IS MOSTLY NOT MINE. IT IS GATEKEEPER.** Measured on matts's live run, twice:
   - 11min20s wall / **8.51s** summed `finished in`; then 22min40s wall / **11.87s**. **99.1% overhead, linear at ~30s per suite.** 88 suites will take ~44 minutes to do ~23 seconds of work.
   - **`rustc` NOT running** (compile already done), test binary state `S` with **RSS 32 KB** -- blocked BEFORE the test code loads -- and `syspolicyd` at **21-22% sustained**.
   - Same binary, first exec vs second: `intent` **20633ms -> 26ms**, `acceptance_surface` **19459ms -> 24ms**, `session_hook_lockout` **10949ms -> 23ms**. `spctl --status` = assessments enabled; binaries are adhoc/linker-signed.
   - **Cargo makes ONE BINARY PER `tests/*.rs`**: 25 intent-cli + 56 intentsvcs = **81 per build**. That is also why `deps/` holds 291 executables and **778,425 files**, and why `target/debug` is 15GB (~25GB across the per-node dirs).
   - **REMEDIES, in impact order:** (a) register the terminal under Privacy & Security -> Developer Tools -- matts's call, it is a real security setting; (b) **consolidate 81 `tests/*.rs` into 2-3 binaries** -- mine, see 2; (c) prune stale generations and `.noindex` the target dirs.
   - **NOT MEASURED AND NOT CLAIMED: the COMPILE phase.** `build.rs` outputs are executables that get RUN and proc-macro dylibs get loaded, so the same tax plausibly applies -- but I have no measurement and two probes today already returned what I expected while being unable to return anything else.
2. **TEST-BINARY CONSOLIDATION IS REVIVED AND I KILLED IT ON A CONFOUNDED CONTROL.** I compared Lamplight (19 binaries, ~10min) against Intent (80 binaries, 1m56s) and concluded we were the faster project. **I was comparing two validation-CACHE states, not two binary counts.** 81 -> 3 binaries cuts validation from ~23min to under a minute even if Developer Tools is never enabled. Changes a spelling four nodes use (`--test X` becomes `--test suite X`, including the `INTENT_BLESS=1` re-pin).
3. **TODO 5, BLOCKED ON matts: the hook's distinct exit code + Half B's refusal + generalising the guard roster's presence test for cc's `canon-ignore-guard.sh`.** One re-cut of one block in `pre-commit.sh`; cc offered to do the roster part to my spec and I took it instead so the file is re-reasoned once. **It changes fail-open semantics every fleet consumer inherits on upgrade.**
4. **`canon_commit_check.sh`'s MISSING ARM -- bytes with no canon record.** vc ruled the eligibility contract already exists and is single-homed at `Project::classify()` (`ATTACHMENT_EXTENSIONS = md, txt, sh`), so a shell restatement would be the fourth list. **Blocked on the surface question: the arm needs the binary to EXPOSE the classification.** ic first for shape, then matts.
5. **`intent sync` HAS NO SCOPE and that is now an empirical finding, not an architectural one.** The correct per-node workflow needs an unscoped whole-estate write; two nodes running it clobber at rc=0. **`canon_commit_check.sh` is rostered `manual` PRECISELY because of it**, so the estate holds a detector it cannot afford to run -- and that gap has already cost one real divergence (`critic-gate.md`, 15,428 bytes of formatter table padding).
6. **BASH-4 SWEEP: DONE, control committed** (`f8e05490`). 0 constructs across the 7 shipped hooks; the finding is that **`bash -n` at 3.2.57 is blind to both classes** and none of the seven sets `-e`, so a stray bash-4 BUILTIN is rc=0 with a plausible number.
7. **`doctor` = option 3, ruled.** XS. Check whether v3's doctor needs the mirror.
8. **WP-11 dist wiring** -- no `dist-workspace.toml`, no formula. Policy-unblocked, sequence-blocked: needs a tag, and `staged_version` is `3.0.0-dev`. AC-11.1/11.4 open.
9. **`output-contracts.md` owes five additions**: suppression as TRANSMISSION; the refuting datum uncollected in the author's own output; ic's _a control that appears to fire and doesn't_; the aggregating formatter; and **the new one -- an instrument that cannot distinguish BLOCKED from BUSY.**

## Watch-outs

- **ELAPSED TIME CANNOT DISTINGUISH BLOCKED FROM BUSY, AND THAT IS WHY THE SLOWNESS SURVIVED FOUR WRONG EXPLANATIONS.** What settled it was a process STATE and an RSS of 32 KB -- available for two days, never looked at, because I was measuring duration.
- **EVERY TIMING FIGURE IN THIS ESTATE IS SUSPECT UNLESS IT RECORDS WHETHER A COMPILE PRECEDED IT.** Subject and revision are not enough; validation-cache state is a hidden variable that moves a number by 800x. My `16.12s / 100.10s` "noise floor" was this mechanism, not noise.
- **THIS REPO IS A v3 PROJECT** (`3.0.0-dev`; store `intent/.cache/intent.db`, gitignored). v2 refuses PROJECT verbs at exit 2; `critic` is a GLOBAL_COMMAND (my change, `92a51134`). Invoke v3 by explicit path; binaries are at `native/rust/target/release/`, not `release/`.
- **A PARITY TOOL IS AN ATTACHMENT OF ST0056.** Committing one without a sync leaves canon stale and the next peer's sync sweeps it into their commit -- measured: `of_n_population.sh` file at `f789ae48` (mine), canon record at `938ed7a3` (vc's). **Announce, sync, commit file and canon together.**
- **THE MARKDOWN FORMATTER IS A SECOND WRITER AND IT IS NOT A PEER.** It re-aligns tables on save, in the window between sync and commit; the longer the cell you just wrote, the bigger the divergence. `critic-gate.md` diverged by 15,428 bytes that way, 100% table padding.
- **`git diff --stat` CANNOT EXPRESS SIZE ON CANON JSON.** An attachment is ONE LINE, so a 9,740-byte tool and a one-word typo both read as `1 insertion`. Compare structures, never the diffstat.
- **A SINGLE `pgrep` SAMPLE CANNOT DISTINGUISH A DEAD OWNER FROM ONE BETWEEN SPAWNS.** I called a stale-lock reading "definitive" off one sample of an intermittent process. Same class as `bash -n`, same day.
- **`bash -n` IS NOT A SYNTAX CHECK FOR `.bats` FILES** (they are not bash) **and at 3.2 it does not reject bash-4 constructs either.** Two distinct false instruments wearing one command.
- **PUSH `local` ONLY.** `upstream` closed (hv), frozen at `5765c5da`.
- **NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. ALWAYS `git commit --only <paths>`** -- a bare commit sweeps a peer's staged index.
- **NEVER mutate a file in place while anything runs it -- ATOMIC REPLACE (write beside it, `mv` over it).** The pre-commit roster runs on every node's every commit.
- **COMMIT BEFORE ANY `intent at` STATUS CHANGE** -- `intent at red|green|na` DESTROYS the row note (issue 0033). **And route status changes through vc**, who holds the canon pen.
- **NEVER `$?` AFTER A PIPE.** **`grep -c` EXITS 1 ON ZERO**, so a `||` fallback fires on a true zero. **`exit` INSIDE `$( )` EXITS ONLY THE SUBSHELL.**
- **MY SHELL IS zsh: it does NOT word-split an unquoted `$var`.** Quote, or use a bash array.
- **`git log` PRINTS LOCAL TIME**; reading one and appending `Z` is wrong by exactly the offset and looks perfect.
- **`cp` ONTO A SYMLINK FOLLOWS IT AND WRITES THROUGH TO THE TARGET.** A rig assembled by symlinking into the real tree is not isolated.
- **A control refuses; documentation reminds; only one is load-bearing.** Demonstrated on me today: I wrote `${INTENT_PROJECT_ROOT}/bin/intent` instead of `$INTENT_BIN` **inside the file arguing against greens that mean nothing**, and a guard caught it, first time, mechanically.

## Decisions

- (2026-08-19) **AN INSTRUMENT THAT MEASURES DURATION CANNOT SEE WHY.** Four wrong explanations for the rust slowness over two days, every one reasoned from elapsed time. The answer came from a process STATE (`S`) and an RSS (32 KB) -- **the binary had not begun executing.** Duration is the one observable that cannot separate blocked from busy, and it is the one everybody reaches for.
- (2026-08-19) **A CONTROL GROUP CHOSEN WITHOUT KNOWING THE MECHANISM IS NOT A CONTROL.** Lamplight-versus-Intent was a real experiment that refuted my own hypothesis, and it was still void: I varied binary count and unknowingly varied validation-cache state. **It killed the right work for a wrong reason for a full day.**
- (2026-08-19) **THINKING HARD ABOUT A CLASS DOES NOT PROTECT YOU FROM IT.** I committed the dispatcher-spelling violation inside the file whose entire subject is greens that mean nothing, on the day I built the instrument for it, while holding the argument in my head. **No care reached it; a control did, first time.**
- (2026-08-19) **A COST WRITTEN DOWN IS NOT A CONSTRAINT UNTIL IT IS A NUMBER.** `critic-gate.md` said IN-SH-CODE-005 would fire on every correct use. I armed it anyway on "noisy beats silent", and it blocked my own commit within the hour -- 13 findings, 13 false positives, **395 hits in 87 of 128 shell files**.
- (2026-08-19) **A COUNT INSIDE A CRITERION IS A RECORD WHOSE SUBJECT CAN DIE UNNOTICED.** AC-00.11's `10 of 41` came from a probe that was never committed; my reconstruction returns 14. ic found the criterion's other figure unverifiable within the hour. **Remedy is not a third number: name the revision, or mark it RECORDED.**
- (2026-08-19) **TWO POPULATIONS THAT COINCIDE ONLY IN A NUMERAL READ AS A CROSS-REFERENCE AND DO NO SUCH WORK.** `10 gated of 17 rostered` versus `10 of 41`; five of the ten gated carry no path shape, so half a "drive the gated ten" plan would have passed vacuously.
- (2026-08-19) **A CHECK THAT WALKS THE RECORD CANNOT SEE WHAT THE RECORD OMITS.** `canon_commit_check.sh` asks whether every RECORDED attachment's bytes are present; a file with no record is never in the population. **A denominator drawn from the record rather than the estate**, inside the tool built to police canon/commit agreement.
- (2026-08-19) **A WORKFLOW WHOSE CORRECT FORM REQUIRES AN OPERATION ONLY SAFE FOR ONE ACTOR IS A SINGLE-WRITER BOTTLENECK WEARING A PER-NODE PROCEDURE'S CLOTHES.** And the second writer in the sync-to-commit window is not a peer at all -- it is the formatter, so no amount of peer coordination closes it.
- (2026-08-19) **A FIFTH VALUE IN A KEY IS HOW AN AXIS GETS SMUGGLED IN** (vc, catching mine before it landed). Arming and run disposition are two axes. vc's general form, from three same-day instances: **a checker that verifies MEMBERSHIP in a vocabulary never verifies that the vocabulary can EXPRESS the states in use** -- and the forced-wrong value always reads as MORE finished.
- (2026-08-19) **A CORRECT DECISION PROCEDURE THAT RETURNS "ASK" IS NOT INVALIDATED BY THE ANSWER BEING "YES"** (vc). Three nodes stood off a lock that turned out to be stale, on reasoning that was right anyway.
