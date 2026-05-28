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
- `archive [as-of <YYYY-MM-DD>]` -- roll DONE/superseded content older than 2 days into weekly history buckets (`history/<Monday>.<file>`) so the live files stay small.

If invoked with no subcommand, default to `status`.

## File layout

```
intent/whiteboard/
  README.md         # protocol reference (slim)
  <stream>.md       # one per stream (eg control.md, ia-ux.md)
  asks.md           # shared; point-to-point cross-stream handoffs
  lamplight.md      # shared; apps/lamplight/** edit channel
  history/          # weekly archive buckets: <YYYYMMDD>.<file> (Monday-anchored)
```

## Stream file shape

YAML frontmatter (machine-readable) + markdown body (human-readable):

```yaml
---
stream_id: <slug>
handle: <short> # optional shorthand for terse asks-routing, eg CC, VC, IC
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

Body sections are advisory; only the frontmatter is required for protocol compliance. `handle:` is optional and additive -- `stream_id` stays the routing key, so adding handles never breaks `pickup` or `asks`.

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

### archive [as-of <YYYY-MM-DD>]

Roll DONE / superseded content out of the live whiteboard files into weekly history buckets, so the live files stay small (they are reloaded every `pickup`, and they grow without bound otherwise). This is judgment-guided, NOT a blind date filter -- like a critic, you read each entry and decide what is genuinely done.

1. Ensure `intent/whiteboard/history/` exists.
2. Set the cutoff = today minus 2 days (or the `as-of` date if given). Content DONE on or before the cutoff is archive-eligible; content after it stays live.
3. For each live file -- the stream files + `asks.md` + `lamplight.md` (NOT `README.md`, NOT live ledgers like `cookies.md`):
   - **KEEP regardless of date:** frontmatter; the current RESUME POINT / STATUS block; standing reference (operating mode, `## Watch-outs`, conventions); any still-OPEN item (an unanswered ask, a live TODO).
   - **ARCHIVE (dated on/before the cutoff AND done):** resolved or absorbed ask threads, superseded RESUME / STATUS blocks, closed-ST reference, absorbed decisions, old edit notices.
   - For each archived entry, compute the Monday of the ISO week its date falls in (`YYYYMMDD`) and append it **verbatim** to `intent/whiteboard/history/<YYYYMMDD>.<file>` (create it with a one-line header if new). The bucket is keyed by the archived CONTENT's week, not the day you run `archive`, so one run can write several weekly buckets.
   - Remove the archived entries from the live file and leave a one-line pointer where they were:
     `> **[archived]** Older entries (DONE on/before <cutoff>) are in history/<YYYYMMDD>.<file>. See intent/whiteboard/history/ for prior weeks.`
4. `prettier --write` the touched files if the project formats markdown.
5. Report per-file line reduction + the buckets written.

**Concurrency (critical -- these are shared files):** do NOT archive a peer's stream file or the shared `asks.md` / `lamplight.md` while that peer is ACTIVE (fresh heartbeat, different `current_session_id`) -- you will collide on the shared working tree / git index. Either the user pauses the other streams for a clean atomic sweep, or archive ONLY your own stream file. Always commit via an explicit pathspec (`git commit --only <paths>`), never a bare commit.

## Stream roles

Streams are **per-project configuration**. A project declares its own streams and handles in its hand-authored `intent/whiteboard/README.md` -- any number of streams, any handles that make sense for that project. There is no roster baked into this skill. `stream_id` remains the routing key; `handle:` is shorthand only.

**Recommended baseline operating model.** For a project that adopts the whiteboard, the normal shape is two streams: one **Control** stream doing the heavy lifting, and one **Verifier** stream providing the independent check. Additional streams are project-specific -- Lamplight, for instance, runs Control / Verifier / Interface with handles `CC` / `VC` / `IC`, but that is _Lamplight's config_, shown here only as illustration. The baseline is a recommendation, not a requirement: a project may run peer-only or with any roster it likes.

### Verifier (optional)

A Verifier stream is the independent check that the other streams' landed or claimed work is **correct, complete, consistent, and faithful to what the user asked**. If the project keeps a documentation stream, documentation becomes the _byproduct_ of verification -- you cannot faithfully document a system that does not do what it claims. The Verifier has **advisory authority only**: it posts findings, the user adjudicates, the owning stream fixes. A Verifier never mutates another stream's code and never blocks its progress.

**Sources -- the triangle.** The Verifier checks three things against each other rather than trusting any one:

- **Ask** -- what the user actually asked: the target stream's Claude Code session transcript at `~/.claude/projects/<project-dir>/<current_session_id>.jsonl` (`current_session_id` is in that stream's whiteboard frontmatter; re-resolve it each audit, since it rotates on the peer's `/compact` or restart). Read it _targeted_ -- tail, grep, or a sub-agent sweep -- never whole; transcripts are large.
- **Plan** -- the stream's plan file at `~/.claude/plans/<name>.md` (streams usually cite the name in `focus:`).
- **Reality** -- the whiteboard + `intent/st/**` + the code + the tests.

**Method.**

- **Fire on claim.** Audit when a stream claims _done / closed / frozen / green_, at WP/ST close, or on the user's request -- not continuously, and not on in-flight edits (a half-written tree yields noise). A "done" claim is the trigger; a frozen-but-wrong artifact gets unfrozen and fixed, never worked around.
- **Read the as-built, never the narrative.** Evidence is `file:line` from a real read. No invented line numbers; no "certainly" without having read the code.
- **Classify every finding** -- expected-vs-real (queued-but-unbuilt vs falls-between-the-cracks), severity, and evidence. The expected-vs-real split is what keeps findings high-signal.
- **Self-refute high-severity findings first.** Try to _kill_ your own finding (did I miss where this already lives? is it a partial, not a contradiction?) before posting. A finding that survives your own attack is worth raising.
- **Advisory output.** Findings go to `asks.md` (plus an optional ledger in the Verifier's own stream file). A high-severity _compounding_ risk -- a false "done" the next unit of work would build on -- escalates to the user directly rather than waiting in the queue. Never mutate another stream's code.
- **Audit your own coverage.** State what you checked AND what you did not. No silent caps; the auditor stays auditable.

The Verifier is a _role a stream adopts_, not a subcommand. A project enables it by designating one stream as Verifier in that project's `whiteboard/README.md`. A `verify <stream>` subcommand that runs a pass on demand is possible future work, not part of this version.

## Protocol invariants

1. The whiteboard is the _live_ coordination channel. `wip.md` is the post-session snapshot. Different tense, different reader.
2. Claims are by ST ID only, never by glob path. Paths inferable from `intent/st/<ID>/info.md` cross-references.
3. `apps/lamplight/**` (or the equivalent shared platform layer) is everybody's territory; coordinate via `lamplight.md`.
4. ST-ID collisions are an `intent st new` race (max+1 with no lock), not solvable here. Surface the collision when noticed; let the user arbitrate.
5. Heartbeat older than 7 days marks a claim reclaimable -- reclaim requires explicit user acknowledgement.
6. `/compact` does NOT end the session; status stays `active` across `/compact`. The next `/in-session` (auto-firing in the new context) calls `pickup` which touches the heartbeat.
7. History buckets (`history/<YYYYMMDD>.<file>`) are Monday-anchored by the ISO week of the archived CONTENT's date, append-only, and never reloaded on `pickup`. `archive` moves verbatim DONE content there to keep live files lean (frontmatter + current + standing sections + a pointer); git history remains the ultimate trace.

## Why this exists

Concurrent sessions need a live coordination surface. `wip.md` works for post-session snapshots but loses fidelity _during_ a session -- decisions, asks, platform edits invisible until next wrap-up. The whiteboard is the missing live channel: each stream owns a file, decisions go in your file, asks go in the shared file, platform edits go in the platform file. Read on pickup, append during work, release on finish.

## Red Flags

| Rationalisation                                                  | Reality                                                                                                         |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| "Only one session active right now -- coordination is overhead"  | The whiteboard is also a journal for future-you. Write it.                                                      |
| "I'll edit `apps/lamplight` without noting it in `lamplight.md`" | The other stream may be editing the same files. One-line ask is cheap insurance.                                |
| "/compact ends the session, so I'll set `status: paused`"        | No. `/compact` is transparent. Status stays active; `/in-session` re-fires `pickup`.                            |
| "I'll glob-claim `apps/control/**` to be safe"                   | Claims are by ST only. Globs drift; STs are the user's mental model.                                            |
| "The other stream's claim is stale; I'll just steal it"          | Reclaim requires explicit user acknowledgement. Don't silently overwrite.                                       |
| "I'll `archive` the other stream's file too while I'm here"      | Only if that stream is paused (or the user said so). Archiving a live peer's file collides on the shared index. |
| "`archive` is just a date filter, I'll script it"                | It's judgment-guided: an old-but-OPEN ask stays; a recent-but-superseded block can go. Read each entry.         |
| "The stream said it's done, so it's done."                       | A "done" claim is the _trigger_ to verify, not the verdict. Read the as-built against the ask.                  |
