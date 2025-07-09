---
verblock: "09 Jul 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.0
---
# STP Usage Rules

This document provides usage patterns and guidelines for working with the Steel Thread Process (STP) system. It's designed to help Large Language Models (LLMs) understand how to effectively use STP commands and workflows in development scenarios.

## Introduction

STP (Steel Thread Process) is a structured development system that facilitates collaboration between developers and LLMs through:

- **Steel Threads**: Self-contained units of work with clear intent
- **Structured Documentation**: Templates that capture context and decisions
- **Task Integration**: Fine-grained task management linked to larger goals
- **Intent Preservation**: Methodologies for maintaining project context

## Core Workflows

### Starting a New STP Project

```bash
# Initialize STP in a new project
stp init "My Project" ./my-project

# Or initialize in current directory
stp init "My Project"
```

This creates the STP directory structure and essential files:

- `stp/` - Main documentation directory
- `CLAUDE.md` - Project-specific instructions for LLMs
- Initial templates and documentation

### Daily Development Workflow

1. **Check Current Work**

   ```bash
   # View current work in progress
   cat stp/prj/wip.md
   
   # List active steel threads
   stp st list --status "In Progress"
   ```

2. **Update Task Status**

   ```bash
   # Check tasks for a steel thread
   stp task list ST0014
   
   # View task board
   stp bl board
   ```

3. **Document Progress**
   - Update `stp/prj/wip.md` with current focus
   - Mark completed tasks: `stp bl task edit <task-id>`
   - Update steel thread status if needed

## Command Usage Patterns

### Steel Thread Management (`stp st`)

Steel threads are the backbone of STP methodology. They represent coherent units of work with clear intent.

#### Creating Steel Threads

```bash
# Create a new steel thread
stp st new "Implement OAuth2 authentication"
# Output: Created new steel thread: ST0015
```

**Best Practices:**

- Use clear, action-oriented titles
- One feature or fix per thread
- Create thread before starting work

#### Managing Steel Thread Lifecycle

```bash
# List all threads
stp st list

# Filter by status
stp st list --status "In Progress"

# View thread details
stp st show ST0015

# Edit thread document
stp st edit ST0015

# Mark as complete
stp st done ST0015
```

#### Synchronizing Thread Index

```bash
# Preview synchronization
stp st sync

# Write updates to steel_threads.md
stp st sync --write
```

### Task Management Integration (`stp task`, `stp bl`)

STP integrates with Backlog.md for fine-grained task tracking while maintaining the high-level steel thread structure.

#### Task Creation and Management

```bash
# Create tasks linked to a steel thread
stp task create ST0015 "Design database schema"
stp task create ST0015 "Implement login endpoint"
stp task create ST0015 "Add session management"

# List tasks for a thread
stp task list ST0015
```

#### Using the Backlog Wrapper

```bash
# Initialize backlog (one-time setup)
stp bl init

# List all tasks (without git errors)
stp bl list

# View kanban board
stp bl board

# Edit a specific task
stp bl task edit task-5
```

**Why use `stp bl` instead of `backlog` directly?**

- Prevents git fetch errors in local projects
- Adds `--plain` flag automatically
- Configured for STP workflow

### Status Synchronization (`stp status`)

Keep steel thread status in sync with task completion:

```bash
# Update thread status based on task completion
stp status sync ST0015

# Show status summary
stp status show ST0015
```

Status rules:

- 0% tasks complete → "Not Started"
- 1-99% complete → "In Progress"  
- 100% complete → "Completed"

### Upgrading STP Files (`stp upgrade`)

Keep your STP installation current:

```bash
# Upgrade all STP files to latest format
stp upgrade

# Force upgrade even with major version changes
stp upgrade --force
```

This command:

- Updates file metadata
- Adds missing fields
- Synchronizes steel thread index
- Reports all changes

## Steel Thread Workflows

### Complete Steel Thread Workflow

1. **Create Thread**

   ```bash
   stp st new "Add user profile management"
   ```

2. **Document Intent** (in the created file)
   - Fill in the Intent section immediately
   - Document constraints and assumptions
   - Note any relevant background

3. **Break Down into Tasks**

   ```bash
   stp task create ST0016 "Design profile data model"
   stp task create ST0016 "Create profile API endpoints"
   stp task create ST0016 "Build profile UI components"
   stp task create ST0016 "Add profile tests"
   ```

4. **Track Progress**

   ```bash
   # Start work on a task
   stp bl task edit task-10  # Change status to "In Progress"
   
   # Check thread status
   stp status show ST0016
   ```

5. **Complete Thread**

   ```bash
   # When all tasks are done
   stp st done ST0016
   
   # Sync the index
   stp st sync --write
   ```

### Migrating Embedded Tasks

For steel threads with tasks listed in the document:

```bash
# Migrate embedded tasks to Backlog
stp migrate ST0014

# This extracts tasks and creates them in Backlog
# Original tasks are preserved in an archive section
```

## LLM Collaboration Patterns

### Session Initialization

When starting an LLM session:

1. LLM reads `CLAUDE.md` for project context
2. LLM reads `stp/eng/tpd/technical_product_design.md`
3. LLM checks `stp/prj/wip.md` for current work
4. LLM can run `stp st list --status "In Progress"`

### Working on a Steel Thread

When an LLM is assigned to work on a steel thread:

```bash
# First, understand the thread
stp st show ST0015

# Check existing tasks
stp task list ST0015

# View detailed task information
stp bl list | grep "ST0015"

# Create new tasks as needed
stp task create ST0015 "Additional task discovered"
```

### Maintaining Context

- Update `stp/prj/wip.md` when starting/stopping work
- Document decisions in steel thread files
- Keep `CLAUDE.md` updated with project conventions
- End sessions by updating task status in Backlog

## Common Patterns and Anti-Patterns

### Good Patterns

✅ **Create Thread First, Then Tasks**

```bash
stp st new "Feature X"
stp task create ST0017 "Task 1"
stp task create ST0017 "Task 2"
```

✅ **Regular Status Syncs**

```bash
# After completing tasks
stp status sync ST0017
stp st sync --write
```

✅ **Document Intent Immediately**

- Fill in Intent section when creating thread
- Capture "why" not just "what"

### Anti-Patterns

❌ **Creating Overly Broad Threads**

- Bad: "Improve application"
- Good: "Add input validation to user forms"

❌ **Skipping Status Updates**

- Threads show wrong status
- Team loses visibility

❌ **Working Without Threads**

- Lost context and intent
- No clear completion criteria

## Integration Best Practices

### With Git

```bash
# Good commit messages reference threads
git commit -m "ST0015: Implement login endpoint"

# Include thread ID in PR titles
gh pr create --title "ST0015: OAuth2 Authentication"
```

### With Documentation

- Keep Backlog tasks updated with detailed progress
- Link blog posts from thread documents
- Update user guides when adding features

### With CI/CD

- Run `stp st sync --write` in CI to catch inconsistencies
- Validate thread status matches task completion
- Check for incomplete threads before release

## Further Reading

For deeper understanding of STP concepts and philosophy:

- [Motivation for STP](../doc/blog/0000-motivation-for-stp.md) - Why intention matters in software
- [Introduction to STP](../doc/blog/0001-introduction-to-stp.md) - System overview and benefits
- [The Steel Thread Methodology](../doc/blog/0002-the-steel-thread-methodology.md) - Deep dive into steel threads
- [Intent Capture in Software Development](../doc/blog/0003-intent-capture-in-software-development.md) - Techniques for preserving context
- [LLM Collaboration with STP](../doc/blog/0004-llm-collaboration-with-stp.md) - Working effectively with AI
- [Getting Started with STP](../doc/blog/0005-getting-started-with-stp.md) - Practical tutorial

## Quick Reference Card

```bash
# Initialize
stp init "Project Name"

# Create work
stp st new "Feature description"
stp task create ST#### "Task description"

# Track progress  
stp st list --status "In Progress"
stp task list ST####
stp bl board

# Update status
stp bl task edit task-id
stp status sync ST####
stp st done ST####

# Maintain system
stp upgrade
stp st sync --write
```

Remember: STP is about capturing intent and maintaining context throughout development. Use it to create a clear narrative of your project's evolution.
