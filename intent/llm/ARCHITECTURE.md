# ARCHITECTURE.md

System architecture for Intent v3. Pair this with `intent/llm/MODULES.md` (the Highlander registry) and `intent/llm/DECISION_TREE.md` (code-placement flow chart).

## System overview

Intent manages software projects through steel threads -- coherent, finite slices of work -- with work packages, acceptance criteria and tests, and issues, and ships a canon of LLM-collaboration artefacts (`AGENTS.md`, `CLAUDE.md`, `usage-rules.md`, `.claude/settings.json`, the pre-commit critic gate).

v3 is a Rust workspace at `native/rust/`. The SQLite store (`intent/.cache/intent.db`) is the durable source of truth; `intent/.canon/` is its committed extract and the Markdown under `intent/st/` is generated from it. Every face -- the CLI, MCP (`intent mcp`), GraphQL (`intent graphql`), the TUI (`intent explore`) -- calls one facade in `intentsvcs`. The v2 Bash CLI is deleted; `bin/` holds only the dev tooling `devbin` and `int`.

## Core layout

| Path                             | Purpose                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `native/rust/crates/intentsvcs/` | The library, and the only crate that touches the store or the file canon: model, SQLite store, ingest, views, sync, canon installer (`canon.rs`), root-file rendering (`rootfiles.rs`), `init.rs`, rule library (`rules.rs`), headless critic (`critic.rs`), skills and subagents payload (`payload.rs`), and the facade every face calls (`facade.rs`). |
| `native/rust/crates/intent-cli/` | The CLI face: `dispatch.rs` loads the verb table, `spine.rs` builds clap from it, `render.rs` renders each verb; also the MCP face (`intent mcp`) and the TUI (`intent explore`).                                                                                                                                                                        |
| `native/rust/crates/intentd/`    | The per-machine daemon (`intent daemon`); `--daemon` on a verb asks it instead of the local process.                                                                                                                                                                                                                                                     |
| `native/rust/crates/testkit/`    | Test-only helpers shared across the workspace's integration tests.                                                                                                                                                                                                                                                                                       |
| `native/rust/build-support/`     | Build scripts; `embed_templates.rs` compiles `lib/templates/{llm,prj}` into the binary for `intent init`.                                                                                                                                                                                                                                                |
| `surface/dispatch-table.json`    | The authored command surface; the CLI, the MCP tool list and the `intent llm` guide render from it.                                                                                                                                                                                                                                                      |
| `schema/`                        | Faces generated from the Rust types (DDL, JSON Schemas, GraphQL SDL).                                                                                                                                                                                                                                                                                    |
| `lib/templates/`                 | Template source: `llm/` and `prj/` are embedded for `init`; `.claude/` and `hooks/` are read at runtime from the install.                                                                                                                                                                                                                                |
| `intent/plugins/claude/`         | Canon payload served from the install: `skills/`, `subagents/`, and the rule library `rules/` (`rules/agnostic/`, `rules/<lang>/`).                                                                                                                                                                                                                      |
| `intent/.canon/`                 | The committed extract of the store (`st/`, `issues/`, `project.json`).                                                                                                                                                                                                                                                                                   |
| `intent/.cache/intent.db`        | The SQLite store: gitignored, and the durable SSOT.                                                                                                                                                                                                                                                                                                      |
| `intent/.config/config.json`     | Project configuration (`intent_version`, `project_name`, `author`, `languages`, ...).                                                                                                                                                                                                                                                                    |
| `intent/st/`                     | Steel-thread files realised for the threads `intent/.intentfiles` declares (`intent st hydrate` / `dehydrate`).                                                                                                                                                                                                                                          |
| `intent/llm/`                    | LLM-facing project documents: `MODULES.md`, `DECISION_TREE.md`, `RULES.md`, `ARCHITECTURE.md`. Read by every LLM session.                                                                                                                                                                                                                                |
| `tests/`                         | BATS suite for the shell assets (`tests/lib/test_helper.bash` provides shared fixtures); Rust tests live beside each crate.                                                                                                                                                                                                                              |

## Key patterns

### Thin coordinator dispatch

`intent-cli` parses, calls, renders: `dispatch.rs` loads the verb table from `surface/dispatch-table.json`, `spine.rs` builds the clap surface from it, and `render.rs` matches each family to a function that calls `intentsvcs` and prints the result; a family with no arm refuses at exit 2 as not implemented. Behaviour lives in `intentsvcs`, and `rusqlite` appears only in that crate (asserted by `intentsvcs/tests/dep_graph_guard.rs`). This is `IN-AG-THIN-COORD-001` (Thin Coordinator) concretised.

### Single template source

All generated content -- AGENTS.md, CLAUDE.md, settings.json, hook scripts -- comes from `lib/templates/`: `init` embeds `llm/` and `prj/` at build time, and `canon.rs` / `rootfiles.rs` read the rest from the install at runtime, expanding placeholders in Rust. There are no parallel copies of template content. This is `IN-AG-HIGHLANDER-001` (Highlander) concretised: one source, many use sites.

### Canon-apply via the canon-installer

`intent claude upgrade` reports what canon would change; `--apply` writes it (`--force` overwrites user-edited canon, `--skip-settings` leaves `.claude/settings.json` alone). The artefacts it owns: `.claude/settings.json`, `CLAUDE.md` / `AGENTS.md` (rendered from `lib/templates/llm/_*`), `usage-rules.md` and `.intent_critic.yml` (seeded only when absent), and the `.git/hooks/pre-commit` chain block (`canon.rs`).

Auto-insert mechanisms (such as the pre-commit chain block) use marker pairs (`intent-chain-block:start` / `:end`) so re-application detects "already done" and skips.

### Steel thread lifecycle

`intent st new "Title"` creates a thread in the store. Statuses are `ThreadStatus` (`intentsvcs/src/model.rs`): Triage, NotStarted, Wip, Hold, Completed, Cancelled; `intent st start|done|cancel|triage|hold|resume|reopen|reinstate` move between them. The CLI is the single mutator -- direct file edits to `status:` are forbidden by `IN-AG-HIGHLANDER-001`. A status change moves no directories: a thread's files under `intent/st/<id>/` exist while `intent/.intentfiles` declares it (`intent st hydrate` / `dehydrate`). Work packages follow the same pattern via `intent wp new|start|done|...`.

## Hook architecture (Claude Code session lifecycle)

`.claude/settings.json` wires Claude Code lifecycle hooks shipped with the canon, each dispatching `intent claude hook <name>`, which execs `<install>/lib/templates/.claude/scripts/<name>.sh` (`install.rs`):

- **SessionStart** (`session-context`) -- prints project context, branch and active steel thread.
- **UserPromptSubmit** (`require-in-session.sh`) -- strict gate: blocks the first prompt until `/in-session` has run. Released cooperatively when the skill writes `/tmp/intent/in-session-<UUID>.sentinel`.
- **Stop** (`session-finish`) -- prints the `/in-finish` reminder at session wrap.

This pattern means every coding session starts with `/in-session`, which loads the right language skills and Highlander/PFIC/Thin-Coordinator/No-Silent-Errors discipline before any code is written.

## Critic dispatch

`intent critic <lang>` is the headless runner (`intentsvcs::critic`, no LLM); `intent critic --languages` lists the languages it serves. Exit codes: 0 clean, 1 findings, 2 invocation error (the gate fails open), 3 an armed rule's tool is absent (blocks). The LLM critics are subagents (`critic-<lang>`, `critic-prose`), invoked as `Task(subagent_type="critic-<lang>", prompt="review <paths>")`; each reads the rule library at invocation and applies the Detection heuristics.

The pre-commit gate is `lib/templates/hooks/pre-commit.sh`, run live from the Intent install and never copied into a project: `intent claude upgrade --apply` writes the shim `lib/templates/hooks/pre-commit-shim.sh` to `pre-commit.intent` in the git hooks directory and inserts a chain block into that directory's `pre-commit`, and the shim reads the install root from `~/.intent/home` and execs the gate (`intentsvcs/src/canon.rs` `install_carrier`). The gate `intent critic <lang> --staged` per declared language. Severity threshold reads from `.intent_critic.yml`.

## Migration history

Per-version detail is in `CHANGELOG.md`. Completed steel threads are listed by `intent st list --status all` and read with `intent st show <id>`.

## Where to read next

- New module? `intent/llm/MODULES.md` first; `intent/llm/DECISION_TREE.md` for placement.
- New rule? `intent/docs/rules.md` for the schema and Detection contract.
- New plugin or extension? `intent/docs/writing-extensions.md`.
- New ST or WP? Use the CLI (`intent st new`, `intent wp new`); do not hand-roll directories.
