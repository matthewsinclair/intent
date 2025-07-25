---
verblock: "08 Jul 2025:v0.2: Matthew Sinclair - Added Backlog.md integration architecture"
stp_version: 1.2.0
---
# 3. Architecture

[index](<./technical_product_design.md>)

## 3.1 System Architecture Overview

The Steel Thread Process (STP) follows a modular architecture with three primary components:

1. **Documentation Structure**: A standardized directory layout and document templates
2. **Command-line Interface**: Shell scripts for managing STP workflows
3. **Process Guidelines**: Documentation of workflow patterns and best practices

This architecture is designed to be lightweight, portable, and to integrate with existing development environments without significant friction.

```
┌─────────────────────────────────────────────────────────────┐
│                      STP System                             │
│                                                             │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Documentation │    │  Command-line │    │    Process    ││
│  │   Structure   │◄───┤   Interface   │────►   Guidelines  ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│         ▲                     ▲                   ▲         │
└─────────┼─────────────────────┼───────────────────┼─────────┘
          │                     │                   │
┌─────────┼─────────────────────┼───────────────────┼─────────┐
│         │                     │                   │         │
│  ┌──────▼──────┐      ┌───────▼─────┐     ┌───────▼─────┐   │
│  │  Project    │      │    Shell    │     │     LLM     │   │
│  │ Repository  │      │ Environment │     │ Interaction │   │
│  └─────────────┘      └─────────────┘     └─────────────┘   │
│                                                             │
│                  Development Environment                    │
└─────────────────────────────────────────────────────────────┘
```

## 3.2 Component Architecture

### 3.2.1 Documentation Structure

The Documentation Structure consists of a standardized directory layout and markdown templates. Key features include:

- **Directory Organization**: Clear separation of project, technical, user, and LLM-specific documentation
- **Templated Documents**: Standardized starting points for all document types
- **Cross-Referencing**: Internal links to maintain relationships between documents
- **Progressive Documentation**: Documents that evolve alongside the code

```
stp/
├── _templ/             # Templates
├── bin/                # STP scripts
├── prj/                # Project documentation
│   ├── st/             # Steel threads
│   └── wip.md          # Work in progress
├── eng/                # Engineering docs
│   └── tpd/            # Technical Product Design
├── usr/                # User documentation
└── llm/                # LLM-specific content
```

### 3.2.2 Command-line Interface

The Command-line Interface provides shell-based tools for managing STP workflows. Key features include:

- **Unified Command**: Single `stp` entry point with subcommands
- **Modular Implementation**: Each subcommand implemented as a separate script
- **Contextual Help**: Built-in documentation for commands
- **Environment Configuration**: Settings for STP behavior

```
┌─────────────────┐
│  stp (main)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐    ┌─────────────────┐
│  Command        │───►│  Command-       │
│  Dispatcher     │    │  specific       │
└────────┬────────┘    │  implementation │
         │             └─────────────────┘
         ▼
┌─────────────────┐
│  Help System    │
└─────────────────┘
```

### 3.2.3 Process Guidelines

The Process Guidelines define how STP is used in practice. Key features include:

- **Steel Thread Methodology**: Process for incremental development
- **LLM Collaboration**: Patterns for effective LLM assistance
- **Documentation Lifecycle**: How documents evolve through project stages
- **Integration Points**: How STP integrates with other development practices

## 3.3 Data Architecture

STP manages several types of data:

1. **Template Data**: Reusable document templates
2. **Project Metadata**: Information about the project and its status
3. **Work History**: Record of completed work and decisions
4. **Configuration Data**: Settings for STP behavior

All data is stored in plain text formats (primarily markdown) to maximize portability and tool compatibility.

## 3.4 Interface Architecture

### 3.4.1 User Interfaces

STP provides two primary user interfaces:

1. **Command-line Interface**: For developer interaction with STP
2. **Document Structure**: For both human and LLM consumption of project information

### 3.4.2 External System Interfaces

STP is designed to interface with:

1. **Version Control Systems**: Through normal file operations
2. **LLM Systems**: Through document content and canned prompts
3. **Development Environments**: Through standard shell integration
4. **Task Management Systems**: Through Backlog.md integration for fine-grained task tracking

## 3.5 Architectural Decisions

| Decision                     | Rationale                                                                  |
|------------------------------|----------------------------------------------------------------------------|
| Use of Markdown              | Maximizes portability and readability for both humans and LLMs             |
| Shell Scripts Only           | Ensures compatibility across development environments without dependencies |
| Directory-Based Organization | Creates clear structure while maintaining simplicity                       |
| Template-Driven Approach     | Reduces friction in creating consistent documentation                      |
| Steel Thread Methodology     | Breaks work into manageable units suitable for LLM collaboration           |

## 3.6 Integration Architecture

STP is designed as an extensible system that can integrate with complementary tools while maintaining its core philosophy of simplicity and portability.

### 3.6.1 Backlog.md Integration

The integration with Backlog.md extends STP's capabilities with fine-grained task management while preserving the separation of concerns:

#### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      STP System                             │
│                                                             │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Steel Threads │    │  STP Commands │    │   Templates   ││
│  │  (Intent)     │◄───┤  (Workflow)   │────►  (Structure)  ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│         ▲                     │                             │
│         │              ┌──────▼──────┐                      │
│         └──────────────┤ Integration │                      │
│                        │   Layer     │                      │
│                        └──────┬──────┘                      │
└───────────────────────────────┼─────────────────────────────┘
                                │
┌───────────────────────────────┼─────────────────────────────┐
│                               ▼                             │
│                        ┌─────────────┐                      │
│                        │ Backlog.md  │                      │
│                        │   System    │                      │
│                        └─────────────┘                      │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Task Tracking │    │    Kanban     │    │  Task Files  ││
│  │  (Execution)  │◄───┤    Board      │────►  (Storage)   ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│                                                             │
│                    External Task Management                 │
└─────────────────────────────────────────────────────────────┘
```

#### Component Responsibilities

**STP Components:**
- **Steel Threads**: Capture high-level objectives, context, and design decisions
- **Documentation**: Maintain project narrative and technical specifications
- **Process Coordination**: Orchestrate the overall development workflow

**Backlog.md Components:**
- **Task Management**: Track individual implementation tasks with rich metadata
- **Status Tracking**: Provide granular task states and progress visibility
- **Visualisation**: Offer Kanban board and browser-based interfaces

**Integration Layer:**
- **Command Wrappers**: `stp bl`, `stp task`, `stp status`, `stp migrate`
- **Status Synchronisation**: Bidirectional status updates between systems
- **Naming Conventions**: Consistent task naming linking to steel threads

#### Data Flow

1. **Steel Thread Creation** → Integration layer creates linked task structure
2. **Task Updates** → Status changes propagate to steel thread status
3. **Migration** → Embedded tasks convert to Backlog.md format
4. **Queries** → Unified view of steel thread and task information

#### Integration Points

1. **File System**: 
   - STP: `/stp/prj/st/` for steel threads
   - Backlog: `/backlog/` for task management
   - No overlap in storage locations

2. **Command Interface**:
   - Native STP commands remain unchanged
   - Integration commands follow STP patterns
   - Wrapper commands prevent common errors

3. **Status Model**:
   - Steel thread status derived from task states
   - Automatic synchronisation available
   - Manual override supported

4. **Workflow Integration**:
   - Steel threads define "what" and "why"
   - Backlog tasks define "how" and "when"
   - Clear separation of concerns maintained
