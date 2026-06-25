---
verblock: "25 Jun 2026:v0.2: matts - Real ACs/ATs per WP"
st_id: ST0047
title: "Add claude_with_intent script to Intent to support muilt-agent agentic coding (MAAC) -- acceptance contract"
---

# ST0047 Add claude_with_intent script to Intent to support muilt-agent agentic coding (MAAC) -- Acceptance

> Canonical acceptance contract for ST0047. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Test harness: the Baize prototype proved WP-01..03 via a dependency-free `test/cwi/cwi_test.sh`; on promotion (WP-04) the ATs are re-homed to Intent's bats suite `tests/unit/claude_with_intent.bats`, driving the command through the real `intent claude` dispatch. AT ids map to its `@test` names.

## Acceptance Criteria

### ST-level

- none -- WP-distributed.

### WP-01 -- Whiteboard provisioner: ws new + ws list (status: Done)

- AC-01.1 `ws new <wsid>` scaffolds a Protocol-3.0 node: `<wsid>/wip.md` with valid frontmatter (node, name, role, session*id, heartbeat_at, status, focus, claims), `<wsid>/.history/.gitkeep`, and an inbox (`# inbox: <peer> -> <wsid>` header + `*(empty)\_` sentinel) for every existing peer.
- AC-01.2 `ws new <wsid>` refuses with a non-zero exit and zero mutation when `<wsid>` already exists.
- AC-01.3 `ws new` accepts any short-ish slug `[a-z0-9-]` and rejects an invalid id (uppercase / space / path separator / over-long) with a typed error, no partial scaffold. (No-Silent)
- AC-01.4 On a whiteboard's first creation, `hv` is provisioned as Workstream Zero by default; working nodes appear only on an explicit `ws new`.
- AC-01.5 `ws list` prints one line per node (id, status, focus, claims, heartbeat-age) read from each `wip.md` frontmatter, and performs no writes.
- AC-01.6 (non-test) Regenerating the Baize `hv`+`cc`+`ic`+`vc` skeleton via `ws new` reproduces the hand-scaffolded golden reference (structural diff empty, modulo authored board content). -- evidence: dogfood 2026-06-25: ws new hv/cc/ic/vc reproduces the Baize golden board (28 paths); cwi_test.sh green -- satisfied: yes

### WP-02 -- Session launcher: start|st with provision-if-absent (status: Done)

- AC-02.1 `start <ws>` for an existing workstream assembles `claude --effort max --permission-mode auto --append-system-prompt <ctx> "/in-session"` (verified via `CWI_DRY_RUN`).
- AC-02.2 `compose_ctx <ws>` output carries the workstream identity line, `.claude/restart.md`, and the standing "show a daily plan, then wait" instruction; it does not bake in the live board.
- AC-02.3 `start <ws>` for an ABSENT workstream stops, reports non-existence, and prompts to create; `n` exits with zero mutation, `y` runs `ws new` then launches. (No-Silent)
- AC-02.4 (non-test) A live `start ic` in Baize lands an interactive session that runs /in-session (gate released via the slash-exemption), picks up the cc opening assignment, and presents a plan. -- evidence: live 2026-06-25: bin/claude_with_intent start ic|vc booted sessions 201b7944/0ebe187b, ran /in-session+pickup, no typing -- satisfied: yes
- AC-02.5 (non-test) The launched posture is the hv-intended `auto` mode (not accept-edits / bypass), confirmed against the TUI status line. -- evidence: live 2026-06-25: permission-mode=auto recorded in both ic/vc transcripts -- satisfied: yes

### WP-03 -- Workstream lifecycle: ws archive + ws hygiene (status: Done)

- AC-03.1 `ws archive <wsid>` retires a node out of active discovery (eg to `.archived/<wsid>/`) with its `.history/` intact; the node no longer appears in `ws list`.
- AC-03.2 `ws hygiene [<ws>]` validates structure (every node: parseable `wip.md` frontmatter + `.history/.gitkeep`; every inbox: header + sentinel-or-entries) and exits non-zero with a report on violation.
- AC-03.3 `ws hygiene` performs only mechanical tidy (normalise sentinels / format; warn on oversized boards + stale heartbeats); it never archives DOING content (the semantic archive stays the Claude-driven `/in-whiteboard archive`).
- AC-03.4 `ws hygiene` passes a clean board (zero exit, no mutation beyond formatting) and flags an intentionally-corrupted fixture (bad frontmatter / missing sentinel / oversized).

### WP-04 -- Promote to Intent (intent claude) + back-fill siblings (status: In Progress)

- AC-04.1 (non-test) The validated capability is invocable as `intent claude start|ws ...` within the existing `intent claude` family. -- evidence: wired at `bin/intent` claude dispatch (`start|ws` -> `intent_claude_cwi`); `intent help claude` documents both; smoke + AT-04.1 green -- satisfied: yes
- AC-04.2 (non-test) `ws new` and the `/in-whiteboard` skill share one format SSOT (the skill points at the script; no divergent scaffold logic). (Highlander) -- evidence: SKILL.md "Scaffolding a node" repointed at `intent claude ws new` + the lazy-inbox drift reconciled to the script; AT-04.2 green -- satisfied: yes
- AC-04.3 (non-test) Back-filled into Laksa + Lamplight + Intent (each runs `intent claude start <ws>` against its own whiteboard). -- evidence: 2026-06-25 smoke -- the command is served centrally from `$INTENT_HOME` (no per-project install): Intent board stood up (hv+cc+vc) + `ws list` and `start cc` dry-run green; Lamplight `ws list`/`ws hygiene` read its live 5-node board (hygiene WARNs the 3 oversized boards, exit 0); Laksa (no board) errors gracefully ("no whiteboard at ...", exit 1). Live interactive `start` per-project is hv's to exercise (cf AC-02.4). -- satisfied: yes

## Acceptance Tests

All ATs live in `tests/unit/claude_with_intent.bats` (ported from the Baize `test/cwi/cwi_test.sh` on promotion), driving the command through the real `intent claude` dispatch against a scratch board (`CWI_WB`) + the `CWI_DRY_RUN` launch seam.

### WP-01

- AT-01.1 claude_with_intent.bats::"AT-01.1 ws new scaffolds a protocol-3.0 node" -- covers AC-01.1 -- status: green
- AT-01.2 claude_with_intent.bats::"AT-01.2 ws new refuses an existing wsid with no mutation" -- covers AC-01.2 -- status: green
- AT-01.3 claude_with_intent.bats::"AT-01.3 ws new rejects invalid ids with no partial scaffold" -- covers AC-01.3 -- status: green
- AT-01.4 claude_with_intent.bats::"AT-01.4 hv is workstream zero (active) by default; working nodes paused" -- covers AC-01.4 -- status: green
- AT-01.5 claude_with_intent.bats::"AT-01.5 ws list prints one line per node from frontmatter, no writes" -- covers AC-01.5 -- status: green
- Coverage: AC-01.1..01.5 test-backed (ATs above); AC-01.6 non-test (dogfood diff evidence on the AC line).

### WP-02

- AT-02.1 claude_with_intent.bats::"AT-02.1 start assembles the verified claude argv (dry-run)" -- covers AC-02.1 -- status: green
- AT-02.2 claude_with_intent.bats::"AT-02.2 compose_ctx carries identity + pickup + plan instruction, not the board" -- covers AC-02.2 -- status: green
- AT-02.3 claude_with_intent.bats::"AT-02.3 provision-if-absent: n exits clean (no mutation), y scaffolds then launches" -- covers AC-02.3 -- status: green
- Coverage: AC-02.1..02.3 test-backed; AC-02.4 + AC-02.5 non-test (live-launch evidence on the AC lines).

### WP-03

- AT-03.1 claude_with_intent.bats::"AT-03.1 ws archive retires a node, history intact, gone from list" -- covers AC-03.1 -- status: green
- AT-03.2 claude_with_intent.bats::"AT-03.2 ws hygiene flags a corrupted fixture non-zero with a report" -- covers AC-03.2 -- status: green
- AT-03.3 claude_with_intent.bats::"AT-03.3 ws hygiene leaves DOING content untouched" -- covers AC-03.3 -- status: green
- AT-03.4 claude_with_intent.bats::"AT-03.4 ws hygiene passes a clean board (zero exit, no mutation)" -- covers AC-03.4 -- status: green
- Coverage: AC-03.1..03.4 test-backed (ATs above).

### WP-04

- AT-04.1 claude_with_intent.bats::"AT-04.1 invocable as intent claude start|ws through the dispatch" -- covers AC-04.1 -- status: green
- AT-04.2 claude_with_intent.bats::"AT-04.2 in-whiteboard skill points at the script with no lazy-inbox drift (SSOT)" -- covers AC-04.2 -- status: green
- Coverage: AC-04.1 + AC-04.2 now test-backed (mechanical guards above); AC-04.3 non-test (per-project back-fill smoke, evidence on the AC line).
