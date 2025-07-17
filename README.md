# Intent: The Steel Thread Process

[![Intent Tests](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml/badge.svg)](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml)

> **Transform how you build software by capturing the "why" alongside the "what"**

Intent is a lightweight, intention-aware development methodology that helps you build better software by preserving the context and reasoning behind every decision. It's designed from the ground up to enhance collaboration between developers and AI assistants, making your development process more efficient and your codebase more maintainable.

## 🎯 The Problem Intent Solves

Ever joined a project and wondered:

- Why was this approach chosen over alternatives?
- What problem was this code originally solving?
- What were the trade-offs considered?
- Why did we structure it this way?

Traditional documentation captures _what_ the code does, but rarely preserves _why_ it exists. This context loss leads to:

- 🔄 Repeated mistakes and circular discussions
- 🤔 Confusion about design decisions
- 🚫 Fear of changing "mysterious" code
- 🤖 Poor AI assistance due to missing context

## 💡 The Intent Solution

Intent introduces **Steel Threads** - self-contained units of work that capture not just tasks, but the entire context of why work is being done. Combined with **Backlog.md** for task management, Intent creates a two-tier system that preserves both strategic intent and tactical execution.

### Key Benefits

- **📝 Never Lose Context**: Every decision is documented with its reasoning
- **🤖 AI-Ready**: LLMs understand your project deeply, providing better assistance
- **👥 Team Continuity**: New developers understand the "why" immediately
- **🔍 Traceable Decisions**: See the evolution of your project's thinking
- **🚀 Faster Development**: Less time explaining, more time building

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
```

### Your First Steel Thread

```bash
# Create a new steel thread
$ intent st new "Add user authentication"
Created: ST0001

# Create associated tasks
$ intent task create ST0001 "Research auth libraries"
$ intent task create ST0001 "Implement login endpoint"
$ intent task create ST0001 "Add session management"

# Check status
$ intent status show ST0001
```

### Integrate with Backlog.md

```bash
# Install Backlog.md (npm required)
npm install -g backlog.md

# Initialize Backlog in your project
intent bl init

# Create and manage tasks
intent bl create ST0001 "Configure OAuth provider"
intent bl list
intent bl board
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

### LLM Integration

```bash
intent llm usage_rules               # Display Intent usage patterns for LLMs
intent llm usage_rules --symlink     # Create usage-rules.md symlink
```

## 🏗️ Project Structure

```
.
├── bin/           # Intent command-line tools
├── docs/          # Documentation and blog posts
├── intent/        # Project artifacts (when using Intent)
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

## 🎯 Use Cases

Intent is particularly valuable for:

- **🚀 Startups**: Preserve founder vision through rapid pivots
- **🏢 Enterprise**: Maintain knowledge through team changes
- **🤖 AI Development**: Provide rich context for LLM assistance
- **📚 Open Source**: Help contributors understand project decisions
- **🎓 Education**: Teach software design thinking

## 🔮 Future Vision

Intent is evolving to become the standard for intention-aware development:

- **Q1 2025**: Enhanced configuration and reporting
- **Q2 2025**: Multi-user collaboration features
- **Q3 2025**: Native AI integrations (MCP support)
- **Q4 2025**: Enterprise features and scalability

See our [roadmap](./docs/blog/0006-next-steps-and-future-work.md#roadmap-the-next-12-months) for details.

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
