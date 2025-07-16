---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# LLM Preamble

This document provides essential context for LLMs working on the [[PROJECT_NAME]] project. Share this document at the beginning of each LLM session to establish baseline understanding.

## Project Context

[[PROJECT_NAME]] follows the Steel Thread Process (STP) methodology, which organizes development into discrete "steel threads" - self-contained units of functionality that enable incremental progress with clear documentation.

## Navigation Guide

When working with this repository, you should focus on these key documents in this specific order:

1. **START HERE**: `stp/eng/tpd/technical_product_design.md` - Contains comprehensive information about the project vision, architecture, and current state.

2. **NEXT**: `stp/prj/st/steel_threads.md` - Provides a complete index of all steel threads with their status. Review this to understand what work has been completed and what remains.

3. **THEN**: `stp/prj/wip.md` - Details the current work in progress and priorities. This is your guide to what should be worked on now.

4. **FINALLY**: Use `stp bl list` and steel thread documents to review historical work completed. Backlog tasks provide detailed progress tracking.

## Documentation Structure

The STP methodology organizes project information through a specific directory structure:

- **stp/prj/**: Project management documents
  - **stp/prj/wip.md**: Current work in progress
  - **Backlog tasks**: Historical record of project activities and progress
  - **stp/prj/st/**: Steel thread documents and index
- **stp/eng/**: Engineering documentation
  - **stp/eng/tpd/**: Technical Product Design documents
- **stp/usr/**: User documentation
  - **stp/usr/user_guide.md**: End-user instructions
  - **stp/usr/reference_guide.md**: Complete feature reference∏
  - **stp/usr/deployment_guide.md**: Deployment instructions
- **stp/llm/**: LLM-specific content
  - **stp/llm/llm_preamble.md**: This document

## Steel Thread Process

Work in this project is organized through steel threads:

1. **Definition**: A steel thread is a self-contained unit of work that represents a logical piece of functionality
2. **Workflow**:
   - Steel threads start as "Not Started"
   - When work begins, they move to "In Progress"
   - When completed, they are marked as "Completed"
   - They can also be "On Hold" or "Cancelled" as needed
3. **Documentation**: Each steel thread has its own markdown document in `stp/prj/st/`
4. **Management**: Steel threads are created, tracked, and completed using STP commands

## Code Style and Conventions

The following code style guidelines apply to this project:

- **Indentation**: Use 2-space indentation in all programming languages
- **Documentation**: Add clear documentation for all code components
- **Naming**: Use descriptive variable and function names
- **Error Handling**: Implement robust error handling according to language best practices
- **Testing**: Include appropriate tests for all new functionality
- **Markdown**: Maintain consistent formatting in all markdown documents

[Add specific code style guidelines for the project's primary programming languages]

## Command Usage

The STP system provides these commands for project management:

- `stp init <project_name> [directory]`: Initialize STP in a project
- `stp st new <title>`: Create a new steel thread
- `stp st done <id>`: Mark a steel thread as complete
- `stp st list [--status <status>]`: List all steel threads
- `stp st show <id>`: Display details of a specific steel thread
- `stp st edit <id>`: Open a steel thread in your default editor
- `stp help [command]`: Display help information

## How to Help

When assisting with this project, you should:

1. First, understand the current context by reviewing the documents in the order specified
2. Focus on the work in progress as defined in `stp/prj/wip.md`
3. Maintain consistency with existing patterns and documentation standards
4. Update documentation alongside code changes
5. Use the steel thread model to organize new work
6. Update task status in Backlog to track progress

[Add any project-specific collaboration guidelines here]

## Project-Specific Information

[Add essential project-specific information here that doesn't fit elsewhere, such as:

- Key technologies used
- External dependencies
- Special development setup instructions
- Important architectural principles
- Known limitations or considerations]
