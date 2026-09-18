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

### WP-01 -- P1: the event log travels -- one committed file per event under intent/.canon/events/YYYY/MM/DD, additive ingest, principal is the author (reverses D53) (status: Not Started)

- AC-01.1 Every event is written as its own file at intent/.canon/events/<YYYY>/<MM>/<DD>/<ulid>.json in the same write set as the canon and views of the act, with principal the author (the project's config author, else git's user.name and user.email, else local), so an act on one clone is readable with intent events on another after a pull, with its author. -- satisfied: no (computed)
- AC-01.2 Ingest of committed event files is additive: a file whose id the store holds is skipped, one it lacks is inserted, a file is never rewritten, and no event_log row is deleted because its file is absent. -- satisfied: no (computed)
- AC-01.3 intent doctor reports an event file that does not parse or whose id disagrees with its name, and reports a committed event the store lacks as store-stale; nothing rebuilds state from events and doctor never reconciles canon against them. -- satisfied: no (computed)
- AC-01.4 intent upgrade removes the intent/events.jsonl ignore line and adds nothing, and intent export keeps producing the single-file form on demand. -- satisfied: no (computed)

### WP-02 -- P2: renumber verbs -- intent st renumber and intent issues renumber repair an id two clones both minted (status: Not Started)

- AC-02.1 intent st renumber <old> <new> refuses when <new> exists and otherwise rewrites the canon file and its name, the realised directory and views, the .intentfiles rows, related references in other threads, whiteboard claims and the attachment paths the store records, emits its own event, and prints the prose references it found with the index and did not rewrite. -- satisfied: no (computed)
- AC-02.2 intent issues renumber <old> <new> does the same for an issue: refuses a taken id, rewrites the canon file, its name and its view, emits its own event, and prints the prose references it did not rewrite. -- satisfied: no (computed)

### WP-03 -- P3: the store after a pull -- store-stale shown on a default doctor run, a CLI door for the non-destructive ingest, post-merge/post-checkout/post-rewrite hooks wired by claude upgrade --apply (status: Not Started)

- AC-03.1 After a git pull, checkout or rewrite with no daemon running, the next verb answers from the merged canon: intent st show of a thread the pull brought answers, because the post-merge, post-checkout and post-rewrite hooks wired by intent claude upgrade --apply ran intent sync --ingest, printed one line when they took anything, and exited 0 either way. -- satisfied: no (computed)
- AC-03.2 A store that lags the committed canon is reported on a default intent doctor run as store-stale, shown and not counted, with the exit code untouched. -- satisfied: no (computed)
- AC-03.3 intent sync --ingest runs the daemon's non-destructive rule from the command line: it takes the disk only where it differs from what the store recorded writing, never deletes a store row because its file is absent, and is safe beside a running daemon and a peer's write under the hold-unless-moved lock. -- satisfied: no (computed)

### WP-04 -- P4: working in a team -- the docs/concepts page written from driven commands, the-store.md corrections, a CI doctor job on the merge result (status: Not Started)

- AC-04.1 (non-test) docs/concepts/working-in-a-team.md exists, is written from driven commands, and every command on it has been driven, with the drive script and log banked beside the page; the-store.md says that after a pull the hook runs sync --ingest and that --to-store is the restore. -- satisfied: no
- AC-04.2 (non-test) A CI job builds intent and runs intent doctor on the merge result, so a merge made on the forge is judged by a gate. -- satisfied: no

## Acceptance Tests

### WP-01 -- P1: the event log travels -- one committed file per event under intent/.canon/events/YYYY/MM/DD, additive ingest, principal is the author (reverses D53) (status: Not Started)

_(no tests in this group)_

### WP-02 -- P2: renumber verbs -- intent st renumber and intent issues renumber repair an id two clones both minted (status: Not Started)

_(no tests in this group)_

### WP-03 -- P3: the store after a pull -- store-stale shown on a default doctor run, a CLI door for the non-destructive ingest, post-merge/post-checkout/post-rewrite hooks wired by claude upgrade --apply (status: Not Started)

_(no tests in this group)_

### WP-04 -- P4: working in a team -- the docs/concepts page written from driven commands, the-store.md corrections, a CI doctor job on the merge result (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
