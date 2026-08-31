---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 07:45Z
status: active
focus: "Ninth fold, pre-compact on hv's call. BOTH MCP halves are LANDED: db3f947a (the serving match, 58 arms, two-sided gate) and fa2c3d36 (the stdio server, ZERO-DEP by vc's 2026-08-31 ruling on the design's own cited evidence). The BOUNCE builds the GraphQL escape hatch -- hv ruled it IN before the tag; AC-09.2 is mine; vc's bounds in TODO 0."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260831/wip-fold-0722Z.md` (ninth fold). Cold-session minimum: state, not story. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree.**

## DOING

**LANDED THIS SESSION, both through full gates:** `db3f947a` -- serve() over all 58 exposed rows, two-sided gate (SERVED==tools() both directions + in-memory drive + negative control), keep-only filter with positive control, unknown params refused by name, agents facade gap closed byte-identical (4/4 baselines), 3 more terminal-channel narrows, 52 roster-guard CheckedBy declarations. `fa2c3d36` -- the stdio MCP server: newline JSON-RPC, tools/call opens a fresh facade per request through the one door, notification silence decided by absent id, Answered::Value|Refused (refusal is DATA on the agent channel -- the critic gate refused Result<Value,String>, right for the 4th time), driven end-to-end (real binary, real project, 2 integration + 8 unit tests), two specimen successions (retirement_is_enumerable COMING-SOON arm re-read; session_hook_lockout marker specimen mcp -> st bootstrap).

## TODO

0. **THE BOUNCE: BUILD THE GraphQL ESCAPE HATCH (AC-09.2, mine).** hv ruled it IN before the tag (first-hand in vc's session, over vc's descope recommendation; menu + measurement on hv's board). vc's bounds, ruled under the pen 2026-08-31: **READS ONLY** (Query: thread/threads/issue/issues -- graphql.rs is a 221-line declared stub, all resolvers through one shared unwired()), derived from ONE roster, **mutations OUT of 3.0.1** (a GraphQL mutation path = second home for the serve() roster); resolvers reach the store through MY facade seam, never a hand-written map beside it. **THE CONSTRAINT TO VERIFY FIRST, before writing a line: intent_graphql must NOT drag a runtime into intent-cli** -- async-graphql needs tokio; the ruled route is BRIDGE TO INTENTD (which has tokio), per AC-00.4's "bridging to intentd when it is up". Expected open question: intentd-not-running must REFUSE with remedy naming `intent daemon start` (the §9 precedent), never hang, never degrade to an in-process executor (= the runtime through the back door). **If the bridge route cannot hold reads-only usefully, GO BACK TO vc BEFORE BUILDING** (their explicit ask).
1. **The 7 `claude subagents` narrowing rows** -- dc's prune-with-census first, CLI reshape behind it; dc NAMES THE HOUR the --kind verbs land (their commitment, don't poll).
2. **flag_reachability's INHERITED_UNREAD trio** (st bootstrap --audit-only/--dry-run/--deliverable): grandfathered as violations of an UNBUILT VERB because unwired_families() keys deferral on the FAMILY and st is wired. cc measured + routed to vc as a finding; cc suggests the exhaustive-match treatment (their export_round_trip precedent). MINE to fix once vc routes it.
3. **AC-09.6 satisfy** waits on hv's class decisions minus what landed; vc carries the list. vc accepted db3f947a's serving match as landed without re-read.
4. Standing queue: 0142 structural half (dc's G/H census is evidence, guide.rs write is mine; dc offered the declared/undeclared-but-consistent/contradicting split shape); TUI remainder (status picker, EMBED pty, intent edit wiring -- hv drives); AC-17.1 browser realiser; AC-17.10; WP-16; ST0064 parked; scratchpad/wt-tui worktree removal.

## Watch-outs -- mechanisms only

1. **jq `//` SWALLOWS false** -- use `has()` for booleans.
2. **REACH IS NOT A DOOR** -- the serving match + two-sided gate is the permanent discriminator.
3. **A PREDICATE THAT CANNOT MATCH ITS SUBJECT RETURNS THE NUMBER THAT MEANS SUCCESS** -- positive-control the FILTER (schema()'s keep-only now carries one at both poles).
4. **HIDE-CLASSIFY (vc)**: terminal-channel / honest-refusal / defect; only the first two hide without a filing.
5. **syn IS THE FLOOR for span-finding on render.rs.**
6. **NEVER `git stash` ON THIS FIVE-WRITER TREE** (bit once, survived by luck); a file held dirty can have HEAD move under it -- rebase YOUR delta onto THEIRS via 3-way apply, verify their lines survive. Peers' index.lock: NEVER remove; wait with an until-loop.
7. **STALE SIBLING intentd**: cc's a3b8aa60 makes RealDaemon REFUSE a sibling older than intentd/intentsvcs sources, naming the file -- `cargo build -p intentd` after touching intentsvcs is the habit (I trip it first, cc's words). A stale binary reports itself as a defect in someone else's key.
8. **ABSOLUTE PATHS** (cwd resets); zsh: unquoted `$var` does NOT word-split; quote `echo ===`.
9. **ANNOUNCE ANY DISK->STORE SYNC FIRST; `intent st attach` is the surgical verb.**
10. **dc's bin/ prune (their shape, corrected by them): the helper carry is IN the prune commit -- there is NO dark window; a red tests.yml at ANY point in that sequence is a DEFECT to REPORT to dc, never expected shape to absorb.** Post-prune, tests.yml and rust.yml test genuinely different things.
11. **The two full-suite runs can exceed 10m under contention** -- background them; 78 suites green at fa2c3d36.

## Decisions

- **2026-08-31 vc (under hv's pen, in ic's channel): 3.0.0 MCP stdio server is ZERO-DEP; rmcp stays ratified for streamable HTTP 3.x.** Deciding fact: the design's own cited Lamplight proof point measures as a depless loop. Discharge: routes through dispatch(op)/intentd. ratified_in recorded in mcp_stdio.rs's header verbatim.
- **2026-08-31 hv (via vc, first-hand): the GraphQL escape hatch is IN, before the tag; reads-only per vc's bounded reading** (TODO 0 carries the bounds).
- **2026-08-30 vc LIMIT: MCP serves in-process only; discharge = dispatch(op) routing** (in mcp.rs's header).
- **2026-08-30 vc + cc: row-level exposed_on_mcp REFUSES on absence; flag-level defaults TRUE** (doc'd in dispatch.rs).
- **2026-08-30 hv (standing, via dc): claude subagents rows NARROW; --kind lifecycle is the wiring path.**
- **2026-08-30 ic (at c5d66741): Esc never quits; quit is an act.**
