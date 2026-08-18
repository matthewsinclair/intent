---
st_id: ST0050
title: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages
---

# ST0050: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages -- Acceptance

> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### Group 01

- AC-01.1 `intent todo update` regenerates `intent/todo.md` as a nested GFM checklist bucketed DOING/TODO/DONE, projected from each unit's real `status:` (threads as `- [ ] STID: title`, work packages as indented `  - [ ] NN: title`). -- satisfied: yes (computed)
- AC-01.2 checkbox glyphs map status: `WIP`->`[-]`, `Not Started`->`[ ]`, `Completed`/`Done`->`[x]`, `Cancelled`->`[~]`. -- satisfied: yes (computed)
- AC-01.3 buckets are correct: DOING = `WIP` threads (+ WPs); TODO = `Not Started`; DONE = threads whose `completed:` is at or after the `## DONE:<T>` watermark (WP-06; the first-generation default `<T>` = start of today, so the zero-flush view is "completed today"). -- satisfied: yes (computed)
- AC-01.4 on-hold threads (`on-hold: TRUE` + `status: WIP`) render in DOING with an `(on-hold)` tag. -- satisfied: yes (computed)
- AC-01.5 `todo.md` contains ONLY the three `## DOING` / `## TODO` / `## DONE:<T>` headings and their data (the `_(none)_` sentinel when a bucket is empty) -- NO title line, NO `_Generated…_` provenance line, NO `_Legend…_` line (hv minimal-output). -- satisfied: yes (computed)
- AC-01.6 output is prettier-stable: the generator's output equals the post-prettier file byte-for-byte (no reflow churn on commit). -- satisfied: yes (computed)
- AC-01.7 `intent todo` / `intent todo list` prints `todo.md` (generating it first if absent); `intent todo help` prints usage. -- satisfied: yes (computed)
- AC-01.8 `intent todo --json` emits valid JSON: an object keyed by bucket (doing/todo/done), each a list of threads carrying `id` / `title` / `status`, each thread carrying its work packages (`id` / `title` / `status`). The JSON and markdown emitters share one enumeration of `intent/st/**` (Highlander -- no second traversal). -- satisfied: yes (computed)

### Group 02

- AC-02.1 `intent todo done <ST[/NN]>` changes real status by wrapping `intent st done` / `intent wp done`, then regenerates `todo.md` -- it never hand-edits a checkbox or `status:`. -- satisfied: yes (computed)
- AC-02.2 `intent todo done` INHERITS the ST0048 acceptance close-gate (D2): a unit with a BLOCKED contract is refused, the gate's message is surfaced verbatim, and status is left unchanged (no bypass). -- satisfied: yes (computed)
- AC-02.3 `intent todo notdone <ST[/NN]>` reopens a completed unit to `WIP` (D1), then regenerates. -- satisfied: yes (computed)
- AC-02.4 `intent todo toggle <ST[/NN]>` flips done/not-done from the unit's current status, then regenerates. -- satisfied: yes (computed)
- AC-02.5 (non-test) the ST/WP specifier (`ST0011` / `11` / `ST0011/01` / `11/01`) is parsed via the shared `intent wp` specifier logic, not a reimplementation (Highlander). -- evidence: parse_wp_specifier extracted to bin/intent_helpers (Highlander); reused by intent_wp + intent_todo spec_info_file; 1/1 == ST1/1 == ST0001/01 proven -- satisfied: yes

### Group 03

- AC-03.1 `todo` is registered in `intent_help` and the top-level usage listing. -- satisfied: yes (computed)
- AC-03.2 `intent todo <args>` dispatches end-to-end via `bin/intent`'s default `intent_<command>` fall-through (no dispatcher edit; the command runs from `PROJECT_ROOT`). -- satisfied: yes (computed)

### Group 04

- AC-04.1 a reusable fixture-project harness under `tests/` proves, green: projection correctness (WIP/Not-Started/Completed STs + WPs -> asserted `todo.md`), minimal-output shape, prettier-stability, the DONE self-sweep, mutation round-trips, gate-inheritance, and `--json` structure. -- satisfied: yes (computed)

### Group 05

- AC-05.1 (non-test) `bin/intent_todo` is registered in `intent/llm/MODULES.md`; README and `usage-rules.md` document `intent todo` (commands, projection model, mutation semantics, `--json`). -- evidence: MODULES.md `Todo view` row; usage-rules.md `### Todo view`; README `See what's in flight` -- satisfied: yes
- AC-05.2 (non-test) CHANGELOG carries a 2.14.0 `intent todo` entry; `impl.md` records the as-built; `tasks.md` reflects completion. -- evidence: CHANGELOG `## [2.14.0]` (ST0050 + ST0051); impl.md as-built; tasks.md WP checklist -- satisfied: yes

### Group 06

- AC-06.1 `intent st done` stamps the frontmatter `completed:` as an ISO 8601 UTC timestamp (`YYYY-MM-DDThh:mm:ssZ`), not a bare `%Y%m%d` date. (The human `- **Completed**:` body bullet stays a `%Y-%m-%d` date; `steel_threads.md` renders the date part -- an ISO `completed:` on the fallback render path is truncated to its date.) -- satisfied: yes (computed)
- AC-06.2 the DONE bucket is watermarked: the heading is `## DONE:<T>`, where `<T>` is the last-flush instant (ISO 8601 UTC). DONE lists threads whose `completed:` is at or after `<T>`; `update` preserves `<T>` (first generation defaults `<T>` to the start of today, UTC -- reproducing the "completed today" view as the zero-flush baseline). -- satisfied: yes (computed)
- AC-06.3 DONE membership tolerates both timestamp forms: a legacy `completed: YYYYMMDD` and an ISO `completed:` are each compared correctly against `<T>` (legacy read as that day's 00:00:00Z). -- satisfied: yes (computed)
- AC-06.4 `intent todo done --flush` advances `<T>` to now and empties the DONE view; a thread's real status is untouched (flush clears the view, not the record in `COMPLETED/`). -- satisfied: yes (computed)
- AC-06.5 `intent todo done --prune` emits the pruned DONE items to stdout (for the caller to archive, eg `>> intent/done.md`; the advisory note goes to stderr), and then flushes. -- satisfied: yes (computed)

## Acceptance Tests

### Group 01

- AT-01.1 `tests/unit/intent_todo.bats` -- covers AC-01.1, AC-01.3 -- status: green -- test: update_projects_buckets_from_status
- AT-01.2 `tests/unit/intent_todo.bats` -- covers AC-01.2 -- status: green -- test: checkbox_glyphs_map_each_status
- AT-01.3 `tests/unit/intent_todo.bats` -- covers AC-01.3 -- status: green -- test: done_bucket_self_sweeps_to_today
- AT-01.4 `tests/unit/intent_todo.bats` -- covers AC-01.4 -- status: green -- test: on_hold_thread_tagged_in_doing
- AT-01.5 `tests/unit/intent_todo.bats` -- covers AC-01.5 -- status: green -- test: todo_md_has_only_headings_and_data
- AT-01.6 `tests/unit/intent_todo.bats` -- covers AC-01.6 -- status: green -- test: output_is_prettier_stable
- AT-01.7 `tests/unit/intent_todo.bats` -- covers AC-01.7 -- status: green -- test: list_prints_and_help_shows_usage
- AT-01.8 `tests/unit/intent_todo.bats` -- covers AC-01.8 -- status: green -- test: json_emits_valid_structured_buckets

### Group 02

- AT-02.1 `tests/unit/intent_todo.bats` -- covers AC-02.1 -- status: green -- test: done_wraps_st_wp_and_regenerates
- AT-02.2 `tests/unit/intent_todo.bats` -- covers AC-02.2 -- status: green -- test: done_inherits_close_gate_on_blocked
- AT-02.3 `tests/unit/intent_todo.bats` -- covers AC-02.3 -- status: green -- test: notdone_reopens_to_wip
- AT-02.4 `tests/unit/intent_todo.bats` -- covers AC-02.4 -- status: green -- test: toggle_flips_from_current_status

### Group 03

- AT-03.1 `tests/unit/intent_todo.bats` -- covers AC-03.1 -- status: green -- test: todo_registered_in_help_and_usage
- AT-03.2 `tests/unit/intent_todo.bats` -- covers AC-03.2 -- status: green -- test: dispatches_via_default_fallthrough

### Group 04

- AT-04.1 `tests/unit/intent_todo.bats` -- covers AC-04.1 -- status: green -- the file as a whole: the harness proving the ACs above

### Group 05

_(no tests in this group)_

### Group 06

- AT-06.1 `tests/unit/intent_todo.bats` -- covers AC-06.1 -- status: green -- test: st_done_stamps_iso_completed
- AT-06.2 `tests/unit/intent_todo.bats` -- covers AC-06.2, AC-06.3 -- status: green -- test: done_bucket_watermarked_and_membership
- AT-06.3 `tests/unit/intent_todo.bats` -- covers AC-06.4 -- status: green -- test: flush_advances_watermark_empties_done
- AT-06.4 `tests/unit/intent_todo.bats` -- covers AC-06.5 -- status: green -- test: prune_emits_then_flushes

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
