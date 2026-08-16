---
id: "0038"
title: v3 exits 1 for an unimplemented command, so a migrated project's pre-commit gate blocks every commit
date: 2026-08-16
reporter: matts
status: OPEN
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

{{TBC}}
