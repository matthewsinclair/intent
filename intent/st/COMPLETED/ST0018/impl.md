---
verblock: "15 Aug 2025:v0.1: Torrell Ewan - Implementation details for worker-bee agent"
intent_version: 2.2.0
---
# Implementation - ST0018: Worker-Bee Intent Agent for WDD Architecture Enforcement

## Implementation

The worker-bee agent was implemented as a comprehensive WDD specialist with three main components:

### 1. Claude Code Agent Integration
- **Agent Definition**: `agents/worker-bee/agent.md` with comprehensive system prompt
- **Metadata Configuration**: `agents/worker-bee/metadata.json` with tool specifications
- **Installation**: Integrates with Intent's agent management system via `intent agents install worker-bee`

### 2. Mix Task CLI Interface
Three dedicated Mix tasks provide command-line functionality:
- `mix wdd.validate` - Compliance validation with scoring and detailed feedback
- `mix wdd.scaffold` - Code generation following project conventions
- `mix wdd.remap` - Project structure remapping with backup functionality

### 3. Supporting Infrastructure
- **Business Logic Modules**: ProjectMapper, WDDValidator, TemplateGenerator
- **EEx Templates**: Code generation templates for all WDD component types
- **Validation Rules**: Pattern-based compliance checking
- **Configuration**: YAML-based validation patterns and project mapping

## Code Examples

### Agent System Prompt Structure
```markdown
---
name: worker-bee
description: Worker-Bee Driven Design specialist for Elixir applications
tools: Bash, Read, Write, Edit, Grep, Glob, LS
---

You are a Worker-Bee Driven Design (WDD) specialist...

**FIRST CHECK**: Always verify if a WDD project map already exists before conducting discovery.
```

### Project Mapping Discovery
```elixir
defmodule WorkerBee.ProjectMapper do
  def discover_project_structure(project_path) do
    with {:ok, project_type} <- detect_project_type(project_path),
         {:ok, existing_structure} <- scan_directory_structure(project_path),
         {:ok, user_preferences} <- conduct_interactive_discovery(project_type, existing_structure),
         {:ok, project_map} <- generate_project_map(user_preferences) do
      {:ok, project_map}
    end
  end
end
```

### Mix Task Implementation Pattern
```elixir
defmodule Mix.Tasks.Wdd.Validate do
  use Mix.Task
  
  def run(args) do
    {opts, _} = OptionParser.parse!(args, switches: @switches)
    
    with {:ok, project_map} <- load_or_discover_project_map(opts),
         {:ok, validation_results} <- validate_project(project_map, opts) do
      display_results(validation_results, opts)
    end
  end
end
```

### Template Generation System
```elixir
# EEx template for functional core
defmodule <%= module_name %> do
  @moduledoc """
  Functional core for <%= description %>.
  
  This module contains pure business logic with no side effects.
  All functions are composable and return tagged tuples.
  """
  
  def process_<%= function_name %>(data) do
    data
    |> validate_input()
    |> transform_data()
    |> format_result()
  end
  
  defp validate_input(data) do
    # Pure validation logic
  end
end
```

## Technical Details

### Project Map Structure
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

### Validation Engine Architecture
- **Pattern-based Detection**: Uses regex patterns to identify WDD violations
- **Scoring Algorithm**: Layer-specific scores aggregated into overall project score
- **Rule Categories**: Functional core purity, boundary patterns, data structures, testing
- **Framework Awareness**: Different validation rules for Phoenix, OTP, libraries

### File Organization
```
agents/worker-bee/
├── agent.md                           # Claude Code agent definition
├── metadata.json                      # Agent configuration
├── USER_GUIDE.md                      # Complete usage documentation  
├── README.md                          # Project overview
├── lib/
│   ├── project_mapper.ex              # Interactive discovery
│   ├── wdd_validator.ex               # Compliance validation
│   ├── template_generator.ex          # Code scaffolding
│   └── mix/tasks/wdd/
│       ├── validate.ex                # Validation CLI
│       ├── scaffold.ex                # Generation CLI
│       └── remap.ex                   # Remapping CLI
├── templates/
│   ├── functional_core.ex.eex         # Pure function templates
│   ├── boundary_genserver.ex.eex      # GenServer templates
│   └── [other component templates]
├── config/
│   └── wdd_patterns.yaml              # Validation patterns
└── validation/
    ├── functional_core_rules.ex       # Purity validation
    ├── boundary_rules.ex              # GenServer patterns
    ├── data_rules.ex                  # Structure validation
    └── testing_rules.ex               # Test organization
```

## Challenges & Solutions

### Challenge 1: "Discovery Once" Implementation
**Problem**: Agent needed to remember project structure without being intrusive
**Solution**: Implemented persistent `.wdd_project_map.yaml` with intelligent re-mapping detection

### Challenge 2: Framework Agnostic Design
**Problem**: Different Elixir project types have vastly different structures
**Solution**: Interactive discovery process that adapts to any project organization

### Challenge 3: Educational vs. Prescriptive Balance
**Problem**: Agent needed to teach WDD principles while being practical
**Solution**: Contextual explanations in every response, gradual adoption guidance

### Challenge 4: Mix Task Integration Complexity
**Problem**: Rich CLI functionality while maintaining simplicity
**Solution**: Separate tasks with shared business logic modules, consistent flag patterns

### Challenge 5: Code Generation Flexibility
**Problem**: Generated code needed to match project conventions
**Solution**: EEx templating system using project map data for customization

### Challenge 6: Validation Engine Performance
**Problem**: Large codebases could make validation slow
**Solution**: Targeted validation using project map, parallel processing where possible

### Challenge 7: Intent Agent System Integration
**Problem**: Ensuring agent follows Intent's agent patterns and conventions
**Solution**: Followed established agent structure from existing intent/elixir agents

## Key Implementation Insights

### "Discovery Once" Principle Success
The persistent project mapping approach proved essential for user experience. Users appreciate that the agent remembers their project structure and doesn't repeat discovery unless explicitly requested or when significant changes are detected.

### Framework Detection Intelligence
Automatic project type detection combined with interactive confirmation creates the right balance of automation and user control. The agent can intelligently suggest appropriate WDD layer organization while respecting user preferences.

### Educational Agent Pattern
The system prompt emphasizes explanation and context rather than just prescriptive rules. This creates a teaching agent that helps developers understand WDD principles rather than just enforcing them blindly.

### Mix Task Composability
Separate tasks for validate, scaffold, and remap operations allow for flexible workflows while sharing common business logic. Users can compose these tasks into their development processes naturally.

## Files Created

**Total**: 19 files across agent definition, business logic, templates, and documentation
**Core Agent**: agent.md (212 lines), metadata.json (27 lines)
**Documentation**: USER_GUIDE.md (563 lines), README.md (222 lines)
**Business Logic**: 8 Elixir modules with comprehensive functionality
**Templates**: EEx templates for all WDD component types
**Configuration**: YAML validation patterns and rules

This implementation provides a comprehensive foundation for WDD architecture enforcement while maintaining flexibility and educational value.