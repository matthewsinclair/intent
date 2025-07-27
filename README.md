# Intent: Build Software with AI by Capturing WHY Code Exists

[![Intent Tests](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml/badge.svg)](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml)

Intent helps you build better software by capturing the "why" behind your code. When you document intentions, both your team and AI assistants understand not just what the code does, but why it exists.

## 🚀 See Intent in Action

Instead of giving your AI assistant vague instructions:

```bash
# ❌ Without Intent:
"Build a cache system for our API"
# AI builds generic cache, misses critical requirements
```

Capture your actual intention:

```bash
# ✅ With Intent:
$ intent st new "Implement rate-limited cache for API protection"
# Document: Need cache because API limits to 100 req/min
# Document: Must handle Black Friday traffic spikes (10K req/s)
# AI builds appropriate solution with rate limiting and burst handling
```

**Result**: Your AI assistant understands the constraints and builds the right solution first time.

## 💡 Why This Matters

### For Solo Developers

**Problem**: Your AI assistant forgets context between sessions  
**Solution**: Intent preserves your project's "why" so AI always understands your goals

### For Teams  

**Problem**: New members waste weeks doing "code archaeology"  
**Solution**: Every feature has a Steel Thread documenting why it exists

### For Future You

**Problem**: "Why did I write this weird code 6 months ago?"  
**Solution**: Your past self documented the API limits that forced that approach

## 🎯 What is a Steel Thread?

A **Steel Thread** is a self-contained feature with documented intentions. Think of it as a container that holds:

- **WHY** you're building something (the intention)
- **WHAT** you're building (the design)
- **HOW** you're building it (the tasks)

Example structure:

```
ST0042: Authentication System/
├── info.md          # Why we need auth, what type, constraints
├── design.md        # JWT vs sessions decision, security model  
├── impl.md          # Technical implementation details
└── tasks.md         # Linked Backlog tasks for execution
```

## 🤖 Intent + LLM in Action

### Example 1: Context Persistence

```markdown
# ❌ Without Intent (every new session):
You: "Help me optimize the user service"
LLM: "What does the user service do? What are the constraints?"
[You spend 10 minutes explaining...]

# ✅ With Intent:
You: "I'm working on ST0042" [paste steel thread]
LLM: "I see you're using JWT tokens with 15-min expiry for stateless auth.
      Given your multi-device requirement, here's a refresh token strategy..."
```

### Example 2: Discovering Hidden Knowledge

```bash
# Months later, you wonder: "Can I simplify this cache?"
$ intent st show ST0015
# Reveals: "Cache exists because API rate limits to 100 req/min"
# Now you know why it's "complex" - it's handling burst traffic!
```

### Example 3: Focused AI Assistance

```markdown
# Steel threads keep AI focused:
- Clear boundaries (one feature, not entire codebase)
- Explicit constraints documented ("must handle 10K req/s")
- Design decisions captured ("chose Redis over Memcached because...")
- Result: AI suggestions align with YOUR architecture
```

## 🤖 Claude Code Integration

Intent v2.0.0 integrates with [Claude Code](https://claude.ai/code) sub-agents to supercharge AI collaboration:

```bash
# Install the Intent agent (one-time setup)
$ intent agents install intent

# Now Claude automatically understands:
# ✓ Steel thread methodology  
# ✓ All Intent commands
# ✓ Your project structure
# ✓ Best practices
```

**The difference is dramatic:**

Without Intent agent:

```
You: "Help me add caching"
Claude: "What's your project structure? What caching do you need?"
[10 minutes explaining Intent, constraints, etc.]
```

With Intent agent:

```  
You: "Help me add caching"
Claude: "I'll create a steel thread for caching:
         
         intent st new 'Implement caching layer'
         
         Let's document the intent first - what are you caching?
         Is this for API rate limits or performance? What's your
         expected traffic pattern? I'll help structure this properly."
```

Claude becomes an Intent-fluent development partner from day one.

## 🎯 What is Backlog.md?

[Backlog.md](https://github.com/backlog/backlog) is a Git-native task manager that lives in your repository as markdown files. Intent integrates with it to create a two-tier system:

- **Steel Threads** (Intent): High-level features with documented "why"
- **Tasks** (Backlog.md): Day-to-day work items that implement the "how"

```bash
# Create a steel thread for the big picture
$ intent st new "Add user authentication"
Created: ST0001

# Break it down into specific tasks
$ intent bl create ST0001 "Research auth libraries"
$ intent bl create ST0001 "Implement login endpoint"
$ intent bl create ST0001 "Add password reset flow"

# View your work visually
$ intent bl board
┌────────────────┬────────────────┬────────────────┐
│      TODO      │   IN PROGRESS  │      DONE      │
├────────────────┼────────────────┼────────────────┤
│ Implement      │ Research auth  │                │
│ login endpoint │ libraries      │                │
│                │                │                │
│ Add password   │                │                │
│ reset flow     │                │                │
└────────────────┴────────────────┴────────────────┘
```

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/matthewsinclair/intent.git
cd intent

# Add Intent to your PATH
export PATH="$PATH:$(pwd)/bin"

# Verify installation
intent --version

# See available commands
intent help

# Install Claude Code agent (if using Claude)
intent agents install intent
```

### 🏆 5-Minute Win: Your First Steel Thread

```bash
# 1. Create a steel thread with clear intention
$ intent st new "Add user authentication"
Created: ST0001

# 2. Document WHY you need auth (this is the magic!)
$ intent st edit ST0001
# Add: "Need auth because customer data must be protected"
# Add: "Using JWT because we have multiple microservices"
# Add: "Must support SSO for enterprise clients"

# 3. Share with your AI assistant
$ intent st show ST0001 | pbcopy
# Now paste into Claude, ChatGPT, etc.
# The AI immediately understands your constraints!
```

### 🏆 15-Minute Win: Add Task Management

```bash
# Install Backlog.md
npm install -g backlog.md

# Initialize in your project
intent bl init

# Break down your steel thread into tasks
intent bl create ST0001 "Research JWT libraries for Node.js"
intent bl create ST0001 "Design token refresh strategy"
intent bl create ST0001 "Implement login endpoint"

# See your progress
intent bl board
```

### 🏆 30-Minute Win: Complete First Feature

```bash
# Work through your tasks with AI assistance
$ intent bl list ST0001
# Copy relevant task to discuss with AI

# As you complete work:
$ intent bl move [task-id] doing
$ intent bl move [task-id] done

# Update steel thread with learnings
$ intent st edit ST0001
# Add: "Learned: JWT refresh tokens need rotation for security"
# Add: "Decision: 15-min access token, 7-day refresh token"

# Your future self (and team) will thank you!
```

## 📚 Documentation

### Getting Started

- **[User Guide](./intent/usr/user_guide.md)** - Step-by-step guide to using Intent
- **[Reference Guide](./intent/usr/reference_guide.md)** - Complete command reference and detailed documentation
- **[Installation Guide](./intent/usr/user_guide.md#installation)** - Detailed installation instructions

### Understanding Intent

- **[Technical Product Design](./intent/eng/tpd/technical_product_design.md)** - The complete vision and architecture of Intent
- **[Blog Series](./docs/blog/)** - In-depth exploration of Intent concepts:
  - [Motivation for Intent](./docs/blog/0000-motivation-for-intent.md) - Why intention matters in software
  - [Introduction to Intent](./docs/blog/0001-introduction-to-intent.md) - What Intent is and how it works
  - [The Steel Thread Methodology](./docs/blog/0002-the-steel-thread-methodology.md) - Deep dive into steel threads
  - [Intent Capture in Software Development](./docs/blog/0003-intent-capture-in-software-development.md) - Practical techniques
  - [LLM Collaboration with Intent](./docs/blog/0004-llm-collaboration-with-intent.md) - Enhancing AI assistance
  - [Getting Started with Intent](./docs/blog/0005-getting-started-with-intent.md) - Practical implementation guide
  - [Next Steps and Future Work](./docs/blog/0006-next-steps-and-future-work.md) - Roadmap and vision

### Project Management

- **[Work in Progress (WIP)](./intent/wip.md)** - Current tasks and daily focus
- **[Steel Threads Index](./intent/st/steel_threads.md)** - All steel threads and their status
- **[Backlog Integration](./CLAUDE.md#task-management-with-backlogmd)** - Task tracking and project history

### Development

- **[CLAUDE.md](./CLAUDE.md)** - AI assistant instructions and project conventions
- **[Architecture Overview](./intent/eng/tpd/3_architecture.md)** - System design and components
- **[Detailed Design](./intent/eng/tpd/4_detailed_design.md)** - Implementation details
- **[Testing Guide](./tests/)** - Test suites and integration tests

## 🛠️ Core Commands

### Steel Thread Management

```bash
intent st new <title>          # Create a new steel thread
intent st list                 # List all steel threads
intent st show <ST####>        # Show details of a specific thread
intent st edit <ST####>        # Edit a steel thread
intent st sync                 # Synchronise the steel thread index
```

### Task Management

```bash
intent task create <ST####> <title>  # Create a task linked to a thread
intent task list <ST####>            # List tasks for a thread
intent status show <ST####>          # Show thread and task status
intent status sync <ST####>          # Sync thread status with tasks
```

### Backlog Integration

```bash
intent bl init                       # Initialize Backlog.md
intent bl create <ST####> <title>    # Create a Backlog task
intent bl list                       # List tasks (without git errors)
intent bl board                      # View Kanban board
```

### Agent Management

```bash
intent agents list                   # Show available and installed agents
intent agents install <name>        # Install an agent to Claude Code
intent agents install --all         # Install all available agents
intent agents status                 # Check agent health and integrity
intent agents sync                   # Update agents with latest versions
intent agents show <name>           # Display detailed agent information
```

### LLM Integration

```bash
intent llm usage_rules               # Display Intent usage patterns for LLMs
intent llm usage_rules --symlink     # Create usage-rules.md symlink
```

## 🏗️ Project Structure

```
.
├── agents/        # Claude Code sub-agents (global)
│   ├── intent/    # Intent methodology agent
│   └── elixir/    # Elixir code doctor agent
├── bin/           # Intent command-line tools
├── docs/          # Documentation and blog posts
├── intent/        # Project artifacts (when using Intent)
│   ├── agents/    # Project-specific sub-agents
│   ├── st/        # Steel threads
│   ├── eng/       # Engineering documentation
│   │   └── tpd/   # Technical Product Design
│   ├── usr/       # User documentation
│   └── wip.md     # Current work
├── lib/           # Templates and libraries
├── tests/         # Test suites
└── backlog/       # Backlog.md tasks (if integrated)
    ├── tasks/     # Active tasks
    ├── drafts/    # Draft tasks
    └── config.yml # Backlog configuration
```

## 🤝 Contributing

We welcome contributions! The best way to contribute is to:

1. Create a steel thread for your contribution
2. Document your intent and approach
3. Break down work into Backlog tasks
4. Submit a PR referencing your steel thread

See our [contribution workflow](./docs/blog/0006-next-steps-and-future-work.md#contributing-to-intent) for details.

## 🎗️ Real-World Examples

### Building a REST API with Intent

```bash
# Capture the real requirements
$ intent st new "Build REST API for mobile app"
# Document: "Must support offline-first sync"
# Document: "10K daily active users expected"
# Document: "Must work on 3G connections"

# Result: Your API design includes sync strategies, caching, and compression
```

### Refactoring Legacy Code

```bash
$ intent st new "Refactor payment processing"
# Document: "Current system fails under Black Friday load"
# Document: "PCI compliance required by Q2"
# Document: "Cannot break existing integrations"

# AI understands constraints and suggests appropriate patterns
```

### Starting a New Project

```bash
$ intent st new "Project inception: E-commerce platform"
# Document: "Target: Small businesses with <100 products"
# Document: "Must integrate with Shopify/WooCommerce"
# Document: "Budget: 3 developers, 6 months"

# Every future decision references these constraints
```

## ❓ FAQ

### How is this different from code comments?

**Comments** explain what code does. **Intent** captures why the code exists, what problems it solves, and what constraints shaped it. This context is what AI assistants need to give good suggestions.

### Do I need to use all features?

No! Start with just steel threads to capture intentions. Add Backlog.md when you need task tracking. Intent grows with your needs.

### How does this help with AI coding?

AI assistants are great at writing code but terrible at understanding your specific context. Intent provides that context in a structured way that AIs can understand and use.

### Can I use Intent without Backlog.md?

Absolutely! Steel threads work independently. Backlog.md just adds visual task management when you need it.

### Is this just more documentation to maintain?

Unlike traditional docs that go stale, Intent documentation drives your development. When you update a steel thread, you're planning work, not writing about completed work.

## 📖 Philosophy

> "Great software isn't just about what it does – it's about why it exists."

Intent transforms software development from a purely technical exercise into a practice that values and preserves human intention. By capturing the "why" alongside the "what", we create software that is not just functional, but truly understood.

## 🚦 Getting Help

- **Quick Start**: Run `intent help` for command overview
- **User Guide**: See [comprehensive guide](./intent/usr/user_guide.md)
- **Examples**: Check the [blog series](./docs/blog/) for real-world usage
- **Issues**: Report bugs on [GitHub Issues](https://github.com/matthewsinclair/intent/issues)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

---

**Start capturing intention today. Your future self (and team) will thank you.**

```bash
# Begin your Intent journey
$ intent st new "My first steel thread"
```
