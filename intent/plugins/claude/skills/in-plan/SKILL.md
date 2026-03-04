---
description: "Planning kickoff: show workplan, invoke coding skills, enforce rules before coding"
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

### 3. Invoke relevant coding skills

Load the appropriate enforcement skills for the work:

- `/in-essentials` -- always
- `/in-elixir-essentials` -- for Elixir code
- `/in-ash-ecto-essentials` -- for Ash/Ecto code
- `/in-phoenix-liveview` -- for LiveView code
- `/in-elixir-testing` -- for test code

### 4. Enforce project rules

These rules apply to ALL languages (Elixir, Rust, Swift, Lua):

- **Highlander Rule**: No duplicated code paths. Check MODULES.md.
- **Thin Coordinators**: Controllers, LiveViews, CLI commands are thin. Business logic in services.
- **PFIC**: Pure-Functional Idiomatic Code. Prefer pure functions, tagged tuples, pattern matching.

### 5. Wait for user review

Present the plan and wait for approval before proceeding. Do not start coding until the user confirms.
