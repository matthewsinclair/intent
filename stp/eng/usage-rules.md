---
verblock: "09 Jul 2025:v1.0: Matthew Sinclair - Regenerated for v1.2.1 directory structure"
stp_version: 1.2.1
---
# STP Usage Rules

This document provides comprehensive usage patterns and workflows for the Steel Thread Process (STP) system. It's designed to help Large Language Models (LLMs) understand how to use STP effectively in development scenarios.

## Introduction

STP is a structured development and documentation system designed for collaboration between developers and LLMs. It provides intention-aware workflows through steel threads, task management integration, and structured documentation patterns.

STP enhances existing development processes by:

- Capturing and preserving intention throughout development
- Providing structured templates for documentation
- Integrating with task management systems
- Facilitating effective LLM collaboration

## Core Workflows

### Starting with STP

When beginning work with STP, follow this sequence:

1. **Initialize project** (if not already done):

   ```bash
   stp init "Project Name"
   ```

2. **Initialize task management**:

   ```bash
   stp bl init
   ```

3. **Review existing steel threads**:

   ```bash
   stp st list
   ```

4. **Check project documentation**:

   ```bash
   # Review technical product design
   cat stp/eng/tpd/technical_product_design.md
   
   # Check current work in progress
   cat stp/prj/wip.md
   ```

### Steel Thread Lifecycle

The typical steel thread workflow follows this pattern:

1. **Create steel thread** with clear intention:

   ```bash
   stp st new "Implement user authentication system"
   ```

2. **Develop in the steel thread directory**:

   ```bash
   # Edit main information
   stp st edit ST0015 info
   
   # Add design decisions
   stp st edit ST0015 design
   
   # Document implementation details
   stp st edit ST0015 impl
   ```

3. **Create linked tasks** for granular work:

   ```bash
   stp task create ST0015 "Design authentication schema"
   stp task create ST0015 "Implement login endpoint"
   stp task create ST0015 "Add session management"
   ```

4. **Track progress** through task completion:

   ```bash
   # View task status
   stp task list ST0015
   
   # Update steel thread status
   stp status sync ST0015
   ```

5. **Complete steel thread**:

   ```bash
   stp st done ST0015
   ```

## Command Usage Patterns

### Steel Thread Commands (`stp st`)

#### Creating Steel Threads

```bash
# Create with descriptive, action-oriented title
stp st new "Implement user authentication"

# Avoid vague titles
stp st new "Fix stuff"  # ❌ Too vague
stp st new "Authentication work"  # ❌ Not action-oriented
```

**Best practices:**

- Use imperative mood ("Implement", "Add", "Fix")
- Be specific about the outcome
- Focus on single features or problems
- Keep title under 50 characters

#### Viewing Steel Threads

```bash
# List all steel threads
stp st list

# Filter by status
stp st list --status "In Progress"

# Show specific steel thread (defaults to info.md)
stp st show ST0015

# Show specific file
stp st show ST0015 design
stp st show ST0015 impl
stp st show ST0015 tasks
stp st show ST0015 results

# Show all files combined
stp st show ST0015 all
```

#### Editing Steel Threads

```bash
# Edit main info file
stp st edit ST0015

# Edit specific files
stp st edit ST0015 design
stp st edit ST0015 impl
stp st edit ST0015 tasks
stp st edit ST0015 results
```

#### Synchronizing Steel Threads

```bash
# Preview sync changes
stp st sync

# Update steel_threads.md index
stp st sync --write

# Adjust table width for display
stp st sync --write --width 120
```

### Task Management Commands

#### Creating Tasks

```bash
# Create task linked to steel thread
stp task create ST0015 "Add password validation"

# Alternative using backlog wrapper
stp bl create ST0015 "Add password validation"
```

**Task naming conventions:**

- Tasks automatically get prefixed with steel thread ID
- Use descriptive action verbs
- Keep tasks granular (1-2 days of work)

#### Viewing Tasks

```bash
# List all tasks for a steel thread
stp task list ST0015

# List all tasks in project
stp bl list

# View Kanban board
stp bl board

# Open browser interface
stp bl browser
```

#### Task Status Management

```bash
# Edit task status
stp bl task edit task-5 --status "In Progress"
stp bl task edit task-5 --status "Done"

# View task details
stp bl task show task-5
```

### Status Synchronization

```bash
# Check current status
stp status show ST0015

# Sync steel thread status with tasks
stp status sync ST0015

# Preview sync changes
stp status sync ST0015 --dry-run

# Generate report for all active threads
stp status report
```

### LLM Integration Commands

```bash
# Display usage rules for LLM context
stp llm usage_rules

# Create symlink for tool integration
stp llm usage_rules --symlink

# Save for reference
stp llm usage_rules > usage-rules.md
```

## Steel Thread Directory Structure (v1.2.1+)

Starting with STP v1.2.1, steel threads are organized as directories:

```
stp/prj/st/
├── ST0001/
│   ├── info.md      # Metadata, objective, context (required)
│   ├── design.md    # Design decisions and approach
│   ├── impl.md      # Implementation details
│   ├── tasks.md     # Task tracking
│   └── results.md   # Results and outcomes
├── ST0002/
│   └── info.md      # Minimum required file
├── COMPLETED/       # Completed steel threads
│   └── ST0003/
├── NOT-STARTED/     # Not started steel threads
│   └── ST0004/
└── CANCELLED/       # Cancelled steel threads
    └── ST0005/
```

### File Purposes

#### info.md (Required)

Contains metadata, objectives, and context:

```yaml
---
verblock: "09 Jul 2025:v0.1: Author Name - Initial version"
stp_version: 1.2.1
status: Not Started|In Progress|Completed|On Hold|Cancelled
created: 20250709
completed: 
---
```

#### design.md (Optional)

Documents design decisions, architectural choices, and approach.

#### impl.md (Optional)

Records implementation details, code structure, and technical decisions.

#### tasks.md (Optional)

Tracks task-related information when not using Backlog integration.

#### results.md (Optional)

Captures outcomes, metrics, and lessons learned.

### Directory Organization

Steel threads can be organized by status:

```bash
# Preview organization
stp st organize

# Actually organize directories
stp st organize --write
```

This moves completed, not-started, and cancelled steel threads to subdirectories.

## Task Management Integration

### Backlog.md Integration

STP integrates with Backlog.md for fine-grained task management:

#### Initialization

```bash
# Initialize with STP-friendly settings
stp bl init
```

This configures Backlog for local use, disabling remote operations.

#### Task Creation Workflow

```bash
# Create steel thread
stp st new "Implement feature X"

# Create associated tasks
stp task create ST0015 "Research requirements"
stp task create ST0015 "Design API endpoints"
stp task create ST0015 "Implement core logic"
stp task create ST0015 "Write tests"
```

#### Task Lifecycle

1. **Creation**: Tasks start in "To Do" status
2. **Development**: Move to "In Progress"
3. **Completion**: Mark as "Done"
4. **Archival**: Archive when no longer needed

```bash
# Update task status
stp bl task edit task-5 --status "In Progress"
stp bl task edit task-5 --status "Done"

# Archive completed tasks
stp bl task archive task-5
```

### Status Synchronization

Steel thread status is automatically determined by task states:

- **Not Started**: No tasks or all tasks in draft
- **In Progress**: At least one task in "To Do" or "In Progress"
- **Completed**: All tasks marked as "Done"
- **On Hold**: Manual designation when work is paused
- **Cancelled**: Manual designation with tasks archived

```bash
# Sync status based on tasks
stp status sync ST0015

# Check current status
stp status show ST0015
```

## LLM Collaboration Patterns

### Effective LLM Workflows

#### Session Initialization

1. **Provide context** from steel thread documentation:

   ```bash
   stp st show ST0015 all
   ```

2. **Share current task status**:

   ```bash
   stp task list ST0015
   ```

3. **Reference usage rules**:

   ```bash
   stp llm usage_rules
   ```

#### Collaborative Development

1. **Start with intention**: Always begin by understanding the steel thread's objective
2. **Work incrementally**: Focus on specific files (design, impl, etc.)
3. **Update documentation**: Keep steel thread files current during development
4. **Link to tasks**: Reference specific tasks when implementing features

#### Best Practices for LLM Collaboration

- **Provide steel thread context** at the start of sessions
- **Reference specific files** when discussing implementation details
- **Update status regularly** as work progresses
- **Use structured prompts** that reference steel thread organization
- **Maintain intention clarity** throughout the conversation

### Common LLM Interaction Patterns

#### Understanding Project Context

```bash
# Get project overview
stp st list

# Review active work
stp st list --status "In Progress"

# Check specific steel thread
stp st show ST0015
```

#### Working on Implementation

```bash
# View design decisions
stp st show ST0015 design

# Document implementation details
stp st edit ST0015 impl

# Track progress
stp task list ST0015
```

#### Completing Work

```bash
# Update results
stp st edit ST0015 results

# Sync final status
stp status sync ST0015

# Mark as complete
stp st done ST0015
```

## Migration from Legacy Formats

### Upgrading STP Files

```bash
# Upgrade all STP files to latest format
stp upgrade

# Force upgrade for major version changes
stp upgrade --force

# Upgrade and organize by status
stp upgrade --organize
```

### Migrating Embedded Tasks

For steel threads with embedded checkbox tasks:

```bash
# Migrate specific steel thread
stp migrate ST0014

# Preview migration
stp migrate --dry-run ST0014

# Migrate all active threads
stp migrate --all-active
```

This extracts checkbox tasks and creates corresponding Backlog tasks.

## Common Workflows

### New Feature Development

1. **Create steel thread**:

   ```bash
   stp st new "Add user profile management"
   ```

2. **Document approach**:

   ```bash
   stp st edit ST0016 design
   ```

3. **Break down into tasks**:

   ```bash
   stp task create ST0016 "Design user profile schema"
   stp task create ST0016 "Implement profile API endpoints"
   stp task create ST0016 "Create profile UI components"
   stp task create ST0016 "Add profile validation"
   stp task create ST0016 "Write integration tests"
   ```

4. **Track implementation**:

   ```bash
   stp st edit ST0016 impl
   ```

5. **Document outcomes**:

   ```bash
   stp st edit ST0016 results
   ```

### Bug Fix Workflow

1. **Create focused steel thread**:

   ```bash
   stp st new "Fix authentication timeout issue"
   ```

2. **Create investigation tasks**:

   ```bash
   stp task create ST0017 "Reproduce timeout issue"
   stp task create ST0017 "Debug session handling"
   stp task create ST0017 "Implement fix"
   stp task create ST0017 "Add regression test"
   ```

3. **Document findings**:

   ```bash
   stp st edit ST0017 impl
   ```

4. **Track resolution**:

   ```bash
   stp status sync ST0017
   ```

### Research and Investigation

1. **Create research steel thread**:

   ```bash
   stp st new "Evaluate caching strategies"
   ```

2. **Create investigation tasks**:

   ```bash
   stp task create ST0018 "Research Redis capabilities"
   stp task create ST0018 "Benchmark performance options"
   stp task create ST0018 "Document recommendations"
   ```

3. **Capture findings**:

   ```bash
   stp st edit ST0018 results
   ```

## Common Mistakes and Solutions

### Mistake: Creating Steel Threads That Are Too Broad

**Problem**: Steel threads like "Implement entire authentication system"

**Solution**: Break into focused threads:

```bash
stp st new "Implement user login functionality"
stp st new "Add password reset feature"
stp st new "Create session management"
```

### Mistake: Not Updating Status

**Problem**: Steel threads with stale status information

**Solution**: Regular status synchronization:

```bash
stp status sync ST0015
stp status report
```

### Mistake: Not Using Directory Structure

**Problem**: Putting all information in info.md

**Solution**: Use appropriate files:

```bash
stp st edit ST0015 design    # For design decisions
stp st edit ST0015 impl      # For implementation details
stp st edit ST0015 results   # For outcomes
```

### Mistake: Disconnected Tasks

**Problem**: Creating tasks without linking to steel threads

**Solution**: Always link tasks:

```bash
stp task create ST0015 "Task description"
# NOT: stp bl task create "Unlinked task"
```

## Testing and Validation

### Running Tests

```bash
# Run all STP tests
cd stp/tests/
./run_tests.sh

# Run specific test suites
./run_tests.sh st
./run_tests.sh task
./run_tests.sh status
```

### Validation Workflow

1. **Test command functionality**:

   ```bash
   stp st new "Test steel thread"
   stp task create ST0019 "Test task"
   stp status sync ST0019
   ```

2. **Verify documentation updates**:

   ```bash
   stp st sync --write
   ```

3. **Check task integration**:

   ```bash
   stp task list ST0019
   stp bl list
   ```

## Advanced Usage

### Customizing Templates

STP templates are located in `stp/_templ/` and can be customized:

- `stp/_templ/st/`: Steel thread templates
- `stp/_templ/eng/`: Engineering documentation templates
- `stp/_templ/usr/`: User documentation templates

### Integration with Other Tools

#### Git Integration

Steel threads work well with git workflows:

```bash
# Create feature branch for steel thread
git checkout -b feature/ST0015-user-auth

# Commit with steel thread reference
git commit -m "ST0015: Implement login endpoint"
```

#### CI/CD Integration

Reference steel threads in deployment scripts:

```bash
# In deployment script
echo "Deploying changes from ST0015: User Authentication"
```

## Further Reading

For deeper understanding of STP concepts and methodology, see these blog posts:

- [Motivation for STP](../doc/blog/0000-motivation-for-stp.md) - Why intention matters in development
- [Introduction to STP](../doc/blog/0001-introduction-to-stp.md) - Overview of the system
- [The Steel Thread Methodology](../doc/blog/0002-the-steel-thread-methodology.md) - Understanding steel threads
- [Intent Capture in Software Development](../doc/blog/0003-intent-capture-in-software-development.md) - Philosophy behind STP
- [LLM Collaboration with STP](../doc/blog/0004-llm-collaboration-with-stp.md) - Working with AI assistants
- [Getting Started with STP](../doc/blog/0005-getting-started-with-stp.md) - Practical tutorial

For comprehensive reference information, see:

- `stp/usr/user_guide.md` - Task-based guidance for users
- `stp/usr/reference_guide.md` - Complete reference for all features
- `stp/eng/tpd/technical_product_design.md` - Technical architecture and design

## Summary

STP provides a structured approach to development that preserves intention while enabling effective collaboration between humans and LLMs. The key principles are:

1. **Start with intention** - Every steel thread begins with clear purpose
2. **Use directory structure** - Organize documentation into focused files
3. **Link tasks to threads** - Maintain traceability from intent to implementation
4. **Sync status regularly** - Keep steel thread status current with task progress
5. **Collaborate effectively** - Use structured context for LLM interactions

By following these patterns and workflows, you can leverage STP to create more intentional, well-documented, and maintainable software development processes.
