---
verblock: "15 Aug 2026:v0.2: vc - Specced from hv's ask; D14's deferral superseded on evidence"
wp_id: WP-14
title: "Coordination model: whiteboard and inboxes in the store, with a bounded API"
scope: L
status: Not Started
---

# WP-14: Coordination model: whiteboard and inboxes in the store, with a bounded API

## Objective

Bring the whiteboard -- node boards and inboxes -- **into the model**, served by a regularised API that bounds what it accepts, so coordination state is queryable from every workstream and cannot grow without limit. Boards become state, not narrative.

## What hv asked, and what is already true

hv's ask has two halves and only one is new work.

**Already designed, needs no WP.** ACs and ATs are already model entities -- `acceptance_criterion` and `acceptance_test` live inside `thread.json` (`data-model.md:83,94`), `intent ac` / `intent at` are already the API, and `acceptance.md` is already a generated view under D02. The choke point hv wants for acceptance already exists in the canon; what remains there is building it, which is WP-04's and WP-06's job, not a new one.

**The new work is the whiteboard**, which `data-model.md:189` currently and deliberately excludes from the model.

## Why D14's deferral does not survive

D14 says the whiteboard stays md-authored through 3.0.0/3.1, restructured in the 3.2 bus ST. **The deferral was argued on transport**, and design.md:240 states the argument in as many words: an inbox append is a file write, D20 already ships `fileChanged(project_id, path)`, so a node gets live delivery "with no new protocol, no new transport, and D14 intact -- the boards stay md-authored and unmodelled, **because a file does not need modelling to fire a change event**".

That is correct, and it answers **delivery**. hv's directive is about **shape, size and searchability**, which the transport argument never reached. D14 is not being overturned as wrong; it is being completed on the question it did not ask.

## The evidence, measured

The live board is **102,886 bytes** across 17 files; `.history` holds a further **251,244 bytes** across 23. The distribution is the finding:

| file             | bytes      |
| ---------------- | ---------- |
| `vc/inbox.ic.md` | **31,998** |
| `ic/wip.md`      | 14,535     |
| `ic/inbox.vc.md` | 15,155     |
| `cc/wip.md`      | 9,588      |
| `vc/inbox.cc.md` | 9,817      |
| `vc/wip.md`      | 8,145      |
| **`hv/wip.md`**  | **308**    |

Three LLM nodes wrote ~100KB of board in roughly two days. The human wrote 308 bytes. **That ratio is the specification**: hv's board is what a board is for, and the other three are what happens without a mechanism.

**The rule already existed and discipline did not keep it.** `vc/wip.md` opens its own watch-outs with "Measurement rules earned on this thread live in `intent/st/ST0056/parity.md` ... not here -- a board does not outlive the session that writes it", and that same file is 8,145 bytes. A rule stated by the node that then breaks it is the definition of a rule needing a mechanism rather than better intentions.

## The principle the bound enforces

**A finding belongs in the artefact it is about; the board carries the pointer.** ic's stale-baseline finding belongs in `parity.md` and its measurement rules do live there. vc's AC-10.7 finding belongs in `acceptance.md` and it does. What the board should have carried in both cases is one line naming the artefact -- not the account, which is durable content that a board archives away within a day.

This is why the bound is not hostile to the prose that has been working. The cross-node findings this thread has produced were load-bearing and several prevented defects. They were valuable **because they landed in durable artefacts**, and the board copy was always the redundant one.

## Deliverables

- **Model.** `wb_node` (moniker, display name, role, session_id, heartbeat_at, status, focus, claims), `wb_item` (node, kind `doing|todo|decision|watchout`, seq, text, state, timestamps), `wb_message` (sender, recipient, sent_at, body, `re` anchor, fyi flag, state `live|handled`). Durable form is committed JSON canon per D01; the DB is the rebuildable runtime index; `.md` becomes a **generated view** under D02, ending the hand-authored board.
- **API.** An `intent wb` family covering the `/in-whiteboard` verbs -- `pickup`, `ask`, `announce`, `decide`, `claim`/`unclaim`, `clear`, `archive`, `touch`, `release`, `status` -- served from the store, available in-process and over GraphQL so any workstream reads any board.
- **The bounds, enforced by refusal.** Per-entry body size, live items per node per kind, and live messages per inbox are bounded and configurable. Exceeding a bound is **refused by name with the bound and the remedy stated**, never truncated and never silently accepted -- the D05 `additionalProperties: false` posture applied to size, and IN-AG-NO-SILENT-001 applied to a write path.
- **Archival becomes automatic.** Handled messages and completed DOING items roll to history on the API's own schedule rather than when a node remembers to run `archive`. The 251KB of `.history` is what remembering-to-archive produces.
- **Search.** Boards and inboxes are FTS-indexed with the rest of the corpus, so WP-13's `intent search` reaches coordination state.
- **Migration.** The existing three-node board (~354KB live + archived) ports into the model, with the ingest reporting what it could not carry rather than dropping it.

## The principle this WP is built on

**A control refuses; documentation reminds; only one of them is load-bearing** (cc's compression, 2026-08-15).

Earned rather than asserted. In one session vc fabricated four whiteboard timestamps -- while writing the clock rule, enforcing it on a peer, and citing it in the message carrying the fourth -- and cc read a corpus through `| head`, lost the eleventh of eleven rows, and published the wrong count into a source comment and a commit message, with `| head` already on their own board three lines from where they were looking. **Neither failure was ignorance; in both cases the author had written the rule that day**, which is close to the strongest available disproof that knowing a rule is sufficient to follow it.

The two mechanisms that did work that session both **refused** and neither asked anyone to remember: the pre-commit clock guard refused a bad stamp, and `lib_corpus.sh` refused a register generated against an incomplete baseline. Every bound in this WP is specified as a refusal for that reason.

## The unstated win: fabricated timestamps stop being possible

The protocol's sharpest failure mode is a hand-written stamp -- a value an LLM node has no clock to produce, so a plausible one is invented whole. Both vc and ic fabricated stamps within the last two days; vc's was refused by the pre-commit clock guard, ic's was wrong twice.

**An API that stamps from the clock eliminates the class by construction.** The guard exists because stamps are hand-authored; once the API is the only writer, the guard demotes from primary defence to a legacy-file check. This is a structural fix to a class currently held by a detector, and it was not part of hv's ask.

## What this costs, stated plainly

Hand-editing a board stops working, because the `.md` becomes generated. That is the choke point functioning as designed rather than a side effect, but it is a real change to how all three nodes work, and `/in-whiteboard` plus the `intent claude ws` family must move with it in the same WP or the protocol documents a workflow the tool refuses.

## Dependencies

- **WP-02** (reified model, schema faces, store) -- the tables land in the same schema-as-truth pipeline.
- **WP-03** (ingest, views, sync) -- the generated-view machinery and the no-clock renderer law already exist; boards are another view.
- **WP-13** (project search) for the FTS reach, though the index itself is WP-03's.
- Supersedes the whiteboard half of the parked 3.2 agent-bus ST (`design.md:234`). The oversight-gates half stays parked.

## Open for hv

- **The bound values.** The mechanism is contracted here; the numbers are configuration and are hv's to set. Recommended starting point, derived from the measurement above rather than invented: message body 2KB, live messages per inbox 10, DOING 5, TODO 15, board total 8KB. `hv/wip.md` at 308 bytes suggests these are generous.
- **Whether `hv` is bounded at all.** The human's board is the one that has never needed a limit, and a refusal aimed at hv is the mechanism annoying the person it was built for.
