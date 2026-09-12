# Design - ST0069: project search, re-elaborated

**Status: reviewed and re-elaborated by vc on 2026-09-12 at hv's instruction; hv gave the go the same day (_"That is the whole point of this work!"_) and ruled it all into 3.0.2. The build plan is the last section.** This document is the search leg of ST0069. The thread's other two legs, the coordination model (inherited from ST0056 WP-14) and contract drift (inherited from ST0056 WP-16), keep their inherited designs in ST0056's cancelled work packages until they are elaborated in their turn.

## What hv asked for

_"Indexing the full source tree and providing vectorised (if that makes sense) and full-text search access to that, to both the LLM and via the intent cli, via something like `intent search [<sqlite-query> | <search-text>]`. Whatever we do here, it needs to be done in intentsvcs and exposed equally through the intent cli and the explorer via `/search`."_

Four requirements fall out of that sentence, and one question:

1. The corpus is the whole source tree, not Intent's own prose.
2. Full-text search over it, and a structured query door beside it.
3. One implementation in `intentsvcs`, skinned three ways: the CLI, the explorer, and the LLM's tool.
4. The explorer gets `/search` as a first-class surface, not a shell-out.
5. Whether vector search makes sense. The answer below is: yes, third, and gated on one decision that is hv's.

## Review of what the thread inherited

ST0056 WP-13 specified this whole and shipped only its first tier. It moved to this thread on 2026-08-30 with its number and its nine criteria (`AC-13.*`) intact, one XL package. `wp done` is gated on the criteria keyed to the package closing, so an XL that delivers seven separable things could only close when all seven had. The re-elaboration cancels it in favour of packages that each close on their own criteria, re-keys every surviving criterion to the package that delivers it, and withdraws the old rows with the reason on the record.

| Inherited | Disposition                                                                                                                                                                                                                                                  |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| AC-13.1   | Shipped in 3.0.1: `intent treeindex` and `intent fileindex` refuse as retired, the `in-handoff` skill is gone. Nothing remains to satisfy; withdrawn as shipped.                                                                                             |
| AC-13.2   | Stands, refined: the scope is the gitignore-aware repository AND every exclusion inside it is a named row, never an absence. Re-keyed to WP-18.                                                                                                              |
| AC-13.3   | Stands, refined: prose and source are two FTS tables with two tokenisers behind one query, one result shape, grouped by tier. Re-keyed to WP-19.                                                                                                             |
| AC-13.4   | Stands, refined: symbols come from each grammar's own tags query; references are name-matched and say so; the binary-size cost of each grammar is measured before it ships. Re-keyed to WP-20.                                                               |
| AC-13.5   | Stands as written (D24 for canon, stat-then-hash for source, each asserted against its own missed-edit case). Re-keyed to WP-18.                                                                                                                             |
| AC-13.6   | Stands as written; it is the estate's dominant rule (the silent-empty class, AC-10.7) applied to an index. Re-keyed to WP-19.                                                                                                                                |
| AC-13.7   | Stands, refined: parity is by construction, because the daemon and the daemonless path run the same reconcile-then-query in `intentsvcs`; a daemonless query reconciles the source corpus before answering. Re-keyed to WP-22.                               |
| AC-13.8   | Stands, and is nearly free: the MCP tool is generated from the register row, so `--json` on the CLI and the tool's response are one envelope from one facade call. Re-keyed to WP-19.                                                                        |
| AC-13.9   | Stands, refined: the seams are proven by the T1 and T2 build not changing the CLI or MCP schema; T3's embedder interface and vector schema are specified here; the runtime decision is put to hv with measured costs rather than assumed. Re-keyed to WP-23. |

## What search is for, stated before what it is

Three consumers, and they want different things from the same index.

- **The LLM agent**, through the MCP tool and through `intent search` in a shell. Its own tools are grep, glob and read, which are always fresh and always exact. What it lacks is a cross-language symbol lookup (_where is X defined; does a Y already exist_, the Highlander question the skills currently answer by grepping a hand-maintained registry), one ranked result over prose and code together (_uninstall_ should return the criterion, the work package, the issue and the Rust function), results that say when they are stale, and precise structured queries over Intent's own model.
- **The human at the CLI**: the same, with snippets and lines.
- **The human in the explorer**: the same, navigable.

**The value is unification, symbols, ranking and entity awareness. It is not speed.** For repositories of this estate's size grep is fast enough, and an index that merely raced grep would lose on freshness and win nothing. This decides what is worth building: the structural tier and the entity-aware envelope carry the value; the lexical tier over source is the floor those stand on; the semantic tier is the roof, and it is worth having only once the chunks under it are symbols rather than line windows.

## As built in 3.0.1, measured

- `doc_sections` is an FTS5 table (`porter unicode61`) over the store's authored prose: thread objective and context, carried attachments as one section each, work-package titles and bodies, issue bodies. No source file is indexed. `intent search <query>` and the generated MCP tool both call `Facade::search`, discriminate an empty index from a miss, and report a hit's line only when the indexed bytes match the disk (issue 0195). The query escaper in `fts.rs` is the one place a user string becomes an FTS expression.
- `file_index` already carries `path, size, mtime, sha256, state` for the sync corpus, hashed always (D24), over a corpus defined by ripgrep's `ignore` walker with git's own rules (D29). The watcher in `intentd` registers the project root non-recursively and `intent/` recursively, debounced, and reads the same scope object the scanner reads.
- The register is the single source for the verb, the MCP tool and the agent guide; a new verb touches the register, the facade, one render arm, one MCP arm and one test file. The explorer's palette already lists `search`, and running it lends the terminal to the CLI and returns; the results are not a pane.
- Nothing structural, nothing semantic: no tree-sitter, no symbol table, no vector store, no chunker. `rusqlite` is `bundled` alone.
- The skills and the CLAUDE.md template tell the agent to answer the Highlander question with `intent modules find`, a substring grep over the markdown rows of `intent/llm/MODULES.md`. No skill names `intent search`.

## Architecture

Everything that decides lives in `intentsvcs`, in one module tree, `index/`, with a pure core and an impure rim in the estate's PFIC shape:

| Part        | Pure or impure | Owns                                                                                                                                                                     |
| ----------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `corpus`    | pure           | The scope rule: which paths are in, which corpus each belongs to (canon, prose file, code file), the exclusion reasons, the kind-from-extension map shared with `critic` |
| `reconcile` | impure         | Walks the scope, applies the two staleness policies, re-indexes what changed, records every skip with its reason, stamps `index_state`                                   |
| `tokenise`  | pure           | The FTS expression for each table from one user string (`fts.rs` grows a second target, it is not copied)                                                                |
| `symbols`   | pure           | tree-sitter parse of one file to symbol rows through the grammar's own tags query                                                                                        |
| `query`     | impure         | Runs the tiers asked for, ranks within each, assembles the envelope, never blends                                                                                        |
| `freshness` | pure           | Turns `index_state` and the skip and stale rows into the envelope's `index` block                                                                                        |
| `sql`       | impure         | The read-only statement door                                                                                                                                             |

The skins call `Facade::search` and `Facade::search_sql` and print. `render.rs` gets one arm per verb; `mcp.rs` gets one serve arm per exposed row; the explorer's `views.rs` turns hits into rows; `wire.rs` gets an `Op::Search` that carries the same envelope. Not one of them ranks, filters or decides freshness.

## The corpus (S1)

The scope is the gitignore-aware repository as D29 defines it and as `ignored_paths_corpus.rs` holds it: the committed `.gitignore` rules, nested files and negations decide membership, and neither `.git/info/exclude` nor the global excludes file does, because a corpus that honoured either would be a property of who is running the tool and would differ between two clones of one commit. (This paragraph said D29 stood whole with both honoured until 2026-09-12; cc measured the landed test before building and the amendment is withdrawn.) What changes is the instrument: today `Ignored::for_root` derives the ignored set as the difference between two walks, and its own note records that on this tree the unfiltered walk visited the whole of `native/rust/target/` and cost doctor ten seconds; widening that technique to the repository rebuilds the defect unbounded. So `Ignored` becomes a matcher, built once per root and asked per path, enumerating nothing, with a test that a large ignored directory is never visited, and it is the one statement of what git ignores: `sync`'s walk and `Project::files_in` both read it. `sync` does not widen with the scope: `scan` is untouched and its roots, the named root files and `intent/`, are the canon corpus, so no source file is ever ingested as canon; the index's own scope is a second question asked of the same object, `Scanned::in_repository`. Inside the scope:

- **Corpus assignment is by kind, not by directory.** The store's entities stay the canon corpus, unchanged. A markdown or text file on disk (`README.md`, `docs/**`, `intent/wip.md`, a whiteboard board) joins the prose table through the existing heading splitter, with `owner_type = file`. A code file joins the source table. The extension-to-language map is new and lives in `index::corpus::lang_of`; its language names are `critic::HEADLESS_LANGUAGES`, pinned by a test, and critic is not changed, because its per-rule globs are deliberately finer than an extension map (the first draft of this sentence claimed a map in critic that does not exist; cc measured it, WP-18's shape). The `corpus` column carries three kinds, `canon`, `prose` and `code`.
- **The store's own projections are not in the disk corpus.** Rendered views under `intent/st/**` and the canon extract under `intent/.canon/**` are the store's prose seen twice; indexing them would return every entity hit beside its own rendering. `intent/.cache/` (the database itself, inside the watched tree), `intent/.backup/` and `.git/` are out by rule, not by path shape; the forward reference already written into `sync.rs` is discharged here.
- **Every exclusion is a row.** A binary (NUL in the first block, as grep decides it), a file over the size cap (a config value with a measured default), an unreadable file and a symlink each get an `index_file` row with `skipped_reason`. `intent index status` lists them. A file the index does not hold is never an absence.
- **Two corpora, two staleness policies** (AC-13.5, the D24 interaction): canon hashes always; source is stat-then-hash. The asymmetry is justified by consequence, a missed same-size same-mtime edit in source costs one stale hit, and each policy is asserted against its own missed-edit case.
- **One scope object, two consumers.** The scanner and the watcher already read one `Scanned`; widening it widens both. The watcher's root registration becomes recursive. On FSEvents that is one watch; on inotify a recursive registration would take a watch per directory, ignored trees included, so the Linux path registers in-scope directories from the filtered walk and re-registers on directory creation. Measured on both before the package closes, because the number that decided the current non-recursive root was the unfiltered tree.

## The tiers

### T1, lexical, widened

The prose table keeps `porter unicode61`. The source table is `unicode61` without stemming: stemming mangles identifiers, and unicode61's default token characters split `snake_case` into its words, so a search for `disabled` finds `parse_disabled`, while `CamelCase` stays one token and is reached by the prefix form `Removal*` that the escaper already preserves. One row per file for T1; T2 adds the symbol rows and a `name_parts` column so the words inside a camel-cased name are searchable too.

**Trigram is the recorded alternative, not the choice.** FTS5's trigram tokeniser matches substrings, which is how people search code, at a larger index and with substring semantics the prose table does not share, so one query would mean two things.

**MEASURED 2026-09-12 AGAINST THE REAL `src_sections` (ic, ruled by vc), AND THE MEASUREMENT CHANGED THE ALTERNATIVE RATHER THAN CONFIRMING IT.** The fixture is seven Intent source files -- `critic.rs`, `fts.rs`, `prose.rs`, `sql_gate.rs`, `nav.rs`, `form.rs`, `remedy.rs`, 157,538 bytes -- indexed by `intent index rebuild`, with the query set taken FROM the corpus rather than invented: 68 `snake_case` function names and 21 CamelCase types, each with the file that defines it. Recall means the defining file comes back.

| query form              | unicode61 | trigram |
| ----------------------- | --------- | ------- |
| whole name              | 68/68     | 68/68   |
| last snake word         | 68/68     | 64/68   |
| first snake word        | 68/68     | 47/68   |
| inner fragment, 5 chars | 10/67     | 67/67   |

CamelCase types: whole name 21/21 under both; inner fragment 0/10 under unicode61 and 10/10 under trigram. The prefix form the escaper preserves works as designed -- `Seve*` finds `Severity`.

**The control, because a mirror is worthless without one:** the in-memory unicode61 table the trigram arm was measured against agrees with the real `src_sections` on all 68 whole-name queries, path set for path set.

**EVERY TRIGRAM MISS IS A QUERY SHORTER THAN THREE CHARACTERS, CHECKED RATHER THAN ASSUMED**: all 21 first-word misses are one or two characters (`as`, `a`, `no`), and all 4 last-word misses are two (`of`, `it`, `on`). FTS5's trigram tokeniser cannot index a term shorter than three, so under trigram **a one- or two-character query returns nothing at all** -- not fewer rows, none -- and that reaches identifiers people really search: `fs`, `os`, `db`, `id`.

**So unicode61 without stemming STAYS, and the trade is stated rather than hidden**: the cost is inner-fragment search, 15% on snake names and 0% on CamelCase, and the remedy for a fragment is the word form or the prefix form, both perfect. The design's original alternative was _trigram if fragment recall is poor_; what the numbers say is _trigram would cost every short query entirely_, which is a different trade from the one anticipated. Fragment search, if it is ever wanted, is a second column or a tier -- a package, and not a migration. `intentsvcs/tests/the_source_tokeniser_is_measured.rs` (AT-19.4) pins the properties the ruling rests on, so a tokeniser change under it reds rather than quietly making these numbers fiction.

### T2, structural

tree-sitter, grammars compiled in for the code languages Intent declares (Rust, Elixir, Swift, Lua, Bash for `shell`), parsed only for the languages in the project's `languages` array. Symbols are extracted through each grammar's own `tags.scm` query, the tree-sitter tagging convention that every maintained grammar ships, so there is no hand-written extractor per language and a new language is a grammar and nothing else. The rows are definitions (module, function, type, constant, with kind, name, parent and span) and references, which are **name-matched occurrences and are named as such on every surface**; without type resolution nothing here can say _callers_, and a surface that said it would be the confident wrong answer this estate refuses.

`intent search --kind def <name>` is the Highlander check: it answers _does a thing with this name already exist_ from the tree rather than from a registry someone remembered to update. The skills, the template and the agent guide point at it once it exists, and `intent modules find` retires on hv's ruling, fail-forward.

**Binary size is the cost, and it is measured, not assumed.** Each grammar is C compiled into the binary; Swift's is the outlier by a wide margin. The package records the delta per grammar before any grammar ships, and hv rules on any grammar above the line hv sets. A grammar behind a build feature is the fallback shape, not the default, because a capability requiring configuration is a capability nobody turns on.

**Measured 2026-09-12 by dc (AC-20.4)**, subject ba3992672, one private worktree with its in-tree target dir under an isolated HOME; rustc 1.98.1, cargo 1.98.1; `[profile.release]` lto fat, codegen-units 1, strip debuginfo; `cargo build --release -p intent-cli --features <set>`; tree-sitter 0.27.0 with rust 0.24.2, elixir 0.3.5, swift 0.7.3, lua 0.5.0, bash 0.25.1. Two controls make the table trustworthy: each grammar is REFERENCED behind `env::var_os` and the probe prints its node-kind count, because a grammar that is compiled but never called is dead-stripped under fat LTO and would read as free; and the runtime (parser and query engine) is measured by a second probe that parses and compiles a query, because the cheap probe strips it.

| build                       | bytes      | delta from baseline |
| --------------------------- | ---------- | ------------------- |
| baseline, no grammar        | 11,758,720 |                     |
| swift                       | 15,546,400 | 3,787,680           |
| elixir                      | 13,181,568 | 1,422,848           |
| bash                        | 13,137,296 | 1,378,576           |
| rust                        | 12,884,464 | 1,125,744           |
| lua                         | 11,810,144 | 51,424              |
| all five                    | 19,539,088 | 7,780,368           |
| all five, runtime exercised | 19,695,040 | 7,936,320           |

The deltas are additive (the five singles sum to within linker alignment of the all-five build), so any subset can be priced from the table; the runtime is paid once. Swift alone is close to half the whole grammar cost. Node-kind count does not predict size (elixir has fewer kinds than rust and costs more), so a sixth language is measured, never estimated. **The line is hv's.** The crates entered the tree through WP-20's symbols module behind per-language features, all off by default until hv rules. One fact found there bears on the line: **`tree-sitter-bash` 0.25.1 ships no tags query** (a highlight query only), so `shell` parses and names no symbols, and the bash row above buys nothing for its bytes. Ruled 2026-09-12: `lang-bash` stays declared and off until upstream ships a tags query, and nobody writes one here, because a hand-written query is the per-language extractor AC-20.2 exists to avoid. The recommendation to hv is therefore the other four on by default.

### T3, semantic: yes, third, and gated on one decision

Does vector search make sense? For a human exploring an unfamiliar repository, yes. For the agent, the marginal value over T1 and T2 is real but smaller than it looks: it already reads ranked hits and reasons over them, and what it cannot do today is the symbol lookup, not the fuzzy one. So T3 is worth having, after T2, because T2 is its chunker: function- and module-level units with names and spans are the right embedding units, and line windows are why most code retrieval is poor.

What T3 needs that nothing else does is an **embedder**, and the 2026-08 design underpriced it. A locally run model needs a runtime, either ONNX Runtime as a dynamic library beside the binary, with platform builds and notarisation consequences for the signed pair, or a pure-Rust inference crate compiled in, with a large dependency tree and build time. That, and not the model weights, is the category change. Three shapes, with the recommendation:

- **Null**: the `Embedder` interface exists and refuses a semantic query with a remedy naming the configuration. Ships first, costs nothing, makes the tier's absence a named absence (S5).
- **HTTP**: an OpenAI-compatible embeddings endpoint from `config.json`, which covers a local Ollama and any hosted provider. Small, no runtime in the binary, opt-in. The posture cost is the operator's choice, stated in the docs: a hosted endpoint sends the corpus to it.
- **Local**: a compiled-in runtime and a small model fetched on first use into `~/.local/share/intent/models/`, governed by the policy-stamp self-healing already in the design. This is the zero-config shape and the expensive one.

**Recommendation: Null and HTTP in this thread, Local after hv rules on the runtime and the binary-size line.** Storage is a vector column beside the symbol rows with cosine ranking in Rust; `sqlite-vec` is the recorded upgrade when a measured corpus outgrows brute force, which the estate's corpora do not.

### T4, type-aware: parked, trigger recorded

A language server adds type resolution at the cost of a stateful process per project per language, version-coupled to the toolchain. Its leverage is refactoring, not search. Revisit when someone wants the refactoring.

## The result shape and the envelope (S3, S4, S5)

One envelope from one facade call, rendered three ways:

```json
{
  "query": "parse_disabled",
  "index": {
    "complete": false,
    "reconciled_at": "2026-09-12T06:58:04Z",
    "corpora": {
      "canon": { "policy": "hash", "files": 0 },
      "source": { "policy": "stat-then-hash", "files": 0 }
    },
    "skipped": [{ "path": "docs/design/intent-logo.png", "reason": "binary" }],
    "stale": []
  },
  "groups": [
    {
      "tier": "structural",
      "hits": [
        {
          "kind": "def",
          "name": "parse_disabled",
          "lang": "rust",
          "path": "native/rust/crates/intentsvcs/src/critic.rs",
          "span": { "start_line": 579, "end_line": 593 },
          "score": 0.0,
          "snippet": "fn parse_disabled(text: &str) -> Vec<String>"
        }
      ]
    },
    { "tier": "lexical", "hits": [] }
  ],
  "matched": 0,
  "returned": 0
}
```

Rules the envelope enforces:

- **Grouped by tier, ranked within a tier, never blended** (S4). A blended score nobody designed is one nobody can debug.
- **A tier a later package adds is a new group, never a new field** (S3). The CLI contract and the MCP tool schema do not move when T3 lands.
- **Both denominators travel** (`matched`, `returned`), the `events` page pattern, so a capped result is never a silent subset.
- **Freshness is part of every answer** (S5). `complete` is false when anything was skipped or is stale, and the terminal rendering prints the reason line before the hits. `--no-reconcile` answers from the index as it stands and names what moved.
- **A hit's line is a claim about the disk**, kept only when the indexed bytes still match; otherwise the hit carries no line and says stale, the rule issue 0195 established for prose. Three cases, found by driving WP-19: a canon section is a field in `thread.json` and never a byte range of a file, so it is never stale on that ground; an unrealised attachment has no file on disk and is not stale either; only a file whose bytes have moved is stale, and the envelope says so for that case alone. Folding the first two into the third reports a healthy project as stale on every canon hit.
- **A terminal miss is still discriminated**: empty groups over an empty corpus is not a miss, and the note goes to stderr as it does today.

## The surfaces

### `intent search`

```
intent search <query> [--tier lexical|structural|semantic]... [--kind def|ref|file|thread|wp|issue|attachment]...
                      [--lang <lang>] [--path <glob>] [--limit <n>] [--no-reconcile] [--json|--format <f>] [--daemon]
intent search --sql <statement> [--limit <n>] [--json]
intent index status [--json]
intent index rebuild [--corpus canon|source]
```

`--format` is terminal-channel only and not exposed on MCP, as `events` does it. A bare query is text; the structured door is the flag and nothing is auto-detected, because a query that begins with `select` is also a thing someone searches for. `query` is therefore optional in the register and the verb refuses a query and `--sql` together, or neither, as a usage error. The door's bounds are defaults, named in its refusals: a default row cap, a hard ceiling, and a bound on SQLite work per statement.

### The structured query door, `--sql`

hv's `<sqlite-query>` half. One statement, run on a second connection opened read-only with `query_only` set and an authorizer that permits reads alone; a second statement, a write, or a pragma that changes state is refused with the remedy naming the read-only contract. Rows come back as objects under `--json` and as an aligned table otherwise, capped by `--limit` with both denominators. Every response carries the store's schema version, and the schema itself is already published by `intent schema`, versioned per release since batch 1 of 3.0.2 made the release regenerate its faces. That published face is the contract the agent writes against.

The overlap with GraphQL is named rather than hidden: GraphQL is the daemon's typed read face, daemon-only; `--sql` is the store's raw read face, daemonless and MCP-exposed. Both are read-only. Whether the estate wants both is a ruling for hv; the recommendation is yes, because the SQL door is small and answers a class of question, joins across the model and the index, that no verb answers today.

### The MCP tool

Generated from the register row as today. `search` gains the flags above as parameters and answers the envelope; the structured door is the same row's `--sql` flag, not a second row, because the tool list is generated from the register and a `search sql` leaf would parse a text query whose first word is `sql` as the leaf (ic, WP-17's shape, 2026-09-12). The row's `help` is the tool's description and states the two modes. Nothing in `mcp.rs` beyond the serve arms.

### The explorer's `/search`

Today `/search` in the palette lends the terminal to the CLI. It becomes a resident pane: a `nav::View::Search` whose rows are the hits (`{name: path:line or entity id, value: snippet}`, each a `button` row as every navigable row in this estate is, the hit's kind carried in the value; the row's `kind` field is the mode machine's widget discriminator and not a taxonomy, found by ic when a `thread` kind opened an editor over a hit), the freshness line in the INFO section, groups as sections. Enter on an entity hit lands on its view through the existing navigation; Enter on a file hit opens the file at its line through the lent terminal, the way `edit` already borrows it. The pane calls the same facade method as the CLI; the TUI's pure layers stay pure and testable without a terminal. An option for hv: the omnibox treats input that matches no entity as a search, which makes the search box the home screen.

### The daemon

`Op::Search` crosses the wire with the envelope. `intentd` keeps the index warm through the widened watcher, incrementally: a changed file is re-tokenised and re-parsed and its rows replaced. A daemonless query reconciles the source corpus first (a stat walk, then hashes for what the stat moved), so its answer is identical to the daemon's for the same tree state; the difference is when reconcile ran, and `--no-reconcile` trades that for a named staleness. The MCP face stays in-process and correct without the daemon, because it reconciles too.

### The agent canon

Once T2 answers the Highlander question, every skill, template and rule that names `intent modules find` for a lookup names `intent search --kind def` instead, the agent guide regenerates from the register, and `intent modules find` retires on hv's ruling. Until then the skills are unchanged; a canon that points at a verb that does not yet exist is the defect the doc audit spent a day removing.

## Data model additions

All derived, all in the per-machine store, none in the committed extract (D34, D35), each DDL block carrying `openness: DERIVED`, each a migration priced like any other.

| Table          | Purpose                                                                                                                                                                                                                                                                                                                                                              |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_index`   | The change detector's table, its writer alone. Rung 19 widened it with the index's four columns; the next rung moves them out, because `replace_file_index` deletes every row the sync scan did not produce and the two corpora are not nested either way (cc measured it 2026-09-12; ruled two tables, one writer each)                                             |
| `index_file`   | One row per in-scope path, skipped ones included: `corpus`, `lang`, `indexed_sha256` (NULL where skipped), `skipped_reason`, `size`, `mtime`. The index's writer alone; nothing hashes a file a skip reason excludes                                                                                                                                                 |
| `src_sections` | FTS5 over code, `unicode61` without stemming: `path, seq, start_line, end_line, kind, name, name_parts, body`                                                                                                                                                                                                                                                        |
| `symbols`      | One table, one writer: `path, lang, name, kind (def or ref), span`, as ic's `Symbol` type carries it; refs are name-matched occurrences and say so in the type. This table said `symbols` and `symbol_refs` until 2026-09-12; one type in two homes with a join to put it back together was the wrong shape once the kind travelled with the type (cc's call, ruled) |
| `index_state`  | Per corpus: scope fingerprint, `reconciled_at`, tokeniser version, grammar versions                                                                                                                                                                                                                                                                                  |
| `embeddings`   | T3: `chunk_id, model, dims, vector BLOB`, a migration that lands with the tier                                                                                                                                                                                                                                                                                       |

The size consequence (D34: FTS is roughly twice its corpus, and it is already most of the database) becomes load-bearing once source is indexed, which is why the index never travels and why `intent index status` reports the measured sizes rather than anyone estimating them.

## Decisions for hv

1. **Tokeniser for source**: `unicode61` without stemming, with trigram as the measured alternative. Recommendation: build `unicode61`, measure recall on an identifier fixture, switch only on evidence.
2. **Grammar size line**: the binary-size delta per grammar is measured before shipping; hv sets the line. Swift is the expected outlier.
3. **T3 runtime**: Null and HTTP embedders now; Local after a ruling on the runtime and the size line. Recommendation as stated.
4. **The SQL door beside GraphQL**: two read-only structured faces, one daemonless. Recommendation: ship `--sql`.
5. **`intent modules find` retires** once `search --kind def` holds. Recommendation: retire, fail-forward, no shim.
6. **The omnibox as search box**: non-entity input becomes a search. Recommendation: after the pane exists, on use.
7. **Cross-estate search** is out of scope; one index per project. A v4 concern.

## Work packages and sequencing

The umbrella package is cancelled and replaced by packages that each close on their own criteria. Sizes are T-shirt.

| WP  | Title                                                                                            | Size | Depends on |
| --- | ------------------------------------------------------------------------------------------------ | ---- | ---------- |
| 17  | The structured query door: `intent search --sql`, read-only over the published schema            | S    | nothing    |
| 18  | The corpus: the gitignore-aware repository, two staleness policies, the widened watcher          | L    | nothing    |
| 19  | Lexical search over the whole corpus: the envelope, `--json`, the MCP tool, `intent index`       | M    | 18         |
| 20  | Structural search: tree-sitter symbols per declared language, and the agent canon that uses them | L    | 19         |
| 21  | The explorer's `/search` pane                                                                    | M    | 19         |
| 22  | Daemon-served search with daemonless parity                                                      | M    | 18, 19     |
| 23  | Semantic seams: the embedder interface, the Null and HTTP embedders, the vector schema           | S    | 20         |
| 24  | The LLM boundary: the harness's own search becomes a door into the index                         | M    | 20         |

WP-17 can land first and alone. WP-18 is the largest single change and the one that makes the daemon load-bearing. WP-20 is where the value is, and WP-24 is why hv wants it. WP-23 is the whole of T3 that this thread commits to; the Local embedder is a package for after hv's ruling.

## Risks, named

- **Tests reach live machine state.** Any test that indexes a tree indexes a fixture tree under an isolated HOME, never the estate it runs in; a test that walked the live repository would index the live store's own directory. The watcher tests run against a scratch project, not the shared checkout.
- **inotify on Linux** takes a watch per directory; the registration strategy is platform-specific and measured.
- **The SQL door couples agents to the schema.** Mitigated by the versioned faces and by `schema_version` in every response; not eliminated.
- **Index size** is the D34 consequence made real. Reported, never estimated.
- **A grammar's tags query is the extractor.** A grammar whose `tags.scm` is thin gives thin symbols; that is reported per language in `intent index status`, not papered over.

## The LLM boundary

The boundary today is decided by tool descriptions, not by capability. The model's own tools are grep, glob and read: always fresh, always exact, no setup, lines straight into context. Intent's index is reachable through `intent search` in a shell and through the MCP tool `intent mcp` serves, and nothing routes the model to either: no `.mcp.json` ships with a project, the tool describes itself as prose search, and the skills name `intent modules find`. A model picks a tool from three inputs, the description, the instructions in context, and what worked last time in the session, and grep wins all three. An index earns the call only when it answers a question grep cannot, in one round trip, with an answer the model can trust without checking; the first stale hit sends the model back to grep for the session, and it is right to go.

The lift is five join points, in order of leverage, on one contract:

1. **Answers grep cannot give.** Units, not lines: `outline <path>` (a file's symbols with spans), `def <name>` (definitions with spans), `context <name>` (a definition and its name-matched references as source spans, the thing an agent does today with a grep, a glob and several reads). These replace read-the-whole-file with read-this-span, which is where the performance is; racing ripgrep is not.
2. **Zero-cost reach.** The MCP tool is in-process over SQLite; `.mcp.json` naming `intent mcp` is canon, seeded by `claude upgrade --apply`; the tool descriptions say when to use the tool and when not, in the words a model matches on, generated from the register.
3. **The canon routes the question.** CLAUDE.md, the skills and the generated guide name the index verbs for finding code and for the Highlander check, with grep as the named fallback when the envelope is not complete.
4. **The hooks make the harness's own search a door into the index.** A PostToolUse hook on grep, served from the install like the session hooks, appends the index's structural answer for the symbol the pattern named. It never blocks and it never lies, because grep still ran; within a session the model learns the better first call. The PreToolUse redirect of symbol-shaped patterns is the stronger form and is safe only under the contract below; it is specified and not built until hv rules.
5. **The contract: as fresh as grep at query time, or say so.** The envelope's `complete`, `skipped` and `stale` are what make the tool trustable; the hook appends nothing when they say the index cannot answer.

Next gen, beyond parity: code joined to intent. The store knows which commits reference which threads and which criteria a package carries; a symbol hit that names the thread and criterion that introduced it is traceability no code search has, and it is uniquely Intent's to build. It is WP-24's stretch criterion, specified before it is built.

## Build plan

- **Ships in 3.0.2** (hv, 2026-09-12: _"this is ALL for 3.0.2"_). Each package lands on main as it closes, red before green, its own commits; the release's second dry-run rehearsal and the cut follow the last package. Batch 4 of the release finishes first on every lane.
- **Lanes.** cc, the engine: WP-18, WP-20's tree-sitter integration, WP-23. dc, the daemon and the install: WP-22, WP-24's hook and canon halves, and WP-20's per-grammar binary-size measurement. ic, the surfaces: WP-17, WP-19, WP-21, WP-24's verbs, descriptions and skills. vc specifies, sequences and verifies every landing; hv rules the decisions listed above as each comes due.
- **Order.** WP-17 and WP-18 first, in parallel. WP-19 on WP-18. WP-20's pure symbols module against fixtures in parallel with WP-19, its integration on WP-19. WP-21 and WP-22 on WP-19. WP-24 on WP-20. WP-23 last.
- **Every package:** its own worktree with the worktree's in-tree target dir, an isolated HOME, tests on fixture trees and never on the estate they run in, red before green, no counts anywhere a reader reads, the register row in the same commit as any flag, and every `--help` or output change reported to ic for the reference.
