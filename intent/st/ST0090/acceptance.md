---
st_id: ST0090
title: The index rebuild says what happened, in the CLI's report and Intent.app's menu
---

# ST0090: The index rebuild says what happened, in the CLI's report and Intent.app's menu -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- One level-3 state on every face, and the index report names its reconcile time (status: Done)

- AC-01.1 One function in intentsvcs names a language's level-3 state from its stored run and the paths gone stale since: `current` only for a run that stored with nothing stale, `stale` for a stored run with stale paths, the extractor's move included, and the run's own `missing` or `failed` whatever is stale; the search envelope and the index report both ask it, and no face prints `Run::state` as the language's state. -- satisfied: yes (computed)
- AC-01.2 `intent index status`, `intent index rebuild` and `intent index resolve` name that state on each language's first line, with the stored line, the dropped line, the extractor line and every stale path below it as today, and a search answer's `index.resolution` names the same state for the same run. -- satisfied: yes (computed)
- AC-01.3 `intent index status --json` and `intent index rebuild --json` name the same state for each language as the text does, and the last run's own outcome stays readable as a fact of its own; a renamed field is named under the next version in CHANGELOG. -- satisfied: yes (computed)
- AC-01.4 `intent index status` and `intent index rebuild`, text and JSON, name when the index was last reconciled, read from the store's `reconciled_at` and never from a clock at render, and an index with no stamp names none. -- satisfied: yes (computed)

### WP-02 -- Intent.app's Rebuild Search Index row copies Gtools' Refresh Search row (status: WIP)

- AC-02.1 Intent.app's Rebuild Search Index item carries a second line, `reconciled <age> ago` in Gtools' age words, from the `reconciled_at` that `intent index status --json` names, and none when the CLI names none; its title and busy caption do not change. -- satisfied: no (computed)
- AC-02.2 A rebuild that exits non-zero, or cannot be run, raises an alert titled `Rebuild failed` carrying the CLI's own `error:` and `remedy:` lines, as Gtools' `Refresh failed` does. -- satisfied: no (computed)
- AC-02.3 Every rebuild is still recorded as one block in Intent.app.log, and a rebuild no longer opens the log by itself, while Run Doctor still opens it. -- satisfied: no (computed)
- AC-02.4 (non-test) hv sees the Rebuild Search Index row's second line after a rebuild, on the build that carries WP-01 and WP-02, and vc records which build that was. -- satisfied: no

## Acceptance Tests

### WP-01 -- One level-3 state on every face, and the index report names its reconcile time (status: Done)

- AT-01.1 `native/rust/crates/intent-cli/tests/the_index_report_names_level_three_as_search_does.rs` -- covers AC-01.1, AC-01.2, AC-01.3 -- status: green -- To write: vc minted the row when it raised ST0090 on 2026-10-31, and cc's WP-01 bank writes the arms its body names; red at the landing, ahead of green, as ST0087's rows were at 0b9c5082d, because green is reachable only from red: dc wrote the arms in its bank refs/bank/dc/st0090-wp01 to assert what the base d664d44fa did not print, the failure issue 0664 records, and no run saw them fail on the base; arms: a_stored_run_with_a_file_changed_since_reads_stale_on_every_face, a_stored_run_with_nothing_stale_reads_current_and_an_answer_names_nothing and a_run_that_did_not_store_keeps_its_own_word_whatever_is_stale; green in dc's judge run 01M696MPDJ812NRYHF3WRPBVPQ on 2026-10-31 at d664d44fa plus the bank (patch-id d8f76ab374d8): the format check, the build, the intentsvcs and intent-cli suites whole and workspace clippy at -D warnings, every leg exit 0; GREEN in vc's verdict of 14:56Z, with vc's critic leg clean; landed with this row
- AT-01.2 `native/rust/crates/intent-cli/tests/the_index_report_names_its_reconcile_time.rs` -- covers AC-01.4 -- status: green -- To write: vc minted the row when it raised ST0090 on 2026-10-31, and cc's WP-01 bank writes the arms its body names; red at the landing, ahead of green, as ST0087's rows were at 0b9c5082d, because green is reachable only from red: dc wrote the arms in its bank refs/bank/dc/st0090-wp01 to assert a reconcile stamp the base d664d44fa never printed, and no run saw them fail on the base; arms: an_index_no_reconcile_has_stamped_names_none and the_report_names_the_stamp_the_last_reconcile_wrote; green in dc's judge run 01M696MPDJ812NRYHF3WRPBVPQ on 2026-10-31 at d664d44fa plus the bank (patch-id d8f76ab374d8): the format check, the build, the intentsvcs and intent-cli suites whole and workspace clippy at -D warnings, every leg exit 0; GREEN in vc's verdict of 14:56Z, with vc's critic leg clean; landed with this row

### WP-02 -- Intent.app's Rebuild Search Index row copies Gtools' Refresh Search row (status: WIP)

- AT-02.1 `native/macos/Intent/IntentTests/RebuildItemTests.swift` -- covers AC-02.1, AC-02.2, AC-02.3 -- status: to-write -- To write: vc minted the row when it raised ST0090 on 2026-10-31, and ic's WP-02 bank writes the arms its body names

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
