---
verblock: "14 Aug 2026:v0.2: vc - Specced from hv's ask; tiered so the ambition is designed now and built in stages"
wp_id: WP-13
title: "Project search: full-text, structural, and the agent search surface"
scope: XL
status: Not Started
---

# WP-13: Project search: full-text, structural, and the agent search surface

## Objective

Make Intent the **search and index surface for the whole project**, queried live, and offer that surface to agents through CLI and MCP. Retire `treeindex` and the handoff skill, which were the previous answers to the same question and were the wrong shape for it.

The ambition is full: lexical, structural and semantic search over the project, with type-aware queries as the eventual ceiling. **It is not built at once.** The tiers below stage it so that v3.0.0 ships everything needing no external model, and the tiers that do need one land afterwards as pure additions -- because the seams they need are designed in now. This is the D15 cloud-seams pattern applied a second time: cheap now, expensive later.

## Why the incumbents go

`treeindex` (762 lines) walks directories bottom-up and shells out to **`claude -p` to generate a prose summary per directory**, cached under `intent/.treeindex/` with fingerprint staleness detection. Three defects, and only the third is fatal:

- It requires the Claude CLI installed and burns an LLM call per directory to refresh.
- It is precomputed prose, so it is stale by construction between refreshes and there is no signal at the point of use telling you which.
- **It answers a question nobody asked.** The need is "where is X", answered on demand and exactly. treeindex answers "what is roughly in this directory", answered in advance and approximately. A summary cannot be more precise than the question it was written before hearing.

An index answers the question that was actually asked, at the moment it is asked, from current bytes. That is a different artefact, not a better summariser -- which is why this is a replacement rather than a repair.

`in-handoff` generates a session-handoff document. Superseded by the same reasoning: a handoff doc is a point-in-time prose snapshot of state that is queryable live.

**This WP is not purely additive.** WP-06 currently lists `treeindex/fileindex` among the commands to port; retiring treeindex removes 762 lines of bash from that port, and the two canon rules built on it (`/in-essentials` rules 3 and 4) collapse into one rule about `intent search`.

`fileindex` (758 lines) is a **separate decision, deliberately not taken here.** Despite the adjacent name it is not an index at all -- it manages file lists with checkbox states. It shares nothing with treeindex but a naming convention, and bundling its fate into this WP because the names rhyme is exactly the class of error this thread exists to stop. It stays in WP-06 until hv rules on it directly.

## The tiers

| Tier | What it answers                                                   | Needs                   | Ships       |
| ---- | ----------------------------------------------------------------- | ----------------------- | ----------- |
| T0   | -- (retirement of treeindex + handoff)                            | nothing                 | 3.0.0       |
| T1   | lexical: "find this string / phrase"                              | FTS5 (already in WP-03) | 3.0.0       |
| T2   | structural: "find this definition / its call sites / this module" | tree-sitter (pure Rust) | 3.0.0       |
| T3   | semantic: "where do we handle project-root resolution"            | an embedding model      | 3.0.x / 3.1 |
| T4   | type-aware: "every caller passing this type"                      | a language server       | parked, 3.x |

**T1 and T2 need no model, no network, and no runtime beyond the binary.** T3 is a category change -- it is the first time Intent would need either a bundled ML model (tens of MB in the binary) or a network call with an API key. That is the cut line, and it is principled rather than arbitrary: it is the line where Intent stops being self-contained.

**D01 is what makes the staging cheap.** The DB is rebuildable and there are no DB migrations ever, so adding the T3 vector tables later costs a `rm intent.db` and a rebuild, not a migration. Deferring a tier is free; getting the query surface wrong is not. So the surface is designed for all four tiers now, and only the backends arrive in stages.

### T2: tree-sitter, not an Elixir-specific parser

hv asked for Elixir AST search on the grounds that it is trivially easy in Elixir. It is -- **in Elixir**. From Rust it is the hard path: there is no mature Rust Elixir parser, and shelling out to `elixir` reintroduces exactly the external-runtime dependency that condemns treeindex.

tree-sitter is Rust-native, has a maintained Elixir grammar, and the same dependency yields Rust, Swift, Lua and Bash grammars at no extra cost. **Intent already knows which to load**: the `languages` array in `config.json` (ST0037) is precisely the per-project grammar manifest this needs, which is a seam that already exists rather than one to build.

So the Elixir-specific ask generalises to every declared language for the same effort. Elixir gets what was asked for; the other four get it as a side effect.

### T4: LSP is a different product

Parked deliberately, with the reason recorded so it is not re-proposed as an oversight. tree-sitter gives symbols, definitions and call sites without a server. A language server adds type information and cross-module resolution, at the cost of running and supervising a stateful process per project, version-fragile against the toolchain. LSP's real leverage is **refactoring**, not search -- a different product with a different acceptance contract. Revisit when someone wants the refactoring, not to make search marginally better.

## The seams that must be right in 3.0.0

These are the parts that are expensive to change later, and the reason the WP is specced whole before it is built in stages.

- **S1 -- index scope becomes the gitignore-aware whole repository.** Today the sync scope is `intent/**` + named root files. Replacing treeindex means indexing source. This is a contract change to `file_index` and to the watcher, and it is the single largest change in this WP.
- **S2 -- two corpora, two staleness policies** (see the D24 interaction below).
- **S3 -- one result shape across all tiers**: `{path, span, kind, tier, score, snippet}`. A tier arriving later must not change the CLI contract or the MCP tool schema. This is the whole reason to specify T3/T4 now.
- **S4 -- scores are never silently blended across tiers in 3.0.0.** Each tier ranks within itself and results are returned grouped by tier. Blending lexical and semantic scores is where search quality quietly dies, and a blended score nobody designed is one nobody can debug.
- **S5 -- daemon-optional, never silently degraded.** Daemonless mode indexes on demand and incrementally; a query against a stale or partial index says so by name, or refuses and names the remedy. It never returns a confident subset -- that is the v2 bug class this release exists to make unconstructible.

### The D24 interaction, named because it is a real conflict

D24 ruled that change detection **hashes always**, stat being reporting metadata only. That was ruled against a corpus of `intent/**` -- hundreds of files -- and it is right there. Hashing an entire source tree on every CLI invocation is not.

Resolution, and it wants hv's ratification because it qualifies a ruling made this morning: **two corpora, two policies.**

- `intent/**` (the canon): hash-always, unchanged. Small, correctness-critical, drives the skew check where a missed edit corrupts the contract.
- the source corpus (everything else): stat-then-hash. Large, and a missed same-size same-mtime edit costs one stale search hit, not a wrong contract.

The asymmetry is justified by consequence, not by convenience, and it must be written down rather than discovered -- a single "change detection" policy covering both corpora would be wrong for one of them whichever way it went.

### Why this is the first forcing argument for intentd

Every other capability in the design works daemonless; intentd ships because hv ruled it into the gate. Whole-repo indexing is the first thing that genuinely **needs** a resident process -- incremental background indexing, debounced, is the only tenable way to keep a source-tree index warm. This WP is where the daemon stops being present and starts being load-bearing.

## Deliverables

- T0: `treeindex` retired -- command, `intent/.treeindex/` cache, `/in-essentials` rules 3 and 4, and every canon reference (usage-rules.md, MODULES.md, ARCHITECTURE.md, working-with-llms.md, README.md). The `in-handoff` skill retired from canon.
- T1: FTS5 corpus widened from Intent prose to the gitignore-aware repository; `intent search` over it (AC-06.4's command, now with a real corpus behind it).
- T2: tree-sitter structural index driven by the `languages` array; symbol / definition / reference queries per declared language.
- The index maintained incrementally by intentd in the background; daemonless fallback that never lies about freshness.
- The MCP search tool (WP-09's tiered surface already lists `search`) returning the S3 result shape.
- The two-corpora staleness policy implemented and asserted.
- T3 and T4 specified in design.md as staged additions, with the seams above proven sufficient to admit them.

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-13` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-03 (ingest, FTS, sync engine and the file index this widens).
- WP-08 (intentd background indexing and watching).
- WP-09 (the MCP surface the search tool joins).
- Retirement (T0) touches WP-06's port list and WP-07's canon subsystem; sequence T0 before WP-06 ports commands that are about to be deleted.
