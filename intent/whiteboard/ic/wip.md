---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 09:39Z
status: active
focus: "Tenth fold, pre-compact on hv's call. The bounce landed everything it carried: the GraphQL escape hatch (dbfc1eb1), schema on the MCP tier (0a6f7784), conservation_check sighted and its counter firing (8f29d3a6, 5cbddfcc; the --dispositions usage fix rode in under cc's 71539ab4 -- the shared-index sweep, carrying rather than losing), two test binaries recompiling (97f2322a). Nothing is queued for me by vc; every TODO item is gated on another node or on hv. On the bounce: re-measure, then take whatever vc routes."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260831/wip-fold-0933Z.md` (tenth fold). Cold-session minimum: state, not story. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree.**

## DOING

**NOTHING IN FLIGHT.** Landed this session, all through the full gate: `dbfc1eb1` the GraphQL escape hatch, reads-only under vc's bound, both faces (`intent graphql`, MCP `intent_graphql`) bridging to intentd over `wire::ask`, daemon-down refusing at rc=2 naming `intent daemon start`, CLI still linking no runtime (`cargo tree -p intent-cli -i tokio` matches nothing), AT-09.2 green and cited; `97f2322a` two intentsvcs test binaries uncompilable since db3f947a; `8f29d3a6` conservation_check.sh sees `intent/.canon/` (two roots, pre-relocation layout refused BY NAME -- vc's ruling); `5cbddfcc` its DOUBLED counter asks canon and fires (194 / 192 / 0 / 0 = 386, predicted before the run, controls both ways -- vc ratified point for point); `71539ab4` (cc's landing of their AT-10.5 pair, which swept my unstaged tool and its canon fields -- the work is in HEAD, the attribution is theirs) `--dispositions` made discoverable (114 -> 2 with the upgrade's own stdout, cc's join); `0a6f7784` `schema` on the MCP tier via `faces::schema` + `Facade::schema`, roster 60, `NoSuchFace` raised for real.

## TODO

1. **flag_reachability's INHERITED_UNREAD trio** (st bootstrap --audit-only/--dry-run/--deliverable) -- MINE once vc routes it; cc suggests the exhaustive-match treatment.
2. **AC-09.6 remainder** -- vc carries the list; hv ruled the eleven unwired MCP rows NARROW as one flip (vc's landing) and the twelve facade gaps BUILD `schema` ONLY, defer eleven (done).
3. **The 7 `claude subagents` narrowing rows** -- behind dc's prune-with-census; dc NAMES THE HOUR the --kind verbs land.
4. **Placed by others, watch only:** the ingest-side duplicate-heading class (issue 0059 carries two `## Related`, canon keeps one -- the twin of DOUBLED-SECTION; cc/vc placing it); design.md:88's stale rmcp line and AT-00.4 citing `graphql_escape_hatch.rs` for the bridging clause (vc's globalfold); the delivered binary 16 files behind HEAD (dc's lane -- `int macos publish` refuses it; the PATH `intent` still carries the current TUI, nothing under `tui/` moved since 62d2d633).
5. Standing queue: 0142 structural half (guide.rs write is mine); TUI remainder (AC-17.1 browser realiser, AC-17.6 edit/browse one model -- hv drives); WP-16; ST0064 parked; scratchpad/wt-tui worktree removal.

## Watch-outs -- mechanisms only

1. **`cargo check --workspace --all-targets` BEFORE EVERY LANDING** -- `cargo test -p intent-cli` builds only the CLI's tests; two intentsvcs binaries sat uncompilable for a day behind a green.
2. **A PEER'S UNCOMMITTED HUNK IN A FILE YOU HOLD DIRTY: stage YOURS through a temporary index** (`GIT_INDEX_FILE=… git read-tree HEAD; git apply --cached mine.patch; git add …; git commit`), build shared-JSON blobs from `git show HEAD:` + your edit and regenerate the md from that, diff the table SEMANTICALLY against HEAD before staging, then `git restore --staged -- <every committed path>` -- created files otherwise read as staged DELETIONS and modified files as staged REVERSIONS of your own change (0178/0179).
3. **A path-scoped `git add` of shared canon carries EVERY node's store writes** (caef64a4 carried three nodes' AT rows) -- land the file a canon row cites promptly, and name who carried the hunk.
4. **AN ATTACHMENT MOVES WITH ITS CANON**: parity tools under `intent/st/ST0056/` are ST0056 attachments; `intent st attach <ST> <path> --from <file>` FIRST (announce it), then tool + `intent/.canon/st/<ST>.json` in ONE commit -- canon_commit_check refuses the other order and the order it prescribes is the only one that leaves history consistent.
5. **The dispatch-table generator keeps LIVE CENSUSES in prose** -- `status`'s new-surface count, `legal_pairs[].n`, and a `recoverability_anomaly` that must exist exactly when the fields disagree -- and refuses on each in turn.
6. **MARKDOWN MUST BE PRETTIER-FORMATTED BEFORE `git add`** -- the gate refuses and REWRITES NOTHING; run `npx --no-install prettier --write <file>` then add.
7. **rustfmt touching intentd/intentsvcs sources makes the sibling intentd STALE to RealDaemon** -- `cargo build -p intentd` after the formatter too.
8. **ABSOLUTE PATHS: the Bash tool's cwd PERSISTS ACROSS CALLS**; a `cd` in one call moves every later relative path (a full-suite run once started where there was no Cargo.toml). zsh: unquoted `$var` does NOT word-split; `$PIPESTATUS` is bash. jq `//` swallows false.
9. **A COUNTER WHOSE ZERO IS IMPOSSIBLE ON ITS OWN SUBJECT IS THE VACUOUS GREEN ONE LAYER DOWN** -- predict the figure BEFORE the run, re-derive the whole population rather than subtract a sample, positive-control the predicate on a file established as carried INDEPENDENTLY and before the predicate is touched, and plant both failure directions so the zeros are shown able to fire.
10. **A FLAG THAT EXISTS AND IS NOT IN THE USAGE LINE READS AS A FINDING ABOUT THE ESTATE** (cc) -- `--dispositions` halved a residue nobody had passed it for.
11. **A WORKED EXAMPLE IN A REMEDY IS SHIPPED SURFACE** -- the example id is `ST0000`; `no_pm_state_in_output` refuses `ST0001`.
12. **REACH IS NOT A DOOR** -- the serving match + two-sided gate is the permanent discriminator (60 tools). **HIDE-CLASSIFY (vc)**: terminal-channel / honest-refusal / defect.
13. **A PEER'S PATH-SCOPED `git add <dir>` SWEEPS YOUR UNSTAGED FILE UNDER IT INTO THEIR COMMIT** -- the usage fix landed under cc's message while it sat modified-and-unstaged in the tools directory; keep a finished change staged in a temporary index or landed, never merely dirty. **NEVER `git stash` on this five-writer tree; never remove a peer's index.lock** -- wait with an until-loop; and grep-filtering a commit's output can swallow git's own "Unable to create index.lock", so check `git log -1` after every commit.
14. **A red tests.yml at ANY point in dc's prune sequence is a DEFECT to report**, never expected shape.

## Decisions

- **2026-08-31 vc RATIFIED the DOUBLED predicate at 5cbddfcc, point for point** (prediction first; whole population re-derived; controls both ways on the named file, established before the predicate moved; ALTERED-ATTACHMENT its own class). cc's AT-10.5 hypothesis was asking about the wrong population; artefact residue is zero; the prose arm's residue is 2 with `--dispositions`, and the two are one ingest-side defect (0059).
- **2026-08-31 vc: `schema` landed on "land" -- the effort-order ruling did not moratorium a finished, green change while six shared files sat dirty.** The clean-landing technique is the estate's.
- **2026-08-31 ic (under vc's bound, hv's ruling): the hatch is DAEMON-ONLY** -- `graphql` is NOT in `SERVED_BY_DAEMON` (no in-process twin) and is declared at `hatch::DAEMON_ONLY`; the roster's membership rule carries the twin clause (cc concurred). `--variables` is JSON TEXT on both faces; `Facade::graphql` is a real method; tokio is an intentsvcs DEV-dependency only.
- **2026-08-31 vc (under hv's pen): the pre-relocation canon layout is REFUSED BY NAME, never accepted** -- a fallback that passes an old tree as current turns a loudly-blind instrument into a silently-wrong one.
- **2026-08-31 vc (under hv's pen, in ic's channel): 3.0.0 MCP stdio server is ZERO-DEP; rmcp stays ratified for streamable HTTP 3.x.**
- **2026-08-31 hv (first-hand in vc's session): the GraphQL escape hatch is IN before the tag, reads-only per vc's bounded reading; the twelve facade gaps: build `schema` only.**
- **2026-08-30 vc LIMIT: MCP serves in-process only; discharge = dispatch(op) routing.** **2026-08-30 vc + cc: row-level exposed_on_mcp REFUSES on absence; flag-level defaults TRUE.** **2026-08-30 hv (via dc): claude subagents rows NARROW.** **2026-08-30 ic: Esc never quits; quit is an act (`:q`).**
