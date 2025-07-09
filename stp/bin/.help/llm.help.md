@short:
Commands for LLM integration and assistance

@description:
The llm command provides utilities for working with Large Language Models (LLMs)
in the context of STP. It includes tools to help LLMs understand how to use STP
effectively and to facilitate better collaboration between developers and AI assistants.

@usage:
stp llm <subcommand> [options]

@subcommands:
usage_rules    Display the STP usage rules document

@options:
--symlink [dir]    Create a symlink to usage-rules.md (for usage_rules subcommand)

@details:
The usage_rules subcommand displays the complete STP usage patterns and workflows
documentation. This document is designed specifically for LLMs to understand:

- How to use STP commands effectively
- Common workflows and patterns
- Best practices for steel thread management
- Task integration with Backlog.md
- LLM collaboration patterns

The usage rules follow the pattern established by the Elixir Hex package 'usage_rules'
and provide comprehensive guidance for AI-assisted development with STP.

@examples:
# Display usage rules for LLMs
stp llm usage_rules

# Create symlink in current directory
stp llm usage_rules --symlink

# Create symlink in specific directory
stp llm usage_rules --symlink ~/my-project

# Pipe to a pager for easier reading
stp llm usage_rules | less

# Save to a file for reference
stp llm usage_rules > stp-usage-rules.md

@notes:
- The usage rules document is located at stp/eng/usage-rules.md
- It can be regenerated using the prompt at stp/eng/prompts/regenerate_usage_rules.md
- LLMs should read this document to understand STP workflows
- The --symlink option creates a symlink named 'usage-rules.md' following the Hex package convention