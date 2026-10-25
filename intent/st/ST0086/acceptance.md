---
st_id: ST0086
title: Add an 'Estate' page to intentd that all intent-using Claudes on an estate can use to post status
---

# ST0086: Add an 'Estate' page to intentd that all intent-using Claudes on an estate can use to post status -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The estate record and its doors: directory, versioned record, one reader, the estate verbs, MCP exposure, the reap rule (status: WIP)

- AC-01.1 One reader in intentsvcs returns the host's estate as one serialisable value, schema `intent/estate-status@1`, with every key present and typed from this package, so no renderer branches on a missing key; a key a later package fills carries the empty value of its declared shape until then. -- satisfied: no (computed)
- AC-01.2 The reader reads Claude Code's own session files under the config dir and never writes them; a test pins exactly the fields it relies on (pid, procStart, sessionId, cwd, name, status, startedAt, updatedAt), and a file lacking one is listed under `unreadable` with the field named, never dropped. -- satisfied: no (computed)
- AC-01.3 Project, node, focus and claims are joined, never stored: the project is the Intent project whose tree holds the session's cwd, a cwd in a git worktree folding to its main checkout; node, focus and claims come from the board whose recorded session id matches, read from the rendered `board.json` header under board schema 3.0 and 4.0 alike, and the reader opens no store. -- satisfied: no (computed)
- AC-01.4 `intent estate wip "<line>"` writes only `~/.local/state/intent/estate/<session-id>.json`, schema `intent/estate-session@1`, whole and by atomic rename, and prints `wrote:`; `intent estate prune` removes only Intent's own records for gone sessions and prints `removed:` per file; neither touches a Claude Code file. -- satisfied: no (computed)
- AC-01.5 A session whose pid is dead, or whose pid now belongs to a process started at a different time than its procStart (ctime via `ps` on macOS, clock ticks via `/proc/<pid>/stat` on Linux), renders as gone; a failed liveness read lists the file as unreadable and calls nobody gone; paused is a board's paused status and heavy is a running slot the session holds. -- satisfied: no (computed)
- AC-01.6 `estate status` and `estate wip` are exposed on MCP and `estate prune`, `estate watch`, `estate unwatch` and `estate statusline` are withheld, each dispatch row carrying its reason (decision 75), and `estate status --json` prints the reader's value unchanged. -- satisfied: no (computed)
- AC-01.7 Every error path of the estate verbs and the reader returns a typed error carrying a remedy, nothing is rescued and swallowed, and an unreadable session file is reported, not raised. -- satisfied: no (computed)

### WP-02 -- intentd serves /estate/status: watches the estate directory outside the ingest path, renders the live top-styled page with the host census (status: Not Started)

- AC-02.1 intentd answers the page's data call `{"op":"form","view":"/estate/status","root":...}` with `result: estate_status` carrying the reader's JSON; /estate/status is a route in the one shell document fed by `/op`, the daemon renders no HTML for it, and `/op` refuses without the bearer token as every op does. -- satisfied: no (computed)
- AC-02.2 The daemon's census tick measures the load averages and counts the running processes of each declared class, attributing each to the project whose tree holds its working directory and counting the rest as unattributed, keeps a history ring the page draws as the last hour, and reaps the WIP record of a gone session on the tick. -- satisfied: no (computed)
- AC-02.3 `intent estate watch <class> <pattern>` writes `census/watches/<class>.json` and changes the pattern on repeat, `intent estate unwatch <class>` removes it, each prints its `wrote:` or `removed:` line, the pattern is a regular expression matched against the process command line, and the census counts no class nobody declared. -- satisfied: no (computed)
- AC-02.4 Plan usage is read first from the account's usage endpoint through Claude Code's Keychain item under Claude Code's own service-name condition with no fallback between the suffixed and bare names, once per census tick after a status ask, and second from `intent estate statusline`, which reads the statusline JSON on stdin, writes the session record's rate limits, prints `wrote:` on stderr and nothing on stdout, and exits 0 on an input with no rate_limits; every failure of either source is named in `usage.error` and the page says which source it shows and how old it is. -- satisfied: no (computed)
- AC-02.5 (non-test) `intent estate status --browser` opens the page served by the running daemon, laid out as the attached `estate-status-mockup.html` hv approved on 2026-10-24 (the top band, the queue, the lanes, the sessions, each panel a native details element whose collapsed summary keeps a live reading), re-reading every few seconds while open. -- satisfied: no

### WP-03 -- The intent explore TUI: /estate beside /projects, on the same reader (status: Not Started)

- AC-03.1 `intent explore` shows /estate beside /projects, rendered from the same intentsvcs reader as the page and the verb, with each session's state, project and node, line and its age, claims and slots, the host block with the census, and the controller, lanes and queue from the same keys the page reads, rendering their empty values without branching before WP-04 lands. -- satisfied: no (computed)
- AC-03.2 /estate re-reads on the explorer's refresh cadence, lists an unreadable session file as unreadable with the field named rather than dropping it, and `/estate` in the omnibox opens it. -- satisfied: no (computed)

### WP-04 -- Slots: a generic pool, ask, grant, start, release protocol on the estate, records beside the status records, capacity enforced by the tool, policy with the controlling Claude (status: Not Started)

- AC-04.1 `intent estate control take` writes `controller.json` beside the lanes with the session, its cross-session name and pid; a second take is refused while the holder's pid is alive, a dead pid allows a new take, `control release` ends it, and lane, grant, deny and revoke writes are accepted from the controller session only. -- satisfied: no (computed)
- AC-04.2 `intent estate slots lane open <lane> --type <type> --capacity <n>` writes `lanes/<lane>.json` and changes the capacity on repeat, `lane close <lane>` refuses while anything is granted in it, and a lane type is whatever the controller names, nothing being built in. -- satisfied: no (computed)
- AC-04.3 `slots ask <type> "<what>" [--lane <lane>]` writes `requests/<id>/ask.json` and prints the id; `grant <id> --lane <lane>`, `deny <id> "<why>"`, `revoke <id> "<why>"`, `start <id>` and `release <id>` each write their one file; a request's state is the fold of the files present (asked, granted, running, ended, denied, revoked); and a grant that would put the lane over its capacity, counting granted and running, is refused naming the count. -- satisfied: no (computed)
- AC-04.4 `slots wait <id>` blocks the requester until a grant or a denial lands, with a timeout; `slots watch` blocks and emits one line per state change under `slots/requests/` in the shape a Monitor consumes; and the UserPromptSubmit hook names the pending asks on the lane types the session controls. -- satisfied: no (computed)
- AC-04.5 `intent estate init [<dir>]` renders a dedicated coordinator's working directory from `lib/templates/estate/` through the same token expander as every root file, with AGENTS.md and CLAUDE.md carrying the coordinator contract and `.claude/settings.json` wiring the `intent claude hook` lifecycle hooks, so a Claude Code session started there is the coordinator with only Intent installed. -- satisfied: no (computed)
- AC-04.6 The page, the TUI and `estate status` show the controller or NO CONTROLLER, every lane with its type, capacity, holders and the queue for its type, oldest first, and each session's row shows the slots it holds; a session holding a running slot in any lane reads as heavy. -- satisfied: no (computed)

## Acceptance Tests

### WP-01 -- The estate record and its doors: directory, versioned record, one reader, the estate verbs, MCP exposure, the reap rule (status: WIP)

_(no tests in this group)_

### WP-02 -- intentd serves /estate/status: watches the estate directory outside the ingest path, renders the live top-styled page with the host census (status: Not Started)

_(no tests in this group)_

### WP-03 -- The intent explore TUI: /estate beside /projects, on the same reader (status: Not Started)

_(no tests in this group)_

### WP-04 -- Slots: a generic pool, ask, grant, start, release protocol on the estate, records beside the status records, capacity enforced by the tool, policy with the controlling Claude (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
