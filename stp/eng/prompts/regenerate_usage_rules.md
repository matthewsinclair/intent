---
verblock: "09 Jul 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.1
---
# Prompt to Regenerate STP Usage Rules

Use this prompt to regenerate the `stp/eng/usage-rules.md` document when STP is updated with new features or commands.

## The Prompt

You need to create a usage-rules.md document for the Steel Thread Process (STP) system. This document should follow the pattern established by the Elixir Hex package `usage_rules` (see <https://hexdocs.pm/usage_rules/readme.html>).

### Context

STP is a structured development and documentation system designed for collaboration between developers and Large Language Models (LLMs). The usage-rules.md document should help LLMs understand how to effectively use STP commands and workflows.

### Requirements

1. **Focus on Usage, Not Implementation**
   - Document HOW to use STP, not how it works internally
   - Emphasize workflows and practical patterns
   - Provide clear examples of command usage

2. **Document Structure**
   Follow this structure:
   - Introduction (brief overview of what STP is for)
   - Core Workflows (common patterns for using STP)
   - Command Usage Patterns (practical usage for each command)
   - Steel Thread Workflows (creating and managing steel threads)
   - Task Management Integration (using Backlog.md through STP)
   - LLM Collaboration Patterns (best practices for AI assistance)
   - Further Reading (references to blog posts)

3. **Information Gathering**
   To create the document, you should:
   - Run `stp help` to see all available commands
   - Run `stp help <command>` for each command to understand its usage
   - Review `stp/usr/user_guide.md` for task-based workflows
   - Review `stp/usr/reference_guide.md` for comprehensive command details
   - Check `stp/doc/blog/` directory for conceptual blog posts
   - Look at `CLAUDE.md` for current project conventions

4. **Writing Style**
   - Be concise but comprehensive
   - Use practical examples
   - Focus on patterns and workflows
   - Write for LLM consumption (clear, structured, unambiguous)
   - Include command examples with expected outputs
   - Highlight common mistakes and how to avoid them

5. **Key Patterns to Document**
   - Starting a new project with STP
   - Creating and managing steel threads (now organized as directories)
   - Understanding the new steel thread directory structure (v1.2.1+)
   - Using the task management integration
   - Synchronizing steel thread status with tasks
   - Upgrading STP files to new versions
   - Working with LLMs using STP structure
   - Navigating steel thread files (info.md, design.md, impl.md, tasks.md, results.md)

6. **Blog Post References**
   Include strategic references to these blog posts for deeper understanding:
   - `0000-motivation-for-stp.md` - Why intention matters
   - `0001-introduction-to-stp.md` - Overview of the system
   - `0002-the-steel-thread-methodology.md` - Understanding steel threads
   - `0003-intent-capture-in-software-development.md` - Philosophy behind STP
   - `0004-llm-collaboration-with-stp.md` - Working with AI assistants
   - `0005-getting-started-with-stp.md` - Practical tutorial

7. **Output Location**
   Save the generated document to: `stp/eng/usage-rules.md`

### Example Section Format

Here's an example of how to format a section:

```markdown
## Creating a New Steel Thread

Steel threads are the core unit of work in STP. Here's how to create and manage them effectively:

### Basic Creation

```bash
stp st new "Implement user authentication"
```

This creates a new steel thread with:

- Auto-generated ID (e.g., ST0015)
- Directory structure with separate files:
  - `info.md` - Metadata, objective, and context
  - `design.md` - Design decisions and approach (optional)
  - `impl.md` - Implementation details (optional)
  - `tasks.md` - Task tracking (optional)
  - `results.md` - Results and outcomes (optional)
- Status set to "Not Started"

### Best Practices

1. **Clear Titles**: Use descriptive, action-oriented titles
2. **One Feature Per Thread**: Keep threads focused on single features
3. **Document Intent**: Fill in the info.md file immediately with clear objectives
4. **Use Separate Files**: Leverage the directory structure to organize different aspects
5. **Update Status**: Keep the status field in info.md current as work progresses

### Common Mistakes

- Creating threads that are too broad
- Forgetting to update status as work progresses
- Not linking tasks to threads

### Integration with Tasks

After creating a steel thread, create linked tasks:

```bash
stp task create ST0015 "Design authentication schema"
stp task create ST0015 "Implement login endpoint"
```

For deeper understanding of the steel thread methodology, see the blog post on [The Steel Thread Methodology](../doc/blog/0002-the-steel-thread-methodology.md).

```

Remember: The goal is to help LLMs understand how to use STP effectively in real development scenarios.
