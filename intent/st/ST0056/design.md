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

| Artefact                   | Canonical structure (committed)                                                           | Authored prose                      | Generated views                                                            |
| -------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------- | -------------------------------------------------------------------------- |
| Steel thread + WPs + AC/AT | `st/<ID>/thread.json` (metadata, status, WP records, contract, objective/context/related) | `design.md`, `impl.md`, `tasks.md`  | `info.md` (100% generated cover), `acceptance.md` (contract + coverage)    |
| Issues                     | `issues/<n>.json` (number, slug, status, priority)                                        | `issues/<n>.md` (body + resolution) | index views; OPEN/CLOSED-as-directories likely retires (status is data)    |
| Project                    | `intent/.config/config.json` (as today)                                                   | --                                  | --                                                                         |
| Indexes                    | --                                                                                        | --                                  | `intent/st/steel_threads.md` (v2 path kept), `intent/todo.md`, `AGENTS.md` |
| Tracking                   | --                                                                                        | `wip.md`, `restart.md`              | --                                                                         |
| Whiteboard                 | deferred to the 3.2 agent-bus ST; md-authored through 3.0.0/3.1                           | boards, inboxes                     | --                                                                         |
| Runtime DB                 | `intent/.cache/intent.db` -- gitignored, rebuilt from committed canon                     | --                                  | --                                                                         |

Format: JSON (not YAML) for canonical structured files -- the 0012 quoting-hazard scar, plus the `config.json` in-house precedent. Tool-written pretty-JSON: stable key order, 2-space indent, trailing newline. YAML/md/anything else are `intent export --format` projections (trivial via serde).

Generated views are committed, with a `doctor`/CI skew check (regenerate, require empty diff) so a hand-edited view is caught, never silently outvoted.

### Ingest, views and sync

- **Down-sync (truth -> views)** is the standard write path: mutation -> validate against schema -> write structured file + regenerate affected views + update DB, atomically. Deterministic and idempotent: same data -> same bytes.
- **Up-sync (files -> DB)** is ingest, and it is **strict**: standard JSON parse + schema validation; invalid input is refused with the finding named. No tolerance ladder for current-version data -- lenience was only ever a coping strategy for having no schema. Prose ingests verbatim (FTS-indexed).
- **Change detection**: SHA-256 hash of every in-scope file on every CLI invocation in daemonless mode; debounced fswatch in daemon mode. Scope: `intent/**` + named root files only. Stat (mtime/size) is retained as reporting metadata, **never as the hash trigger** -- rehash-on-stat-change cannot by construction catch the same-size same-mtime rewrite AC-03.3 requires, and stat is exactly the partial evidence v3 exists to stop answering from. Corrected by vc 2026-08-14 (surfaced by ic) after the original line specified the conflabd `db_sync.rs` rehash-on-change optimisation, which the contract rules out; if hashing scope ever costs enough to matter, taking that optimisation is a register-recorded deviation, never an accident.
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

### Project search (WP-13): four tiers, two of them in 3.0.0

Intent becomes the search and index surface for the whole project, replacing `treeindex` (762 lines that shell out to `claude -p` for a prose summary per directory, cached and stale by construction) and the `in-handoff` skill. Both answered "what is roughly here" in advance and approximately; the need is "where is X" on demand and exactly. That is a different artefact, not a better summariser.

| Tier | Question                                               | Needs              | Ships       |
| ---- | ------------------------------------------------------ | ------------------ | ----------- |
| T1   | lexical -- find this string                            | FTS5               | 3.0.0       |
| T2   | structural -- find this definition, its call sites     | tree-sitter        | 3.0.0       |
| T3   | semantic -- where do we handle project-root resolution | an embedding model | 3.0.x / 3.1 |
| T4   | type-aware -- every caller passing this type           | a language server  | parked      |

**T2 is why tree-sitter and not an Elixir-specific parser.** Elixir AST work is trivial in Elixir and the hard path from Rust -- no mature Rust Elixir parser, and shelling out to `elixir` reintroduces exactly the external-runtime dependency that condemns treeindex. tree-sitter is Rust-native with a maintained Elixir grammar, and the same dependency yields Rust, Swift, Lua and Bash. The `languages` array in `config.json` (ST0037) is already the per-project grammar manifest; the Elixir-specific ask generalises to all five declared languages for the same effort.

**T2 is also the chunker for T3, which makes the ordering a dependency rather than a convenience.** Naive line-window chunking is why most code retrieval is poor; tree-sitter yields function- and module-level units with names and spans, which are the right embedding units. Building structural search first is what makes semantic search worth having.

#### T3: the model decision, and why it is not an architecture decision

The only genuinely new thing T3 needs is an embedding model, and that is the first time Intent would require either a large binary or a network call. Three shapes were considered:

- **Remote API** (Voyage, OpenAI): small binary, but a network dependency, an API key, per-token cost, and the posture change that matters -- embedding a private codebase means sending it to a third party. Rejected as a default.
- **Bring-your-own only** (Intent defines the interface, ships nothing): keeps Intent self-contained, but a capability requiring configuration is a capability nobody turns on, which is precisely how treeindex died. Rejected as the only shape.
- **Zero-config local, fetched on first use**: adopted. The model lands in `~/.local/share/intent/models/` on the first semantic query, announced rather than silent, and is governed by the **policy-stamp self-healing already in the design (AC-08.7)** -- a versioned local artefact that regenerates when missing or stale. The binary stays small, the default needs no configuration, no network call happens at install time, and the embedding interface still admits a swapped-in model or a remote endpoint for anyone who wants one.

Storage is a `sqlite-vec` vec0 table beside FTS5 in the same DB. **D01 is what makes deferring T3 free**: the DB is rebuildable and there are no migrations ever, so adding vector tables later costs `rm intent.db` and a rebuild.

#### T4: parked, with the trigger written down

A language server adds type resolution, cross-module definition and typed call hierarchy -- real capability, at the cost of a stateful process per project per language, version-coupled to the toolchain. Elixir's official LSP is the obvious candidate and the natural trigger to revisit. Parked because **LSP's leverage is refactoring, not search**: tree-sitter already answers the search questions, and a different product wants its own thread rather than a lien on this release.

#### Why the seams (S1-S5, WP-13 info.md) are sufficient to admit T3 and T4 later

This is AC-13.9's substance, and the reason it is safe to specify two tiers now and build them later:

- **S1 (gitignore-aware whole-repo scope)** -- T3 and T4 index the same corpus T1/T2 do. No scope change is ever needed.
- **S2 (two corpora, two staleness policies)** -- orthogonal to tier; a new tier inherits it.
- **S3 (one result shape, `{path, span, kind, tier, score, snippet}`)** -- a new tier adds a VALUE to `tier`, never a field. The CLI contract and the MCP tool schema are untouched by T3 and T4 arriving.
- **S4 (scores never blended across tiers)** -- a new tier ranks within its own group, so no ranking rewrite and no silently-changed relevance for existing queries.
- **S5 (degradation is named, never silent)** -- an unavailable tier (no model fetched, no language server) is a named absence through the same mechanism as a stale index. T3 and T4 are therefore _always_ optional at runtime without a special case.

**Whole-repo indexing is also the first genuinely forcing argument for intentd.** Everything else works daemonless; keeping a source-tree index warm incrementally does not. This is where the daemon stops being present and starts being load-bearing.

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

WP-01 closures (2026-08-14, post-ratification):

- D18 **Two shipped binaries** -- `intent` and `intentd` -- from one workspace, one brew formula. `brew services` wants a real daemon binary; conflab/conflabd is the working precedent; cargo-dist handles multi-bin formulae.
- D19 **launchd label `com.matthewsinclair.intentd`** (reverse-domain, the conflab `space.conflab.daemon` pattern); plist at `~/Library/LaunchAgents/`, logs at `~/.local/share/intent/`, binary resolution PATH-first-then-sibling.
- D20 **3.0.0 subscriptions are exactly two**: `projectChanged(project_id)` and `fileChanged(project_id, path)`. Nothing more ships until a consumer (TUI/bus) exists to need it.
- D21 **`intent/.cache/` is gitignored whole-dir**; the DB lives at `intent/.cache/intent.db` (+ WAL/SHM siblings at runtime). ~~The treeindex cache location is unchanged until WP-06 ports the command; if it moves under `.cache/`, that is its own register entry.~~ **Second sentence STRUCK 2026-08-15 by D31**: it assumed treeindex would be ported and hv has ruled it retired. D21's decision -- the gitignored directory and the DB's location -- is unaffected, and AC-01.4, whose `.cache`-layout evidence rests on that decision, does not reopen.

**D01 disambiguation, 2026-08-15 -- "SSOT in the SQLite DB" means the RUNTIME reading. Recorded, not decided.** hv, mid-window and then AFK: _"we should be working towards SSOT in the SQLite db instance, and then working out from there. Obviously we have a lot of code to write before we get there."_ Two nodes read that independently and both stopped on the same ambiguity rather than nodding it through, which is why it is written here. **Runtime SSOT** -- every reader queries the DB; the committed JSON canon stays the durable truth the DB is rebuilt from -- **is D01 exactly**, is what WP-02/03/04/06 are built on, and is the reading in force. **Durable SSOT** -- the DB becomes the truth and files become projections of it -- is a different architecture and would cost three things already built on: `rm intent/.cache/` stops being always-safe, D01's "no DB migrations ever" is reversed permanently, and git loses the ability to diff, merge or review the model. It would also un-defer WP-13's T3, since design.md:110 makes deferring vector search free **only** while the DB is disposable. hv's own framing points at the runtime reading -- "a lot of code to write before we get there" and "working out from there" both describe building outward from a reified model rather than relocating truth -- and hv's following message, "Yes, please continue", answers the question about continuing, not this one. **D01 is in the hv-ratified D01-D21 set, so it is not vc's to reinterpret and this note does not reinterpret it**: it records which reading the estate is built to and flags that the other requires hv to reopen D01 explicitly. Standing authorisation does not reach a ratified decision; reversing D01 is the definition of existential. Question logged for hv's return. (Raised independently by cc and vc.)

WP-03 openings (vc, 2026-08-14) -- **ADOPTED under hv standing authorisation**, listed apart from the ratified log above because the authorisation is not the same thing as review. All four were forced by starting WP-03: three were surfaced by the builders inside the first hour, which is the argument for the contract leading the build rather than trailing it.

**What "ADOPTED under hv standing authorisation" means, stated once so nothing downstream has to guess.** hv, 2026-08-14, mid-window: _"go with your recs, unless they're existential"_. That is authorisation to proceed on vc's recommendation without waiting; it is **not** a record that hv read and approved each ruling individually. Every decision so marked was made by vc, is reversible by hv in one line, and carries its rationale inline so the reversal has something to argue with. The distinction matters because a decision log that cannot tell "the owner ruled this" from "the owner let this proceed" has lost the thing a decision log is for -- and D01-D21 above ARE the first kind, ratified in session with hv, which is why the two sets stay apart.

- D22 **`info.md` is 100% generated; `objective`, `context` and `related` are modelled fields on `steel_thread`.** Resolves the D02 mixed-file violation v2's info.md is. No sixth default steel-thread doc -- rejected on reversal cost. Rationale in data-model.md. (Surfaced by cc.)
- D23 **No generated view contains a render-time value, and the renderer has no clock.** Derived from AC-03.4, not chosen: regenerate-and-diff cannot come back empty if a view stamps its own render time, so the skew check would be trained-to-be-ignored. Three v2 instances at `f7434f1`, one of them inside the generated-banner pattern the data model ratifies. (Surfaced by cc.)
- D24 **Change detection hashes always; stat is reporting metadata, never the hash trigger.** The contract (AC-03.3) governs where it and the architecture narrative disagree -- that is what a contract is for. Corrects the original conflabd rehash-on-change line. (Surfaced by ic.)
- D27 **Message TEXT is not in the parity contract; voice, shape, exit codes, grammar and semantics are.** v3 error messages diverge from v2's strings by design, and the ~28 BATS assertions reading them classify **`deviate`** (parity.md:32 -- "asserts surface we are deliberately changing"), citing this D-number. Not `corrected`: v2's `Steel thread not found: ST9999` is not wrong, so a bug-fix class would misdescribe it. The divergence is forced by **AC-04.4**, which requires every facade error to render a remedy with its full cause chain. **Text-in-scope would make AC-04.4 and AC-05.2 mutually unsatisfiable** for every message v2 wrote without a remedy, which is most of them -- and two ratified ACs cannot contradict. D17 says voice and exit codes carry over; it does not say text, and parity.md's in-scope list says "stdout shape, stderr voice, exit codes, grammar, behavioural semantics" rather than every string byte-for-byte.

  A third option was proposed by cc and **declined**: keep v2's text as the FIRST LINE, with remedy and cause chain additive below, so `assert_output_contains` passes without losing AC-04.4. It fails on evidence and on principle. **Evidence: nothing outside the test estate consumes v2's message text** -- no shipped hook or `.claude/script` parses intent's output, so the ~28 assertions are the entire consumer, and design.md:146 already says tests asserting deliberately-changed surface retire by class. **Principle: it is the mirror of the move AC-05.2 fences.** Rather than narrowing the contract to fit the result, it widens the product to fit the test -- same defect, opposite end. It would also create an obligation with no antecedent: there is no defined first line for an error v2 never had. cc flagged their own option as "make the assertion pass while keeping what I like" and asked for a second pair of eyes; the instinct was right.

- D26 **The dispatch-table SSOT lives at `surface/dispatch-table.json`** (workspace root), with its generated view beside it as `surface/dispatch-table.md`. It was authored at `intent/st/ST0056/dispatch-table.json` and the shipped binary `include_str!`s it, which is correct -- one copy, compiled in -- but `intent/st/` carries `COMPLETED/` / `CANCELLED/` / `NOT-STARTED/` and `bin/intent_st` relocates a thread's directory on a status transition, so marking ST0056 Completed breaks the build **in WP-12, the release itself**. Root rather than inside a crate because consumers span crates (clap surface at WP-05; MCP tool list and `intent llm` guide at AC-09.1 / AC-09.4). `surface/` is the authored mirror of `schema/`: schema holds faces generated FROM the Rust types, surface holds the authored table that faces are generated FROM -- same committed-and-drift-checked discipline, opposite direction, separate directories so the authored/generated line D02 exists to hold stays visible. **This entry originally said the move was three things -- the file, the `include_str!`, and `gen_dispatch_table.sh`'s defaults -- and named the third as the one that gets forgotten. It was five.** cc found two more by grepping the estate for the old string rather than by editing the list they were handed: `coverage_map.sh`'s `CANON=` (a second tool in the same directory reading the same path) and the **MODULES.md registry row**, which is project canon that had just started recording a location that was no longer true.

**The generalisation, which is worth more than the corrected count: when a path moves, the unit of work is every reference in the ESTATE, not every reference you were told about.** A `grep -rn` over the old string costs nothing and is the only thing that closes the set -- a hand-enumerated list is an instrument, and this one was 40% wrong while sounding authoritative. `coverage_map.sh` would have failed loudly rather than silently, because ic's no-silent-empty-surface rule made it refuse a missing canon by name; it would still have failed, and nothing in the specified move would have caught it.

Verified after the move: regeneration at the new path reproduces the committed view byte-identically, so the move changed a location and nothing else. The whiteboard and inbox references to the old path were deliberately NOT rewritten (cc's call, endorsed): they are correct as history, and editing them to agree with the present would make the record lie about the past. Live pointers move; historical ones stand. (Raised by cc, who declined to move a file two nodes read.)

- D28 **`work_package` gains `objective` and `body` as modelled prose fields, and `WP/<NN>/info.md` becomes 100% generated.** This is **D22 one level down, and nobody applied it there** -- the WP `info.md` is the same mixed file for exactly the same reasons (frontmatter + status as structure, `## Objective` / `## Deliverables` / `## Dependencies` as authored prose, the `## Acceptance` block as fixed generated boilerplate), and the model had no home for any of it: `work_package` was `seq`, `title`, `scope`, `status` and nothing else. **The consequence is a lossy migration, which hv has ratified as never permitted.** WP-10 would port every work package by dropping its authored prose -- and the largest instance in this repository is `ST0056/WP/13/info.md`, the spec for the search work package, which the migration porting it would destroy. Found while verifying cc's `collect_wp_text` against AC-06.4's "WP text": the implementation indexes `wp.title` with `body: String::new()` and is **correct against the model**; there is no WP prose in v3 to index, which is the defect. Two fields rather than three-mirroring-D22 because real work packages exceed the template freely (WP-13 carries `## Why the incumbents go`, `## The tiers`, `## The seams`): `objective` is the one section the template guarantees and that views and summaries want structured, `body` takes every other authored section verbatim, so the pair is lossless by construction. **`deliverables` is deliberately NOT modelled as an array.** It is the artefact this thread already demoted -- WP-02 closed 5/5 with `intent schema` unbuilt because ACs are gated and deliverable lists are prose nobody diffs -- and giving it structure would re-privilege the thing the contract just replaced. It stays inside `body`. Accepted cost is D22's, unchanged: multi-paragraph markdown lives in a JSON string field, tool-written, stored verbatim, never reflowed. (Surfaced by vc verifying cc.)
- D29 **The ingest corpus excludes gitignored paths.** Derived from D01 rather than chosen: durable truth is **committed** schema-validated JSON, so a path git can never commit can never be canon -- it must never produce residue and must never block a read. **Measured at WP-06, on this repository: `intent search "steel thread"` exits 1 having read nothing, with 24 residue lines that are 100% `.DS_Store` and 100% gitignored** (`.gitignore:45`). Ingest walks the filesystem, git does not, and strict ingest (D05) then correctly refuses a corpus containing what it correctly cannot parse -- so **every macOS checkout is dead on arrival**, and because AC-10.2 makes residue a migration BLOCK, the fleet rollout at AC-10.5 fails at its first step on Lamplight, Utilz and Baize alike. D05 is not weakened by this: the ingest stays strict, the CORPUS gets defined. **This is deliberately not a `.DS_Store` special case, because the same rule is already load-bearing elsewhere and currently held by luck**: `intent/.cache/intent.db` exists and escapes the scan today through path shape, not through any exclusion rule -- `ingest.rs` contains no ignore handling at all -- and D21 gitignores that directory whole. WP-13 widens the corpus to the whole project for search, at which point a binary SQLite file enters scope. One rule now, or two bugs later. The rule keys on **ignored**, never on untracked: a `thread.json` created and not yet committed must still ingest, which is most of what a working session looks like. A project with no git has no ignore file and therefore nothing ignored, so it degrades to everything-in-scope rather than to nothing. Second defect found alongside and fixed with it: **each residue path is reported exactly twice**, so every residue count is doubled and AC-10.2's per-line classed report shows a migrator twelve problems as twenty-four. (Surfaced by vc verifying cc's WP-06 landings.)
- D25 **`intent/st/steel_threads.md` keeps its v2 path and becomes 100% generated**; the `<!-- BEGIN/END: STEEL_THREAD_INDEX -->` region markers and the `stp_version` frontmatter do not survive the port. A region marker in a v3 view is a defect, not a compatibility feature. The incumbent is the worked example of why D02 exists: its authored half rotted (a March 2025 verblock, `stp_version: 1.2.0`) while its generated table stayed current.
- D30 **The whiteboard enters the model, served by a bounded API. Supersedes D14's deferral; WP-14 carries it.** (hv ruling, 2026-08-15, direct.) D14 held the boards md-authored until the 3.2 bus ST, and **the deferral was argued entirely on transport** -- design.md:240 concludes "no new protocol, no new transport, and D14 intact ... **because a file does not need modelling to fire a change event**". That is true about delivery and never reached the question hv is asking, which is shape, size and searchability. D14 is completed rather than overturned. **The evidence is a ratio, not a total**: the live board is 102,886 bytes across 17 files with 251,244 more archived, and within that `vc/inbox.ic.md` alone is 31,998 while **`hv/wip.md` is 308** -- three LLM nodes wrote ~100KB in two days against the human's 308 bytes, and hv's board is the one that has always been the right shape. **The rule already existed and discipline did not keep it**: `vc/wip.md` opens its watch-outs with "a board does not outlive the session that writes it" and is itself 8,145 bytes, which is the precise signature of a rule needing a mechanism. The bound is therefore on the board, not on the thinking -- **a finding belongs in the artefact it is about and the board carries the pointer**, which is what already happened correctly for ic's measurement rules (`parity.md`) and vc's AC-10.7 (`acceptance.md`); the board copy was the redundant one in both cases. Mechanically: durable form is committed JSON canon per D01, the DB is the rebuildable index, `.md` becomes a generated view per D02, and the API **refuses** an over-bound write by name with the remedy -- D05's `additionalProperties: false` posture applied to size, never truncation. Two consequences neither hv nor this ruling went looking for. First, **fabricated timestamps stop being constructible**: the stamp becomes an API read of the clock, so the pre-commit clock guard demotes from primary defence to a legacy check -- a class currently held by a detector, closed by construction, and both vc and ic fabricated a stamp within the two days preceding this ruling. Second, it **closes the largest hole in ic's egest symmetry** (23:47Z proposal): `data-model.md:189` lists the whiteboard first among what does not survive losing the files, and this removes it from that list. Accepted cost, stated because it is real: hand-editing a board stops working, so `/in-whiteboard` and the `intent claude ws` family must move in the same WP or the protocol will document a workflow the tool refuses. **The AC/AT half of hv's ask needs no decision** -- `acceptance_criterion` and `acceptance_test` are already model entities inside `thread.json` (`data-model.md:83,94`) with `intent ac`/`intent at` as the API and `acceptance.md` already generated under D02, so the choke point hv wants there is already canon and merely unbuilt.

- D31 **`treeindex` and the `in-handoff` skill are RETIRED, not ported.** (hv ruling, 2026-08-15, direct — the ratification AC-13.1 was missing and ic's register row was blocked on.) hv's reasoning is that both are answers the model makes unnecessary: **the source-tree index in the DB obviates `treeindex`, and the DB model obviates handover entirely** — "state moves out of per-session `.md`s shared between workstreams into durable state in the intentdb". That is the same movement as D30 one artefact over, so the two decisions are one idea: a per-session prose artefact summarising state is a workaround for state not being queryable, and it stops being needed the moment state is. Consequences, all now unblocked: `treeindex_commands.bats` (53 tests) reclassifies from `deviate`/BLOCKED to **`retire`**, and a retire row needs no ratification-ref, so ic's open register question closes without a deviation being ratified; **WP-06 does not port `treeindex`** (762 lines of bash off its list) and must not, since porting a retiring command is work that gets un-done; WP-13's T0 tier is ratified rather than vc-specced; and D21's forward-looking clause is struck above. **`fileindex` is NOT covered by this and stays open** — it shares a naming convention with `treeindex` and nothing else, and bundling it because the names rhyme is the error class this thread exists to remove.
- D32 **Durable state is in the model; services expose mutations of it; APIs expose the services.** (hv ruling, 2026-08-15, direct, stated as general and applying across the board.) The layering: **state** → **service-layer mutations** → **API surfaces (CLI, MCP, GraphQL)**. No surface mutates state except through a service call, and every service call is reachable from every surface, so the CLI and an LLM have identical power by construction rather than by discipline. Ruled in answer to a concrete gap — `intent ac` could satisfy an AC but never un-satisfy one, so a verifier whose evidence proved incomplete had to hand-edit the file the CLI exists to own — and the general form is the answer: **a state that can be entered and not left is a missing mutation, not a missing flag.** Every AC/AT state transition, every whiteboard operation under D30, and the acceptance apparatus generally are in scope. **One phrase in the ruling is deliberately NOT read here**: "durable state is in the db" is recorded as stated and is NOT taken as reversing D01's truth model (durable = committed JSON canon, DB rebuildable, `rm intent.db` always safe), because the contrast hv was drawing was model-versus-scattered-md-files, not JSON-canon-versus-DB. Two nodes have now stopped on that ambiguity independently; it is queued for hv as its own question rather than settled by inference, because reversing D01 costs `rm intent.db`, reintroduces DB migrations permanently, and ends git's ability to diff or review the model.

- D33 **No node ever authors a timestamp. Stamping is the system's, project-wide, and the hand-authored clock rules are DELETED once it is.** (hv ruling, 2026-08-15, direct: _"we should never ever get into this clock nonsense again. We simply leave all timestamping up to the db and only ever look at timestamps as they relate to db entries that have a db-enforced timestamp. That should save a lot of heartache, and it means we can drop any other extraneous clock rules that seem (inexplicably) difficult to conform to."_) This generalises AC-14.4 past the whiteboard to **every stamped field in the model** -- boards, inboxes, events, AC/AT transitions, verblocks. The ruling is the correct diagnosis and the evidence for it is unusually strong: three nodes fabricated stamps inside two days, one of them four times **in the session where it was writing the rule, enforcing it on a peer, and citing it in the message carrying the fourth breach**. There is no internal clock to be approximately right about, so a stamp is generated like any other token unless composition is interrupted to read one -- which makes conformance an attention tax that is paid perfectly right up until it is not. **This is the sharpest available instance of "a control refuses; documentation reminds".** Two things the ruling constrains that it did not have to say. **First, the drop is conditional and is a deletion, not a softening**: the hand-authored rules and the pre-commit clock guard stay in force at full strength while boards remain md-authored, because the class is still constructible until the API is the only writer; the moment WP-14 lands they are removed rather than kept "just in case", since a rule that outlives its mechanism is exactly the reminder-shaped thing this ruling exists to kill. **Second, "db-enforced" must NOT mean a DB-side default.** Under D01 the DB is a rebuildable index and `rm intent.db` is always safe, so a `DEFAULT CURRENT_TIMESTAMP` column would re-fire on every rebuild and rewrite every historical stamp to the rebuild time -- silently, and identically to a correct one, which is the same failure shape as the fabricated stamp it replaces. The enforcement therefore belongs to the **service write path**, which reads the clock once and persists the value into committed JSON canon; the DB indexes what canon already says. Stated as a requirement rather than an inference about hv's wording: **whatever the truth model, a timestamp must survive a rebuild unchanged.** This is the second concrete instance of the D01 ambiguity already queued from D32, and it argues that the question be settled rather than carried.

## Alternatives considered

- **md-as-truth with strict ingest** (vc's first proposal): rejected by hv -- markdown cannot carry its own schema; the bespoke row-grammar tax recurs forever (0012/0017/close-gate were three instances); byte-faithful round-tripping was the hardest engineering in the draft and exists only to prop this up.
- **YAML canon**: rejected on the 0012 quoting-hazard scar; LLM hand-editing of YAML is the measured failure mode.
- **DB-only truth (no committed canon)**: rejected -- git is how Intent data travels; a binary DB cannot clone/merge/revert.
- **Per-project daemons**: rejected -- fleet views and the agent bus want one process; registry + per-project DBs give isolation without N daemons.
- **Daemon-mandatory CLI**: rejected -- basic operations must not depend on a resident process; the in-process executor satisfies Thin Coordinator at the API boundary.

## Open questions

None. The four WP-01 questions closed as D18-D21 above. Companion WP-01 specs: `data-model.md` (entities + draft JSON Schema), `migration.md` (flow, residue classes, fleet corpus harness), `parity.md` (conformance contract, register format, command inventory, IC handoff).

## Parked for 3.x (each its own ST)

TUI dashboard (ratatui, subscriptions) · agent bus incl. whiteboard restructure + hv oversight gates (the conflabd `mcp/policy.rs` gating pattern) · Laksa web page · macOS menubar app (Conflab `native/macos` is the reference; TN3171 cert lesson recorded) · `intent_ex` hex client · sqlite-vec semantic search.

### The agent bus is smaller than it looked (observed live, 2026-08-14)

Three nodes ran this session with a **live cross-session channel** -- a Claude Code harness capability (`ListAgents` / `SendMessage`, unix sockets under `/tmp/cc-socks/`), not anything Intent provides. It changed the working tempo materially: idle-to-dispatched in minutes, and two findings crossed nodes mid-build that the board would have delivered a session late. It also carried a wrong premise from one node into another's ruling in a single hop, which the board's latency had previously been masking as an accidental review delay.

What that demonstrates for the parked bus ST is that **most of it may already be specified**. An inbox append is a file write; D20 already ships `fileChanged(project_id, path)` over intentd's socket in 3.0.0. A node subscribed to `intent/whiteboard/<node>/inbox.*.md` gets live delivery with no new protocol, no new transport, and D14 intact -- the boards stay md-authored and unmodelled, because a file does not need modelling to fire a change event. The 3.2 question shrinks from "build a bus" to "point the existing subscription at the board, and decide the oversight gates".

Three constraints the live session surfaced, recorded now while the evidence is fresh:

1. **Ledger and traffic are different jobs.** The socket is ephemeral -- nothing on it survives a `/compact`, and it is in no repository. The board is the durable, auditable, single-writer record. Today the mirroring from one to the other was manual and disciplined, which is not a property that survives contact with a bad day.
2. **Therefore the board is authored and delivery is derived, never the reverse.** One authored home (the inbox file), with live push as a transport detail -- `wb ask` writes the entry and notifies a live peer. The inverse (send live, remember to mirror) is the authored-twice defect this whole release exists to stop.
3. **A cross-node claim carries its evidence, and the receiver RE-RUNS it rather than reading it.** Measured both ways in one session: re-running ic's three findings confirmed them and caught an overclaim; not re-running cc's absence claim -- re-checking its own wrong path instead of testing its premise -- turned a bad premise into a ruling in one hop. Speed multiplies corrections and errors equally.

Intent cannot route through the harness's channel and must not depend on it (consumers may not have it); it can use it when present. The transport Intent owns is intentd's.
