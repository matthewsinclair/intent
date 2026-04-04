---
verblock: "04 Apr 2026:v0.1: matts - Initial version"
intent_version: 2.8.0
status: WIP
slug: cherry-pick-superpowers-skills-and-bring-them
created: 20260404
completed:
---

# ST0030: Cherry-Pick Superpowers Patterns for Intent

## Objective

Extract the highest-value development methodology patterns from the [Superpowers](https://github.com/obra/superpowers) Claude Code plugin (v5.0.7, by Jesse Vincent) and build Intent-native equivalents that integrate with steel threads and the existing skill system. Do NOT install Superpowers wholesale; cherry-pick only what is additive.

## Context

A design spike (plan file: `temporal-wishing-panda.md`) analyzed the Superpowers plugin against Intent's skill system. Key findings:

- **Complementary layers**: Superpowers provides ecosystem-agnostic development methodology (TDD, systematic debugging, structured planning, code review). Intent provides domain expertise (Elixir/Ash/Phoenix), project management (steel threads), and session lifecycle.
- **Conflicts**: Superpowers' auto-activation and TDD absolutism clash with Intent's explicit invocation model and Ash's declarative patterns.
- **Cherry-pick targets**: Three high-priority patterns and three medium-priority patterns identified as additive to Intent without introducing conflicts.

### High-Priority Patterns

1. **Verification-before-completion** -- "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE." Catches Claude claiming "tests pass" without running them.
2. **Rationalization tables / anti-pattern docs** -- Every Superpowers skill includes adversarial "Red Flags" and "Common Rationalizations" tables that anticipate how the model will skip requirements.
3. **Skill dependency chains** -- `depends_on:` / `chains_to:` frontmatter fields to create a navigable workflow graph without auto-activation.

### Medium-Priority Patterns

4. **Plan granularity standards** -- No placeholders, 2-5 minute steps, complete code blocks, specific file paths.
5. **Systematic debugging with 3-strike rule** -- After 3 failed fixes, stop and question the architecture.
6. **Two-stage code review** -- Spec compliance review followed by code quality review, chaining existing agents.

## Related Steel Threads

- ST0026: Highlander Audit (established callback architecture, skill system)
- ST0028: TCA v3.0 (most recent skill suite creation)
- ST0025: Highlander violations (shared helpers pattern)

## Context for LLM

This ST has 6 work packages organized by priority. WP-01 through WP-03 are high-priority and can be done independently. WP-04 through WP-06 are medium-priority and depend on WP-01 (for frontmatter changes). The rationalization tables in WP-02 should be studied carefully -- they are the most novel pattern from Superpowers and the one most likely to improve existing Intent skills.
