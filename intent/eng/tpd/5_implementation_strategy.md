---
verblock: "20 Feb 2026:v2.4.0: Matthew Sinclair - Updated for Intent v2.4.0"
intent_version: 2.4.0
---

# 5. Implementation Strategy

[index](./technical_product_design.md)

## 5.1 Development Approach

The Steel Thread Process (STP) will be developed using its own methodology - we will use steel threads to build the STP system itself. This meta-approach allows us to validate the process while creating it.

The implementation will proceed in phases:

1. **Foundation Phase**: Create core directory structure, basic templates, and essential scripts
2. **Functionality Phase**: Implement all command-line tools and complete templates
3. **Documentation Phase**: Create comprehensive documentation and guides
4. **Testing Phase**: Test in various environments and with different projects
5. **Refinement Phase**: Address feedback and optimize the system

## 5.2 Steel Threads

The STP system will be implemented through the following steel threads:

| ID                             | Title                  | Description                                                  |
| ------------------------------ | ---------------------- | ------------------------------------------------------------ |
| [ST0009](../../prj/st/ST0009/) | Process Refinement     | Refine overall process based on experience                   |
| [ST0008](../../prj/st/ST0008/) | LLM Integration        | Create LLM preamble and canned prompts                       |
| [ST0007](../../prj/st/ST0007/) | User Documentation     | Create user, reference, and deployment guides                |
| [ST0006](../../prj/st/ST0006/) | Help System            | Implement the help documentation system                      |
| [ST0005](../../prj/st/ST0005/) | Initialization Command | Implement project initialization                             |
| [ST0004](../../prj/st/ST0004/) | Steel Thread Commands  | Implement commands for steel thread management               |
| [ST0003](../../prj/st/ST0003/) | Template System        | Create all document templates                                |
| [ST0002](../../prj/st/ST0002/) | Core Script Framework  | Implement the main `stp` script and command dispatching      |
| [ST0001](../../prj/st/ST0001/) | Directory Structure    | Create the initial directory structure and placeholder files |

## 5.3 Task Breakdown

### ST0001: Directory Structure

[ST0001](../../prj/st/ST0001/)

- Create root level directories
- Create subdirectories for each component
- Create placeholder files for templates

### ST0002: Core Script Framework

[ST0002](../../prj/st/ST0002/)

- Implement main `stp` script with command dispatching
- Implement environment variable handling
- Implement error handling framework
- Create script templates

### ST0003: Template System

[ST0003](../../prj/st/ST0003/)

- Create templates for project documents
  - Work in progress template
  - Journal template
  - Steel thread templates
- Create templates for engineering documents
  - Technical product design templates
- Create templates for user documents
  - User guide template
  - Reference guide template
  - Deployment guide template
- Create templates for LLM documents
  - LLM preamble template

### ST0004: Steel Thread Commands

[ST0004](../../prj/st/ST0004/)

- Implement `stp st new` command
- Implement `stp st done` command
- Implement `stp st list` command
- Implement steel thread status tracking

### ST0005: Initialization Command

[ST0005](../../prj/st/ST0005/)

- Implement `stp init` command
- Implement template copying
- Implement directory creation
- Implement configuration initialization

### ST0006: Help System

[ST0006](../../prj/st/ST0006/)

- Create help documentation structure
- Implement `stp help` command
- Create help content for all commands

### ST0007: User Documentation

[ST0007](../../prj/st/ST0007.md)

- Create user guide content
- Create reference guide content
- Create deployment guide content

### ST0008: LLM Integration

[ST0008](../../prj/st/ST0008.md)

- Create LLM preamble content
- Create canned prompts for common tasks
- Implement prompt management

### ST0009: Process Refinement

[ST0009](../../prj/st/ST0009.md)

- Review and refine overall process
- Address feedback from earlier stages
- Optimize workflows

## 5.4 Dependencies

The implementation dependencies are as follows:

```
ST0001 ──► ST0002 ──► ST0004 ──► ST0007
   │           │         │          │
   │           │         │          ▼
   │           │         │       ST0009
   │           │         ▼
   │           │      ST0005
   │           │
   │           ▼
   │        ST0006
   │
   ▼
ST0003 ────────────────► ST0008
```

## 5.5 Timeline

| Phase         | Steel Threads          | Timeline |
| ------------- | ---------------------- | -------- |
| Foundation    | ST0001, ST0002, ST0003 | Week 1   |
| Functionality | ST0004, ST0005, ST0006 | Week 2   |
| Documentation | ST0007, ST0008         | Week 3   |
| Refinement    | ST0009                 | Week 4   |

## 5.6 Environment Setup

The development environment requires:

- POSIX-compliant shell (bash, zsh)
- Git for version control
- Text editor with markdown support
- LLM access for assistance (eg Claude Code)

## 5.7 Testing Strategy

Testing will include:

1. **Unit Testing**: Manual testing of individual commands
2. **Integration Testing**: Testing workflows with multiple commands
3. **Environment Testing**: Testing in different shell environments
4. **Project Testing**: Testing with sample projects
5. **LLM Testing**: Testing interaction with different LLMs

## 5.8 Implementation Risks and Mitigations

| Risk                                | Impact | Likelihood | Mitigation                                                                     |
| ----------------------------------- | ------ | ---------- | ------------------------------------------------------------------------------ |
| Shell script compatibility issues   | High   | Medium     | Stick to POSIX-compatible features; test across environments                   |
| Complex workflows becoming unwieldy | Medium | Medium     | Focus on simplicity; implement only essential functionality                    |
| Template maintenance overhead       | Medium | Low        | Design templates for minimal maintenance; use variables where appropriate      |
| LLM integration challenges          | High   | Medium     | Focus on general principles; provide platform-specific options where necessary |
| Documentation becoming outdated     | Medium | High       | Automate documentation updates; make updating easy                             |

## 5.9 AS-BUILT Updates

### 5.9.1 Actual Implementation Flow

Intent v2.0.0 was developed through organic growth:

```
Phase 1: Foundation (v0.0.0)
ST0001 → ST0002 → ST0003 → ST0004 → ST0005 → ST0006

Phase 2: Enhancement (v1.0.0 - v1.2.1)
ST0007 → ST0008 → ST0009 → ST0010 → ST0011 → ST0012

Phase 3: Rebrand (v2.0.0)
ST0013 (blog) → ST0014 → ST0015 → ST0016 (migration)
```

### 5.9.2 Key Implementation Achievements (v2.0.0)

1. **Self-Hosting Success**: Intent built using Intent methodology
2. **Test Coverage**: 86 tests covering all critical functionality
3. **Migration Tools**: Comprehensive upgrade from any version
4. **User Experience**: Bootstrap, doctor, and upgrade commands
5. **Documentation**: 7-part blog series and updated TPD

### 5.9.3 Lessons Learned (v2.0.0)

1. **JSON > YAML**: Better validation and hierarchy support
2. **Flattened Structure**: Easier navigation than nested directories
3. **Git Safety**: Wrapper commands prevent common errors
4. **Progressive Enhancement**: Core functionality first, then UX
5. **Self-Documentation**: Using the tool to build itself ensures accuracy

## 5.10 AS-BUILT: v2.1.0 through v2.4.0

### 5.10.1 Implementation Flow (v2.1.0 - v2.4.0)

```
Phase 4: Agent Init (v2.1.0)
ST0017 (agent init command, manifest management)

Phase 5: Plugin Architecture (v2.2.0)
ST0018 (plugin architecture, command routing)

Phase 6: AGENTS.md + Subagent Rename (v2.3.0)
ST0019 (treeindex system)
AGENTS.md commands, Claude subagent namespace

Phase 7: Skills + Elixir Modernization (v2.4.0)
ST0020 (WP-01..WP-10: skills, subagent refactor, treeindex, upgrade)
```

### 5.10.2 Key Achievements (v2.4.0)

1. **Plugin Architecture**: Extensible command system proven across 4 plugins
2. **Skills System**: 4 skills with lifecycle management (install, sync, uninstall)
3. **Treeindex**: Pre-computed codebase navigation across all projects
4. **302 Tests**: Up from 86 at v2.0.0, comprehensive BATS coverage
5. **8 Target Projects**: Skills and subagents deployed to production projects
6. **Claude Upgrade Tool**: Automated project modernization with diagnose/plan/execute

### 5.10.3 Lessons Learned (v2.4.0)

1. **Skills + Subagents complementary**: Skills shape generation, subagents do deep review
2. **SHA256 manifests**: Reliable artifact tracking, but sync needs source-vs-local distinction
3. **Em dashes cause multi-byte truncation**: Use only ASCII in CLI-displayed content
4. **Terminal width detection unreliable**: Need cascading fallback strategy
5. **YAML frontmatter enables discovery**: Claude Code picks up skills with proper metadata
