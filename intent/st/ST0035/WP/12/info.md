---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-12
title: "Socrates Diogenes FAQ and cross-refs"
scope: ExtraSmall
status: Not Started
---

# WP-12: Socrates / Diogenes FAQ and cross-refs

## Objective

Resolve the reported user confusion around Socrates vs Diogenes agent naming. Cross-reference the canonical FAQ (authored in WP03 at `intent/docs/working-with-llms.md#socrates-vs-diogenes`) from both agent definition files (`intent/plugins/claude/subagents/socrates/agent.md`, `intent/plugins/claude/subagents/diogenes/agent.md`). No code change to either agent.

## Context

User reported during Phase 0: "it was always called Socrates and then it changes and I am not sure what happened there." Git log forensics (captured in `design.md` D6):

- **2025-08-05 (commit `7f4529e`)**: Socrates debuts as "CTO Review Mode". Personas: Socrates (CTO) + Plato (Tech Lead). Never had a testing role.
- **2026-02-20 (commit `37a0ed0`)**: Diogenes debuts as a separate new agent. Personas: Aristotle (Empiricist) + Diogenes (Skeptic) for test-specification dialogs.

They were never the same agent. Zero overlap. The confusion source is the shared Socratic-dialog methodology and the Greek-philosopher names.

WP03 authors the canonical FAQ paragraph. WP12 adds a cross-reference from each agent's `agent.md` file to that FAQ. No rename. No consolidation. No split.

## Deliverables

1. **Cross-reference in socrates/agent.md**: a one-line note near the top of the file ("See [Socrates vs Diogenes FAQ](../../../../docs/working-with-llms.md#socrates-vs-diogenes) — they are two different agents for two different domains.").
2. **Cross-reference in diogenes/agent.md**: mirror the line.
3. **Verification**: `grep -l "working-with-llms" intent/plugins/claude/subagents/socrates/agent.md intent/plugins/claude/subagents/diogenes/agent.md` returns both.
4. **Relative-path correctness**: verify the path from each agent.md resolves correctly to the target section.

## Approach

1. Confirm WP03 has committed `working-with-llms.md` with the `#socrates-vs-diogenes` anchor.
2. Read both `agent.md` files; identify the best insertion point (probably right after the YAML frontmatter or "Description" heading).
3. Add the cross-reference line in both files.
4. Verify anchor target exists.
5. Commit: `docs: cross-reference Socrates/Diogenes FAQ in agent.md files`.

## Acceptance Criteria

- [ ] `intent/plugins/claude/subagents/socrates/agent.md` contains the cross-reference line.
- [ ] `intent/plugins/claude/subagents/diogenes/agent.md` contains the cross-reference line.
- [ ] Relative paths resolve correctly (manual click-test or `grep` validation).
- [ ] `grep 'working-with-llms' intent/plugins/claude/subagents/{socrates,diogenes}/agent.md` returns matches.
- [ ] No other changes to either agent's definition, persona, or role.
- [ ] Commit follows Intent conventions.

### Tests to add

None. This is a one-line cross-ref change in two files.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP03 (canonical FAQ must exist at the referenced anchor).
- **Blocks**: None.

## Implementation Notes

- **Cross-reference phrasing**: keep it neutral, not apologetic. "Two different agents for two different domains" is the right tone.
- **Relative path**: from `intent/plugins/claude/subagents/socrates/agent.md` to `intent/docs/working-with-llms.md`, the relative path is `../../../../docs/working-with-llms.md`. Double-check by counting directory levels.
- **Anchor format**: markdown anchor for "## Socrates vs Diogenes FAQ" is `#socrates-vs-diogenes`. Confirm by reading the committed `working-with-llms.md`.
- **No `verblock` bump unless the agent.md style expects it**: these are subagent definitions; check existing agent.md files for verblock convention.

## Risks and Edge Cases

- **Risk**: Anchor drifts in `working-with-llms.md` after this WP lands. **Mitigation**: WP03 anchors should be stable at this point; if they change, WP12's cross-ref is updated in the same commit that moves the anchor.
- **Risk**: Agent definition change later invalidates the cross-ref. **Mitigation**: unlikely. Both agent.md files are stable.
- **Edge**: Path resolution differs when agent.md is installed at `~/.claude/agents/<name>.md` vs the source at `intent/plugins/claude/subagents/<name>/agent.md`. **Mitigation**: absolute URL anchor on claude.ai not applicable here — the reference is a human-readable link, not a machine-followed URL. Text is sufficient.

## Verification Steps

1. `grep "Socrates vs Diogenes" intent/plugins/claude/subagents/{socrates,diogenes}/agent.md` — both hit.
2. `grep "working-with-llms" intent/plugins/claude/subagents/{socrates,diogenes}/agent.md` — both hit.
3. Read both files; confirm the line is present and reads naturally.
4. Verify the anchor in `working-with-llms.md` matches.

## Size and Estimate

- **Size**: XS. Single session, ~10 minutes of work.

## Exit Checklist

- [ ] Both cross-refs in place.
- [ ] Anchor confirmed.
- [ ] Committed.
