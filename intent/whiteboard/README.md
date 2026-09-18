# Whiteboard -- Protocol 3.0 (Intent)

Live coordination channel for concurrent Claude Code sessions -- and the human -- working on Intent itself. Each participant is a **node** (a workstream) with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what keeps the board contention-free and cleansable. `intent/wip.md` stays the post-session snapshot; the whiteboard is the live channel.

The full protocol lives in the `/in-whiteboard` skill (pickup / ask / announce / decide / claim / clear / archive / touch / release / status). A node joins by `intent wb register`, its board and inboxes render from that row, and the session launch is `intent claude start <node>`. The file-era `intent claude ws` family (ST0047: scaffold, list, archive, hygiene) is gone, retired by ST0069 AC-14.12. This file is the protocol pointer plus the Intent roster.

## Provenance

The whiteboard process was pioneered **by convention in Lamplight** (`../Lamplight/intent/whiteboard`) -- five hand-run nodes -- which remains the reference for how MAAC works in practice. **Baize** was the first **productised** use (the MVP). This board was stood up first-class by the file-era `intent claude ws new`, since retired; it is now rows in the store, rendered under `intent/whiteboard/`. Intent dogfoods MAAC on its own development.

## Nodes (workstreams)

`hv` is **Workstream Zero** -- the always-present human node.

| Node | Name              | Scope (Intent)                                                                                                                      |
| ---- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `hv` | Hypervisor        | Workstream Zero: adjudicates scope, sequences work, owns releases plus commits-to-main; standing directives plus escalation landing |
| `cc` | Control Claude    | the engine: `native/rust/crates/`, `intent/plugins/`, `lib/templates/`, the rule library, and skills; ST/WP execution               |
| `vc` | Validation Claude | independent check (correct / complete / consistent / faithful to hv's ask); advisory; the bats suite plus critic discipline         |
| `ic` | Interface Claude  | the dispatch-table SSOT and everything rendered from it: command surface, help, voice, exit codes, MCP tool list, `intent llm`      |
| `dc` | DevX Claude       | dev-x and build environment, so that `cc` concentrates on functionality for the CLI / daemon (hv's words, 2026-08-15)               |

**THE hv INBOX'S READER IS `vc`, AND NAMING ONE IS NOW REQUIRED BY THE PROTOCOL RATHER THAN OPTIONAL HERE.** hv's own statement of it, 2026-08-19: _the workstreams can write in the hv channel FOR me, but I need that stuff surfaced TO me by vc._ So `hv/inbox.<node>.md` stays the durable write surface every node uses, and **`vc` is obliged to monitor it and surface its contents to hv in the live channel.** A node's escalation is not delivered when the write returns; it is delivered when vc has surfaced it.

This was added because the obligation did not exist in writing and its absence cost four days: four nodes wrote correctly into hv inboxes, in the right format, and hv was reading none of it. **Not one write failed, so nothing reported the gap** -- the write surface and the delivery were never distinguishable from inside. `/in-whiteboard` now requires every project's roster to name this reader for exactly that reason.

`ic`'s scope line above is vc's charter ruling of 2026-08-14, adopted under hv's standing authorisation and open to replacement.

`dc` was added by hv on 2026-08-15 and its scope line above is **hv's own framing, quoted rather than elaborated**, because the boundary between `dc` and `cc` is not yet ruled. vc's proposal, offered to hv and NOT adopted here: `dc` owns the environment the code builds and ships in (`native/` layout and workspace files, `.github/workflows/`, `.gitignore`, `bin/` (now only the devbin: `bin/int`, `bin/devbin`, `bin/.devbin/`), hooks and pre-commit gate wiring, toolchain pinning, release mechanics); `cc` owns the code (`native/rust/crates/**`); and a disputed file is settled by asking whether changing it changes what the tool DOES or only how it gets built.

**This file has no single writer, which is why its rows can lag.** It described `cc`'s lane as `crates/` for the whole of the `native/` reorganisation and nobody owned correcting it. The roster of record is the store: `intent wb register <moniker> --name <display> --role <role>` declares a node and `intent wb status` reads the roster back, so a row here that disagrees with `intent wb status` is this file's defect. What only this file carries is the scope column and the provenance.

## Layout + single-writer rule

```
intent/whiteboard/
  README.md                 # this file -- protocol pointer + roster
  <node>/
    board.json              # the node's row, rendered from the store
    wip.md                  # the node's board, rendered from the store (single-writer = the node, through `intent wb`)
    inbox.<sender>.md       # messages FROM <sender>, rendered (single-writer = the sender)
    .history/YYYYMMDD/      # the hand-authored era's archives; nothing writes here now
```

- `<node>/wip.md` -- changed only by `<node>` acting through `intent wb --node <node>`; a hand edit is skew and `intent doctor` reports it.
- `<node>/inbox.<sender>.md` -- appended only by `<sender>` through `intent wb ask` or `announce`; read and cleared only by `<node>` (the owner), with `intent wb clear`.

## The board's header block is NOT YAML

The `---` block at the top of a `wip.md` looks like YAML frontmatter and is not. It is a **line-oriented `key: value` block**, which is what every reader in the tool has always implemented:

- one line per key -- no multi-line values, block scalars, nesting, or comments;
- a single pair of surrounding quotes is a display delimiter and is stripped for display;
- **quotes inside a value are literal and are never escaped** -- write `focus: "the counted body is the SENT body"` exactly as it reads.

Escaping a quote to be "valid YAML" puts a literal backslash in your board. The whiteboard header guard (`lib/templates/hooks/whiteboard-header-guard.sh`) refuses the escape forms at commit time and says nothing about YAML validity, because validity is not the contract. Full rationale in the `/in-whiteboard` skill.

See the `/in-whiteboard` skill for the invariants (heartbeat reclaim, announce-before-shared-edit, archive your own items only) and the per-subcommand procedures.
