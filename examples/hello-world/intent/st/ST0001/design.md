---
verblock: "01 Jul 2025:v0.1: Intent User - Initial version"
intent_version: 2.0.0
---
# ST0001: Design Document

## Overview

Design for the hello-world project demonstrating Intent v2.0.0 structure.

## Key Design Decisions

### 1. JSON Configuration
- Moved from YAML to JSON for configuration
- No external dependencies needed (can parse with sed/grep)
- Cleaner, more standard format

### 2. Flattened Structure
```
Old: stp/prj/st/
New: intent/st/

Old: stp/eng/
New: intent/eng/

Old: stp/usr/
New: intent/ref/
```

### 3. Tool Separation
```
Tool components:
- bin/         (executables)
- lib/         (templates, resources)

Project artifacts:
- intent/      (steel threads, engineering docs)
- backlog/     (task management)
```

## Benefits

1. **Clarity**: Clear separation between tool and usage
2. **Simplicity**: Flattened structure is easier to navigate
3. **Deployment**: Can deploy just bin/ and lib/ for the tool
4. **Flexibility**: Projects can customize intent_dir and backlog_dir