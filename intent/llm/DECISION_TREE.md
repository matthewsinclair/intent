# Decision Tree - Where Does This Code Belong?

> Use this tree when you're about to write new code for Intent.
> Always cross-reference MODULES.md -- if a module already owns that concern, put the code there.

## Intent Decision Tree

### Step 1: What kind of code is it?

**Is it behaviour -- a computation, a check, a read or write of project state?**

- The owning module under `native/rust/crates/intentsvcs/src/` -- the only crate that touches the store or the file canon
- NOT in `intent-cli`, which parses, calls the facade and renders

**Is it a new top-level command or flag (like `intent foo`)?**

- Add the row to `surface/dispatch-table.json` -- the command surface's single source; its `help` text is the help
- Add the arm in `native/rust/crates/intent-cli/src/render.rs`; a family with no arm refuses at exit 2 as not implemented
- Put the behaviour in `intentsvcs`

**Is it a new subcommand under an existing family (like `intent claude bar`)?**

- Add its row under that family in `surface/dispatch-table.json`, then its arm in `render.rs`

**Is it install/sync/uninstall logic for skills, subagents or project canon?**

- `intentsvcs::payload` (`payload.rs`) -- skills and subagents, one implementation behind both kinds
- `intentsvcs::canon` (`canon.rs`) -- a project's canon files
- DO NOT duplicate install/sync/uninstall logic

**Is it a new skill or subagent definition?**

- Skill: new directory under `intent/plugins/claude/skills/in-<name>/`
- Subagent: new directory `intent/plugins/claude/subagents/<name>/` holding `agent.md`

**Is it a new template?**

- LLM templates: `lib/templates/llm/`
- Project structure: `lib/templates/prj/`

**Is it a test?**

- Rust: a colocated `#[cfg(test)]` module, or an integration test under `native/rust/crates/<crate>/tests/` (shared helpers in `testkit`)
- Shell assets: a BATS file under `tests/unit/`, using `tests/lib/test_helper.bash`

### Step 2: Does a module already own this?

1. Check MODULES.md
2. If yes: add code to that module
3. If no: register in MODULES.md first, then create the file

### Step 3: Where does this rule belong?

**Is the principle language-agnostic?** (Highlander, PFIC, Thin Coordinator, No Silent Errors)

- `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`
- A pattern rule (one governing a code shape) lists at least two `concretised_by:` language-specific rule IDs; a procedural rule (one governing an action, eg `IN-AG-RED-CONTROL-001`) carries none

**Is it Elixir-specific?**

- `intent/plugins/claude/rules/elixir/<category>/<slug>/RULE.md`
- Categories: `code`, `test`, `ash`, `phoenix`, `lv`
- Must include runnable `good_test.exs` + `bad_test.exs` (test rules) or `good.exs` + `bad.exs` (code rules)

**Is it Rust / Swift / Lua / Shell-specific?**

- `intent/plugins/claude/rules/{rust,swift,lua,shell}/<category>/<slug>/RULE.md`
- Textual examples only (per `_schema/CI-LIMITATIONS.md`)

**Is it a prose rule (authoring or web content)?**

- `intent/plugins/claude/rules/{prose,author,content}/<category>/<slug>/RULE.md` -- `prose` is the shared base (`style`); `author` and `content` add `style` and `craft`
- Textual examples only; enforced by the `critic-prose` subagent, not by `intent critic`

**Does the rule only apply to a specific team or project?**

- Not in this build: `~/.intent/ext/` is read by nothing. `intent critic <lang> --rules <dir>` runs the headless critic against a rules root you name instead of canon.
- See `intent/docs/writing-extensions.md`

Always run `intent claude rules validate <id>` before committing a new rule.

### Step 4: Where does this skill belong?

**Is it broadly applicable to Intent users?**

- Canon: `intent/plugins/claude/skills/<slug>/SKILL.md`
- Install via `intent claude skills install <slug>`

**Is it specific to a user, team, or domain workflow?**

- Not in this build: `~/.intent/ext/` is read by nothing, and `intent claude skills install` resolves canon only

### Step 5: Is this a rule, a skill, or a subagent?

**An atomic, cite-able standard with Detection and good/bad examples?**

- Rule. `intent/plugins/claude/rules/<lang>/<category>/<slug>/RULE.md`
- Owned by the rule file. Skills cite it; Critics enforce it.

**A procedural guide loaded on demand (eg "session kick-off", "test writing")?**

- Skill. `intent/plugins/claude/skills/<slug>/SKILL.md`
- Skills can list rule IDs in frontmatter; never restate rule prose.

**A focused worker with its own context window and tool loadout (eg "critic", "test-spec generator")?**

- Subagent. `intent/plugins/claude/subagents/<name>/agent.md` (canon; `~/.intent/ext/` is read by nothing in this build).

If the same prose lives in two of these layers, the duplicate is the bug. The rule file always wins.

### Step 6: Anti-patterns

If you're tempted to...

| Temptation                                 | Correct Location                                        |
| ------------------------------------------ | ------------------------------------------------------- |
| Duplicate a helper function                | The owning `intentsvcs` module (Highlander Rule)        |
| Write install/sync/uninstall from scratch  | `intentsvcs::payload`                                   |
| Inline a literal that exists as a template | Read the template; `rootfiles::substitute` expands it   |
| Put business logic in command dispatch     | `intentsvcs`; `render.rs` parses, calls, renders        |
| Create a new config parser                 | `Project::config()` in `intentsvcs::project`            |
| Hardcode terminal width                    | `terminal_width()` in `intent-cli/src/render.rs`        |
| Add error handling without a remedy        | A typed error implementing `intentsvcs::remedy::Remedy` |
