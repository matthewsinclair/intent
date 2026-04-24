---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-03
title: "Write intent/docs/working-with-llms.md canon tech note"
scope: Medium
status: Not Started
---

# WP-03: Write intent/docs/working-with-llms.md canon tech note

## Objective

Author `intent/docs/working-with-llms.md` — the long-form narrative tech note explaining Intent's canonical LLM-config surface for humans. Owns the "why" for all ten canon decisions (D1–D10 in `design.md`), the three-file architecture (AGENTS.md, CLAUDE.md, usage-rules.md), the internal-enforcement layer (`intent/llm/MODULES.md` + `DECISION_TREE.md`), the `.claude/settings.json` hook system, the critic cadence, and the Socrates-vs-Diogenes FAQ.

## Context

`usage-rules.md` (WP02) is rules-only — terse, prescriptive. `AGENTS.md` is auto-generated and can't carry opinion. `CLAUDE.md` is Claude-specific. None of them is the right place for the system-level narrative: why three files? why hooks? why pre-commit critic instead of CI-only? why Socrates and Diogenes both?

`working-with-llms.md` is that place. It's the canonical explanation — the doc that a new user (human or LLM) reads to understand Intent's opinionated LLM-config stance. Other docs cross-reference it for rationale.

This WP also owns the Socrates/Diogenes FAQ sidebar (~200 words) since WP12 only adds cross-references to agent.md files — the content lives here. That FAQ resolves the user confusion reported during Phase 0 planning.

## Deliverables

1. **New doc** at `intent/docs/working-with-llms.md` covering:
   - **Overview**: three-file canon + internal enforcement + narrative doc (= this file).
   - **D1–D10 decision narrative**: one section per decision, explaining the choice and trade-offs. Reference `intent/st/NOT-STARTED/ST0035/design.md` for the full risk register.
   - **Three-file architecture**: what goes in AGENTS.md vs CLAUDE.md vs usage-rules.md. Diagram (ascii-art) showing the layering.
   - **Hook architecture**: how `.claude/settings.json` hooks inject reminders; SessionStart + Stop events; why soft-reminder over hard-gate.
   - **Critic cadence**: pre-commit primary, CI secondary, session-end advisory. Explains when the LLM subagent critic-\* is invoked (stage 2 of `/in-review`) vs the headless `bin/intent_critic` (pre-commit).
   - **Skills auto-loaded via /in-session**: which skills fire per language; how language detection works; how to extend.
   - **Extensions at ~/.intent/ext/**: brief — defers to `writing-extensions.md` for authoring.
   - **Socrates vs Diogenes FAQ**: one paragraph ruling out the "it was always Socrates" confusion, citing the git-log forensics.
   - **For Elixir projects specifically**: how `mix usage_rules.sync` interacts with Intent's `usage-rules.md`.
   - **Troubleshooting**: common gotchas (e.g., SessionStart hook not firing, pre-commit blocking on a false positive, `intent claude upgrade --apply` refuses to overwrite a hand-edited file).
2. **Cross-references added from other docs**:
   - `README.md` links to working-with-llms.md under a "For LLM collaboration" heading.
   - `usage-rules.md` (from WP02) "See also" links here.
   - Root `AGENTS.md` template (WP08) links here.
   - `CLAUDE.md` template (WP09) links here.
3. **MODULES.md registration** — new doc listed under `intent/docs/`.

## Approach

1. Outline the doc: ~8 sections matching the deliverables list above. Target length: 500–800 lines. Longer is fine if forensic; but scannable.
2. Write the D1–D10 narrative first — this is the most substantive section. Pull rationale straight from `design.md`, expand with examples.
3. Write the three-file architecture section with an ascii diagram.
4. Write the hook architecture section referencing `.claude/settings.json` — stub the JSON example from what WP04 will ship (coordinate with WP04).
5. Write the critic cadence section referencing `bin/intent_critic` (stubbed; WP05 ships it) and `.git/hooks/pre-commit` (WP06).
6. Write the skills auto-load section — reference `in-session/SKILL.md` and the language detection logic in it.
7. Write the Socrates/Diogenes FAQ — use the commit-hash forensics from the Phase 0 research (Socrates `7f4529e` 2025-08-05; Diogenes `37a0ed0` 2026-02-20). Keep it one paragraph.
8. Write troubleshooting — anticipate the common gotchas.
9. Add cross-references in the four other files.
10. Commit.

## Acceptance Criteria

- [ ] `intent/docs/working-with-llms.md` exists and is ≥ 400 lines.
- [ ] All ten D1–D10 decisions have a narrative section covering trade-offs.
- [ ] Three-file architecture section includes an ascii diagram.
- [ ] Hook architecture section includes a JSON snippet showing `SessionStart` and `Stop` stanzas.
- [ ] Critic cadence section explains the split between `bin/intent_critic` (mechanical) and critic-\* subagents (LLM-based, richer review).
- [ ] Socrates vs Diogenes FAQ is present and cites the two git commit hashes.
- [ ] Troubleshooting section has ≥ 5 anticipated gotchas with fixes.
- [ ] Cross-references exist from `README.md`, `usage-rules.md`, root `AGENTS.md` template, root `CLAUDE.md` template.
- [ ] `intent/llm/MODULES.md` lists the new doc.
- [ ] Markdown linter: zero errors.
- [ ] Internal cross-references (e.g., `design.md D7`) resolve correctly.

### Tests to add

None directly — prose doc. Any existing "doc presence" smoke test updated to include this file.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP02 (refs `usage-rules.md`'s new structure).
- **Blocks**: WP08 (AGENTS.md generator references this doc), WP09 (CLAUDE.md template), WP12 (FAQ location — this WP authors the FAQ; WP12 adds cross-refs from `agent.md` files), WP13 (Intent's own CLAUDE.md).

## Implementation Notes

- **Voice**: authoritative, explanatory, opinionated. This is the doc that makes Intent's opinions visible. Don't hedge — the decisions are already made in `design.md`.
- **Length**: aim for 500–800 lines. If you're hitting 1000+, something's probably a separate doc.
- **Examples over abstractions**: every major section should include a concrete example (JSON snippet, CLI invocation, before/after file layout).
- **No duplication with `design.md`**: `design.md` is the decision log for ST0035 specifically (it's historical). `working-with-llms.md` is the living doc (current state). Some overlap is fine; copy-paste is not.
- **Reference existing docs**: `rules.md`, `critics.md`, `writing-extensions.md` already exist and are authoritative. Link, don't duplicate.
- **Socrates/Diogenes FAQ content**: can be reused verbatim across agent.md files (WP12) — keep the canonical version here.

## Risks and Edge Cases

- **Risk**: Doc drifts out of sync with later WPs that change canon details. **Mitigation**: write it _after_ WP02 lands (required) and _before_ WP08 merges — so the canon is stable when this doc is committed. Add a version line to the doc footer tied to Intent version.
- **Risk**: Too long to read. **Mitigation**: TOC at the top; each section summarises in its first paragraph.
- **Risk**: Example JSON for `.claude/settings.json` drifts from what WP04 actually ships. **Mitigation**: coordinate with WP04 — WP04 author should land first or WP03 updates its example post-WP04. Flag this in WP04's info.md.

## Verification Steps

1. `wc -l intent/docs/working-with-llms.md` ≥ 400.
2. `grep -c '^## D' intent/docs/working-with-llms.md` ≥ 10.
3. `grep -l "working-with-llms" README.md usage-rules.md` — all present.
4. `grep "7f4529e\|37a0ed0" intent/docs/working-with-llms.md` — both hashes present (Socrates/Diogenes FAQ).
5. Read doc end-to-end; confirm every decision has an example.
6. Cross-link integrity: each `design.md`-style reference points at a real section.

## Size and Estimate

- **Size**: M (Medium). 2–3 sessions.
- Session 1: Outline + D1–D10 sections.
- Session 2: Architecture diagram + hooks + critics + FAQ.
- Session 3: Skills auto-load + extensions brief + troubleshooting + cross-refs + polish.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Doc committed.
- [ ] Cross-refs committed in all four other files.
- [ ] MODULES.md updated.
- [ ] Socrates/Diogenes FAQ text finalised (WP12 reuses it).
- [ ] No placeholder text ("TODO", "TBD") remaining.
