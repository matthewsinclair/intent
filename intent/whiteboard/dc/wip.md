---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-18 07:33Z
status: paused
focus: "PAUSED at an aggressive fold. **THE REPO IS HOISTED (`0ec2ac79`) AND THE PRE-COMMIT GATE PASSES WHILE ENFORCING NOTHING** -- `intent` on PATH is v2, v2 correctly refuses the 3.0.0-dev tree at exit 2, and 2 is the code the gate FAILS OPEN on, so all five languages fail open and the gate reports success. Three correct behaviours composing into the largest instance of the day. **SUITE FIXED AND LANDED (`e474b419`, pushed, verified off the remote):** 299 of 1403 failures were ONE cause -- fixtures declare 3.0.0 while `bin/intent` is v2.19.0 -- and the escape hatch was documented in the helper own comment with NOTHING in the tree setting it. **BLOCKED ON hv, BOTH OUTWARD-FACING: (1) the shim**, built and driven three arms, install REFUSED by the auto-mode classifier writing to `~/.local/libexec`, so hv must grant a Bash rule or run it; **(2) the exit-2 overload fix in `bin/**`**. **THE ONE LESSON: a green produced by something other than the thing under test** -- plus three rehearsal failures with one hole, the copy cannot hold what the real subject has."
claims: [ST0056/11]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.** No clauses. hv ruled it four times and it was reinterpreted after three of them, twice by me inside ten minutes.

- **You never ask what time it is.** Not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and then writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap that two writers interleave in, so two records get stamped in the opposite order to the one they were written in.
- **hv's structural close, and it is the form to build against: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times, and every one returned was set by SQLite on a record. **Direction is not symmetric -- IN is forbidden, OUT is fine**: a returned time is evidence a record was written; an accepted time is a second clock with extra steps.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION WILL HOLD.** Every earlier statement of D42 asked where a value came from and whether its source was legitimate -- judgement calls this estate failed three times in one day from three nodes. **A time-typed input parameter is a defect by inspection.** Asking where a caller got a value is a discussion; asking whether a signature accepts one is a grep. **A signature that accepts a time is a standing invitation that gets accepted eventually**, no matter how careful today's author is.
- **The defect is always one step earlier.** Reaching for a clock means you are about to write a time into something that is not a durable record. The fix is never a better clock; it is not writing the time.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it's just a label". The third fooled cc, vc and me independently -- **better provenance is not the absence of a confection.**
- **SCOPE, hv: devbin is NOT Intent** -- external, vendored, no db, does what it likes with time. My whole D42 directive resolved to a no-op, and reporting that beat inventing work to look responsive.
- **A board stamp is a label, not data, and nothing may read it as one.** The ordering that exists and cannot be fabricated is the **commit**. The `## (...)` heading is legacy, kept only because nodes parse the shipped format live.

## The truth model -- canon, held not restated

Ratified in `design.md` (D01 as reversed) and `data-model.md`, and deliberately not duplicated here, because a second home is how two copies drift. The three points that change what I DO: **the db is the durable SSOT and files are re-creatable secondary artefacts**; **the typed API is the only door in**, so conformance is by construction rather than by anyone checking; **migrations are normal** -- anything justified by "we can never migrate" rests on a constraint nobody made.

## DOING

- **NOTHING IN FLIGHT.** The suite work is landed (`e474b419`); the two things left in my lane are both with hv, below.

## TODO

1. **THE SHIM -- BUILT, VERIFIED, NOT INSTALLED, AND THE INSTALL WAS REFUSED BY THE HARNESS.** hv said "go for it"; writing to `~/.local/libexec` was denied by the auto-mode classifier. **hv must add a Bash permission rule or run the install.** It resolves the project the way v2 `find_project_root` does, reads the declared version, and execs the matching binary. Driven three arms: v2 project -> v2, v3 project -> v3, outside a project -> v2. **The third arm is deliberate: `init` and `bootstrap` are NOT implemented in v3.** Source in the session scratchpad as `intent-dispatch`; fresh v3 built from HEAD in a private target dir.
1. **THE EXIT-2 OVERLOAD FIX -- vc ADOPTED IT, IT LIVES IN `bin/**`, SO IT IS hv's.** `bin/intent:277` (version refusal: permanent, project-wide) and `bin/intent_critic:89,95` (invocation error: transient, per-language) share exit 2 across TWO PROGRAMS, and the wrapper refuses before dispatch so the critic own 2 never happens. **Give the version refusal its own code; the gate then refuses on it and fails open on 2 with no new logic and no message-matching.** Blast radius is two files. **3 IS NOT FREE -- `bin/.devbin/lib/helpers:66` uses it.**
1. **THE INDEX (vc tasked, 2026-08-18).** hv retired the WIP/DONE directories: `intent/st` and `intent/issues` become an index plus render-on-demand. `intent/st/steel_threads.md` already exists, 66 lines, newest-at-top, WIP first -- **it needs links into each thread, and `intent/issues/` has no equivalent at all.** The reader is a human in an editor, not a parser. **40 issues numbered 0001-0061 WITH GAPS -- the highest number is not the count, and an index should not hide the gaps.** Additive and safe to build now, but **NOTHING MAY DELETE A FILE until ic conservation returns zero** for `LOST-PROSE` and `UNACCOUNTED`. **Issue bodies are the ONLY copy of all 40 and cc owns the `body` field.**
1. **REBUILD BEFORE MEASURING.** My own provenance check fired on the last commit: `native/rust/target/release/intent` names `32708b02` while the checkout was `607951e7`, and **`intentd` carries NO source-commit marker at all though its crate has the embed -- those bytes predate it.**
1. **Carried, unchanged:** AC-11.1 / AC-11.4 still need one hv publication (nothing technical blocks either); AC-10.4 subject pinned (4 files, 8039 bytes, shas in `14c3fb01`); `devbin upgrade` writes the manifest with the `lib/install` it has just replaced; 0048 awaiting hv; issues 0030 / 0031; `testkit::project_fixture()`.

## Watch-outs

- **THIS REPO IS NOW A v3 PROJECT, AND THE ENTRY HERE PREVIOUSLY SAID THE OPPOSITE.** It declares `3.0.0-dev`, the store is `intent/.cache/intent.db` (gitignored, 3.3M), and **v2 refuses every verb here at exit 2.** Until the shim lands, invoke the v3 binary by explicit path.
- **PUSH `local` ONLY.** `upstream` is closed (hv, 2026-08-16, CI/CD budget). Now enforced.
- **NEVER `git pull --rebase` in this shared tree; a peer `.git/index.lock` means WAIT, never remove it. ALWAYS `git commit --only <paths>`** -- a bare commit sweeps a peer staged index, and `--amend` ignores `--only` entirely.
- **NEVER mutate `bin/**` or `tests/**` in place while anything is running them.** Both `~/.local/bin/intent` and `~/bin/intent` symlink into `bin/intent`. **And check for a live suite before starting one: I raced hv twice today.**
- **COMMIT BEFORE ANY `intent at` STATUS CHANGE.** `intent at red|green|na` DESTROYS the row note (issue 0033).
- **A PEER CANNOT WAIVE AN hv RULE, AND BEING RIGHT ON THE SUBSTANCE IS NOT THE SAME AS BEING ENTITLED TO SAY SO.** vc told me committing my own fixes never needed authorisation. They were right; I asked hv anyway. **If I take a peer read on when an hv rule applies, their being right is luck rather than method.** Same shape as: a peer cannot authorise what a harness refused.
- **MY SHELL IS zsh AND IT DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** `"$B" $v` with `v="st list"` passes ONE argument. It bit cc and me on the same day, and **both times the harness failure was indistinguishable from the finding being hunted.**
- **NEVER `$?` AFTER A PIPE** -- you get the last stage status. Three of us hit it today.
- **`--date=format:` IGNORES `TZ`; `--date=format-local:` RESPECTS IT.** `git log` prints LOCAL time; reading one and appending `Z` is wrong by exactly the local offset and looks perfect.
- **macOS: signing MUTATES the binary, notarisation does NOT -- checksum AFTER signing.** The only check that means anything is a quarantined copy under `spctl`; `codesign --verify --strict` returns 0 on an ad-hoc signature.
- **`target/release/` IS SHARED MUTABLE STATE.** Build into a private `CARGO_TARGET_DIR`; never sign there.
- **A control refuses; documentation reminds; only one is load-bearing.** My own watch-outs are not controls and I should stop expecting them to be.

## Decisions

- **THE DAY RULE, and every finding and every error of mine reduces to it: A GREEN PRODUCED BY SOMETHING OTHER THAN THE THING UNDER TEST.** A binary arm dead behind an `exit 0`; a fixture that passed because cargo put the binary inside the repository; a refusal that read `git status` and spoke about the bytes; a checker reading a marker positionally while the gate read it anywhere. **None was caught by care. Each was caught by MOVING something** -- the target dir, the subject, the clock, the desk.
- **PROVE THE DISCRIMINATOR DISCRIMINATES BEFORE TRUSTING IT.** Three of my checks in one day discriminated NOTHING and each looked like a pass: `bash -n` on a `.bats` file fails identically on the control, because `@test "..." {` is not bash; `bats --count` on a scratchpad copy fails identically because it cannot resolve `../lib/test_helper.bash`; and `info` prints the same banner from v2 and v3. **Running the control exposed all three.**
- **THE REHEARSAL POPULATION CANNOT HOLD WHAT THE REAL POPULATION HAS -- three mechanisms in one morning, and "clone and test" reads as the CAREFUL option, which is why it wins.** vc: the real subject holds `intent/.cache/intent.db`, which no clone can contain because it is gitignored. cc: the real subject held my fix, which no clone could contain because it was uncommitted. **Mine is the only one where instrument and subject were both fine and the INTERVAL was the defect** -- a peer changed the state between my two readings and I told them their live finding was a phantom. **The remedy differs: the other two are answered by looking at the real subject; mine by re-reading at the moment of the claim.**
- **A LISTING IS NOT AN INVENTORY, AND `--help` IS SERVED BEFORE DISPATCH.** v3 help lists 31 commands; `intent config --help` succeeds while `intent config` says "not implemented yet". **My first coverage probe was void, and my second -- counting parent verbs -- conflated "no implementation at all" with "bare form unimplemented, subcommands fine".** The leaf count was 6 of 31, **a floor with a stated depth limit of two levels**, which is why I missed `claude ws` and cc found it.
- **A CLASSIFIER KEYED ON A MESSAGE IS BLIND TO THE POPULATION THAT NEVER PRINTS ONE.** I reported hv failures as 297 version-guard / 2 other; it was 299 / 0. The two stragglers assert with `[[ ]]`, which prints nothing on failure. **The broken classification and the correct one would have agreed on "2 remain"; only reading a failing block in full told them apart.**
- **A LEGEND MUST BE DERIVED FROM THE DATA IT LABELS.** My check printed a hit for `exit 3` and a hardcoded "(empty = 3 is unused)" underneath it -- an hour after I said the same thing about someone else instrument.
- **BYPASS IS A RESPONSE TO NOISE, NOT TO STRICTNESS** (mine, vc adopted). A gate that is deterministic, fires once, has one remedy, and cannot fire spuriously does not teach bypass. **Five fail-open lines on every commit is what teaches people to stop reading gate output.**
- **VERIFY THE PREMISE OF A QUEUED ACTION AT THE MOMENT YOU ACT ON IT.** I wrote that rule and then handed vc a task built on a stale one.
- **A RECORD NAMES THE COMMIT IT COVERS, NEVER "HEAD"; A MEASURED FIGURE NAMES ITS SUBJECT AND REVISION.** A ceiling and a total render identically; a filter that cannot discriminate does not announce itself.
- **MIS-CREDITING TOWARD YOURSELF GETS CAUGHT; MIS-CREDITING AWAY FROM YOURSELF DOES NOT.** ic credited me with `same_end_state_check.sh`; I doubted it, checked, and my own history said I built it (`66ba461d`). **Checking was right even though the answer was "yes, yours".**
- **THE PUSH RESULT CARRIES NO INFORMATION ABOUT THE REMOTE IN EITHER DIRECTION.** Only `git ls-remote` plus `merge-base --is-ancestor` is evidence.
