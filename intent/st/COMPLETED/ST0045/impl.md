# Implementation - ST0045: Update whiteboard for per-workstream files

## Implementation

As-built for the Protocol 3.0 contract + drift-closure work. The skill body itself was rewritten before this contract existed (`f021818`, `b66484b`); this thread proves it complete against a ratified AC set, closes the reference-vs-skill drift, and documents the four under-specified corners.

### Skill completeness additions (`intent/plugins/claude/skills/in-whiteboard/SKILL.md`)

- **AC-01.2 -- `.history/.gitkeep`**: the file-layout block now shows `.history/.gitkeep` (git does not track an empty directory) above the `YYYYMMDD/` buckets, plus a "Scaffolding a node" note: create `<node>/`, an empty `.history/.gitkeep`, and `wip.md`; inboxes are not pre-created.
- **AC-01.1 -- inbox init**: new `## inbox.<sender>.md shape` section -- one inbox per ordered (sender -> recipient) pair, the `# inbox: <sender> -> <recipient>` header, the single-writer/single-reader rule, and the `_(empty)_` sentinel that `clear`/`archive` leave behind so an inbox is never an ambiguous zero-byte file. `ask` step 1 now states it creates an absent inbox on first send and replaces a lone `_(empty)_` with the first entry.
- **AC-01.4 -- message-entry format**: new `### Message-entry format` subsection -- REQUIRED: the `## (YYYY-MM-DD HH:MM)` minute-granularity timestamp heading (doubles as the reply anchor) + the `<text>` body; RECOMMENDED/optional: `Re: <prior-anchor>` (threading) and `FYI only -- no response needed.` (absent => reply expected). A reply is a new entry in the opposite-direction inbox.
- **AC-01.3 -- the `hv` node**: new `### The hv (hypervisor) node` subsection -- structurally a node, with three human-driven differences: no `/in-session` loop (so `session_id` optional/`none` and never matched on the active-peer test), advisory heartbeat (exempt from the 7-day reclaim rule), and an optional `## Standing directives` section peers read at pickup like `## Decisions`.

### Drift closure -- chaining skills + canon doc (AC-02.2)

The 2.0 flat-stream vocabulary survived in three shipped surfaces; all rewritten to the per-node model:

- `intent/plugins/claude/skills/in-session/SKILL.md` step 5 -- "reads stream files, surfaces other-stream state" -> "reads your node's board + inboxes, surfaces peer-node state".
- `intent/plugins/claude/skills/in-finish/SKILL.md` step 1 + skill-chain line -- "your stream's `status: paused`" / "your stream file's `## Recent decisions affecting other streams`" / "whiteboard stream" -> node-board language (`## Decisions`, "whiteboard node").
- `intent/docs/working-with-llms.md` "Multi-session coordination" section -- the whole 2.0 layout (`<stream>.md`, shared `asks.md`, `<platform>.md`, `history/<YYYYMMDD>.<file>`) replaced with the per-node directory layout, a new "Protocol 3.0" subsection, "Node identity" (incl. `hv`), `announce`-based shared-platform coordination, and node/`hv` heartbeat semantics. Reference now points at both ST0040 (2.0 rationale) and ST0045 (3.0 rewrite).

## Code Examples

The mechanical guard, `tests/unit/whiteboard_protocol_3_guard.bats`:

- `TWO_OH_TOKENS='[^[:alpha:]]asks\.md|lamplight\.md|cookies\.md|per-stream|stream file|other-stream|<stream>\.md'` -- the retired-model token set; `asks.md` is boundary-guarded so it does not match the `tasks.md` substring.
- A line carrying a 2.0 token is a violation UNLESS it also carries a retired marker (`retired|supersede|legacy|2\.0`) -- that exception is what lets the docs name the old model historically while banning live use of it.
- AT-02.1 scans `in-whiteboard/SKILL.md`; AT-02.2 asserts the chaining skills name the 3.0 subcommands (`/in-whiteboard pickup`, `/in-whiteboard release`) AND that none of `in-session/SKILL.md`, `in-finish/SKILL.md`, `working-with-llms.md` hard-codes the flat model.

## Technical Details

- Guard scope for AT-02.2 is the three live-reference surfaces (the two chaining skills + the canon narrative), not a blind tree-wide grep: ST archives under `intent/st/COMPLETED/ST0040/` legitimately describe the 2.0 model and must not trip the guard. The in-whiteboard SKILL.md has its own guard (AT-02.1) with the retired-marker exception.
- Skill-file discipline held: no em dashes (`--` throughout), `eg` not `e.g.` in the edited skills. Em dashes are used only in `working-with-llms.md` (a doc, not a list-rendered skill), matching that file's existing style.

## Challenges & Solutions

- **Red-first vs already-built skill.** The 3.0 skill body predated the contract, so AT-02.1 was green on arrival (a regression guard, not a driver). AT-02.2 supplied the genuine red-first signal because the chaining skills + canon doc still carried 2.0 vocabulary. Documented as such rather than manufacturing artificial red.
- **`asks.md` substring trap.** The first guard run flagged `tasks.md` lines (in-finish, working-with-llms) as false positives. Fixed with a non-alpha boundary guard on the `asks.md` token; re-ran to a clean red showing only the genuine flat-model offenders before building to green.
