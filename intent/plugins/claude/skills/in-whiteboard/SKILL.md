---
description: "Multi-session coordination through `intent wb`: per-node boards and single-writer inboxes in the store, claim threads, work packages and issues, broadcast, heartbeat, release"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination (Protocol 3.0)

Coordinator for multiple Claude Code sessions -- and the human -- running concurrently against one Intent project. Each participant is a **node** with its own board. **`intent wb` is how you read and write it**, and the files under `intent/whiteboard/<node>/` are rendered views of what the store holds. Every board and every inbox has exactly one writer: no verb writes any board but the acting node's own, and a message is always recorded as sent by the acting node. The acting node is whatever `--node` names, so the rule holds only as far as every session passes its own moniker; that single-writer rule is what makes the board contention-free and cleansable. The whiteboard is the _live_ channel; `intent/wip.md` is the post-session snapshot.

**Protocol 3.0** supersedes 2.0 (flat shared `asks.md` + per-stream files). v3.0 = per-node directories + a single-writer inbox model + the human as a first-class `hv` (hypervisor) node.

## Nodes

A node is a participant. The 2-letter moniker is the routing key, the handle, and the name of its rendered directory. Nodes are **per-project configuration**: the project declares its roster (monikers, display names, roles) in its hand-authored `intent/whiteboard/README.md`, and `intent wb register` is what puts that declaration on the board. No roster is baked into this skill -- `intent wb status` is how you find out who the nodes are.

A project that wants the human in the loop gives them a node, conventionally `hv` (the **hypervisor**): the human who adjudicates scope, sequences work, owns releases, and is where escalations land. The human is addressed as `hv` in all protocol language, never by name. The hypervisor node is human-driven -- it is read like any other node, but the human maintains it (or has it maintained on their behalf) rather than running `pickup` on a heartbeat.

### The hv (hypervisor) node

`hv` is structurally a node like any other -- a `<hv>/wip.md` peers read at pickup, inboxes peers append to -- with three differences that follow from being human-driven:

- **No session loop.** `hv` is not driven by `/in-session` / `pickup`, so its `session_id` is optional and conventionally `none`. Peers therefore never match it on the "different `session_id`" active-peer test; they read it for its directives and route escalations to `hv/inbox.<you>.md`.
- **Heartbeat is advisory.** A stale `hv` heartbeat does not mark anything reclaimable -- the human is always authoritative -- so the 7-day reclaim rule does not apply to `hv`.
- **Standing directives.** Beyond the canonical `wip.md` body, `hv` may carry a `## Standing directives` section: durable instructions every node honours (sequencing, scope rulings, release policy). Peers read it at pickup the way they read `## Decisions`. On a generated board a standing directive is its own item kind, `directive`: `intent wb add directive "<text>" --node hv` writes one, the board view renders it under `## Standing directives` on `hv`'s board alone, the same verb on any other node is refused by name, and a fold never archives one.

#### What the hv inbox is FOR, and who is obliged to read it

**The `hv` inbox is the DURABLE escalation surface: the record that survives when a node cannot reach the human live.** Everything above describes its structure and none of it says that, and the omission is not cosmetic -- **a channel described only by its shape gets used as a queue and mistaken for a delivery.**

**It is required wherever a node cannot reach the human on a live channel.** Where a live channel does exist it is redundant for that exchange, not for the project: reachability is a property of a RUN, not of a project. The same human is reachable during an interactive session and unreachable at 3am, so a project-level exemption would retire the durable surface at exactly the moments it was designed for.

**AND A WRITE SURFACE WITH NO NAMED READER IS A QUEUE, NOT A CHANNEL.** So the project's `README.md` roster MUST name who is obliged to read `hv/inbox.*` and surface its contents to the human. Without that, writing succeeds every time, delivery never happens, and **nothing observable distinguishes the two** -- measured on this protocol in August 2026: nodes wrote correctly into `hv` inboxes for days, in the right format, and the human was reading none of it. Not one write failed, so nothing reported the gap.

Intent's own roster names the validation node, in the human's words: _the workstreams can write in the hv channel FOR me, but I need that stuff surfaced TO me by vc._ **The obligation is what matters, not the mechanism** -- a reader, a bot, a scheduled sweep and a store trigger all satisfy it, and a project that keeps the whiteboard in a database rather than in files owes exactly the same thing.

**A node reporting an escalation is not finished when the write returns.** It is finished when a named reader has it. If the roster names nobody, that is the defect to fix first -- ahead of whatever was being escalated.

## The verbs at a glance

**EVERY ONE OF THESE IS A COMMAND, AND `intent wb` IS THE ONLY DOOR.** They read and write the coordination model in the store; the files under `intent/whiteboard/` are rendered views of it. Every verb that writes as a node takes `--node <moniker>` to name the node acting; `status`, `show <node>` and `register` do not, and `migrate` names the board it carries as a positional.

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

`intent wb ask` also takes `--re <anchor>` to thread a reply and `--fyi` to say no reply is expected. `pickup`, `status` and `show` take `--json`. `pickup` and `show` also take `--all`, which lists the handled messages a default read only counts. `pickup` also takes `--focus <line>`, which records what the node is on in its header and is the only way to set `focus:` on a generated board, and `--session <id>`, which records the session id; without `--session` it records the `CLAUDE_CODE_SESSION_ID` the process runs with.

**NOT EVERY VERB IS ONE AN AGENT MAY REACH FOR UNASKED, AND THE RULE IS A FIELD RATHER THAN A LIST.** Each row declares `exposed_on_mcp`, and that is what decides whether the verb is offered on the tool tier. It follows the row's `recoverability`: reads and writes whose second call changes nothing are offered, and a verb that ACCUMULATES something permanent is not. A row that departs from that records why in `recoverability_anomaly`, as `wb register` does: it is idempotent and withheld anyway. So reading a board, stamping a heartbeat, claiming a thread and marking a sender's messages handled are ordinary; putting a message, a decision or an item on a board is a thing you do because you were asked to, and registering who the participants of a project ARE is a human's declaration. **Read the field, never a list of names** -- a sentence here naming which verbs are which would go stale, silently, the first time one row's field moved, and the split is a consequence rather than a policy.

**WHAT THIS FILE IS FOR, NOW THAT THE VERBS EXIST, IS THE HALF A COMMAND CANNOT CARRY**: when a verb is the wrong thing to run, and what has to be true before you run it. A verb enforces its own shape -- the bound, the single writer, the clock -- and cannot know whether an inbox entry was actually handled or whether a ruling has been executed. That judgement is below, and it is the reason this skill is longer than the verb list.

## File layout

```
intent/whiteboard/
  README.md                 # protocol reference + the project's node roster (hand-authored)
  <node>/
    board.json              # the node's board as the store carries it: canon, tool-written
    wip.md                  # generated view: header block + DOING + TODO + Holds + Watch-outs + Decisions
    inbox.<sender>.md       # generated view, one per OTHER registered node: messages FROM that sender
    .history/
      .gitkeep              # tracks the otherwise-empty archive dir (git ignores empty dirs)
      YYYYMMDD/             # the hand-authored era's fold archives; not written any more
      pre-migration/        # the copies `wb migrate` keeps of a hand-authored wip.md and any inbox it dropped a line from
```

**THESE FILES ARE GENERATED VIEWS AND YOU DO NOT EDIT THEM.** The board and every inbox are rendered from the store, so a hand edit is not a write -- it is skew, and `intent doctor` reports it as skew. `intent wb` is what changes a board; the file is what the change looks like afterwards. The shapes below are documented because you READ them constantly, not because you author them.

**`.history/` IS NO LONGER WHERE ARCHIVED CONTENT GOES.** Archived is a STATE an item or a message carries, not a directory it moves to: the row keeps its number and its text and stays readable, it just stops counting against the live bound. `intent wb archive` and `intent wb clear` are that transition. Existing `.history/` directories stay as the record of the hand-authored era and are not reloaded on pickup, exactly as before.

**A NODE JOINS BY BEING REGISTERED, AND ITS BOARD AND INBOXES RENDER FROM THAT ROW.** There is no directory to create and no file to seed: the row is the node, and everything under `intent/whiteboard/<node>/` is a view of it.

`intent wb register <moniker> --name "<display name>" --role <role>` names one node from its arguments. That is the form for every node that joins once boards are generated views, because there is no longer a hand-written header for anything to read. Running it again with the same values changes nothing; running it with different ones is refused rather than quietly taking the new values, so a node cannot be silently redefined. A deliberate change is `intent wb register <moniker> --name "<display name>" --role <role> --correct`, which changes only the name and role of a node that is already registered and keeps its board. Where the node still has a hand-authored `wip.md`, arguments that contradict its header's `name:` or `role:` are refused as well. **Registering is not a clean tree:** beside a hand-authored `wip.md` it writes no board or inbox view, so `intent wb migrate <node>` can still read the file, and it records one `wb.register` event under `intent/.canon/events/`. For a node with no hand-authored board it renders the board as well. An unchanged re-register writes nothing.

`intent wb register` with no arguments is the other form: it registers the roster from each node's own `wip.md` header, and refuses, naming every board, if any header lacks `node:`, `name:` or `role:`. Idempotent by moniker -- a second run adds nothing and changes nothing. It writes no board or inbox view, and a run that registers anything records one `wb.register` event for the roster. **It registers configuration and carries no content.** A node whose board is still hand-authored markdown is carried into the store by `intent wb migrate <node>`, which refuses and names every line the model cannot carry unless `--drop-uncarried` is passed; the dropped lines are kept byte for byte under `<node>/.history/pre-migration/`. Until a node is migrated, every verb that writes its board, and every message addressed to it, is refused.

**REGISTERING IS NOT A TIDY-UP.** Who the participants of a project are is a thing a human declares: every board, every item and every message afterwards hangs off the rows it writes, so an agent does not register a roster unasked.

Single-writer rule:

- `<node>/wip.md` -- written only by `<node>`.
- `<node>/inbox.<sender>.md` -- appended only by `<sender>`; read + cleansed only by `<node>` (the owner).

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

A hold is work this node has stopped, deliberately, waiting on something. **The item is not the content; the CONDITION is.** _Holding 0162 until the shared daemon is free_ is a hold. _Holding 0162_ is an item that has left DOING and entered nothing, and it is indistinguishable at every later reading from work that was quietly dropped.

Why the condition rather than the lifter: naming who or what you are waiting on records a dependency, and a dependency can be discharged by someone who never reads your board. Naming the CONDITION records what has to become true, which is checkable by you at every pickup without asking anyone. A hold whose condition cannot be written down is not a hold; it is an abandonment, and it belongs in TODO with a reason or out of the board entirely.

**A hold with no condition is a silent exclusion**, which is the same defect as an instrument that narrows its population without saying so: the work leaves the count and nothing marks the departure. Two holds that look identical -- one blocked permanently, one blocked until this afternoon -- collapse into one shape the moment the condition is dropped, and the temporary one then reads as permanent forever.

Check every hold at pickup and move the released ones back into TODO. This section is NOT archived by a fold while its condition stands unmet.

Only the header block is required for protocol compliance; the body sections are the working content.

### The header block is NOT YAML

It looks like YAML frontmatter and it is not. It is a **line-oriented `key: value` block**, and it is read that way by the header guard, by the roster registration that reads a hand-authored board, and by every tool that ever printed one. The rules are the whole specification:

- **One line per key.** The value is everything after the first `: ` to the end of that line. There are no multi-line values, no block scalars, no nesting, no comments, and no continuation lines.
- **Quotes are a display delimiter, not syntax, and the delimiter is the DOUBLE quote.** A single pair of surrounding `"` is stripped for display; quotes INSIDE a value are literal and are **never escaped**. Write `focus: "the counted body is the SENT body"` exactly as it reads. Writing `\"` puts a backslash in your board. **Single quotes are not delimiters and are never stripped** -- `focus: 'plain text'` renders with its quotes visible, which is the intended outcome rather than a gap. Two delimiter forms would mean a value whose content legitimately opens and closes with `'` silently loses two characters, and the rendered view would differ from the file with nothing saying so; visible quotes are a wart the author fixes at the next fold. **The one format whose whole purpose is having almost no rules does not get a second quoting rule.**
- **`claims:` is a comma-separated list in square brackets**, read as text.

This is a deliberate ruling, not an accident, and it was made because the alternative loses. The block is hand-written by LLM nodes in prose-heavy fields, which is close to the worst case for a quoting-sensitive format: a `focus:` line quoting a phrase is the natural thing to write, and under YAML it is invalid. Measured on a live board, a sizeable share of headers were unparseable at a point in time, and a sweep of one node's recent revisions found invalid headers in more than one episode -- **all of which repaired themselves** at the next fold, before anyone noticed. A defect whose lifetime is shorter than the interval between observations leaves no corpse, so the real rate is higher than any point-in-time count.

Under YAML the correct board also rendered worse: the reader that printed a roster stripped the delimiters without unescaping, so a node that complied displayed `\"` mid-prose. The format the tooling implements, the format the nodes write, and the format that reads correctly are the same one; the word "YAML" was the only thing out of step, so the word is what changed.

**AND ON A GENERATED BOARD YOU ARE READING THIS BLOCK RATHER THAN WRITING IT.** The header is rendered from the node's row, so the quoting question stops arising the moment a board is a view: the renderer emits the one form described above and a value carrying a quote carries it literally, because nothing is parsing it back. The rules stay written down because the guards still refuse a bad block in a hand-authored board, and because a reader who does not know the format reads a rendered board wrongly too.

#### Writing valid YAML here is the failure mode with no natural control

The measurement above is about a node writing **invalid** YAML, and that direction has a built-in corrective: the next node to read the board sees something broken and repairs it. **The opposite direction does not, and cannot.** A node that knows YAML, meeting a `"` inside a double-quoted value, escapes it -- which is correct YAML, produced by care, and looks completely fine. Nothing about it reads as wrong. The only symptom is a reader stripping the delimiters **without unescaping**, deliberately, so the backslash or the doubled apostrophe renders mid-prose at a moment nobody is looking.

**So the escape forms are refused at commit time** by `lib/templates/hooks/whiteboard-header-guard.sh`, a separate guard from the clock guard below -- one concern, one home, because a guard's name must not come to cover checks it does not describe. It reads only header blocks of live boards (never `.history/`, whose archives replay old headers verbatim), only lines the commit ADDS (inherited breakage must never wedge a board), and **never prose** -- nodes report this class to each other by quoting it, and scanning prose would make reporting the defect an offence. Like the clock guard it never auto-corrects: it prints the repaired line so the fix is a copy-paste.

The fix is never a better escape. It is to stop treating the block as YAML.

## inbox.<sender>.md shape

One inbox per ordered (sender -> recipient) pair: `<recipient>/inbox.<sender>.md` holds the messages `<sender>` has sent `<recipient>`. The sender is the sole writer (append-only); the recipient is the sole reader and owns its lifecycle (read, action, then `clear`, which marks the entries handled).

An inbox exists because the pair of nodes exists: once both are registered, the view renders in both directions whether or not a message has been sent. A fresh one is its header line plus the empty sentinel:

```
# inbox: <sender> -> <recipient>

_(empty)_
```

The `# inbox: <sender> -> <recipient>` header restates the single-writer routing the path already encodes, so the file is self-describing when read alone. `_(empty)_` is the sentinel for a pair that has never exchanged a message, so an inbox is never an ambiguous zero-byte file. `clear` removes nothing: a handled entry stays in the view with `(handled)` at the end of its heading, and leaves only the live count, which is what `pickup` and `show` list by default and what the inbox bound reads.

### Message-entry format

Each entry appended by `ask` / `announce`:

```
## (YYYY-MM-DD HH:MMZ) [claimed <stamp>] [Re: <prior-anchor>] [FYI only -- no response needed.] [(handled)] [(edited)]

<text>
```

Required fields: the `## (YYYY-MM-DD HH:MMZ)` timestamp heading (minute granularity -- it doubles as the anchor a reply threads against) and the `<text>` body. Recommended / optional: `Re: <prior-anchor>` (present only when threading a reply to a prior entry's timestamp) and `FYI only -- no response needed.` (present only when no reply is expected; absent means the sender expects a reply). A reply is a new entry in the opposite-direction inbox (`<original-sender>/inbox.<you>.md`), carrying `Re:` the entry it answers. The renderer adds three markers nobody writes: `claimed <stamp>` on an entry `wb migrate` carried from a hand-authored inbox, which is the stamp the markdown claimed, verbatim and never parsed, `(handled)` on an entry the recipient has cleared, and `(edited)` on an entry its sender changed with `wb edit message`. Every `announce` is sent as FYI.

**THE SEPARATOR BETWEEN THOSE FIELDS IS NOT SIGNIFICANT -- one or more spaces, both legal.** This spec said three spaces until 2026-09-02, and **a corpus read found that NO heading carrying a `Re:` or `FYI` field had kept the documented spacing**: the pre-commit gate refuses unformatted markdown and the formatter collapses runs of spaces, so every node wrote the documented form and every one was rewritten on the way in. **A format nobody can write is not a format.** Nothing depends on the separator -- `whiteboard-clock-guard.sh` keys on the STAMP and mentions `Re:` only in prose, and `wb migrate`, the one tool that reads these fields, finds them by their `Re: ` and `FYI only` tokens -- so the spec moved rather than the files. **The existing headings are deliberately NOT rewritten**: a bulk byte-change across append-only surfaces to satisfy a cosmetic field nothing reads is the exact harm the `.prettierignore` exemption exists to prevent.

### Every timestamp is READ FROM A CLOCK, never written from memory

This applies to every `## (...)` entry heading, every `heartbeat_at`, and every date you put in a `## Decisions` line. **Run this command and copy its output. Do not retype it, do not adjust it, do not infer it from context, and do not carry one forward from earlier in the session:**

```
date -u +'%Y-%m-%d %H:%MZ'
```

**A timestamp you did not read off a clock is fabricated data, not an approximation.** An LLM node has no clock and no felt duration -- there is nothing to be approximately right about, so a plausible-looking value is invented whole. This is not a style rule about zone suffixes; it is the difference between a record and a guess that reads exactly like one.

Two failures, both observed, both silent:

- **Fabrication.** A node stamped a reply BEFORE the message it was replying to, and another stamped a heartbeat well ahead of true UTC -- matching neither `date` nor `date -u` on the machine, so it came from no clock at all. Neither was noticed until a third node compared boards against `date -u`.
- **Wrong clock (Lamplight, 2026-07-24).** Heartbeats correctly in UTC, entry headings in local BST an hour ahead, so a correctly-stamped entry sorted BELOW a wrongly-stamped one. `date` and `date -u` differ by two characters and by the local offset.

Both destroy the same thing: the board's only cross-node ordering. "Who saw what, and in what order" is the question the inboxes exist to answer, and it stops being answerable the moment one stamp is invented -- **and it fails silently, because a fabricated timestamp is indistinguishable from a real one by inspection.** Use commits when you need ordering you can prove.

Corollaries:

- **Trailing `Z` is mandatory.** An unmarked heading means the writer used the wrong command; assume local and treat its ordering as unreliable.
- **Never rewrite a peer's stamp** -- it is their file. Flag it to them.
- **Never repair your own fabricated stamp by inventing a better one.** You cannot recover a time you never read. Annotate it as unverifiable and move on; a corrected-looking fake is worse than an admitted one.
- **`git log` prints LOCAL time.** It is the usual source of the +1h error: reading a time off it and appending a `Z` produces a stamp that is wrong by exactly the local offset and looks perfect.
- **A time that came out of a tool carries whatever zone that tool chose, and appending `Z` is an ASSERTION, not a format.** `git log` is the usual case and not the only one: `stat -f '%Sm'` prints local; `ls -la` prints local; `git log --date=format:` prints the commit's OWN recorded zone and IGNORES `TZ`, so `TZ=UTC git log --date=format:'%H:%MZ'` returns local and looks like it worked -- `--date=format-local:` is the form that honours `TZ`. The rule cannot enumerate every tool, so the general form is the keeper: `date -u`, or `date -u -r <epoch>`, or say nothing. Measured 2026-08-26 on the Intent board: two nodes each rendered a real read an hour ahead by appending `Z` to a local listing, and one did it INSIDE the audit it was running to catch the first instance.
- **A stamp typed from the last one you read is fabricated too.** The offset error is +1h exactly; this one drifts by however long the turn felt. Same cure: a clock value goes into a message or a board only when the command that produced it is in front of you in this turn, verbatim, or it does not go in at all. Cross-session messages carry the same `## (...)` ordering claims as the boards and sit under none of the whiteboard guards, so the discipline is the only check on that channel.

### On a generated board the clock is the SERVICE's, and that is the point

**EVERY STAMP `intent wb` WRITES IS READ FROM THE CLOCK BY THE SERVICE AT THE MOMENT OF THE WRITE.** No verb takes a timestamp, none of them has a flag for one, and there is nothing to validate -- the fabricated-stamp class is closed by construction rather than by detection, which is what the rule above spent two years asking for. A heartbeat, a message's stamp and an item's `archived_at` all come from the store.

So the discipline below is NOT retired, and it is narrower than it was: it now governs everything you write by hand that a verb does not stamp for you -- a hand-authored board before the cutover, prose in a message body, a date you put in a commit or a report. **A clock value goes in only when the `date -u` read is in front of you in this turn, verbatim.**

### This is enforced, not merely written down

`lib/templates/hooks/whiteboard-clock-guard.sh` runs from the pre-commit gate and **refuses the commit** -- the bad stamp never lands. It is opt-in by the presence of `intent/whiteboard/`, so nothing changes for a project without a board. Built and measured in Lamplight, brought upstream because Intent ships this protocol and every consumer inherits the hole otherwise.

It is one of the whiteboard guards -- with the header guard and the `.history/` append-only guard -- and they are deliberately separate files: this one's name and contract are TIMESTAMPS, the header guard's is the header block's format. The roster lives in the install's `lib/templates/hooks/pre-commit-guards.sh`, which **runs every applicable guard before deciding**, so a board carrying a bad stamp AND an escaped value is one editing session rather than two commit attempts. A project holds only a shim (`.git/hooks/pre-commit.intent`) that finds the install through `~/.local/share/intent/home`; the gate, the roster and the guard bodies are all read live from there, so a new guard reaches every consumer on its next commit without anyone touching `.git/hooks/`.

The checks below each close a hole the others cannot see:

| check | what it catches                      | how                                                                   |
| ----- | ------------------------------------ | --------------------------------------------------------------------- |
| **A** | a stamp in the future                | a stamp's minute cannot postdate the commit adding it; zero tolerance |
| **B** | a missing trailing `Z`               | syntactic, exact, no clock, no tolerance                              |
| **C** | an append-only inbox going backwards | compares two board stamps to each other; needs no clock at all        |

Why all of them. **A alone does not catch the local-clock error**: an unmarked `## (2026-08-14 14:19)` is read as UTC, so it only trips A _while still in the future_ -- once a commit lags past the local offset the same bad stamp sails through, and lag is normal (measured: most stamps commit within the hour, with a tail of hours). **A and B both compare a stamp to a clock**, so a fabricated stamp landing in the _past_ passes both in silence -- which is the failure this rule names first. C is the two-sided test: a real `date -u` read can never break it, because time does not run backwards.

Two things the guard deliberately does not do. It **never auto-corrects** -- a guard that silently fixes the stamp hides the class from the node that needs to learn its clock was wrong; it prints the right value so the fix is a copy-paste. And **check C never blocks on pre-existing breakage**, only on stamps the current commit adds, because a guard that must be bypassed to work is a guard nobody keeps.

**It does not close the class, and you should not read a green as proof that it has.** A fabricated stamp that carries a `Z`, lands in the past, and still increases monotonically passes all three checks. Smaller target, not an empty one -- which is the whole reason the rule above is stated as a rule and not as "the hook will catch it".

## Node-identity discovery

Every verb that writes as a node takes `--node <moniker>`, and which node you are is the one thing the tool cannot work out for you.

1. If the invocation carries a moniker, use it.
2. Otherwise infer from cues: the session's own name, the working directory, the user's framing, which node's board names this session.
3. If still ambiguous, ask the user before writing anything. **A write under the wrong moniker is a write on somebody else's board**, and the single-writer invariant is enforced against the moniker you passed rather than against who you are.

The moniker is durable; subsequent sessions of that node inherit it.

## Each verb, and the judgement it cannot carry

### `pickup`

`intent wb pickup --node <you>` -- it prints your board, `hv`'s standing content, and every peer's header state, and moves your heartbeat once, in that order (the touch precedes the read, because your own board is part of what comes back).

**YOUR MESSAGES COME BACK AS THE LIVE SET.** The ones still waiting are listed and the handled ones are one line, `handled: N message(s) -- --all lists them`, because a session start asks what still needs an answer, and an inbox that only grows would otherwise answer it with the archive. `--all` lists every message, handled included: reach for it when the archive is the question, never to start a session.

**`hv`'S DIRECTIVES, WATCH-OUTS AND DECISIONS COME BACK IN FULL**, because they bind every node and a read that showed your own watch-outs and nobody else's looked complete. Other peers come back as HEADERS, each with a `not shown:` line counting its live items by kind. `intent wb show <peer>` is the door for one whole board, and **every board is readable from every workstream** -- the single-writer invariant is about WRITES and never made a board private.

**What the verb cannot do is read your inboxes FOR you.** Surface what came in to the user; an entry nobody mentions is an entry nobody handles.

### `ask <node> <body>`

`intent wb ask <node> "<body>" --node <you>`, with `--re <anchor>` when you are threading a reply and `--fyi` when no reply is expected. The path from sender to recipient is the message's own identity, so there is no `to:` or `from:` line to write.

A reply is a new message in the other direction, carrying `--re` the entry it answers.

**A SOCKET MESSAGE IS NOT A DELIVERY AND NEITHER IS THIS ONE.** The verb returns when the message is stored. It is handled when a reader has acted on it, which is a different event, and the gap between them is where three rulings went unregistered by a busy peer in one afternoon. Durable things go in the inbox; things that must be ACTED ON get said twice.

### `announce <body>`

`intent wb announce "<body>" --node <you>` -- one message to every registered node but you.

Use it for 1-to-all signals: a shared platform layer you are about to touch, a protocol change, a broadcast decision. It is not a second way to hold a conversation; a thing that needs an answer is an `ask`.

### `add <kind> <text>`

`intent wb add <doing|todo|watchout|hold|directive> "<text>" --node <you>`. The service assigns the `seq`.

**A HOLD CARRIES THE CONDITION THAT RELEASES IT, AND THE CONDITION IS THE CONTENT.** _Holding 0162 until the shared daemon is free_ is a hold; _holding 0162_ is an item that left DOING and entered nothing, indistinguishable at every later reading from work that was quietly dropped. The verb cannot check this. Nothing can -- it is judgement, and it is yours.

`decision` is refused here by name and redirected to `wb decide`, so there is one door per kind.

**`directive` IS `hv`'s, AND NO OTHER NODE WRITES ONE.** A standing directive is an instruction every node honours, so it lives on the hypervisor's board, and `wb add directive` from any other node is refused by name. A call a node made itself is a `decision`, which `wb decide` writes.

### `decide <text>`

`intent wb decide "<text>" --node <you>` -- a decision is an item on your own board rather than a message, because a decision is broadcast by sitting somewhere its peers read at pickup. Giving it recipients would turn one durable statement into four copies that can diverge.

### `claim <id>` / `unclaim <id>`

`intent wb claim <id> --node <you>`, and `unclaim` to drop it. It takes a thread as `ST0000`, a work package as `ST0000/01`, or an issue as `ISSUE:0000`.

**Before you claim, look at who else does.** `intent wb status --json` carries every node's claims, and `intent wb show <node>` prints one node's; if an active peer already holds it, stop and surface the overlap for the hypervisor to arbitrate rather than claiming alongside them.

### `clear <sender>`

`intent wb clear <sender> --node <you>` -- it marks every live message that sender sent you handled.

**AN ENTRY MAY BE MARKED HANDLED ONLY IF IT WAS ANSWERED, ACTIONED, OR RE-STATED LIVE ON YOUR BOARD. CLEARING IS NOT ONE OF THE THREE, AND READING IT IS NOT EITHER.** The op moves an entry to where nobody looks, so a cleared-but-unactioned entry leaves a board that is affirmatively WRONG rather than merely stale. Measured in Laksa 2026-08-31 (laksa-cc, adopted fleet-wide, routed here because the enforcement home is this file): a node routed two red guards to a peer, correctly attributed; the peer's next fold cleared and archived the entry unactioned and wrote _nothing in flight_, and the tree stayed red until a SECOND report caught it a session later. **No guard is possible, because actioned-ness is judgement** -- the precondition is prose and the discipline is yours.

### `archive <kind> <seq>`

`intent wb archive <doing|todo|decision|watchout|hold|directive> <seq> --node <you>` -- one item of your own leaves the live count.

**ARCHIVED IS A STATE AND NEVER A DELETION.** The row keeps its number and its text and stays readable; it just stops counting, which is how a board that has started refusing a write begins accepting again. It reports what MOVED, so archiving something already archived says so rather than lying.

**It takes a KIND as well as a `seq` because `seq` alone is ambiguous**: items are numbered within (node, kind), so you can hold a `doing` 1 and a `decision` 1 at once.

**AND THE STATE CHANGE IS THE SCHEDULE.** There is no fold to remember, no sweep, no timer: an item leaves the live count the moment you state that it is finished with, because handled and done are facts only you can state. What the verb cannot judge is whether it IS finished with -- see the fold rules below, and in particular the one about rulings.

### `edit <kind> <id> <text>`

`intent wb edit <doing|todo|decision|watchout|hold|directive> <id> "<text>" --node <you>` -- one item of your own, live or archived, gets new text; `<id>` is the item's number, as `wb show` prints it.

`intent wb edit message "<anchor>" "<text>" --to <recipient> --node <you>` -- one message you sent, handled or live, gets a new body. `<anchor>` is the first stamp in its heading in the recipient's inbox view, the value `wb ask --re` takes, and `<anchor>#<n>` picks one of several sent in one minute; a minute holding several is refused with each of them listed. `--to` is required for a message and refused for an item. **AN ANNOUNCE IS ONE MESSAGE TO EVERY PEER**, so editing any copy edits every copy, and the answer names each recipient; where one copy no longer reads what the announce said, only the addressed copy changes.

**WHERE THE OLD TEXT GOES IS THE POINT, AND THE ANSWER SAYS WHICH.** If no commit holds the event that carries it, that event is amended in place. If a commit holds it, a `wb.edit` event records the change and git history keeps the old text: rewriting history is the repository's decision, never this verb's. Either way the answer names, each list on a line of its own, every file under `intent/` that HEAD already carries the old text in, every staged file the next commit would carry it in, and every unstaged or untracked file the next commit could carry it in, all read rather than assumed. Committed means present at HEAD -- a file you staged is not committed, and the verb names any board or event file staged before the edit, because a plain `git commit` carries the index, not the disk. `git add` restages the corrected file.

**THE ALL-CLEAR IS SAID ONLY WHEN BOTH SEARCHES RAN AND FOUND NOTHING, AND IT SAYS WHAT WAS SEARCHED**: _no file under intent/ holds the old text, at HEAD or in the next commit_. A peer's committed item quoting the text, a copy `wb migrate` kept under `.history/pre-migration/`, a draft the edit could not match to the item, and another item that says the same thing all leave the text where a commit carries it, so the answer names each of them rather than assuming them away. The lines name what holds the text, not whose it is. When the new text contains the old text, every file holding the new text holds the old text as well, and the answer says so on a line of its own rather than shortening the lists. Outside `intent/`, and in commits before HEAD, nothing is searched.

**THE ADDRESS IS YOUR OWN BOARD, OR A MESSAGE YOU SENT.** A peer's item is the peer's to edit, exactly as it is the peer's to archive, and a message is its sender's to edit.

**AN EDITED ITEM OR MESSAGE SAYS SO, AND NEVER WHAT IT SAID.** It renders `(edited)` at the end of an item's first line, in the views, in `wb show` and in `wb pickup`; on a message, after `(handled)` in its inbox heading and after the route in `wb show` and `wb pickup`. `board.json` carries the stamp as `edited_at`, so a rebuild keeps the mark. An edit of an uncommitted draft leaves no trace in the event log, deliberately, so this mark is how a peer who acted on the text can tell that it changed.

### `touch`

`intent wb touch --node <you>` -- your heartbeat, and nothing else.

**No caller supplies the time and there is no flag for one.** A heartbeat says _this node was alive at this moment_; a caller-supplied value would be the fabricated stamp with the model's blessing. The service reads the clock at the write, which closes that class by construction rather than by detection.

### `release`

`intent wb release --node <you>` -- it sets you paused AND stamps, because the last thing a paused node says is WHEN it stopped. A status change alone would leave a cleanly-released node looking exactly like one that died mid-turn, and telling those apart is why a board carries a heartbeat at all.

**THIS IS A SESSION-END OP AND NOTHING ELSE.** Not for a localfold, not for a compact, not for a context reset -- see invariant 6. If the session continues, the status continues.

### `status` and `show`

`intent wb status` for one line per node -- role, name, status, heartbeat, its live item and message counts, and the focus line untruncated, because the focus is the field a person is actually reading for. Claims are in `--json` and in `show`. `intent wb show <node>` for one node's board: its header, its live items, its live messages, and a count of the archived items and of the handled messages; `--all` lists the handled messages as well. Neither writes.

### Folding: localfold and globalfold

The human may say "localfold" or "globalfold" (terms from Lamplight; defined in `/in-finish`). **localfold** = tidy your OWN node before a compact -- archive what is finished with, clear what you have handled. **IT DOES NOT `release`.** A localfold before a compact is not a session ending, and invariant 6 says exactly that; `release` belongs to `/in-finish` when the session actually ends. This sentence read _then `release`_ until 2026-08-31 and contradicted invariant 6 and the red-flag table **in this same file**, four sections apart -- which is why nodes kept re-deriving the answer under compact pressure instead of reading it. **globalfold** = the project-wide snapshot (`intent/wip.md` / `restart.md` / `done.md`), typically the coordinating or validation node's job, not a per-node op.

**AN UNEXECUTED RULING IS LIVE STATE, NOT HISTORY. A fold archives the NARRATIVE of a ruling and never the ruling itself while it is unexecuted.** Execution status is the discriminator; the date is evidence of nothing. Verify execution against the ARTEFACT, never against the board that records it. Measured on this protocol 2026-08-30: a fold applied the rule _cut any mention of DONE work_ to a whole dated ruling record, which keyed on **dated** where the rule keys on **done** -- so the fold enforcing _doing and todo only_ is the thing that removed todo items. Not one word was lost, which is precisely the failure: a live directive reachable only by grepping an archive is discoverable by nobody. Buried directives were found days later, one of them shipping the option the human had explicitly DECLINED.

**A HOLD IS NOT ARCHIVED WHILE ITS CONDITION STANDS UNMET.** Check every hold when you pick up and move the released ones back into TODO -- which is `wb archive hold <seq>` and then `wb add todo`, because the condition being met is a third thing, neither DONE nor retirement.

**A FOLD NEVER ARCHIVES A DIRECTIVE.** `wb archive directive <seq> --node hv` exists because `hv` retires a directive once it is spent, and the verb cannot tell that act from a fold tidying the board, so the rule lives here rather than in the verb. A directive leaves `hv`'s board when `hv` says it is spent, never because a fold is making room.

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

1. **One writer per board.** Every verb writes only the acting node's own board, a message is recorded under the acting node as sender, and only the recipient clears its inbox. `wip.md` = the node; `inbox.<sender>.md` = the sender; the recipient owns its inbox lifecycle. **The acting node is whatever `--node` names, so the rule is enforced against the moniker you pass and not against who you are**: passing somebody else's moniker is the one way to break it, and nothing detects it.
2. **Live channel, not snapshot.** `intent/wip.md` is the post-session snapshot; `<node>/wip.md` is the live board.
3. **Claims by ST ID** (in the `wip.md` header block), never glob paths.
4. **Broadcast via `announce` -> peers' inboxes.** No shared file; a shared platform layer (eg `apps/lamplight/**`) is coordinated by announcing before you touch it.
5. **Heartbeat older than 7 days marks a claim reclaimable** -- reclaim requires explicit hypervisor acknowledgement.
6. **`/compact` does NOT end a session** -- status stays `active`; the next `pickup` touches the heartbeat.
7. **You archive your own items only**, and archived is a state rather than a directory: the row stays readable and stops counting. Existing `.history/YYYYMMDD/` trees are the record of the hand-authored era, append-only, never reloaded on pickup.
8. **The human is `hv`** in all protocol language, never by name.

## Why this exists

Concurrent sessions need a live coordination surface, and `wip.md` (the post-session snapshot) loses fidelity _during_ a session. Protocol 2.0 supplied that with shared files (`asks.md`, per-stream files, `lamplight.md`), but those had N writers each: contention on every edit, cleanse that required cross-stream coordination, and unbounded growth that chewed context. 3.0 fixes all three by giving every file exactly one writer -- a per-node board you alone write, and per-sender inboxes each written by one peer and cleansed by you. Coordination stays live; contention and cleanse-pain go away.

## Red Flags

| Rationalisation                                             | Reality                                                                                         |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| "One `inbox.md` per node is simpler than per-sender."       | One file, N writers -- back to 2.0 contention. Per-sender is what makes it single-writer.       |
| "I'll edit a peer's `wip.md` to correct it."                | Never. You write only your own node. Send an `ask` to its inbox.                                |
| "I'll keep a shared file for platform edits."               | That is the retired `lamplight.md`. Use `announce` -- broadcast to inboxes, no shared file.     |
| "/compact ended the session, so I'll set `status: paused`." | No. `/compact` is transparent. Status stays active; `/in-session` re-fires `pickup`.            |
| "This ruling is dated, so the fold archives it."            | Only if it was EXECUTED. Unexecuted, it is todo work, and `.history/` is where nobody looks.    |
| "I read the inbox entry, so `clear` is just hygiene."       | Reading is not handling. Answer it, action it, or re-state it live -- then clear.               |
| "I am holding this item." (no condition)                    | Then it is not a hold. Name what has to become true, or it reads as abandoned at every pickup.  |
| "I'll archive the whole board while I'm here."              | You archive only your own `<you>/` dir. Single-owner, collision-free -- that is the point.      |
| "The node said it's done, so it's done."                    | A "done" claim is the _trigger_ to verify, not the verdict. Read the as-built against the ask.  |
| "I know roughly what time it is."                           | You do not. You have no clock. Run `date -u`; a plausible stamp is fabricated, not approximate. |
| "I stamped one earlier this session, I'll reuse it."        | Time passed. Re-run `date -u` for every stamp, including the second one in the same turn.       |
| "I'll just edit the board file, it is right there."         | It is a rendered view. Your edit is skew doctor reports, and the next render drops it.          |
| "`wb clear` tidies the inbox before I fold."                | It states that you HANDLED them. Answer, action, or re-state live -- then clear.                |
| "The verb returned ok, so the peer has it."                 | Stored is not delivered. A thing that must be acted on gets said twice.                         |
| "I'll register the roster while I am in here."              | Who the participants are is a human's declaration. Registering is not a tidy-up.                |
