---
verblock: "15 Jun 2026:v0.3: matts - Ratified + numeric AC/AT ids (parser-conformant)"
st_id: ST0045
title: "Update whiteboard for per-workstream files -- acceptance contract"
---

# ST0045 Update whiteboard for per-workstream files -- Acceptance

> Canonical acceptance contract for ST0045. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> STATUS: RATIFIED 2026-06-15 (matts, open-gate). ACs derived from design.md plus the reference-vs-skill drift audit against the live Lamplight Protocol 3.0 implementation. Ships in v2.12.0. (AC/AT ids are numeric per the `intent ac/at` parser.)

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) The `in-whiteboard` skill is Protocol 3.0 and generic: per-node `<node>/` directories, single-writer `wip.md` plus per-sender `inbox.<sender>.md`, the `hv` hypervisor convention, and no roster baked into the skill (nodes discovered by listing `intent/whiteboard/*/`). -- evidence: in-whiteboard SKILL.md is Protocol 3.0 + generic (per-node dirs, single-writer inboxes, hv node, no baked roster); shipped in v2.12.0 (matts) -- satisfied: yes

### Skill completeness (reference-vs-skill drift)

- AC-01.1 (non-test) Inbox-file initialization is specified in the skill: the `# inbox: <sender> -> <recipient>` header + single-writer note + the `_(empty)_` sentinel, and which subcommand creates an absent inbox on first send. -- evidence: SKILL.md ## inbox.<sender>.md shape + ask step 1 -- satisfied: yes
- AC-01.2 (non-test) The empty hidden archive dir scaffolding is documented: each node's `.history/` is tracked via `.history/.gitkeep` (git does not track empty directories). -- evidence: SKILL.md file-layout .history/.gitkeep + scaffolding note -- satisfied: yes
- AC-01.3 (non-test) The `hv` node variant is documented: human-driven, `session_id` optional or `none`, may carry a standing-directives section beyond the canonical body. -- evidence: SKILL.md ### The hv (hypervisor) node -- satisfied: yes
- AC-01.4 (non-test) The message-entry format is internally consistent and the required-vs-recommended fields are stated (timestamp granularity; `Re:` / `FYI only` optional). -- evidence: SKILL.md ### Message-entry format -- satisfied: yes

### Mechanical guards

- AC-02.1 The shipped `in-whiteboard/SKILL.md` contains no live references to the retired 2.0 model (`asks.md`, `lamplight.md`, `cookies.md`, the flat per-stream files, the `to:` / `from:` ask header) except where explicitly naming them as retired.
- AC-02.2 The chaining skills (`in-session` pickup step, `in-finish` release step) reference the 3.0 subcommands, and no shipped skill / doc hard-codes the flat-file 2.0 model.

### Rollout + docs

- AC-03.1 (non-test) Rollout is complete: v2.12.0 version bump, CHANGELOG + `intent/history/v2.12.0.md` entry covering Protocol 3.0, and `intent claude skills sync` installs the 3.0 skill. -- evidence: CHANGELOG + intent/history/v2.12.0.md written; 3.0 skill synced (in-session/in-finish/in-whiteboard updated); VERSION bump via scripts/release --minor (matts, immediately following) -- satisfied: yes
- AC-03.2 (non-test) ST0045 `tasks.md` + `impl.md` are populated (work breakdown + as-built). -- evidence: ST0045 tasks.md + impl.md populated -- satisfied: yes

## Acceptance Tests

### Mechanical guards

- AT-02.1 `tests/unit/whiteboard_protocol_3_guard.bats::in-whiteboard SKILL.md has no live 2.0 references` -- covers AC-02.1 -- status: green
- AT-02.2 `tests/unit/whiteboard_protocol_3_guard.bats::chaining skills reference 3.0 subcommands and no shipped doc hard-codes the flat-file model` -- covers AC-02.2 -- status: green
- Coverage: AC-02.1 / AC-02.2 covered by the ATs above. AC-00.1, AC-01.1..01.4, AC-03.1, AC-03.2 are non-test (evidence carried on the AC line, satisfied via `intent ac satisfy`). No uncovered test-backed ACs.
