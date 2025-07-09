# Steel Thread Process (STP)

[![STP Tests](https://github.com/matthewsinclair/dev-stp/actions/workflows/tests.yml/badge.svg)](https://github.com/matthewsinclair/dev-stp/actions/workflows/tests.yml)

> **Transform how you build software by capturing the "why" alongside the "what"**

STP is a lightweight, intention-aware development methodology that helps you build better software by preserving the context and reasoning behind every decision. It's designed from the ground up to enhance collaboration between developers and AI assistants, making your development process more efficient and your codebase more maintainable.

## 🎯 The Problem STP Solves

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

## 💡 The STP Solution

STP introduces **Steel Threads** - self-contained units of work that capture not just tasks, but the entire context of why work is being done. Combined with **Backlog.md** for task management, STP creates a two-tier system that preserves both strategic intent and tactical execution.

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
git clone https://github.com/matthewsinclair/stp.git
cd stp

# Add STP to your PATH
export PATH="$PATH:$(pwd)/stp/bin"

# Verify installation
stp --version

# See available commands
stp help
```

### Your First Steel Thread

```bash
# Create a new steel thread
$ stp st new "Add user authentication"
Created: ST0001

# Create associated tasks
$ stp task create ST0001 "Research auth libraries"
$ stp task create ST0001 "Implement login endpoint"
$ stp task create ST0001 "Add session management"

# Check status
$ stp status show ST0001
```

### Integrate with Backlog.md

```bash
# Install Backlog.md (npm required)
npm install -g @backlog/cli

# Initialize Backlog in your project
stp bl init

# Create and manage tasks
stp bl create ST0001 "Configure OAuth provider"
stp bl list
stp bl board
```

## 📚 Documentation

### Getting Started
- **[User Guide](./stp/usr/user_guide.md)** - Step-by-step guide to using STP
- **[Reference Guide](./stp/usr/reference_guide.md)** - Complete command reference and detailed documentation
- **[Installation Guide](./stp/usr/user_guide.md#installation)** - Detailed installation instructions

### Understanding STP
- **[Technical Product Design](./stp/eng/tpd/technical_product_design.md)** - The complete vision and architecture of STP
- **[Blog Series](./stp/doc/blog/)** - In-depth exploration of STP concepts:
  - [Motivation for STP](./stp/doc/blog/0000-motivation-for-stp.md) - Why intention matters in software
  - [Introduction to STP](./stp/doc/blog/0001-introduction-to-stp.md) - What STP is and how it works
  - [The Steel Thread Methodology](./stp/doc/blog/0002-the-steel-thread-methodology.md) - Deep dive into steel threads
  - [Intent Capture in Software Development](./stp/doc/blog/0003-intent-capture-in-software-development.md) - Practical techniques
  - [LLM Collaboration with STP](./stp/doc/blog/0004-llm-collaboration-with-stp.md) - Enhancing AI assistance
  - [Getting Started with STP](./stp/doc/blog/0005-getting-started-with-stp.md) - Practical implementation guide
  - [Next Steps and Future Work](./stp/doc/blog/0006-next-steps-and-future-work.md) - Roadmap and vision

### Project Management
- **[Work in Progress (WIP)](./stp/prj/wip.md)** - Current tasks and daily focus
- **[Steel Threads Index](./stp/prj/st/steel_threads.md)** - All steel threads and their status
- **[Backlog Integration](./CLAUDE.md#task-management-with-backlogmd)** - Task tracking and project history

### Development
- **[CLAUDE.md](./CLAUDE.md)** - AI assistant instructions and project conventions
- **[Architecture Overview](./stp/eng/tpd/3_architecture.md)** - System design and components
- **[Detailed Design](./stp/eng/tpd/4_detailed_design.md)** - Implementation details
- **[Testing Guide](./stp/tests/)** - Test suites and integration tests

## 🛠️ Core Commands

### Steel Thread Management
```bash
stp st new <title>          # Create a new steel thread
stp st list                 # List all steel threads
stp st show <ST####>        # Show details of a specific thread
stp st edit <ST####>        # Edit a steel thread
stp st sync                 # Synchronise the steel thread index
```

### Task Management
```bash
stp task create <ST####> <title>  # Create a task linked to a thread
stp task list <ST####>            # List tasks for a thread
stp status show <ST####>          # Show thread and task status
stp status sync <ST####>          # Sync thread status with tasks
```

### Backlog Integration
```bash
stp bl init                       # Initialize Backlog.md
stp bl create <ST####> <title>    # Create a Backlog task
stp bl list                       # List tasks (without git errors)
stp bl board                      # View Kanban board
```

### LLM Integration
```bash
stp llm usage_rules               # Display STP usage patterns for LLMs
stp llm usage_rules --symlink     # Create usage-rules.md symlink
```

## 🏗️ Project Structure

```
stp/
├── bin/           # STP command-line tools
├── doc/           # Documentation and blog posts
├── eng/           # Engineering documentation
│   └── tpd/       # Technical Product Design
├── prj/           # Project management
│   ├── st/        # Steel threads
│   └── wip.md     # Current work
├── tests/         # Test suites
└── usr/           # User documentation

backlog/           # Backlog.md tasks (if integrated)
├── tasks/         # Active tasks
├── drafts/        # Draft tasks
└── config.yml     # Backlog configuration
```

## 🤝 Contributing

We welcome contributions! The best way to contribute is to:

1. Create a steel thread for your contribution
2. Document your intent and approach
3. Break down work into Backlog tasks
4. Submit a PR referencing your steel thread

See our [contribution workflow](./stp/doc/blog/0006-next-steps-and-future-work.md#contributing-to-stp) for details.

## 🎯 Use Cases

STP is particularly valuable for:

- **🚀 Startups**: Preserve founder vision through rapid pivots
- **🏢 Enterprise**: Maintain knowledge through team changes
- **🤖 AI Development**: Provide rich context for LLM assistance
- **📚 Open Source**: Help contributors understand project decisions
- **🎓 Education**: Teach software design thinking

## 🔮 Future Vision

STP is evolving to become the standard for intention-aware development:

- **Q1 2025**: Enhanced configuration and reporting
- **Q2 2025**: Multi-user collaboration features
- **Q3 2025**: Native AI integrations (MCP support)
- **Q4 2025**: Enterprise features and scalability

See our [roadmap](./stp/doc/blog/0006-next-steps-and-future-work.md#roadmap-the-next-12-months) for details.

## 📖 Philosophy

> "Great software isn't just about what it does – it's about why it exists."

STP transforms software development from a purely technical exercise into a practice that values and preserves human intention. By capturing the "why" alongside the "what", we create software that is not just functional, but truly understood.

## 🚦 Getting Help

- **Quick Start**: Run `stp help` for command overview
- **User Guide**: See [comprehensive guide](./stp/usr/user_guide.md)
- **Examples**: Check the [blog series](./stp/doc/blog/) for real-world usage
- **Issues**: Report bugs on [GitHub Issues](https://github.com/matthewsinclair/stp/issues)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

---

**Start capturing intention today. Your future self (and team) will thank you.**

```bash
# Begin your STP journey
$ stp st new "My first steel thread"
```