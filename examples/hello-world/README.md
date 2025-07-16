# Hello World - Intent v2.0.0 Example

This is an example project demonstrating the Intent v2.0.0 structure.

## Project Structure

```
hello-world/
├── .intent/
│   └── config.json      # JSON configuration (new in v2.0.0)
├── intent/              # Flattened structure (was stp/)
│   ├── st/              # Steel threads (was stp/prj/st/)
│   │   ├── ST0001/
│   │   └── ST0002/
│   ├── eng/             # Engineering docs
│   ├── ref/             # Reference docs (was usr/)
│   ├── llm/             # LLM context
│   └── _archive/        # Archived content
└── backlog/             # Task management
```

## Key Differences from v1.x

1. **JSON Config**: Uses `.intent/config.json` instead of YAML
2. **Flattened Paths**: `intent/st/` instead of `stp/prj/st/`
3. **Tool Separation**: Executables in `bin/`, templates in `lib/`
4. **Renamed Directories**: `usr/` → `ref/`

## Configuration

The `.intent/config.json` file:
```json
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog",
  "author": "Intent User",
  "editor": "vim"
}
```

## Usage

After installing Intent v2.0.0:

```bash
# Initialize a new project
intent init

# Create a steel thread
intent st new "My Feature"

# Check status
intent status

# Run doctor for diagnostics
intent doctor
```