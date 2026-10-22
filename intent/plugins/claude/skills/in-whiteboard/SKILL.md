---
description: "Multi-session coordination through `intent wb`: per-node boards and single-writer inboxes in the store, claim threads, work packages and issues, broadcast, heartbeat, release"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination (Protocol 3.0)

Coordinates several Claude Code sessions, and the human, on one Intent project. Each participant is a **node** with its own board. **`intent wb` is the only door**: the files under `intent/whiteboard/<node>/` are rendered views of the store. Every board and every inbox has one writer: a verb writes only the acting node's board, and a message is recorded as sent by the acting node, which is whatever `--node` names. The whiteboard is the live channel; `intent/wip.md` is the post-session snapshot.

**This file carries what a verb cannot**: when a verb is the wrong thing to run, and what must be true first. The measurements and incidents behind each rule are in this file's git history, in the version before it was cut to the rules.

## Nodes

A node is a participant; its 2-letter moniker is the routing key and its directory's name. The roster is per-project: `intent/whiteboard/README.md` declares it, `intent wb status` shows who is registered, and **who the participants are is a human's declaration**, so an agent never registers a roster unasked.

### The hv (hypervisor) node

The human is a node, conventionally `hv`, never named in protocol language. It adjudicates scope, sequences work, owns releases, and receives escalations.

- **No session loop**: its `session_id` is conventionally `none`, and its heartbeat is advisory: a stale `hv` heartbeat marks nothing reclaimable, because the human is always authoritative.
- **Standing directives**: `intent wb add directive "<text>" --node hv`. They render on `hv`'s board alone, every node honours them, no other node can write one, and a fold never archives one.
- **Its inbox is the durable escalation surface**, required whenever a node cannot reach the human live (reachability belongs to a run, not a project). **A write surface with no named reader is a queue, not a channel**: the README MUST name who reads `hv/inbox.*` and surfaces it to the human. An escalation is done when that reader has it, not when the write returns; if nobody is named, fix that first.

## The verbs at a glance

Every verb that writes as a node takes `--node <moniker>`; `status`, `show <node>` and `register` do not, and `migrate` takes the board as a positional.

| What you want                                        | The verb                            |
| ---------------------------------------------------- | ----------------------------------- |
| Start a session                                      | `intent wb pickup`                  |
| See where every node stands                          | `intent wb status`                  |
| Read one node's whole board                          | `intent wb show <node>`             |
| Message one node                                     | `intent wb ask <node> <body>`       |
| Broadcast to every peer                              | `intent wb announce <body>`         |
| Record what you are doing, or a watch-out, or a hold | `intent wb add <kind> <text>`       |
| Record a cross-node decision                         | `intent wb decide <text>`           |
| Take or drop a claim                                 | `intent wb claim` / `unclaim <id>`  |
| Retire one of your own items                         | `intent wb archive <kind> <seq>`    |
| Change or redact your item or a message you sent     | `intent wb edit <kind> <id> <text>` |
| Mark one sender's messages handled                   | `intent wb clear <sender>`          |
| Say you are still alive                              | `intent wb touch`                   |
| End a session                                        | `intent wb release`                 |
| Put the project's nodes on the board                 | `intent wb register`                |
| Carry a hand-authored board into the store           | `intent wb migrate <node>`          |

`ask` also takes `--re <anchor>` (thread a reply) and `--fyi` (no reply expected). `pickup`, `status` and `show` take `--json`; `pickup` and `show` take `--all` (handled messages too). `pickup --focus <line>` is the only way to set the focus line; `--session <id>` records the session id (default `CLAUDE_CODE_SESSION_ID`).

**Not every verb is one an agent reaches for unasked.** Each row's `exposed_on_mcp` follows its recoverability: reads, and writes whose repeat changes nothing, are offered; a verb that accumulates something permanent is done because you were asked. Exceptions say why in `recoverability_anomaly` (eg `wb register`). Read the field, never a list.

## File layout

```
intent/whiteboard/
  README.md                 # protocol reference + the project's node roster (hand-authored)
  <node>/
    board.json              # the node's canon, tool-written: its items, the messages it SENT, its handled marks
    wip.md                  # generated view: header block + DOING + TODO + Holds + Watch-outs + Decisions
    inbox.<sender>.md       # generated view, one per OTHER registered node: messages FROM that sender, rendered from the sender's board.json alone
    .history/
      .gitkeep              # tracks the otherwise-empty archive dir (git ignores empty dirs)
      YYYYMMDD/             # the hand-authored era's fold archives; not written any more
      pre-migration/        # the copies `wb migrate` keeps of a hand-authored wip.md and any inbox it dropped a line from
```

**These files are generated views: never edit them.** A hand edit is skew that `intent doctor` reports and the next render drops. **Archived is a state, not a directory**: the row keeps its number and text, stays readable, and stops counting against the live bound. Old `.history/` directories are the hand-authored era's record, never reloaded.

**A node joins by being registered**: `intent wb register <moniker> --name "<display name>" --role <role>`. An unchanged repeat writes nothing, a changed value is refused, and `--correct` changes name and role on purpose; with no arguments it registers from each hand-authored board's header. `intent wb migrate <node>` carries a hand-authored board into the store, refusing every line it cannot carry unless `--drop-uncarried` (dropped lines kept under `<node>/.history/pre-migration/`). Until then, every write to its board and every message addressed to it is refused.

## wip.md shape

```
---
node: <moniker>
name: <display name>
role: <role>
session_id: <UUID|none>
heartbeat_at: <UTC, read from `date -u` -- see "Every timestamp is READ FROM A CLOCK">
status: active | paused
focus: "<one-line current goal>"
claims: [STxxxx, ...]
---
# <Name> (<node>)
## DOING        -- in-flight work (archived, as a state, when it is finished with)
## TODO         -- queued / next
## Holds        -- work you are NOT doing, each with the CONDITION that releases it
## Standing directives -- hv's board only (see "The hv (hypervisor) node")
## Watch-outs   -- durable cautions peers should know (standing; not archived)
## Decisions    -- cross-node decisions, broadcast by being read at pickup
```

### `## Holds` -- the condition is the required field, not the item

**A hold carries the condition that releases it, and the condition is the content.** _Holding 0162 until the shared daemon is free_ is a hold; _holding 0162_ is an item that left DOING and entered nothing, indistinguishable later from work quietly dropped. Name what must become true (checkable by you at every pickup), not who you wait on. A hold whose condition cannot be written down belongs in TODO with a reason, or off the board. Check every hold at pickup; move released ones back to TODO.

### The header block is NOT YAML

A line-oriented `key: value` block; these rules are the whole specification:

- **One line per key**: the value runs from the first `: ` to the end of the line. No multi-line values, nesting or comments.
- **The double quote is a display delimiter**: one surrounding pair is stripped; quotes inside are literal and **never escaped** (`\"` puts a backslash on the board). Single quotes are never stripped.
- **`claims:`** is a comma-separated list in square brackets.

On a generated board the renderer writes it. The header guard refuses escapes in a hand-authored block; the fix is to stop treating it as YAML, never a better escape.

## inbox.<sender>.md shape

`<recipient>/inbox.<sender>.md` holds what `<sender>` sent `<recipient>`, rendered from `<sender>/board.json` alone, so the sender is its one writer; the recipient reads it and clears. An unused pair shows `# inbox: <sender> -> <recipient>` over `_(empty)_`. `clear` removes nothing and changes no inbox file: it puts a mark on each message in the recipient's own `board.json`, the message leaves the live count, and `wb pickup` and `wb show --all` say what is handled. The file never does.

```
## (YYYY-MM-DD HH:MMZ) [claimed <stamp>] [Re: <prior-anchor>] [FYI only -- no response needed.] [(edited)]

<text>
```

The minute stamp is the anchor a reply threads against (`--re`). `Re:` marks a reply; `FYI only` marks no reply expected, and its absence means one is. A reply is a new entry in the opposite inbox. The renderer adds `claimed` (a migrated entry's claimed stamp) and `(edited)`; handled state is the recipient's and is not in the file. Every `announce` is FYI. Field separators are not significant.

## Every timestamp is READ FROM A CLOCK, never written from memory

**Every stamp `intent wb` writes is read by the service at the write**; no verb takes one. What you write by hand (a date in a message, a commit, a report, a hand-authored board) goes in only when this read is in front of you in this turn, verbatim:

```
date -u +'%Y-%m-%d %H:%MZ'
```

**A timestamp not read off a clock is fabricated, not approximate**: a node has no clock, a fake stamp is indistinguishable from a real one, and one destroys the board's only cross-node ordering.

- **Trailing `Z` is mandatory, and appending it is an assertion.** Read an unmarked stamp as local, its ordering unreliable. `git log`, `ls -la` and `stat` print local time; `git log --date=format:` ignores `TZ` (`--date=format-local:` with `TZ=UTC` honours it). Otherwise `date -u` or `date -u -r <epoch>`, or say nothing.
- **A stamp copied from the last one you read is fabricated too**: re-run `date -u` for each.
- **Never rewrite a peer's stamp** (flag it); **never repair your own fake with a better-looking one** (annotate it unverifiable).
- **The pre-commit whiteboard guards** refuse a future stamp, a missing `Z`, and an inbox going backwards, on what the commit adds, and print the right value rather than fix it. A green is not proof: a fake with a `Z`, in the past and increasing, passes.

## Node-identity discovery

Every writing verb takes `--node <moniker>`, which the tool cannot work out. Use the moniker the invocation carries; else infer it (the session's name, the working directory, the user's framing, which board names this session); else ask before writing. **A write under the wrong moniker is a write on somebody else's board**: the single-writer rule is enforced against the moniker passed. Later sessions of the node inherit it.

## Each verb, and the judgement it cannot carry

- **`pickup`** moves your heartbeat, then prints your board and live messages (handled ones as a count), `hv`'s directives, watch-outs and decisions in full, and each peer's header (`intent wb show <peer>` reads one whole board; every board is readable by every node). **It cannot read your inboxes FOR you: surface what came in to the user.** An entry nobody mentions is an entry nobody handles.
- **`ask`** returns when the message is stored, not when a reader has acted on it. **Durable things go in the inbox; things that must be ACTED ON get said twice**: in the inbox and again on a live channel. A reply is a new message the other way, carrying `--re`.
- **`announce`** is one FYI to every other node, for 1-to-all signals (a shared layer you are about to touch, a protocol change). A thing needing an answer is an `ask`.
- **`add <doing|todo|watchout|hold|directive>`**: the service assigns the seq. A hold needs its condition. `directive` is `hv`'s alone; `decision` is redirected to `decide`.
- **`decide`** puts a decision on your own board, where peers read it at pickup; no recipients, so no copies to diverge.
- **`claim` / `unclaim`** take a thread as `ST0000`, a work package as `ST0000/01`, or an issue as `ISSUE:0000`. **Check who holds it first** (`intent wb status --json`); if an active peer does, surface the overlap to `hv` rather than claiming alongside.
- **`clear <sender>`** marks that sender's live messages handled, in your own `board.json`; no inbox file changes. **Clear an entry only if it was answered, actioned, or re-stated live on your board. Reading is not handling.** A cleared-but-unactioned entry makes the board affirmatively WRONG, and no guard can catch it: actioned-ness is judgement. Re-count live messages right before clearing, or a message that landed after your read is swept too.
- **`archive <kind> <seq>`** retires your own item (seqs are numbered per kind). **Archived is a state, never a deletion**; the answer says what moved. Whether the item IS finished with is your judgement (see Folding).
- **`edit <kind> <id> <text>`** changes your own item, live or archived; `edit message "<anchor>" "<text>" --to <recipient>` changes a message you sent (`<anchor>#<n>` for one of several in a minute; one copy of an announce edits all). An uncommitted event is amended in place; a committed one gets a `wb.edit` event and git keeps the old text. The answer names every file under `intent/` still holding the old text (HEAD, staged, unstaged) and gives the all-clear only when both searches found nothing. Edits render `(edited)`, never the old text.
- **`touch`** stamps your heartbeat, nothing else; no caller supplies the time.
- **`release`** sets you paused and stamps, so a clean stop differs from a dead session. **Session end only**: never for a localfold, a compact or a context reset.
- **`status`** is one line per node (role, name, status, heartbeat, live counts, focus); **`show <node>`** is one board. Neither writes.

### Folding: localfold and globalfold

**localfold** = tidy your OWN node before a compact: archive what is finished with, clear what you have handled. **It does NOT `release`.** **globalfold** = the project-wide snapshot (`intent/wip.md`, `restart.md`, `done.md`), usually the coordinating or validation node's job.

- **An unexecuted ruling is live state, not history.** A fold archives the NARRATIVE of a ruling, never the ruling while it is unexecuted. Execution is the test, not the date; verify it against the ARTEFACT, not the board that records it.
- **A hold is not archived while its condition stands unmet**; a released one goes back to TODO (`wb archive hold <seq>`, then `wb add todo`).
- **A fold never archives a directive**; it leaves `hv`'s board when `hv` says it is spent.

## Node roles

Per-project; the README is the source of truth. A common shape: a **control** node doing the heavy lifting, a **validation** node, others as needed, and `hv`.

### Validation / Verifier (optional)

Checks that other nodes' landed or claimed work is **correct, complete, consistent, and faithful to what `hv` asked**. Advisory only: it posts findings, `hv` adjudicates, the owner fixes; it never mutates another node's code or blocks it.

- **Sources, the triangle**: the ASK (the node's transcript, `~/.claude/projects/<project-dir>/<session_id>.jsonl`, read targeted; re-resolve `session_id` each audit), the PLAN (its plan file, often cited in `focus:`), REALITY (the whiteboard, `intent/st/**`, code, tests).
- **Method**: fire on a claim (done, closed, frozen, green), not on in-flight edits; read the as-built, not the narrative (`file:line` from a real read); classify each finding (queued or fell through the cracks, severity, evidence); refute your own HIGH findings before posting; findings go to the owner's inbox, a compounding false "done" to `hv`'s; say what you checked AND what you did not.

## Protocol invariants

1. **One writer per file**: a board's `board.json` and `wip.md` are its node's, and `<recipient>/inbox.<sender>.md` is its sender's; only the recipient clears, by marking in its own `board.json`. Enforced against the moniker you pass, not who you are.
2. **Live channel, not snapshot**: `<node>/wip.md` is live; `intent/wip.md` is the snapshot.
3. **Claims by ST ID**, never glob paths.
4. **Broadcast via `announce`**, never a shared file; announce before touching a shared platform layer.
5. **A heartbeat older than 7 days marks a claim reclaimable**, with `hv`'s explicit acknowledgement; never `hv`'s own.
6. **`/compact` does NOT end a session**: status stays `active`; the next `pickup` touches the heartbeat.
7. **You archive your own items only**; old `.history/YYYYMMDD/` trees are append-only, never reloaded.
8. **The human is `hv`**, never named.
9. **You commit what your verbs wrote, as each verb prints it on its `wrote:` lines**: your own `board.json` and `wip.md`, every `<peer>/inbox.<you>.md`, and your own event files. A `re-rendered:` line names another node's file and who owns it; leave it to that node. The one exception is the re-render a new board shape forces on every board at once, which is one mechanical commit by the coordinating node or `hv`, outside this rule. `wb register` and `wb migrate` are roster acts: each prints everything it rendered on `wrote:` lines, and whoever runs it commits all of it.

## Red Flags

| Rationalisation                                             | Reality                                                                                         |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| "I'll edit a peer's `wip.md` to correct it."                | Never. You write only your own node. Send an `ask` to its inbox.                                |
| "/compact ended the session, so I'll set `status: paused`." | No. `/compact` is transparent. Status stays active; `/in-session` re-fires `pickup`.            |
| "This ruling is dated, so the fold archives it."            | Only if it was EXECUTED. Unexecuted, it is todo work, and `.history/` is where nobody looks.    |
| "I read the inbox entry, so `clear` is just hygiene."       | Reading is not handling. Answer it, action it, or re-state it live -- then clear.               |
| "I am holding this item." (no condition)                    | Then it is not a hold. Name what has to become true, or it reads as abandoned at every pickup.  |
| "The node said it's done, so it's done."                    | A "done" claim is the _trigger_ to verify, not the verdict. Read the as-built against the ask.  |
| "I know roughly what time it is."                           | You do not. You have no clock. Run `date -u`; a plausible stamp is fabricated, not approximate. |
| "I'll just edit the board file, it is right there."         | It is a rendered view. Your edit is skew doctor reports, and the next render drops it.          |
| "The verb returned ok, so the peer has it."                 | Stored is not delivered. A thing that must be acted on gets said twice.                         |
