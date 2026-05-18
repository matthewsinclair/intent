# Design - ST0040: Whiteboard protocol for multi-Claude sessions in the one repo

## Approach

A per-project `intent/whiteboard/` directory holds the live cross-session coordination state. Each session belongs to a **stream** (durable identity, eg `control`, `ia-ux`); the stream owns one file. Cross-stream signals (asks, handoffs) go in a single shared `asks.md`. The shared platform layer (`apps/lamplight/**` or its equivalent in non-Lamplight projects) gets its own file because no steel-thread claim covers it cleanly.

### Tense / reader / cadence (why the whiteboard is a different surface from wip.md)

| Surface             | Tense    | Reader              | Update cadence                |
| ------------------- | -------- | ------------------- | ----------------------------- |
| `done.md` + history | past     | humans + future-me  | post-session                  |
| `wip.md`            | present  | humans              | session-end snapshot          |
| **whiteboard**      | **live** | **the other agent** | **during work, per-decision** |

`wip.md` is the snapshot; the whiteboard is the journal between sessions. Different readers, different update cadence.

### File layout

```
intent/whiteboard/
  README.md         # protocol reference (slim)
  <stream>.md       # one per stream (eg control.md, ia-ux.md)
  asks.md           # shared; point-to-point cross-stream handoffs
  lamplight.md      # shared; apps/lamplight/** edit channel (rename per project)
```

### Stream file shape

YAML frontmatter (machine-readable) + markdown body (human-readable):

```yaml
---
stream_id: <slug>
current_session_id: <UUID or "none">
session_started_at: <ISO 8601>
heartbeat_at: <ISO 8601>
status: active # active | paused | stale
focus: "<one-line current goal>"
claimed_steel_threads: [STxxxx, ...]
recent_memory_writes: [<memory_slug>, ...]
---
# <Stream Name>

## Recent decisions affecting other streams
## Watch-outs
## Notes for next session of this stream
```

Body sections are advisory; only the frontmatter is required for protocol compliance.

### Skill surface

One skill (`/in-whiteboard`) with subcommands -- matches the muscle memory of `intent st <verb>` and `intent wp <verb>`:

```
/in-whiteboard pickup    # read all streams, surface state, touch own heartbeat
/in-whiteboard claim     # add ST to own stream's claimed_steel_threads
/in-whiteboard unclaim   # remove ST from own stream's claimed_steel_threads
/in-whiteboard touch     # update heartbeat only
/in-whiteboard ask       # append point-to-point entry to asks.md
/in-whiteboard decide    # append decision to own stream's Recent decisions
/in-whiteboard lamplight # append entry to lamplight.md
/in-whiteboard release   # status: paused, heartbeat = now
/in-whiteboard status    # one-line summary of all streams (no writes)
```

### Skill chain integration

- `/in-session` chains to `/in-whiteboard pickup` after gate release (new step 5; existing "Confirm and proceed" renumbered to step 6).
- `/in-finish` chains to `/in-whiteboard release` before any wip.md/restart.md/done.md updates (new step 1; existing steps 1-5 renumbered 2-6).
- Both chains are **opt-in-by-presence**: if `intent/whiteboard/` doesn't exist in the project root, the chain skips silently. This keeps existing projects without a whiteboard fully unchanged.

## Design Decisions

User-confirmed via plan-mode AskUserQuestion on 2026-05-18:

1. **v0 scope: stream files + `asks.md` + `lamplight.md`.** Four files in `intent/whiteboard/` plus per-stream files. No `decisions.md`.
2. **Skill auto-invocation: yes.** `/in-session` and `/in-finish` chain to `/in-whiteboard` automatically. Session-start coordination becomes invisible-by-default.
3. **Stale-claim threshold: 7 days.** Heartbeat older than 7d marks a claim reclaimable; reclaim still requires explicit user acknowledgement.

### Locked invariants

1. The whiteboard is the _live_ coordination channel. `wip.md` is the snapshot. Different tense, different reader.
2. Claims are by **steel-thread ID only**, never by glob path. Paths inferred from `intent/st/<ID>/info.md`.
3. The shared platform layer (`apps/lamplight/**` or equivalent) is everybody's territory; coordinate via `lamplight.md`.
4. ST-ID collisions are an `intent st new` race (max+1 with no lock), not solvable in the whiteboard. Surface; let the user arbitrate.
5. Heartbeat older than 7 days marks a claim reclaimable -- reclaim requires explicit user acknowledgement.
6. `/compact` does NOT end the session; status stays `active` across `/compact`. The next `/in-session` (auto-firing in the new context) calls `pickup` which touches the heartbeat.

## Architecture

The skill is a thin coordinator: parse subcommand -> read/write specific files in `intent/whiteboard/` -> surface state to user. No external state, no daemon, no inter-process locking. File-system last-writer-wins is acceptable because:

- Per-stream files have a single writer (the owning stream's current session).
- Shared files (`asks.md`, `lamplight.md`) are **append-only** -- new entries never modify or move existing entries.
- The Claude Code edit tools serialise file-system writes within a session.

The only contention surface is the rare case where both streams hit `asks.md` simultaneously. In practice this is bounded by Claude Code's per-session tool ordering. If it ever causes a problem, switch to per-file-per-message (eg `asks/2026-05-18T16-32-from-control.md`); not worth the complexity in v0.

## Alternatives Considered

### Single coordination file with marked sections (rejected)

Extend the existing `intent/wip.md` "Multi-session note" convention -- single file, each stream owns a marked section. Rejected because:

- wip.md is the post-session snapshot (different tense, different reader, different update cadence) -- mixing live and snapshot in one file confuses both readers.
- Single-file contention is real for live updates: two sessions touching the file simultaneously is more likely than two sessions appending to shared `asks.md`.
- The existing "add sections, don't overwrite" rule in wip.md already shows strain -- the line was added because it had been overwritten enough times to need a rule.

### Glob-path claims (rejected)

Claim scope as glob patterns (`apps/control/**`, `intent/st/ST0177/**`). Rejected because:

- Claims drift from edits. The claimer types `apps/control/**` but actually edits `apps/lamplight/lib/lamplight/core/runloop.ex` -- invisible to the other stream.
- Steel-thread IDs are the user's mental model; globs are not.
- Glob granularity invites bikeshedding ("should I claim `apps/control/lib/**` or just `apps/control/lib/run/**`?"). ST IDs sidestep this entirely.

### `decisions.md` event log (rejected)

A cross-stream append-only event log in addition to per-stream "Recent decisions" sections. Rejected because:

- Duplicates `intent/history/YYYYMM-done.md` past-tense, badly.
- Cross-stream decision-signal already has two homes: the deciding stream's own file (where it belongs as ownership-anchored history) and `asks.md` when the other stream needs to act on it.
- A third event log dilutes `done.md`'s authority as the permanent record.

### Multi-session-per-stream modelling (rejected)

Track every Claude Code instance as a separate entity, even when several run the same stream. Rejected because:

- If a user accidentally runs three Control sessions, the protocol can't help -- this is a misuse mode that has to be visible at the human layer.
- Modelling it adds complexity (per-instance heartbeats, conflict resolution within a stream) for zero gain.
- The chosen approach -- surface "other session detected" on pickup when `current_session_id` differs -- is sufficient: the user sees the collision and arbitrates.

### Hook-based enforcement (deferred to a possible v1)

A `PreToolUse` hook that blocks edits to files outside the current stream's claimed scope. Deferred because:

- ST-only claims (the design) cannot be mechanically translated to file-path checks without re-introducing the glob drift problem.
- False-positive blast radius is high -- legitimate edits get blocked, friction spikes.
- Advisory v0 is sufficient for the two-stream case; revisit if N>=3 streams or if drift becomes a real problem.

### Per-project whiteboard config file (deferred)

`intent/.config/whiteboard.json` declaring the project's known streams. Deferred because:

- Stream identity inference on `pickup` (working directory + branch + user framing + wip.md) is sufficient in practice.
- Adding a config file before the inference proves brittle is premature.
- If inference brittleness shows up, add `streams: [<slug>, ...]` config in a later pass.
