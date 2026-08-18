---
verblock: "15 Aug 2025:v0.1: Torrell Ewan - Design specifications for worker-bee agent"
intent_version: 2.2.0
---

# Design - ST0018: Worker-Bee Intent Agent for WDD Architecture Enforcement

## Approach

Implement a comprehensive Intent agent specializing in Worker-Bee Driven Design (WDD) through:

1. **Interactive Discovery Pattern**: One-time project structure mapping with persistent storage
2. **Mix Task Integration**: CLI tools for validation, scaffolding, and remapping
3. **Educational Agent**: Claude Code sub-agent providing contextual WDD guidance
4. **Framework Agnostic**: Support for Phoenix, OTP, libraries, Nerves, umbrella projects

## Design Decisions

### "Discovery Once" Principle

**Decision**: Agent checks for existing `.wdd_project_map.yaml` before conducting discovery
**Rationale**: Minimizes user interruption while maintaining flexibility for project evolution

### Mix Task Architecture

**Decision**: Separate tasks for validate, scaffold, and remap operations
**Rationale**: Clear separation of concerns, composable workflows, familiar Elixir patterns

### EEx Template System

**Decision**: Use Elixir's native EEx templating for code generation
**Rationale**: Leverages existing Elixir tooling, allows customization, maintains consistency

### YAML Project Maps

**Decision**: Store project structure in `.wdd_project_map.yaml` format
**Rationale**: Human-readable, version-controllable, widely supported format

## Architecture

### Agent Layer Structure

```
worker-bee/
├── agent.md                    # Claude Code agent definition
├── metadata.json              # Agent configuration
├── USER_GUIDE.md              # Complete usage documentation
├── lib/                       # Core business logic
│   ├── project_mapper.ex      # Discovery and mapping
│   ├── wdd_validator.ex       # Compliance validation
│   ├── template_generator.ex  # Code scaffolding
│   └── mix/tasks/wdd/         # CLI interface
├── templates/                 # EEx generation templates
├── config/                    # Validation patterns
└── validation/                # WDD compliance rules
```

### WDD 6-Layer Enforcement

1. **Data** - Immutable structures, proper typing
2. **Functions** - Pure business logic, no side effects
3. **Tests** - Behavior-focused, layer-appropriate
4. **Boundaries** - GenServers, APIs, side effect management
5. **Lifecycles** - OTP supervision, application structure
6. **Workers** - Concurrency, background processing

### Validation Engine

- **Pattern-based detection** using configurable rules
- **Scoring system** with layer-specific and overall metrics
- **Smart suggestions** for re-mapping when structure evolves
- **Framework awareness** for context-appropriate validation

## Alternatives Considered

### Alternative 1: Macro-based Code Generation

**Rejected**: Would require compile-time dependency, limiting flexibility
**Chosen**: Mix task with EEx templates for runtime generation

### Alternative 2: Hard-coded Project Structure

**Rejected**: Inflexible for diverse project organizations
**Chosen**: Interactive discovery with persistent mapping

### Alternative 3: Single Validation Command

**Rejected**: Would create overly complex interface
**Chosen**: Separate tasks for validate, scaffold, remap operations

### Alternative 4: JSON Project Configuration

**Rejected**: Less human-readable than YAML
**Chosen**: YAML for better developer experience

### Alternative 5: Framework-specific Agents

**Rejected**: Would fragment WDD knowledge across multiple agents
**Chosen**: Single agent with framework awareness and detection
