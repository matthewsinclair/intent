---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Updated for Intent v2.0.0"
intent_version: 2.0.0
---
# Intent Usage Rules

This document provides usage patterns and guidelines for working with Intent v2.0.0. It's designed to help Large Language Models (LLMs) understand how to effectively use Intent commands and workflows in development scenarios.

## Introduction

Intent (formerly STP - Steel Thread Process) is a structured development system that facilitates collaboration between developers and LLMs through:

- **Steel Threads**: Self-contained units of work with clear intent
- **Structured Documentation**: Templates that capture context and decisions
- **Task Integration**: Fine-grained task management linked to larger goals
- **Intent Preservation**: Methodologies for maintaining project context

## Core Workflows

### Starting a New Intent Project

```bash
# Initialize Intent in current directory
intent init "My Project"

# Or specify directory
intent init "My Project" ./my-project
```

This creates the Intent v2.0.0 structure:

- `.intent/config.json` - Project configuration
- `intent/` - Main documentation directory
- `intent/st/` - Steel threads directory
- `intent/wip.md` - Work in progress
- `CLAUDE.md` - Project-specific instructions for LLMs

### Daily Development Workflow

1. **Check Current Work**

   ```bash
   # View current work in progress
   cat intent/wip.md
   
   # List active steel threads
   intent st list --status "In Progress"
   ```

2. **Update Task Status**

   ```bash
   # Check tasks for a steel thread
   intent task list ST0014
   
   # List current tasks (uses backlog_list_status)
   intent bl list
   
   # View all tasks
   intent bl list --all
   ```

3. **Document Progress**
   - Update `intent/wip.md` with current focus
   - Mark completed tasks: `intent bl done <task-id>`
   - Sync thread status: `intent status sync ST0014`

## Command Usage Patterns

### Steel Thread Management (`intent st`)

Steel threads are the backbone of Intent methodology. They represent coherent units of work with clear intent.

#### Creating Steel Threads

```bash
# Create a new steel thread
intent st new "Implement OAuth2 authentication"
# Output: Created new steel thread: ST0015
# Creates: intent/st/ST0015/info.md
```

**Best Practices:**

- Use clear, action-oriented titles
- One feature or fix per thread
- Create thread before starting work

#### Managing Steel Thread Lifecycle

```bash
# List all threads
intent st list

# Filter by status
intent st list --status "In Progress"

# View thread details
intent st show ST0015        # Shows info.md
intent st show ST0015 design # Shows design.md

# Edit thread files
intent st edit ST0015        # Edits info.md
intent st edit ST0015 tasks  # Edits tasks.md

# Repair malformed metadata
intent st repair             # Dry-run on all threads
intent st repair --write     # Actually repair all threads
intent st repair ST0015      # Dry-run on specific thread
```

#### Steel Thread Structure

In Intent v2.0.0, each steel thread is a directory:

```
intent/st/ST0015/
├── info.md      # Main information (required)
├── design.md    # Design decisions (optional)
├── impl.md      # Implementation details (optional)
├── tasks.md     # Task tracking (optional)
└── results.md   # Results and outcomes (optional)
```

### Task Management Integration (`intent task`, `intent bl`)

Intent v2.0.0 provides enhanced Backlog.md integration with configurable filtering.

#### Task Creation and Management

```bash
# Create tasks linked to a steel thread
intent task create ST0015 "Design database schema"
intent task create ST0015 "Implement login endpoint"
intent task create ST0015 "Add session management"

# List tasks for a thread
intent task list ST0015

# Count task completion
intent task count ST0015
```

#### Using the Enhanced Backlog Wrapper

```bash
# Initialize backlog (one-time setup)
intent bl init

# List tasks (filtered by backlog_list_status config)
intent bl list

# List ALL tasks regardless of status
intent bl list --all

# View kanban board
intent bl board

# Create new task
intent bl create "ST0015 - Design auth flow"

# Mark task complete
intent bl done task-5
```

**Why use `intent bl` instead of `backlog` directly?**

- Prevents git fetch errors with automatic `--plain`
- Respects `backlog_list_status` configuration
- Maintains ST#### naming convention
- Provides git-safe wrapper

### Status Synchronization (`intent status`)

Keep steel thread status in sync with task completion:

```bash
# Show status comparison
intent status show ST0015

# Update thread status based on tasks
intent status sync ST0015

# Check all threads
intent status check
```

Status rules:

- 0% tasks complete → "Not Started"
- 1-99% complete → "In Progress"  
- 100% complete → "Completed"

### Migration and Upgrades

#### Upgrading from STP to Intent v2.0.0

```bash
# Upgrade any STP version to Intent v2.0.0
intent upgrade

# Custom backup directory
intent upgrade --backup-dir ./my-backup
```

The upgrade process:

1. Detects current STP version
2. Creates timestamped backup
3. Migrates directory structure
4. Converts YAML config to JSON
5. Updates all file references

#### First-Time Setup

```bash
# Global Intent setup
intent bootstrap

# Force recreation of config
intent bootstrap --force
```

#### Diagnostics

```bash
# Check for configuration issues
intent doctor

# Auto-fix problems
intent doctor --fix
```

#### Repairing Steel Thread Metadata

After migrations or manual edits, steel thread metadata may become corrupted. Use the repair command:

```bash
# Check what repairs are needed (dry-run)
intent st repair

# Repair all steel threads
intent st repair --write

# Repair specific steel thread
intent st repair ST0001 --write
```

The repair command fixes:
- Malformed YAML frontmatter (escaped newlines)
- Legacy field names (stp_version → intent_version)
- Conflicting status values between frontmatter and body
- Invalid date formats
- Missing required fields

## Steel Thread Workflows

### Complete Steel Thread Workflow

1. **Create Thread**

   ```bash
   intent st new "Add user profile management"
   ```

2. **Document Intent** (in the created file)
   - Fill in the Intent section immediately
   - Document constraints and assumptions
   - Note any relevant background

3. **Break Down into Tasks**

   ```bash
   intent task create ST0016 "Design profile data model"
   intent task create ST0016 "Create profile API endpoints"
   intent task create ST0016 "Build profile UI components"
   intent task create ST0016 "Add profile tests"
   ```

4. **Track Progress**

   ```bash
   # Start work on a task
   intent bl task edit task-10  # Change status to "In Progress"
   
   # Check thread status
   intent status show ST0016
   ```

5. **Complete Thread**

   ```bash
   # When all tasks are done
   intent st done ST0016
   
   # Update thread status
   intent status sync ST0016
   ```

### Migrating Embedded Tasks

For steel threads with tasks listed in the document:

```bash
# Migrate embedded tasks to Backlog
intent migrate ST0014

# This extracts tasks and creates them in Backlog
# Original tasks are preserved in an archive section
```

## LLM Collaboration Patterns

### Session Initialization

When starting an LLM session:

1. LLM reads `CLAUDE.md` for project context
2. LLM reads `intent/eng/tpd/technical_product_design.md`
3. LLM checks `intent/wip.md` for current work
4. LLM can run `intent st list --status "In Progress"`

### Working on a Steel Thread

When an LLM is assigned to work on a steel thread:

```bash
# First, understand the thread
intent st show ST0015

# Check existing tasks
intent task list ST0015

# View detailed task information
intent bl list | grep "ST0015"

# Create new tasks as needed
intent task create ST0015 "Additional task discovered"
```

### Maintaining Context

- Update `intent/wip.md` when starting/stopping work
- Document decisions in steel thread files
- Keep `CLAUDE.md` updated with project conventions
- End sessions by updating task status in Backlog

## Common Patterns and Anti-Patterns

### Good Patterns

✅ **Create Thread First, Then Tasks**

```bash
intent st new "Feature X"
intent task create ST0017 "Task 1"
intent task create ST0017 "Task 2"
```

✅ **Regular Status Syncs**

```bash
# After completing tasks
intent bl done task-123
intent status sync ST0017
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

- Run `intent doctor` in CI to catch configuration issues
- Validate thread status matches task completion
- Check for incomplete threads before release

## Further Reading

For deeper understanding of Intent concepts and philosophy:

- [Motivation for Intent](../../docs/blog/0000-motivation-for-intent.md) - Why intention matters
- [Introduction to Intent](../../docs/blog/0001-introduction-to-intent.md) - System overview
- [The Steel Thread Methodology](../../docs/blog/0002-the-steel-thread-methodology.md) - Deep dive
- [Intent Capture](../../docs/blog/0003-intent-capture-in-software-development.md) - Preserving context
- [LLM Collaboration](../../docs/blog/0004-llm-collaboration-with-intent.md) - Working with AI
- [Getting Started](../../docs/blog/0005-getting-started-with-intent.md) - Practical tutorial
- [Next Steps](../../docs/blog/0006-next-steps-and-future-work.md) - Future development

## Quick Reference Card

```bash
# Initialize
intent init "Project Name"

# First-time setup
intent bootstrap

# Create work
intent st new "Feature description"
intent task create ST#### "Task description"

# Track progress  
intent st list --status "In Progress"
intent task list ST####
intent bl list           # Filtered by config
intent bl list --all     # All tasks

# Update status
intent bl done task-id
intent status sync ST####

# Maintain system
intent doctor --fix
intent upgrade           # From any STP version
intent st repair --write # Fix metadata issues
```

## Configuration Quick Reference

```json
// .intent/config.json
{
  "version": "2.0.0",
  "project_name": "My Project",
  "author": "username",
  "backlog_list_status": "todo"  // Filter default
}
```

Remember: Intent is about capturing and preserving intention throughout development. Use it to create a clear narrative of your project's evolution.
