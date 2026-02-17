---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: Create LiveView Reference Doc + Skill
scope: Medium
status: Not Started
---

# WP-04: Create LiveView Reference Doc + Skill

## Objective

Create comprehensive LiveView guidance covering operational patterns missing from current agent.md — two-phase rendering, streams, navigation semantics, error boundaries.

## Deliverables

- `intent/plugins/claude/subagents/elixir/liveview.md` — reference doc (~200-300 lines)
- `intent/plugins/claude/skills/phoenix-liveview/SKILL.md` — skill (~120 lines)

## Coverage Gaps Being Filled

- `connected?/1` guidance (prevents PubSub bugs during static render)
- Two-phase rendering (mount called twice: static then connected)
- `stream/3` for efficient large-list handling (Phoenix 1.7+)
- `push_navigate/2` vs `push_patch/2` semantics
- File upload patterns and security
- Error boundary patterns
- `assign_async/3` for non-blocking data loading

## Skill Rules (~7)

1. Two-phase mount — guard PubSub/async with `connected?(socket)`
2. Streams for large lists — never assign full collections
3. `@impl true` on all callbacks
4. Thin LiveViews — domain logic in context/domain modules
5. `push_navigate` vs `push_patch` — correct semantics
6. `assign_async` for non-blocking data loading
7. Component extraction for repeated HEEX patterns

## Acceptance Criteria

- [ ] Reference doc covers all gaps listed above
- [ ] Skill under 150 lines with ~7 rules + examples
- [ ] No overlap with elixir-essentials or ash-ecto-essentials
- [ ] Covers Phoenix 1.7+ patterns (streams, verified routes)

## Dependencies

- WP-01 (distilled rules inform LiveView patterns)
