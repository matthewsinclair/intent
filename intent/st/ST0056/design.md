# Design - ST0056: Intent v3.0.0 -- Rust CLI + SQLite + bidirectional .md sync + MCP

## Problem

Intent v2 is 27 shell files, 12,492 lines (measured 2026-08-14 at `9cdebe0`), on a hostile platform (bash 3.2, no `declare -A`, `set -e` arithmetic traps, BSD/GNU sed splits). The deeper defect is structural: every v2 reader reimplements parsing, so every reader answers confidently from partial evidence. The v2.19.0 release closed five instances of that one class (0011, 0017, 0019, 0020, 0022), and each fix added another grep. v2 also bolted schema onto markdown three separate times -- the 0012 header-block ruling, the 0017 AT row grammar, the close-gate contract -- each a bespoke mini-standard needing its own lint, its own `--fix`, and its own weeks of drift cost.

v3 reifies the data model: one schema, one parser, one query surface. The bug class becomes unconstructible, and nobody designs a row grammar again.

## Architecture

### The layering contract

```
in-process:  intent (cli) ──────────────▶ intentsvcs ──▶ sqlite + files
remote:      intent (cli) ──▶ graphql ──▶ intentd ──▶ intentsvcs ──▶ sqlite + files
```

- **`intentsvcs` is the Highlander layer**: all of Intent's functionality, sole owner of BOTH the DB and the file canon (it writes structured files and regenerates views; the CLI and daemon never touch disk or DB).
- The GraphQL layer is a mechanically thin 1:1 skin over the intentsvcs facade; the clap layer is another. Thin Coordinator holds at every seam.
- **The DB-access rule is a mechanical guard, not a convention**: `rusqlite` appears in exactly one `Cargo.toml` (intentsvcs). The dependency graph is the enforcement, asserted in CI.
- **Dual-path conformance suite**: the standing guard against the two entry skins drifting -- the same operation run through in-process and through a live intentd must produce identical results. Every verb lands with one.
- CLI routing rule: if the intentd socket exists and answers, the CLI MUST route to it (never two sync engines live at once); when absent, it executes in-process against the same facade.

### Truth model

- **The schema is the source of truth for structure.** Reified, versioned, validating everything.
- **Durable truth** = committed, schema-validated structured files (JSON) in git. Git remains the merge, history, and collaboration substrate.
- **Runtime truth** = the per-project SQLite DB (`intent/.cache/intent.db`, gitignored per the 0018 treeindex precedent), rebuilt from the committed canon at any time. `rm intent.db` is always safe; consequently there are NO DB migrations, ever -- a schema bump deletes and rebuilds.
- **Markdown is a realisation**: generated views for structure, the authoring surface for prose. Always present on disk.
- **The authored-once principle**: every fact has exactly one authored representation; everything else is generated from it and marked as such. Structure is authored via mutations (CLI/MCP) and serialised to committed JSON; prose is authored in .md bodies; views are generated.
- **No mixed files**: a file is either 100% authored or 100% generated. Mixed files resurrect region-markers and partial-parse drift.

### The reified model: one master, three generated faces

The model is authored once, in the intentsvcs Rust type layer, and generates three committed, CI-drift-checked artefacts:

| Face        | Generated via               | Consumed by                                                  |
| ----------- | --------------------------- | ------------------------------------------------------------ |
| JSON Schema | schemars from the types     | validation of every structured file; Elixir/external clients |
| SQL DDL     | migration-free schema apply | the runtime DB (delete + rebuild on schema bump)             |
| GraphQL SDL | async-graphql export        | CLI/MCP/TUI; Elixir codegen                                  |

`intent schema` prints them. Header blocks are line-oriented `key: value` everywhere -- the 0012 not-YAML ruling generalised; never YAML, anywhere.

### File layout (per artefact)

| Artefact                   | Canonical structure (committed)                                       | Authored prose                                  | Generated views                                                                     |
| -------------------------- | --------------------------------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------- |
| Steel thread + WPs + AC/AT | `st/<ID>/thread.json` (metadata, status, WP records, full contract)   | `design.md`, `impl.md`, objective/context prose | `info.md` (cover: status, dates, rollup), `acceptance.md` (contract + coverage)     |
| Issues                     | `issues/<n>.json` (number, slug, status, priority)                    | `issues/<n>.md` (body + resolution)             | index views; OPEN/CLOSED-as-directories likely retires (status is data)             |
| Project                    | `intent/.config/config.json` (as today)                               | --                                              | --                                                                                  |
| Indexes                    | --                                                                    | --                                              | `steel_threads.md`, `todo.md`, `AGENTS.md` (the proven generated-committed pattern) |
| Tracking                   | --                                                                    | `wip.md`, `restart.md`                          | --                                                                                  |
| Whiteboard                 | deferred to the 3.2 agent-bus ST; md-authored through 3.0.0/3.1       | boards, inboxes                                 | --                                                                                  |
| Runtime DB                 | `intent/.cache/intent.db` -- gitignored, rebuilt from committed canon | --                                              | --                                                                                  |

Format: JSON (not YAML) for canonical structured files -- the 0012 quoting-hazard scar, plus the `config.json` in-house precedent. Tool-written pretty-JSON: stable key order, 2-space indent, trailing newline. YAML/md/anything else are `intent export --format` projections (trivial via serde).

Generated views are committed, with a `doctor`/CI skew check (regenerate, require empty diff) so a hand-edited view is caught, never silently outvoted.

### Ingest, views and sync

- **Down-sync (truth -> views)** is the standard write path: mutation -> validate against schema -> write structured file + regenerate affected views + update DB, atomically. Deterministic and idempotent: same data -> same bytes.
- **Up-sync (files -> DB)** is ingest, and it is **strict**: standard JSON parse + schema validation; invalid input is refused with the finding named. No tolerance ladder for current-version data -- lenience was only ever a coping strategy for having no schema. Prose ingests verbatim (FTS-indexed).
- **Change detection**: git-index-style stat scan (mtime/size, SHA-256 rehash on change -- the conflabd `db_sync.rs` pattern) on every CLI invocation in daemonless mode; debounced fswatch in daemon mode. Scope: `intent/**` + named root files only.
- **`intent ingest --from-md`** exists as an explicit recovery path (and IS the v2 migrator); it is not the daily flow.
- A file that stops parsing (eg merge-conflict markers) enters a named unparsed state surfaced by `doctor`; commands needing it refuse with the finding named. v2 greps through conflict markers silently; v3 refuses.
- FTS5 across all bodies powers `intent search` from CLI and MCP.

### intentd

- **One intentd per machine, N projects**: a project registry (canonicalised root paths, registered on first contact), per-project DBs opened by the daemon, every operation bound to a project context at connection time. Moved/deleted roots surface in `doctor`, not crashes.
- Unix-socket GraphQL (thin skin over the facade); a separate **mgmt plane** (status, shutdown, reload, registry ops -- the conflabd `mgmt/` split) for `intent daemon status/stop`.
- **CLI owns the lifecycle via launchd** (conflabd pattern): LaunchAgent plist under `~/Library/LaunchAgents/`, logs under `~/.local/share/intent/`, binary resolution PATH-first-then-sibling-of-current-exe. PID file with observable (never silent) cleanup. GIT_HASH baked into the version string.
- **Policy-stamp self-healing**: generated local artefacts carry a version marker; on boot, missing/stale -> regenerate. Old installs heal without a migration.
- Watching: `notify-debouncer-full` + the `ignore` crate (debounced, gitignore-aware), never raw notify events.
- Minimal subscriptions in 3.0.0 (project/file changed) -- the seam the TUI and bus consume in 3.x.
- intentd ships IN v3.0.0 (hv ruling; one major release, patched by 3.0.x).

### MCP

- Tiered surface: ~10-12 typed high-traffic tools (st/wp lifecycle, ac/at, issues, todo, search, wb, doctor) + one `intent_graphql` escape hatch = full API access without tool-bloat context cost. MCP resources for read surfaces (wip, boards, ST docs).
- Under authored-once, **the mutation surface is how agents write structure** -- view-editing is demoted from workflow to recoverable mistake.
- rmcp (official SDK): stdio (`intent mcp`, per-session, in-process) now; bridge-to-daemon mode with per-request target resolution (the Lamplight `mcp.rs` pattern -- daemon restarts never strand a session); streamable HTTP from intentd for the 3.x multi-agent era (proven in conflabd for exactly this Claude Code use case).
- Tool definitions, CLI help, and the `intent llm` agent guide all render from the one dispatch-table SSOT (the Lamplight DD-6 pattern).

### Cloud seams (intentc-shaped, v4-facing)

Four cheap-now/expensive-later seams ship in 3.0.0; no cloud code does:

1. **`project_id` UUID** stamped into `config.json` at migration. Natural keys stay human-legible (ST0056, AC-01.2); the UUID namespaces them: `(project_id, natural_id)` is the global identity.
2. **A principal on every facade call**, defaulting to `local`. The forcing function is nearer than the cloud: the 3.2 agent bus needs principals (vc/cc/hv) before intentc does. Conflab's identity contract (`agent+<handle>`, server owns the api_key) is the eventual shape.
3. **An append-only event log table**: every mutation writes an envelope. Audit trail + subscription feed + the substrate a future sync protocol replays.
4. **A reserved `server` block in config** (unused in v3). The anticipated wire is Phoenix channels over WebSocket (the conflabd `ws.rs` pattern: `phx_join`/`phx_leave`, subscription list surviving reconnects) -- intentc will be Elixir/Phoenix/Ash, so the seam is already idiomatic.

## Migration (v2 -> v3)

- **Floor: v2.19.0. Two-hop policy**: older projects run v2's own `intent upgrade` first (that ledger exists and works; it is never reimplemented in Rust). The v2.19.0 consumer sweeps are therefore v3 migration prep.
- The migrator IS the legacy md parser (frozen once the fleet is over): read the v2 estate strictly -> refuse what cannot convert without loss, named (the `--fix` discipline, with the 87-destroyed-links scar to prove it) -> emit `thread.json` et al + regenerate fresh views -> stamp 3.0.0 + `project_id` -> build DB -> converge canon. One visible commit; rollback is `git revert` + the v2 formula.
- **The fleet corpus is the acceptance fixture**: every artefact in Intent + Lamplight + Utilz + Baize must ingest losslessly or appear in the residue report by name. Semantic completeness (every AC, AT, status, date, link, prose block accounted for), not byte round-trip. The Lamplight baseline (`intent/analysis/20260814-lamplight-at-sweep-baseline.md`, 1639 AT rows at `15dbccc92`) is the first fixture. Intent's own tree migrates first as canary.
- **Consumers' hooks do not change**: 0016's runtime-resolved `intent claude hook <name>` + byte-identical settings.json means the binary swap is invisible at the hook layer.

## Parity contract

BATS is the conformance harness, honestly scoped: **stdout, exit codes, and behaviour** are the parity contract; **file layout is a ratified deviation class** decided here, not discovered in test triage. Tests asserting what the tool says and does port; tests asserting how bytes were laid out in files that are now views retire with the layout, recorded in a keep/retire/deviate register. The `ok:`/`error:` lowercase voice (0023) and exit codes carry over unchanged. Rust-native tests grow alongside; BATS retires when the shell does (WP-12).

## Stack shortlist

Provenance: L = proven in Lamplight `native/cli`, C = proven in Conflab `native/daemon`.

| Crate                            | Role                                      | Provenance |
| -------------------------------- | ----------------------------------------- | ---------- |
| clap (derive)                    | CLI                                       | L, C       |
| async-graphql (+ axum)           | GraphQL schema + intentd serving          | C          |
| rusqlite (bundled, FTS5)         | store; WAL mode                           | C          |
| schemars                         | JSON Schema face                          | C          |
| jsonschema (in-doc resolve only) | ingest validation                         | C          |
| rmcp                             | MCP server (stdio + streamable HTTP)      | C          |
| notify-debouncer-full + ignore   | watching                                  | C          |
| thiserror                        | typed errors with remedies                | L, C       |
| serde_ignored                    | No-Silent inbound decode                  | L          |
| proptest                         | serialise/deserialise laws                | --         |
| rust-embed                       | embedded canon (templates, skills, rules) | --         |
| serial_test (file_locks)         | HOME-mutating test serialisation          | C          |
| cargo-dist                       | brew tap + release artefacts              | --         |

Declined from Conflab: mlua (agent scripting), AppleScript bridge (Conflab-specific); sqlite-vec noted as the 3.x path to semantic search. Lamplight's graphql_client codegen and Conflab's hand-written query constants are both client-of-a-foreign-server patterns; ours collapses because both ends live in one workspace.

## Design decisions (ratified by hv, 2026-08-14, this session)

- D01 **Truth model**: schema-as-truth; durable = committed schema-validated JSON; runtime = rebuildable SQLite; md = generated views + authored prose. `rm intent.db` always safe; no DB migrations ever.
- D02 **Authored-once + no mixed files.**
- D03 **JSON canon** (YAML et al as export projections).
- D04 **Generated views committed** + skew check (AGENTS.md pattern generalised).
- D05 **Strict ingest**; lenience only in the frozen legacy parser.
- D06 **Layering**: intentsvcs sole DB/file owner; CLI dual-mode (in-process facade | GraphQL to intentd); dep-graph guard; dual-path conformance suite.
- D07 **One intentd per machine**, N projects, per-project DBs, registry.
- D08 **intentd ships in v3.0.0** (hv overruled the 3.1 deferral; one major release + 3.0.x patches).
- D09 **Migration floor v2.19.0**, two-hop.
- D10 **In-tree cargo workspace**; shell pruned at the cut; BATS conformance on the narrowed contract.
- D11 **MCP tiered surface** + `intent_graphql` escape hatch.
- D12 **Elixir-oriented v3.0.0** = SDL + JSON Schema artefacts, socket/localhost consumability, stable voice/exit codes; `intent_ex` hex client deferred to demand.
- D13 **Header blocks line-oriented `key: value` everywhere** (0012 generalised; never YAML).
- D14 **Whiteboard stays md-authored** through 3.0.0/3.1; restructured in the 3.2 bus ST.
- D15 **Cloud seams in 3.0.0**: project_id, principal, event log, reserved server block.
- D16 **Homebrew is a core deliverable** (cargo-dist), not a stretch goal.
- D17 **Binary voice and exit codes carry over from v2** (`ok:`/`error:` lowercase).

## Alternatives considered

- **md-as-truth with strict ingest** (vc's first proposal): rejected by hv -- markdown cannot carry its own schema; the bespoke row-grammar tax recurs forever (0012/0017/close-gate were three instances); byte-faithful round-tripping was the hardest engineering in the draft and exists only to prop this up.
- **YAML canon**: rejected on the 0012 quoting-hazard scar; LLM hand-editing of YAML is the measured failure mode.
- **DB-only truth (no committed canon)**: rejected -- git is how Intent data travels; a binary DB cannot clone/merge/revert.
- **Per-project daemons**: rejected -- fleet views and the agent bus want one process; registry + per-project DBs give isolation without N daemons.
- **Daemon-mandatory CLI**: rejected -- basic operations must not depend on a resident process; the in-process executor satisfies Thin Coordinator at the API boundary.

## Open questions (WP-01 closes these)

- One shipped binary vs two (`intent` + `intentd`): lean two -- `brew services` prefers a real daemon binary; conflab/conflabd is the working precedent.
- launchd label naming (conflab uses reverse-domain `space.conflab.daemon`).
- Subscription extent for 3.0.0 (minimum: project/file changed).
- Exact `intent/.cache/` layout and gitignore convergence at migration.

## Parked for 3.x (each its own ST)

TUI dashboard (ratatui, subscriptions) · agent bus incl. whiteboard restructure + hv oversight gates (the conflabd `mcp/policy.rs` gating pattern) · Laksa web page · macOS menubar app (Conflab `native/macos` is the reference; TN3171 cert lesson recorded) · `intent_ex` hex client · sqlite-vec semantic search.
