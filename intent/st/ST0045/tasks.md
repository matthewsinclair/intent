# Tasks - ST0045: Update whiteboard for per-workstream files

## Tasks

Work breakdown mapped to the ratified ACs in `acceptance.md`. The Protocol 3.0 skill rewrite (per-node dirs + single-writer inboxes + `hv`) was landed pre-contract (commits `f021818` + `b66484b`); this thread adds the AC/AT contract, closes the reference-vs-skill drift, and rolls out.

- [x] AC-00.1 -- `in-whiteboard` SKILL.md is Protocol 3.0 and generic (per-node dirs, single-writer `wip.md` + per-sender inboxes, `hv` convention, no baked-in roster). Landed pre-contract; ratified here.
- [x] AC-01.1 -- document inbox-file init in SKILL.md: `# inbox: <sender> -> <recipient>` header + single-writer note + `_(empty)_` sentinel + which subcommand creates an absent inbox (added `## inbox.<sender>.md shape` + `ask` step 1).
- [x] AC-01.2 -- document `.history/.gitkeep` scaffolding (file-layout block + scaffolding note).
- [x] AC-01.3 -- document the `hv` node variant (added `### The hv (hypervisor) node`: no session loop, `session_id` optional/none, advisory heartbeat, `## Standing directives`).
- [x] AC-01.4 -- message-entry format internally consistent + required-vs-recommended fields stated (added `### Message-entry format`).
- [x] AC-02.1 / AC-02.2 -- write `tests/unit/whiteboard_protocol_3_guard.bats` (red-first), then bring the chaining skills + canon doc to 3.0 to green it.
- [x] AC-03.2 -- populate `tasks.md` + `impl.md` (this file + impl.md).
- [ ] AC-03.1 -- rollout: rides the v2.12.0 release (version bump, CHANGELOG, `intent/history/v2.12.0.md`, `intent claude skills sync`). Satisfied at release.

## Task Notes

- AT-02.1 (in-whiteboard SKILL.md: no live 2.0 refs) is a green-by-construction regression guard -- the skill was already 3.0 when the guard was written, so it passed on arrival. It exists to catch a 2.0 regression, not to drive the rewrite.
- AT-02.2 was genuinely red-first: `in-session/SKILL.md`, `in-finish/SKILL.md`, and the whiteboard section of `intent/docs/working-with-llms.md` all hard-coded the flat per-stream model until rewritten here.
- The guard's 2.0-token set boundary-guards `asks.md` (`[^[:alpha:]]asks\.md`) so it does not match the `tasks.md` substring.

## Dependencies

- AC-03.1 depends on the v2.12.0 release (shared with ST0043). All other ACs are self-contained in this thread.
