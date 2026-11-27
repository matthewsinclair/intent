# ARCHITECTURE.md

System architecture for Intent v3. Pair this with `intent/llm/MODULES.md` (the Highlander registry) and `intent/llm/DECISION_TREE.md` (where code goes).

## System overview

Intent manages software projects through steel threads, with work packages, acceptance criteria and tests, and issues. It also ships a canon of LLM-collaboration artefacts: `AGENTS.md`, `CLAUDE.md`, `usage-rules.md`, `.claude/settings.json` and the pre-commit critic gate.

v3 is a Rust workspace at `native/rust/`. The SQLite store (`intent/.cache/intent.db`) is the durable source of truth. `intent/.canon/` is its committed extract, and the Markdown under `intent/st/` is generated from it. Every face calls one facade in `intentsvcs`: the CLI, MCP (`intent mcp`), GraphQL (`intent graphql`) and the TUI (`intent explore`). `bin/` holds only the dev tooling, `devbin` and `int`.

## Core layout

| Path                             | Purpose                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `native/rust/crates/intentsvcs/` | The library, and the only crate that touches the store or the file canon: model, SQLite store, ingest, views, sync, canon installer (`canon.rs`), root-file rendering (`rootfiles.rs`), `init.rs`, rule library (`rules.rs`), headless critic (`critic.rs`), skills and subagents payload (`payload.rs`), and the facade every face calls (`facade.rs`). |
| `native/rust/crates/intent-cli/` | The CLI face: `dispatch.rs` loads the verb table, `spine.rs` builds clap from it, and `render.rs` renders each verb. Also the MCP face (`intent mcp`) and the TUI (`intent explore`).                                                                                                                                                                    |
| `native/rust/crates/intentd/`    | The per-machine daemon (`intent daemon`). `--daemon` on a verb asks it instead of the local process.                                                                                                                                                                                                                                                     |
| `native/rust/crates/testkit/`    | Test-only helpers shared across the workspace's integration tests.                                                                                                                                                                                                                                                                                       |
| `native/rust/build-support/`     | Build scripts. `embed_templates.rs` compiles `lib/templates/{llm,prj}` into the binary for `intent init`.                                                                                                                                                                                                                                                |
| `surface/dispatch-table.json`    | The authored command surface. The CLI, the MCP tool list and `intent llm guide` render from it.                                                                                                                                                                                                                                                          |
| `schema/`                        | Faces generated from the Rust types (DDL, JSON Schemas, GraphQL SDL).                                                                                                                                                                                                                                                                                    |
| `lib/templates/`                 | Template source. `llm/` and `prj/` are also embedded in the binary for `init`. At runtime `claude upgrade`, `agents sync` and the hooks read every template, `llm/` included, from the install.                                                                                                                                                          |
| `intent/plugins/claude/`         | Canon payload served from the install: `skills/`, `subagents/`, `teams/` and the rule library `rules/` (`rules/agnostic/`, `rules/<lang>/`).                                                                                                                                                                                                             |
| `intent/.canon/`                 | The committed extract of the store (`st/`, `records/`, `issues/`, `events/`, `project.json`).                                                                                                                                                                                                                                                            |
| `intent/.cache/intent.db`        | The SQLite store: gitignored, and the durable SSOT.                                                                                                                                                                                                                                                                                                      |
| `intent/.config/config.json`     | Project configuration (`intent_version`, `project_name`, `author`, `languages`, ...).                                                                                                                                                                                                                                                                    |
| `intent/st/`                     | The files of each realised thread (`intent st hydrate` / `dehydrate`).                                                                                                                                                                                                                                                                                   |
| `intent/llm/`                    | LLM-facing project documents: `MODULES.md` (searched with `intent modules find <name>`, never read whole), `DECISION_TREE.md`, `RULES.md`, `ARCHITECTURE.md`. Read on demand. Nothing loads them into a session.                                                                                                                                         |
| `native/macos/`                  | The macOS menubar app (a Swift / Xcode project). `intent app` starts, stops and reports it (`intentsvcs::macapp`).                                                                                                                                                                                                                                       |
| `tests/`                         | BATS suite for the shell assets (`tests/lib/test_helper.bash` provides shared fixtures). Rust tests live beside each crate.                                                                                                                                                                                                                              |

## Key patterns

### Thin coordinator dispatch

`intent-cli` parses, calls and renders. `dispatch.rs` loads the verb table from `surface/dispatch-table.json`, `spine.rs` builds the clap surface from it, and `render.rs` matches each family to a function that calls `intentsvcs` and prints the result. A family with no arm refuses at exit 2 as not implemented. Behaviour lives in `intentsvcs`, and `rusqlite` appears only in that crate (`intentsvcs/tests/dep_graph_guard.rs` asserts it). This is `IN-AG-THIN-COORD-001` concretised.

### Single template source

All generated content (AGENTS.md, CLAUDE.md, settings.json, hook scripts) comes from `lib/templates/`. `init` renders from the copies of `llm/` and `prj/` embedded at build time. `canon.rs` and `rootfiles.rs` read templates, `llm/` included, from the install at runtime, and expand placeholders in Rust. There are no parallel copies of template content. This is `IN-AG-HIGHLANDER-001` concretised.

### Canon-apply

`intent claude upgrade` reports what canon would change, and `--apply` writes it. `--force` overwrites user-edited canon: `CLAUDE.md` without its marker, `.claude/settings.json` without Intent's hook door, and `.intent_critic.yml`. `--skip-settings` leaves `.claude/settings.json` and `.mcp.json` alone. The artefacts it owns (`canon.rs`):

- `.claude/settings.json`, and `CLAUDE.md` / `AGENTS.md` rendered from `lib/templates/llm/_*`.
- `usage-rules.md`, `.mcp.json` and `.intent_critic.yml`, seeded only when absent.
- In the hooks directory that `git rev-parse --git-path hooks` names: the `pre-commit.intent` carrier with the `pre-commit` chain block, and the `post-merge`, `post-checkout` and `post-rewrite` carriers with theirs.

A chain block sits between the markers `# intent-chain-block:start` and `# intent-chain-block:end`. Re-application rewrites only the lines between them to canon's block, writes nothing when the block is already canon's, and passes every line outside the markers through byte for byte. A hook whose block is doubled, unclosed or carries the retired marker is held and named.

### Steel thread lifecycle

`intent st new "Title"` creates a thread in the store. The statuses are `ThreadStatus` (`intentsvcs/src/model.rs`): Triage, NotStarted, Wip, Hold, Completed, Cancelled. `intent st start|done|cancel|triage|hold|resume|reopen|reinstate` move a thread between them, and the CLI is the only mutator. A status change moves no directories: a thread's files under `intent/st/<id>/` are on disk while the thread is realised (`intent st hydrate` / `dehydrate`). Work packages follow the same pattern through `intent wp new|start|done|...`.

## Session hooks

`.claude/settings.json` wires three Claude Code lifecycle hooks. Each runs `intent claude hook <name>`, which execs `<install>/lib/templates/.claude/scripts/<name>.sh` (`install.rs`):

- **SessionStart** (`session-context`): prints project context, branch and active steel thread.
- **UserPromptSubmit** (`require-in-session`): a strict gate that blocks the first prompt until `/in-session` has run. The skill releases it by writing `/tmp/intent/in-session-<UUID>.sentinel`. On a prompt that passes, it also prints what `intent estate slots pending` prints.
- **Stop** (`session-finish`): names the uncommitted paths at the end of a turn, and gives no instruction.

## Critics and the pre-commit gate

`intent critic <lang>` is the headless runner (`intentsvcs::critic`, no LLM). `intent critic --languages` lists the languages it serves, and `intent llm guide` gives its exit codes. The LLM critics are subagents (`critic-<lang>`, `critic-prose`). Each reads the rule library when invoked and applies the Detection heuristics.

The pre-commit gate is `lib/templates/hooks/pre-commit.sh`, run live from the Intent install and never copied into a project. `intent claude upgrade --apply` writes the shim `lib/templates/hooks/pre-commit-shim.sh` to `pre-commit.intent` in the git hooks directory, and inserts a chain block into that directory's `pre-commit` (`canon.rs`, `install_carrier`). The shim reads the install root from `~/.local/share/intent/home` and execs the gate. The gate runs, in order:

1. The guard roster, `lib/templates/hooks/pre-commit-guards.sh`, read from this checkout when it is an Intent source tree and from the install otherwise.
2. `intent critic <lang> --staged --severity-min <threshold>` for each declared language.
3. `intent doctor`.

It blocks on a failing guard, on critic exit 1 or 3 and on doctor exit 1, and fails open on any other code. The threshold is `.intent_critic.yml`'s `severity_min`: `warning` by default, and `style` when only `show_all: true` is set.

## Where to read next

- New module? `intent/llm/MODULES.md` first, then `intent/llm/DECISION_TREE.md` for placement.
- New rule? `intent/docs/rules.md` for the schema and Detection contract.
- New plugin or extension? `intent/docs/writing-extensions.md`.
- New ST or WP? Use the CLI (`intent st new`, `intent wp new`).
