---
st_id: ST0095
title: Git-style ids: threads, work packages and issues unique by construction
---

# ST0095: Git-style ids: threads, work packages and issues unique by construction -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Core tests take ids from the mint (status: Not Started)

- AC-01.1 (non-test) No test in intentsvcs or intentd names an id it expects a mint to produce: each binds what the mint returned. -- satisfied: no

### WP-02 -- The mint verbs say their id, and CLI tests read it (status: WIP)

- AC-02.1 st new, wp new and issues add each print the id they minted in one machine-readable form, the same for all three. -- satisfied: no (computed)
- AC-02.2 (non-test) No intent-cli test names an id it expects a mint to produce, and one helper beside result_lines is the only place a CLI test reads one. -- satisfied: no

### WP-03 -- The commit gate spares the main store; scripts capture ids (status: Not Started)

- AC-03.1 A commit made with a build whose store schema is newer than the checkout's store leaves that store at its own schema: the commit gate judges a copy. -- satisfied: no (computed)
- AC-03.2 The brew smoke passes whatever id the mint returns. -- satisfied: no (computed)
- AC-03.3 Two runs of the explorer screenshots write byte-identical images from seeded ids. -- satisfied: no (computed)

### WP-04 -- The waist: one module owns an id (status: Not Started)

- AC-04.1 (non-test) A thread, work package or issue id is a typed value that only intentsvcs::id constructs, and no other file formats, parses or scans one. -- satisfied: no
- AC-04.2 The change is behaviour-neutral: an existing project's canon, views and output are byte-for-byte what they were, and intent sync plans nothing. -- satisfied: no (computed)
- AC-04.3 (non-test) intentsvcs::id is registered in intent/llm/MODULES.md before the file exists. -- satisfied: no

### WP-05 -- The merge knows both families (status: Not Started)

- AC-05.1 A merge of two branches that each minted hashed records plans no step that takes a side. -- satisfied: no (computed)
- AC-05.2 A merge in which both sides added rows to .intentfiles is resolved by intent sync --apply --yes alone, and the two aggregate views are regenerated. -- satisfied: no (computed)
- AC-05.3 An ordinal both sides minted before the upgrade is repaired at the merge by moving this clone's record to a hashed id. -- satisfied: no (computed)
- AC-05.4 Two branches that each add a work package to one thread merge with both packages kept and no claim moved. -- satisfied: no (computed)

### WP-06 -- The hashed family (status: Not Started)

- AC-06.1 Two clones, and two worktrees, that each mint a thread, a work package in one shared thread, and an issue from one base, merge with different ids and no repair. -- satisfied: no (computed)
- AC-06.2 A new id is its kind's marker and the front of the SHA-256 of its creation event's id, 16 hex for a thread and an issue and 8 for a work package, and the record carries that event's id as origin. -- satisfied: no (computed)
- AC-06.3 No verb mints an ordinal. -- satisfied: no (computed)
- AC-06.4 intent upgrade rewrites an existing project's canon format to the same bytes on two clones upgraded apart, and changes no id. -- satisfied: no (computed)
- AC-06.5 A canon file whose name is neither family is refused by name, and none is skipped. -- satisfied: no (computed)
- AC-06.6 (non-test) Brew 3.4.0 refuses an upgraded project by the schema's name and deletes nothing. -- satisfied: no
- AC-06.7 A mint whose short form equals that of a visible record of its own kind draws a new creation event id before anything is written, and refuses loudly after its last draw. -- satisfied: no (computed)

### WP-07 -- Prefix resolution (status: Not Started)

- AC-07.1 A unique prefix of at least four hex characters names its record, an ambiguous one returns every candidate, and an ordinal-shaped spelling is always the ordinal. -- satisfied: no (computed)
- AC-07.2 Canon, claims, .intentfiles, related, criterion groups and event subjects carry whole ids, never a prefix. -- satisfied: no (computed)

### WP-08 -- The order (status: Not Started)

- AC-08.1 Every listing orders ordinals by number, then hashed records by creation, in its present buckets and direction. -- satisfied: no (computed)
- AC-08.2 A project that holds only ordinals renders every view as it did before. -- satisfied: no (computed)

### WP-09 -- Criteria follow their package, and doctor verifies (status: Not Started)

- AC-09.1 wp done refuses a hashed work package while a criterion of its own is unsatisfied. -- satisfied: no (computed)
- AC-09.2 doctor reports a hashed id that is not its marker and the hash of its origin. -- satisfied: no (computed)
- AC-09.3 acceptance.md lists a hashed package's criteria under its heading, and doctor names two records of one kind that share a short form as an advisory. -- satisfied: no (computed)

### WP-10 -- Every door takes and shows both families (status: Not Started)

- AC-10.1 Every verb, MCP tool and address that takes an id takes a whole id of either family or a unique prefix, and refuses an ambiguous prefix with each candidate's id, kind, title and created date. -- satisfied: no (computed)
- AC-10.2 Every existing spelling of an ordinal keeps its meaning: 46, 0046, ST46, ST0046, s46, i21 and ST0092/09. -- satisfied: no (computed)
- AC-10.3 A table shows a hashed id by its short form, unclipped, beside ordinals, and the issues table has a Created column. -- satisfied: no (computed)
- AC-10.4 GraphQL carries every id as a string, and no URI, claim or manifest row carries a prefix. -- satisfied: no (computed)
- AC-10.5 st remint and issues remint move an ordinal to a hashed id with one argument, and a renumber call is refused with its remedy. -- satisfied: no (computed)
- AC-10.6 One command answers, for a token, whether it is an id, of which kind, and which record it names. -- satisfied: no (computed)

### WP-11 -- The TUI, the app and the served page (status: Not Started)

- AC-11.1 The TUI lists, opens and moves between hashed threads, work packages and issues, and an all-digit id keeps its leading zero. -- satisfied: no (computed)
- AC-11.2 (non-test) The Swift app shows a hashed fixture, and the served page's refusals name both families. -- satisfied: no

### WP-12 -- The pipeline asks the waist; the drill; the fleet (status: Not Started)

- AC-12.1 No script, hook, template or workflow carries a pattern for an id, and the commit gate refuses a new one. -- satisfied: no (computed)
- AC-12.2 (non-test) The drill: a clone of this repository upgraded with the new build carries a hashed thread, work package and issue through the whole commit gate, doctor and the pull-request check, before the rehearsal. -- satisfied: no
- AC-12.3 (non-test) The fleet's upgrade note says what each estate does, and every machine upgrades before any hashed record is committed there. -- satisfied: no

### WP-13 -- Docs, skills, rules and templates teach the new ids (status: Not Started)

- AC-13.1 (non-test) No page, skill, rule or template teaches minting or predicting an ordinal, and each that teaches an id teaches both families and the prefix. -- satisfied: no
- AC-13.2 (non-test) The team page's collision sections are written from a drive of the new build in which the five-open-merge-requests shape mints five different ids, and every command on every changed page was driven. -- satisfied: no
- AC-13.3 (non-test) The upgrade note and the release notes say what a project does on upgrade, and that nothing is renumbered. -- satisfied: no

## Acceptance Tests

### WP-01 -- Core tests take ids from the mint (status: Not Started)

_(no tests in this group)_

### WP-02 -- The mint verbs say their id, and CLI tests read it (status: WIP)

_(no tests in this group)_

### WP-03 -- The commit gate spares the main store; scripts capture ids (status: Not Started)

_(no tests in this group)_

### WP-04 -- The waist: one module owns an id (status: Not Started)

_(no tests in this group)_

### WP-05 -- The merge knows both families (status: Not Started)

_(no tests in this group)_

### WP-06 -- The hashed family (status: Not Started)

_(no tests in this group)_

### WP-07 -- Prefix resolution (status: Not Started)

_(no tests in this group)_

### WP-08 -- The order (status: Not Started)

_(no tests in this group)_

### WP-09 -- Criteria follow their package, and doctor verifies (status: Not Started)

_(no tests in this group)_

### WP-10 -- Every door takes and shows both families (status: Not Started)

_(no tests in this group)_

### WP-11 -- The TUI, the app and the served page (status: Not Started)

_(no tests in this group)_

### WP-12 -- The pipeline asks the waist; the drill; the fleet (status: Not Started)

_(no tests in this group)_

### WP-13 -- Docs, skills, rules and templates teach the new ids (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
