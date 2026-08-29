# Intent v2.13.0 Release Notes

**Release Date**: 2026-06-25

## Overview

Intent v2.13.0 makes **multi-agent agentic coding (MAAC)** a first-class capability (ST0047). It adds one command surface to `intent claude` that lets several Claude Code sessions -- and the human -- work the same Intent project at once, coordinating through a shared, contention-free **whiteboard**:

- **`intent claude start <ws>`** launches a Claude Code session bound to a whiteboard workstream, with the right effort / permission posture and a standing pickup-then-plan instruction.
- **`intent claude ws new|list|archive|hygiene`** is the deterministic lifecycle for the workstreams themselves.

The coordination model underneath -- **Whiteboard Protocol 3.0** -- shipped in v2.12.0 (ST0045); v2.13.0 is where it became a command you run rather than a convention you hand-maintain. This note documents the whole system end to end, because the parts only make sense together.

> This release note was authored retroactively in the 2.13.1 cycle: v2.13.0 shipped without one, and it introduced the most intricate workflow in Intent. The mechanics below are the canonical reference.

## What is MAAC?

A non-trivial change to a real codebase is often several streams of work at once: build the engine, validate it independently, write the docs, sequence the whole thing. A single Claude Code session holds one of those streams well; it does not hold four. MAAC runs **one session per stream**, plus the human, and gives them a place to coordinate that does not lose fidelity the way a shared scratch file does.

Each participant -- a Claude session, or the human -- is a **node**. Nodes coordinate through `intent/whiteboard/`: a live, on-disk channel that is the cross-session counterpart to `intent/wip.md` (which remains the post-session snapshot). Different tense, different reader, different cadence.

## The whiteboard (Protocol 3.0)

### Nodes and the single-writer rule

Every node owns one directory, `intent/whiteboard/<node>/`, and the protocol's one hard rule is that **every file has exactly one writer**:

- `<node>/wip.md` -- the node's live board (frontmatter + DOING / TODO / Watch-outs / Decisions). Written **only** by that node.
- `<node>/inbox.<sender>.md` -- one per peer. Appended **only** by `<sender>`; read and cleansed **only** by the owner.

Point-to-point messaging routes to `<recipient>/inbox.<you>.md`. A broadcast is an `announce` that writes one line into **every** peer's inbox (this subsumes the old shared "platform" file). Archiving only ever touches your own directory. Because no file has two writers, the three costs of the previous flat-file model (Protocol 2.0: a shared `asks.md`, one file per stream, a shared platform file) -- edit contention, cross-stream cleanse coordination, and unbounded context growth -- simply do not arise.

A node's `wip.md` frontmatter carries its identity and live state:

```yaml
---
node: cc
name: Control Claude
role: control
session_id: <UUID|none>
heartbeat_at: <ISO 8601>
status: active | paused
focus: "<one-line current goal>"
claims: [ST0048, ...]
---
```

Work is claimed by **ST id** (`claims:`), never by glob path. A heartbeat older than seven days marks a claim reclaimable (with explicit human acknowledgement).

### The hv (hypervisor) node

The human is a first-class node, conventionally **`hv`** -- the hypervisor: it adjudicates scope, sequences work, owns releases, and is where escalations land. It is structurally a node like any other (peers read its `wip.md`, append to its inboxes), with three human-driven differences: its `session_id` is optional (`none`), its heartbeat is advisory (exempt from the seven-day reclaim), and it may carry a `## Standing directives` section that every node honours.

A common roster is one **control** node doing the build, one **validation** node (the independent check), and `hv`. Intent's own board is exactly that -- `hv` + `cc` + `vc`, with no interface node, because Intent is CLI plus data, not UX. The roster is per-project, declared in `intent/whiteboard/README.md`.

### Layout

```
intent/whiteboard/
  README.md                 # protocol pointer + the project's node roster
  <node>/
    wip.md                  # the node's live board (single-writer = the node)
    inbox.<sender>.md       # messages FROM <sender> (single-writer = the sender)
    .history/
      .gitkeep              # tracks the otherwise-empty archive dir
      YYYYMMDD/             # the node's archived DONE work + handled inbox entries
```

A fresh inbox is its header plus the "no live entries" sentinel, so it is never an ambiguous zero-byte file:

```
# inbox: cc -> vc

_(empty)_
```

## Two surfaces, one model: the Highlander split

The whiteboard has two interfaces, and they own different things on purpose:

- **The `/in-whiteboard` skill** owns the **judgement** ops -- the ones a Claude session performs with context: `pickup`, `ask`, `announce`, `decide`, `claim` / `unclaim`, `clear`, the semantic `archive`, `touch`, `release`, `status`.
- **`intent claude ws ...`** owns the **mechanical** lifecycle -- the deterministic, scriptable ops: `new`, `list`, `archive`, `hygiene`.

There is no second copy of the scaffolding logic: the skill's "scaffold a node" step points at `intent claude ws new`. One format, one source of truth.

### The skill ops (judgement)

A session meets the whiteboard through `/in-whiteboard`, chained automatically from the lifecycle skills: `/in-session` fires `/in-whiteboard pickup` (read your board + inboxes, surface peer state, touch your heartbeat); `/in-finish` fires `/in-whiteboard release` (mark yourself paused). In between, a node uses `ask <node> <text>` (point-to-point), `announce <text>` (broadcast, eg before touching a shared layer), `decide <text>` (record a cross-node decision), `claim <STxxxx>` (take a scope, with an overlap check against active peers), and `clear` / `archive` (cleanse and roll your own DONE content into your own history). All of it is opt-in by directory presence: no `intent/whiteboard/`, no behaviour.

### The commands (mechanical)

**`intent claude ws new <ws>`** scaffolds a Protocol 3.0 node to spec: `<ws>/wip.md` with valid frontmatter, `<ws>/.history/.gitkeep`, and an `_(empty)_` inbox in **both** directions with every existing peer. On a board's first creation, `hv` is provisioned as Workstream Zero; working nodes are made to order.

**`intent claude ws list`** prints one line per node (id, status, focus, claims, heartbeat age) read from each `wip.md` frontmatter -- read-only.

**`intent claude ws archive <ws>`** retires a node into `.archived/`, keeping its `.history/` intact; it disappears from `ws list`.

**`intent claude ws hygiene [<ws>]`** is a mechanical structural lint: it validates each node (parseable frontmatter, `.gitkeep`, well-formed inboxes), warns on oversized boards and stale heartbeats, and **never** touches DOING content -- the semantic archive stays the Claude-driven `/in-whiteboard archive`.

**`intent claude start <ws>`** launches a Claude Code session bound to a workstream. It composes the node identity, the project `.claude/restart.md`, and a standing "run `/in-session`, pick up your board, then show a plan and wait" instruction, and execs:

```
claude --effort max --permission-mode auto --append-system-prompt <context> "/in-session"
```

The seeded `/in-session` is admitted by the in-session gate's slash-exemption (so the launch does not deadlock the way pasted prose would), and it chains `/in-whiteboard pickup`. If the workstream does not exist yet, `start` offers to provision it first. `CWI_DRY_RUN` prints the assembled `claude` argv instead of launching -- the test seam and a safe preview.

The command lives at `intent/plugins/claude/bin/intent_claude_cwi` and resolves the **current project** via `find_project_root` (not the tool's install root), so it is served centrally from `$INTENT_HOME`: every project gets `intent claude start` / `ws` the moment the tool updates, with no per-project install.

## Getting started

```bash
# 1. Stand up a board (hv is created automatically the first time).
intent claude ws new cc          # the control workstream
intent claude ws new vc          # the validation workstream

# 2. See the roster.
intent claude ws list

# 3. Launch a session bound to a workstream.
intent claude start cc           # boots Claude Code, runs /in-session, picks up the cc board

# 4. Keep it tidy.
intent claude ws hygiene         # structural lint; warns, never destroys
```

The human maintains the `hv` board by hand (or has it maintained); the Claude sessions run their own boards through `/in-whiteboard`.

## The arc: convention to product

MAAC was not designed on paper. It was **pioneered by convention in Lamplight** -- five hand-run nodes coordinating concurrent sessions on one project -- which remains the operational reference for how it actually runs. It was first **productised in Baize** (the MVP): a whiteboard hand-scaffolded to the Protocol 3.0 spec as a golden reference, plus a POSIX-shell `claude_with_intent` prototype that turned the manual launch ritual into one command. ST0047 makes that capability **first-class in Intent**, served centrally to every project, and retires the Baize prototype -- there is no divergent second copy (Highlander).

Intent now dogfoods MAAC on its own development: it carries its own `intent/whiteboard/` with `hv` + `cc` + `vc`.

## Upgrade

```bash
intent upgrade --apply
```

The `intent claude start` / `ws` commands are served centrally, so there is nothing to install per project -- after the tool updates, they are available everywhere. To begin using MAAC on a project, stand up a board with `intent claude ws new` (see Getting started). Projects that never create an `intent/whiteboard/` see zero change: the whole system is opt-in by directory presence.

## Why

Concurrent sessions need a live coordination surface, and `wip.md` -- the post-session snapshot -- loses fidelity during a session. Protocol 2.0 supplied that surface with shared files, but every shared file had N writers: contention on each edit, cleanse that required cross-stream coordination, and growth that chewed context. Protocol 3.0 gives every file exactly one writer -- a per-node board you alone write, and per-sender inboxes each written by one peer and cleansed by you -- so coordination stays live while contention and cleanse-pain go away. v2.13.0 turns that model into commands, so a workflow that took a page of manual ritual is now `intent claude ws new` and `intent claude start`.
