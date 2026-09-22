---
st_id: ST0079
title: Add 'outs[tanding]' verb to show all outstanding items from a single verb
---

# ST0079: Add 'outs[tanding]' verb to show all outstanding items from a single verb -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 `intent outstanding` and its alias `intent outs` print one table of every outstanding steel thread, work package and issue, through the list verbs' shared output layer (`--format`, `--width`, `--markdown`): the kind (ST, WP or Issue) in the leftmost column, then ID, Status and Title, with threads first, then work packages, then issues. -- WITHDRAWN: Withdrawn for ST0079/02 on hv's TODO of 2026-09-22, verbatim: "TODO: In the verb 'intent outs' we need to ensure that any open WPs are shown merged in directly underneath their own ST. At the moment, we are just showing STs then WPs then Issues. The WPs are relevant as WPs for a particular ST and should be show underneath each ST to which they belong." Its order clause (threads first, then work packages, then issues) is what changes; AC-02.1 carries its text with the new order. (by hv)
- AC-00.2 The threads it lists are exactly those bare `intent st list` lists and the issues exactly those bare `intent issues` lists, in the same order; a work package is listed when it is WIP, under any thread, with its id as STxxxx/NN. -- WITHDRAWN: Withdrawn for ST0079/02 on hv's TODO of 2026-09-22, verbatim: "TODO: In the verb 'intent outs' we need to ensure that any open WPs are shown merged in directly underneath their own ST. At the moment, we are just showing STs then WPs then Issues. The WPs are relevant as WPs for a particular ST and should be show underneath each ST to which they belong." A thread bare `intent st list` does not list is now shown as its WIP work package's parent (hv, 2026-09-22), so "the threads it lists are exactly those bare `intent st list` lists" no longer holds; AC-02.2 carries its text with the parent row and where it sorts. (by hv)
- AC-00.3 `--show` takes a comma-separated list of st, wp, is, issue, issues and all, all being the default, and the table shows those kinds and no other; any other value is refused with a remedy naming the accepted values. -- satisfied: yes (computed)
- AC-00.4 Below the table one line counts the rows of each kind shown against how many of that kind exist, and with nothing outstanding that line is printed alone, so an empty result reads as none of N rather than as missing data. -- WITHDRAWN: Withdrawn for ST0079/02 on hv's TODO of 2026-09-22, verbatim: "TODO: In the verb 'intent outs' we need to ensure that any open WPs are shown merged in directly underneath their own ST. At the moment, we are just showing STs then WPs then Issues. The WPs are relevant as WPs for a particular ST and should be show underneath each ST to which they belong." A thread shown only as a work package's parent is counted apart from the outstanding threads; AC-02.3 carries its text with that clause. (by hv)
- AC-00.5 The same rows are served to agents as a read-only MCP tool. -- satisfied: yes (computed)

### WP-01 -- TUI Omnibox /outs[tanding]: the same table intent outs prints, inside the TUI (status: Done)

- AC-01.1 In the explorer, `/outstanding` and `/outs` each open a view of the rows `intent outs` prints, read from `Facade::outstanding` and laid out in the order it returns them, each row with its kind (ST, WP or Issue) leftmost, then ID, Status and Title. -- satisfied: yes (computed)
- AC-01.2 The view ends with the counts line `intent outs` prints, and with nothing outstanding it shows that line alone. -- satisfied: yes (computed)
- AC-01.3 Enter on a row opens that thread, work package or issue, and Backspace returns to the view. -- satisfied: yes (computed)
- AC-01.4 An argument after the command is refused on the info row, and nothing runs. -- satisfied: yes (computed)

### WP-02 -- outs shows each open WP directly underneath its own ST (status: WIP)

- AC-02.1 `intent outstanding` and its alias `intent outs` print one table of every outstanding steel thread, work package and issue, through the list verbs' shared output layer (`--format`, `--width`, `--markdown`): the kind (ST, WP or Issue) in the leftmost column, then ID, Status and Title, each thread followed directly by its WIP work packages in sequence order, and the issues after the last thread. -- satisfied: no (computed)
- AC-02.2 The outstanding threads it lists are exactly those bare `intent st list` lists and the issues exactly those bare `intent issues` lists, in the same order; a work package is listed when it is WIP, under any thread, with its id as STxxxx/NN, and a thread bare `intent st list` does not list is shown as that work package's parent row, with the thread's own status, placed among the threads where `intent st list --status all` places it. -- satisfied: no (computed)
- AC-02.3 Below the table one line counts the rows of each kind shown against how many of that kind exist, and a thread shown only as a work package's parent is counted apart from the outstanding threads; with nothing outstanding that line is printed alone, so an empty result reads as none of N rather than as missing data. -- satisfied: no (computed)

## Acceptance Tests

### ST-level

- AT-00.1 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-00.1 -- status: green -- red on the base: the installed pair 850918a73, whose compiled half equals base f4b54ab52, answers `intent outs` with 'unrecognized subcommand' at rc 1 (measured 2026-09-21 22:4xZ); then green at 1a0b5c5eb: vc judged the bank at patch-id 3e4d981bf, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-00.2 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-00.2 -- status: green -- red on the base: the installed pair 850918a73, whose compiled half equals base f4b54ab52, answers `intent outs` with 'unrecognized subcommand' at rc 1 (measured 2026-09-21 22:4xZ); then green at 1a0b5c5eb: vc judged the bank at patch-id 3e4d981bf, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-00.3 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-00.3 -- status: green -- red on the base: the installed pair 850918a73, whose compiled half equals base f4b54ab52, answers `intent outs` with 'unrecognized subcommand' at rc 1 (measured 2026-09-21 22:4xZ); then green at 1a0b5c5eb: vc judged the bank at patch-id 3e4d981bf, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-00.4 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-00.4 -- status: green -- red on the base: the installed pair 850918a73, whose compiled half equals base f4b54ab52, answers `intent outs` with 'unrecognized subcommand' at rc 1 (measured 2026-09-21 22:4xZ); then green at 1a0b5c5eb: vc judged the bank at patch-id 3e4d981bf, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-00.5 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-00.5 -- status: green -- red on the base: the installed pair 850918a73, whose compiled half equals base f4b54ab52, answers `intent outs` with 'unrecognized subcommand' at rc 1 (measured 2026-09-21 22:4xZ); then green at 1a0b5c5eb: vc judged the bank at patch-id 3e4d981bf, and the whole workspace suite ran rc 0 in a private worktree with this test among it

### WP-01 -- TUI Omnibox /outs[tanding]: the same table intent outs prints, inside the TUI (status: Done)

- AT-01.1 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-01.1 -- status: green -- red on the base: the AT hunk alone, applied to c3514e151 in a throwaway worktree, does not compile: E0599 no `View::Outstanding` at cli_end_to_end.rs:1855 and E0425 no `render::outstanding_view` at :1870 (measured 2026-09-22 between 07:34Z and 07:35Z); then green at 52cd8b55d: vc judged the bank at patch-id cbf0c5180 on base c3514e151, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-01.2 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-01.2 -- status: green -- red on the base: the AT hunk alone, applied to c3514e151 in a throwaway worktree, does not compile: E0425 no `render::outstanding_view` at cli_end_to_end.rs:1912 and :1922 (measured 2026-09-22 between 07:34Z and 07:35Z); then green at 52cd8b55d: vc judged the bank at patch-id cbf0c5180 on base c3514e151, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-01.3 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-01.3 -- status: green -- red on the base: the AT hunk alone, applied to c3514e151 in a throwaway worktree, does not compile: E0425 no `render::outstanding_view` at cli_end_to_end.rs:1943 and E0599 no `View::Outstanding` at :1980 (measured 2026-09-22 between 07:34Z and 07:35Z); then green at 52cd8b55d: vc judged the bank at patch-id cbf0c5180 on base c3514e151, and the whole workspace suite ran rc 0 in a private worktree with this test among it
- AT-01.4 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs` -- covers AC-01.4 -- status: green -- red on the base by its shared test crate: the AT hunk alone, applied to c3514e151 in a throwaway worktree, does not compile (six errors, in AT-01.1 to AT-01.3), so this test cannot run there (measured 2026-09-22 between 07:34Z and 07:35Z); then green at 52cd8b55d: vc judged the bank at patch-id cbf0c5180 on base c3514e151, and the whole workspace suite ran rc 0 in a private worktree with this test among it

### WP-02 -- outs shows each open WP directly underneath its own ST (status: WIP)

_(no tests in this group)_

---

_Generated by Intent v3.2.0 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
