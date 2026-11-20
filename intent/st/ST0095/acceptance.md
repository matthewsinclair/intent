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

### ST-level

- AC-00.1 (non-test) Highlander, as hv decision 67 asks: at the close, an independent sweep of the whole tree finds one home for every id concern (mint, stored-form read, operator read, format, order, validate, scan, criterion and test labels, constants) in every language, and every finding of the sweep of the existing code made at the start (refs/bank/vc/st0095/highlander-*) is resolved in the package that owns it or answered on the record. -- satisfied: no
- AC-00.2 (non-test) Highlander on every change, as hv decision 67 asks: every unit of this thread lands with a critic-<lang> review of its diff for each language the diff touches, each Highlander finding fixed or answered, read by vc against the diff before land. -- satisfied: no

### WP-01 -- Core tests take ids from the mint (status: Done)

- AC-01.1 (non-test) No test in intentsvcs or intentd names an id it expects a mint to produce: each binds what the mint returned. -- evidence: ae11d9939: cc's bank wp01-v1 (patch-id 9a84ec4da587) landed by vc on cc's proof, bin/devbin gate rust green at the base plus the bank, and cc's reading of every predicting site in intentsvcs and intentd -- satisfied: yes

### WP-02 -- The mint verbs say their id, and CLI tests read it (status: Done)

- AC-02.1 st new, wp new and issues add each print the id they minted in one machine-readable form, the same for all three. -- satisfied: yes (computed)
- AC-02.2 (non-test) No intent-cli test names an id it expects a mint to produce, and one helper beside result_lines is the only place a CLI test reads one. -- evidence: ed1caeb36: ic's bank wp02 (patch-id 97c87b96bb5a) landed by vc on ic's proof; no intent-cli test names an id it expects a mint to produce, and tests/common's created() is the one reader of a minted id -- satisfied: yes

### WP-03 -- The commit gate spares the main store; scripts capture ids (status: WIP)

- AC-03.1 A commit made with a build whose store schema is newer than the checkout's store leaves that store at its own schema: the commit gate judges a copy. -- WITHDRAWN: Restated as AC-03.4, non-test, with the same text: its proof is a driven run of the guard against a schema-33 store and with no store, which no committed test carries, and a criterion's kind cannot be changed in place. This replaces the withdrawal just before it, whose reason ended in a garbled placeholder. (by vc)
- AC-03.2 The brew smoke passes whatever id the mint returns. -- WITHDRAWN: Restated as AC-03.5, non-test, with the same text: its proof is a driven run of bin/devbin brew smoke against the installed keg, which no committed test carries, and a criterion's kind cannot be changed in place. (by vc)
- AC-03.3 Two runs of the explorer screenshots write byte-identical images from seeded ids. -- satisfied: no (computed)
- AC-03.4 (non-test) A commit made with a build whose store schema is newer than the checkout's store leaves that store at its own schema: the commit gate judges a copy. -- evidence: dc's proof of refs/bank/dc/st0095/wp03-guard-v3 (3df1d4102), sent to vc 2026-11-19 11:21Z and landed by vc: red at the base ed3fe6361 on the 2026-11-08 store at schema 33, where the gate passed and migrated the store to 35; green on the final code, G2' at 11:18:20Z on that store (the copy path, rc 1 naming the hand-edited view alone, the store byte-identical at 33 with no WAL) and G4 at 11:18:55Z with no store (the copy path, no store made). -- satisfied: yes
- AC-03.5 (non-test) The brew smoke passes whatever id the mint returns. -- evidence: dc's proof of refs/bank/dc/st0095/wp03-capture-wip3 (538e58d46), sent to vc 2026-11-19 11:21Z and landed by vc: bin/devbin brew smoke from the capture worktree against the installed 3.4.0 keg (461e39ba145d) at 10:58:02Z, rc 0, every arm ok; arm 7 takes the fixture's id from minted_id in bin/.devbin/cmd/shared/mint.lib and names the thread by its title, so no id is predicted. Only comments changed after the run. 3.4.0's first mint is ST0001, so the base passes too: the first run whose first mint is a hash is the cut's own brew smoke on the release keg. -- satisfied: yes

### WP-04 -- The waist: one module owns an id (status: Done)

- AC-04.1 (non-test) A thread, work package or issue id is a typed value that only intentsvcs::id constructs, and no other file formats, parses or scans one. -- evidence: cc's proof of refs/bank/cc/st0095/wp04-v4-on-566c69e23, sent to vc 2026-11-19 11:55Z and landed at 31dd4430c: critic-rust's checklist found every pre-change id site of the two audits (hl-A, hl-B) now goes through intentsvcs::id; vc's read of the diff: no line it adds outside id.rs formats an id by hand, and 54 such lines go. Its answered findings are owned by WP-06, WP-07, WP-08, WP-09 and ic's WP-10, each named in the proof. -- satisfied: yes
- AC-04.2 An existing project's canon and views are byte-for-byte what they were, intent sync plans nothing, and output changes only where an id was spelled two ways, each change listed in the proof. -- WITHDRAWN: Restated as AC-04.4, non-test: its proof is a driven smoke on a clone of this project, which no committed test carries, and a criterion's kind cannot be changed in place. AC-04.4 also says what the smoke measured: on this repository sync still plans the one step that brew 3.4.0 plans on the same commit, removing 38 committed issue views that intent/.intentfiles does not declare, which is the repository's state and ST0096 WP-01's to settle. (by vc)
- AC-04.3 (non-test) intentsvcs::id is registered in intent/llm/MODULES.md before the file exists. -- evidence: fa86e7e35: intentsvcs::id registered in intent/llm/MODULES.md, landed while id.rs exists on no branch of main -- satisfied: yes
- AC-04.4 (non-test) An existing project's canon and views are byte-for-byte what they were, intent sync plans nothing that the release before it does not plan on the same commit, and output changes only where an id was spelled two ways, each change listed in the proof. -- evidence: cc's smoke in its proof of 2026-11-19 11:55Z, landed at 31dd4430c: the v4 binary on a fresh clone of this project at 4a831efa7 under an isolated HOME; sync --to-disk left git status empty; sync's one remaining step is byte-identical (cmp) to the one brew 3.4.0 plans on the same commit, removing 38 committed issue views that intent/.intentfiles does not declare; eleven output changes, each where an id was spelled two ways, are listed in the proof. -- satisfied: yes

### WP-05 -- The merge knows both families (status: WIP)

- AC-05.1 A merge of two branches that each minted hashed records plans no step that takes a side. -- satisfied: yes (computed)
- AC-05.2 A merge in which both sides added rows to .intentfiles is resolved by intent sync --apply --yes alone, and the two aggregate views are regenerated. -- WITHDRAWN: Its subject has gone: ST0096 (hv decision 66) takes .intentfiles and the two aggregate views out of git, so ST0095's merge neither merges .intentfiles as a set nor regenerates the lists. A branch still in today's format runs intent upgrade before it merges, and issue 0704 is superseded by ST0096. cc's call, 2026-11-19. (by cc)
- AC-05.3 An ordinal both sides minted before the upgrade is repaired at the merge by moving this clone's record to a hashed id. -- satisfied: yes (computed)
- AC-05.4 Two branches that each add a work package to one thread merge with both packages kept and no claim moved. -- satisfied: yes (computed)

### WP-06 -- The hashed family (status: WIP)

- AC-06.1 Two clones, and two worktrees, that each mint a thread, a work package in one shared thread, and an issue from one base, merge with different ids and no repair. -- satisfied: no (computed)
- AC-06.2 A new id is its kind's marker and the front of the SHA-256 of its creation event's id, 16 hex for a thread and an issue and 8 for a work package, and the record carries that event's id as origin. -- satisfied: yes (computed)
- AC-06.3 No verb mints an ordinal. -- satisfied: no (computed)
- AC-06.4 intent upgrade rewrites an existing project's canon format to the same bytes on two clones upgraded apart, and changes no id. -- satisfied: yes (computed)
- AC-06.5 A canon file whose name is neither family is refused by name, and none is skipped. -- satisfied: yes (computed)
- AC-06.6 (non-test) Brew 3.4.0 refuses an upgraded project by the schema's name and deletes nothing. -- satisfied: no
- AC-06.7 A mint whose short form equals that of a visible record of its own kind draws a new creation event id before anything is written, and refuses loudly after its last draw. -- satisfied: yes (computed)

### WP-07 -- Prefix resolution (status: Not Started)

- AC-07.1 A unique prefix of at least four hex characters names its record, an ambiguous one returns every candidate, and an ordinal-shaped spelling is always the ordinal. -- satisfied: no (computed)
- AC-07.2 Canon, claims, .intentfiles, related, criterion groups and event subjects carry whole ids, never a prefix. -- satisfied: no (computed)

### WP-08 -- The order (status: WIP)

- AC-08.1 Every listing orders ordinals by number, then hashed records by creation, in its present buckets and direction. -- satisfied: no (computed)
- AC-08.2 A project that holds only ordinals renders every view as it did before. -- satisfied: no (computed)

### WP-09 -- Criteria follow their package, and doctor verifies (status: Not Started)

- AC-09.1 wp done refuses a hashed work package while a criterion of its own is unsatisfied. -- satisfied: no (computed)
- AC-09.2 doctor reports a hashed id that is not its marker and the hash of its origin. -- satisfied: no (computed)
- AC-09.3 acceptance.md lists a hashed package's criteria under its heading, and doctor names two records of one kind that share a short form as an advisory. -- satisfied: no (computed)

### WP-10 -- Every door takes and shows both families (status: WIP)

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

### WP-14 -- Kind tags and the owner of a path have one home (status: Done)

- AC-14.1 One enum names the id families, with one wire spelling for each surface, and intent search --kind wp finds a work package's prose (issue 0708). -- satisfied: yes (computed)
- AC-14.2 One function answers which thread or issue owns a path, every caller asks it, and the project config carries no st_prefix key. -- satisfied: yes (computed)

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- Core tests take ids from the mint (status: Done)

_(no tests in this group)_

### WP-02 -- The mint verbs say their id, and CLI tests read it (status: Done)

- AT-02.1 `native/rust/crates/intent-cli/tests/the_mint_verbs_say_their_id.rs` -- covers AC-02.1 -- status: green -- Written with WP-02's code; the row moves through red as the machine requires. Green in ic's bin/devbin gate rust on gyges at f7780b922, the base plus the landed bank (ed1caeb36).

### WP-03 -- The commit gate spares the main store; scripts capture ids (status: WIP)

_(no tests in this group)_

### WP-04 -- The waist: one module owns an id (status: Done)

_(no tests in this group)_

### WP-05 -- The merge knows both families (status: WIP)

- AT-05.1 `native/rust/crates/intent-cli/tests/a_pull_is_reflected_by_the_next_verb.rs:729` -- covers AC-05.1 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-05.3 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs:279` -- covers AC-05.3 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-05.4 `native/rust/crates/intent-cli/tests/a_pull_is_reflected_by_the_next_verb.rs:827` -- covers AC-05.4 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4

### WP-06 -- The hashed family (status: WIP)

- AT-06.2 `native/rust/crates/intentsvcs/tests/the_one_mint.rs:64` -- covers AC-06.2 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.4 `native/rust/crates/intentsvcs/tests/canon_round_trip.rs:372` -- covers AC-06.4 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.5 `native/rust/crates/intentsvcs/tests/ingest_refusal.rs:364` -- covers AC-06.5 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.7 `native/rust/crates/intentsvcs/tests/the_one_mint.rs:78` -- covers AC-06.7 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4

### WP-07 -- Prefix resolution (status: Not Started)

_(no tests in this group)_

### WP-08 -- The order (status: WIP)

_(no tests in this group)_

### WP-09 -- Criteria follow their package, and doctor verifies (status: Not Started)

_(no tests in this group)_

### WP-10 -- Every door takes and shows both families (status: WIP)

_(no tests in this group)_

### WP-11 -- The TUI, the app and the served page (status: Not Started)

_(no tests in this group)_

### WP-12 -- The pipeline asks the waist; the drill; the fleet (status: Not Started)

_(no tests in this group)_

### WP-13 -- Docs, skills, rules and templates teach the new ids (status: Not Started)

_(no tests in this group)_

### WP-14 -- Kind tags and the owner of a path have one home (status: Done)

- AT-14.1 `native/rust/crates/intentsvcs/src/id.rs:1817` -- covers AC-14.1 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.2 `native/rust/crates/intent-cli/tests/search_surface.rs:181` -- covers AC-14.1 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.3 `native/rust/crates/intentsvcs/tests/one_answer_to_who_owns_a_path.rs:23` -- covers AC-14.2 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.4 `native/rust/crates/intentsvcs/tests/retired_st_prefix.rs:134` -- covers AC-14.2 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
