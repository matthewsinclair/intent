---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-28 14:50Z
status: active
focus: "ST0065 CLAIMED AND BOTH CATALOGUE LEGS LANDED -- WP-01 f33fcccd, WP-02 143cbc8e. NOTHING IN THE CANON SET OR THE SKILLS IS EDITED: verdicts are recommendations, hv rules first. THREE MISGUIDED, all one shape -- a document that outlived its mechanism: AGENTS.md is called the primary contract and nothing injects it on the Claude Code path; MODULES.md was retired from init on 08-24 and SIX sites still instruct it, including the CLI remedy; DECISION_TREE.md still seeds an Elixir/Phoenix tree into a project with languages: []. The session budget nobody owned is ~87KB / ~22k tokens. NEXT: the 14 skills swept but not read, and .claude/restart.md needs in-finish read before it can be judged."
claims: [ST0065, ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. NOTHING CLAIMED TODAY. HOLDING BY NAME** under hv's 13:51Z assignment ruling. The day's full record is in two verbatim archives: `.history/20260828/wip.md` (the hoist and Phase 4, 42KB) and `.history/20260828/wip-fold-1425Z.md` (the afternoon, 14KB). **Neither was overwritten by the other, which took a deliberate check** -- the morning fold wrote plain `wip.md` against a convention of `wip-fold-HHMMZ.md`, so a second same-day fold would have silently destroyed the day's primary record.

**WHAT STANDS:** Conflab hopped to v3; every parity leg I verified passed at `7652c9b4`. Routed out and not mine: `wp list` reach (0103, cc), doctor's `gate-not-running` false positive and the json-no-counted-marker gap (dc), guard-home (0113, dc).

**MY INTENT CLAIMS ARE PAUSED, NOT ABANDONED** -- ST0057/{02,05,07,08,11,14} and ST0061, untouched since the Conflab reassignment.

## TODO

- **NOTHING IS OPEN FOR ME. Pick up from vc, not from this board.**
- **ON hv's QUEUE, NOT MINE TO START:** whether an estate-measuring instrument is node scratch or project apparatus. `board_overlap.py` measures MY BOARD and lives in my single-writer dir; `parity4.py` / `census_cmp.py` / `dangling.py` measured AN ESTATE and died with their session (designs and mutation tables are in the morning archive). **The gating obstacle I first gave vc for this was invented and I withdrew it** -- the precedent for a durable node-local instrument is a precedent of ONE and I set it myself.
- **OFFERED TO vc, NOT STARTED:** driving an instrument against a deliberately dangled binary pair, if dc wants that arm exercised.

## Watch-outs

**THE DAY'S ONE CLASS, AND IT COST ME THREE RETRACTIONS: I COMPARED NUMBERS WITHOUT ASKING WHAT POPULATION EACH COUNTED.** Every figure below is as-measured at the Conflab hop and is a HISTORICAL example, not live state.

- **531 vs 216 WPs** -- canon held all 531; `wp list` returns rc=0 with zero rows for whole threads. Reach, not loss. **I caught this ONLY because a 315 gap looks like catastrophe.**
- **190 vs 54 findings** -- 54 is doctor's COUNTED figure, 190 the json array including 136 uncounted advisories. **I did NOT catch it, and told my coordinator their correct number was wrong.** Both plausible, so nothing looked odd. **THE SIZE OF A DISCREPANCY IS WHAT MAKES IT VISIBLE, NOT ANY VIRTUE IN THE READING** -- the discipline is _ask what population each number counts_, every time, not _notice when they look wrong_.
- **A file's sha256 vs a binary's commit marker** -- **I compared an identifier to an identifier without establishing they name the same KIND of thing.** **The values are deliberately not quoted here:** both moved the same afternoon, and a watch-out that quotes live values goes stale as silently as any other ledger. **The keeper is the DISTINCTION, never the numbers** -- and the estate had this written down in `self_provenance_check.sh`, a guard I had not run.

**A VERIFICATION IS A CLAIM ABOUT A MOMENT, AND I BROKE THIS ONE HAVING WRITTEN IT DOWN.** I measured a file pre-hop, accurately, then reached for that reading post-hop to corroborate a finding instead of re-reading a file replaced an hour earlier. **A FALSE POSITIVE AND A STALE MEASUREMENT CORROBORATED EACH OTHER INTO A CONCLUSION NEITHER COULD SUPPORT ALONE.** Two sources agreeing is the control I demand of everyone; here the agreement was the trap. **Independence is not two instruments -- it is two instruments whose errors cannot share a cause, and _both taken before the file changed_ is a shared cause.**

**`names no X` AND `does no X` ARE DIFFERENT CLAIMS, AND THIS IS NOW A FAMILY WITH A DATE.** The shim names no `GUARD_RUNNER` **because it delegates** (`:129` resolves the gate, `:152` execs it) -- that is doctor's `gate-not-running` false positive. Then issue 0113, same shape: `guard_home_check.sh` reads the right file and greps it for `GUARD_HOME="$_repo_root"`, which lives in the GATE BODY and which the shim never carries by construction; its printed remedy reinstalls the shim, so the NOTE is permanent. **A component that does not NAME a thing may be the one that CALLS it, and a check written before the delegating layer cannot tell them apart.** **A stale reading is a mistake; a true premise with a false conclusion is a trap, and the second recruits careful people** (cc's framing).

**HYPOTHESIS, NOT A MEASUREMENT, WITH ITS TEST STATED.** If the shim's introduction is the common cause, every check written against the PRE-SHIM carrier is aimed one layer too high, and members surface one at a time as each check next fires -- which is how these two arrived. **The family would then be ENUMERABLE AHEAD OF TIME:** for each check, take the strings it expects in the carrier and ask which the shim actually carries and which the gate body does. **Two points explained is not a measured property of the corpus, and the test is cheap and has not been run.**

**A RETRACTION DOES NOT OVERTAKE THE FALSE THING IT IS CHASING.** Mine was two nodes downstream before it reached one. **Send it to every node the claim could have reached, and say what to PULL, not just what was wrong.**

**INSTRUMENTS.**

- **TWO SIBLINGS IN ONE DIRECTORY HAD OPPOSITE `ROOT=` CONTRACTS** -- `view_skew_check.sh:78` honours the env var, `thread_view_skew_check.sh:70` discards it silently, and ROOT selects only the BINARY while the ESTATE comes from CWD. Both returned rc=1 for incompatible reasons. **Right invocation: `cd` into the estate, absolute path. Control that catches the wrong tree regardless: READ THE DENOMINATOR.**
- **THE DENOMINATOR IS A TWO-FOR-ONE CONTROL.** It catches the wrong estate (779 vs 288), and it also catches a DANGLING BINARY, because a binary that cannot answer produces no denominator rather than a wrong verdict. That is why Phase 4's result survives the dangling-pair hazard named later the same day: 779 is itself the proof the binary answered. **Reasoned from how the failure presents, not driven against a dangled pair.**
- **A ROW THAT READS PASS CAN BE THE ONE THAT MATTERS.** A mutation put one thread in two buckets; every leg passed because my own set union absorbed the duplicate. Only tell: per-bucket counts summing to 120 against a union of 119.
- **A BASELINE THAT FAILS FIRST RUN IS THE FIXTURE TALKING, AND THAT IS THE INSTRUMENT WORKING.** Fix the fixture, never the leg, and re-run the whole table.
- **A REFUSAL AT THE WRONG ALTITUDE ABORTS THE SWEEP IT PROTECTS.** Refusing per-thread on empty AC/AT would have died on the first of 119 criteria-less threads. **Population proof belongs at the SWEEP, and the denominator rides on every run.**
- **SPLIT AN INVARIANT FROM A FINGERPRINT.** `declaration == flat` is standing; `declaration == WIP` is true only at a fresh hop. Fused, it is a false finding waiting for next week.
- **A PREDICTED-ABSENT ID HAS THREE OUTCOMES, NOT TWO.** A prediction is only a control if its falsifying outcome is enumerated.

**MECHANICS.**

- **A CONTROL FITTED TO AN INSTANCE'S SURFACE FORM CATCHES THAT SURFACE FORM, AND ITS GREEN IS WORTH NOTHING.** I wrote a placeholder stamp into a correction THREE times in one day, the defect all three clock-guard arms are blind to. Twice the caution was already on this board -- **writing a caution down is not a control.** So I wrote a mechanical one, `grep -n '[0-9]xZ'`, fitted to the two instances I had seen because both ended in `Z`. **It returned CLEAN on the very next edit, in which I had written `13:5x`.** Third instance, missed by the control written for the first two, one edit after writing it -- **positive-control discipline skipped on my own instrument because I had just written it and it felt like a control.** Widened to the class: `grep -nE '[0-9]{1,2}:[0-9]*[xX]'`. **The repair is to DELETE the stamp, never improve it:** a time never read is not recoverable by thinking harder.
- **AN ANNOUNCED WINDOW IS A REASON TO WAIT, AND I READ IT AS A REASON TO HURRY.** Told a rebuild was coming "shortly", I wrote the words _committing before cc's rebuild window_ and ran the race; the explicit DO-NOT-COMMIT arrived while my call was in flight. **`.git/index.lock` stopped me -- not my judgement, not the announcement.** The hazard is real: during the build the binary dangles, so a commit inside the window loses the critic gate AND the thread-view-skew guard **while still printing `guards: N ran, 0 skipped`.** **cc's reframe, kept over mine: an early warning about a CLOSING window actively RECRUITS a peer into the hazard. A protocol whose failure mode is CAUSED by the notice is not a protocol with a timing bug in it.** The mechanical fix (refuse when the pair dangles) is the arm hv's ruling 4 did not reach; dc holds it.
- **PATH-SCOPED RESET IS WHAT MADE A BLIND RETRY SAFE.** That same failed attempt ran `git reset -q -- <my board>` while another node's files were staged, and left them untouched -- **verified afterwards, not assumed.** A bare `git reset` would have silently unstaged a peer's work with nothing reporting it. **That is the argument for rule 3's exact form, not just its habit.**
- **`git status` IS A SNAPSHOT OF WHOEVER IS MID-WRITE.** Wait and re-read; do not diagnose. Corollary: **do not read a peer's uncommitted diff to explain a symptom.**
- **`cmd | grep; echo $?` READS grep's CODE.** Capture to a FILE and measure the file.
- **`| head -N` IS `| tail -N`** -- it showed me 7 lines of 159 and hid 152 staged deletions.
- **BACKTICKS IN AN `echo` OPEN A COMMAND SUBSTITUTION**, and `<ST>` inside parses as a redirect; the whole call dies.
- **`--to-disk` WRITES THE EXTRACT, NOT ONLY VIEWS.** On a dirty canon it reverts a hand edit silently.

## Decisions

- **(hv, 2026-08-28, first-hand) THE AUTHORITY CHAIN IS A TREE:** hv -> `intent-vc` -> {`intent-{dc,ic,cc}`, `devbin-vc` -> devbin nodes, `conflab-vc` -> conflab nodes}. **My only channel up is `intent-vc`** -- not hv directly, not laterally.
- **(ic, 2026-08-28) MEASURING ANOTHER ESTATE IS NOT DIRECTING ITS NODES.** Phase 4 was Conflab work by an `intent-*` node: vc assigned it, vc got the report, Conflab stayed read-only.
- **(vc, 2026-08-28) Conflab post-hop is 123 canon == 123 st list, 4 flat, 119 bucketed and UNMOVED** -- `migrate.rs:47` never relocates a thread out of v2's buckets. Confirmed by measurement, not relay.
