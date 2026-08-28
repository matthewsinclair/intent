## Operational Knowledge

### Intent Commands

- `intent st new "Title"` / `intent st list` / `intent st show <id>` -- steel thread management
- `intent st start <id>` / `intent st done <id>` -- status transitions
- `intent wp new <STID> "Title"` / `intent wp list <STID>` / `intent wp show <STID/NN>` -- work packages
- `intent wp start <STID/NN>` / `intent wp done <STID/NN>` -- WP status transitions
- Specifiers accept bare numbers: `5` = `ST0005`, `5/01` = `ST0005/WP/01`
- `intent claude skills list` / `install` / `sync` / `show` -- skill management
- `intent claude subagents list` / `install` / `sync` -- subagent management
- `intent claude prime [--refresh] [--dry-run]` -- refresh this memory file
- `intent plugin list` -- discover available plugins
- `intent doctor` -- check dependencies and configuration

### Skills (invoke via /skill-name)

- `/in-start` -- session orientation (read-only, no coding)
- `/in-plan` -- planning kickoff (document before coding)
- `/in-standards` -- load coding discipline into context
- `/in-next` -- identify next work unit
- `/in-finish` -- end-of-session wrap-up (docs only, no code)
- `/in-essentials` -- Intent workflow rules (always active)
- `/in-elixir-essentials` -- Elixir coding rules
- `/in-ash-ecto-essentials` -- Ash/Ecto database rules
- `/in-phoenix-liveview` -- LiveView lifecycle rules
- `/in-elixir-testing` -- test quality rules
- `/in-autopsy` -- session forensics

### Session Workflow

1. Start: run `/in-session` -- the entry point, enforced by the `UserPromptSubmit` gate. Run it again after every `/compact` or context reset.
2. Plan: run `/in-plan` before writing code
3. Code: follow the rules; search for an existing owner before adding one
4. Finish: run `/in-finish` to capture state

### Conventions

- No Claude attribution in commits (no Co-Authored-By, no AI signatures)
- All markdown tables must be column-aligned
- Steel thread docs updated BEFORE coding begins
- Search for an existing owner before creating any new module. Where the project keeps an `intent/llm/MODULES.md` registry, search it with `intent modules find <name>` and register the row before creating the file. `intent init` does not create a registry, so most projects have none and that is normal.
