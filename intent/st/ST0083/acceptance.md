---
st_id: ST0083
title: Agent contracts at the root or in .claude/, chosen per project, and CLAUDE.md imports AGENTS.md
---

# ST0083: Agent contracts at the root or in .claude/, chosen per project, and CLAUDE.md imports AGENTS.md -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- CLAUDE.md imports AGENTS.md: the @AGENTS.md line in _CLAUDE.md (status: Done)

- AC-01.1 (non-test) The generated CLAUDE.md opens with the line `@AGENTS.md`, rendered from `lib/templates/llm/_CLAUDE.md`, so Claude Code loads AGENTS.md at launch in both layouts; `intent claude upgrade --apply` writes the line into an existing generated CLAUDE.md with its user block carried, and a CLAUDE.md without the generated footer is held as before. -- evidence: 16f351a90 (ST0083 WP-01) and AT-01.1 green; root layout: refs/bank/cc/0083/evidence/wp01-d01-drive.log (claude upgrade --apply writes the line with the user block carried, a footerless CLAUDE.md is held); dotclaude layout: refs/bank/cc/0083/evidence/wp01-d01b-dotclaude-drive.log on WP-02's build (.claude/CLAUDE.md opens with @AGENTS.md); Claude Code itself loaded a worktree's AGENTS.md through the import in cc's session of 2026-10-15 -- satisfied: yes

### WP-02 -- The contracts key and contracts_dir: every writer, reader, the sync scope and the daemon watch derive the place from it (status: WIP)

- AC-02.1 (non-test) `intent/.config/config.json` accepts `"contracts": "root"` or `"contracts": "dotclaude"`; an absent key reads as `root` and is never written back, and any other value is refused by name with the two values it accepts. -- evidence: a85dfbdf0 (ST0083 WP-02, patch-id a2246ee4ecc9, judged on vc's GO); AT-02.1 green (contracts_layout.rs: the key reads root and dotclaude, an absent key reads as root, a third value is refused by name with the two it accepts, and root is never written back) -- satisfied: yes
- AC-02.2 (non-test) Under `dotclaude`, `intent init`, `intent agents init`, `intent agents sync`, `intent agents validate` and `intent claude upgrade --apply` read and write `.claude/AGENTS.md` and `.claude/CLAUDE.md` and never a root copy, and `usage-rules.md` is not seeded; under `root` every one of them behaves as in 3.2.3, and `usage-rules.md` stays at the root in both. -- satisfied: no
- AC-02.3 (non-test) The sync scope follows the key: in a `dotclaude` project an edit to `.claude/AGENTS.md` or `.claude/CLAUDE.md` is ingested by `intent sync --to-store` and by the running daemon exactly as an edit to the root copy is in a `root` project, and a root copy in a `dotclaude` project is not in scope. -- evidence: a85dfbdf0 (ST0083 WP-02, patch-id a2246ee4ecc9); AT-02.1 green (the scope takes the contracts where the key puts them and leaves a copy elsewhere out, in both layouts); AT-02.2 green (the running daemon ingests a .claude/AGENTS.md edit with nobody running sync); AT-02.3 green (the watcher names a .claude/ contract edit through the leaf, .claude/ and root doors and names nothing for a root copy) -- satisfied: yes

### WP-03 -- The doors: init --contracts and intent agents layout (status: Not Started)

- AC-03.1 (non-test) `intent init --contracts dotclaude` creates the project with both contracts under `.claude/` and the key in its config; `intent init` without the flag writes `root` behaviour and no key. -- satisfied: no
- AC-03.2 (non-test) `intent agents layout <root|dotclaude>` sets the key and moves both contracts to the layout named, through git when they are tracked; it refuses by name when the destination already holds a different file, removes a `usage-rules.md` byte-identical to its seed when moving to `dotclaude`, reports an authored one and leaves it, and says what it moved. -- satisfied: no

### WP-04 -- doctor: both layouts present, a held settings file with a dead hook command, an empty or placeholder author (status: Not Started)

- AC-04.1 (non-test) `intent doctor` counts a finding when a contract exists in both layouts, naming both paths and `intent agents layout` as the remedy. -- satisfied: no
- AC-04.2 (non-test) `intent doctor` reports, as an advisory naming the file and the command, a `.claude/settings.json` that canon holds (one without the marker `intent claude hook`) whose hook command names a path that does not exist on this machine. -- satisfied: no
- AC-04.3 (non-test) `intent doctor` reports a config whose `author` is empty or reads `Author Unknown`, naming the field. -- satisfied: no
- AC-04.4 (non-test) `intent doctor` reports, as an advisory naming each file, a `.claude/scripts/*.sh` whose header names Intent and that no hook command in `.claude/settings.json` or `.claude/settings.local.json` references: v2 wrote these copies into every project and v3 reads none of them, so the remedy is `git rm` of the named files. -- satisfied: no

### WP-05 -- The templated settings deny editing the generated AGENTS.md in either layout (status: Not Started)

- AC-05.1 (non-test) `lib/templates/.claude/settings.json` carries `permissions.deny` rules against editing and writing `./AGENTS.md` and `./.claude/AGENTS.md`, carries none against `CLAUDE.md`, and stays byte-identical on every machine. -- satisfied: no

### WP-06 -- intent deinit: strip what Intent wrote from a project being closed (status: Not Started)

- AC-06.1 (non-test) `intent deinit` without `--apply` lists every file and hook block it would remove and writes nothing; with `--apply` it removes the generated contracts in either layout, a `.claude/settings.json` and `.mcp.json` carrying Intent markers, Intent hook chain blocks and carriers, and a `.intent_critic.yml` byte-identical to its seed, naming each, and names each file it leaves because it cannot prove Intent wrote it; `--all` also removes `intent/` with the store. -- satisfied: no

### WP-07 -- A store under Dropbox is marked ignored for the Dropbox client (status: Not Started)

- AC-07.1 (non-test) When the project root is under a Dropbox folder, `intent init` and the first store open mark `intent/.cache` ignored for the Dropbox client present (the File Provider and legacy attributes on macOS, `com.dropbox.ignored` on Linux) and say so once; `intent doctor` reports an unmarked `intent/.cache` under Dropbox as an advisory with the command that marks it. -- satisfied: no

### WP-08 -- Docs, reference and CHANGELOG for the two layouts, the import and deinit (status: Not Started)

- AC-08.1 (non-test) The docs describe the two layouts, the key, the import and `deinit` where the contracts are described today (`intent/docs/working-with-llms.md`, `docs/working-with-agents.md`, `docs/getting-started.md`, `docs/migrating-from-v2.md`); the two skills that state the root layout as fact are corrected (`in-essentials/SKILL.md` rule 2 and `in-standards/SKILL.md` step 1); the reference regenerates for the new verbs and flags; and `CHANGELOG.md` carries the lines under `## [3.2.4] - in progress`. -- satisfied: no

## Acceptance Tests

### WP-01 -- CLAUDE.md imports AGENTS.md: the @AGENTS.md line in _CLAUDE.md (status: Done)

- AT-01.1 `tests/unit/claude_md_template.bats` -- covers AC-01.1 -- status: green -- Arm 14 of tests/unit/claude_md_template.bats. RED at base bed306a63 on 2026-10-15, on 701c4d98b's binary, whose embedded llm/ templates are byte-identical to the base's: 'the generated CLAUDE.md opens with # ScratchProj, not @AGENTS.md'. GREEN on tmp/wt-cc-0083's own release build in the judging run on vc's GO the same day (bats 781 of 781 ok, ok 78), landed at 16f351a90 with the bank's patch-id 271b9eae66ef. AC-01.1's run-time half (claude upgrade --apply writes the line, carries the user block, and holds a footerless CLAUDE.md) is driven evidence on refs/bank/cc/0083/evidence/wp01-d01-drive.log.

### WP-02 -- The contracts key and contracts_dir: every writer, reader, the sync scope and the daemon watch derive the place from it (status: WIP)

- AT-02.1 `native/rust/crates/intentsvcs/tests/contracts_layout.rs` -- covers AC-02.1, AC-02.2, AC-02.3 -- status: green -- RED at base e92c1de20: the arm cannot compile there, because the API it calls does not exist at that commit -- git grep finds no `pub enum Contracts`, no `pub fn root_file_path` and no fallible `Scanned::for_root` at e92c1de20, and one of each at a85dfbdf0. GREEN on tmp/wt-cc-0083's own build in WP-02's judging run on vc's GO, 2026-10-15 11:00Z to 11:08Z, with the workspace at 3164 passed and 0 failed; landed at a85dfbdf0 with the bank's patch-id a2246ee4ecc9. contracts_layout.rs: 9 passed, 0 failed.
- AT-02.2 `native/rust/crates/intentd/tests/daemon_watch.rs` -- covers AC-02.3 -- status: green -- RED at base e92c1de20: the arm cannot compile there, because the API it calls does not exist at that commit -- git grep finds no `pub enum Contracts`, no `pub fn root_file_path` and no fallible `Scanned::for_root` at e92c1de20, and one of each at a85dfbdf0. GREEN on tmp/wt-cc-0083's own build in WP-02's judging run on vc's GO, 2026-10-15 11:00Z to 11:08Z, with the workspace at 3164 passed and 0 failed; landed at a85dfbdf0 with the bank's patch-id a2246ee4ecc9. The live-daemon arm a_dotclaude_contract_edit_reaches_the_store_with_nobody_running_sync passed.
- AT-02.3 `native/rust/crates/intentd/src/watch.rs` -- covers AC-02.3 -- status: green -- RED at base e92c1de20: the arm cannot compile there, because the API it calls does not exist at that commit -- git grep finds no `pub enum Contracts`, no `pub fn root_file_path` and no fallible `Scanned::for_root` at e92c1de20, and one of each at a85dfbdf0. GREEN on tmp/wt-cc-0083's own build in WP-02's judging run on vc's GO, 2026-10-15 11:00Z to 11:08Z, with the workspace at 3164 passed and 0 failed; landed at a85dfbdf0 with the bank's patch-id a2246ee4ecc9. The unit arm a_dotclaude_contract_edit_is_named_and_a_root_copy_edit_is_not passed.

### WP-03 -- The doors: init --contracts and intent agents layout (status: Not Started)

_(no tests in this group)_

### WP-04 -- doctor: both layouts present, a held settings file with a dead hook command, an empty or placeholder author (status: Not Started)

_(no tests in this group)_

### WP-05 -- The templated settings deny editing the generated AGENTS.md in either layout (status: Not Started)

_(no tests in this group)_

### WP-06 -- intent deinit: strip what Intent wrote from a project being closed (status: Not Started)

_(no tests in this group)_

### WP-07 -- A store under Dropbox is marked ignored for the Dropbox client (status: Not Started)

_(no tests in this group)_

### WP-08 -- Docs, reference and CHANGELOG for the two layouts, the import and deinit (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
