# Decision Tree - Where Does This Code Belong?

> Use this tree when you're about to write new code for Intent.
> Always cross-reference MODULES.md -- if a module already owns that concern, put the code there.

## Intent (Bash CLI) Decision Tree

### Step 1: What kind of code is it?

**Is it a reusable utility function (error handling, checksums, config access)?**

- `bin/intent_helpers` -- the ONE place for shared helpers
- NOT duplicated across scripts

**Is it a new top-level command (like `intent foo`)?**

- New script at `bin/intent_foo`
- Register in `bin/intent` dispatch (GLOBAL_COMMANDS or project commands)
- Add help file at `lib/help/foo.help.md`

**Is it a new subcommand under an existing plugin (like `intent claude bar`)?**

- Add to the existing plugin script
- Update the plugin's `plugin.json` commands array
- Update the plugin's help file

**Is it install/sync/uninstall logic for a plugin?**

- Use the shared callback library: `intent/plugins/claude/lib/claude_plugin_helpers.sh`
- Define the 8 callbacks, source the library
- DO NOT duplicate install/sync/uninstall logic

**Is it a new skill or subagent definition?**

- Skill: new directory under `intent/plugins/claude/skills/in-<name>/`
- Subagent: new `.md` file under `intent/plugins/claude/subagents/`

**Is it a new template?**

- LLM templates: `lib/templates/llm/`
- Code archetypes: `lib/templates/archetypes/<language>/`
- Project structure: `lib/templates/prj/`

**Is it a test?**

- New BATS file under `tests/unit/`
- Use helpers from `tests/lib/test_helper.bash`

### Step 2: Does a module already own this?

1. Check MODULES.md
2. If yes: add code to that module
3. If no: register in MODULES.md first, then create the file

### Step 3: Where does this rule belong?

**Is the principle language-agnostic?** (Highlander, PFIC, Thin Coordinator, No Silent Errors)

- `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`
- Must list at least two `concretised_by:` language-specific rule IDs

**Is it Elixir-specific?**

- `intent/plugins/claude/rules/elixir/<category>/<slug>/RULE.md`
- Categories: `code`, `test`, `ash`, `phoenix`, `lv`
- Must include runnable `good_test.exs` + `bad_test.exs` (test rules) or `good.exs` + `bad.exs` (code rules)

**Is it Rust / Swift / Lua / Shell-specific?**

- `intent/plugins/claude/rules/{rust,swift,lua,shell}/<category>/<slug>/RULE.md`
- Textual examples only (per `_schema/CI-LIMITATIONS.md`)

**Does the rule only apply to a specific team or project?**

- User extension: `~/.intent/ext/<ext-name>/rules/<lang>/<category>/<slug>/RULE.md`
- See `intent/docs/writing-extensions.md`

Always run `intent claude rules validate <id>` before committing a new rule.

### Step 4: Where does this skill belong?

**Is it broadly applicable to Intent users?**

- Canon: `intent/plugins/claude/skills/<slug>/SKILL.md`
- Install via `intent claude skills install <slug>`

**Is it specific to a user, team, or domain workflow?**

- User extension: `~/.intent/ext/<ext-name>/skills/<slug>/SKILL.md`
- Discovered transparently alongside canon skills; shadow warning if name collides

### Step 5: Is this a rule, a skill, or a subagent?

**An atomic, cite-able standard with Detection and good/bad examples?**

- Rule. `intent/plugins/claude/rules/<lang>/<category>/<slug>/RULE.md`
- Owned by the rule file. Skills cite it; Critics enforce it.

**A procedural guide loaded on demand (e.g. "session kick-off", "test writing")?**

- Skill. `intent/plugins/claude/skills/<slug>/SKILL.md`
- Skills can list rule IDs in frontmatter; never restate rule prose.

**A focused worker with its own context window and tool loadout (e.g. "critic", "test-spec generator")?**

- Subagent. `intent/plugins/claude/subagents/<name>/agent.md` (canon) or `~/.intent/ext/<name>/subagents/<name>/agent.md` (extension).

If the same prose lives in two of these layers, the duplicate is the bug. The rule file always wins.

### Step 6: Anti-patterns

If you're tempted to...

| Temptation                                    | Correct Location                            |
| --------------------------------------------- | ------------------------------------------- |
| Duplicate a helper function in a new script   | `bin/intent_helpers` (Highlander Rule)      |
| Write install/sync/uninstall from scratch     | Plugin callback library                     |
| Inline a heredoc that exists as a template    | Use sed substitution on the template file   |
| Put business logic in command dispatch        | Dedicated script sourced by `bin/intent`    |
| Create a new config parser                    | `get_config_field()` in intent_helpers      |
| Hardcode terminal width                       | `detect_terminal_width()` in intent_helpers |
| Add error handling without `error()` function | Use `error()` from intent_helpers           |
