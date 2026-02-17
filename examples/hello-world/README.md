# Hello World - Intent v2.4.0 Example

This is an example project demonstrating the Intent v2.4.0 structure.

## Project Structure

```
hello-world/
├── .intent/
│   └── config.json      # JSON configuration (new in v2.0.0)
├── intent/              # Flattened structure (was stp/)
│   ├── .treeindex/      # Shadow directory for LLM-oriented summaries
│   ├── st/              # Steel threads (was stp/prj/st/)
│   │   ├── ST0001/
│   │   └── ST0002/
│   ├── eng/             # Engineering docs
│   ├── usr/             # User documentation
│   ├── llm/             # LLM context (includes AGENTS.md)
│   └── plugins/         # Plugin architecture
│       ├── agents/      # AGENTS.md plugin
│       └── claude/      # Claude subagents
└── backlog/             # Task management
```

## Key Differences from v1.x

1. **JSON Config**: Uses `.intent/config.json` instead of YAML
2. **Flattened Paths**: `intent/st/` instead of `stp/prj/st/`
3. **Tool Separation**: Executables in `bin/`, templates in `lib/`
4. **Plugin Architecture**: Subagents live in `intent/plugins/claude/subagents/`
5. **Treeindex**: LLM-oriented directory summaries in `intent/.treeindex/`
6. **AGENTS.md**: Universal AI agent instructions in `intent/llm/AGENTS.md`
7. **Skills**: Always-on Claude Code enforcement in `.claude/skills/`
8. **Three-file system**: AGENTS.md + RULES.md + ARCHITECTURE.md in `intent/llm/`

## Configuration

The `.intent/config.json` file:
```json
{
  "intent_version": "2.4.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog",
  "author": "Intent User",
  "editor": "vim"
}
```

## Usage

After installing Intent v2.4.0:

```bash
# Initialize a new project
intent init

# Initialize AGENTS.md for AI assistants
intent agents init

# Initialize Claude subagent configuration
intent claude subagents init

# Create a steel thread
intent st new "My Feature"

# Install Claude Code skills (Elixir projects)
intent claude skills install --all

# Generate directory summaries for LLM orientation
intent treeindex lib

# Check status
intent status

# Run doctor for diagnostics
intent doctor
```
