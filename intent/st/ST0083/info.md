---
st_id: ST0083
title: Agent contracts at the root or in .claude/, chosen per project, and CLAUDE.md imports AGENTS.md
status: WIP
created: 2026-10-15
completed:
---

# ST0083: Agent contracts at the root or in .claude/, chosen per project, and CLAUDE.md imports AGENTS.md

## Objective

Every Intent project's generated `CLAUDE.md` imports `AGENTS.md` with an `@AGENTS.md` line, so Claude Code loads the tool-agnostic contract at launch. Today it does not: under Claude Code's default, a project that has any `CLAUDE.md` loads that file only and never reads `AGENTS.md`, and Intent's template carried the pointer as prose, which is a request the harness ignores.

A project declares where its two agent contracts live, through one key in `intent/.config/config.json`: `contracts: root`, the default, for a code repository, where `AGENTS.md` at the root is what the agents.md cross-tool spec's readers (Codex, Cursor, Copilot, Aider) open; or `contracts: dotclaude`, for a content tree, a book or an estate whose root is read by people who are not engineers and whose only agent is Claude Code, which reads `.claude/AGENTS.md` and `.claude/CLAUDE.md` natively. `intent init`, `intent agents init|sync|validate`, `intent claude upgrade --apply`, `intent doctor`, the sync scope and the daemon's watch all derive the place from that key, and nothing recreates a root copy under `dotclaude`. `intent init --contracts` sets it at birth; `intent agents layout` moves an existing project.

With it, in the same release: `doctor` reports both layouts present, a held `.claude/settings.json` whose hook command names a path that does not exist, and an empty or placeholder author; the templated `.claude/settings.json` denies editing the generated `AGENTS.md` in either layout; `intent deinit` strips what Intent wrote from a project being closed; and a store created under a Dropbox folder is marked ignored for the Dropbox client, so a family-synced tree never syncs a live SQLite WAL. Everything here ships in 3.2.4.

## Context

## Where this came from

On 2026-10-15 hv moved the agent contracts of two shared Dropbox estates and of a book inside one of them from the estate root into `.claude/`, because family members who sync those trees should not see or touch tooling files. Another estate's validation session then briefed Intent, at hv's request, with the asks its audit produced, and hv ruled, first relayed and then first-hand in vc's session: the two layouts are the requirement, not a proposal; Intent knows which a project is from a per-project key; init, agents sync, agents validate and doctor honour it; the `@AGENTS.md` import applies to both; and the changes go into 3.2.4. The brief, measured line by line against this tree at c5f6f646e and against the Claude Code docs read the same day, with every `file:line`, is banked as a blob on `refs/bank/vc/contracts/contracts-layout-brief-measured.md` (sha256 7363a9f5a607); read it before touching any site below.

## What is measured (3.2.3 dev source at c5f6f646e)

Every writer and reader of the two contracts joins the project root by a literal name and no key exists: `rootfiles.rs:291` (the `agents sync`/`init` writer), `rootfiles.rs:582` (`validate`), `canon.rs:709` (`claude upgrade --apply`), `init.rs:84` and `:120`, `sync.rs:36` and `:594` (`ROOT_FILES`, the sync and ingest scope), `intentd/src/watch.rs:177` (the root watch), `doctor.rs:2073` (`root_file_findings`, which inherits the root from `canon::apply` in report mode), and the `agents` messages at `render.rs:13683`, `:13708`, `:13725`, `:13731`. `Config` flattens unknown keys into `extra`, so an older binary carries the new key byte for byte. `lib/templates/llm/_CLAUDE.md` has no `@` import. `lib/templates/.claude/settings.json` is byte-identical on every machine and is not substituted (issue 0016). `doctor` reports a root file only when canon would write it, so a held `.claude/settings.json` (one lacking the marker `intent claude hook`, the v2.12 absolute-path form) is never reported. Nothing in Intent names Dropbox; the store is `intent/.cache/intent.db` (project.rs:1847) and its open does `create_dir_all` (store.rs:3636). No v3 store exists under Dropbox on hv's machine yet; the first will be the book.

Claude Code's memory doc, verbatim: "By default, Claude reads `AGENTS.md` only when you have no `CLAUDE.md` in your working directory or above it"; with a `CLAUDE.md`, `.claude/CLAUDE.md` or `CLAUDE.local.md` present Claude reads "Your `CLAUDE.md` files only"; a `CLAUDE.md` that imports `AGENTS.md` gets both; imports resolve relative to the importing file, four hops max; an imported `AGENTS.md` is never read twice; `.claude/AGENTS.md` and `.claude/CLAUDE.md` are locations Claude Code reads itself. Dropbox's page: File Provider macOS `xattr -w 'com.apple.fileprovider.ignore#P' 1 <dir>`, legacy macOS `xattr -w com.dropbox.ignored 1 <dir>`, Linux `attr -s com.dropbox.ignored -V 1 <dir>`; an ignored item is deleted from the server and other devices, which is right for a store that rebuilds from committed canon as a fresh clone does.

## hv's rulings (2026-10-15, hv's "Ok, proceed" on vc's nine recommendations; wording vc's, authority hv's)

1. The signal is an explicit key, never `languages: []`: an empty `languages` is a legitimate state of a code repository (a fresh `init`, or a language Intent has no pack for), and inferring a layout from it would move a code repository's `AGENTS.md` silently.
2. The book in hv's shared estate is `dotclaude`; its root is a manuscript to the people who open it.
3. The import lands first, as its own XS bank, before 3.2.4's cut: it is a defect in every project's contract today.
4. Everything in this thread ships in 3.2.4.
5. `intent deinit` is in this thread.
6. The templated settings deny editing `AGENTS.md`, both paths, never `CLAUDE.md` (whose user block between `<!-- user:start -->` and `<!-- user:end -->` is a sanctioned edit surface).
7. Another estate's v2.12 project rides the fleet sweep under hv decision 31; its two markers are read before any `--force`.
8. Intent marks `intent/.cache` Dropbox-ignored itself when a project sits under a Dropbox root.
9. Under `dotclaude`, `usage-rules.md` is not seeded (a visible tooling file at a manuscript root with no reader there); an authored one is left alone and reported.

## Design constraints that are decisions, not options

- The key is an enum of two values, not a path: both are locations Claude Code reads natively, and a free path would admit a place no tool reads. Absent means `root`; the default is never written back (the `index`/`whiteboard` block pattern), so no existing config changes a byte.
- One function answers where the contracts live, and every site above derives from it, the sync scope and the daemon watch included. `usage-rules.md` stays at the root under both layouts; `.mcp.json`, `.intent_critic.yml` and `.claude/settings.json` do not move.
- The import line is `@AGENTS.md` in both layouts, because the two files always sit together; nothing is path-aware.
- `lib/templates/` is live on save in every estate on hv's machine (hv decision 30), so WP-01 and WP-05 are built in a detached worktree and judged before the save reaches the main tree.
- `agents layout` moves through git when the files are tracked, refuses when the destination holds a different file, and removes a `usage-rules.md` byte-identical to its seed.
- `deinit` writes nothing without `--apply`, removes only what it can prove Intent wrote (markers, byte-identity to a seed) and names what it leaves.
- This repository is public: nothing committed names a client, a client's estate or its people; hv's own estates are named as before.

## The book, as the first `dotclaude` project

hv's word, first-hand: the book is not upgraded from v2.14; its Intent content is deleted and the project is recreated with `intent init --contracts dotclaude` on the pair that carries the key, with `intent/.cache` marked Dropbox-ignored before the store exists. The steps are in the banked write-up. That run is this thread's live evidence for AC-02.2 and AC-03.1, executed by the estate's own validation session.

## Work Packages

| WP    | Title                                                                                                                   | Size | Status      |
| ----- | ----------------------------------------------------------------------------------------------------------------------- | ---- | ----------- |
| WP-01 | CLAUDE.md imports AGENTS.md: the @AGENTS.md line in _CLAUDE.md                                                          | XS   | Done        |
| WP-02 | The contracts key and contracts_dir: every writer, reader, the sync scope and the daemon watch derive the place from it | M    | Done        |
| WP-03 | The doors: init --contracts and intent agents layout                                                                    | S    | Done        |
| WP-04 | doctor: both layouts present, a held settings file with a dead hook command, an empty or placeholder author             | S    | Not Started |
| WP-05 | The templated settings deny editing the generated AGENTS.md in either layout                                            | XS   | Done        |
| WP-06 | intent deinit: strip what Intent wrote from a project being closed                                                      | S    | Not Started |
| WP-07 | A store under Dropbox is marked ignored for the Dropbox client                                                          | S    | Done        |
| WP-08 | Docs, reference and CHANGELOG for the two layouts, the import and deinit                                                | S    | Not Started |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
