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

### Step 3: Anti-patterns

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
