---
st_id: ST0081
title: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release
---

# ST0081: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Linux release binaries built by GitHub Actions (status: WIP)

- AC-01.1 (non-test) A `workflow_dispatch` run, with no tag, builds `intent` and `intentd` for x86_64 and aarch64 Linux packaged with the support payload, and keeps each package as a workflow artifact. -- satisfied: no
- AC-01.2 (non-test) The same run executes the packaged `intent --version` inside a container of the oldest Linux distribution the workflow names as its floor, and that step fails the run when the binary does not start. -- satisfied: no

### WP-02 -- Linux assets on each tag's GitHub release, one writer per release object (status: Not Started)

- AC-02.1 (non-test) Every release tag's GitHub release carries the Linux assets beside the macOS assets, and `bin/devbin release check` names no second writer of the release object. -- satisfied: no

### WP-03 -- Intent's Linux install through Homebrew, in docs/install.md (status: WIP)

- AC-03.1 (non-test) Following the docs' Linux install takes a fresh WSL distro from a release download to an `intent --version` that names the release and an `intent doctor` with no finding in a new project, and every command on the page has been run. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The WSL walk-through moved to ST0082 with the WSL page on hv's ruling of 2026-10-14; the Linux install half is AC-03.3, through Homebrew.
- AC-03.2 (non-test) The docs name WSL as Intent's Windows route, tell readers to keep repos in the WSL filesystem rather than under `/mnt/c` and why, and tell them to run Claude Code inside WSL. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The WSL page moved to ST0082 on hv's ruling of 2026-10-14.
- AC-03.3 (non-test) On Linux, following `docs/install.md#linux` takes a machine with Homebrew and no Intent to an `intent --version` that names the release and an `intent doctor` with no finding in a new project, with no checkout of Intent's source, and every command on the page has been run. -- satisfied: no

### WP-04 -- First install on a real Windows machine (status: Cancelled)

- AC-04.1 (non-test) On a real Windows machine, Intent installed under WSL from a tagged release by following the docs reports its version, passes `intent doctor`, and creates and lists a thread in a project in the WSL filesystem. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The real Windows install moved to ST0082 with WP-04 on hv's ruling of 2026-10-14.

### WP-05 -- intent daemon start --at-login refuses on Linux before writing anything (status: Done)

- AC-05.1 On Linux, `intent daemon start --at-login` and `intent daemon stop --at-login` refuse before writing anything, naming login enrolment as macOS-only and `intent daemon start` without the flag as the way to run the daemon. -- satisfied: yes (computed)

### WP-06 -- The Homebrew formula installs Intent on Linux (status: WIP)

- AC-06.1 (non-test) On Linux, `brew install matthewsinclair/intent/intent` installs the release's Linux package for the machine's architecture with no checkout of Intent's source, and the installed `intent --version` names the release. -- satisfied: no

### WP-07 -- Intent on Ubuntu from its Linux package, as a user uses it (status: WIP)

- AC-07.1 (non-test) On Ubuntu with uutils coreutils and mawk, Intent installed from its Linux package with no checkout of Intent's source reports its version, bootstraps, initialises a project in which `intent doctor` reports no finding, creates and lists a thread, starts and stops the daemon, and fires its hooks in a Claude Code session. -- satisfied: no

## Acceptance Tests

### WP-01 -- Linux release binaries built by GitHub Actions (status: WIP)

_(no tests in this group)_

### WP-02 -- Linux assets on each tag's GitHub release, one writer per release object (status: Not Started)

_(no tests in this group)_

### WP-03 -- Intent's Linux install through Homebrew, in docs/install.md (status: WIP)

_(no tests in this group)_

### WP-04 -- First install on a real Windows machine (status: Cancelled)

_(no tests in this group)_

### WP-05 -- intent daemon start --at-login refuses on Linux before writing anything (status: Done)

- AT-05.1 `native/rust/crates/intent-cli/tests/at_login_is_refused_off_macos.rs` -- covers AC-05.1 -- status: green -- Its two arms run off macOS only and are ignored on macOS, where the flag is supported; they are proven on kovacs, hv's Ubuntu VM, and then on Linux CI. vc's ORDER 5 on kovacs (hv's Ubuntu 26.04.1 VM, aarch64), script refs/bank/cc/0081/kovacs/o5-at051.sh, quoted from kovacs-sysadm's log as vc relayed it. RED at base 8c51de484 with the test half alone (blob 20712d9d2c1f, fetched and matched): "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 802 filtered out" -- the start arm panicked at at_login_is_refused_off_macos.rs:53:3, "a LaunchAgents tree was written: error: `launchctl` could not be run", and the stop arm at :69:3, "stop --at-login did not refuse: ok: intentd is not enrolled at login". GREEN at 11017d7bd, where WP-05 landed: "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 802 filtered out"; o5 ended rc=0 at 11:19:12Z by kovacs's clock, its worktree removed and its home clean.

### WP-06 -- The Homebrew formula installs Intent on Linux (status: WIP)

_(no tests in this group)_

### WP-07 -- Intent on Ubuntu from its Linux package, as a user uses it (status: WIP)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
