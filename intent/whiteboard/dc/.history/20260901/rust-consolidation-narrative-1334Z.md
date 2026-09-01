# The Rust consolidation day -- reasoning, 2026-09-01

Archived at the 1334Z localfold. The board keeps the CLASSES; this keeps why.

## What the day actually was

hv asked why the build was slow, and the answer turned out to be a ruling hv made on 2026-08-27 and Intent never applied. Laksa applied it the same day and quotes it in its manifest. Lamplight had applied it. **Intent, the estate that made the ruling, had 257 separately linked test targets, 201 of them added in the month the ruling was live.** A ruling not applied where it was authored is not a slow rollout; it is a ruling nobody enforces, and the author is the last to notice because they remember deciding it.

## The numbers, because they reframe the exercise

    target tree held                     113 G  (cc 41G, dc 11G, private 919M)
    time to DELETE it                    ~35 min
    time to REBUILD BOTH BINARIES COLD   1m 36s
    target tree after                    6.0 G
    suite after                          2022 passed, 0 failed, 14 suites

The cache cost more to hold than to regenerate. Boundary that must travel with it: 1m36s is `build all`, the two RELEASE binaries; the TEST build is the expensive half and is where the 400,000-file debug tree lived.

## Why 52G was per-node forks, which is not a discipline problem

Cargo's build lock works correctly and INVISIBLY. A blocked build is indistinguishable from a hung one, so a node "fixes the hang" by forking `CARGO_TARGET_DIR`. I caused an instance in front of hv: my `cargo check` held the lock, hv's test run queued, hv reported "the rust tests look like they're hanging". So a prohibition only holds if the wait is made visible -- both halves or neither.

## The 50-minute outage, and it is an ORDERING defect

`0196` deletes the shared pair before building; the shared-artefact guard redirects a dirty build to `target/private/`. Together the build deleted `release/` and then declined to replace it. **The guard prints the dangling symlinks BEFORE the delete, so the information exists and the ordering defeats it.** And either node's single uncommitted file kept the build redirected, so neither of two sessions could restore service alone -- while the pre-commit gate refuses on a dangling `intent`, so the fix was blocked by the outage. Broken by both of us stashing path-scoped, then one build.

## The class this day was about, and I was its worst offender

An instrument that cannot exhibit the failure it is checking for returns clean, and clean reads as evidence. Instances, mine unless noted:

1. Grepped `cmd/clean.d/` for warnings. The directory does not exist -- `clean` is a builtin. Empty read as absence; I told hv devbin carried no warning when it warns thoroughly.
2. Proved the orphan guard with `--exact` on a bare name. The test is in a module, so it selected 0 tests and exited 0. **The baseline arm added AS A CONTROL passed the same vacuous way.**
3. Ran a flake check 8 times, then 8 more, against a filter selecting nothing -- `--exact` needs `--` to reach the harness.
4. Told cc nothing asserted `at set`. My grep was `'at set '` with a trailing space; the assertion is `contains("at set")`.
5. Census counted test FILES as targets, and counted every crate once per git WORKTREE. Reported Lamplight at 1-of-10 when it is 1-of-1 -- **one step from becoming fleet canon**.
6. Claimed `daemon_subscriptions` fixed on 8 clean runs. hv hit it immediately. Two-sided afterwards: 0 failures in 10 parallel, 2 in 9 serial -- **serialising made it WORSE, so the stated cause could not be the cause.**
7. Positive-controlled the `time`/`tee` pattern with a subshell and an external command. `dvb` is an autoload stub and neither could exhibit it. Then proposed job control -- a mechanism I could not test -- instead of running `type dvb`, one command, which ended it.

Peers hit the same shape: cc's `-i FAILED` matching `0 failed`; cc's per-crate verification passing while the crate that PINS its value went red; cc's `pkill -f` on a pattern every session runs; lamplight-vc's `stat -t` re-stamping local as UTC.

**The keeper, in lamplight-vc's words: a guard must say that it ran. An instrument whose "nothing to report" is byte-identical to its own absence is unreadable as evidence in either direction, and no care by the reader repairs it.**

## The discriminator that matters more than any instance

**Does the test need isolation, or does the product need fixing?** `dual_path_conformance` (mutates cwd) and `daemon_subscriptions` (per-process event stream) kept their own targets because a shared process cannot give them what they ask for. `daemon_address` looked identical and was not -- isolating it would have lowered concurrency until the window stopped being hit, converting a product race into a green. cc fixed it properly: a forked child inherits the lock fd, `O_CLOEXEC` clears the copy only at `exec`, so the holder's own close does not release it.

## Corrections I owe and took

- Told laksa-vc twice that their hold could lift, both times attaching my reading of hv's SCOPE to a fact that was mine to report. They refused both times and were right. **Marking uncertainty had become a substitute for resolving it.**
- Told cc "only you can clear them" when my own unstaged file was equally blocking.
- Inferred "Intent may edit estates" from "estates never edit Intent" -- flagged as an inference and then used as established. vc caught it before I acted.
