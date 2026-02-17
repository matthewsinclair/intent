---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-05
title: Expand Testing Reference
scope: Medium
status: Not Started
---

# WP-05: Expand Testing Reference

## Objective

Create a comprehensive testing reference doc covering all major Elixir testing patterns that LLMs commonly get wrong.

## Deliverables

- `intent/plugins/claude/subagents/elixir/testing.md` (~200-400 lines)

## Coverage

- **DataCase** — Ecto sandbox, async tests, factory patterns
- **ConnCase** — Controller tests, auth setup, JSON API testing
- **LiveView testing** — render_component, live, follow redirect, form events
- **Mox** — Behaviour-based mocking, expectations, stubs, verify_on_exit
- **Ash testing** — Domain action tests, policy tests, Ash.Generator, globally unique values
- **Changeset validation testing** — Error assertion patterns
- **Async test considerations** — Sandbox ownership, concurrent tests
- **File upload testing** — LiveView upload patterns

## Acceptance Criteria

- [ ] Covers all 8 areas listed above
- [ ] Each section has concrete code examples
- [ ] Follows existing style.md test conventions (success:/failure: prefixes)
- [ ] References Ash testing patterns from agent.md
- [ ] 200-400 lines

## Dependencies

- WP-03 (testing references Ecto/Ash patterns)
- WP-04 (testing references LiveView patterns)
