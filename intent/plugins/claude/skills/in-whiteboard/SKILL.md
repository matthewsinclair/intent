---
description: "Multi-session coordination via intent/whiteboard/<node>/: per-node boards + single-writer inboxes, claim ST scopes, broadcast, heartbeat, release"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination (Protocol 3.0)

Coordinator for multiple Claude Code sessions -- and the human -- running concurrently against one Intent project. Each participant is a **node** with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what makes the board contention-free and cleansable. The whiteboard is the _live_ channel; `intent/wip.md` is the post-session snapshot.

**Protocol 3.0** supersedes 2.0 (flat shared `asks.md` + per-stream files). v3.0 = per-node directories + a single-writer inbox model + the human as a first-class `hv` (hypervisor) node.

## Nodes

A node is a participant. The 2-letter moniker is the directory name, the routing key, and the handle. Nodes are **per-project configuration**: the project declares its roster (monikers, display names, roles) in its hand-authored `intent/whiteboard/README.md`. No roster is baked into this skill -- it discovers nodes by listing the immediate subdirectories of `intent/whiteboard/`.

A project that wants the human in the loop gives them a node, conventionally `hv` (the **hypervisor**): the human who adjudicates scope, sequences work, owns releases, and is where escalations land. The human is addressed as `hv` in all protocol language, never by name. The hypervisor node is human-driven -- it is read like any other node, but the human maintains it (or has it maintained on their behalf) rather than running `pickup` on a heartbeat.

### The hv (hypervisor) node

`hv` is structurally a node like any other -- a `<hv>/wip.md` peers read at pickup, inboxes peers append to -- with three differences that follow from being human-driven:

- **No session loop.** `hv` is not driven by `/in-session` / `pickup`, so its `session_id` is optional and conventionally `none`. Peers therefore never match it on the "different `session_id`" active-peer test; they read it for its directives and route escalations to `hv/inbox.<you>.md`.
- **Heartbeat is advisory.** A stale `hv` heartbeat does not mark anything reclaimable -- the human is always authoritative -- so the 7-day reclaim rule does not apply to `hv`.
- **Standing directives.** Beyond the canonical `wip.md` body, `hv` may carry a `## Standing directives` section: durable instructions every node honours (sequencing, scope rulings, release policy). Peers read it at pickup the way they read `## Decisions`.

## When to invoke

- `pickup` -- chained from `/in-session`; read own board + own inboxes + peer state, touch heartbeat.
- `ask <node> <text>` -- send a point-to-point message to another node.
- `announce <text>` -- broadcast one line to every peer (eg before touching a shared platform layer).
- `decide <text>` -- record a cross-node decision on your own board.
- `claim <STxxxx>` / `unclaim <STxxxx>` -- add/remove an ST from your board's claims.
- `clear <sender>` -- archive handled entries out of one of your own inboxes.
- `archive` -- roll your own DONE board content + handled inbox entries into your own history.
- `touch` -- refresh your heartbeat.
- `release` -- chained from `/in-finish`; set your status paused.
- `status` -- read-only one-line-per-node summary.

If invoked with no subcommand, default to `status`.

## File layout

```
intent/whiteboard/
  README.md                 # protocol reference + the project's node roster
  <node>/
    wip.md                  # the node's live board: frontmatter + DOING + TODO + watch-outs + decisions
    inbox.<sender>.md       # one per OTHER node: messages FROM that sender (single-writer)
    .history/
      .gitkeep              # tracks the otherwise-empty archive dir (git ignores empty dirs)
      YYYYMMDD/             # the node's archived DONE work + handled inbox entries
```

Scaffolding a node: create `<node>/`, an empty `<node>/.history/.gitkeep` (git does not track an empty directory, and the archive dir starts empty), and the node's `wip.md`. Inboxes are not pre-created -- the first sender creates `<node>/inbox.<sender>.md` on its first `ask` (see below).

Single-writer rule:

- `<node>/wip.md` -- written only by `<node>`.
- `<node>/inbox.<sender>.md` -- appended only by `<sender>`; read + cleansed only by `<node>` (the owner).

## wip.md shape

```yaml
---
node: <moniker>
name: <display name>
role: <role> # eg hypervisor | control | interface | validation | author
session_id: <UUID|none>
heartbeat_at: <ISO 8601>
status: active | paused
focus: "<one-line current goal>"
claims: [STxxxx, ...]
---
# <Name> (<node>)
## DOING        -- in-flight work (archived into .history/ when done)
## TODO         -- queued / next
## Watch-outs   -- durable cautions peers should know (standing; not archived)
## Decisions    -- cross-node decisions, broadcast by being read at pickup
```

Only the frontmatter is required for protocol compliance; the body sections are the working content.

## inbox.<sender>.md shape

One inbox per ordered (sender -> recipient) pair: `<recipient>/inbox.<sender>.md` holds the messages `<sender>` has sent `<recipient>`. The sender is the sole writer (append-only); the recipient is the sole reader and owns its lifecycle (read, action, `clear` into history).

Inboxes are created on demand, never pre-seeded: the first `ask` (or `announce`) creates an absent `<recipient>/inbox.<you>.md` before appending its first entry. A freshly created inbox is its header line plus the empty sentinel:

```
# inbox: <sender> -> <recipient>

_(empty)_
```

The `# inbox: <sender> -> <recipient>` header restates the single-writer routing the path already encodes, so the file is self-describing when read alone. `_(empty)_` is the "no live entries" sentinel: `clear` and `archive` leave the header + `_(empty)_` behind when they remove the last handled entry, so an inbox is never an ambiguous zero-byte file.

### Message-entry format

Each entry appended by `ask` / `announce`:

```
## (YYYY-MM-DD HH:MM)   [Re: <prior-anchor>]   [FYI only -- no response needed.]

<text>
```

Required fields: the `## (YYYY-MM-DD HH:MM)` timestamp heading (minute granularity -- it doubles as the anchor a reply threads against) and the `<text>` body. Recommended / optional: `Re: <prior-anchor>` (present only when threading a reply to a prior entry's timestamp) and `FYI only -- no response needed.` (present only when no reply is expected; absent means the sender expects a reply). A reply is a new entry in the opposite-direction inbox (`<original-sender>/inbox.<you>.md`), carrying `Re:` the entry it answers.

## Node-identity discovery

On `pickup`, determine which node this session is:

1. If args carry a moniker (`/in-whiteboard pickup vc`), use it.
2. Otherwise infer from cues: working directory, branch, recent commits, the user's framing, which node's `wip.md` carries this session's `session_id`.
3. If still ambiguous, ask the user before writing anything.

The moniker is durable; subsequent sessions of that node inherit it via the existing `<node>/` directory.

## Procedure per subcommand

### pickup

1. List `intent/whiteboard/*/` to enumerate nodes. Determine your node (see discovery).
2. Read your `<you>/wip.md` (resume state) and all four `<you>/inbox.*.md` (incoming). Surface any non-empty inbox entries to the user.
3. Read each peer's `<peer>/wip.md` frontmatter. For each peer with `status: active` AND `heartbeat_at` within 7 days AND a different `session_id`: surface "node X active (heartbeat <relative>, focus: <focus>)". Active but older than 7 days: "node X appears stale".
4. Update your `<you>/wip.md` frontmatter: `session_id` (this session, or `unknown`), `heartbeat_at` (now), `status: active`. Keep `claims` + body intact.
5. Report a one-line summary of peer state + your inbound messages.

### ask <node> <text>

1. If `intent/whiteboard/<node>/inbox.<you>.md` does not exist, create it with its `# inbox: <you> -> <node>` header + `_(empty)_` sentinel (see inbox shape). Append a message entry (see Message-entry format) -- the path encodes sender -> recipient, so the 2.0 `to:`/`from:` line is implicit:

   ```
   ## (YYYY-MM-DD HH:MM)   [Re: <prior-anchor>]   [FYI only -- no response needed.]

   <text>
   ```

   If the inbox already carries only `_(empty)_`, replace that sentinel with the first entry.

2. Touch your heartbeat.

A reply goes to `<sender>/inbox.<you>.md` (the inbox flips direction).

### announce <text>

1. Append the same one-line entry to EVERY peer's `<peer>/inbox.<you>.md` (all nodes except yourself).
2. Touch your heartbeat.

Use for 1-to-all signals -- eg "touching `apps/lamplight/**` for ST-X" (a shared platform-layer edit; the retired `lamplight.md` job), or a protocol/decision broadcast.

### decide <text>

1. Append `- (YYYY-MM-DD) <text>` to your `<you>/wip.md` `## Decisions` section (peers read it at pickup).
2. Touch heartbeat.

### claim <STxxxx> / unclaim <STxxxx>

1. Add/remove `STxxxx` in your `wip.md` `claims`.
2. On claim, scan peers' `wip.md` `claims`: if an _active_ peer already claims it, stop and surface the overlap for the hypervisor to arbitrate.
3. Touch heartbeat.

### clear <sender>

1. In your `<you>/inbox.<sender>.md`, move the handled entries verbatim into `<you>/.history/<YYYYMMDD>/inbox.<sender>.md`, and remove them from the live inbox (leaving the header + `_(empty)_` if none remain).
2. You own your inbox -- no peer files touched. Touch heartbeat.

### archive

Roll your OWN node's DONE content out of the live files into your own history, daily-or-more, so the live files stay lean (they are read on every pickup).

1. Ensure `<you>/.history/<YYYYMMDD>/` exists (today, or the content's own date).
2. From `<you>/wip.md`: move DONE `## DOING` items + superseded blocks into `<you>/.history/<YYYYMMDD>/wip.md`. KEEP frontmatter, live DOING/TODO, `## Watch-outs`, and still-relevant `## Decisions`.
3. From each `<you>/inbox.<sender>.md`: move handled entries into history (same as `clear`).
4. `prettier --write` the touched files if the project formats markdown.
5. **Single-owner: you only ever touch your own `<you>/` directory, so there is no peer-collision hazard** -- this is the key simplification over 2.0's shared-file archive. Commit via explicit pathspec (`git commit --only <you>/...`), never `-A`.

### touch

1. Update your `wip.md` `heartbeat_at` to now. No other change.

### release

1. Set your `wip.md` `status: paused`; update `heartbeat_at`. Leave `claims` + body intact.

### status

1. Read every `<node>/wip.md` frontmatter.
2. Print one line per node: `<node>: <status>, focus=<focus>, claims=[...], heartbeat=<relative>`. No writes.

## Node roles

Roles are per-project. A common shape: one **control** node doing the heavy lifting, one **validation** node (the independent check, below), whatever else the project needs (interface, author, ...), and the **hypervisor** (`hv`) for the human. The project's `README.md` is the source of truth for the roster.

### Validation / Verifier (optional)

A validation node is the independent check that the other nodes' landed or claimed work is **correct, complete, consistent, and faithful to what the hypervisor asked**. If the project keeps a documentation function, documentation becomes the _byproduct_ of verification -- you cannot faithfully document a system that does not do what it claims. Advisory authority only: it posts findings, the hypervisor adjudicates, the owning node fixes. It never mutates another node's code and never blocks its progress.

**Sources -- the triangle:**

- **Ask** -- what the human actually asked: the target node's Claude Code session transcript at `~/.claude/projects/<project-dir>/<session_id>.jsonl` (`session_id` is in that node's `wip.md` frontmatter; re-resolve each audit, it rotates on `/compact` or restart). Read it _targeted_ (tail / grep / sub-agent sweep), never whole.
- **Plan** -- the node's plan file at `~/.claude/plans/<name>.md` (often cited in `focus:`).
- **Reality** -- the whiteboard + `intent/st/**` + code + tests.

**Method:**

- **Fire on claim** -- done / closed / frozen / green, at WP/ST close, schema-freeze, or the hypervisor's request -- not continuously, and not on in-flight edits.
- **Read the as-built, never the narrative.** Evidence is `file:line` from a real read; no invented line numbers; no "certainly" without having read the code.
- **Classify every finding** -- expected-vs-real (queued-but-unbuilt vs falls-between-the-cracks), severity, evidence.
- **Self-refute HIGH findings first** -- try to kill your own finding before posting.
- **Advisory output** -- findings go to the owning node's inbox (`<owner>/inbox.<you>.md`); a compounding risk (a false "done" the next unit would build on) escalates to `hv/inbox.<you>.md`. Never mutate another node's code.
- **Audit your own coverage** -- state what you checked AND what you did not.

## Protocol invariants

1. **One writer per file.** `wip.md` = the node; `inbox.<sender>.md` = the sender. The recipient owns its inbox lifecycle (reads, actions, clears into its own history).
2. **Live channel, not snapshot.** `intent/wip.md` is the post-session snapshot; `<node>/wip.md` is the live board.
3. **Claims by ST ID** (in `wip.md` frontmatter), never glob paths.
4. **Broadcast via `announce` -> peers' inboxes.** No shared file; a shared platform layer (eg `apps/lamplight/**`) is coordinated by announcing before you touch it.
5. **Heartbeat older than 7 days marks a claim reclaimable** -- reclaim requires explicit hypervisor acknowledgement.
6. **`/compact` does NOT end a session** -- status stays `active`; the next `pickup` touches the heartbeat.
7. **Archive your own dir only**, daily-or-more; `.history/YYYYMMDD/` is append-only and never reloaded on pickup.
8. **The human is `hv`** in all protocol language, never by name.

## Why this exists

Concurrent sessions need a live coordination surface, and `wip.md` (the post-session snapshot) loses fidelity _during_ a session. Protocol 2.0 supplied that with shared files (`asks.md`, per-stream files, `lamplight.md`), but those had N writers each: contention on every edit, cleanse that required cross-stream coordination, and unbounded growth that chewed context. 3.0 fixes all three by giving every file exactly one writer -- a per-node board you alone write, and per-sender inboxes each written by one peer and cleansed by you. Coordination stays live; contention and cleanse-pain go away.

## Red Flags

| Rationalisation                                             | Reality                                                                                        |
| ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| "One `inbox.md` per node is simpler than per-sender."       | One file, N writers -- back to 2.0 contention. Per-sender is what makes it single-writer.      |
| "I'll edit a peer's `wip.md` to correct it."                | Never. You write only your own node. Send an `ask` to its inbox.                               |
| "I'll keep a shared file for platform edits."               | That is the retired `lamplight.md`. Use `announce` -- broadcast to inboxes, no shared file.    |
| "/compact ended the session, so I'll set `status: paused`." | No. `/compact` is transparent. Status stays active; `/in-session` re-fires `pickup`.           |
| "I'll archive the whole board while I'm here."              | You archive only your own `<you>/` dir. Single-owner, collision-free -- that is the point.     |
| "The node said it's done, so it's done."                    | A "done" claim is the _trigger_ to verify, not the verdict. Read the as-built against the ask. |
