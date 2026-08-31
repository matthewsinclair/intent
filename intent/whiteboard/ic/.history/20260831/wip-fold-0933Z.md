---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 09:25Z
status: active
focus: "The GraphQL escape hatch is LANDED at dbfc1eb1, reads-only under vc's bound, both faces bridging to intentd with the CLI still linking no runtime. TODO 0 is done; conservation_check.sh sees .canon (8f29d3a6) and its DOUBLED counter fires (5cbddfcc: 194/192/0/0, prediction held); schema is on the MCP tier (0a6f7784, roster 60). Nothing is queued for me by vc; the remaining TODO items are gated on others."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260831/wip-fold-0722Z.md` (ninth fold). Cold-session minimum: state, not story. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree.**

## DOING

**LANDED ON THE BOUNCE, through full gates:** `dbfc1eb1` -- the GraphQL escape hatch (AC-00.4's bridging clause, AC-09.2 under vc's bounded reading). `Op::Graphql`/`Response::Graphql` on the wire; four resolvers over a snapshot taken through `st_list()`/`issue_list()` by a real `Facade::graphql`; intentd's store thread executes on tokio's own handle; `intent graphql <query> [--variables <JSON>]` and the MCP tool `intent_graphql` are ONE round trip in `hatch.rs`, refusing at rc=2 naming `intent daemon start` when no daemon answers; `EmptyMutation` is the reads-only enforcement, refusals inside `errors` at rc=1. Driven both faces against a real intentd (`graphql_escape_hatch.rs`, 5 arms; `graphql_answers_through_the_store_door.rs`; `graphql_reads_through_the_facade.rs`). **Verified before building: `cargo tree -p intent-cli -i tokio` matches no package, before and after.** AT-09.2 green, cited, note carries the bound verbatim. `97f2322a` ahead of it: two intentsvcs test binaries had not compiled since db3f947a (my Install/RootFile variants without arms) -- found by `cargo check --workspace --all-targets`, which now precedes every landing of mine.

**LANDED `8f29d3a6`:** `conservation_check.sh` sees `intent/.canon/` again -- two roots (CANON = `.canon`, REALISED = the intent dir), the five collectors on the flat file, the pre-relocation layout refused BY NAME (vc ratified cc's property as a ruling). Driven in scratch: OLD tool rc=2 `converted 0` (the control); FIXED tool converted 344 / STRANDED 386 / ALTERED 114 / ADDED 0 / 1370 byte-identical / liveness ok over 56 -- cc's numbers to read against AT-10.5. Canon moved with it (`intent st attach`, the order canon_commit_check prescribes).

**LANDED `0a6f7784`:** `schema` on the MCP tier -- `faces::schema` the store-free one home (the verb is GLOBAL, so its terminal arm can open no facade), `Facade::schema` beside it as the tier's door, `NoSuchFace` raised for real with a remedy that names the faces, roster 60. hv's ruling: build `schema`, defer eleven. **LANDED `5cbddfcc`:** DOUBLED asked the REALISED tree and could not fire (the migrated canary realises ONE flat info.md of 55); it asks CANON now -- generated names DOUBLED via the thread record, authored names RELOCATED via attachment sha256, new ALTERED-ATTACHMENT, STRANDED only when canon carries nothing. Predicted 194 / ~162+ / 0 before running; measured DOUBLED 194, RELOCATED 192, ALTERED 0, STRANDED 0 = 386. Controls both ways on ST0001/design.md (bucket sha == canon sha, established first). AC-10.5's live residue is the prose arm's ALTERED 114 -- cc's.

## TODO

1. **flag_reachability's INHERITED_UNREAD trio** (st bootstrap --audit-only/--dry-run/--deliverable) -- MINE once vc routes it; cc suggests the exhaustive-match treatment.
2. **AC-09.6 satisfy** -- waits on hv's class decisions minus what landed; vc carries the list. **hv ruled the eleven unwired MCP rows NARROW as one flip** (vc has it in their working copy, landing on top of my row).
3. **The 7 `claude subagents` narrowing rows** -- behind dc's prune-with-census; dc NAMES THE HOUR the --kind verbs land.
4. **For vc's globalfold, flagged not owned (vc has both queued):** design.md:88's `rmcp ... stdio ... now` against the 2026-08-31 zero-dep ruling; AC-09.2's unbounded wording (`ac edit` is vc's). AT-00.4 (vc's `mcp_surface.rs`) can cite `graphql_escape_hatch.rs` for the bridging clause.
5. Standing queue: 0142 structural half (guide.rs write is mine); TUI remainder (AC-17.1 browser realiser, AC-17.6 -- hv drives); WP-16; ST0064 parked; scratchpad/wt-tui worktree removal.

## Watch-outs -- mechanisms only

1. **`cargo check --workspace --all-targets` BEFORE EVERY LANDING.** `cargo test -p intent-cli` builds the CLI's tests and not intentsvcs's; two exhaustive-match test binaries there sat uncompilable for a day behind a "78 suites green".
1. **A PEER'S UNCOMMITTED HUNK IN A FILE YOU HOLD DIRTY: stage YOURS through a temporary index** (`GIT_INDEX_FILE=… git read-tree HEAD; git apply --cached mine.patch; git add …; git commit`). `--only` is path-scoped and would commit theirs under your name; `checkout HEAD -- file` would destroy theirs. Split the diff by hunk content first.
1. **A path-scoped `git add` of shared canon carries EVERY node's store writes** -- caef64a4 carried three nodes' AT rows; HEAD then cited a test file that was untracked until its author landed. Land the file the canon cites promptly, and say who carried the hunk.
1. **The dispatch-table generator keeps LIVE CENSUSES in prose** (`status`'s new-surface count, `legal_pairs[].n`) and refuses on each in turn -- a new row moves both; the `why` block's counts were reworded count-free.
1. **rustfmt touching intentd/intentsvcs sources makes the sibling intentd STALE to RealDaemon** -- `cargo build -p intentd` after the formatter too, not only after edits.
1. **ABSOLUTE PATHS: the Bash tool's cwd persists ACROSS calls**, so a `cd` in one call moves every later relative path -- the full-suite run once started in the project root and found no Cargo.toml.
1. **jq `//` SWALLOWS false**; zsh: unquoted `$var` does NOT word-split, `$PIPESTATUS` is bash (zsh spells it `$pipestatus`).
1. **REACH IS NOT A DOOR** -- the serving match + two-sided gate is the permanent discriminator (59 tools now).
1. **HIDE-CLASSIFY (vc)**: terminal-channel / honest-refusal / defect; only the first two hide without a filing.
1. **NEVER `git stash` ON THIS FIVE-WRITER TREE; never remove a peer's index.lock** -- wait with an until-loop.
1. **ANNOUNCE ANY DISK->STORE SYNC FIRST; `intent st attach` is the surgical verb.**
1. **A red tests.yml at ANY point in dc's prune sequence is a DEFECT to report**, never expected shape.

## Decisions

- **2026-08-31 vc RATIFIED the DOUBLED predicate at 5cbddfcc, point for point:** prediction recorded before the run (194 / ~162+ / 0 -> 194 / 192 / 0 / 0); the whole 386 re-derived against canon rather than subtracted from; both controls on the file vc named, established BEFORE the predicate was touched; ALTERED-ATTACHMENT kept as its own class. cc's AT-10.5 hypothesis was asking about the wrong population; attachment residue is zero and the prose arm's ALTERED 114 is the first real number that row has had. The clean-landing technique (blob from `git show HEAD:`, md regenerated from it, semantic diff, `restore --staged` every committed path) is now the estate's.
- **2026-08-31 ic (under vc's bound, hv's ruling): the hatch is DAEMON-ONLY -- `graphql` is NOT in `SERVED_BY_DAEMON` (no in-process twin, so the AC-08.2 identity claim cannot be made for it) and is declared at `hatch::DAEMON_ONLY`; the roster's membership rule now carries the twin clause.** `--variables` is JSON TEXT on both faces (one row, one type). `Facade::graphql` is a real method so the row's `facade: "graphql"` is true. tokio is an intentsvcs DEV-dependency only, for the resolver test.
- **2026-08-31 vc (under hv's pen, in ic's channel): 3.0.0 MCP stdio server is ZERO-DEP; rmcp stays ratified for streamable HTTP 3.x.** ratified_in recorded in mcp_stdio.rs's header verbatim.
- **2026-08-31 hv (first-hand in vc's session): the GraphQL escape hatch is IN, before the tag; reads-only per vc's bounded reading.**
- **2026-08-30 vc LIMIT: MCP serves in-process only; discharge = dispatch(op) routing** (in mcp.rs's header).
- **2026-08-30 vc + cc: row-level exposed_on_mcp REFUSES on absence; flag-level defaults TRUE.**
- **2026-08-30 hv (standing, via dc): claude subagents rows NARROW; --kind lifecycle is the wiring path.**
- **2026-08-30 ic (at c5d66741): Esc never quits; quit is an act.**
