---
description: "Multi-session coordination via intent/whiteboard/: claim ST scopes, signal cross-stream, heartbeat, release on session end"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination

Coordinator for multiple Claude Code sessions running concurrently against one Intent project. Each session belongs to a _stream_ (a durable identity, eg `control`, `ia-ux`) that owns a file under `intent/whiteboard/`. The whiteboard is the _live_ channel between streams; `wip.md` is the post-session snapshot.

## When to invoke

- `pickup` -- chained from `/in-session` after gate release; or manually after switching stream.
- `claim <STxxxx>` -- when entering a steel-thread your stream is working on.
- `unclaim <STxxxx>` -- when leaving / closing a steel-thread.
- `touch` -- periodically during work to keep heartbeat fresh.
- `ask <to-stream> <text>` -- when you need the other stream to act, decide, or review.
- `decide <text>` -- when this stream makes a decision affecting the other stream.
- `lamplight <text>` -- before touching `apps/lamplight/**` (shared platform layer).
- `release` -- chained from `/in-finish`; sets `status: paused`.
- `status` -- read-only summary of all streams.

If invoked with no subcommand, default to `status`.

## File layout

```
intent/whiteboard/
  README.md         # protocol reference (slim)
  <stream>.md       # one per stream (eg control.md, ia-ux.md)
  asks.md           # shared; point-to-point cross-stream handoffs
  lamplight.md      # shared; apps/lamplight/** edit channel
```

## Stream file shape

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
- (YYYY-MM-DD) <decision>

## Watch-outs
- <thing the other stream should know>

## Notes for next session of this stream
- <pickup hint>
```

Body sections are advisory; only the frontmatter is required for protocol compliance.

## Stream-identity discovery

On `pickup`, determine which stream this session belongs to in this order:

1. If args carry a stream slug (`/in-whiteboard pickup ia-ux`), use it.
2. Otherwise infer from cues: working directory, current branch, recent commits, the user's framing of the session, the wip.md "In flight" sections (which often label streams by ST ownership).
3. If still ambiguous, ask the user before writing anything.

The slug is durable across sessions; once a stream identity is established, subsequent sessions of that stream inherit it via the existing stream file.

## Procedure per subcommand

### pickup

1. Read every `intent/whiteboard/*.md` to memory.
2. Determine your stream identity (see "Stream-identity discovery").
3. For each _other_ stream file: if `status: active` AND `heartbeat_at` is within 7 days AND `current_session_id` differs from yours: surface "other stream X is active (heartbeat <relative time>, focus: <focus>)". If `status: active` but `heartbeat_at` is older than 7 days: surface "other stream X appears stale".
4. Update YOUR stream file's frontmatter: `current_session_id` (current Claude Code session id, or `unknown`), `session_started_at` (now), `heartbeat_at` (now), `status: active`. Keep `claimed_steel_threads` intact.
5. Read `asks.md` for entries addressed to your stream (filter on `to: <your-stream>`); surface them to the user.
6. Read `lamplight.md` for recent platform-edit entries (last 7 days); surface if any.
7. Report a one-line summary of other-stream state + any inbound asks.

### claim <STxxxx>

1. Add `STxxxx` to your stream's `claimed_steel_threads` if not already present.
2. Scan other stream files: if `STxxxx` is in another _active_ stream's `claimed_steel_threads`: stop, surface the overlap, do not proceed without user arbitration.
3. Update `heartbeat_at`.

### unclaim <STxxxx>

1. Remove `STxxxx` from your stream's `claimed_steel_threads` (no-op if absent).
2. Update `heartbeat_at`.

### touch

1. Update your stream file's `heartbeat_at` to now. No other change.

### ask <to-stream> <text>

1. Append entry to `intent/whiteboard/asks.md`:

   ```
   ## (YYYY-MM-DD HH:MM) to: <to-stream> from: <your-stream>
   Re: <prior-ask-anchor>            # optional, if replying to a prior ask

   FYI only -- no response needed.   # optional, when no reply is expected

   <text>
   ```

2. Update your stream file's `heartbeat_at`.

Header conventions (optional, layered on top of the required `to:` / `from:` line):

- **`Re: <prior-ask-anchor>`** -- when an ask replies to a previous one. The anchor is the prior entry's `(YYYY-MM-DD HH:MM)` timestamp. Keeps threads legible in append-only `asks.md`.
- **`FYI only -- no response needed.`** -- mark explicitly when the recipient stream should not queue a reply. Without it, the recipient assumes a reply is wanted.

(These conventions are borrowed from the cross-project LLMsend protocol; in-whiteboard is the intra-project sibling.)

### decide <text>

1. Append entry to your stream file's `## Recent decisions affecting other streams` section:

   ```
   - (YYYY-MM-DD) <text>
   ```

2. Update `heartbeat_at`.

### lamplight <text>

1. Append entry to `intent/whiteboard/lamplight.md`:

   ```
   ## (YYYY-MM-DD HH:MM) by: <your-stream>

   <text>
   ```

2. Update your stream file's `heartbeat_at`.

### release

1. Set your stream file's `status: paused`.
2. Update `heartbeat_at` to now.
3. Leave `claimed_steel_threads` intact (the stream remains owner across sessions).

### status

1. Read every `intent/whiteboard/*.md` (stream files only; not README).
2. Print one line per stream: `<stream>: <status>, focus=<focus>, claims=[STxxxx, ...], heartbeat=<relative time>`. No writes.

## Protocol invariants

1. The whiteboard is the _live_ coordination channel. `wip.md` is the post-session snapshot. Different tense, different reader.
2. Claims are by ST ID only, never by glob path. Paths inferable from `intent/st/<ID>/info.md` cross-references.
3. `apps/lamplight/**` (or the equivalent shared platform layer) is everybody's territory; coordinate via `lamplight.md`.
4. ST-ID collisions are an `intent st new` race (max+1 with no lock), not solvable here. Surface the collision when noticed; let the user arbitrate.
5. Heartbeat older than 7 days marks a claim reclaimable -- reclaim requires explicit user acknowledgement.
6. `/compact` does NOT end the session; status stays `active` across `/compact`. The next `/in-session` (auto-firing in the new context) calls `pickup` which touches the heartbeat.

## Why this exists

Concurrent sessions need a live coordination surface. `wip.md` works for post-session snapshots but loses fidelity _during_ a session -- decisions, asks, platform edits invisible until next wrap-up. The whiteboard is the missing live channel: each stream owns a file, decisions go in your file, asks go in the shared file, platform edits go in the platform file. Read on pickup, append during work, release on finish.

## Red Flags

| Rationalisation                                                  | Reality                                                                              |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| "Only one session active right now -- coordination is overhead"  | The whiteboard is also a journal for future-you. Write it.                           |
| "I'll edit `apps/lamplight` without noting it in `lamplight.md`" | The other stream may be editing the same files. One-line ask is cheap insurance.     |
| "/compact ends the session, so I'll set `status: paused`"        | No. `/compact` is transparent. Status stays active; `/in-session` re-fires `pickup`. |
| "I'll glob-claim `apps/control/**` to be safe"                   | Claims are by ST only. Globs drift; STs are the user's mental model.                 |
| "The other stream's claim is stale; I'll just steal it"          | Reclaim requires explicit user acknowledgement. Don't silently overwrite.            |
