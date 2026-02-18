# Worker-Bee Agent User Guide

Complete guide for using the Worker-Bee Intent agent to enforce Worker-Bee Driven Design (WDD) in your Elixir projects.

## Quick Start

### 1. Install the Agent

```bash
intent agents install worker-bee
```

### 2. Initial Project Discovery

Run this command in your Elixir project to trigger the interactive project mapping:

```bash
mix wdd.validate
```

The agent will:

- Scan your project structure
- Ask targeted questions about your layer organization
- Create a `.wdd_project_map.yaml` file with your specific structure
- Remember your choices for future validations

### 3. Daily Development Workflow

```bash
# Generate WDD-compliant components
mix wdd.scaffold component UserService

# Validate architecture compliance
mix wdd.validate --min-score 75.0

# Create specific layer components
mix wdd.scaffold functional PaymentProcessor
mix wdd.scaffold boundary NotificationService
```

## Understanding Worker-Bee Driven Design

### The 6 Layers

Worker-Bee uses a mnemonic: **"Do Fun Things with Big, Loud Worker-Bees"**

1. **Data** - Immutable structures, structs, types
2. **Functions** - Pure business logic with no side effects
3. **Tests** - Behavior-focused testing at all layers
4. **Boundaries** - GenServers, APIs, side effect management
5. **Lifecycles** - OTP supervision, application startup/shutdown
6. **Workers** - Concurrency, background jobs, process pools

### Key Principles

**Functional Core**

- No side effects (no GenServer calls, file I/O, network operations)
- Pure function composition using pipes (`|>`)
- Single-purpose functions with clear responsibilities
- Pattern matching over conditionals
- Railway-Oriented Programming with tagged tuples

**Boundary Layer**

- Separate process machinery from business logic
- Use `with` statements for error composition
- Return `{:ok, result}` or `{:error, reason}`
- Validate input at boundaries, delegate to functional core
- Prefer `GenServer.call` over `cast` for back pressure

## Project Mapping (One-Time Setup)

### When Discovery Happens

The agent **only** conducts discovery when:

- No `.wdd_project_map.yaml` file exists
- You explicitly run `mix wdd.remap`
- You use the `--remap` flag with validation/scaffolding
- The agent detects significant structural changes

### Discovery Questions

The agent will ask about your specific project:

```
What type of Elixir project is this?
[1] Phoenix Web Application
[2] Phoenix API
[3] OTP Application
[4] Library
[5] Nerves/Embedded
[6] Umbrella Project

Where would you like your functional core modules?
Current structure shows: lib/my_app/, lib/my_app_web/
Options:
[1] lib/my_app/core/
[2] lib/my_app/business/
[3] lib/my_app_web/functional_core/
[custom] Enter custom path
```

### Example Project Map

After discovery, you'll have a `.wdd_project_map.yaml`:

```yaml
project_name: "my_app"
project_type: phoenix_web
root_path: "/path/to/project"

wdd_layers:
  data: "lib/my_app/types"
  functions: "lib/my_app/core"
  tests: "test"
  boundaries: "lib/my_app_web"
  lifecycles: "lib/my_app/application.ex"
  workers: "lib/my_app/workers"

naming_conventions:
  module_prefix: "MyApp"
  functional_core_suffix: "Core"
```

## Using the Agent with Claude Code

### Basic Agent Invocation

```
Task(
  description="Validate WDD compliance",
  prompt="Review my functional core modules for purity and suggest improvements",
  subagent_type="worker-bee"
)
```

### Specific Use Cases

**Architecture Review**

```
Task(
  description="WDD architecture review",
  prompt="Analyze my current project structure and suggest WDD layer organization. I have a Phoenix app with contexts in lib/my_app/ and web modules in lib/my_app_web/",
  subagent_type="worker-bee"
)
```

**Code Generation**

```
Task(
  description="Generate WDD component",
  prompt="Create a complete WDD component for user authentication including functional core, boundary layer, and tests",
  subagent_type="worker-bee"
)
```

**Compliance Validation**

```
Task(
  description="Check WDD compliance",
  prompt="Validate this module for functional core purity: [paste your code]. Check for side effects and suggest improvements.",
  subagent_type="worker-bee"
)
```

**Refactoring Guidance**

```
Task(
  description="WDD refactoring advice",
  prompt="I have this GenServer that's doing too much business logic. Help me separate concerns using WDD principles: [paste code]",
  subagent_type="worker-bee"
)
```

## Mix Tasks Reference

### `mix wdd.validate`

Validates your project against WDD principles.

```bash
# Basic validation
mix wdd.validate

# Validate specific layer
mix wdd.validate --layer functions

# Validate single file
mix wdd.validate --file lib/my_app/core/user_service.ex

# Set minimum compliance score
mix wdd.validate --min-score 80.0

# Force re-mapping if needed
mix wdd.validate --remap

# JSON output for CI/CD
mix wdd.validate --output json
```

**Example Output:**

```
🔍 Worker-Bee WDD Validation Report
=====================================

Project: MyApp (phoenix_web)
Overall Compliance: 78.5/100

✅ Data Layer (lib/my_app/types): 95/100
   - Proper struct definitions
   - Good use of defaults

⚠️  Functions Layer (lib/my_app/core): 65/100
   - VIOLATION: GenServer.call found in user_service.ex:42
   - SUGGESTION: Move side effects to boundary layer

❌ Boundaries Layer (lib/my_app_web): 45/100
   - VIOLATION: Business logic in controller
   - SUGGESTION: Extract to functional core
```

### `mix wdd.scaffold`

Generates WDD-compliant code following your project conventions.

```bash
# Generate complete component
mix wdd.scaffold component UserManagement

# Generate specific layers
mix wdd.scaffold functional PaymentProcessor
mix wdd.scaffold boundary EmailService
mix wdd.scaffold data User
mix wdd.scaffold worker BackgroundProcessor
mix wdd.scaffold supervisor TaskSupervisor

# Dry run to preview
mix wdd.scaffold component OrderProcessing --dry-run

# Force overwrite existing files
mix wdd.scaffold functional UserService --force
```

**Generated Structure:**

```
lib/my_app/
├── core/
│   ├── user_management.ex           # Functional core
│   └── user_management/
│       ├── user_validator.ex
│       └── user_transformer.ex
├── types/
│   └── user.ex                     # Data structures
└── boundaries/
    └── user_management_server.ex   # GenServer boundary

test/
├── core/
│   └── user_management_test.exs    # Unit tests
└── boundaries/
    └── user_management_server_test.exs  # Integration tests
```

### `mix wdd.remap`

Updates your project structure mapping.

```bash
# Interactive remapping
mix wdd.remap

# Skip confirmation prompts
mix wdd.remap --force

# Don't create backup
mix wdd.remap --no-backup

# Quiet mode
mix wdd.remap --quiet
```

## Common Workflows

### Starting a New Feature

1. **Plan the Component**

   ```bash
   # Use agent to design the architecture
   Task(
     description="Design WDD component",
     prompt="I need to add user notification functionality. Help me design the WDD layers and structure.",
     subagent_type="worker-bee"
   )
   ```

2. **Generate the Scaffold**

   ```bash
   mix wdd.scaffold component UserNotifications
   ```

3. **Implement Business Logic**
   - Focus on functional core first (pure functions)
   - Add data structures as needed
   - Keep side effects in boundary layer

4. **Validate Compliance**
   ```bash
   mix wdd.validate --layer functions --min-score 85.0
   ```

### Refactoring Existing Code

1. **Assess Current State**

   ```bash
   mix wdd.validate --file lib/my_app/problematic_module.ex
   ```

2. **Get Refactoring Guidance**

   ```
   Task(
     description="WDD refactoring plan",
     prompt="This module violates WDD principles: [paste code]. Provide step-by-step refactoring plan to separate concerns.",
     subagent_type="worker-bee"
   )
   ```

3. **Implement Gradually**
   - Extract pure functions first
   - Move side effects to boundaries
   - Add proper error handling
   - Update tests

4. **Validate Improvements**
   ```bash
   mix wdd.validate --file lib/my_app/refactored_module.ex
   ```

### Code Review Process

1. **Pre-commit Validation**

   ```bash
   mix wdd.validate --min-score 75.0
   ```

2. **Agent-Assisted Review**

   ```
   Task(
     description="WDD code review",
     prompt="Review these changes for WDD compliance: [paste diff or file]. Focus on functional core purity and boundary separation.",
     subagent_type="worker-bee"
   )
   ```

3. **Team Education**
   ```
   Task(
     description="Explain WDD violation",
     prompt="Explain to my team why this code violates WDD principles and how to fix it: [paste code]",
     subagent_type="worker-bee"
   )
   ```

## Framework-Specific Guidance

### Phoenix Applications

**Contexts as Boundaries**

- Phoenix contexts naturally map to WDD boundary layer
- Keep business logic in functional core, not contexts
- Use contexts for API and side effect coordination

**Controllers**

- Thin controllers that delegate to contexts
- Input validation and serialization only
- No business logic in controllers

**LiveView Components**

- UI logic separate from business logic
- Event handlers delegate to contexts
- Pure functions for data transformation

### OTP Applications

**Supervision Trees**

- Map to WDD lifecycle layer
- Keep supervisor logic simple
- Business logic in supervised processes

**GenServers**

- Focus on process management, not business logic
- Delegate complex operations to functional core
- Use `with` statements for error handling

### Libraries

**Pure Functional APIs**

- Emphasize functional core layer
- Minimal or no process machinery
- Clear module organization
- Comprehensive documentation

## Best Practices

### Do's

✅ **Start with Data and Functions**

- Define your data structures first
- Build pure functions that transform data
- Add boundaries only when needed

✅ **Use Agent for Architecture Decisions**

- Consult the agent when designing new components
- Ask for WDD-specific guidance
- Get explanations of violations

✅ **Validate Regularly**

- Run `mix wdd.validate` frequently
- Set compliance score targets
- Address violations early

✅ **Embrace the Discovery Process**

- Answer mapping questions thoughtfully
- Consider your team's conventions
- Update mapping when project evolves

### Don'ts

❌ **Don't Skip Project Mapping**

- Always let the agent understand your structure
- Don't assume default layouts
- Don't ignore re-mapping suggestions

❌ **Don't Mix Concerns**

- Keep business logic out of GenServers
- Avoid side effects in functional core
- Don't put UI logic in business modules

❌ **Don't Ignore Validation Warnings**

- Address compliance violations promptly
- Understand WHY rules exist
- Ask agent for clarification when confused

## Troubleshooting

### Agent Not Finding Project Map

**Problem:** Agent keeps asking for project structure
**Solution:**

```bash
# Check if map file exists
ls -la .wdd_project_map.yaml

# If missing, run discovery
mix wdd.validate

# If corrupted, re-map
mix wdd.remap
```

### Low Compliance Scores

**Problem:** Validation shows low scores
**Solution:**

```bash
# Get detailed feedback
mix wdd.validate --verbose

# Ask agent for specific help
Task(
  description="Fix WDD violations",
  prompt="My compliance score is low. Help me understand and fix these specific violations: [paste validation output]",
  subagent_type="worker-bee"
)
```

### Generated Code Doesn't Match Project

**Problem:** Scaffolded code doesn't follow your patterns
**Solution:**

```bash
# Update project mapping
mix wdd.remap

# Verify layer paths are correct
cat .wdd_project_map.yaml

# Regenerate with updated mapping
mix wdd.scaffold component MyComponent --force
```

### Agent Seems Confused About Project

**Problem:** Agent suggestions don't fit your project type
**Solution:**

```
Task(
  description="Update project understanding",
  prompt="My project structure has changed significantly. It's now a [Phoenix app/OTP app/library] with [describe structure]. Please help me re-map the WDD layers.",
  subagent_type="worker-bee"
)
```

## Advanced Usage

### CI/CD Integration

```bash
# In your CI pipeline
mix wdd.validate --output json --min-score 70.0
if [ $? -ne 0 ]; then
  echo "WDD compliance below threshold"
  exit 1
fi
```

### Team Adoption Strategy

1. **Start with New Code**
   - Use agent for all new components
   - Don't refactor everything at once
   - Set compliance targets gradually

2. **Education Focus**
   - Use agent to explain violations
   - Share WDD principles with team
   - Review generated code together

3. **Gradual Migration**
   - Identify high-impact violations first
   - Refactor incrementally
   - Measure compliance improvement

### Custom Templates

The agent uses EEx templates that can be customized:

- `templates/functional_core.ex.eex`
- `templates/boundary_genserver.ex.eex`
- Add your own templates to match team conventions

## Getting Help

### Agent Assistance

The worker-bee agent is designed to be educational. Always ask for explanations:

```
Task(
  description="Explain WDD concept",
  prompt="I don't understand why [specific pattern] violates WDD principles. Can you explain the reasoning and show me the correct approach?",
  subagent_type="worker-bee"
)
```

### Common Questions

**Q: How do I handle database operations in functional core?**
A: You don't. Database operations are side effects that belong in the boundary layer. Pass data to functional core, return instructions for what to persist.

**Q: Can I use Logger in functional core?**
A: No. Logging is a side effect. Return success/error tuples and let boundary layer handle logging.

**Q: What about configuration access?**
A: Pass configuration as parameters to functional core functions. Don't access Application config directly.

**Q: How do I test GenServer behavior?**
A: Integration tests in boundary layer test the process behavior. Unit tests in functional core test business logic.

Remember: The worker-bee agent is here to help you understand and apply WDD principles. Don't hesitate to ask for clarification, examples, or step-by-step guidance for any WDD concept.
