---
title: "Getting Started with STP"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 2489
---

# Getting Started with STP: Your Practical Implementation Guide

After exploring the [philosophy](./0000-motivation-for-stp.md) and [methodology](./0002-the-steel-thread-methodology.md) behind STP, you're ready to implement it in your own projects. This guide walks you through installation, daily workflow, and practical tips for success.

What makes this guide unique? We used STP itself to manage the creation of this blog series. Throughout this post, I'll share real command outputs and workflow states from our actual process, giving you an authentic view of STP in action.

## Installation and Setup

### Prerequisites

STP requires minimal dependencies:
- Bash shell (version 4.0+)
- Git for version control
- A text editor that handles Markdown
- Optional: Node.js for Backlog.md integration

### Installing STP

1. **Clone the repository**:
```bash
git clone https://github.com/matthewsinclair/stp.git
cd stp
```

2. **Add STP to your PATH**:
```bash
export PATH="$PATH:$(pwd)/stp/bin"
# Add to your shell profile for persistence
```

3. **Verify installation**:
```bash
$ stp --version
STP version 1.0.0
```

### Installing Backlog.md

STP integrates beautifully with Backlog.md for task management:

```bash
# Install Backlog globally
npm install -g @backlog/cli

# Initialize in your project
stp bl init
```

### Bootstrapping STP in Your Project

**For new projects**:
```bash
mkdir my-project && cd my-project
git init
stp init
```

This creates the STP directory structure:
```
my-project/
├── stp/
│   ├── _templ/     # Templates
│   ├── bin/        # STP scripts
│   ├── doc/        # Documentation
│   ├── eng/        # Engineering docs
│   │   └── tpd/    # Technical Product Design
│   ├── prj/        # Project management
│   │   ├── st/     # Steel threads
│   │   ├── journal.md
│   │   └── wip.md
│   └── usr/        # User documentation
└── backlog/        # Backlog.md tasks
```

**For existing projects**:
```bash
cd existing-project
stp init --integrate
```

This adds STP without disrupting your current structure.

## Basic Commands and Workflow

### Core STP Commands

**Steel Thread Management**:
```bash
# Create a new steel thread
$ stp st new
Enter title: Implement user authentication
Created: ST0015

# List all steel threads
$ stp st list
ID     | Title                  | Status      | Created    
-------|------------------------|-------------|------------
ST0015 | Implement user auth... | Not Started | 2025-07-08
ST0014 | Directory Structure... | In Progress | 2025-03-20

# Show specific steel thread
$ stp st show ST0015

# Sync steel thread index
$ stp st sync --write
```

**Task Management with Backlog Integration**:
```bash
# Create tasks linked to steel threads
$ stp task create ST0015 "Research auth libraries"
Created task task-59

# List tasks for a steel thread
$ stp task list ST0015
Tasks for ST0015:
================
task-59      [todo]          Research auth libraries

# Use Backlog commands via wrapper
$ stp bl list    # Avoids git errors
$ stp bl board   # View Kanban board
```

**Status Synchronisation**:
```bash
# Check if thread status matches task completion
$ stp status show ST0015
Steel Thread: ST0015
Current Status: Not Started
Task Summary:
  Total Tasks: 1
  - Todo: 1
Recommended Status: Not Started

# Sync status based on task completion
$ stp status sync ST0015
```

## Daily Workflow with STP

### Starting Your Day

1. **Check your WIP document**:
```bash
$ cat stp/prj/wip.md
# Work in Progress

## Current Focus
Working on authentication system (ST0015)
- Researching JWT vs session approaches
- Need to decide on token storage strategy
```

2. **Review active steel threads**:
```bash
$ stp st list --status in_progress
```

### Creating Your First Steel Thread

Let's walk through creating a real steel thread:

```bash
$ stp st new
Enter title: Add user profile editing
Created: ST0016

$ cd stp/prj/st
$ edit ST0016.md
```

The template guides you:

```markdown
---
status: Not Started
created: 20250708
---
# ST0016: Add user profile editing

## Objective
Enable users to update their profile information including 
name, email, and preferences.

## Context
User feedback shows frustration with inability to update 
profiles after registration. This impacts user retention.

## Approach
1. Create profile edit API endpoints
2. Add validation for email uniqueness
3. Implement UI with real-time validation
4. Add audit logging for changes
```

### Breaking Down Work

Create tasks for implementation:

```bash
$ stp task create ST0016 "Design profile edit API"
Created task task-60

$ stp task create ST0016 "Implement backend validation"
Created task task-61

$ stp task create ST0016 "Create profile edit UI"
Created task task-62

$ stp task create ST0016 "Add audit logging"
Created task task-63
```

### Managing Active Work

```bash
# Start working on a task
$ cd backlog/tasks
$ edit "task-60 - ST0016-Design-profile-edit-API.md"
# Change status: To Do -> In Progress

# Check progress
$ stp task list ST0016
Tasks for ST0016:
================
task-60      [in_progress]   Design profile edit API
task-61      [todo]          Implement backend validation
task-62      [todo]          Create profile edit UI
task-63      [todo]          Add audit logging

# Update steel thread status
$ stp status sync ST0016
Updated ST0016 status from 'Not Started' to 'In Progress'
```

## Case Study: How We Used STP to Write This Blog Series

This blog series itself demonstrates STP in action. Let me show you the actual workflow we used.

### The Steel Thread

We started with ST0013:

```bash
$ stp st show ST0013
# ST0013: STP Blog Post Series

- **Status**: Not Started → In Progress
- **Created**: 2025-03-11
- **Author**: Matthew Sinclair

## Objective
Create a series of blog posts about the Steel Thread Process (STP) 
to explain its purpose, philosophy, and implementation.
```

### Task Breakdown

We created 52 tasks across 7 blog posts. Here's our initial task list:

```bash
$ stp bl list
To Do:
  task-6 - ST0013 - Research existing docs for blog 0000
  task-7 - ST0013 - Write introduction section for blog 0000
  task-8 - ST0013 - Write 'Current challenges' section
  [...49 more tasks...]
```

### The Workflow in Action

As we worked, tasks moved through the pipeline:

```bash
# Starting blog post 0000
$ edit "task-6 - ST0013-Research-existing-docs.md"
# status: To Do → In Progress

# After research
# status: In Progress → Done

# Move to writing
$ edit "task-7 - ST0013-Write-introduction.md" 
# status: To Do → In Progress
```

### Progress Tracking

After completing blog post 0003:

```bash
$ stp task list ST0013 | grep -c "\[done\]"
29

$ stp status show ST0013
Steel Thread: ST0013
Current Status: Not Started
Task Summary:
  Total Tasks: 52
  - Done: 29
  - In Progress: 1  
  - Todo: 22
Recommended Status: In Progress
```

### Key Insights from Our Process

1. **Granular tasks maintain momentum**: Each blog section as a separate task meant constant progress
2. **Status synchronisation reveals truth**: The mismatch between "Not Started" and 29 completed tasks showed we needed to sync
3. **Templates guide consistency**: Every blog post followed the same task pattern
4. **Meta-documentation is powerful**: Using STP to document STP creation provides authentic examples

## Integrating STP into Your Existing Process

STP enhances rather than replaces your current methodology:

### With Agile/Scrum

- **User Stories** → Map to steel threads
- **Sprint Planning** → Break threads into Backlog tasks
- **Daily Standups** → Reference `stp task list` output
- **Sprint Review** → Show completed threads

### With Kanban

- **Work Items** → Steel threads flow across board
- **WIP Limits** → Limit in-progress threads
- **Flow Metrics** → Track via `stp status`

### Gradual Adoption Strategy

1. **Week 1**: Start with WIP and journal only
2. **Week 2**: Create first steel thread for new feature
3. **Week 3**: Add Backlog integration
4. **Week 4**: Full workflow with status sync

Teams report smooth adoption with no workflow disruption.

## Common Patterns and Best Practices

### Steel Thread Granularity

✓ **Good Steel Thread Size**:
- Completable in 1-2 weeks
- Delivers visible value
- Has clear success criteria
- 5-15 associated tasks

❌ **Too Large**:
- "Redesign entire application"
- No clear endpoint
- Dozens of tasks

❌ **Too Small**:
- "Fix typo in README"
- Single task
- No strategic value

### Task Breakdown Strategy

```
Steel Thread: Add user notifications
├── Research: Evaluate notification services
├── Design: Create notification schema
├── Backend: Implement notification API
├── Frontend: Add notification UI
├── Testing: Integration tests
└── Documentation: Update API docs
```

### Essential Best Practices

1. **Always use the `stp bl` wrapper**: Prevents git errors
   ```bash
   # Good
   $ stp bl list
   
   # Avoid
   $ backlog list  # May cause git conflicts
   ```

2. **Consistent task naming**: `ST#### - Description`
   ```bash
   $ stp task create ST0016 "Implement caching layer"
   # Creates: task-64 - ST0016-Implement-caching-layer.md
   ```

3. **Update documentation as you code**:
   - Start with intent in steel thread
   - Add discoveries to Implementation Notes
   - Capture decisions in real-time

4. **LLM collaboration pattern**:
   ```bash
   # 1. Share context
   $ cat stp/prj/st/ST0016.md | pbcopy
   
   # 2. Get LLM help
   "I'm working on ST0016, need help with..."
   
   # 3. Update documentation
   $ edit stp/prj/st/ST0016.md
   # Add new insights to Implementation Notes
   ```

5. **Daily journal habit**:
   ```bash
   $ edit stp/prj/journal.md
   ## 2025-07-08
   - Completed profile edit API (ST0016)
   - Discovered rate limiting issue
   - Decision: Implement token bucket
   ```

## Resources for Further Learning

### STP Documentation
- **Reference Guide**: `stp/usr/reference_guide.md`
- **Command Help**: `stp <command> --help`
- **Template Library**: `stp/_templ/`

### Example Projects Using STP
- STP itself (meta!)
- [Community showcase](https://github.com/stp-community)

### Related Tools and Integrations
- [Backlog.md](https://backlog.md) - Task management
- [Claude Code](https://claude.ai/code) - LLM pair programming
- Git hooks for automation

### Getting Help
- GitHub Issues: Report bugs and request features
- Discussions: Share patterns and get advice
- Wiki: Community-contributed guides

## Start Your STP Journey Today

You now have everything needed to implement STP in your projects. Start small:

1. Install STP
2. Create your first steel thread
3. Break it into tasks
4. Experience the clarity

Remember: STP isn't about perfect documentation. It's about capturing enough intention to make future development decisions with confidence.

The journey from confusion to clarity starts with a single steel thread.

[Continue to: Next Steps and Future Work →](./0006-next-steps-and-future-work.md)
