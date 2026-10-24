# ST0086 design draft: the estate view

Written on 2026-10-24 from the rubber-duck between hv and vc. What is marked measured was measured; the rest is design to be measured before it is built.

## What it is

The estate is the host (rhadamanth, kovacs, gyges each its own). The estate view is a live top-styled table of every Claude session on the host: who it is, which project and node it stands in, whether it is busy, idle, in a heavy run or paused, its one-line WIP, its claims, and the host's load with the cargo and bats census. Three readers render one source of truth: intentd at /estate/status, the intent explore TUI at /estate, and `intent estate status --json` for Claudes.

## The source of truth

A directory of session records: `~/.local/state/intent/estate/<session-id>.json`. State, not config (intentd's socket already lives under `~/.local/state/intent/run`, measured). One writer per file, the session itself; written whole by atomic rename; no locking. Record fields: schema version (pinned from day one, since the dev, opt and brew switch lets two intents share a host), session id (the id the UserPromptSubmit gate already uses), cross-session name, pid, host, project root, node moniker where the session is a whiteboard node, state (active, idle, heavy, paused), started and updated stamps read by the tool at the write, the heavy run in flight (what, started) if any, and a WIP line only for a session with no board.

Highlander: a session that is a whiteboard node does not copy its focus or claims into the record; the reader joins them from that project's store, which intentd (registry, ST0074) and the TUI already open. The record carries presence only.

## Writers

The hooks, so no Claude has to remember: SessionStart registers the record; UserPromptSubmit refreshes the heartbeat and marks busy; Stop marks idle; SessionEnd removes the record. The explicit verbs: `intent estate wip "<line>"` (a session with no board), `intent estate run start "<what>"` and `run end`, `intent estate prune`. MCP exposes `status` and `wip` because a repeat changes nothing; `run start` accumulates and stays CLI.

## Readers

One reader function in intentsvcs returns the joined table from the directory plus the stores it points at; the page, the TUI pane and the verb are renderers of it (PFIC, thin coordinator). The daemon adds the host census (load, cargo and bats processes) it measures itself, which retires vc's census script.

## Cleanup

A record whose pid is dead renders as gone and the daemon reaps it on its next tick; `intent estate prune` does the same where no daemon runs. A live pid with an old heartbeat renders as idle with its age and is never reaped. A heavy run whose session is gone renders as orphaned and leaves with the session. Readers never write.

## Work packages

- WP-01 the record and its doors: directory, record, reader, verbs, hooks, MCP exposure, the reap rule.
- WP-02 the page: intentd watches the directory outside the ingest path and serves /estate/status, live, top-styled, with the census.
- WP-03 the TUI: /estate in intent explore on the same reader, beside /projects.

## Out of scope

Cross-project announce; federation of estates across hosts (kovacs on rhadamanth's page); any slot-desk logic beyond recording runs.

## To measure before the design is final

- Whether a hook sees the session's cross-session name or only its id.
- Whether intentd's watcher can take a directory outside any project without entering the ingest path (the ingest is what reverts the last write of a burst).
- What SessionEnd delivers when a session is killed rather than ended, so the pid rule is the backstop.
- The TUI's /projects reader, so /estate reuses its shape.
