---
st_id: ST0055
title: Add `intent issues` command
---

# ST0055: Add `intent issues` command -- Acceptance

> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Foundation & format: dir-per-issue canon, Intent-owned template, scaffold, MODULES row, dispatch skeleton (status: WIP)

- AC-01.1 `bin/intent_issues` exists, is executable, and `intent issues` dispatches to it as a project command (fails cleanly outside an Intent project). -- satisfied: yes (computed)
- AC-01.2 (non-test) The issue template lives at `lib/templates/issues/_ISSUE.md` (Intent-owned single source; no inline heredoc in `bin/intent_issues`). -- evidence: lib/templates/issues/_ISSUE.md exists; intent_issues stamps via sed, no heredoc; critic-shell no-heredoc clean -- satisfied: yes
- AC-01.3 (non-test) `MODULES.md` carries a row for `bin/intent_issues`, added before the module ships. -- evidence: MODULES.md 'Issue tracker' row for bin/intent_issues -- satisfied: yes
- AC-01.4 `intent issues help` and an unknown subcommand both print usage listing the five verbs; exit code is 0 for `help`, non-zero for the unknown verb. -- satisfied: yes (computed)

### WP-02 -- Create & list: add (dir-per-issue, id alloc, stamp), list --kind open/closed/all (status: WIP)

- AC-02.1 `intent issues add "Title"` allocates the next zero-padded 4-digit id (max+1 across OPEN+CLOSED), writes `intent/issues/OPEN/NNNN/NNNN-slug.md` with stamped frontmatter (id/title/date/reporter/status=OPEN/severity), and prints `NNNN:Title`. -- satisfied: yes (computed)
- AC-02.2 A second `add` allocates the next sequential id with no collision (gaps tolerated, ids not reused below max). -- satisfied: yes (computed)
- AC-02.3 `intent issues` and `intent issues list` (no args) list OPEN issues by default. -- satisfied: yes (computed)
- AC-02.4 `--kind open|closed|all` filters the listing; an empty bucket prints a clean empty state, not an error; an invalid `--kind` fails via `error()`. -- satisfied: yes (computed)
- AC-02.5 The first `add` lazily scaffolds `intent/issues/{OPEN,CLOSED}/` (with `.gitkeep`); no per-project `_templ/` is written. -- satisfied: yes (computed)
- AC-02.6 `add --severity SEV` sets the frontmatter severity; omitted defaults to `medium`; an invalid severity fails via `error()`. -- satisfied: yes (computed)
- AC-02.7 `new` is an alias for `add` (identical behaviour). -- satisfied: yes (computed)

### WP-03 -- Inspect & lifecycle: show (+--json), close, open (move NNNN dir + status mirror) (status: WIP)

- AC-03.1 `intent issues show ID` prints the issue's frontmatter + body for an id in either bucket. -- satisfied: yes (computed)
- AC-03.2 `intent issues show ID --json` emits valid (jq-parseable) JSON with id/title/status/severity/date/reporter fields. -- satisfied: yes (computed)
- AC-03.3 `intent issues close ID` moves the whole `OPEN/NNNN/` directory to `CLOSED/NNNN/` and sets frontmatter `status: CLOSED`, atomically. -- satisfied: yes (computed)
- AC-03.4 `intent issues open ID` moves `CLOSED/NNNN/` to `OPEN/NNNN/` and sets `status: OPEN`. -- satisfied: yes (computed)
- AC-03.5 A legacy `RESOLVED` issue is read, listed, and shown as CLOSED (normalised; no third state surfaces). -- satisfied: yes (computed)
- AC-03.6 An unknown/missing ID on show/close/open fails via `error()` (non-zero, clear message) -- no silent success. -- satisfied: yes (computed)

### WP-04 -- Gate & integration: bats green, critic-shell clean, intent help, doctor, ac gate (status: WIP)

- AC-04.1 (non-test) The new bats suite is green (matts runs the full suite). -- evidence: matts ran the full suite: all tests pass -- satisfied: yes
- AC-04.2 (non-test) `critic-shell` review of `bin/intent_issues` is clean (Highlander / Thin-Coordinator / No-Silent-Errors). -- evidence: critic-shell re-review: 0 critical, 0 warning; Highlander/Thin-Coordinator/portability clean -- satisfied: yes
- AC-04.3 (non-test) `intent help` / `intent help issues` documents the command and its five verbs. -- evidence: intent help lists issues; intent issues help lists five verbs -- satisfied: yes
- AC-04.4 (non-test) `intent doctor` is green after a scaffold. -- evidence: intent doctor: all checks passed -- satisfied: yes

### WP-05 -- Fleet normalisation (POST-SHIP, cross-repo): normalise Utilz/Lamplight intent/issues to canon (status: Not Started)

- AC-05.1 (non-test) Every fleet project with an `intent/issues/` tree (Lamplight, Conflab, Utilz, Intent) is normalised to the ratified canon: directory-per-issue `{OPEN,CLOSED}/NNNN/NNNN-slug.md`, `RESOLVED` -> CLOSED, vendored `_templ/` removed (Intent now owns the template). -- evidence: Utilz (0171297), Lamplight (7058fd3a8), Conflab (49428b4f), Intent (issue 0001) all dir-per-issue, statuses CLOSED, _templ dropped; verified via intent issues list --kind all -- satisfied: yes
- AC-05.2 `issue_file` picks the frontmatter-bearing primary among a multi-`.md` issue directory (legacy satellites like `NNNN-resolved.md` / `-session.md` carry no frontmatter), so `show` / `list` never surface an empty satellite. Required to adopt Lamplight's rich issues. -- satisfied: yes (computed)

## Acceptance Tests

### WP-01 -- Foundation & format: dir-per-issue canon, Intent-owned template, scaffold, MODULES row, dispatch skeleton (status: WIP)

- AT-01.1 `tests/unit/intent_issues.bats` -- covers AC-01.1 -- status: green -- test: "dispatch: intent issues routes to bin/intent_issues"
- AT-01.4 `tests/unit/intent_issues.bats` -- covers AC-01.4 -- status: green -- test: "help: usage lists the five verbs; unknown verb exits non-zero"

### WP-02 -- Create & list: add (dir-per-issue, id alloc, stamp), list --kind open/closed/all (status: WIP)

- AT-02.1 `tests/unit/intent_issues.bats` -- covers AC-02.1 -- status: green -- test: "add: allocates id, writes OPEN/NNNN/NNNN-slug.md, prints ID:Title"
- AT-02.2 `tests/unit/intent_issues.bats` -- covers AC-02.2 -- status: green -- test: "add: second add increments id, no collision"
- AT-02.3 `tests/unit/intent_issues.bats` -- covers AC-02.3 -- status: green -- test: "list: default lists OPEN"
- AT-02.4 `tests/unit/intent_issues.bats` -- covers AC-02.4 -- status: green -- test: "list: --kind filters; empty state clean; invalid kind errors"
- AT-02.5 `tests/unit/intent_issues.bats` -- covers AC-02.5 -- status: green -- test: "add: lazily scaffolds issues dirs, no _templ"
- AT-02.6 `tests/unit/intent_issues.bats` -- covers AC-02.6 -- status: green -- test: "add: --severity sets severity; invalid severity errors"
- AT-02.7 `tests/unit/intent_issues.bats` -- covers AC-02.7 -- status: green -- test: "add: 'new' is an alias for 'add'"

### WP-03 -- Inspect & lifecycle: show (+--json), close, open (move NNNN dir + status mirror) (status: WIP)

- AT-03.1 `tests/unit/intent_issues.bats` -- covers AC-03.1 -- status: green -- test: "show: prints frontmatter + body"
- AT-03.2 `tests/unit/intent_issues.bats` -- covers AC-03.2 -- status: green -- test: "show --json: valid json with expected fields"
- AT-03.3 `tests/unit/intent_issues.bats` -- covers AC-03.3 -- status: green -- test: "close: moves OPEN/NNNN dir to CLOSED + status"
- AT-03.4 `tests/unit/intent_issues.bats` -- covers AC-03.4 -- status: green -- test: "open: moves CLOSED/NNNN dir to OPEN + status"
- AT-03.5 `tests/unit/intent_issues.bats` -- covers AC-03.5 -- status: green -- test: "read: RESOLVED normalised to CLOSED"
- AT-03.6 `tests/unit/intent_issues.bats` -- covers AC-03.6 -- status: green -- test: "error: unknown id on show/close/open exits non-zero"

### WP-04 -- Gate & integration: bats green, critic-shell clean, intent help, doctor, ac gate (status: WIP)

_(no tests in this group)_

### WP-05 -- Fleet normalisation (POST-SHIP, cross-repo): normalise Utilz/Lamplight intent/issues to canon (status: Not Started)

- AT-05.2 `tests/unit/intent_issues.bats` -- covers AC-05.2 -- status: green -- test: "show: picks the frontmatter-bearing primary among multi-file issue dirs"

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
