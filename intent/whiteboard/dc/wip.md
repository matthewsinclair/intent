---
node: dc
name: DevX Claude
role: worker
session_id: 10363f5b-9b74-464d-9671-dced031b51c6
heartbeat_at: 2026-08-18 15:57Z
status: paused
focus: "FOLDED AND PAUSED (aggressive localfold) while hv and vc do surgery. HALF A IS SHIPPED -- bin/intent:55 carries critic at 92a51134, pushed local, and the gate is no longer dark: languages returning rc=2 went 5 -> 0, confirmed live on my own commit. Rig COMMITTED at parity/tools/critic_global_rig.sh; RIG_CANARY=1 reproduces the dark gate on demand. Half B SCOPED at aa220755 and NOT started -- mostly declarations, and its design question is with hv. EVERY remaining item of mine is blocked on hv."
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

Nothing in flight. Folded and paused on hv's instruction while hv and vc do surgery.

## TODO

**Everything below is blocked on hv. Nothing here is mine to start.**

1. **Half B -- SCOPED (`aa220755`), NOT STARTED, and it must not start until hv rules the design question.** 1 clean arm / 3 at a stated cost / 4 inexpressible under the runner's contract / 5 declare-none. **The question: 7 of 13 rules name `shellcheck` or `clippy` in their OWN Detection text and the runner can only `grep`** -- so for the two languages that have a real parser, the gate is barred from using it. **Nine declarations written on the premise that grep is the only instrument would all need revisiting**, which is why settling it first is sequencing rather than caution. Per-rule table in `critic-gate.md`.
2. **The hook's fail-open branch (`pre-commit.sh:288-292`) -- WITH hv, named SEPARATELY from Half A.** Half A works AROUND that branch; it does not fix it. **The pending `critic` exit-code narrowing (a migration refusal returns `2`) would put a third condition in the bucket the hook discards, recreating this exact defect in v3 with a ruling behind it rather than by accident.** The fix is not the exit code, which is right; it is that the fail-open was written for a MISSING critic and now covers a PRESENT one that is REFUSING. Two candidate discriminators went up as candidates; neither vc nor I chose. **Whatever lands is proved by a RED.**
3. **AC-11.5 -- the release artefacts still name no commit. Mine to carry, NOT blocked on cc.** Measured at `4ef953db`: **both** `release/intent` and `release/intentd` now read `dirty-4ef953db`, so their bytes match no commit. **They moved since my earlier reading** (`release/intent` was `dirty-bb0baf85` and `release/intentd` carried NO marker at all), so someone rebuilt them -- **the number moved for a reason that is not mine, and the remaining need is unchanged: a rebuild from a CLEAN tree.** The binary arm itself is BUILT (`b11ca6ac`, corrected `a13bd338`); the `deferred.md` line saying otherwise was stale.
4. **`doctor` prints `intent v2.19.0` while auditing a `3.0.0-dev` project -- THE HOLD HAS EXPIRED, so this is now RAISEABLE.** It was held only until the release-script item landed so the two would not compete in one channel. **vc landed that item.** Nobody has re-checked the condition; I did, at this fold.
5. **`--skip-rust-tests` -- with hv as an option, deliberately not decided.** vc put it up rather than choosing, on my point that it must not reach hv as a clean one-liner.
6. **The shim: HOLD.** It DEFEATS Half A by routing away from v2's dispatcher, and it pointed at a schema-stale binary.

## Watch-outs

- **THIS REPO IS A v3 PROJECT** (`3.0.0-dev`; store `intent/.cache/intent.db`, gitignored). v2 refuses PROJECT verbs at exit 2; `GLOBAL_COMMANDS` run fine. **UPDATED BY MY OWN CHANGE AT `92a51134`: `critic` IS NOW A GLOBAL COMMAND**, so `intent critic <lang>` dispatches and no longer refuses -- that is Half A and it is deliberate. `st`, `wp`, `at`, `ac`, `todo` still refuse. **Invoke v3 by explicit path, and use `target/debug/`, not `release/`, which runs schema generations behind. The shim is NOT landing.**
- **PUSH `local` ONLY.** `upstream` closed (hv, CI/CD budget).
- **NEVER `git pull --rebase` here; a peer `.git/index.lock` means WAIT. ALWAYS `git commit --only <paths>`** -- a bare commit sweeps a peer's staged index, and `--amend` ignores `--only`.
- **NEVER mutate `bin/**` or `tests/**` in place while anything runs them**; both PATH symlinks point into `bin/intent`. **Check for a live suite first.**
- **COMMIT BEFORE ANY `intent at` STATUS CHANGE** -- `intent at red|green|na` DESTROYS the row note (issue 0033).
- **`cargo test` IS A CONCURRENT WRITER ON THE ESTATE** (vc, reproduced in a clean clone on a green 598-pass run; `dispatch_ssot.rs` fixed at `1ff7f2c1`, two root-reaching readers remain). **Any measurement against the live estate has an uncontrolled writer under it unless nothing is running.** `git status --porcelain` cannot see it -- `.gitignore:127` ignores the store, **so the release verifies the generated views and never the SSOT.**
- **A PEER CANNOT WAIVE AN hv RULE**, and being right on the substance is not the same as being entitled to say so. Same shape: a peer cannot authorise what a harness refused.
- **MY SHELL IS zsh AND IT DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** Twelve paths passed as one argument came back plausibly green. Use a bash array.
- **NEVER `$?` AFTER A PIPE.** Three of us hit it today; ic reported rc=0 for a tool that exited 2.
- **`git log` prints LOCAL time**; reading one and appending `Z` is wrong by exactly the offset and looks perfect.
- **macOS: signing MUTATES the binary, notarisation does NOT -- checksum AFTER signing.** Only a quarantined copy under `spctl` means anything.
- **`target/release/` IS SHARED MUTABLE STATE.** Private `CARGO_TARGET_DIR`; never sign there.
- **A control refuses; documentation reminds; only one is load-bearing.** My own watch-outs are not controls.
- **THE HALF-A CEILING NO LONGER LIVES ON THIS BOARD.** The shell / rust / elixir proxy-coverage table and its denominator are `## HALF B` in `critic-gate.md`, which is committed. **Two copies drift; the doc is the one -- and restating the figures here is where the drift would have started.**

## Decisions

- **THE DAY RULE, and every finding and every error of mine reduces to it: A GREEN PRODUCED BY SOMETHING OTHER THAN THE THING UNDER TEST.** **None was caught by care. Each was caught by MOVING something** -- the target dir, the subject, the clock, the desk.
- **PROVE THE DISCRIMINATOR DISCRIMINATES BEFORE TRUSTING IT.** Six of my own instruments failed this way in two days, each looking exactly like the finding it hunted: `bash -n` on a `.bats` file; `bats --count` on a copy that cannot resolve its helper; `info` printing one banner from both binaries; a sweep reporting `examined: 0` on an estate of 51; `grep -c ... || echo 0` printing two values; and **`across N file(s)` in critic output, which echoes argc -- a nonexistent path reports "1 file(s)" and clean.**
- **A ZERO IS NOT A RESULT UNTIL THE CHECK HAS PRODUCED A NON-ZERO.** I swept 51 shellcheck directives, nominated 16, and confirmed all 16 sound -- **but only reported it after deleting one directive and watching the finding reappear.** Same reason the bats estate-digest verdict is trustworthy: one byte moved it and `git checkout --` returned it.
- **A PROXY IS NOT THE PARSER.** My structural sweep nominated 16 and shellcheck cleared every one. **Nominate with a proxy; adjudicate with the real tool.**
- **A CLASSIFIER KEYED ON A MESSAGE IS BLIND TO THE POPULATION THAT NEVER PRINTS ONE.** 297/2 was really 299/0; `[[ ]]` prints nothing on failure. **Both classifications agreed on "2 remain".**
- **VERIFY THE PREMISE OF A QUEUED ACTION AT THE MOMENT YOU ACT ON IT.** I wrote that rule and then handed vc a task built on a stale one. **Today it paid: hv asked for shim install commands and the premise had gone false since "go for it".**
- **A CONFLATION CAN BE RE-RULED IN, AND A CORRECT EXIT CODE IS WHERE IT HIDES.** The v2 dark gate was an ACCIDENT of hoisting: two conditions sharing exit 2 and a hook that cannot separate them. The pending v3 ruling puts a THIRD condition in the same bucket **on purpose and for good reasons**, so the next instance arrives with an argument for it rather than as an oversight. **Defensible at the surface that emits it, fatal at the surface that reads it** -- and only visible by looking at both ends at once, which is the same two-sided blindness as test 2 and Half B.
- **A DEFERRAL NOBODY RE-VERIFIES IS PICKED UP AS LIVE WORK THAT WAS ALREADY DONE, AND THE PICKUP IS THE HARM RATHER THAN THE DELETION.** ic found that a pickup recorded only in `deferred.md` evaporates when the file dies. **The sharper form is the opposite direction: while the error sits in a file with an end condition it dies on schedule; copied onto a board it acquires a name, an owner and permanence -- and a named owner READS AS VERIFICATION to the next reader.** The prescribed remedy is what makes the stale premise durable. **One day old, and one of its four facts on my side was already false.** Cost of catching it: three git commands and a `strings`. **THE RE-VERIFICATION IS CHEAPER THAN THE WALK.**
- **AN INSTRUMENT THAT LIVES IN A SCRATCHPAD IS A CLAIM, NOT A CONTROL -- AND THE DOCUMENT DESCRIBING IT READS IDENTICALLY EITHER WAY.** `critic_global_rig.sh` was cited by name, its internals described in detail, in the paragraph telling a cold re-driver how to use it. **It was never committed at any point in history and was on no disk.** The other 39 instruments under `parity/tools/` are all committed; mine broke the convention and nothing caught it, **because prose about a thing is indistinguishable from prose about a thing that exists.** Same shape as ic's `deferred.md` finding one artefact over: the record survived and its subject did not.
- **THE CANARY IS THE DELIVERABLE, NOT THE TEST.** 6/6 with the fix says almost nothing on its own -- it is the same green the dark gate was emitting. **3/6 with the control, where the commit titled _"this commit must be REFUSED"_ gets CREATED, is the entire finding reproducible on demand by anyone, forever.** ic asked for a critic that FAILS on purpose; the more useful artefact turned out to be a gate that PASSES on purpose, on command.
- **SILENT IS A THIRD STATE, AND IT IS THE ONE THE GATE CANNOT REPORT.** Shell and rust rules are not "unarmed" -- they carry no proxy AND no declaration, and `critic_runner.sh:18` skips a proxy-less rule **silently**. Elixir's ten unarmed rules DECLARE the absence and name the subagent instead. **Armed / declared-none / silent are three states, the estate's vocabulary had two, and the missing word is where the defect lived** -- `critic shell` returns rc=0 because nothing ever asked a question, which is indistinguishable from clean.
- **A CEILING IS NOT DISCOVERED BY ATTEMPTING THE WORK; IT IS DISCOVERED BY READING WHAT THE INSTRUMENT ACCEPTS.** Four of thirteen rules were decided by `critic_proxy_is_simple`'s refusal of `-v` and `-L` before I formed any opinion about them. **Scoping the tool's contract first turned a 13-rule judgement call into a 9-rule one.**
- **A SELF-REPORT IS EVIDENCE WHEN IT IS SOURCED INDEPENDENTLY OF ITS SUBJECT AND BOUND TO IT -- AND "SHARED WRITER" IS THE WRONG DISCRIMINATOR, BECAUSE IT IS PRESENT IN THE CASE THAT WORKS.** vc generalised my narrow rule to _"not evidence whenever the self-report and the thing reported share a writer"_ and **that form forbids AC-11.5's own ratified remedy** -- the source-commit marker shares a writer with the bytes and is the estate's answer to _only the artefact can answer what was_. **It is COUPLING that varies, and it fails at BOTH ends:** the vendor manifest is derived FROM its subject, so `27 of 27` compares a thing to a copy of itself (my rig's own abort condition); the AC status field is decoupled ENOUGH TO BE EDITED while reality holds still; **the marker alone is sourced from git -- outside the bytes -- and bound into them, which is why it can DISAGREE, and did (`release/intentd` carries nothing, `release/intent` says `bb0baf85` at HEAD `ce532a97`). A vacuous report cannot produce a disagreement; that is how you tell them apart.**
- **MY OWN NARROW FINDING ACQUIRED ITS DEFECT IN BEING GENERALISED, WHICH IS THE THIRD TIME TODAY.** The `cargo test` one-liner, Half B as a one-liner, and now this. **The narrow form was a MEASUREMENT (two observed mis-sets); the general form swapped it for a structural property, and the property picked was the one shared with the working case.** A rule that covers less and holds beats one that reads better. **And I had to catch this one in my OWN rule after a peer improved it** -- the improvement is where I stopped checking.
- **A ROW THAT HAS MIS-SET ITS OWN STATUS FIELD IS THE LAST THING TO TRUST ABOUT ITS OWN STATUS.** AC-11.5 has done it twice, in opposite directions, from its own prose. **So the evidence admissible about that row is commits and artefacts, which have no such failure mode** -- and that is a general rule, not a courtesy to one row.
- **A RECORD NAMES THE COMMIT IT COVERS, NEVER "HEAD".**
- **THE PUSH RESULT CARRIES NO INFORMATION ABOUT THE REMOTE IN EITHER DIRECTION.** Only `git ls-remote` plus `merge-base --is-ancestor` is evidence.
- **A DOCUMENT WRITTEN FOR COLD PICKUP MUST CARRY ITS OWN STALENESS.** vc's re-drive caution arrived in a peer message, which archives; the numbers it qualifies sat in a committed doc, which does not. **The caveat has to live in the same file as the figure, or the figure outlives it.**
