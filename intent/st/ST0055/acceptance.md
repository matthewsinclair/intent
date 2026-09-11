---
st_id: ST0055
title: Add `intent issues` command
---

# ST0055: Add `intent issues` command -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Foundation & format: dir-per-issue canon, Intent-owned template, scaffold, MODULES row, dispatch skeleton (status: Done)

- AC-01.1 (non-test) `bin/intent_issues` exists, is executable, and `intent issues` dispatches to it as a project command (fails cleanly outside an Intent project). -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-01.2 (non-test) The issue template lives at `lib/templates/issues/_ISSUE.md` (Intent-owned single source; no inline heredoc in `bin/intent_issues`). -- evidence: lib/templates/issues/_ISSUE.md exists; intent_issues stamps via sed, no heredoc; critic-shell no-heredoc clean -- satisfied: yes
- AC-01.3 (non-test) `MODULES.md` carries a row for `bin/intent_issues`, added before the module ships. -- evidence: MODULES.md 'Issue tracker' row for bin/intent_issues -- satisfied: yes
- AC-01.4 (non-test) `intent issues help` and an unknown subcommand both print usage listing the five verbs; exit code is 0 for `help`, non-zero for the unknown verb. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes

### WP-02 -- Create & list: add (dir-per-issue, id alloc, stamp), list --kind open/closed/all (status: Done)

- AC-02.1 (non-test) `intent issues add "Title"` allocates the next zero-padded 4-digit id (max+1 across OPEN+CLOSED), writes `intent/issues/OPEN/NNNN/NNNN-slug.md` with stamped frontmatter (id/title/date/reporter/status=OPEN/severity), and prints `NNNN:Title`. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.2 (non-test) A second `add` allocates the next sequential id with no collision (gaps tolerated, ids not reused below max). -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.3 (non-test) `intent issues` and `intent issues list` (no args) list OPEN issues by default. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.4 (non-test) `--kind open|closed|all` filters the listing; an empty bucket prints a clean empty state, not an error; an invalid `--kind` fails via `error()`. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.5 (non-test) The first `add` lazily scaffolds `intent/issues/{OPEN,CLOSED}/` (with `.gitkeep`); no per-project `_templ/` is written. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.6 (non-test) `add --severity SEV` sets the frontmatter severity; omitted defaults to `medium`; an invalid severity fails via `error()`. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-02.7 (non-test) `new` is an alias for `add` (identical behaviour). -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes

### WP-03 -- Inspect & lifecycle: show (+--json), close, open (move NNNN dir + status mirror) (status: Done)

- AC-03.1 (non-test) `intent issues show ID` prints the issue's frontmatter + body for an id in either bucket. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-03.2 (non-test) `intent issues show ID --json` emits valid (jq-parseable) JSON with id/title/status/severity/date/reporter fields. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-03.3 (non-test) `intent issues close ID` moves the whole `OPEN/NNNN/` directory to `CLOSED/NNNN/` and sets frontmatter `status: CLOSED`, atomically. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-03.4 (non-test) `intent issues open ID` moves `CLOSED/NNNN/` to `OPEN/NNNN/` and sets `status: OPEN`. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-03.5 (non-test) A legacy `RESOLVED` issue is read, listed, and shown as CLOSED (normalised; no third state surfaces). -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes
- AC-03.6 (non-test) An unknown/missing ID on show/close/open fails via `error()` (non-zero, clear message) -- no silent success. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes

### WP-04 -- Gate & integration: bats green, critic-shell clean, intent help, doctor, ac gate (status: Done)

- AC-04.1 (non-test) The new bats suite is green (matts runs the full suite). -- evidence: matts ran the full suite: all tests pass -- satisfied: yes
- AC-04.2 (non-test) `critic-shell` review of `bin/intent_issues` is clean (Highlander / Thin-Coordinator / No-Silent-Errors). -- evidence: critic-shell re-review: 0 critical, 0 warning; Highlander/Thin-Coordinator/portability clean -- satisfied: yes
- AC-04.3 (non-test) `intent help` / `intent help issues` documents the command and its five verbs. -- evidence: intent help lists issues; intent issues help lists five verbs -- satisfied: yes
- AC-04.4 (non-test) `intent doctor` is green after a scaffold. -- evidence: intent doctor: all checks passed -- satisfied: yes

### WP-05 -- Fleet normalisation (POST-SHIP, cross-repo): normalise Utilz/Lamplight intent/issues to canon (status: Done)

- AC-05.1 (non-test) Every fleet project with an `intent/issues/` tree (Lamplight, Conflab, Utilz, Intent) is normalised to the ratified canon: directory-per-issue `{OPEN,CLOSED}/NNNN/NNNN-slug.md`, `RESOLVED` -> CLOSED, vendored `_templ/` removed (Intent now owns the template). -- evidence: Utilz (0171297), Lamplight (7058fd3a8), Conflab (49428b4f), Intent (issue 0001) all dir-per-issue, statuses CLOSED, _templ dropped; verified via intent issues list --kind all -- satisfied: yes
- AC-05.2 (non-test) `issue_file` picks the frontmatter-bearing primary among a multi-`.md` issue directory (legacy satellites like `NNNN-resolved.md` / `-session.md` carry no frontmatter), so `show` / `list` never surface an empty satellite. Required to adopt Lamplight's rich issues. -- evidence: tests/unit/intent_issues.bats was green at this thread's close (v2); retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6) -- satisfied: yes

## Acceptance Tests

### WP-01 -- Foundation & format: dir-per-issue canon, Intent-owned template, scaffold, MODULES row, dispatch skeleton (status: Done)

- AT-01.1 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-01.1 -- status: n/a -- test: "dispatch: intent issues routes to bin/intent_issues" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-01.4 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-01.4 -- status: n/a -- test: "help: usage lists the five verbs; unknown verb exits non-zero" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close

### WP-02 -- Create & list: add (dir-per-issue, id alloc, stamp), list --kind open/closed/all (status: Done)

- AT-02.1 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.1 -- status: n/a -- test: "add: allocates id, writes OPEN/NNNN/NNNN-slug.md, prints ID:Title" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.2 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.2 -- status: n/a -- test: "add: second add increments id, no collision" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.3 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.3 -- status: n/a -- test: "list: default lists OPEN" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.4 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.4 -- status: n/a -- test: "list: --kind filters; empty state clean; invalid kind errors" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.5 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.5 -- status: n/a -- test: "add: lazily scaffolds issues dirs, no _templ" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.6 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.6 -- status: n/a -- test: "add: --severity sets severity; invalid severity errors" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-02.7 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-02.7 -- status: n/a -- test: "add: 'new' is an alias for 'add'" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close

### WP-03 -- Inspect & lifecycle: show (+--json), close, open (move NNNN dir + status mirror) (status: Done)

- AT-03.1 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.1 -- status: n/a -- test: "show: prints frontmatter + body" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-03.2 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.2 -- status: n/a -- test: "show --json: valid json with expected fields" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-03.3 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.3 -- status: n/a -- test: "close: moves OPEN/NNNN dir to CLOSED + status" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-03.4 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.4 -- status: n/a -- test: "open: moves CLOSED/NNNN dir to OPEN + status" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-03.5 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.5 -- status: n/a -- test: "read: RESOLVED normalised to CLOSED" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close
- AT-03.6 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-03.6 -- status: n/a -- test: "error: unknown id on show/close/open exits non-zero" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close

### WP-04 -- Gate & integration: bats green, critic-shell clean, intent help, doctor, ac gate (status: Done)

_(no tests in this group)_

### WP-05 -- Fleet normalisation (POST-SHIP, cross-repo): normalise Utilz/Lamplight intent/issues to canon (status: Done)

- AT-05.2 (non-test) Witness tests/unit/intent_issues.bats, green at this thread's close; retired with the v2 shell at the 3.0.1 cut (ST0056 AC-00.6). -- covers AC-05.2 -- status: n/a -- test: "show: picks the frontmatter-bearing primary among multi-file issue dirs" -- RETIRED at the 3.0.1 cut: its witness tests/unit/intent_issues.bats went with the v2 shell (ST0056 AC-00.6); it was green at this thread's close

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
