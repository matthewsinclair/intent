---
description: "Planning kickoff: document the work, show the workplan, and wait for approval before coding"
chains_to: []
---

# Planning Kickoff

Structure the next piece of work before writing any code. Plans are documented in steel threads before implementation begins.

## Procedure

### 1. Document the work first

ALWAYS create or update steel thread and work package docs BEFORE coding:

- If no ST exists for this work, create one: `intent st new "Title"`
- If no WP exists, create one: `intent wp new <STID> "Title"`
- Write the plan into the record: the thread's objective with `intent set <STID> objective --from <file>`, a WP's with `intent set intent:///threads/<STID>/wp/<NN> objective --from <file>`, or a design document attached with `intent st attach <STID> design.md --from <file>`. `WP/<NN>/info.md` is a generated view; never write into it
- If the thread has an `acceptance.md`, ratify its Acceptance Criteria first -- the open-gate. The verifier owns the boundary; settle it before writing code. `/in-verify` carries the AC/AT steps.

### 2. Show detailed workplan

Present the workplan with:

- What will be built or changed
- Which files will be modified -- search for an existing owner before adding a new one
- What new modules are needed. To check for prior art, ask the index first: `intent search --kind def <name>` answers whether a thing with that name already exists anywhere in the tree, and the answer carries the index's own freshness. When it says the index is not complete for the paths that matter, fall back to grep. Where the project keeps a registry, `intent modules find <name>` searches that as well. Where the project keeps a `intent/llm/MODULES.md` registry, register the new row before creating the file. Most projects have no registry: `intent init` does not create one, and its absence is normal -- which is why the index, not the registry, is the first question.
- Expected test approach

### 3. Plan quality standards

Every plan must meet these standards before presenting to the user:

**No placeholders**: Plans must not contain "TBD", "TODO", "handle edge cases", "implement as needed", or similar deferred language. If you don't know the answer yet, that's a question for the user, not a placeholder.

**Specific file paths**: Every step must name the files it will create or modify. "Update the config" is not a step. "Add pool_size to config/runtime.exs" is.

**Small steps**: Each step should be independently verifiable. If a step is bigger than an XS, split it.

**Verification per step**: Each step includes how to verify it worked (test command, build command, or manual check). A step without verification is not a step.

### 4. Apply the loaded rules

The skills `/in-session` loaded for the project's declared languages are the ones that apply, with the Rules of the Road `AGENTS.md` indexes. Check the plan against them before presenting it.

### 5. Wait for user review

Present the plan and wait for approval before proceeding. Do not start coding until the user confirms.

## Skill Chain

After planning is approved, pick the first work unit from the plan:

- `intent outstanding` -- the outstanding threads, work packages and issues in one table, read from their real status.

## Red Flags

| Rationalization                       | Reality                                               |
| ------------------------------------- | ----------------------------------------------------- |
| "This is simple, no plan needed"      | Simple tasks grow. A plan is an XS.                   |
| "I'll figure it out as I go"          | Ad-hoc coding produces ad-hoc results.                |
| "The user wants speed, skip planning" | Plans prevent rework. Rework is slower than planning. |
| "I already know the codebase"         | Search anyway. Memory drifts.                         |
