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
└── tasks.md         # Implementation tasks and tracking
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

Intent v2.4.0 integrates with [Claude Code](https://claude.ai/code) through sub-agents and skills to supercharge AI collaboration:

```bash
# Initialize subagent configuration (one-time setup)
$ intent claude subagents init

# Install the Intent subagent
$ intent claude subagents install intent

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

# Install Claude Code subagent (if using Claude)
intent claude subagents install intent
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

### Treeindex (Directory Summaries)

```bash
intent treeindex <dir>               # Generate .treeindex directory summaries
intent treeindex --depth 3 <dir>     # Control traversal depth (default: 2)
intent treeindex --check <dir>       # Check staleness without regenerating
intent treeindex --prune <dir>       # Remove orphaned shadow entries
intent treeindex --force <dir>       # Regenerate regardless of staleness
intent treeindex --dry-run <dir>     # Preview without writing
```

### AGENTS.md Management

```bash
intent agents init                   # Initialize AGENTS.md for the project
intent agents generate               # Generate/regenerate AGENTS.md
intent agents sync                   # Update AGENTS.md with latest project state
intent agents validate               # Validate AGENTS.md against specification
```

### Claude Subagent Management

```bash
intent claude subagents list         # Show available and installed subagents
intent claude subagents install <name>  # Install a subagent to Claude Code
intent claude subagents install --all   # Install all available subagents
intent claude subagents status       # Check subagent health and integrity
intent claude subagents sync         # Update subagents with latest versions
intent claude subagents show <name>  # Display detailed subagent information
```

### Claude Skills Management

```bash
intent claude skills list            # Show available and installed skills
intent claude skills install <name>  # Install a skill to .claude/skills/
intent claude skills install --all   # Install all available skills
intent claude skills sync            # Update installed skills with latest versions
intent claude skills uninstall <name>  # Remove Intent-managed skills
intent claude skills show <name>     # Display skill content and status
```

### LLM Guidance Upgrade

```bash
intent claude upgrade                # Diagnose LLM guidance files (dry-run)
intent claude upgrade --apply        # Apply upgrade changes
intent claude upgrade --project-dir DIR  # Target external project
```

## 🏗️ Project Structure

```
.
├── bin/              # Intent command-line tools
├── docs/             # Documentation and blog posts
├── intent/           # Project artifacts (when using Intent)
│   ├── .treeindex/   # Shadow directory for LLM-oriented summaries
│   ├── st/           # Steel threads
│   ├── eng/          # Engineering documentation
│   │   └── tpd/      # Technical Product Design
│   ├── usr/          # User documentation
│   ├── llm/          # LLM-specific content (AGENTS.md, usage-rules)
│   ├── plugins/      # Plugin architecture
│   │   ├── agents/   # AGENTS.md plugin (inc. templates)
│   │   └── claude/   # Claude Code integration
│   │       ├── subagents/  # Subagent definitions
│   │       └── skills/     # Skill definitions
│   └── wip.md        # Current work
├── lib/              # Templates and libraries
└── tests/            # Test suites
```

## 🤝 Contributing

We welcome contributions! The best way to contribute is to:

1. Create a steel thread for your contribution
2. Document your intent and approach
3. Submit a PR referencing your steel thread

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

No! Start with just steel threads to capture intentions. Add treeindex for LLM context, Claude subagents for AI integration, and skills for specialized workflows. Intent grows with your needs.

### How does this help with AI coding?

AI assistants are great at writing code but terrible at understanding your specific context. Intent provides that context in a structured way that AIs can understand and use.

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
