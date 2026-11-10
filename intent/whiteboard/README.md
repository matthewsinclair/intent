# Whiteboard -- Protocol 3.0 (Intent)

Live coordination channel for concurrent Claude Code sessions -- and the human -- working on Intent itself. Each participant is a **node**, or **seat**, with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what keeps the board contention-free and cleansable. `intent/wip.md` stays the post-session snapshot; the whiteboard is the live channel.

The verbs are `intent wb`'s, and `intent wb --help` lists them; the protocol, and the judgement each verb cannot carry, live in the `/in-whiteboard` skill. A node joins by `intent wb register`, its board and inboxes render from that row, and the session launch is `intent claude start <node>`. The file-era `intent claude ws` family (ST0047: scaffold, list, archive, hygiene) is gone, retired by ST0069 AC-14.12. This file is the protocol pointer plus the Intent roster. The words used here, Team, seat, moniker, role, remit and mediator, are defined once, in `intent/plugins/claude/teams/TERMS.md`.

## The Team

Profile: team. Mediator: vc. Human seat: hv.

| Moniker | Name              | Role       | Driven by | Remit                                                                                                       |
| ------- | ----------------- | ---------- | --------- | ----------------------------------------------------------------------------------------------------------- |
| `vc`    | Validation Claude | validation | a session | Boss of the Claude team: sequencing, briefs, validation of landed and claimed work, and the one route to hv |
| `cc`    | Control Claude    | control    | a session | Back-end services and the non-user-facing implementation of services                                        |
| `ic`    | Interface Claude  | interface  | a session | User-facing interfaces (UI and UX) and machine-facing interfaces (REST, GraphQL and the rest)               |
| `ac`    | Authoring Claude  | authoring  | a session | Authoring and content creation                                                                              |
| `dc`    | DevX Claude       | devx       | a session | The dev-x pipeline, end to end                                                                              |
| `hv`    | Hypervisor        | hypervisor | the human | Scope, sequencing, release policy, and every ratified decision                                              |

Every seat is registered whether or not it has ever been started. Each remit is the `team` profile's default, word for word, and Intent sets none of its own. The roster of record is the store: `intent wb status` reads it back, so a row here that disagrees with it is this file's defect.

### The route to hv

vc is the mediator, the one seat through which hv is reached. A seat that needs something from hv sends its request to vc, with `intent wb ask vc` and again over `SendMessage` when it must be acted on, and never to hv: hv reads no board unprompted and writes to no inbox. hv may speak to any seat directly, and a seat that hv rules to directly passes the ruling to vc, who records it on hv's board. No seat waits idle on hv: it asks vc, holds only the work that depends on the answer, and carries on with the rest. The files under `hv/inbox.*` have one reader, vc.

hv's standing brief, on hv's board, says the same to every seat at every `intent wb pickup`.

### What this section is

It is written by hand, on hv's word of 2026-11-10, in the form `intent wb init` will generate. Until that verb exists the tool does not refuse an ask to hv, a remit is not a stored field, the declared profile is not in the store, no seat can be retired, and this file has no single writer, so a row here can lag.

## Intent's own notes

### Where each remit lands in this tree

- `vc`: the independent check of landed and claimed work (correct, complete, consistent, faithful to hv's ask), the briefs and orders each lane works to, and hv's delegated pen, under which it directs the lanes (hv decision 4).
- `cc`: the engine. `native/rust/crates/`, `intent/plugins/`, `lib/templates/`, the rule library and the skills.
- `ic`: the dispatch table and everything rendered from it: the command surface, help, voice, exit codes, the MCP tool list and `intent llm`. Intent.app and the TUI are user-facing interfaces, so by the remit they are ic's too.
- `dc`: dev-x and the build environment, so that `cc` concentrates on functionality for the CLI and the daemon (hv's words, 2026-08-15).

ic's line is vc's charter ruling of 2026-08-14, adopted under hv's standing authorisation and open to replacement. The line between `dc` and `cc` is not ruled. vc's proposal, offered to hv and not adopted: `dc` owns the environment the code builds and ships in (`native/` layout and workspace files, `.github/workflows/`, `.gitignore`, `bin/` (now only the devbin: `bin/int`, `bin/devbin`, `bin/.devbin/`), hooks and pre-commit gate wiring, toolchain pinning, release mechanics); `cc` owns the code (`native/rust/crates/**`); and a disputed file is settled by asking whether changing it changes what the tool DOES or only how it gets built.

### Provenance

The whiteboard process was pioneered **by convention in Lamplight** (`../Lamplight/intent/whiteboard`) -- five hand-run nodes -- which remains the reference for how MAAC works in practice. **Baize** was the first **productised** use (the MVP). This board was stood up first-class by the file-era `intent claude ws new`, since retired; it is now rows in the store, rendered under `intent/whiteboard/`. Intent dogfoods MAAC on its own development.

### Layout + single-writer rule

```
intent/whiteboard/
  README.md                 # this file -- protocol pointer + roster
  <node>/
    board.json              # the node's canon, rendered from the store: its items, the messages it sent, its handled marks
    wip.md                  # the node's board, rendered from the store (single-writer = the node, through `intent wb`)
    inbox.<sender>.md       # messages FROM <sender>, rendered (single-writer = the sender)
    .history/               # the hand-authored era's archives, and the pre-migration copies `intent wb migrate` keeps
```

- `<node>/wip.md` -- changed only by `<node>` acting through `intent wb --node <node>`; a hand edit is skew and `intent doctor` reports it.
- `<node>/inbox.<sender>.md` -- written only by `<sender>`, rendered from `<sender>/board.json` alone: appended through `intent wb ask` or `announce`, a sent message's body changed through `intent wb edit message`, and a handled message taken out through `intent wb archive message`; read only by `<node>` (the owner), which clears with `intent wb clear` by marking in its own `board.json`, so the file never changes when it does.
- A node commits what its own verbs wrote, as each verb prints it on its `wrote:` lines: its `board.json` and `wip.md`, every `<peer>/inbox.<node>.md`, and its own event files. A `re-rendered:` line is another node's file to commit, named with its owner (issue 0612).

### The board's header block is NOT YAML

The `---` block at the top of a `wip.md` looks like YAML frontmatter and is not. It is a **line-oriented `key: value` block**, which is what every reader in the tool has always implemented:

- one line per key -- no multi-line values, block scalars, nesting, or comments;
- a single pair of surrounding quotes is a display delimiter and is stripped for display;
- **quotes inside a value are literal and are never escaped** -- write `focus: "the counted body is the SENT body"` exactly as it reads.

Escaping a quote to be "valid YAML" puts a literal backslash in your board. The whiteboard header guard (`lib/templates/hooks/whiteboard-header-guard.sh`) refuses the escape forms at commit time and says nothing about YAML validity, because validity is not the contract. Full rationale in the `/in-whiteboard` skill.

See the `/in-whiteboard` skill for the invariants (heartbeat reclaim, announce-before-shared-edit, archive your own items only) and the per-subcommand procedures.
