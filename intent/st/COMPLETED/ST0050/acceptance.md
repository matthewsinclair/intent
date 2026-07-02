---
verblock: "02 Jul 2026:v0.1: matts - Initial version"
st_id: ST0050
title: "intent todo: a flat DOING/TODO/DONE view of steel threads and work packages -- acceptance contract"
---

# ST0050 intent todo: a flat DOING/TODO/DONE view of steel threads and work packages -- Acceptance

> Canonical acceptance contract for ST0050. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> Exemption (ST0048): the close-gate is fail-by-default -- a unit with an empty or missing contract is refused. A unit that is deliberately AC-free (eg a pure content / authorial task) declares `acceptance: exempt` in the frontmatter above; the gate then passes and announces the exemption. Omit it (the default) and the contract is enforced. Never inferred from emptiness; always declared.

## Acceptance Criteria

### ST-level

None -- WP-distributed.

### WP-01 -- Read path + output (minimal markdown + --json) (status: WIP)

- AC-01.1 `intent todo update` regenerates `intent/todo.md` as a nested GFM checklist bucketed DOING/TODO/DONE, projected from each unit's real `status:` (threads as `- [ ] STID: title`, work packages as indented `  - [ ] NN: title`).
- AC-01.2 checkbox glyphs map status: `WIP`->`[-]`, `Not Started`->`[ ]`, `Completed`/`Done`->`[x]`, `Cancelled`->`[~]`.
- AC-01.3 buckets are correct: DOING = `WIP` threads (+ WPs); TODO = `Not Started`; DONE = threads whose `completed:` is at or after the `## DONE:<T>` watermark (WP-06; the first-generation default `<T>` = start of today, so the zero-flush view is "completed today").
- AC-01.4 on-hold threads (`on-hold: TRUE` + `status: WIP`) render in DOING with an `(on-hold)` tag.
- AC-01.5 `todo.md` contains ONLY the three `## DOING` / `## TODO` / `## DONE:<T>` headings and their data (the `_(none)_` sentinel when a bucket is empty) -- NO title line, NO `_Generated…_` provenance line, NO `_Legend…_` line (hv minimal-output).
- AC-01.6 output is prettier-stable: the generator's output equals the post-prettier file byte-for-byte (no reflow churn on commit).
- AC-01.7 `intent todo` / `intent todo list` prints `todo.md` (generating it first if absent); `intent todo help` prints usage.
- AC-01.8 `intent todo --json` emits valid JSON: an object keyed by bucket (doing/todo/done), each a list of threads carrying `id` / `title` / `status`, each thread carrying its work packages (`id` / `title` / `status`). The JSON and markdown emitters share one enumeration of `intent/st/**` (Highlander -- no second traversal).

### WP-02 -- Mutation verbs (status: Not Started)

- AC-02.1 `intent todo done <ST[/NN]>` changes real status by wrapping `intent st done` / `intent wp done`, then regenerates `todo.md` -- it never hand-edits a checkbox or `status:`.
- AC-02.2 `intent todo done` INHERITS the ST0048 acceptance close-gate (D2): a unit with a BLOCKED contract is refused, the gate's message is surfaced verbatim, and status is left unchanged (no bypass).
- AC-02.3 `intent todo notdone <ST[/NN]>` reopens a completed unit to `WIP` (D1), then regenerates.
- AC-02.4 `intent todo toggle <ST[/NN]>` flips done/not-done from the unit's current status, then regenerates.
- AC-02.5 (non-test) the ST/WP specifier (`ST0011` / `11` / `ST0011/01` / `11/01`) is parsed via the shared `intent wp` specifier logic, not a reimplementation (Highlander). -- evidence: parse_wp_specifier extracted to bin/intent_helpers (Highlander); reused by intent_wp + intent_todo spec_info_file; 1/1 == ST1/1 == ST0001/01 proven -- satisfied: yes

### WP-03 -- CLI integration (status: Not Started)

- AC-03.1 `todo` is registered in `intent_help` and the top-level usage listing.
- AC-03.2 `intent todo <args>` dispatches end-to-end via `bin/intent`'s default `intent_<command>` fall-through (no dispatcher edit; the command runs from `PROJECT_ROOT`).

### WP-04 -- Tests (status: Not Started)

- AC-04.1 a reusable fixture-project harness under `tests/` proves, green: projection correctness (WIP/Not-Started/Completed STs + WPs -> asserted `todo.md`), minimal-output shape, prettier-stability, the DONE self-sweep, mutation round-trips, gate-inheritance, and `--json` structure.

### WP-05 -- Docs + release (status: Not Started)

- AC-05.1 (non-test) `bin/intent_todo` is registered in `intent/llm/MODULES.md`; README and `usage-rules.md` document `intent todo` (commands, projection model, mutation semantics, `--json`). -- evidence: MODULES.md `Todo view` row; usage-rules.md `### Todo view`; README `See what's in flight` -- satisfied: yes
- AC-05.2 (non-test) CHANGELOG carries a 2.14.0 `intent todo` entry; `impl.md` records the as-built; `tasks.md` reflects completion. -- evidence: CHANGELOG `## [2.14.0]` (ST0050 + ST0051); impl.md as-built; tasks.md WP checklist -- satisfied: yes

### WP-06 -- DONE lifecycle (flush / prune) + ISO completion timestamp (status: WIP)

- AC-06.1 `intent st done` stamps the frontmatter `completed:` as an ISO 8601 UTC timestamp (`YYYY-MM-DDThh:mm:ssZ`), not a bare `%Y%m%d` date. (The human `- **Completed**:` body bullet stays a `%Y-%m-%d` date; `steel_threads.md` renders the date part -- an ISO `completed:` on the fallback render path is truncated to its date.)
- AC-06.2 the DONE bucket is watermarked: the heading is `## DONE:<T>`, where `<T>` is the last-flush instant (ISO 8601 UTC). DONE lists threads whose `completed:` is at or after `<T>`; `update` preserves `<T>` (first generation defaults `<T>` to the start of today, UTC -- reproducing the "completed today" view as the zero-flush baseline).
- AC-06.3 DONE membership tolerates both timestamp forms: a legacy `completed: YYYYMMDD` and an ISO `completed:` are each compared correctly against `<T>` (legacy read as that day's 00:00:00Z).
- AC-06.4 `intent todo done --flush` advances `<T>` to now and empties the DONE view; a thread's real status is untouched (flush clears the view, not the record in `COMPLETED/`).
- AC-06.5 `intent todo done --prune` emits the pruned DONE items to stdout (for the caller to archive, eg `>> intent/done.md`; the advisory note goes to stderr), and then flushes.

## Acceptance Tests

### WP-01

- AT-01.1 tests/unit/intent_todo.bats::update_projects_buckets_from_status -- covers AC-01.1, AC-01.3 -- status: green
- AT-01.2 tests/unit/intent_todo.bats::checkbox_glyphs_map_each_status -- covers AC-01.2 -- status: green
- AT-01.3 tests/unit/intent_todo.bats::done_bucket_self_sweeps_to_today -- covers AC-01.3 -- status: green
- AT-01.4 tests/unit/intent_todo.bats::on_hold_thread_tagged_in_doing -- covers AC-01.4 -- status: green
- AT-01.5 tests/unit/intent_todo.bats::todo_md_has_only_headings_and_data -- covers AC-01.5 -- status: green
- AT-01.6 tests/unit/intent_todo.bats::output_is_prettier_stable -- covers AC-01.6 -- status: green
- AT-01.7 tests/unit/intent_todo.bats::list_prints_and_help_shows_usage -- covers AC-01.7 -- status: green
- AT-01.8 tests/unit/intent_todo.bats::json_emits_valid_structured_buckets -- covers AC-01.8 -- status: green
- Coverage: all WP-01 ACs covered.

### WP-02

- AT-02.1 tests/unit/intent_todo.bats::done_wraps_st_wp_and_regenerates -- covers AC-02.1 -- status: green
- AT-02.2 tests/unit/intent_todo.bats::done_inherits_close_gate_on_blocked -- covers AC-02.2 -- status: green
- AT-02.3 tests/unit/intent_todo.bats::notdone_reopens_to_wip -- covers AC-02.3 -- status: green
- AT-02.4 tests/unit/intent_todo.bats::toggle_flips_from_current_status -- covers AC-02.4 -- status: green
- Coverage: AC-02.1..02.4 covered; AC-02.5 is non-test (Highlander review).

### WP-03

- AT-03.1 tests/unit/intent_todo.bats::todo_registered_in_help_and_usage -- covers AC-03.1 -- status: green
- AT-03.2 tests/unit/intent_todo.bats::dispatches_via_default_fallthrough -- covers AC-03.2 -- status: green
- Coverage: AC-03.1 + AC-03.2 covered (guards -- the CLI integration is the existing auto-discovery + fall-through architecture).

### WP-04

- AT-04.1 tests/unit/intent_todo.bats (the file as a whole -- the harness proving the ACs above) -- covers AC-04.1 -- status: green
- Coverage: AC-04.1 is the green-suite rollup of the ATs above.

### WP-05

- Coverage: AC-05.1 + AC-05.2 are non-test (MODULES.md + docs + CHANGELOG evidence).

### WP-06

- AT-06.1 tests/unit/intent_todo.bats::st_done_stamps_iso_completed -- covers AC-06.1 -- status: green
- AT-06.2 tests/unit/intent_todo.bats::done_bucket_watermarked_and_membership -- covers AC-06.2, AC-06.3 -- status: green
- AT-06.3 tests/unit/intent_todo.bats::flush_advances_watermark_empties_done -- covers AC-06.4 -- status: green
- AT-06.4 tests/unit/intent_todo.bats::prune_emits_then_flushes -- covers AC-06.5 -- status: green
- Coverage: all WP-06 ACs covered.
