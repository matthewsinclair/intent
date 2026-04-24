---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-13
title: "Update Intent own CLAUDE.md to reference canon"
scope: Small
status: Not Started
---

# WP-13: Update Intent own CLAUDE.md to reference canon

## Objective

Rewrite Intent's own `CLAUDE.md` (at repo root) to reflect the new canon: it becomes a Claude-specific overlay that references `AGENTS.md`, `usage-rules.md`, and `intent/docs/working-with-llms.md` rather than duplicating content. Keeps Intent-specific dev guidance (Highlander Rule, Thin scripts, No silent failures, Check before you create, Register before you code, Single template source) but prunes duplication with the three root files.

## Context

Intent's own `CLAUDE.md` is 236 lines today. It was authored pre-v2.9.0 and carries rule content (Highlander, MODULES.md pointer, etc.) alongside Claude-specific directives. With D2 (CLAUDE.md as overlay, not standalone), the rule content belongs in `usage-rules.md`; the project overview belongs in `AGENTS.md`; the "why" belongs in `working-with-llms.md`.

Intent eats its own vegetables: the same overlay pattern we ship for downstream projects (via the WP09 template) applies to Intent itself. Intent's own CLAUDE.md is the reference example.

## Deliverables

1. **Rewritten `/Users/matts/Devel/prj/Intent/CLAUDE.md`** — short, Claude-specific overlay. Structure mirrors WP09's template:
   - Reference block (Intent v2.9.1; see AGENTS.md first).
   - Required skills (run `/in-session` at start; auto-loads coding skills per language).
   - Memory directory pointer.
   - Hooks pointer (`.claude/settings.json` → `working-with-llms.md#session-hooks`).
   - File mapping (intent/, .intent/, AGENTS.md, usage-rules.md, working-with-llms.md).
   - Rules of the road pointers (Highlander, Thin Coordinator, No Silent Errors — brief; full list in `usage-rules.md`).
   - Critic dispatch (`/in-review`, `Task(subagent_type=...)`, `bin/intent_critic`).
   - Author (matts).
   - Any Intent-specific addenda (legacy STP migration notes, v2.8.x upgrade notes) — keep because they're historically informative, but move the detailed migration section to a dedicated `intent/docs/migration-history.md` if it grows.
2. **Ensure CLAUDE.md points at the canon**: no duplication of AGENTS.md / usage-rules.md content.
3. **Preserve Intent's own rules table of contents**: the 6 numbered Intent rules (Highlander, Thin scripts, etc.) stay — they're the quick-reference. Full elaboration lives in `usage-rules.md` and `working-with-llms.md`.
4. **Key Reference Files section**: updated to current canon file set (root `AGENTS.md`, root `usage-rules.md`, `intent/docs/working-with-llms.md`, `intent/llm/MODULES.md`, `intent/llm/DECISION_TREE.md`, `intent/wip.md`, `intent/restart.md`).

## Approach

1. Read current Intent `CLAUDE.md` (already read in Phase 0 research).
2. Identify content that duplicates what will be in `AGENTS.md`, `usage-rules.md`, or `working-with-llms.md` — pull out.
3. Identify Intent-specific content worth keeping (the 6 numbered rules, migration history, author, GPG/commit rules) — keep and refine.
4. Rewrite following WP09's template structure, Intent-specific.
5. Verify cross-references resolve.
6. `tests/run_tests.sh` (no change expected; this is a doc change).
7. Commit: `docs: rewrite Intent CLAUDE.md as Claude-specific overlay`.

## Acceptance Criteria

- [ ] Intent's `CLAUDE.md` is ≤ 120 lines (currently 236).
- [ ] References root `AGENTS.md` for project overview.
- [ ] References `usage-rules.md` for full rules.
- [ ] References `intent/docs/working-with-llms.md` for rationale.
- [ ] References `.claude/settings.json` for hooks.
- [ ] Contains Intent's 6 numbered rules table-of-contents (with pointer to `usage-rules.md` for full).
- [ ] No duplication of AGENTS.md or usage-rules.md content.
- [ ] "Key Reference Files" section lists current canon files.
- [ ] Author / contact info preserved.
- [ ] Commit follows Intent conventions.

### Tests to add

None — doc change.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP03 (working-with-llms.md must exist), WP09 (CLAUDE.md template defines the overlay pattern Intent dogfoods).
- **Blocks**: WP14 (self-apply expects Intent's CLAUDE.md to be canon-compatible).

## Implementation Notes

- **Don't lose content**: the existing `CLAUDE.md` has valuable dev-guide content (GPG commits, Intent conventions, migration history). Find the right home: Intent-dev-specific stuff stays in `CLAUDE.md`; canon stuff moves to `usage-rules.md` / `AGENTS.md` / `working-with-llms.md`.
- **Voice**: authoritative-but-concise. This file is for developers of Intent (internal audience) _and_ for LLMs working on Intent. Keep it scannable.
- **Author line**: preserve the `## Author\n\nmatts` section.
- **Migration history**: existing `## Migration Notes` section is useful but growing. Consider moving the content to `intent/docs/migration-history.md` and linking. Defer the move to a follow-up ST if it's > 20 lines after rewrite.
- **Don't reintroduce duplication**: if the rewrite hits "I want to explain the rules here", that's a signal to link `usage-rules.md` instead.

## Risks and Edge Cases

- **Risk**: Shortening CLAUDE.md breaks LLMs that were relying on content now moved. **Mitigation**: `/in-session` auto-loads `usage-rules.md` via the skill chain; LLMs with session hooks will still get the rules. Verify in WP14 self-apply.
- **Risk**: Migration history section grows too much. **Mitigation**: split to `migration-history.md` as a follow-up.
- **Edge**: Intent's `CLAUDE.md` is also used as the reference example for "how a downstream project's CLAUDE.md should look after canon apply." Confirm it matches the WP09 template's structure.

## Verification Steps

1. `wc -l CLAUDE.md` ≤ 120.
2. `grep -l "AGENTS.md\|usage-rules.md\|working-with-llms" CLAUDE.md` — all three references present.
3. `grep "Highlander\|Thin" CLAUDE.md` — quick-reference still present.
4. Read end-to-end; confirm no duplication of AGENTS.md content.
5. `tests/run_tests.sh` — green.

## Size and Estimate

- **Size**: S (Small). 1–2 sessions.

## Exit Checklist

- [ ] CLAUDE.md rewritten.
- [ ] ≤ 120 lines.
- [ ] Cross-references resolve.
- [ ] No duplication.
- [ ] Committed.
