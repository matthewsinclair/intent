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

### WP-06 -- The hashed family (status: Done)

- AC-06.1 Two clones, and two worktrees, that each mint a thread, a work package in one shared thread, and an issue from one base, merge with different ids and no repair. -- satisfied: yes (computed)
- AC-06.2 A new id is its kind's marker and the front of the SHA-256 of its creation event's id, 16 hex for a thread and an issue and 8 for a work package, and the record carries that event's id as origin. -- satisfied: yes (computed)
- AC-06.3 No verb mints an ordinal. -- satisfied: yes (computed)
- AC-06.4 intent upgrade rewrites an existing project's canon format to the same bytes on two clones upgraded apart, and changes no id. -- satisfied: yes (computed)
- AC-06.5 A canon file whose name is neither family is refused by name, and none is skipped. -- satisfied: yes (computed)
- AC-06.6 (non-test) Brew 3.4.0 cannot read an upgraded project: it reports each canon file as schema-invalid, says it could not read the committed canon, and changes nothing. -- evidence: refs/bank/devbin-dc/st0095/wp13/drive-log-run2 (blob 8bad2f3d0), section 'UPGRADE 4: an older Intent on the upgraded project': brew 3.4.0 (461e39ba145d) runs st list on the upgraded project and prints 'could not read the committed canon' with one schema-invalid finding per canon file, rc=1. Every 3.4.0 verb reads the canon first, so none reaches a write; the drive ran a read verb and captured no git status after it -- satisfied: yes
- AC-06.7 A mint whose short form equals that of a visible record of its own kind draws a new creation event id before anything is written, and refuses loudly after its last draw. -- satisfied: yes (computed)

### WP-07 -- Prefix resolution (status: Done)

- AC-07.1 A unique prefix of at least four hex characters names its record, an ambiguous one returns every candidate, and an ordinal-shaped spelling is always the ordinal. -- satisfied: yes (computed)
- AC-07.2 Canon, claims, .intentfiles, related, criterion groups and event subjects carry whole ids, never a prefix. -- satisfied: yes (computed)

### WP-08 -- The order (status: Done)

- AC-08.1 Every listing orders ordinals by number, then hashed records by creation, in its present buckets and direction. -- satisfied: yes (computed)
- AC-08.2 A project that holds only ordinals renders every view as it did before. -- satisfied: yes (computed)

### WP-09 -- Criteria follow their package, and doctor verifies (status: Done)

- AC-09.1 wp done refuses a hashed work package while a criterion of its own is unsatisfied. -- satisfied: yes (computed)
- AC-09.2 doctor reports a hashed id that is not its marker and the hash of its origin. -- satisfied: yes (computed)
- AC-09.3 acceptance.md lists a hashed package's criteria under its heading, and doctor names two records of one kind that share a short form as an advisory. -- satisfied: yes (computed)

### WP-10 -- Every door takes and shows both families (status: Done)

- AC-10.1 Every verb, MCP tool and address that takes an id takes a whole id of either family or a unique prefix, and refuses an ambiguous prefix with each candidate's id, kind, title and created date. -- satisfied: yes (computed)
- AC-10.2 Every existing spelling of an ordinal keeps its meaning: 46, 0046, ST46, ST0046, s46, i21 and ST0092/09. -- satisfied: yes (computed)
- AC-10.3 A table shows a hashed id by its short form, unclipped, beside ordinals, and the issues table has a Created column. -- satisfied: yes (computed)
- AC-10.4 GraphQL carries every id as a string, and no URI, claim or manifest row carries a prefix. -- satisfied: yes (computed)
- AC-10.5 st remint and issues remint move an ordinal to a hashed id with one argument, and a renumber call is refused with its remedy. -- satisfied: yes (computed)
- AC-10.6 One command answers, for a token, whether it is an id, of which kind, and which record it names. -- satisfied: yes (computed)

### WP-11 -- The TUI, the app and the served page (status: Done)

- AC-11.1 The TUI lists, opens and moves between hashed threads, work packages and issues, and an all-digit id keeps its leading zero. -- satisfied: yes (computed)
- AC-11.2 (non-test) The Swift app shows a hashed fixture, and the served page's refusals name both families. -- evidence: 9df6dfad4: AT-11.2 (intentd store.rs) and the hashed-fixture arm in native/macos/Intent/IntentTests/ProjectServiceTests.swift:17, gated by devbin-cc and read by ic and vc -- satisfied: yes

### WP-12 -- The pipeline asks the waist; the drill; the fleet (status: Not Started)

- AC-12.1 No script the commit gate or CI runs, no hook and no template carries a pattern for an id, and the commit gate refuses a new one. (Scoped to the gated half by vc under hv's pen, on hv decision 79, 2026-11-21; the by-hand half is AC-12.4, descoped.) -- satisfied: yes (computed)
- AC-12.2 (non-test) The drill: a clone of this repository upgraded with the new build carries a hashed thread, work package and issue through the whole commit gate, doctor and the pull-request check, before the rehearsal. -- satisfied: no
- AC-12.3 (non-test) The fleet's upgrade note says what each estate does, and every machine upgrades before any hashed record is committed there. -- satisfied: no
- AC-12.4 The by-hand parity scripts the id census still lists (the manual conservation_check, v2_estate_issue_carry, sync_issue_loss, upgrade_issue_loss, canon_clone_completeness, sparse_tree_equals_manifest, canon_concurrent_diff, v2_prose_survival, estate_census and docs_defect_disposition_check, and realise_plan.sh, 22 census rows) ask the id door or are retired. -- DESCOPED to ST0097 by vc, under hv's pen (decision 4), on hv decision 79, 2026-11-21

### WP-13 -- Docs, skills, rules and templates teach the new ids (status: Not Started)

- AC-13.1 (non-test) No page, skill, rule or template teaches minting or predicting an ordinal, and each that teaches an id teaches both families and the prefix. -- evidence: landed 4a4930e4e; gate 01M7XBC3 (rust rc 0, shell 884 ok); drive log refs/bank/devbin-dc/st0095/wp13/drive-log-run2 -- satisfied: yes
- AC-13.2 (non-test) The team page's collision sections are written from a drive of the new build in which the five-open-merge-requests shape mints five different ids, and every command on every changed page was driven. -- evidence: landed 4a4930e4e; gate 01M7XBC3 (rust rc 0, shell 884 ok); drive log refs/bank/devbin-dc/st0095/wp13/drive-log-run2 -- satisfied: yes
- AC-13.3 (non-test) The upgrade note and the release notes say what a project does on upgrade, and that nothing is renumbered. -- evidence: landed 4a4930e4e; gate 01M7XBC3 (rust rc 0, shell 884 ok); drive log refs/bank/devbin-dc/st0095/wp13/drive-log-run2 -- satisfied: yes

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

- AT-05.1 `native/rust/crates/intent-cli/tests/a_pull_is_reflected_by_the_next_verb.rs:662` -- covers AC-05.1 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-05.3 `native/rust/crates/intent-cli/tests/a_pull_is_repaired_by_one_command.rs:202` -- covers AC-05.3 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-05.4 `native/rust/crates/intent-cli/tests/a_pull_is_reflected_by_the_next_verb.rs:779` -- covers AC-05.4 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4

### WP-06 -- The hashed family (status: Done)

- AT-06.1 `native/rust/crates/intent-cli/tests/two_clones_and_two_worktrees_that_each_mint_merge_with_no_repair.rs:29` -- covers AC-06.1 -- status: green -- green in cc's gate (Desk 01M7YH2P) at f1787d26c, landed at 8ae5ce5b0
- AT-06.2 `native/rust/crates/intentsvcs/tests/the_one_mint.rs:63` -- covers AC-06.2 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.3 `native/rust/crates/intent-cli/tests/no_verb_mints_an_ordinal.rs:18` -- covers AC-06.3 -- status: green -- green in cc's gate (Desk 01M7YH2P) at f1787d26c, landed at 8ae5ce5b0
- AT-06.4 `native/rust/crates/intentsvcs/tests/canon_round_trip.rs:405` -- covers AC-06.4 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.5 `native/rust/crates/intentsvcs/tests/ingest_refusal.rs:363` -- covers AC-06.5 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4
- AT-06.7 `native/rust/crates/intentsvcs/tests/the_one_mint.rs:78` -- covers AC-06.7 -- status: green -- the test is written; the machine is to-write, red, green. Green in bin/devbin gate rust at a577deb9f, 2026-11-20 15:02Z to 15:06Z by date -u; landed in ed302b7e4

### WP-07 -- Prefix resolution (status: Done)

- AT-07.1 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:105` -- covers AC-07.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in WP-07 v7's land gate (Desk 01M7WYJ501, 17:09Z by date -u), landed at 62045266e
- AT-07.2 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:270` -- covers AC-07.2 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in WP-07 v7's land gate (Desk 01M7WYJ501, 17:09Z by date -u), landed at 62045266e
- AT-07.3 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:159` -- covers AC-07.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in WP-07 v7's land gate (Desk 01M7WYJ501, 17:09Z by date -u), landed at 62045266e
- AT-07.4 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:204` -- covers AC-07.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in WP-07 v7's land gate (Desk 01M7WYJ501, 17:09Z by date -u), landed at 62045266e

### WP-08 -- The order (status: Done)

- AT-08.1 `native/rust/crates/intentsvcs/src/id.rs:3000` -- covers AC-08.1 -- status: green -- Written after the land. Red on the base: it has no id::record_order. Green at dc's gate of the landed tree, 41c6c6373 (Desk 01M7X0ZZSGJADV9BB70QPMMEZJ, 17:50:14Z to 17:55:06Z), landed at cbae96463.
- AT-08.2 `native/rust/crates/intentsvcs/src/id.rs:3046` -- covers AC-08.2 -- status: green -- Written after the land. A regression arm, with no red on the base by construction: ordinals alone order by number before and after WP-08. Green at dc's gate of the landed tree, 41c6c6373 (Desk 01M7X0ZZSGJADV9BB70QPMMEZJ, 17:50:14Z to 17:55:06Z), landed at cbae96463.

### WP-09 -- Criteria follow their package, and doctor verifies (status: Done)

- AT-09.1 `native/rust/crates/intentsvcs/tests/a_hashed_package_is_judged_by_its_own_criteria.rs:26` -- covers AC-09.1 -- status: green -- Written after the land. A regression arm, with no red on the base: the close gate already reads a group as a typed WpId (issue 0712, ST0095 WP-07). No planted fault (cc, 17:23Z). Green at dc's gate of the landed tree, 41c6c6373 (Desk 01M7X0ZZSGJADV9BB70QPMMEZJ, 17:50:14Z to 17:55:06Z), landed at cbae96463.
- AT-09.2 `native/rust/crates/intentsvcs/tests/a_hashed_package_is_judged_by_its_own_criteria.rs:54` -- covers AC-09.2 -- status: green -- Written after the land. Red on the base: doctor has no IdNotFromOrigin check. Green at dc's gate of the landed tree, 41c6c6373 (Desk 01M7X0ZZSGJADV9BB70QPMMEZJ, 17:50:14Z to 17:55:06Z), landed at cbae96463.
- AT-09.3 `native/rust/crates/intentsvcs/tests/a_hashed_package_is_judged_by_its_own_criteria.rs:87` -- covers AC-09.3 -- status: green -- Written after the land. Red on the base: doctor has no ShortFormShared advisory, and it counted a short-form pair as DuplicateId. Green at dc's gate of the landed tree, 41c6c6373 (Desk 01M7X0ZZSGJADV9BB70QPMMEZJ, 17:50:14Z to 17:55:06Z), landed at cbae96463.

### WP-10 -- Every door takes and shows both families (status: Done)

- AT-10.1 `native/rust/crates/intent-cli/tests/a_door_takes_a_hashed_ids_prefix.rs:9` -- covers AC-10.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.2 `native/rust/crates/intent-cli/tests/mcp_resources.rs:262` -- covers AC-10.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.3 `native/rust/crates/intentsvcs/tests/graphql_reads_through_the_facade.rs:187` -- covers AC-10.1 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.4 `native/rust/crates/intentsvcs/tests/operator_id_spellings.rs:28` -- covers AC-10.2 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.5 `native/rust/crates/intentsvcs/tests/operator_id_spellings.rs:59` -- covers AC-10.2 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.6 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:203` -- covers AC-10.2 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.7 `native/rust/crates/intentsvcs/tests/graphql_reads_through_the_facade.rs:19` -- covers AC-10.4 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.8 `native/rust/crates/intentsvcs/tests/a_prefix_names_one_record.rs:269` -- covers AC-10.4 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.9 `native/rust/crates/intent-cli/tests/two_clones_that_mint_one_id_are_repaired_by_remint.rs:29` -- covers AC-10.5 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.10 `native/rust/crates/intent-cli/tests/two_clones_that_mint_one_id_are_repaired_by_remint.rs:122` -- covers AC-10.5 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.11 `native/rust/crates/intent-cli/tests/every_door_reads_an_id_through_the_waist.rs:231` -- covers AC-10.6 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 80497e0c8 on 9a87b035c (gyges, 17:35Z to 17:41Z by date -u), whose tree landed at fa5ebbbbf; the marker line over each arm is a comment
- AT-10.12 `native/rust/crates/intent-cli/tests/cli_end_to_end.rs:1174` -- covers AC-10.3 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 2ebba3aa0 on 26c4b0ca0 (gyges, 18:53Z to 18:58Z by date -u), whose tree landed at 42de84456; the marker line over each arm is a comment
- AT-10.13 `native/rust/crates/intentsvcs/tests/operator_id_spellings.rs:239` -- covers AC-10.2 -- status: green -- row written after the land: red is the machine's route to green here, not a measured red. green in ic's gate at 2ebba3aa0 on 26c4b0ca0 (gyges, 18:53Z to 18:58Z by date -u), whose tree landed at 42de84456; the marker line over each arm is a comment

### WP-11 -- The TUI, the app and the served page (status: Done)

- AT-11.1 `native/rust/crates/intent-cli/src/render.rs:18742` -- covers AC-11.1 -- status: green -- the arm, written on base d88b1dccd where the explorer printed stored ids (devbin-dc, bank 75b45bd46); green on Desk run 01M7YJQ7 (build exit 0, test rust exit 0, 0 failed), landed at 33d631cbc
- AT-11.2 `native/rust/crates/intentd/src/store.rs:1121` -- covers AC-11.2 -- status: green -- the served page's two refusals each name both id families in the waist's own sentences, and a hashed path resolves with its id carried unchanged. Green at 9df6dfad4 (devbin-cc's gate, read by ic and vc)

### WP-12 -- The pipeline asks the waist; the drill; the fleet (status: Not Started)

- AT-12.1 `tests/unit/gated_id_door.bats` -- covers AC-12.1 -- status: green -- the three gated parity scripts ask the id door, read-only, in place of their patterns (refs/bank/dc/st0095/ac12-route/v1, landed a3e3e6461); the refusal of a new pattern is id_pattern_check.sh, gated, whose census only shrinks. Red: the arms, written on base 4b24577e7, where each gated script matched a four-digit ordinal and a hashed record dangled or went unjudged. Green on F 4b24577e7: Desk 01M7YR2W (build all, check clippy, gate rust, each rc 0) and 01M7YS30 (test shell, all 914, 0 failed); landed a3e3e6461

### WP-13 -- Docs, skills, rules and templates teach the new ids (status: Not Started)

_(no tests in this group)_

### WP-14 -- Kind tags and the owner of a path have one home (status: Done)

- AT-14.1 `native/rust/crates/intentsvcs/src/id.rs:2794` -- covers AC-14.1 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.2 `native/rust/crates/intent-cli/tests/search_surface.rs:184` -- covers AC-14.1 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.3 `native/rust/crates/intentsvcs/tests/one_answer_to_who_owns_a_path.rs:23` -- covers AC-14.2 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u
- AT-14.4 `native/rust/crates/intentsvcs/tests/retired_st_prefix.rs:134` -- covers AC-14.2 -- status: green -- Written with the unit, ST0095 WP-14. Landed in 7ed879ab3; green in bin/devbin gate rust at 13f40d241, 2026-11-20 15:17Z to 15:22Z by date -u

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
