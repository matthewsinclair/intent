---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.0.0
---
# 4. Detailed Design

[index](<./technical_product_design.md>)

## 4.1 Directory Structure

The STP system organizes documentation into a structured directory hierarchy:

```
stp/
├── _templ/             # Templates directory
│   ├── prj/            # Project document templates
│   │   ├── _wip.md
│   │   ├── _journal.md
│   │   └── st/
│   │       ├── _steel_threads.md
│   │       └── _ST####.md
│   ├── eng/            # Engineering document templates
│   │   └── tpd/
│   │       ├── _technical_product_design.md
│   │       ├── _1_introduction.md
│   │       ├── ...
│   ├── usr/            # User document templates
│   │   ├── _user_guide.md
│   │   ├── _reference_guide.md
│   │   └── _deployment_guide.md
│   └── llm/            # LLM document templates
│       └── _llm_preamble.md
├── bin/                # STP scripts
│   ├── stp             # Main STP command
│   ├── stp_init        # Init command implementation
│   ├── stp_st          # Steel thread command implementation
│   ├── stp_help        # Help command implementation
│   └── ...             # Other command implementations
├── prj/                # Project documentation
│   ├── st/             # Steel threads
│   │   ├── steel_threads.md   # Steel thread index
│   │   ├── ST0001.md          # Individual steel thread
│   │   └── ...
│   ├── wip.md          # Work in progress
│   └── journal.md      # Project journal
├── eng/                # Engineering docs
│   └── tpd/            # Technical Product Design
│       ├── technical_product_design.md   # Main TPD document
│       ├── 1_introduction.md            # TPD sections
│       └── ...
├── usr/                # User documentation
│   ├── user_guide.md
│   ├── reference_guide.md
│   └── deployment_guide.md
└── llm/                # LLM-specific content
    ├── llm_preamble.md
    └── *.prompt.md     # Canned prompts
```

## 4.2 Document Templates

### 4.2.1 Project Templates

#### Work In Progress (WIP) Template

The WIP document captures the current state of development and active tasks.

**Structure:**

```markdown
# Work In Progress

## Current Focus
[Brief description of the current development focus]

## Active Steel Threads
- ST####: [Brief description]
- ...

## Upcoming Work
- [Item 1]
- ...

## Notes
[Any additional notes about the current work]
```

#### Journal Template

The Journal document maintains a chronological record of project activities.

**Structure:**

```markdown
# Project Journal

## YYYY-MM-DD
### [Activity Title]
[Description of activity, decisions made, challenges encountered, etc.]

## YYYY-MM-DD
...
```

#### Steel Thread Templates

**Steel Threads Index Template:**

```markdown
# Steel Threads

| ID     | Title   | Status   | Created  | Completed | Link                    |
|--------|---------|----------|----------|-----------|-------------------------|
| ST0002 | [Title] | [Status] | YYYYMMDD | YYYYMMDD  | [ST0002](<./ST0002.md>) |
| ST0001 | [Title] | [Status] | YYYYMMDD | YYYYMMDD  | [ST0002](<./ST0002.md>) |
| ...    | ...     | ...      | ...      | ...       |                         |
```

**Individual Steel Thread Template:**

```markdown
# ST####: [Title]

- **Status**: [Not Started|In Progress|Completed]
- **Created**: YYYY-MM-DD
- **Completed**: YYYY-MM-DD

## Objective
[Clear statement of what this steel thread aims to accomplish]

## Context
[Background information and context for this steel thread]

## Approach
[Planned approach for implementing this steel thread]

## Tasks
- [ ] Task 1
- [ ] Task 2
- ...

## Implementation Notes
[Notes on implementation details, decisions, challenges, etc.]

## Results
[Summary of results after completion]
```

### 4.2.2 Engineering Templates

Technical Product Design templates follow the structure outlined in previous sections.

### 4.2.3 User Documentation Templates

User Guide, Reference Guide, and Deployment Guide templates follow standard technical documentation formats.

### 4.2.4 LLM Templates

The LLM Preamble template provides context and instructions for the LLM:

```markdown
# LLM Preamble

## Project Context
[Brief description of the project]

## Collaboration Guidelines
[Guidelines for how the LLM should collaborate with developers]

## Code Style and Conventions
[Code style and conventions to follow]

## Document Structure
[Description of the document structure for context]

## Process Guidelines
[Guidelines for the steel thread process]
```

## 4.3 Command-line Interface

### 4.3.1 Command Structure

The STP command-line interface follows a subcommand pattern:

```
stp <command> [options] [arguments]
```

Main commands include:

- `init`: Initialize STP in a project
- `st`: Manage steel threads
- `help`: Display help information

Subcommands include:

- `st new`: Create a new steel thread
- `st done`: Mark a steel thread as complete
- `st list`: List all steel threads

### 4.3.2 Command Implementation

Each command is implemented as a separate shell script:

1. `stp`: Main dispatcher that validates input and calls appropriate subcommand
2. `stp_<command>`: Implements specific command functionality
3. `stp_help`: Displays help information from `.help` directory

### 4.3.3 Help System

The help system uses markdown files in a `.help` directory:

```
.help/
├── init.help.md
├── st.help.md
└── ...
```

Each help file follows a standardized format with sections for short description, detailed description, and usage information.

## 4.4 Process Guidelines

### 4.4.1 Steel Thread Workflow

The steel thread workflow follows these steps:

1. **Creation**: Developer creates a new steel thread
2. **Planning**: Developer defines objective, context, and approach
3. **Implementation**: Developer implements tasks with LLM assistance
4. **Documentation**: LLM and developer document implementation details
5. **Completion**: Developer marks the steel thread as complete

### 4.4.2 LLM Collaboration Model

The LLM collaboration model defines how developers work with LLMs:

1. **Context Setting**: Share relevant project documents with the LLM
2. **Task Description**: Clearly describe the current task
3. **Interactive Development**: Iteratively work with the LLM to develop solutions
4. **Documentation**: Have the LLM update documentation as work progresses
5. **Context Preservation**: Capture key information for future sessions

## 4.5 Data Flow

### 4.5.1 Command Data Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  User Input │────►│ STP Command │────►│  Subcommand │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                                               ▼
                                        ┌─────────────┐
                                        │   Project   │
                                        │  Documents  │
                                        └─────────────┘
```

### 4.5.2 Document Update Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Developer  │────►│     LLM     │────►│   Updated   │
│    Input    │     │             │     │  Documents  │
└─────────────┘     └─────────────┘     └─────────────┘
```

## 4.6 Error Handling

STP implements error handling at multiple levels:

1. **Command Validation**: Validate input parameters and provide clear error messages
2. **Execution Validation**: Check for required files and directories before operations
3. **Status Reporting**: Provide clear success/failure indicators for operations
4. **Recovery Guidance**: Suggest recovery steps when errors occur

## 4.7 Security Considerations

STP addresses security through:

1. **No External Dependencies**: Minimizing attack surface through self-contained implementation
2. **File Permission Management**: Ensuring appropriate permissions for created files
3. **Input Validation**: Sanitizing user input to prevent script injection
4. **No Sensitive Data**: Avoiding storage of credentials or sensitive information
