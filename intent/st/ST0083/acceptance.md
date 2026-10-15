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

### WP-01 -- CLAUDE.md imports AGENTS.md: the @AGENTS.md line in _CLAUDE.md (status: Not Started)

- AC-01.1 (non-test) The generated CLAUDE.md opens with the line `@AGENTS.md`, rendered from `lib/templates/llm/_CLAUDE.md`, so Claude Code loads AGENTS.md at launch in both layouts; `intent claude upgrade --apply` writes the line into an existing generated CLAUDE.md with its user block carried, and a CLAUDE.md without the generated footer is held as before. -- satisfied: no

### WP-02 -- The contracts key and contracts_dir: every writer, reader, the sync scope and the daemon watch derive the place from it (status: Not Started)

- AC-02.1 (non-test) `intent/.config/config.json` accepts `"contracts": "root"` or `"contracts": "dotclaude"`; an absent key reads as `root` and is never written back, and any other value is refused by name with the two values it accepts. -- satisfied: no
- AC-02.2 (non-test) Under `dotclaude`, `intent init`, `intent agents init`, `intent agents sync`, `intent agents validate` and `intent claude upgrade --apply` read and write `.claude/AGENTS.md` and `.claude/CLAUDE.md` and never a root copy, and `usage-rules.md` is not seeded; under `root` every one of them behaves as in 3.2.3, and `usage-rules.md` stays at the root in both. -- satisfied: no
- AC-02.3 (non-test) The sync scope follows the key: in a `dotclaude` project an edit to `.claude/AGENTS.md` or `.claude/CLAUDE.md` is ingested by `intent sync --to-store` and by the running daemon exactly as an edit to the root copy is in a `root` project, and a root copy in a `dotclaude` project is not in scope. -- satisfied: no

### WP-03 -- The doors: init --contracts and intent agents layout (status: Not Started)

- AC-03.1 (non-test) `intent init --contracts dotclaude` creates the project with both contracts under `.claude/` and the key in its config; `intent init` without the flag writes `root` behaviour and no key. -- satisfied: no
- AC-03.2 (non-test) `intent agents layout <root|dotclaude>` sets the key and moves both contracts to the layout named, through git when they are tracked; it refuses by name when the destination already holds a different file, removes a `usage-rules.md` byte-identical to its seed when moving to `dotclaude`, reports an authored one and leaves it, and says what it moved. -- satisfied: no

### WP-04 -- doctor: both layouts present, a held settings file with a dead hook command, an empty or placeholder author (status: Not Started)

- AC-04.1 (non-test) `intent doctor` counts a finding when a contract exists in both layouts, naming both paths and `intent agents layout` as the remedy. -- satisfied: no
- AC-04.2 (non-test) `intent doctor` reports, as an advisory naming the file and the command, a `.claude/settings.json` that canon holds (one without the marker `intent claude hook`) whose hook command names a path that does not exist on this machine. -- satisfied: no
- AC-04.3 (non-test) `intent doctor` reports a config whose `author` is empty or reads `Author Unknown`, naming the field. -- satisfied: no

### WP-05 -- The templated settings deny editing the generated AGENTS.md in either layout (status: Not Started)

- AC-05.1 (non-test) `lib/templates/.claude/settings.json` carries `permissions.deny` rules against editing and writing `./AGENTS.md` and `./.claude/AGENTS.md`, carries none against `CLAUDE.md`, and stays byte-identical on every machine. -- satisfied: no

### WP-06 -- intent deinit: strip what Intent wrote from a project being closed (status: Not Started)

- AC-06.1 (non-test) `intent deinit` without `--apply` lists every file and hook block it would remove and writes nothing; with `--apply` it removes the generated contracts in either layout, a `.claude/settings.json` and `.mcp.json` carrying Intent markers, Intent hook chain blocks and carriers, and a `.intent_critic.yml` byte-identical to its seed, naming each, and names each file it leaves because it cannot prove Intent wrote it; `--all` also removes `intent/` with the store. -- satisfied: no

### WP-07 -- A store under Dropbox is marked ignored for the Dropbox client (status: Not Started)

- AC-07.1 (non-test) When the project root is under a Dropbox folder, `intent init` and the first store open mark `intent/.cache` ignored for the Dropbox client present (the File Provider and legacy attributes on macOS, `com.dropbox.ignored` on Linux) and say so once; `intent doctor` reports an unmarked `intent/.cache` under Dropbox as an advisory with the command that marks it. -- satisfied: no

### WP-08 -- Docs, reference and CHANGELOG for the two layouts, the import and deinit (status: Not Started)

- AC-08.1 (non-test) The docs describe the two layouts, the key, the import and `deinit` where the contracts are described today (`intent/docs/working-with-llms.md`, `docs/working-with-agents.md`, `docs/getting-started.md`, `docs/migrating-from-v2.md`), the reference regenerates for the new verbs and flags, and `CHANGELOG.md` carries the lines under `## [3.2.4] - in progress`. -- satisfied: no

## Acceptance Tests

### WP-01 -- CLAUDE.md imports AGENTS.md: the @AGENTS.md line in _CLAUDE.md (status: Not Started)

_(no tests in this group)_

### WP-02 -- The contracts key and contracts_dir: every writer, reader, the sync scope and the daemon watch derive the place from it (status: Not Started)

_(no tests in this group)_

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
