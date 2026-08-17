---
id: "0038"
title: v3 exits 1 for an unimplemented command, so a migrated project's pre-commit gate blocks every commit
date: 2026-08-16
reporter: matts
status: CLOSED
severity: high
---

# 0038: v3 exits 1 for an unimplemented command, so a migrated project's pre-commit gate blocks every commit

## Tags

parity, exit-codes, hooks, migration, ST0056

## Summary

**A project that migrates to v3 while any command the pre-commit hook calls is still unbuilt cannot commit at all.** v3 exits **1** for "known command that is not implemented yet"; the shipped pre-commit hook reads exit 1 as **findings at or above threshold** and blocks the commit. The hook was designed to fail OPEN in exactly this situation -- its own header reserves `2+` for "the critic tooling itself is unavailable" -- and v2 honours that, exiting 2 for a bad or unavailable critic invocation. v3 collapsed "unavailable" into the code that means "your code is bad".

The user-facing result is a commit gate that refuses everything, with a remedy that cannot be followed: it prints `error: `critic` is a known command that is not implemented yet` and then instructs the user to _"review the findings above, fix them, and re-commit"_.

**No acceptance criterion would catch this.** AC-10.4 is scoped to `.claude/settings.json` + `.claude/scripts/**`; `.git/hooks` is not covered by any AC in the thread, and AT-10.4 (`migrate_hooks_continuity.rs`) is still `to-write`, so the test that would pin the meaning of "hooks" does not exist yet.

## Reproduction

Measured 2026-08-16 against `native/rust/target/release/intent` (3.0.0-dev), end to end through the shipped hook rather than by reading the case statement.

**The parity baseline (v2), which is what the hook's contract was written against:**

```
$ intent critic nosuchlang --staged --severity-min warning --format text
error: first argument must be a language (...)          -> exit 2
$ intent critic shell --files /dev/null --severity-min warning
ok: no shell findings at severity >= warning ...        -> exit 0
```

**v3, same question:**

```
$ intent critic shell --staged --severity-min warning --format text
error: `critic` is a known command that is not implemented yet
  remedy: nothing in this build provides it -- `intent --help` lists what does
                                                        -> exit 1
```

**End to end.** A throwaway project with `intent/.config/config.json` declaring `languages: ["shell"]`, one staged shell file, and the v3 binary first on `PATH`, running `lib/templates/hooks/pre-commit.sh`:

```
error: `critic` is a known command that is not implemented yet
  remedy: nothing in this build provides it -- `intent --help` lists what does

intent critic gate: commit blocked by findings at severity >= warning.
  review the findings above, fix them, and re-commit.
  to bypass (use sparingly): git commit --no-verify

HOOK EXIT: 1
```

**The blast radius is wider than `critic`.** Every unavailable form exits 1, including two that are not the same kind of event:

| invocation         | v3 message                                  | exit |
| ------------------ | ------------------------------------------- | ---- |
| `intent agents`    | known command that is not implemented yet   | 1    |
| `intent llm`       | known command that is not implemented yet   | 1    |
| `intent organize`  | unrecognized subcommand                     | 1    |
| `intent treeindex` | unrecognized subcommand                     | 1    |
| `intent critic`    | required arguments were not provided <LANG> | 1    |

## Root Cause

The hook's contract is documented in `lib/templates/hooks/pre-commit.sh`:

```
#   0  no findings at or above threshold (commit proceeds)
#   1  findings at or above threshold (commit blocked)
#   2+ reserved; hook itself always exits 0 or 1 after aggregating
```

and its dispatch honours it -- `1` blocks, `*)` prints `invocation error (exit $rc); fail-open`. So the fail-open path exists, is correct, and is simply never reached, because nothing v3 emits lands in it.

**This is an exit-code parity defect, not a hook defect.** `parity.md:101` already names this consumer explicitly: _"D17 carries v2's codes over, INV-02 forces an override of clap's default, and the pre-commit gate reads one -- so an exit code read through a pipe is the single most load-bearing number the harness produces."_ The gate reads one, and the number changed.

The not-implemented text has one home (`intent-cli/src/render.rs:420`), so the message is Highlander-clean; it is the **code** that is wrong, and the code is shared with genuine usage errors, which may want a different answer from "this command does not exist yet".

## Impact

**Every consumer, at the moment of migration, for as long as any hook-invoked command is unbuilt.** The failure is not subtle -- nobody can commit -- but three properties make it worse than an obvious breakage:

- **It arrives without being asked for.** Issue 0036 already records that `brew install` SHADOWS a v2 install rather than replacing it, so `intent` becomes v3 in every project on the machine at once. The first contact with this defect is in a project the user was not thinking about.
- **The remedy is unfollowable and looks like the user's fault.** "Review the findings above, fix them, and re-commit" names no file and no rule, because there is no finding. The only escape is `--no-verify`.
- **It trains the bypass.** A gate that blocks unconditionally is the always-red aggregate one level up: the first `--no-verify` is correct and unavoidable, and the habit outlives the cause.

WP ordering decides whether anyone meets it: WP-10 (migration) shipping before WP-07 (critic) puts every migrated project in this state.

## Proposed Fix

Not settled here -- the exit-code contract is ic's parity lane and the CLI is cc's build. Recorded so the decision is made rather than defaulted:

1. **Separate "unavailable" from "wrong".** v2's convention -- `2+` for invocation/availability errors, `1` for a substantive negative verdict -- is what every existing consumer was written against, and the pre-commit gate is only the consumer we happen to have measured.
2. **Distinguish the three cases now folded together**: a known-but-unbuilt command, an unknown subcommand, and a usage error are three different events and only the last is arguably the caller's fault.
3. **Close the AC gap.** `.git/hooks` is uncovered: AC-10.4 names `.claude/**` only. Either widen it or add a criterion that a migrated project can still commit. AT-10.4 is `to-write`, so this is the cheapest possible moment to fix the wording rather than the test.

## Related

- ST0056 -- v3.0.0; WP-07 (critic, embedded rules) and WP-10 (migration) are the two work packages whose ordering exposes this
- AC-10.4 -- hooks continuity, scoped to `.claude/settings.json` + `.claude/scripts/**`
- AT-10.4 -- `migrate_hooks_continuity.rs`, still `to-write`
- 0036 -- `brew install` shadows a v2 install, which is how a user meets this without asking for it
- 0016 -- runtime-resolved hook paths, the mechanism that makes consumer hooks survive the binary swap

## Resolutions

**The code fix has landed (cc, 2026-08-16). Clause 3 -- the AC gap -- has not, and it is vc's; this issue stays OPEN for it.**

### What v2 actually does, measured rather than inferred

The first thing the fix needed was the baseline, and measuring it **narrowed the change**. Run inside a real Intent project (`bin/intent`, this repository), because outside one v2 refuses everything at the project gate with exit 1 and every row looks the same:

| event                                    | v2  | v3 before | v3 after |
| ---------------------------------------- | --- | --------- | -------- |
| success                                  | 0   | 0         | 0        |
| unknown subcommand                       | 1   | 1         | 1        |
| usage error (a required argument absent) | 1   | 1         | 1        |
| a negative verdict from a gate           | 1   | 1         | 1        |
| the tooling cannot answer                | 2   | **1**     | **2**    |

**Two of the three cases this issue proposed separating were already right and had to stay 1.** v2 does not use 2 for usage errors generally -- it uses 2 in exactly one place, `intent critic` rejecting a language it does not have, which is the "this tool cannot answer" case. So the fix is one row, not three, and the other two rows are now pinned so they cannot drift into 2 either.

### The change

`spine::EXIT_UNAVAILABLE = 2` was **named in the doc comment above `EXIT_OK`/`EXIT_ERROR` and never declared**, which is the whole defect in one line.

The error channel was a bare `String`, which answers "what do I print" and nothing else -- and two further questions were already riding on it out of band: an EMPTY string meant "the verdict is on stdout, print nothing more" (four sites: the close gate, `at lint`, `doctor`, `sync`), and there was no way at all to say "this build cannot answer", so that case borrowed the code for "the answer is no". `spine::Failure` names the three kinds and the exit code follows from the kind rather than from whoever wrote the call site. `unwired()` returns `Failure::Unavailable`; nothing else changed meaning.

### Verified against the consumer, not just the number

The hook is unmodified. Its `2+` fail-open branch was correct all along and simply never reached. Driving the shipped `lib/templates/hooks/pre-commit.sh` against the v3 binary first on `PATH`, in a throwaway project declaring `languages: ["shell"]` with one staged shell file -- the same fixture that produced the report:

```
intent critic (shell) invocation error (exit 2); fail-open.
error: `critic` is a known command that is not implemented yet
  remedy: nothing in this build provides it -- `intent --help` lists what does

HOOK EXIT: 0
```

### The guard that existed and could not fire

`tests/exit_codes.rs` already carried `the_critic_exception_is_not_flattened_by_the_override`, whose doc comment said it existed "so a blanket always-exit-1 cannot pass" -- **and a blanket always-exit-1 is what shipped.** It ran `intent critic --help`, which exits 0 with an empty stderr, and asserted `code != 2 || !stderr.contains("unexpected argument")`: the first disjunct was always true, so the assertion held for every possible behaviour of the binary. It has been replaced with one that asks for the code on an invocation that FAILS.

Three tests now cover this, and the mutation (reverting `Unavailable` to `Error`) reds all three, including the end-to-end one:

- `the_unavailable_exception_is_not_flattened_by_the_override` -- the replacement, no disjunction
- `an_unbuilt_command_is_not_the_same_event_as_a_bad_invocation` -- all three codes asserted TOGETHER, so changing every code to one new value cannot pass
- `a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt` -- drives the shipped hook

### CLOSED 2026-08-17 (vc) -- re-measured end to end at `3088c39c`, clean tree, and both outstanding clauses discharged or routed

**The reproduction was re-run from scratch rather than re-read.** Throwaway v2 project, `languages: ["shell"]`, the shipped `lib/templates/hooks/pre-commit.sh` installed at `.git/hooks/pre-commit`, one staged file, the v3 debug binary rebuilt from a verified-clean tree and reached through a shim directory on a `PATH` scoped to the single `git commit` invocation:

```
intent critic (shell) invocation error (exit 2); fail-open.
error: `critic` is a known command that is not implemented yet

commit rc=0   -- and the commit is in the log
```

**The commit lands.** Verified by counting commits in `git log`, not by trusting the return code.

**The blast-radius table from Reproduction, re-measured at HEAD:**

| invocation         | v3 message at HEAD                          | exit | vs filing |
| ------------------ | ------------------------------------------- | ---- | --------- |
| `intent agents`    | known command that is not implemented yet   | **2** | fixed |
| `intent llm`       | known command that is not implemented yet   | **2** | fixed |
| `intent organize`  | unrecognized subcommand                     | 1    | unchanged |
| `intent treeindex` | unrecognized subcommand                     | 1    | unchanged |
| `intent critic`    | required arguments were not provided <LANG> | 1    | unchanged |

**32 commands now answer 2**, which is the fail-open population the hook's `*)` branch was always waiting for. **The four that stayed at 1 are exactly the retired ones** -- `organize`, `treeindex`, `help`, `st_zero` -- because a retired command is removed from the clap surface and never reaches dispatch, so the exit-code work cannot see it. **That residue is 0044's Proposed Fix 1 and is not this issue's**; the pre-commit hook calls only `critic` and `info`, neither of which is retired.

**Clause 3 is DISCHARGED, and by a route this issue proposed against.** It asked for AC-10.4's path list to be widened to include `.git/hooks`. That is the wrong instrument and the argument is recorded in AC-10.9: a byte-identity criterion cannot see a semantic break, so adding a path buys coverage in the axis that already worked and none in the axis that failed. **AC-10.9 covers `.git/hooks` behaviourally instead** -- drive the shipped hook, assert the commit lands -- and AT-10.9 cites the test cc built.

**Clause 2 (WP-07) stands and is not blocking.** `intent critic` with no `<LANG>` exits 2 in v2 and 1 in v3. It is a declared parity row (INV-02) that cannot be pinned until `critic` exists, which is exactly what this issue's own note says.

**One thing found while re-measuring, and it is why AC-10.9 was widened the same day: this fix's fixture is MIGRATED, because that is the state this issue was about.** The unmigrated project -- the state every project in the estate occupies until WP-10 runs on it -- inherited that scope through the criterion, the test and every instrument pointed at it, and is covered by none of them. `Facade::open` gates on migration first, the refusal maps to `EXIT_ERROR`, and the same hook blocks the commit on it. **It does not reproduce today only because `critic` is unbuilt and lands in the very fail-open branch this issue's fix created.** Filed as **0045**, latent, with the polarity table that explains why no code choice fixes it.

### Outstanding (at filing -- see the closure above for their disposition)

- **Clause 3 (vc).** `.git/hooks` is still uncovered: AC-10.4 names `.claude/settings.json` + `.claude/scripts/**` only. AT-10.4 is still `to-write`, so the wording is still the cheap thing to fix rather than the test.
- **One divergence deliberately not fixed here (WP-07).** `intent critic` with NO language exits 2 in v2 (its own arg parsing) and 1 in v3 (clap's usage error, INV-02). When WP-07 builds `critic`, its language validation owes v2's 2 -- the row exists in v2 and v3 cannot reach it yet, so pinning it now would assert a path that does not exist.
