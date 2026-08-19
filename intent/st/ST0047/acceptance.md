---
st_id: ST0047
title: Add claude_with_intent script to Intent to support muilt-agent agentic coding (MAAC)
---

# ST0047: Add claude_with_intent script to Intent to support muilt-agent agentic coding (MAAC) -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Whiteboard provisioner: ws new + ws list (status: Done)

- AC-01.1 `ws new <wsid>` scaffolds a Protocol-3.0 node: `<wsid>/wip.md` with valid frontmatter (node, name, role, session*id, heartbeat_at, status, focus, claims), `<wsid>/.history/.gitkeep`, and an inbox (`# inbox: <peer> -> <wsid>` header + `*(empty)\_` sentinel) for every existing peer. -- satisfied: yes (computed)
- AC-01.2 `ws new <wsid>` refuses with a non-zero exit and zero mutation when `<wsid>` already exists. -- satisfied: yes (computed)
- AC-01.3 `ws new` accepts any short-ish slug `[a-z0-9-]` and rejects an invalid id (uppercase / space / path separator / over-long) with a typed error, no partial scaffold. (No-Silent) -- satisfied: yes (computed)
- AC-01.4 On a whiteboard's first creation, `hv` is provisioned as Workstream Zero by default; working nodes appear only on an explicit `ws new`. -- satisfied: yes (computed)
- AC-01.5 `ws list` prints one line per node (id, status, focus, claims, heartbeat-age) read from each `wip.md` frontmatter, and performs no writes. -- satisfied: yes (computed)
- AC-01.6 (non-test) Regenerating the Baize `hv`+`cc`+`ic`+`vc` skeleton via `ws new` reproduces the hand-scaffolded golden reference (structural diff empty, modulo authored board content). -- evidence: dogfood 2026-06-25: ws new hv/cc/ic/vc reproduces the Baize golden board (28 paths); cwi_test.sh green -- satisfied: yes

### WP-02 -- Session launcher: start|st with provision-if-absent (status: Done)

- AC-02.1 `start <ws>` for an existing workstream assembles `claude --effort max --permission-mode auto --append-system-prompt <ctx> "/in-session"` (verified via `CWI_DRY_RUN`). -- satisfied: yes (computed)
- AC-02.2 `compose_ctx <ws>` output carries the workstream identity line, `.claude/restart.md`, and the standing "show a daily plan, then wait" instruction; it does not bake in the live board. -- satisfied: yes (computed)
- AC-02.3 `start <ws>` for an ABSENT workstream stops, reports non-existence, and prompts to create; `n` exits with zero mutation, `y` runs `ws new` then launches. (No-Silent) -- satisfied: yes (computed)
- AC-02.4 (non-test) A live `start ic` in Baize lands an interactive session that runs /in-session (gate released via the slash-exemption), picks up the cc opening assignment, and presents a plan. -- evidence: live 2026-06-25: bin/claude_with_intent start ic|vc booted sessions 201b7944/0ebe187b, ran /in-session+pickup, no typing -- satisfied: yes
- AC-02.5 (non-test) The launched posture is the hv-intended `auto` mode (not accept-edits / bypass), confirmed against the TUI status line. -- evidence: live 2026-06-25: permission-mode=auto recorded in both ic/vc transcripts -- satisfied: yes

### WP-03 -- Workstream lifecycle: ws archive + ws hygiene (status: Done)

- AC-03.1 `ws archive <wsid>` retires a node out of active discovery (eg to `.archived/<wsid>/`) with its `.history/` intact; the node no longer appears in `ws list`. -- satisfied: yes (computed)
- AC-03.2 `ws hygiene [<ws>]` validates structure (every node: parseable `wip.md` frontmatter + `.history/.gitkeep`; every inbox: header + sentinel-or-entries) and exits non-zero with a report on violation. -- satisfied: yes (computed)
- AC-03.3 `ws hygiene` performs only mechanical tidy (normalise sentinels / format; warn on oversized boards + stale heartbeats); it never archives DOING content (the semantic archive stays the Claude-driven `/in-whiteboard archive`). -- satisfied: yes (computed)
- AC-03.4 `ws hygiene` passes a clean board (zero exit, no mutation beyond formatting) and flags an intentionally-corrupted fixture (bad frontmatter / missing sentinel / oversized). -- satisfied: yes (computed)

### WP-04 -- Promote to Intent (intent claude) + back-fill siblings (status: Done)

- AC-04.1 (non-test) The validated capability is invocable as `intent claude start|ws ...` within the existing `intent claude` family. -- evidence: wired at `bin/intent` claude dispatch (`start|ws` -> `intent_claude_cwi`); `intent help claude` documents both; smoke + AT-04.1 green -- satisfied: yes
- AC-04.2 (non-test) `ws new` and the `/in-whiteboard` skill share one format SSOT (the skill points at the script; no divergent scaffold logic). (Highlander) -- evidence: SKILL.md "Scaffolding a node" repointed at `intent claude ws new` + the lazy-inbox drift reconciled to the script; AT-04.2 green -- satisfied: yes
- AC-04.3 (non-test) Back-filled into Laksa + Lamplight + Intent (each runs `intent claude start <ws>` against its own whiteboard). -- evidence: 2026-06-25 smoke -- satisfied: yes

## Acceptance Tests

### WP-01 -- Whiteboard provisioner: ws new + ws list (status: Done)

- AT-01.1 `tests/unit/claude_with_intent.bats` -- covers AC-01.1 -- status: green -- test: "AT-01.1 ws new scaffolds a protocol-3.0 node"
- AT-01.2 `tests/unit/claude_with_intent.bats` -- covers AC-01.2 -- status: green -- test: "AT-01.2 ws new refuses an existing wsid with no mutation"
- AT-01.3 `tests/unit/claude_with_intent.bats` -- covers AC-01.3 -- status: green -- test: "AT-01.3 ws new rejects invalid ids with no partial scaffold"
- AT-01.4 `tests/unit/claude_with_intent.bats` -- covers AC-01.4 -- status: green -- test: "AT-01.4 hv is workstream zero (active) by default; working nodes paused"
- AT-01.5 `tests/unit/claude_with_intent.bats` -- covers AC-01.5 -- status: green -- test: "AT-01.5 ws list prints one line per node from frontmatter, no writes"

### WP-02 -- Session launcher: start|st with provision-if-absent (status: Done)

- AT-02.1 `tests/unit/claude_with_intent.bats` -- covers AC-02.1 -- status: green -- test: "AT-02.1 start assembles the verified claude argv (dry-run)"
- AT-02.2 `tests/unit/claude_with_intent.bats` -- covers AC-02.2 -- status: green -- test: "AT-02.2 compose_ctx carries identity + pickup + plan instruction, not the board"
- AT-02.3 `tests/unit/claude_with_intent.bats` -- covers AC-02.3 -- status: green -- test: "AT-02.3 provision-if-absent: n exits clean (no mutation), y scaffolds then launches"

### WP-03 -- Workstream lifecycle: ws archive + ws hygiene (status: Done)

- AT-03.1 `tests/unit/claude_with_intent.bats` -- covers AC-03.1 -- status: green -- test: "AT-03.1 ws archive retires a node, history intact, gone from list"
- AT-03.2 `tests/unit/claude_with_intent.bats` -- covers AC-03.2 -- status: green -- test: "AT-03.2 ws hygiene flags a corrupted fixture non-zero with a report"
- AT-03.3 `tests/unit/claude_with_intent.bats` -- covers AC-03.3 -- status: green -- test: "AT-03.3 ws hygiene leaves DOING content untouched"
- AT-03.4 `tests/unit/claude_with_intent.bats` -- covers AC-03.4 -- status: green -- test: "AT-03.4 ws hygiene passes a clean board (zero exit, no mutation)"

### WP-04 -- Promote to Intent (intent claude) + back-fill siblings (status: Done)

- AT-04.1 `tests/unit/claude_with_intent.bats` -- covers AC-04.1 -- status: green -- test: "AT-04.1 invocable as intent claude start|ws through the dispatch"
- AT-04.2 `tests/unit/claude_with_intent.bats` -- covers AC-04.2 -- status: green -- test: "AT-04.2 in-whiteboard skill points at the script with no lazy-inbox drift (SSOT)"

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
