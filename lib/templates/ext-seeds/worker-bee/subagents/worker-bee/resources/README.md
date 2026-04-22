# Worker-Bee Agent

A specialized Intent agent for enforcing Worker-Bee Driven Design (WDD) principles in Elixir applications.

## Overview

The Worker-Bee agent helps maintain architectural consistency by:

1. **Project Structure Discovery** - Interactive mapping of your project to WDD layers
2. **WDD Compliance Validation** - Automated checking against the 6-layer architecture
3. **Code Scaffolding** - Generation of WDD-compliant modules and components
4. **Educational Guidance** - Contextual explanations of WDD principles

## Features

### Project Structure Mapping

Before any validation or scaffolding, the agent conducts an interactive session to understand your specific project structure:

- Detects project type (Phoenix, OTP, library, etc.)
- Maps existing code to WDD layers
- Creates a persistent project configuration
- Respects your naming conventions and organization preferences

### WDD Layer Architecture

Enforces the 6-layer Worker-Bee Driven Design architecture:

- **Data** - Immutable data structures and types
- **Functions** - Pure business logic without side effects
- **Tests** - Behavior-focused testing at all layers
- **Boundaries** - GenServers, APIs, and side effect management
- **Lifecycles** - OTP supervision and application management
- **Workers** - Concurrency and background processing

### Mix Tasks

#### `mix wdd.validate`

Validates project compliance against WDD principles:

```bash
# Validate entire project
mix wdd.validate

# Validate specific layer
mix wdd.validate --layer functions

# Validate single file
mix wdd.validate --file lib/my_app/core/user_service.ex

# Generate JSON report
mix wdd.validate --output json

# Require minimum compliance score
mix wdd.validate --min-score 80.0
```

#### `mix wdd.scaffold`

Generates WDD-compliant code following your project patterns:

```bash
# Generate functional core module
mix wdd.scaffold functional UserService

# Generate complete WDD component
mix wdd.scaffold component UserManagement

# Generate boundary layer
mix wdd.scaffold boundary PaymentProcessor

# Generate data structure
mix wdd.scaffold data User

# Dry run to preview generation
mix wdd.scaffold component OrderProcessing --dry-run
```

## Installation for Another Project

To use this agent in another project:

1. Copy the entire `intent/agents/worker-bee/` directory to your target project
2. Run `intent agents install worker-bee` in the target project
3. Use the agent via Claude Code's Task tool:

```
Task(
  description="Map project structure",
  prompt="Help me establish WDD layer mapping for my project and validate compliance",
  subagent_type="worker-bee"
)
```

## Validation Rules

### Functional Core Layer

- No side effects (no GenServer calls, file I/O, network operations)
- Pure function composition with pipes
- Single-purpose functions
- Pattern matching over conditionals
- Proper error handling with tagged tuples

### Boundary Layer

- Proper GenServer patterns
- Railway-Oriented Programming with `with` statements
- Input validation at API boundaries
- Clear separation of client API from server implementation
- Delegation to functional core for business logic

### Data Layer

- Immutable data structures
- Proper struct definitions with defaults
- Appropriate data structure choices
- Flat structure over deep nesting

### Testing Layer

- Behavior-focused tests (not implementation)
- Descriptive test names
- Proper test organization with describe blocks
- Specific assertions over generic ones

## Framework Support

Works with any Elixir project type:

- **Phoenix** - Web applications and APIs
- **OTP Applications** - Process-oriented systems
- **Libraries** - Pure functional libraries
- **Nerves** - Embedded systems
- **Umbrella Projects** - Multi-application systems

## Educational Approach

The agent provides:

- Contextual explanations of WDD principles
- Specific recommendations for your codebase
- Incremental improvement suggestions
- Examples from your actual code
- Guidance on gradual refactoring

## Configuration

Project mapping is stored in `.wdd_project_map.yaml`:

```yaml
project_name: "my_app"
project_type: phoenix_web
root_path: "/path/to/project"

wdd_layers:
  data: "lib/my_app/types"
  functions: "lib/my_app_web/functional_core"
  tests: "test"
  boundaries: "lib/my_app_web"
  lifecycles: "lib/my_app/application.ex"
  workers: "lib/my_app/workers"

naming_conventions:
  module_prefix: "MyApp"
  functional_core_suffix: "Core"
```

## Best Practices

1. **Start with Discovery** - Always begin with project structure mapping
2. **Incremental Adoption** - Use WDD principles gradually, don't rewrite everything
3. **Test Behavior** - Focus on what your code does, not how it does it
4. **Keep Core Pure** - No side effects in functional core
5. **Validate Early** - Run `mix wdd.validate` regularly during development

## Examples

### Typical Usage Flow

1. **Initial Setup**

   ```bash
   # Agent discovers your project structure
   mix wdd.validate  # Triggers discovery session
   ```

2. **Generate Components**

   ```bash
   # Create new WDD-compliant component
   mix wdd.scaffold component OrderProcessor
   ```

3. **Validate Compliance**

   ```bash
   # Check compliance regularly
   mix wdd.validate --min-score 75.0
   ```

4. **Iterative Improvement**
   ```bash
   # Focus on specific issues
   mix wdd.validate --layer functions --verbose
   ```

## Troubleshooting

### No Project Map Found

Run `mix wdd.validate` to trigger interactive discovery session.

### Validation Failures

Use `--verbose` flag to see detailed violation information and recommendations.

### Generation Conflicts

Use `--force` flag to overwrite existing files, or `--dry-run` to preview changes.

## Contributing

This agent follows Worker-Bee Driven Design principles in its own implementation:

- Pure validation logic in functional core modules
- GenServer boundaries for state management
- Comprehensive test coverage
- Clear separation of concerns

Generated by Worker-Bee Agent v1.0.0
