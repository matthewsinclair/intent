# Whiteboard -- Protocol 3.0 (Intent)

Live coordination channel for concurrent Claude Code sessions -- and the human -- working on Intent itself. Each participant is a **node** (a workstream) with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what keeps the board contention-free and cleansable. `intent/wip.md` stays the post-session snapshot; the whiteboard is the live channel.

The full protocol lives in the `/in-whiteboard` skill (pickup / ask / announce / decide / claim / clear / archive / touch / release / status). The deterministic lifecycle -- scaffold / list / archive / hygiene -- and the session launch are `intent claude ws ...` and `intent claude start <node>` (ST0047). This file is the protocol pointer plus the Intent roster.

## Provenance

The whiteboard process was pioneered **by convention in Lamplight** (`../Lamplight/intent/whiteboard`) -- five hand-run nodes -- which remains the reference for how MAAC works in practice. **Baize** was the first **productised** use (the MVP). This board is the capability stood up first-class via `intent claude ws new`: Intent now dogfoods MAAC on its own development.

## Nodes (workstreams)

`hv` is **Workstream Zero** -- the always-present human node. The roster is five.

| Node | Name                   | Scope (Intent)                                                                                                                      |
| ---- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `hv` | Hypervisor (the human) | Workstream Zero: adjudicates scope, sequences work, owns releases plus commits-to-main; standing directives plus escalation landing |
| `cc` | Control Claude         | the engine: `bin/` CLI, `native/rust/crates/`, `intent/plugins/`, `lib/templates/`, the rule library, and skills; ST/WP execution   |
| `vc` | Validation Claude      | independent check (correct / complete / consistent / faithful to hv's ask); advisory; the bats suite plus critic discipline         |
| `ic` | Interface Claude       | the dispatch-table SSOT and everything rendered from it: command surface, help, voice, exit codes, MCP tool list, `intent llm`      |
| `dc` | DevX Claude            | dev-x and build environment, so that `cc` concentrates on functionality for the CLI / daemon (hv's words, 2026-08-15)               |

The earlier form of this section said Intent is CLI plus data rather than UX, so there was deliberately no interface node and the roster was three. That stopped being true when `ic` was scaffolded, and a roster doc that describes a node out of existence is worse than no roster doc. Corrected by vc 2026-08-14 on ic's ask, with hv AFK and the pen handed to all three nodes; `ic`'s scope line above is vc's charter ruling, adopted under hv's standing authorisation of 2026-08-14 and open to replacement.

`dc` was added by hv on 2026-08-15 and its scope line above is **hv's own framing, quoted rather than elaborated**, because the boundary between `dc` and `cc` is not yet ruled. vc's proposal, offered to hv and NOT adopted here: `dc` owns the environment the code builds and ships in (`native/` layout and workspace files, `.github/workflows/`, `.gitignore`, the devbin, hooks and pre-commit gate wiring, toolchain pinning, release mechanics); `cc` owns the code (`native/rust/crates/**`); and a disputed file is settled by asking whether changing it changes what the tool DOES or only how it gets built. **`bin/` is the one genuine collision** -- it holds both the v2 bash CLI (product) and `bin/int` (devbin) -- and it is open for hv rather than assumed by either node.

**This file has no single writer, which is why it goes stale.** It described `cc`'s lane as `crates/` for the whole of the `native/` reorganisation and nobody owned correcting it. Two candidate fixes, both open: give it a writer, or generate the roster rows from each node's own `wip.md` header so it cannot disagree with the boards it describes (cc's suggestion; the D30 direction, and probably free out of WP-14).

## Layout + single-writer rule

```
intent/whiteboard/
  README.md                 # this file -- protocol pointer + roster
  <node>/
    wip.md                  # the node's live board (single-writer = the node)
    inbox.<sender>.md       # messages FROM <sender> (single-writer = the sender)
    .history/YYYYMMDD/      # the node's archived DONE work + handled inbox entries
```

- `<node>/wip.md` -- written only by `<node>`.
- `<node>/inbox.<sender>.md` -- appended only by `<sender>`; read and cleansed only by `<node>` (the owner).

## The board's header block is NOT YAML

The `---` block at the top of a `wip.md` looks like YAML frontmatter and is not. It is a **line-oriented `key: value` block**, which is what every reader in the tool has always implemented:

- one line per key -- no multi-line values, block scalars, nesting, or comments;
- a single pair of surrounding quotes is a display delimiter and is stripped for display;
- **quotes inside a value are literal and are never escaped** -- write `focus: "the counted body is the SENT body"` exactly as it reads.

Escaping a quote to be "valid YAML" puts a literal backslash in your board. `intent claude ws hygiene` enforces this rule and says nothing about YAML validity, because validity is not the contract. Full rationale in the `/in-whiteboard` skill.

See the `/in-whiteboard` skill for the invariants (heartbeat reclaim, announce-before-shared-edit, archive-your-own-dir-only) and the per-subcommand procedures.
