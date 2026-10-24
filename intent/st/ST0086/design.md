# ST0086 design draft: the estate view

Written on 2026-10-24 from the rubber-duck between hv and vc. What is marked measured was measured; the rest is design to be measured before it is built.

## What it is

The estate is the host (rhadamanth, kovacs, gyges each its own). The estate view is a live top-styled table of every Claude session on the host: who it is, which project and node it stands in, whether it is busy, idle, in a heavy run or paused, its one-line WIP, its claims, and the host's load with the cargo and bats census. Three readers render one source of truth: intentd at /estate/status, the intent explore TUI at /estate, and `intent estate status --json` for Claudes.

## The source of truth

A directory of session records: `~/.local/state/intent/estate/<session-id>.json`. State, not config (intentd's socket already lives under `~/.local/state/intent/run`, measured). One writer per file, the session itself; written whole by atomic rename; no locking. Record fields: schema version (pinned from day one, since the dev, opt and brew switch lets two intents share a host), session id (the id the UserPromptSubmit gate already uses), cross-session name, pid, host, project root, node moniker where the session is a whiteboard node, state (busy, idle or paused), started and updated stamps read by the tool at the write, and a WIP line only for a session with no board. Heavy and gone are not states a session writes; the reader joins both (vc's ruling, 2026-10-24): heavy is a running slot the session holds in any lane, a light lane included (see Slots and lanes), and gone is a dead pid (see Cleanup).

Highlander: a session that is a whiteboard node does not copy its focus or claims into the record; the reader joins them from that project's store, which intentd (registry, ST0074) and the TUI already open. The record carries presence only.

## Writers

The hooks, so no Claude has to remember: SessionStart registers the record; UserPromptSubmit refreshes the heartbeat and marks busy; Stop marks idle; SessionEnd removes the record. The explicit verbs: `intent estate wip "<line>"` (a session with no board), `intent estate prune`, and the slot verbs below. MCP exposes `status` and `wip` because a repeat changes nothing; the slot verbs accumulate and stay CLI.

## Readers

One reader function in intentsvcs returns the joined table from the directory plus the stores it points at; the page, the TUI pane and the verb are renderers of it (PFIC, thin coordinator). The daemon adds the host census (load, cargo and bats processes) it measures itself, which retires vc's census script.

The page is not a second document (vc's ruling, 2026-10-24): under D56 the daemon serves one shell document, so /estate/status is a route in `shell.html`, fed by `/op` with the reader's JSON, and the daemon renders no HTML for it. The mockup attached as `estate-status-mockup.html` shows the page's layout and the raw record shapes it reads (census, controller, lanes, requests with one key per file present, sessions, and the boards the reader joins), from an inline sample that re-renders on a timer.

## Cleanup

A record whose pid is dead renders as gone and the daemon reaps it on its next tick; `intent estate prune` does the same where no daemon runs. A live pid with an old heartbeat renders as idle with its age and is never reaped. A slot whose holder is gone renders as orphaned until its controller revokes it; a controller whose pid is dead leaves its record orphaned, and the next `control take` succeeds it. Readers never write.

## Slots and lanes

A generic way for one session to control how much of something runs at once and for others to ask for a share. Nothing in the tool knows what a lane is for; the names below (heavy, light, kovacs) are the ones 2026-10-24 ran by hand and are examples, not a fixed set.

- A lane type is a class the controller declares (heavy, light, kovacs). A lane is an instance of a type with its own running capacity: `intent estate slots lane open <lane> --type <type> --capacity <n>`; the same verb changes the capacity, and `lane close <lane>` ends it when nothing is granted in it. How many lanes, of which types, with what capacity, is the controller's call. On 2026-10-24 that was two heavy lanes of capacity one on the Mac (cargo, bats), one light lane beside bats, and one kovacs lane.
- A request asks by type, and may name a lane: `intent estate slots ask <type> "<what>" [--lane <lane>]` returns an id and queues it. Queued is per type. `slots wait <id>` blocks the requester until a grant or a denial lands, with a timeout.
- The controller: `grant <id> --lane <lane>`, `deny <id> "<why>"`, `revoke <id> "<why>"`. A grant that would put the lane over its capacity, counting granted and running, is refused and names the count; the controller raises the capacity if it means it. Which request goes first is the controller's judgement, never the tool's.
- The holder: `start <id>` and `release <id>`. `status` shows every lane with its type, capacity, holders and the queue for its type, and each session's row shows the slots it holds.
- The files are the protocol, under `~/.local/state/intent/estate/slots/`: `lanes/<lane>.json` written by the controller; each request a directory `requests/<id>/` holding `ask.json` from the requester, `grant.json` (naming the lane) or `deny.json` from the controller, `start.json` and `end.json` from the holder, `revoke.json` from the controller. A request belongs to no lane until granted. Its state is the fold of the files present (asked, granted, running, ended, denied, revoked), so one writer per file holds, no lock exists, and the readers read it like the status records.
- Learning without polling: `wait` for the requester; for a controller, the UserPromptSubmit hook names the pending asks on the lane types its session controls, the way it injects project context today.
- Out of the first cut: priorities, and any policy beyond capacity. Succession is by pid liveness only (see The coordinator).

## The coordinator

The controller is whichever session hv assigns to manage the workload: a dedicated session that stands in no Intent project, or a node inside one. The protocol treats both the same and names the role by a record instead of relying on a volunteer, which is what 2026-10-24 did.

- Control is a record: `intent estate control take` writes `controller.json` beside the lanes, one per host, with the session, its cross-session name and pid; `control release` ends it. A second take is refused while the holder's pid is alive; a dead pid leaves the record orphaned and a new take is allowed, which is the whole of succession in the first cut. Lane, grant, deny and revoke writes are accepted from the controller session only.
- The page and `status` show the controller at the top, or NO CONTROLLER, in which case asks queue and nothing is granted.
- Waking it: `intent estate slots watch` blocks and emits one line per state change under `slots/requests/` (asked, started, released), the shape a Monitor consumes, and the coordinator's contract arms it at session start. Requesters have `wait`, the coordinator has `watch`, and the UserPromptSubmit hook injects the pending asks on top.
- A dedicated coordinator's status record has no project root and no node and its WIP line is set directly; a project node that takes control keeps its project and node, and the reader joins its focus as for any node.
- All of it inside Intent, nothing else needed: `intent estate init [<dir>]` renders a dedicated coordinator's working directory from Intent's own templates (`lib/templates/estate/`, through the same token expander as every root file): its AGENTS.md and CLAUDE.md carrying the coordinator contract (take control at session start, arm `slots watch` under a monitor, judge asks against lane capacity and the estate view, release control at session end), and `.claude/settings.json` wiring the same `intent claude hook` lifecycle hooks a project gets. A Claude Code session started in that directory is the coordinator with only Intent installed. A project node that is assigned control follows the same contract, carried to it by the hook injection.

## Work packages

- WP-01 the record and its doors: directory, record, reader, verbs, hooks, MCP exposure, the reap rule.
- WP-02 the page: intentd watches the directory outside the ingest path and answers the reader over `/op`; /estate/status is a route in the one shell document (D56), live, top-styled, with the census, laid out as `estate-status-mockup.html`.
- WP-03 the TUI: /estate in intent explore on the same reader, beside /projects.
- WP-04 slots, lanes and the coordinator: lane types and lanes with a capacity, the request records, the controller record with take and release, `intent estate init` and the coordinator templates, the slot verbs, the capacity refusal, wait and watch, the hook naming pending asks, and the controller, lanes and slots on the page and in the TUI.

## Out of scope

Cross-project announce; federation of estates across hosts (kovacs on rhadamanth's page); slot policy beyond capacity; anything a sleeve or dotfiles framework would add on top, since the coordinator runs on Intent alone.

## To measure before the design is final

- Whether a hook sees the session's cross-session name or only its id.
- Whether intentd's watcher can take a directory outside any project without entering the ingest path (the ingest is what reverts the last write of a burst).
- What SessionEnd delivers when a session is killed rather than ended, so the pid rule is the backstop.
- The TUI's /projects reader, so /estate reuses its shape.
