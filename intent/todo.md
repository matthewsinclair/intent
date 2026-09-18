# TODO

A DOING / TODO / DONE view, projected from steel-thread and work-package status: one row per steel thread, with its work packages nested beneath it. Generated -- change a status with the CLI, never by editing this file.

## DOING

- [-] ST0056: Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent
  - [x] 01: Design canon: architecture, data model, migration and parity specs
  - [x] 02: Workspace and reified model: intentsvcs types, schema faces, store
  - [x] 03: Ingest, views and sync engine
  - [x] 04: intentsvcs facade: core command families
  - [x] 05: CLI in-process mode and BATS conformance harness
  - [x] 06: CLI parity long tail
  - [x] 07: Canon and claude subsystem
  - [x] 08: intentd daemon
  - [x] 09: MCP server and agent guide
  - [x] 10: Migration and fleet ingest harness
  - [-] 11: Distribution: Homebrew tap, signing, notarisation
  - [x] 12: Cutover and v3.0.0 release
  - [~] 13: Project search: full-text, structural, and the agent search surface
  - [~] 14: Coordination model: whiteboard and inboxes in the store, with a bounded API
  - [x] 15: Skills catalogue triage: KEEP, UPDATE or RETIRE every Intent2-era skill
  - [~] 16: Contract drift: a shipped field with no model row is refused
  - [x] 17: Form DSL: one declaration, TUI and WEB realisers, CRUD through intentsvcs
  - [x] 18: Store growth: the doc-section index duplicates on every mutation, and explore gets a progress indicator
- [-] ST0078: Using Intent on a multi-person project with a Git workflow including PRs
  - [x] 01: P1: the event log travels -- one committed file per event under intent/.canon/events/YYYY/MM/DD, additive ingest, principal is the author (reverses D53)
  - [-] 02: P2: renumber verbs -- intent st renumber and intent issues renumber repair an id two clones both minted
  - [x] 03: P3: the store after a pull -- store-stale shown on a default doctor run, a CLI door for the non-destructive ingest, post-merge/post-checkout/post-rewrite hooks wired by claude upgrade --apply
  - [-] 04: P4: working in a team -- the docs/concepts page written from driven commands, the-store.md corrections, a CI doctor job on the merge result
  - [x] 05: P5: one command after a pull -- bare intent sync prints the plan for this clone, --apply applies it, --to-disk and --to-store keep their meanings; quiet, reversible and non-reversible steps, --yes for the reversible asks, the hooks run the quiet subset

## TODO

- [?] ST0060: Add 'intent vault' as a way for intent projects to manage local credentials
- [?] ST0077: Level 3 stores the references only the toolchain sees, and a roster of the targets it defines

## DONE:2026-09-10T00:00:51Z

- [~] ST0046: Add modules (properly) to the intent cli
- [x] ST0057: Disk as a sparse projection of the store
  - [x] 01: Canon relocation to intent/.canon/
  - [x] 02: .intentfiles: the manifest and its refusing grammar
  - [x] 03: Attachment canon: opaque as file, and the naming gate
  - [x] 04: intent organize: four answers, one refusal, one gate
  - [x] 05: intent edit <ID>
  - [x] 06: The full text realisation, and the refusal that expires
  - [x] 07: intent:// addressing and read resolution
  - [x] 08: The mutation surface: write-by-address and the missing verbs
  - [x] 09: The event log records the model and not the disk
  - [x] 10: doctor reports a dehydrated view as missing
  - [x] 11: Default disposition realises WIP threads only: organize --default writes .intentfiles; init, migration and upgrade share the function
  - [~] 12: Issues get a realised form and a sigil, then join the default declaration
  - [~] 13: The v2 tree survives migration and disagrees with the store: ingest bucket files as attachments, then remove the bucket
  - [x] 14: The DONE cutoff is canon state, not history: it travels by git
- [x] ST0058: Local cutover: v3 usable across the estate before it is releasable
  - [x] 01: One path to the native CLI: int cli carries the coherence guard
- [~] ST0059: Rebuild all Intent docs for v3
- [x] ST0061: dehydrate
- [~] ST0062: help
- [~] ST0063: start
- [x] ST0064: The Intent menubar app: controlling intentd from the macOS menubar
  - [x] 01: Menubar app: control of intentd after Geodica's design -- no product logic, every control an intent verb, the daemon is the authority on the daemon
  - [~] 02: CLI shell-out foundation and login-shell PATH capture (AC-01.1, AC-01.9)
  - [~] 03: Daemon health predicate, three endpoint states, and the turtle menubar icon (AC-01.2, AC-01.3, AC-01.6, AC-01.8)
  - [~] 04: The tailing console and its tail-orphan trap (AC-01.4)
  - [~] 05: The intent:// URL handler as a client of the one resolver (AC-01.5)
  - [~] 06: The int macos build, run, test, install and notarise pipeline (AC-01.7)
  - [~] 07: The app shell: menubar menu, settings window, and app entry
- [x] ST0065: Review Intent's use of .md files and /in-* skills for the v3 release
  - [x] 01: Root-and-branch review and rationalisation of the Intent-canon .md bootstrap set
  - [~] 02: Audit and cleanup of the /in-* skills: retire the senseless, correct the wrong, rewrite the misguided
- [x] ST0068: Update Intent's docs for v3
- [x] ST0069: v3 post-cut: project search, store-backed coordination, and contract drift
  - [x] 01: Issues get a realised form and a sigil, then join the default declaration
  - [x] 02: The v2 tree survives migration and disagrees with the store: ingest bucket files as attachments, then remove the bucket
  - [~] 13: Project search: full-text, structural, and the agent search surface
  - [x] 14: Coordination model: whiteboard and inboxes in the store, with a bounded API
  - [x] 16: Contract drift: a shipped field with no model row is refused
  - [x] 17: The structured query door: intent search --sql, read-only over the published schema
  - [x] 18: The corpus: the gitignore-aware repository, two staleness policies, the widened watcher
  - [x] 19: Lexical search over the whole corpus: the envelope, --json, the MCP tool, intent index
  - [x] 20: Structural search: tree-sitter symbols per declared language, and the agent canon that uses them
  - [x] 21: The explorer's /search pane
  - [x] 22: Daemon-served search with daemonless parity
  - [x] 23: Semantic seams: the embedder interface, the Null and HTTP embedders, the vector schema
  - [x] 24: The LLM boundary: the harness's own search becomes a door into the index
- [~] ST0070: LLM config rationalisation -- one home per fact
- [x] ST0074: Machine-wide projects: intentd's config home, the project registry and discover, the explorer's project picker, and the menubar status line
  - [x] 01: The explorer handles /threads and /issues itself
  - [x] 02: The menubar's one status line, in Gtools' shape
  - [x] 03: The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file
  - [x] 04: The explorer's project picker: /projects, and intent explore outside a project
  - [x] 05: Where intentd's durable configuration lives: a standards-compliant home, ruled by hv
- [x] ST0075: The Intent.app Console: daemon logs and one-off verbs in one window, copied from Gtools
  - [x] 01: The verb: intent daemon logs, with --lines and --follow, its register row and tests
  - [x] 02: The window: the Console copied from Gtools, the palette, Console on Cmd-L, Close and Clear Console
  - [x] 03: The streaming items: Run Doctor and Rebuild Search Index into the Console; Start, Stop and Restart noted there
- [x] ST0076: A typed symbol index for Rust and Elixir: kinds, containers, qualified references and resolved references
  - [x] 01: Typed definitions: kind, container, arity and span, one row per syntax node, with an extractor version
  - [x] 02: Rust qualified references: scoped calls, type uses and macro token trees, with the qualifier
  - [x] 03: Elixir qualified references: remote calls with their module, captures, pipes, alias expansion, use, import and require
  - [x] 04: Surfaces: search by kind and container, qualified context naming its level, SQL columns, MCP parameters and instructions
  - [x] 05: Rust resolved references through rust-analyzer's SCIP export
  - [x] 06: Elixir resolved references through the compiler's tracer, on an explicit verb
  - [x] 07: Level-3 surfaces: resolution in the envelope, resolved facts on a hit, and search by target

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
