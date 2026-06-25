# Whiteboard -- Protocol 3.0 (Intent)

Live coordination channel for concurrent Claude Code sessions -- and the human -- working on Intent itself. Each participant is a **node** (a workstream) with its own directory under `intent/whiteboard/`. Every file has exactly one writer; that single-writer rule is what keeps the board contention-free and cleansable. `intent/wip.md` stays the post-session snapshot; the whiteboard is the live channel.

The full protocol lives in the `/in-whiteboard` skill (pickup / ask / announce / decide / claim / clear / archive / touch / release / status). The deterministic lifecycle -- scaffold / list / archive / hygiene -- and the session launch are `intent claude ws ...` and `intent claude start <node>` (ST0047). This file is the protocol pointer plus the Intent roster.

## Provenance

The whiteboard process was pioneered **by convention in Lamplight** (`../Lamplight/intent/whiteboard`) -- five hand-run nodes -- which remains the reference for how MAAC works in practice. **Baize** was the first **productised** use (the MVP). This board is the capability stood up first-class via `intent claude ws new`: Intent now dogfoods MAAC on its own development.

## Nodes (workstreams)

`hv` is **Workstream Zero** -- the always-present human node. Intent is CLI plus data, not UX, so there is deliberately no interface node; the roster is three.

| Node | Name                   | Scope (Intent)                                                                                                                      |
| ---- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `hv` | Hypervisor (the human) | Workstream Zero: adjudicates scope, sequences work, owns releases plus commits-to-main; standing directives plus escalation landing |
| `cc` | Control Claude         | the engine: `bin/` CLI, `intent/plugins/`, `lib/templates/`, the rule library, and skills; ST/WP execution                          |
| `vc` | Validation Claude      | independent check (correct / complete / consistent / faithful to hv's ask); advisory; the bats suite plus critic discipline         |

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

See the `/in-whiteboard` skill for the invariants (heartbeat reclaim, announce-before-shared-edit, archive-your-own-dir-only) and the per-subcommand procedures.
