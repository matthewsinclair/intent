---
st_id: ST0096
title: A forge merges every open merge request: no file that every record shares
---

# ST0096: A forge merges every open merge request: no file that every record shares -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) Highlander, as hv decision 67 asks: at the close, an independent sweep finds one owner of realisation, one writer of the ignore lines and one declared set of generated aggregate views, in every language, and every finding of the sweep of the existing code made at the start (its .intentfiles and aggregate-view sections) is resolved in the package that owns it or answered on the record. -- satisfied: no
- AC-00.2 (non-test) Highlander on every change, as hv decision 67 asks: every unit of this thread lands with a critic-<lang> review of its diff for each language the diff touches, each Highlander finding fixed or answered, read by vc against the diff before land. -- satisfied: no

### WP-01 -- Realisation lives in the record, and the manifest goes (status: Not Started)

- AC-01.1 Every verb in design.md's census table writes no file outside the thread or issue it changes: that record's canon file, its events and its realised views. -- satisfied: no (computed)
- AC-01.2 intent upgrade folds intent/.intentfiles into the records and removes it, byte-deterministically, so a project upgraded on two branches apart merges clean. -- satisfied: no (computed)
- AC-01.3 (non-test) The design records cc's choice between R1 and R2 with its reason before WP-01's code starts, with the disposition of ST0095 WP-05's .intentfiles set merge and of issue 0704. -- evidence: intent/st/ST0096/design.md: 'Realisation leaves the shared manifest' records cc's choice of R1 with its reason, and 'What this changes in ST0095' records that WP-05 drops its .intentfiles set merge and that issue 0704 is superseded; cc's calls of 2026-11-19, recorded by vc -- satisfied: yes

### WP-02 -- The two lists leave git (status: WIP)

- AC-02.1 A new project and an upgraded one keep intent/st/steel_threads.md, intent/todo.md and every thread's intent/st/<ST>/acceptance.md on disk and out of git, and the verbs and intent sync keep them current. -- satisfied: no (computed)
- AC-02.2 doctor names a project whose aggregate views (the two lists, or any thread's acceptance.md) are still tracked, with intent upgrade as the fix, as an advisory that is not counted. -- satisfied: no (computed)
- AC-02.3 doctor shows an ignored aggregate view that is missing, or that differs from what the store renders, with intent sync --to-disk as the fix, and counts neither, so a fresh clone's first commit passes the commit gate; a view still tracked is counted as today. -- satisfied: no (computed)

### WP-03 -- The commit gate and the pull-request check follow (status: Not Started)

- AC-03.1 The repository's commit gate and pull-request check pass on a project with its aggregate views (the two lists and every thread's acceptance.md) tracked and on one with them untracked, and read their paths from the one declared set. -- satisfied: no (computed)

### WP-04 -- The proof: five merge requests through a forge stand-in (status: Not Started)

- AC-04.1 Five merge requests opened from one main, in two clones and two worktrees, each creating and starting a thread and adding a work package and an issue, merge in turn by a bare repository's merge as a forge merges, with no conflict, and intent doctor is clean after each is pulled. -- satisfied: no (computed)
- AC-04.2 (non-test) Two merge requests opened from one main that change one thread, one adding a work package and the other a criterion and a test, merge in turn by a bare repository's merge as a forge merges, with no conflict, and intent doctor is clean after each is pulled. -- satisfied: no

### WP-05 -- Each work package, criterion and test has its own canon file (status: Not Started)

- AC-05.1 A thread's canon in the release's format is one file for the thread's own fields and one file per work package, criterion, test and attachment under .canon/st/<thread>/, each named by its record's stored id or label, and no file lists, counts or orders a thread's children: the thread's directory holds them, read in the waist's order. -- satisfied: no (computed)
- AC-05.2 Two branches from one base, one adding a work package to a thread and the other adding a criterion and a test to the same thread, change disjoint files, and git merges them with no conflict; intent doctor is clean on the merge. (dc's AC-04.2 proves the same through the forge stand-in; this is the unit's own two-branch test.) -- satisfied: no (computed)
- AC-05.3 intent upgrade moves an existing thread to that layout byte-for-byte the same on two clones upgraded apart, and changes no id and no value; no committed view is rendered from more than one record, so the cover renders only the thread's own fields (hv decision 74); the closed records' views still committed on main are removed by the upgrade under removed:; an attachment whose bytes on disk differ from canon is refused by name and never removed; and a canon case pair (two attachments whose paths differ only in case, issue 0722) is refused by name before the split, since a folding disk would put both in one canon file. -- satisfied: no (computed)

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- Realisation lives in the record, and the manifest goes (status: Not Started)

_(no tests in this group)_

### WP-02 -- The two lists leave git (status: WIP)

_(no tests in this group)_

### WP-03 -- The commit gate and the pull-request check follow (status: Not Started)

_(no tests in this group)_

### WP-04 -- The proof: five merge requests through a forge stand-in (status: Not Started)

_(no tests in this group)_

### WP-05 -- Each work package, criterion and test has its own canon file (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
