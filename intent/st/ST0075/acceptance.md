---
st_id: ST0075
title: The Intent.app Console: daemon logs and one-off verbs in one window, copied from Gtools
---

# ST0075: The Intent.app Console: daemon logs and one-off verbs in one window, copied from Gtools -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The verb: intent daemon logs, with --lines and --follow, its register row and tests (status: Done)

- AC-01.1 intent daemon logs prints a header naming both daemon logs, then the last lines of each (40 by default, --lines N), and exits 0 -- satisfied: yes (computed)
- AC-01.2 a daemon log that intentd has not written yet is named as absent in the header, never skipped in silence -- satisfied: yes (computed)
- AC-01.3 intent daemon logs --follow prints lines the daemon appends after it started, until it is terminated -- satisfied: yes (computed)
- AC-01.4 intent daemon logs --follow leaves no tail running once it ends, whether it is sent SIGTERM, SIGINT or SIGKILL or its stdin closes (issue 0281's ruling) -- satisfied: yes (computed)

### WP-02 -- The window: the Console copied from Gtools, the palette, Console on Cmd-L, Close and Clear Console (status: WIP)

- AC-02.1 Console (Cmd-L) opens a window tailing intent daemon logs --follow while it is visible, and its footer names the files being tailed -- satisfied: yes (computed)
- AC-02.2 Console lines are coloured by kind: error, caused by and intentd could-not lines as errors, warning and remedy lines as warnings, the markers as the accent -- satisfied: yes (computed)
- AC-02.3 the Console keeps the last lines up to its capacity and drops the oldest, so a command run while the window is closed is there when it opens -- satisfied: yes (computed)

### WP-03 -- The streaming items: Run Doctor and Rebuild Search Index into the Console; Start, Stop and Restart noted there (status: Not Started)

- AC-03.1 Run Doctor streams intent doctor into the Console between a marker naming the command and a marker carrying its exit and duration, and brings the Console forward, so a clean pass is visible -- satisfied: no (computed)
- AC-03.2 Rebuild Search Index streams intent index rebuild into the Console the same way -- satisfied: no (computed)
- AC-03.3 Start, Stop and Restart intentd write a marked block with their command and result into the Console -- satisfied: no (computed)
- AC-03.4 a second one-off while one is running is refused with an alert naming the running command, not queued -- satisfied: no (computed)

## Acceptance Tests

### WP-01 -- The verb: intent daemon logs, with --lines and --follow, its register row and tests (status: Done)

- AT-01.1 `native/rust/crates/intent-cli/tests/daemon_logs_prints_and_follows.rs` -- covers AC-01.1 -- status: green
- AT-01.2 `native/rust/crates/intent-cli/tests/daemon_logs_prints_and_follows.rs` -- covers AC-01.2 -- status: green
- AT-01.3 `native/rust/crates/intent-cli/tests/daemon_logs_prints_and_follows.rs` -- covers AC-01.3 -- status: green
- AT-01.4 `native/rust/crates/intent-cli/tests/daemon_logs_prints_and_follows.rs` -- covers AC-01.4 -- status: green

### WP-02 -- The window: the Console copied from Gtools, the palette, Console on Cmd-L, Close and Clear Console (status: WIP)

- AT-02.1 `native/macos/Intent/IntentTests/ConsoleTests.swift` -- covers AC-02.1 -- status: green -- red at 3e1578d03, one mutation per criterion's subject in one app-test run, reverted after and the worktree's tree proven unchanged: with the header's two logs never returned, testTheTailingHeaderNamesBothLogs, testAPathContainingAndSplitsAtTheSharedDirectory and testOnlyTheFirstLineIsReadAsTheHeader failed; green at 3e1578d03 as banked in refs/bank/ic/st0075-wp02 (76ec91175): app-test ran 49 tests with 0 failures; rebanked at 878804412 after Decision A landed, changing only two comments in Theme.swift, not re-run
- AT-02.2 `native/macos/Intent/IntentTests/ConsoleTests.swift` -- covers AC-02.2 -- status: green -- red at 3e1578d03, one mutation per criterion's subject in one app-test run, reverted after and the worktree's tree proven unchanged: with `warning:` not matched, testLinesAreClassifiedOverIntentdsOwnShapes failed; green at 3e1578d03 as banked in refs/bank/ic/st0075-wp02 (76ec91175): app-test ran 49 tests with 0 failures; rebanked at 878804412 after Decision A landed, changing only two comments in Theme.swift, not re-run
- AT-02.3 `native/macos/Intent/IntentTests/ConsoleTests.swift` -- covers AC-02.3 -- status: green -- red at 3e1578d03, one mutation per criterion's subject in one app-test run, reverted after and the worktree's tree proven unchanged: with the ring never dropping a line, testTheRingKeepsTheLastLinesAndSaysHowManyItDropped failed; green at 3e1578d03 as banked in refs/bank/ic/st0075-wp02 (76ec91175): app-test ran 49 tests with 0 failures; rebanked at 878804412 after Decision A landed, changing only two comments in Theme.swift, not re-run

### WP-03 -- The streaming items: Run Doctor and Rebuild Search Index into the Console; Start, Stop and Restart noted there (status: Not Started)

- AT-03.1 -- covers AC-03.1 -- status: to-write
- AT-03.2 -- covers AC-03.2 -- status: to-write
- AT-03.3 -- covers AC-03.3 -- status: to-write
- AT-03.4 -- covers AC-03.4 -- status: to-write

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
