---
id: "0045"
title: exit 1 blocks the git gate and exit 2 blocks the Claude Code gate, so no non-zero code is safe in both -- and the migration refusal returns the one that blocks every commit
date: 2026-08-17
reporter: matts
status: OPEN
severity: high
---

# 0045: exit 1 blocks the git gate and exit 2 blocks the Claude Code gate, so no non-zero code is safe in both -- and the migration refusal returns the one that blocks every commit

## Tags

exit-codes, hooks, migration, latent, measured, ST0056

## Summary

Intent has two hook consumers and they read failure with **opposite polarity**. The git pre-commit gate blocks on **1** and fails open on everything else. Claude Code's `UserPromptSubmit` blocks on **2** and lets everything else through. Both were measured, in separate rigs, and both are working as designed.

**The consequence has not been stated anywhere until now: there is no non-zero exit code a command can return that is safe in both consumers.** `1` blocks commits. `2` blocks prompts. The only code that blocks neither is `0`, which is the silent-success shape this whole thread exists to prevent. A command that genuinely cannot answer must therefore pick which consumer to break.

**And the migration refusal picked the one that blocks commits.** `Facade::open` gates on migration before anything else, so every facade-opening command in an unmigrated project returns `FacadeError::Unmigrated`, which maps to `EXIT_ERROR` = **1**. Feed that 1 to the shipped pre-commit hook and the commit is refused with `commit blocked by findings at severity >= warning` -- over a project with no findings and nothing to fix.

**Today this is LATENT, and the only thing holding it off is that `critic` is unbuilt.** The hook calls exactly two commands. `intent info` does not gate (it renders the pending state itself and exits 0). `intent critic` is unimplemented, so it exits 2 into the fail-open branch. **The moment `critic` is implemented the way every other command is implemented -- through `Facade::open` -- every unmigrated project with v3 on PATH loses the ability to commit.**

Found by vc, 2026-08-17, running the reverse sweep owed on issue 0044.

## Reproduction

All measurements at `3088c39c`, clean tree, `native/rust/target/debug/intent` rebuilt from that commit. The v3 binary is never placed on the machine's PATH: each `git commit` runs under `env PATH="<shimdir>:$PATH"` scoped to that single invocation.

### 1. The git gate's polarity, every code

A throwaway v2 project (`intent_version: 2.19.0`, `languages: ["shell"]`), the shipped `lib/templates/hooks/pre-commit.sh` installed at `.git/hooks/pre-commit`, one staged file, and a shim answering `intent critic` with a chosen code:

| `intent critic` exits | hook rc | commit               |
| --------------------- | ------- | -------------------- |
| 0                     | 0       | proceeds             |
| **1**                 | **1**   | **BLOCKED**          |
| 2                     | 0       | proceeds (fail-open) |
| 3                     | 0       | proceeds (fail-open) |

Verified by counting commits in the log, not by trusting the return code.

### 2. Claude Code's polarity, measured earlier on issue 0043

Five arms against Claude Code 2.1.233, a `settings.json` wiring `UserPromptSubmit` to one command:

| hook exits | prompt      |
| ---------- | ----------- |
| 0          | runs        |
| 1          | runs        |
| **2**      | **BLOCKED** |

### 3. The two tables, laid over each other

| producer code | git pre-commit | Claude Code `UserPromptSubmit` |
| ------------- | -------------- | ------------------------------ |
| 0             | proceeds       | proceeds                       |
| 1             | **BLOCKS**     | proceeds                       |
| 2             | proceeds       | **BLOCKS**                     |

**Every non-zero code blocks exactly one of the two.** This is the finding.

### 4. The migration refusal is a 1

Four events, one project, one binary, one run:

```
rc=1  st nosuchsub    :: error: unrecognized subcommand 'nosuchsub'
rc=1  st show         :: error: the following required arguments were not provided: <ID>
rc=1  st list         :: error: this project has not been migrated to Intent v3 -- it declares Intent 2.19.0, ...
rc=2  init            :: error: `init` is a known command that is not implemented yet
```

**The tool already owns a code for "I cannot do this" and does not use it for the case a migration actually hits.**

### 5. Today's baseline: the commit succeeds, for a reason nobody chose

Real binary, no shim, unmigrated project, shipped hook:

```
intent critic (shell) invocation error (exit 2); fail-open.
error: `critic` is a known command that is not implemented yet

commit rc=0   -- the commit lands
```

**This is AC-10.9 holding, end to end.** It holds because `critic` is unbuilt.

### 6. The same hook, with `critic` reaching the facade

The only thing simulated is _which_ command produced the code. The code, the message, the hook and the project are all the shipped ones -- the shim forwards `intent critic ...` to `intent st list`, a real implemented command that opens the facade:

```
error: this project has not been migrated to Intent v3 -- it declares Intent 2.19.0, and 1 steel thread carries v2 canon this binary cannot read (ST0001)
  remedy: run `intent upgrade` to migrate this project to Intent v3

intent critic gate: commit blocked by findings at severity >= warning.
  review the findings above, fix them, and re-commit.
  to bypass (use sparingly): git commit --no-verify

commit rc=1   -- the commit does NOT land
```

**Read the two halves of that output together.** The true remedy is printed, and then the gate overrides it with a false one: _review the findings above, fix them, and re-commit_. There are no findings. There is nothing in the staged code to fix. **A user who follows the instruction the gate gives them cannot ever succeed**, and the instruction they must follow instead is three lines above, framed as if it were a critic finding.

## Root Cause

**`Facade::open` gates first, and it gates everything** (`crates/intentsvcs/src/facade.rs:399, 405`):

```rust
fn readable(project: &Project) -> Result<(), FacadeError> {
  match project.migration() {
    Migration::Done => Ok(()),
    Migration::Pending(pending) => Err(FacadeError::Unmigrated(pending)),
  }
}

pub fn open(project: Project, ctx: FacadeContext) -> Result<Self, FacadeError> {
  Self::readable(&project)?;
  ...
```

The gate is correct and its placement is reasoned in its own doc comment: it sits at the facade rather than in `ingest` **precisely so that `doctor` and the WP-10 migrator can still read an unmigrated project**, because "a gate that stopped them would take away the two tools whose entire job is this state."

**That list has exactly the right shape and is one entry short.** It exempts commands whose _own job_ is the unmigrated state. `critic`'s job is not the unmigrated state -- but its _consumer_ fails closed on the code the gate produces, which is a second, independent reason to exempt a command, and the doc comment does not contemplate it.

`FacadeError::Unmigrated` then reaches `Failure::Error(_) => EXIT_ERROR` (`crates/intent-cli/src/spine.rs:137`). Measured, not inferred: `intent st list` in the fixture returns 1.

**Why no instrument caught it.** `tests/exit_codes.rs` carries `a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt` -- the end-to-end hook test 0038 produced. Its fixture is _migrated_, because that is the condition 0038 was about. **The unmigrated project is the case the migration passes through, and no test drives the hook in it.** The test is correct and its name states its scope honestly; the gap is that the neighbouring state was never given one.

## Impact

**Ordered by when it bites.**

1. **The latent commit outage.** Whoever implements `critic` will reach for `Facade::open`, because that is how every other command is written and it is the correct-looking thing to do. Nothing in the code, the tests or the specs will tell them that this one command's caller fails closed. **The defect will be introduced by someone doing the obvious right thing**, and it will be found by an operator who cannot commit.

2. **It arrives during the migration window specifically.** An unmigrated project is not an exotic state -- it is the state every project in the estate is in, right up until it is migrated. Installing v3 puts the binary on PATH before anything is migrated; that ordering is forced, because `intent upgrade` is what does the migrating.

3. **The message actively misdirects.** `review the findings above, fix them, and re-commit` over a project with no findings sends the operator to look at their own staged code. The correct remedy is on screen and is framed as the thing to be reviewed.

4. **`--no-verify` is the remedy an operator will actually find**, and it is printed by the gate itself. A whole estate learning to bypass the pre-commit gate during a migration window is a worse outcome than the outage, and it outlives it.

**Not claimed: that this is live today.** It is not. `critic` is unbuilt and the commit proceeds -- measured in section 5. **This is filed now precisely because it is cheap now**: an exemption written before `critic` exists costs one line and a comment, and the same exemption written after costs an outage, a diagnosis, and a fleet-wide hook re-install to a file that is not tracked (`.git/hooks/` reaches a clone only via `intent claude upgrade --apply`).

## THE CONSUMER REGISTER, ENUMERATED ONCE (vc, 2026-08-17)

0044's Proposed Fix 3 asks for the caller list to be written down, and notes that **four issues have each been diagnosed against whichever consumer happened to be in view**. Here is the list, swept rather than recalled: every place in this repository that invokes `intent` and does something with the result.

**11 call sites across 7 files, and FIVE distinct policies.**

| #   | consumer                                                 | invocation                              | policy                                                                           |
| --- | -------------------------------------------------------- | --------------------------------------- | -------------------------------------------------------------------------------- |
| 1   | `lib/templates/hooks/pre-commit.sh:207`                  | `intent critic <lang> --staged`         | **1 = BLOCK**, 0 = pass, everything else fails open                              |
| 2   | `.claude/settings.json` `UserPromptSubmit`               | `intent claude hook require-in-session` | **2 = BLOCK**, 0 and 1 pass                                                      |
| 3   | `.claude/settings.json` `SessionStart`                   | `intent claude hook session-context`    | advisory -- no code blocks                                                       |
| 4   | `bin/.devbin/lib/cmd/check:42`                           | `intent critic --languages`             | **any non-zero = `die`**                                                         |
| 5   | `bin/.devbin/cmd/build.d/release:372`                    | `intent doctor`                         | **any non-zero = `abort`**                                                       |
| 6   | `bin/.devbin/cmd/build.d/release:632`                    | `intent agents sync`                    | **any non-zero = `abort`**                                                       |
| 7   | `bin/.devbin/cmd/build.d/release:659`                    | `intent claude upgrade --apply`         | **any non-zero = `abort`**                                                       |
| 8   | `bin/.devbin/lib/cmd/docs:30`                            | `intent agents sync`                    | non-zero = warn + fail the command                                               |
| 9   | `bin/.devbin/lib/cmd/docs:80`                            | `intent treeindex <dir>`                | non-zero = warn + fail the command                                               |
| 10  | `lib/templates/hooks/pre-commit.sh:115`                  | `intent info`                           | captures the code, **prints it, never branches on it**; keys on the parsed VALUE |
| 11  | `lib/templates/.claude/scripts/post-tool-advisory.sh:73` | `intent critic <lang> --files`          | **reads nothing** -- `\|\| true` and `2>/dev/null`                               |

**The register makes the conflict concrete rather than abstract. Exit `1` currently means, simultaneously:**

- **"block this commit"** (1)
- **"let this prompt through"** (2)
- **"abort this release"** (5, 6, 7)
- **"nothing at all"** (11)

**One producer cannot satisfy all four, and no number can.** That is the same conclusion as the two-table cross-tab above, arrived at from the consumer side and with the population enumerated instead of sampled.

**Three things the sweep turned up that no previous enumeration had.**

- **The release script is a consumer and was on nobody's list.** Three call sites, all `abort`-on-non-zero, and two of them invoke commands v3 does not implement (`agents sync`, `claude upgrade --apply`). **`int build release` refuses outright with v3 on PATH** -- loudly and correctly, but it means the release path is coupled to the exit surface and nobody had said so.
- **`doctor` is consumed by an `abort` and returns `1` for a RESULT.** It exits 1 when it finds findings, prints no `error:` line at all, and reports an unmigrated project as one of its findings. Consumer 5 therefore aborts a release in an unmigrated project, which is correct -- **and it is correct by coincidence, because the same 1 would arrive from a usage error.**
- **Consumer 11's `|| true` was written for exactly today.** Its comment says so: _"`intent critic` lands in ST0035/WP05. If the subcommand isn't present yet, the `|| true` swallows failure."_ A correct forward-compat hedge at the time; under v3 it means the advisory is **permanently silent with no indication**, and `2>/dev/null` will keep hiding real errors after `critic` is built. **The hedge outlived the condition it hedged against, which is the supersession-not-propagated class again.**

**Consumers 4 and 10 already carry the reasoning in their own comments** -- both name a v3 binary shadowing v2 as the known trigger and cite 0036/0042 -- so two of the eleven have been thought about properly, by dc, and the other nine have not.

**This register belongs beside the constants in `spine.rs`** (0044's Proposed Fix 3, cc's). **A consumer whose policy is not written next to the codes it consumes is a consumer that will be diagnosed against last time's bug.**

## Proposed Fix

**The polarity conflict is structural and cannot be fixed by choosing a better number.** Two directions, and the choice belongs to hv.

1. **Exempt `critic` from the migration gate, and say why in the doc comment.** `critic` reads staged files and per-language rules; it does not need the thread canon the gate protects. Add it to `readable`'s exemption list on the second ground -- _its consumer fails closed on the refusal code_ -- so the reason is on the page for the next command that acquires a fail-closed caller. Cheapest, and it closes the measured path.

2. **Stop making the exit code carry the distinction.** `claude hook` already demonstrates the shape that works: it **delegates**, so the code a consumer sees is one deliberately chosen for that consumer rather than one inherited from a generic failure. Any command with a hook consumer wants the same treatment. Larger, and it is the real answer to 0044 as well.

**Whichever route, the canary is the missing one:** drive the shipped `pre-commit.sh` in an **unmigrated** project, with a command that reaches the facade, and assert the commit lands. That is one test next to the one 0038 already produced, and it is the instrument that would have caught this.

**Deliberately NOT proposed: moving the migration refusal to exit 2.** It would fix the git side and break the Claude Code side, where 2 is the block. That is issue 0043 rebuilt, and it is what the two tables in section 3 exist to prevent anyone concluding.

## Related

- 0044 -- the exit-1 overload this is a consequence of; the reverse sweep owed there is what found this
- 0038 -- the same gate, the same consumer, a different producer of 1; fixed, and its fix is what makes today's baseline green
- 0043 -- the mirror on the Claude Code side, where 2 is the blocking code; closed
- 0042 -- the other pre-commit path, which fails OPEN by a deliberate call recorded in its own resolution
- AC-10.9 / AT-10.9 -- the criterion that holds today, and holds because `critic` is unbuilt
- AC-10.4 -- hooks continuity, still scoped to `.claude/**` and not `.git/hooks` (0038 clause 3)
- `IN-AG-NO-SILENT-001` -- inverted here: not a failure reported as success, but a non-failure reported as one, with a remedy that cannot be followed

## Resolutions

{{TBC}}
