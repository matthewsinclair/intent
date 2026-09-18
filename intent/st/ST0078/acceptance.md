---
st_id: ST0078
title: Using Intent on a multi-person project with a Git workflow including PRs
---

# ST0078: Using Intent on a multi-person project with a Git workflow including PRs -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- P1: the event log travels -- one committed file per event under intent/.canon/events/YYYY/MM/DD, additive ingest, principal is the author (reverses D53) (status: WIP)

- AC-01.1 Every PROJECT act is written as its own event file at intent/.canon/events/<YYYY>/<MM>/<DD>/<ulid>.json in the same write set as the canon and views of the act, with principal the author (git's user.name and user.email, else the project's config author, else local), so an act on one clone is readable with intent events on another after a pull, with its author; machine-scoped acts (heartbeats, ingests, the destructive restore, index rebuilds) stay in the store only, because they describe one machine and are false on every other clone (hv, 2026-09-18). -- satisfied: yes (computed)
- AC-01.2 Ingest of committed event files is additive: a file whose id the store holds is skipped, one it lacks is inserted, a file is never rewritten, and no event_log row is deleted because its file is absent. -- satisfied: yes (computed)
- AC-01.3 intent doctor reports an event file that does not parse or whose id disagrees with its name, and reports a committed event the store lacks as store-stale; nothing rebuilds state from events and doctor never reconciles canon against them. -- satisfied: yes (computed)
- AC-01.4 intent upgrade removes the intent/events.jsonl ignore line and adds nothing, and writes, once and idempotently, an event file for every project event the store already holds and the tree lacks, so a project's history from before this change travels too; intent export keeps producing the single-file form on demand. -- satisfied: no (computed)

### WP-02 -- P2: renumber verbs -- intent st renumber and intent issues renumber repair an id two clones both minted (status: WIP)

- AC-02.1 intent st renumber <old> <new> refuses when <new> exists and otherwise rewrites the canon file and its name, the realised directory and views, the .intentfiles rows, related references in other threads, whiteboard claims and the attachment paths the store records, emits its own event, and prints the prose references it found with the index and did not rewrite. -- satisfied: yes (computed)
- AC-02.2 intent issues renumber <old> <new> does the same for an issue: refuses a taken id, rewrites the canon file, its name and its view, emits its own event, and prints the prose references it did not rewrite. -- satisfied: yes (computed)

### WP-03 -- P3: the store after a pull -- store-stale shown on a default doctor run, a CLI door for the non-destructive ingest, post-merge/post-checkout/post-rewrite hooks wired by claude upgrade --apply (status: Done)

- AC-03.1 After a git pull, checkout or rewrite with no daemon running, the next verb answers from the merged canon: intent st show of a thread the pull brought answers, because the post-merge, post-checkout and post-rewrite hooks wired by intent claude upgrade --apply ran intent sync --apply with no terminal, printed one line when they took anything or when the pass refused, and exited 0 either way. -- satisfied: yes (computed)
- AC-03.2 A store that lags the committed canon is reported on a default intent doctor run as store-stale, shown and not counted, with the exit code untouched. -- satisfied: yes (computed)
- AC-03.3 Bare intent sync prints the plan for this clone and writes nothing, and intent sync --apply applies it; under P3 the plan has one step, the daemon's non-destructive ingest rule run unchanged from the command line, the same engine and no second implementation: it takes the disk only where it differs from what the store recorded writing, a recorded file the pull removed included, never deletes a row whose file was never written, and is safe beside a running daemon and a peer's write under the hold-unless-moved lock; --apply with --to-disk or --to-store is refused. -- satisfied: yes (computed)

### WP-04 -- P4: working in a team -- the docs/concepts page written from driven commands, the-store.md corrections, a CI doctor job on the merge result (status: WIP)

- AC-04.1 (non-test) docs/concepts/working-in-a-team.md exists, is written from driven commands, and every command on it has been driven, with the drive script and its log attached to ST0078 under intent/st/ST0078/ and named from the page; the-store.md says that after a pull the hook runs intent sync --apply and that --to-store is the restore. -- satisfied: no
- AC-04.2 (non-test) A CI job builds intent and runs intent doctor on the merge result, so a merge made on the forge is judged by a gate. -- satisfied: no

### WP-05 -- P5: one command after a pull -- bare intent sync prints the plan for this clone, --apply applies it, --to-disk and --to-store keep their meanings; quiet, reversible and non-reversible steps, --yes for the reversible asks, the hooks run the quiet subset (status: Done)

- AC-05.1 intent sync reads the store, the tree and git's status and prints the plan it would apply as ordered steps, each with its recoverability, under a digest, and writes nothing; intent sync --apply applies it; --plan <digest> refuses when the tree has moved since the plan was shown. -- satisfied: yes (computed)
- AC-05.2 The plan handles a pull's aftermath in order: a branch behind its upstream is reported and nothing is run; a store that lags canon is ingested; an unmerged generated view is regenerated from the merged canon and staged; a canon add/add is repaired by renumbering the local id to the next free one; a canon content conflict asks for a side; stale views are regenerated; a stale index is refreshed (the incremental reconcile search runs, never the full rebuild); doctor runs last and its verdict is the exit code. -- satisfied: yes (computed)
- AC-05.3 A quiet step never asks; a reversible step asks y/N and --yes answers it; a non-reversible step always asks a person and no flag or environment variable answers for it; without a terminal and without --yes, --apply runs the quiet steps, skips every ask and prints one line naming what is left, which is what the hooks run. -- satisfied: yes (computed)
- AC-05.4 intent sync reads git's status and its unmerged index, stages only the files it regenerated to resolve a conflict it was asked to resolve, and never runs git pull, git commit or git push. -- satisfied: yes (computed)

## Acceptance Tests

### WP-01 -- P1: the event log travels -- one committed file per event under intent/.canon/events/YYYY/MM/DD, additive ingest, principal is the author (reverses D53) (status: WIP)

- AT-01.1 `native/rust/crates/intent-cli/tests/the_event_log_travels.rs` -- covers AC-01.1 -- status: green -- an_act_on_one_clone_is_read_on_another_with_its_author, the_author_is_git_then_the_config_then_local and a_heartbeat_stays_on_the_machine_that_beat; the machine-scoped roster is held by op_roster_and_the_live_log's machine_scoped_ops_are_rostered_and_stay_on_the_machine; green in P1's whole judging run on 06b0b0c38 over e447f15cf (cc, 2026-09-18)
- AT-01.2 `native/rust/crates/intent-cli/tests/the_event_log_travels.rs` -- covers AC-01.2 -- status: green -- ingest_takes_what_is_missing_and_changes_nothing_else and the_sync_plan_names_waiting_event_files_and_apply_takes_them; green in P1's whole judging run on 06b0b0c38 over e447f15cf (cc, 2026-09-18)
- AT-01.3 `native/rust/crates/intent-cli/tests/the_event_log_travels.rs` -- covers AC-01.3 -- status: green -- doctor_reports_bad_event_files_and_events_the_store_lacks; green in P1's whole judging run on 06b0b0c38 over e447f15cf (cc, 2026-09-18)

### WP-02 -- P2: renumber verbs -- intent st renumber and intent issues renumber repair an id two clones both minted (status: WIP)

- AT-02.1 `native/rust/crates/intentsvcs/tests/renumber_moves_an_id_and_what_names_it.rs` -- covers AC-02.1, AC-02.2 -- status: green -- judged green by ic's P2 run of 2026-09-18 on 930a2aed6 (scratchpad judge-p2c: intentsvcs 1798/0, intent-cli 1018/0, bats 678/0, doc, both clippy forms and fmt clean); re-judged green after vc's finding (the remedy and the CLI arm name sync --apply) by the re-run judge-p2d: intentsvcs 1798/0, intent-cli 1018/0, bats 678/0, doc, both clippy forms and fmt clean
- AT-02.2 `native/rust/crates/intent-cli/tests/two_clones_that_mint_one_id_are_repaired_by_renumber.rs` -- covers AC-02.1, AC-02.2 -- status: green -- judged green by ic's P2 run of 2026-09-18 on 930a2aed6 (scratchpad judge-p2c: intentsvcs 1798/0, intent-cli 1018/0, bats 678/0, doc, both clippy forms and fmt clean); re-judged green after vc's finding (the remedy and the CLI arm name sync --apply) by the re-run judge-p2d: intentsvcs 1798/0, intent-cli 1018/0, bats 678/0, doc, both clippy forms and fmt clean

### WP-03 -- P3: the store after a pull -- store-stale shown on a default doctor run, a CLI door for the non-destructive ingest, post-merge/post-checkout/post-rewrite hooks wired by claude upgrade --apply (status: Done)

- AT-03.1 `native/rust/crates/intent-cli/tests/a_pull_is_reflected_by_the_next_verb.rs` -- covers AC-03.1 -- status: green -- written; the judging run follows -- judged green by dc's P3 runs of 2026-09-18 (scratchpad p3-logs and p3-logs-2), landed patch-id 2c4e35fed
- AT-03.2 `native/rust/crates/intent-cli/tests/a_stale_store_shows_on_a_default_doctor_run.rs` -- covers AC-03.2 -- status: green -- judged green by dc's P3 runs of 2026-09-18 (scratchpad p3-logs and p3-logs-2), landed patch-id 2c4e35fed
- AT-03.3 `native/rust/crates/intentsvcs/tests/sync_ingest_takes_only_what_the_store_did_not_write.rs` -- covers AC-03.3 -- status: green -- judged green by dc's P3 runs of 2026-09-18 (scratchpad p3-logs and p3-logs-2), landed patch-id 2c4e35fed
- AT-03.4 `native/rust/crates/intent-cli/tests/sync_plan_and_apply_from_the_command_line.rs` -- covers AC-03.3 -- status: green -- judged green by dc's P3 runs of 2026-09-18 (scratchpad p3-logs and p3-logs-2), landed patch-id 2c4e35fed

### WP-04 -- P4: working in a team -- the docs/concepts page written from driven commands, the-store.md corrections, a CI doctor job on the merge result (status: WIP)

_(no tests in this group)_

### WP-05 -- P5: one command after a pull -- bare intent sync prints the plan for this clone, --apply applies it, --to-disk and --to-store keep their meanings; quiet, reversible and non-reversible steps, --yes for the reversible asks, the hooks run the quiet subset (status: Done)

- AT-05.1 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs` -- covers AC-05.1 -- status: green -- judged green by dc's P5 runs of 2026-09-18: the whole run at patch-id 20e99be51 (scratchpad p5-logs) and the intentsvcs re-run at d40af6929 (p5-logs-2), landed patch-id d40af6929
- AT-05.2 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs` -- covers AC-05.2 -- status: green -- judged green by dc's P5 runs of 2026-09-18: the whole run at patch-id 20e99be51 (scratchpad p5-logs) and the intentsvcs re-run at d40af6929 (p5-logs-2), landed patch-id d40af6929
- AT-05.3 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs` -- covers AC-05.3 -- status: green -- judged green by dc's P5 runs of 2026-09-18: the whole run at patch-id 20e99be51 (scratchpad p5-logs) and the intentsvcs re-run at d40af6929 (p5-logs-2), landed patch-id d40af6929
- AT-05.4 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs` -- covers AC-05.4 -- status: green -- judged green by dc's P5 runs of 2026-09-18: the whole run at patch-id 20e99be51 (scratchpad p5-logs) and the intentsvcs re-run at d40af6929 (p5-logs-2), landed patch-id d40af6929

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
