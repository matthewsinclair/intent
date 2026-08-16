---
description: "Multi-session coordination via intent/whiteboard/<node>/: per-node boards + single-writer inboxes, claim ST scopes, broadcast, heartbeat, release"
chains_to: []
---

# Whiteboard -- Multi-Session Coordination (Protocol 3.0)

Coordinator for multiple Claude Code sessions -- and the human -- running concurrently against one Intent project. Each participant is a **node** with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what makes the board contention-free and cleansable. The whiteboard is the _live_ channel; `intent/wip.md` is the post-session snapshot.

**Protocol 3.0** supersedes 2.0 (flat shared `asks.md` + per-stream files). v3.0 = per-node directories + a single-writer inbox model + the human as a first-class `hv` (hypervisor) node.

## Nodes

A node is a participant. The 2-letter moniker is the directory name, the routing key, and the handle. Nodes are **per-project configuration**: the project declares its roster (monikers, display names, roles) in its hand-authored `intent/whiteboard/README.md`. No roster is baked into this skill -- it discovers nodes by listing the immediate subdirectories of `intent/whiteboard/`.

A project that wants the human in the loop gives them a node, conventionally `hv` (the **hypervisor**): the human who adjudicates scope, sequences work, owns releases, and is where escalations land. The human is addressed as `hv` in all protocol language, never by name. The hypervisor node is human-driven -- it is read like any other node, but the human maintains it (or has it maintained on their behalf) rather than running `pickup` on a heartbeat.

### The hv (hypervisor) node

`hv` is structurally a node like any other -- a `<hv>/wip.md` peers read at pickup, inboxes peers append to -- with three differences that follow from being human-driven:

- **No session loop.** `hv` is not driven by `/in-session` / `pickup`, so its `session_id` is optional and conventionally `none`. Peers therefore never match it on the "different `session_id`" active-peer test; they read it for its directives and route escalations to `hv/inbox.<you>.md`.
- **Heartbeat is advisory.** A stale `hv` heartbeat does not mark anything reclaimable -- the human is always authoritative -- so the 7-day reclaim rule does not apply to `hv`.
- **Standing directives.** Beyond the canonical `wip.md` body, `hv` may carry a `## Standing directives` section: durable instructions every node honours (sequencing, scope rulings, release policy). Peers read it at pickup the way they read `## Decisions`.

## When to invoke

- `pickup` -- chained from `/in-session`; read own board + own inboxes + peer state, touch heartbeat.
- `ask <node> <text>` -- send a point-to-point message to another node.
- `announce <text>` -- broadcast one line to every peer (eg before touching a shared platform layer).
- `decide <text>` -- record a cross-node decision on your own board.
- `claim <STxxxx>` / `unclaim <STxxxx>` -- add/remove an ST from your board's claims.
- `clear <sender>` -- archive handled entries out of one of your own inboxes.
- `archive` -- roll your own DONE board content + handled inbox entries into your own history.
- `touch` -- refresh your heartbeat.
- `release` -- chained from `/in-finish`; set your status paused.
- `status` -- read-only one-line-per-node summary.

If invoked with no subcommand, default to `status`.

## File layout

```
intent/whiteboard/
  README.md                 # protocol reference + the project's node roster
  <node>/
    wip.md                  # the node's live board: header block + DOING + TODO + watch-outs + decisions
    inbox.<sender>.md       # one per OTHER node: messages FROM that sender (single-writer)
    .history/
      .gitkeep              # tracks the otherwise-empty archive dir (git ignores empty dirs)
      YYYYMMDD/             # the node's archived DONE work + handled inbox entries
```

Scaffolding a node is the deterministic job of `intent claude ws new <node>` (the provisioner -- ST0047), not a hand ritual: it creates `<node>/`, `<node>/.history/.gitkeep` (git does not track an empty directory), the node's `wip.md`, and an `_(empty)_` inbox in **both** directions with every existing peer (`<node>/inbox.<peer>.md` + `<peer>/inbox.<node>.md`). The `intent claude ws` family (`new` / `list` / `archive` / `hygiene`) plus `intent claude start <node>` (launch a session bound to a node) own this mechanical lifecycle; this skill owns the judgement ops below. Both honour the one on-disk format described here.

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
## DOING        -- in-flight work (archived into .history/ when done)
## TODO         -- queued / next
## Watch-outs   -- durable cautions peers should know (standing; not archived)
## Decisions    -- cross-node decisions, broadcast by being read at pickup
```

Only the header block is required for protocol compliance; the body sections are the working content.

### The header block is NOT YAML

It looks like YAML frontmatter and it is not. It is a **line-oriented `key: value` block**, and every reader in the tool -- `fm_get`, `ws list`, `ws hygiene` -- reads it that way. The rules are the whole specification:

- **One line per key.** The value is everything after the first `: ` to the end of that line. There are no multi-line values, no block scalars, no nesting, no comments, and no continuation lines.
- **Quotes are a display delimiter, not syntax, and the delimiter is the DOUBLE quote.** A single pair of surrounding `"` is stripped for display; quotes INSIDE a value are literal and are **never escaped**. Write `focus: "the counted body is the SENT body"` exactly as it reads. Writing `\"` puts a backslash in your board. **Single quotes are not delimiters and are never stripped** -- `focus: 'plain text'` renders with its quotes visible, which is the intended outcome rather than a gap. Two delimiter forms would mean a value whose content legitimately opens and closes with `'` silently loses two characters, and the rendered view would differ from the file with nothing saying so; visible quotes are a wart the author fixes at the next fold. **The one format whose whole purpose is having almost no rules does not get a second quoting rule.**
- **`claims:` is a comma-separated list in square brackets**, read as text.

This is a deliberate ruling, not an accident, and it was made because the alternative loses. The block is hand-written by LLM nodes in prose-heavy fields, which is close to the worst case for a quoting-sensitive format: a `focus:` line quoting a phrase is the natural thing to write, and under YAML it is invalid. Measured on a five-node board, two of five were unparseable at a point in time, and a sweep of one node's last 25 revisions found four invalid in two separate episodes -- **all of which repaired themselves** at the next fold, before anyone noticed. A defect whose lifetime is shorter than the interval between observations leaves no corpse, so the real rate is higher than any point-in-time count.

Under YAML the correct board also renders worse: `ws list` strips the delimiters without unescaping, so a node that complied would display `\"` mid-prose. The format the tooling actually implements, the format the nodes actually write, and the format that reads correctly are the same one; the word "YAML" was the only thing out of step, so the word is what changed.

`intent claude ws hygiene` enforces exactly this rule: every line in the block is a single-line `key: value`, and the required keys are readable. It does NOT check YAML validity, because validity is not the contract.

#### Writing valid YAML here is the failure mode with no natural control

The measurement above is about a node writing **invalid** YAML, and that direction has a built-in corrective: the next node to read the board sees something broken and repairs it. **The opposite direction does not, and cannot.** A node that knows YAML, meeting a `"` inside a double-quoted value, escapes it -- which is correct YAML, produced by care, and looks completely fine. Nothing about it reads as wrong. The only symptom is `fm_get` stripping the delimiters **without unescaping** (deliberately -- see `intent_claude_cwi`), so `ws list` renders the backslash or the doubled apostrophe mid-prose, at a moment nobody is looking.

**So the escape forms are refused at commit time** by `lib/templates/hooks/whiteboard-header-guard.sh`, a separate guard from the clock guard below -- one concern, one home, because a guard's name must not come to cover checks it does not describe. It reads only header blocks of live boards (never `.history/`, whose archives replay old headers verbatim), only lines the commit ADDS (inherited breakage must never wedge a board), and **never prose** -- nodes report this class to each other by quoting it, and scanning prose would make reporting the defect an offence. Like the clock guard it never auto-corrects: it prints the repaired line so the fix is a copy-paste.

The fix is never a better escape. It is to stop treating the block as YAML.

## inbox.<sender>.md shape

One inbox per ordered (sender -> recipient) pair: `<recipient>/inbox.<sender>.md` holds the messages `<sender>` has sent `<recipient>`. The sender is the sole writer (append-only); the recipient is the sole reader and owns its lifecycle (read, action, `clear` into history).

Inboxes are pre-seeded in both directions when a node is scaffolded (`ws new` writes the header + `_(empty)_` sentinel for every existing peer pair). `ask` / `announce` also create an absent `<recipient>/inbox.<you>.md` on demand before appending -- so a hand-added node, or a board predating the provisioner, self-heals. Either way, a fresh inbox is its header line plus the empty sentinel:

```
# inbox: <sender> -> <recipient>

_(empty)_
```

The `# inbox: <sender> -> <recipient>` header restates the single-writer routing the path already encodes, so the file is self-describing when read alone. `_(empty)_` is the "no live entries" sentinel: `clear` and `archive` leave the header + `_(empty)_` behind when they remove the last handled entry, so an inbox is never an ambiguous zero-byte file.

### Message-entry format

Each entry appended by `ask` / `announce`:

```
## (YYYY-MM-DD HH:MM)   [Re: <prior-anchor>]   [FYI only -- no response needed.]

<text>
```

Required fields: the `## (YYYY-MM-DD HH:MMZ)` timestamp heading (minute granularity -- it doubles as the anchor a reply threads against) and the `<text>` body. Recommended / optional: `Re: <prior-anchor>` (present only when threading a reply to a prior entry's timestamp) and `FYI only -- no response needed.` (present only when no reply is expected; absent means the sender expects a reply). A reply is a new entry in the opposite-direction inbox (`<original-sender>/inbox.<you>.md`), carrying `Re:` the entry it answers.

### Every timestamp is READ FROM A CLOCK, never written from memory

This applies to every `## (...)` entry heading, every `heartbeat_at`, and every date you put in a `## Decisions` line. **Run this command and copy its output. Do not retype it, do not adjust it, do not infer it from context, and do not carry one forward from earlier in the session:**

```
date -u +'%Y-%m-%d %H:%MZ'
```

**A timestamp you did not read off a clock is fabricated data, not an approximation.** An LLM node has no clock and no felt duration -- there is nothing to be approximately right about, so a plausible-looking value is invented whole. This is not a style rule about zone suffixes; it is the difference between a record and a guess that reads exactly like one.

Two failures, both observed, both silent:

- **Fabrication.** A node stamped a reply 25 minutes BEFORE the message it was replying to, and another stamped a heartbeat ~99 minutes ahead of true UTC -- matching neither `date` nor `date -u` on the machine, so it came from no clock at all. Neither was noticed until a third node compared boards against `date -u`.
- **Wrong clock (Lamplight, 2026-07-24).** Heartbeats correctly in UTC, entry headings in local BST an hour ahead, so a correctly-stamped entry sorted BELOW a wrongly-stamped one. `date` and `date -u` differ by two characters and by the local offset.

Both destroy the same thing: the board's only cross-node ordering. "Who saw what, and in what order" is the question the inboxes exist to answer, and it stops being answerable the moment one stamp is invented -- **and it fails silently, because a fabricated timestamp is indistinguishable from a real one by inspection.** Use commits when you need ordering you can prove.

Corollaries:

- **Trailing `Z` is mandatory.** An unmarked heading means the writer used the wrong command; assume local and treat its ordering as unreliable.
- **Never rewrite a peer's stamp** -- it is their file. Flag it to them.
- **Never repair your own fabricated stamp by inventing a better one.** You cannot recover a time you never read. Annotate it as unverifiable and move on; a corrected-looking fake is worse than an admitted one.
- **`git log` prints LOCAL time.** It is the usual source of the +1h error: reading a time off it and appending a `Z` produces a stamp that is wrong by exactly the local offset and looks perfect.

### This is enforced, not merely written down

`lib/templates/hooks/whiteboard-clock-guard.sh` runs from the pre-commit gate and **refuses the commit** -- the bad stamp never lands. It is opt-in by the presence of `intent/whiteboard/`, so nothing changes for a project without a board. Built and measured in Lamplight, brought upstream because Intent ships this protocol and every consumer inherits the hole otherwise.

It is one of two whiteboard guards, and they are deliberately separate files: this one's name and contract are TIMESTAMPS, the header guard's is the header block's format. The shipped `pre-commit.sh` declares both in one roster and **runs every one of them before deciding**, so a board carrying a bad stamp AND an escaped value is one editing session rather than two commit attempts. Only that hook is copied into a project; the guard bodies are read live out of `INTENT_HOME`, which is why a new guard reaches every consumer on their next `intent upgrade` without anyone touching `.git/hooks/`.

Three checks, because each closes a hole the others cannot see:

| check | what it catches                      | how                                                                 |
| ----- | ------------------------------------ | ------------------------------------------------------------------- |
| **A** | a stamp in the future                | a stamp cannot postdate the commit adding it; 120s jitter tolerance |
| **B** | a missing trailing `Z`               | syntactic, exact, no clock, no tolerance                            |
| **C** | an append-only inbox going backwards | compares two board stamps to each other; needs no clock at all      |

Why all three. **A alone does not catch the local-clock error**: an unmarked `## (2026-08-14 14:19)` is read as UTC, so it only trips A _while still in the future_ -- once a commit lags past the local offset the same bad stamp sails through, and lag is normal (measured: 93% of stamps commit within the hour, tail to nine hours). **A and B both compare a stamp to a clock**, so a fabricated stamp landing in the _past_ passes both in silence -- which is the failure this rule names first. C is the two-sided test: a real `date -u` read can never break it, because time does not run backwards.

Two things the guard deliberately does not do. It **never auto-corrects** -- a guard that silently fixes the stamp hides the class from the node that needs to learn its clock was wrong; it prints the right value so the fix is a copy-paste. And **check C never blocks on pre-existing breakage**, only on stamps the current commit adds, because a guard that must be bypassed to work is a guard nobody keeps.

**It does not close the class, and you should not read a green as proof that it has.** A fabricated stamp that carries a `Z`, lands in the past, and still increases monotonically passes all three checks. Smaller target, not an empty one -- which is the whole reason the rule above is stated as a rule and not as "the hook will catch it".

## Node-identity discovery

On `pickup`, determine which node this session is:

1. If args carry a moniker (`/in-whiteboard pickup vc`), use it.
2. Otherwise infer from cues: working directory, branch, recent commits, the user's framing, which node's `wip.md` carries this session's `session_id`.
3. If still ambiguous, ask the user before writing anything.

The moniker is durable; subsequent sessions of that node inherit it via the existing `<node>/` directory.

## Procedure per subcommand

### pickup

1. List `intent/whiteboard/*/` to enumerate nodes. Determine your node (see discovery).
2. Read your `<you>/wip.md` (resume state) and all four `<you>/inbox.*.md` (incoming). Surface any non-empty inbox entries to the user.
3. Read each peer's `<peer>/wip.md` header block (line-oriented `key: value`, NOT YAML -- see wip.md shape). For each peer with `status: active` AND `heartbeat_at` within 7 days AND a different `session_id`: surface "node X active (heartbeat <relative>, focus: <focus>)". Active but older than 7 days: "node X appears stale".
4. Update your `<you>/wip.md` header block: `session_id` (this session, or `unknown`), `heartbeat_at` (now), `status: active`. Keep `claims` + body intact. One line per key; do not escape quotes inside a value.
5. Report a one-line summary of peer state + your inbound messages.

### ask <node> <text>

1. Your `inbox.<you>.md` in `<node>/` usually already exists (`ws new` pre-seeds it); if it is absent (a hand-added node), create it with its `# inbox: <you> -> <node>` header + `_(empty)_` sentinel (see inbox shape). Append a message entry (see Message-entry format) -- the path encodes sender -> recipient, so the 2.0 `to:`/`from:` line is implicit:

   ```
   ## (YYYY-MM-DD HH:MM)   [Re: <prior-anchor>]   [FYI only -- no response needed.]

   <text>
   ```

   If the inbox already carries only `_(empty)_`, replace that sentinel with the first entry.

2. Touch your heartbeat.

A reply goes to `<sender>/inbox.<you>.md` (the inbox flips direction).

### announce <text>

1. Append the same one-line entry to EVERY peer's `<peer>/inbox.<you>.md` (all nodes except yourself).
2. Touch your heartbeat.

Use for 1-to-all signals -- eg "touching `apps/lamplight/**` for ST-X" (a shared platform-layer edit; the retired `lamplight.md` job), or a protocol/decision broadcast.

### decide <text>

1. Append `- (YYYY-MM-DD) <text>` to your `<you>/wip.md` `## Decisions` section (peers read it at pickup).
2. Touch heartbeat.

### claim <STxxxx> / unclaim <STxxxx>

1. Add/remove `STxxxx` in your `wip.md` `claims`.
2. On claim, scan peers' `wip.md` `claims`: if an _active_ peer already claims it, stop and surface the overlap for the hypervisor to arbitrate.
3. Touch heartbeat.

### clear <sender>

1. In your `<you>/inbox.<sender>.md`, move the handled entries verbatim into `<you>/.history/<YYYYMMDD>/inbox.<sender>.md`, and remove them from the live inbox (leaving the header + `_(empty)_` if none remain).
2. You own your inbox -- no peer files touched. Touch heartbeat.

### archive

Roll your OWN node's DONE content out of the live files into your own history, daily-or-more, so the live files stay lean (they are read on every pickup).

1. Ensure `<you>/.history/<YYYYMMDD>/` exists (today, or the content's own date).
2. From `<you>/wip.md`: move DONE `## DOING` items + superseded blocks into `<you>/.history/<YYYYMMDD>/wip.md`. KEEP frontmatter, live DOING/TODO, `## Watch-outs`, and still-relevant `## Decisions`.
3. From each `<you>/inbox.<sender>.md`: move handled entries into history (same as `clear`).
4. `prettier --write` the touched files if the project formats markdown.
5. **Single-owner: you only ever touch your own `<you>/` directory, so there is no peer-collision hazard** -- this is the key simplification over 2.0's shared-file archive. Commit via explicit pathspec (`git commit --only <you>/...`), never `-A`.

### touch

1. Update your `wip.md` `heartbeat_at` to now, read from `date -u +'%Y-%m-%d %H:%MZ'`. No other change. Run the command every time -- "now" is not a value you already know.

### release

1. Set your `wip.md` `status: paused`; update `heartbeat_at`. Leave `claims` + body intact.

### Fold vocabulary (localfold / globalfold)

The human may say "localfold" or "globalfold" (terms from Lamplight; defined in `/in-finish`). In whiteboard terms: **localfold** = tidy your OWN node before a compact -- migrate settled `## Decisions` into `wip.md`, `archive` your own DONE content, then `release`. **globalfold** = the project-wide snapshot (`intent/wip.md` / `restart.md` / `done.md`), typically the coordinating / validation node's job, not a per-node op. Either way you only ever fold your own `<you>/` directory.

### status

1. Read every `<node>/wip.md` header block.
2. Print one line per node: `<node>: <status>, focus=<focus>, claims=[...], heartbeat=<relative>`. No writes.

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

1. **One writer per file.** `wip.md` = the node; `inbox.<sender>.md` = the sender. The recipient owns its inbox lifecycle (reads, actions, clears into its own history).
2. **Live channel, not snapshot.** `intent/wip.md` is the post-session snapshot; `<node>/wip.md` is the live board.
3. **Claims by ST ID** (in the `wip.md` header block), never glob paths.
4. **Broadcast via `announce` -> peers' inboxes.** No shared file; a shared platform layer (eg `apps/lamplight/**`) is coordinated by announcing before you touch it.
5. **Heartbeat older than 7 days marks a claim reclaimable** -- reclaim requires explicit hypervisor acknowledgement.
6. **`/compact` does NOT end a session** -- status stays `active`; the next `pickup` touches the heartbeat.
7. **Archive your own dir only**, daily-or-more; `.history/YYYYMMDD/` is append-only and never reloaded on pickup.
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
| "I'll archive the whole board while I'm here."              | You archive only your own `<you>/` dir. Single-owner, collision-free -- that is the point.      |
| "The node said it's done, so it's done."                    | A "done" claim is the _trigger_ to verify, not the verdict. Read the as-built against the ask.  |
| "I know roughly what time it is."                           | You do not. You have no clock. Run `date -u`; a plausible stamp is fabricated, not approximate. |
| "I stamped one earlier this session, I'll reuse it."        | Time passed. Re-run `date -u` for every stamp, including the second one in the same turn.       |
