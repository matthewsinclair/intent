---
description: "Multi-session coordination through `intent wb`: per-node boards and single-writer inboxes in the store, claim threads, work packages and issues, broadcast, heartbeat, release"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination

Coordinates several Claude Code sessions, and the human, on one Intent project. Each participant is a **node** with its own board. **`intent wb` is the only door**: the files under `intent/whiteboard/<node>/` are rendered views of the store.

**This file carries what a verb cannot**: when a verb is the wrong thing to run, and what must be true first. The measurements and incidents behind each rule are in this file's git history, in the version before it was cut to the rules.

## Nodes

A node is a participant; its 2-letter moniker is the routing key and its directory's name. The roster is per-project: `intent/whiteboard/README.md` declares it, `intent wb status` shows who is registered, and **who the participants are is a human's declaration**, so an agent never registers a roster unasked.

### The hv (hypervisor) node

The human is a node, conventionally `hv`, never named in protocol language. It adjudicates scope, sequences work, owns releases, and receives escalations.

- **No session loop**: its `session_id` is conventionally `none`, and its heartbeat is advisory: a stale `hv` heartbeat marks nothing reclaimable, because the human is always authoritative.
- **Standing directives**: `intent wb add directive "<text>" --node hv`. They render on `hv`'s board alone, every node honours them, and no other node can write one.
- **Its inbox is the durable escalation surface**, required whenever a node cannot reach the human live (reachability belongs to a run, not a project). **A write surface with no named reader is a queue, not a channel**: the README MUST name who reads `hv/inbox.*` and surfaces it to the human. An escalation is done when that reader has it, not when the write returns; if nobody is named, fix that first.

## The verbs

The help for `intent wb` lists every verb, and each verb's own help its flags. Every verb that writes as a node takes `--node <moniker>`; `status`, `show <node>` and `register` do not, and `migrate` takes the board as a positional. Two things the help does not say: `pickup --focus <line>` is the only way to set the focus line, and `pickup`, `status` and `show` take `--json` for a script.

**Not every verb is one an agent reaches for unasked.** Each row's `exposed_on_mcp` follows its recoverability: reads, and writes whose repeat changes nothing, are offered; a verb that accumulates something permanent is done because you were asked. Exceptions say why in `recoverability_anomaly` (eg `wb register`). Read the field, never a list.

## The files

`intent/whiteboard/<node>/` holds the node's `board.json`, its canon, and the views rendered from it: `wip.md` (a header block, then DOING, TODO, Holds, Watch-outs and Decisions, and on `hv`'s board its standing directives) and one `inbox.<sender>.md` per other registered node. **These files are generated views: never edit them.** A hand edit is skew that `intent doctor` reports and the next render drops. An inbox holds what one sender sent, rendered from the sender's `board.json` alone, so the sender is its one writer; `clear` marks messages handled in the recipient's own `board.json` and changes no inbox file. **Archived is a state, not a directory**: the row keeps its number and text, stays readable, and stops counting against the live bound. `.history/` holds the hand-authored era's archives and the copies `wb migrate` keeps, and is never reloaded.

**A node joins by being registered**: `intent wb register <moniker> --name "<display name>" --role <role>`. An unchanged repeat writes nothing, a changed value is refused, and `--correct` changes name and role on purpose. `intent wb migrate <node>` carries a hand-authored board into the store, refusing every line it cannot carry unless `--drop-uncarried`. Until then, every write to its board and every message addressed to it is refused.

### `## Holds` -- the condition is the required field, not the item

**A hold carries the condition that releases it, and the condition is the content.** _Holding 0162 until the shared daemon is free_ is a hold; _holding 0162_ is an item that left DOING and entered nothing, indistinguishable later from work quietly dropped. Name what must become true (checkable by you at every pickup), not who you wait on. A hold whose condition cannot be written down belongs in TODO with a reason, or off the board. Check every hold at pickup; move released ones back to TODO.

### The header block is NOT YAML

`wip.md`'s header is a line-oriented `key: value` block the renderer writes: one line per key, the value running from the first `: ` to the end of the line, one surrounding pair of double quotes stripped as a display delimiter, and quotes inside literal and **never escaped** (`\"` puts a backslash on the board). `claims:` is a comma-separated list in square brackets. The header guard refuses escapes in a hand-authored block; the fix is to stop treating it as YAML, never a better escape.

### An inbox entry's heading

`## (YYYY-MM-DD HH:MMZ) [Re: <prior-anchor>] [FYI only -- no response needed.] [(edited)]`: the minute stamp is the anchor a reply threads against with `--re`. `Re:` marks a reply; `FYI only` marks no reply expected, and its absence means one is. Every `announce` is FYI.

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

A node session is started with `intent claude start <node>`, which binds it to its workstream, and `intent estate status` shows every Claude session on the machine, node or not. The tool cannot work out `--node <moniker>`: use the moniker the invocation carries; else infer it (the session's name, the working directory, the user's framing, which board names this session); else ask before writing. **A write under the wrong moniker is a write on somebody else's board**: the single-writer rule is enforced against the moniker passed. Later sessions of the node inherit it.

## Each verb, and the judgement it cannot carry

- **`pickup`** moves your heartbeat, then prints your board and live messages (handled ones as a count), `hv`'s directives, watch-outs and decisions in full, and each peer's header (every board is readable by every node). **It cannot read your inboxes FOR you: surface what came in to the user.** An entry nobody mentions is an entry nobody handles.
- **`ask`** returns when the message is stored, not when a reader has acted on it. **Durable things go in the inbox; things that must be ACTED ON get said twice**: in the inbox and again on a live channel.
- **`announce`** is one FYI to every other node, for 1-to-all signals (a shared layer you are about to touch, a protocol change). A thing needing an answer is an `ask`.
- **`add <doing|todo|watchout|hold|directive>`**: the service assigns the seq. `directive` is `hv`'s alone; `decision` is redirected to `decide`.
- **`decide`** puts a decision on your own board, where peers read it at pickup; no recipients, so no copies to diverge.
- **`claim` / `unclaim`** take a thread as `ST0000`, a work package as `ST0000/01`, or an issue as `ISSUE:0000`. **Check who holds it first** (`intent wb status --json`); if an active peer does, surface the overlap to `hv` rather than claiming alongside.
- **`clear <sender>`** marks that sender's live messages handled. **Clear an entry only if it was answered, actioned, or re-stated live on your board. Reading is not handling.** A cleared-but-unactioned entry makes the board affirmatively WRONG, and no guard can catch it: actioned-ness is judgement. Re-count live messages right before clearing, or a message that landed after your read is swept too.
- **`archive <kind> <id>`** retires your own item (ids are numbered per kind); the answer says what moved. Whether the item IS finished with is your judgement (see Folding).
- **`archive message <anchor> --to <recipient>`** retires a message you SENT once its recipient has handled it: the recipient's inbox view drops it, and `wb show --all` and the JSON still carry it. Refused while the recipient holds it live, because only the recipient clears; one copy of an announce at a time; `--handled` archives everything you sent that its recipients have handled. A board carrying an archived message needs Intent 3.2.6 or later, so archive nothing while another machine reads the estate's boards with an older Intent.
- **`edit <kind> <id> <text>`** changes your own item, live or archived; `edit message "<anchor>" "<text>" --to <recipient>` changes a message you sent (`<anchor>#<n>` for one of several in a minute; one copy of an announce edits all). An uncommitted event is amended in place; a committed one gets a `wb.edit` event and git keeps the old text. The answer names every file under `intent/` still holding the old text (HEAD, staged, unstaged) and gives the all-clear only when both searches found nothing. Edits render `(edited)`, never the old text.
- **`touch`** stamps your heartbeat, nothing else.
- **`release`** sets you paused and stamps, so a clean stop differs from a dead session. **Session end only**: never for a localfold, a compact or a context reset.
- **`status`** and **`show <node>`** write nothing.

### Folding

`/in-finish` defines localfold and globalfold. On a board, a localfold archives what is finished with, clears what you have handled, and archives what you sent that its recipients have handled (`wb archive message --handled`). **It does NOT `release`.**

- **An unexecuted ruling is live state, not history.** A fold archives the NARRATIVE of a ruling, never the ruling while it is unexecuted. Execution is the test, not the date; verify it against the ARTEFACT, not the board that records it.
- **A hold is not archived while its condition stands unmet**; a released one goes back to TODO (`wb archive hold <id>`, then `wb add todo`).
- **A fold never archives a directive**; it leaves `hv`'s board when `hv` says it is spent.
- **A landed unit's worktree is removed by its author**, at the chain END that lands the unit, or at the author's next fold if it is still on disk. Every check runs from outside the worktree, because a session that removes the tree it stands in loses its working directory:
  - its index tree is its base with its landed banks applied;
  - nothing in it is unstaged or untracked;
  - its base is an ancestor of main;
  - no stash entry names it, because the stash is the repository's and outlives the worktree.

  Then `git worktree remove <path>`, never with `--force`: read a refusal as a finding. A re-bank that supersedes a worktree removes the old one the same way.

## Node roles

Per-project; the README is the source of truth. A common shape: a **control** node doing the heavy lifting, a **validation** node that checks landed or claimed work and posts findings for `hv` to adjudicate (advisory only: it never mutates another node's work or blocks it), others as needed, and `hv`.

## Protocol invariants

1. **One writer per file**: a board's `board.json` and `wip.md` are its node's, and `<recipient>/inbox.<sender>.md` is its sender's; only the recipient clears, by marking in its own `board.json`. Enforced against the moniker you pass, not who you are.
2. **Live channel, not snapshot**: `<node>/wip.md` is live; `intent/wip.md` is the snapshot.
3. **Claims by ST ID**, never glob paths.
4. **Broadcast via `announce`**, never a shared file; announce before touching a shared platform layer.
5. **A long-stale heartbeat marks a claim reclaimable only with `hv`'s explicit acknowledgement**; never `hv`'s own.
6. **`/compact` does NOT end a session**: status stays `active`; the next `pickup` touches the heartbeat.
7. **You archive your own items only**; old `.history/YYYYMMDD/` trees are append-only, never reloaded.
8. **The human is `hv`**, never named.
9. **You commit what your verbs printed, as `/in-essentials` says**: on a board that is your own `board.json` and `wip.md`, every `<peer>/inbox.<you>.md`, and your own event files. The one exception is the re-render a new board shape forces on every board at once, which is one mechanical commit by the coordinating node or `hv`, outside this rule. `wb register` and `wb migrate` are roster acts: each prints everything it rendered on `wrote:` lines, and whoever runs it commits all of it.

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
