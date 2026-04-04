---
description: "Planning kickoff: show workplan, invoke coding skills, enforce rules before coding"
chains_to: ["in-next"]
---

# Planning Kickoff

Structure the next piece of work before writing any code. Plans are documented in steel threads before implementation begins.

## Procedure

### 1. Document the work first

ALWAYS create or update steel thread and work package docs BEFORE coding:

- If no ST exists for this work, create one: `intent st new "Title"`
- If no WP exists, create one: `intent wp new <STID> "Title"`
- Write the plan in the ST's `design.md` or WP's `info.md`

### 2. Show detailed workplan

Present the workplan with:

- What will be built or changed
- Which files will be modified (check MODULES.md first)
- What new modules are needed (register in MODULES.md first)
- Expected test approach

### 3. Plan quality standards

Every plan must meet these standards before presenting to the user:

**No placeholders**: Plans must not contain "TBD", "TODO", "handle edge cases", "implement as needed", or similar deferred language. If you don't know the answer yet, that's a question for the user, not a placeholder.

**Specific file paths**: Every step must name the files it will create or modify. "Update the config" is not a step. "Add pool_size to config/runtime.exs" is.

**Small steps**: Each step should be independently verifiable. If a step modifies more than 3 files, split it. If a step takes more than a few minutes of coding, split it.

**Verification per step**: Each step includes how to verify it worked (test command, build command, or manual check). A step without verification is not a step.

### 4. Invoke relevant coding skills

Load the appropriate enforcement skills for the work:

- `/in-essentials` -- always
- `/in-elixir-essentials` -- for Elixir code
- `/in-ash-ecto-essentials` -- for Ash/Ecto code
- `/in-phoenix-liveview` -- for LiveView code
- `/in-elixir-testing` -- for test code

### 5. Enforce project rules

These rules apply to ALL languages (Elixir, Rust, Swift, Lua):

- **Highlander Rule**: No duplicated code paths. Check MODULES.md.
- **Thin Coordinators**: Controllers, LiveViews, CLI commands are thin. Business logic in services.
- **PFIC**: Pure-Functional Idiomatic Code. Prefer pure functions, tagged tuples, pattern matching.

### 6. Wait for user review

Present the plan and wait for approval before proceeding. Do not start coding until the user confirms.

## Skill Chain

After planning is approved, consider:

- `/in-next` -- pick the first work unit from the plan

## Red Flags

| Rationalization                       | Reality                                               |
| ------------------------------------- | ----------------------------------------------------- |
| "This is simple, no plan needed"      | Simple tasks grow. The plan takes 2 minutes.          |
| "I'll figure it out as I go"          | Ad-hoc coding produces ad-hoc results.                |
| "The user wants speed, skip planning" | Plans prevent rework. Rework is slower than planning. |
| "I already know the codebase"         | Check MODULES.md anyway. Memory drifts.               |
