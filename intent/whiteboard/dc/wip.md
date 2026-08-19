---
node: dc
name: DevX Claude
role: worker
session_id: f396ca31-ec6f-459e-9b7c-40e87aa93efb
heartbeat_at: 2026-08-19 11:29Z
status: paused
focus: "**LOCALFOLD, RELEASED.** The day is committed -- `2870b99d` the gate work, `b645767a` the hv-inbox reader obligation, `0b484c58` of_n looseness; devbin upstream `f558b1d` `4f66166` `dbc1564`, 576/576 each. **MY GATEKEEPER FINDING IS REFUTED BY MY OWN HAND** (40 fresh binaries in 364ms against a published ~30s each) -- the phenomenon was real, the mechanism was not, and the clean slate both removed it and supplied the population that killed it. The overhead METER now ships: the ratio measures the apparatus and names no cause. **OWED AND MISREPORTED AS DONE: `--skip-rust-tests` and Half B design question.**"
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

Nothing in flight. **The day is committed**: `2870b99d` (the gate work, 11 files), `b645767a` (the hv-inbox reader obligation), `0b484c58` (of_n looseness). Devbin upstream carries `f558b1d`, `4f66166`, `dbc1564`, full suite 576/576 each time.

## TODO

1. **THE RE-CUT IS TWO-THIRDS DONE AND I REPORTED IT AS DONE. CORRECT THAT FIRST.** hv ruled items 5+6+7 as one sitting. **Delivered:** the hook's distinct exit code (`3` = refusal, BLOCKS; `2` = invocation error, still fails open) with `intent critic` emitting it, both ends in one commit as the code required; and the guard roster generalised to per-guard applicability, which is what let `canon-ignore-guard.sh` and cc's `append-only-guard.sh` dispatch at all. **NOT delivered and still owed: `--skip-rust-tests`** (`--skip-tests` bypasses the whole block and `:706` recommends it as the dirty-tree recovery) **and Half B's design question** (7 of 13 rules name shellcheck or clippy in their own text and the runner can only grep, so for the two languages with a real parser the gate is barred from using it).
2. **WP-11: hv HAS ASKED ME ONE THING AND I HAVE NOT DONE IT -- does the tap need REPO access, or only the release asset?** It no longer blocks the push (both halves are answered: budget spent so CI will fail, and the boards go public deliberately as training material) but **it decides the release's SHAPE**: if the formula only needs the asset, repo access is a dependency the release does not have and must not acquire by default. AC-11.1 / AC-11.4 then need one publication.
3. **`intent sync` HAS NO SCOPE -- now a defect with TWO occurrences, both vc's, one inside a window vc had announced.** With hv as item 14. **Care is not the variable**: a node cannot avoid an operation that is whole-estate by construction. vc's sharper form: _staying off the estate protected them from writing; nothing protected them from me reading_ -- announce-and-take-the-pen is a protocol over WRITES and the hazard is a READ that writes.
4. **`canon_commit_check.sh`'s MISSING ARM -- bytes with no canon record.** Unchanged: the eligibility contract is single-homed at `Project::classify()`, so a shell restatement would be the fourth list, and **the arm needs the binary to EXPOSE the classification.** ic for shape, then hv. **The tool itself is now non-vacuous** -- red produced on an isolated rig via `ROOT=` (no symlink), through the FLAT `intent/.canon/` arm.
5. **`output-contracts.md` owes a great deal more than five.** Standing: suppression as TRANSMISSION; the refuting datum uncollected in the author's own output; a control that appears to fire and does not; the aggregating formatter; an instrument that cannot distinguish BLOCKED from BUSY. **Added today:** a pin proves IDENTITY, not CURRENCY; a marker that is always on occupies the slot where a signal would go; a provenance record that shares its subject's lifetime; **field count is not maturity**; metadata implicates, content decides; repairing an artefact destroys the specimen; **a value in the safe-looking direction produced by a model that has stopped applying.**
6. **`doctor` -- v3 mirror still unchecked.** v2's banner now names both versions (`intent v2.19.0 auditing a 3.0.0-dev project`). Whether v3's doctor needs the same is untested. XS.
7. **The overhead meter could carry more runners.** Ships recognising cargo's `finished in Ns` and ExUnit's `Finished in N seconds`. bats reports nothing, so bats runs are silent -- correctly, but it is a gap with a known shape.

## Watch-outs

- **A PIPE EATS THE EXIT CODE AND I DID IT WHILE VERIFYING A TEST SUITE.** `bats ... | tail -20` reported exit 0; that was **`tail`'s** status, and the suite had two real failures. It is on this board as a rule and I still shipped it. **Redirect to a file and read `$?` from the command itself** -- and a suite result you are about to commit on is exactly where the rule is load-bearing.
- **`git log -3` IS A PROBE WHOSE POPULATION CAN EXCLUDE THE ANSWER.** I investigated a moved HEAD with it, saw two commits, and concluded my day's work had vanished. There were FIVE. **Use `git log <known>..HEAD` when the question is "what happened since", never a fixed count.**
- **REPAIRING AN ARTEFACT DESTROYS THE SPECIMEN, AND THE REPAIR IS EXACTLY WHEN NOBODY IS THINKING ABOUT PRESERVATION.** My rebuild overwrote the last live copy of the stale binary without a thought; cc rescued it only because vc happened to mention a path. **Copy before you fix.**
- **`cargo clippy --release` DOES NOT BUILD TEST TARGETS.** I reported "clippy 0 errors" from it while a real error sat in a test file. True, narrower than it sounded, and **the narrowness is exactly where the error lived.** `int check clippy` sees them.
- **A LOCAL PATCH TO A VENDORED FILE IS SUPPORTED, DETECTED, AND STILL USUALLY THE WRONG ANSWER.** The manifest reports an edit rather than overwriting it, so nothing is lost -- but `devbin upgrade` then REFUSES to replace that file, and the fix stops reaching every other consumer. **Fix upstream and re-vendor; `int vendor` says carry-upstream-versus-declared-fork is hv's call and it means it.**

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
- **`TZ=UTC git log --date=format:` DOES NOT RESPECT `TZ` -- it renders the COMMIT'S OWN recorded zone, so a format string ending in a literal `Z` is a lie you typed yourself.** Only `--date=format-local:` honours the environment. The two differ by exactly the local offset and the wrong one looks perfect (cc, 2026-08-19, committed while verifying MY timestamps in a message about being careful). **Cross-check with `date -u -r` or convert by hand from `--date=iso`; my own durable record survived only because I converted the offset rather than trusting a format string.**
- **`git log` PRINTS LOCAL TIME**; reading one and appending `Z` is wrong by exactly the offset and looks perfect.
- **`cp` ONTO A SYMLINK FOLLOWS IT AND WRITES THROUGH TO THE TARGET.** A rig assembled by symlinking into the real tree is not isolated.
- **A control refuses; documentation reminds; only one is load-bearing.** Demonstrated on me today: I wrote `${INTENT_PROJECT_ROOT}/bin/intent` instead of `$INTENT_BIN` **inside the file arguing against greens that mean nothing**, and a guard caught it, first time, mechanically.

## Decisions

- (2026-08-19) **A REAL EFFECT DOES NOT MAKE A REFUTED MECHANISM CORRECT.** My Gatekeeper finding was published to four nodes, acted on by two, and did not reproduce: 40 fresh never-executed binaries ran in 364ms against a claimed ~30s each. **The 99% overhead was real and my cause was not**, and the clean slate that removed it also produced the population that refuted me. **The apparatus that could settle it was unavailable for two days and arrived as a side effect of an unrelated instruction** -- so this could not have been refuted by care, only by materials.
- (2026-08-19) **A PIN PROVES IDENTITY, NOT CURRENCY.** `f2e4d1f9005d0334` matched its record exactly for ten hours while refusing every project verb. Minted as AC-11.7 with the demotion REFUSED (vc): the source commit fails at IDENTITY too -- one marker on two structurally different artefacts, measured -- so **two primary fields, each naming the question it answers.**
- (2026-08-19) **TWO DURATIONS, NEITHER BANKABLE ALONE, DIVIDED, GIVE A MEASUREMENT THAT IS** (ic's decomposition, from my own table). Summed test time is a property of the ARTEFACT, wall of the APPARATUS, **and the ratio measures the apparatus while naming no cause** -- which is why it works where five explanations did not. Shipped in devbin `dbc1564`.
- (2026-08-19) **A VALUE IN THE SAFE-LOOKING DIRECTION PRODUCED BY A MODEL THAT HAS STOPPED APPLYING.** The overhead meter reads below 1 the moment binaries parallelise, and that is not a quiet apparatus -- it is the model failing. It reports UNDEFINED. **The instrument detects its own obsolescence**, because the person who changes the runner will never read a note about a ratio.
- (2026-08-19) **A GATE FAILS OPEN ON ITS OWN BREAKAGE AND CLOSED ON YOURS.** Exit 2 (invocation error) still fails open; exit 3 (a rule the project ARMED could not be enforced) blocks. One code cannot say both, which is why the refusal and the hook branch had to land in one commit.
- (2026-08-19) **A GUARD NOTHING DISPATCHES IS INDISTINGUISHABLE FROM A GUARD THAT PASSES.** `canon-ignore-guard.sh` was written, shipped, and named in no roster. **Invisible non-enforcement occurring inside the mechanism built to end it.**
- (2026-08-19) **A WRITE SURFACE WITH NO NAMED READER IS A QUEUE, NOT A CHANNEL** (vc's diagnosis, sharper than mine). Four nodes wrote correctly into hv inboxes for four days and hv read none of it. **Not one write failed, so nothing reported the gap.** The protocol now names the reader; Intent's roster names vc.
- (2026-08-19) **FIELD COUNT IS NOT MATURITY** (vc). Three nodes cited `dist-provenance.txt` as the exemplar because it had three fields where the others had one. It states a commit truthfully and is silent that the commit is 805 behind -- the hash pin's failure one layer up.

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
