---
description: "Two-stage code review: spec compliance then rule-library compliance via critic-<lang>"
chains_to: ["in-verify"]
---

# Two-Stage Code Review

Review code in two distinct passes. Do not combine them — each pass has a different focus.

## When to invoke

- After completing a work package or significant task
- Before marking a steel thread as done
- When the user asks for a code review

## Procedure

### Stage 1: Spec compliance

Does the implementation match the plan?

Checklist:

- [ ] All deliverables from the WP/ST `info.md` are implemented
- [ ] No extra features added beyond what was planned
- [ ] File paths match what was specified in the plan
- [ ] Edge cases identified in the design are handled
- [ ] No "TODO" or "FIXME" left unresolved from the plan

**For Elixir projects with test specifications**: delegate to the `diogenes` agent for test-spec validation. Provide the design doc and the implementation; ask diogenes to verify alignment between spec and tests.

### Stage 2: Rule-library compliance

Is the code consistent with the Intent rule library? This stage dispatches to the right `critic-<lang>` subagent based on the project's ecosystem.

#### Language detection

Detect the project's primary language by filesystem probe, in this order:

1. **Elixir** — `mix.exs` exists at project root → critic-elixir.
2. **Rust** — `Cargo.toml` exists at project root → critic-rust (WP07).
3. **Swift** — `Package.swift` exists at project root → critic-swift (WP07).
4. **Lua** — `.luarc.json` exists OR the project is `.lua`-dominated → critic-lua (WP07).
5. **Shell** — `bin/` or `scripts/` contains bash/zsh scripts with shebangs → critic-shell (WP12).

A mixed project dispatches to the critic whose language matches the files being reviewed.

#### Generic rule checklist (applies to every language)

Confirm no concretised-by rule is violated at the agnostic level:

- [ ] `IN-AG-HIGHLANDER-001` — no duplicated code paths (check MODULES.md)
- [ ] `IN-AG-THIN-COORD-001` — coordinators parse → call → render
- [ ] `IN-AG-PFIC-001` — pattern-match, pipe, tagged-tuple, compose idioms in play
- [ ] `IN-AG-NO-SILENT-001` — no rescue-and-swallow, no discarded fallible results

#### Delegate to `critic-<lang>` (WP07; for v2.9.0 use the rule pack directly)

Until WP07 ships the critic subagents, perform Stage 2 manually by reading the rules in the language pack:

- **Elixir** → `intent/plugins/claude/rules/elixir/{code,test,ash,phoenix,lv}/` — 19 rules
- **Rust/Swift/Lua** → delivered in WP06
- **Shell** → delivered in WP12

When WP07 lands, Stage 2 becomes:

```
Task(subagent_type="critic-elixir", prompt="review lib/**/*.ex test/**/*.exs")
```

### Stage 3: After both stages

- Fix critical issues before proceeding
- Log non-critical issues as TODOs in `tasks.md`
- Invoke `/in-verify` to confirm fixes

## Red Flags

| Rationalisation                           | Reality                                                |
| ----------------------------------------- | ------------------------------------------------------ |
| "The code works, review is unnecessary."  | Working code can still be wrong. Review catches drift. |
| "I wrote it, I know it's correct."        | Author blindness is real. Review with fresh eyes.      |
| "Review will slow us down."               | Rework from missed issues is slower than review.       |
| "Stage 1 is enough, skip quality review." | Correct-but-messy code becomes tomorrow's bug.         |
