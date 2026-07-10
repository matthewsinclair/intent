---
verblock: "10 Jul 2026:v0.2: matts - AC/AT boundary for intent issues"
st_id: ST0055
title: "Add intent issues command -- acceptance contract"
---

# ST0055 Add `intent issues` command -- Acceptance

> Canonical acceptance contract for ST0055. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written).

## Acceptance Criteria

### ST-level

None -- WP-distributed. The steel thread is done when WP-01..04 ACs are all satisfied. WP-05 (fleet normalisation) is a post-ship cross-repo chore, tracked here but sequenced after the command ships and is dogfooded; it does not gate the Intent release.

### WP-01 -- Foundation & format (status: built; ATs green)

- AC-01.1 `bin/intent_issues` exists, is executable, and `intent issues` dispatches to it as a project command (fails cleanly outside an Intent project).
- AC-01.2 (non-test) The issue template lives at `lib/templates/issues/_ISSUE.md` (Intent-owned single source; no inline heredoc in `bin/intent_issues`). -- evidence: lib/templates/issues/_ISSUE.md exists; intent_issues stamps via sed, no heredoc; critic-shell no-heredoc clean -- satisfied: yes
- AC-01.3 (non-test) `MODULES.md` carries a row for `bin/intent_issues`, added before the module ships. -- evidence: MODULES.md 'Issue tracker' row for bin/intent_issues -- satisfied: yes
- AC-01.4 `intent issues help` and an unknown subcommand both print usage listing the five verbs; exit code is 0 for `help`, non-zero for the unknown verb.

### WP-02 -- Create & list (status: built; ATs green)

- AC-02.1 `intent issues add "Title"` allocates the next zero-padded 4-digit id (max+1 across OPEN+CLOSED), writes `intent/issues/OPEN/NNNN/NNNN-slug.md` with stamped frontmatter (id/title/date/reporter/status=OPEN/severity), and prints `NNNN:Title`.
- AC-02.2 A second `add` allocates the next sequential id with no collision (gaps tolerated, ids not reused below max).
- AC-02.3 `intent issues` and `intent issues list` (no args) list OPEN issues by default.
- AC-02.4 `--kind open|closed|all` filters the listing; an empty bucket prints a clean empty state, not an error; an invalid `--kind` fails via `error()`.
- AC-02.5 The first `add` lazily scaffolds `intent/issues/{OPEN,CLOSED}/` (with `.gitkeep`); no per-project `_templ/` is written.
- AC-02.6 `add --severity SEV` sets the frontmatter severity; omitted defaults to `medium`; an invalid severity fails via `error()`.
- AC-02.7 `new` is an alias for `add` (identical behaviour).

### WP-03 -- Inspect & lifecycle (status: built; ATs green)

- AC-03.1 `intent issues show ID` prints the issue's frontmatter + body for an id in either bucket.
- AC-03.2 `intent issues show ID --json` emits valid (jq-parseable) JSON with id/title/status/severity/date/reporter fields.
- AC-03.3 `intent issues close ID` moves the whole `OPEN/NNNN/` directory to `CLOSED/NNNN/` and sets frontmatter `status: CLOSED`, atomically.
- AC-03.4 `intent issues open ID` moves `CLOSED/NNNN/` to `OPEN/NNNN/` and sets `status: OPEN`.
- AC-03.5 A legacy `RESOLVED` issue is read, listed, and shown as CLOSED (normalised; no third state surfaces).
- AC-03.6 An unknown/missing ID on show/close/open fails via `error()` (non-zero, clear message) -- no silent success.

### WP-04 -- Gate & integration (status: green except AC-04.1 full-suite -- verifier)

- AC-04.1 (non-test) The new bats suite is green (matts runs the full suite). -- evidence: matts ran the full suite: all tests pass -- satisfied: yes
- AC-04.2 (non-test) `critic-shell` review of `bin/intent_issues` is clean (Highlander / Thin-Coordinator / No-Silent-Errors). -- evidence: critic-shell re-review: 0 critical, 0 warning; Highlander/Thin-Coordinator/portability clean -- satisfied: yes
- AC-04.3 (non-test) `intent help` / `intent help issues` documents the command and its five verbs. -- evidence: intent help lists issues; intent issues help lists five verbs -- satisfied: yes
- AC-04.4 (non-test) `intent doctor` is green after a scaffold. -- evidence: intent doctor: all checks passed -- satisfied: yes

(The close-gate is the verification mechanism, not a criterion -- there is no self-referential "gate PASS" AC.)

### WP-05 -- Fleet normalisation (status: done; shipped in v2.17.1)

- AC-05.1 (non-test) Every fleet project with an `intent/issues/` tree (Lamplight, Conflab, Utilz, Intent) is normalised to the ratified canon: directory-per-issue `{OPEN,CLOSED}/NNNN/NNNN-slug.md`, `RESOLVED` -> CLOSED, vendored `_templ/` removed (Intent now owns the template). -- evidence: Utilz (0171297), Lamplight (7058fd3a8), Conflab (49428b4f), Intent (issue 0001) all dir-per-issue, statuses CLOSED, _templ dropped; verified via intent issues list --kind all -- satisfied: yes
- AC-05.2 `issue_file` picks the frontmatter-bearing primary among a multi-`.md` issue directory (legacy satellites like `NNNN-resolved.md` / `-session.md` carry no frontmatter), so `show` / `list` never surface an empty satellite. Required to adopt Lamplight's rich issues.

## Acceptance Tests

Test file: `tests/unit/intent_issues.bats` (to be created red-first).

### WP-01

- AT-01.1 tests/unit/intent_issues.bats::"dispatch: intent issues routes to bin/intent_issues" -- covers AC-01.1 -- status: green
- AT-01.4 tests/unit/intent_issues.bats::"help: usage lists the five verbs; unknown verb exits non-zero" -- covers AC-01.4 -- status: green
- Coverage: AC-01.2 + AC-01.3 are non-test (evidence on the AC line).

### WP-02

- AT-02.1 tests/unit/intent_issues.bats::"add: allocates id, writes OPEN/NNNN/NNNN-slug.md, prints ID:Title" -- covers AC-02.1 -- status: green
- AT-02.2 tests/unit/intent_issues.bats::"add: second add increments id, no collision" -- covers AC-02.2 -- status: green
- AT-02.3 tests/unit/intent_issues.bats::"list: default lists OPEN" -- covers AC-02.3 -- status: green
- AT-02.4 tests/unit/intent_issues.bats::"list: --kind filters; empty state clean; invalid kind errors" -- covers AC-02.4 -- status: green
- AT-02.5 tests/unit/intent_issues.bats::"add: lazily scaffolds issues dirs, no _templ" -- covers AC-02.5 -- status: green
- AT-02.6 tests/unit/intent_issues.bats::"add: --severity sets severity; invalid severity errors" -- covers AC-02.6 -- status: green
- AT-02.7 tests/unit/intent_issues.bats::"add: 'new' is an alias for 'add'" -- covers AC-02.7 -- status: green
- Coverage: every WP-02 AC has an AT.

### WP-03

- AT-03.1 tests/unit/intent_issues.bats::"show: prints frontmatter + body" -- covers AC-03.1 -- status: green
- AT-03.2 tests/unit/intent_issues.bats::"show --json: valid json with expected fields" -- covers AC-03.2 -- status: green
- AT-03.3 tests/unit/intent_issues.bats::"close: moves OPEN/NNNN dir to CLOSED + status" -- covers AC-03.3 -- status: green
- AT-03.4 tests/unit/intent_issues.bats::"open: moves CLOSED/NNNN dir to OPEN + status" -- covers AC-03.4 -- status: green
- AT-03.5 tests/unit/intent_issues.bats::"read: RESOLVED normalised to CLOSED" -- covers AC-03.5 -- status: green
- AT-03.6 tests/unit/intent_issues.bats::"error: unknown id on show/close/open exits non-zero" -- covers AC-03.6 -- status: green
- Coverage: every WP-03 AC has an AT.

### WP-04

- Coverage: all WP-04 ACs are non-test (bats-green / critic / doc / doctor evidence on each AC line).

### WP-05

- AT-05.2 tests/unit/intent_issues.bats::"show: picks the frontmatter-bearing primary among multi-file issue dirs" -- covers AC-05.2 -- status: green
- Coverage: AC-05.1 is non-test (cross-repo, post-ship; evidence = per-repo `intent issues list` + tree diff).
