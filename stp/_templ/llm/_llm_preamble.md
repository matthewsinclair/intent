---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# LLM Preamble

This document provides context and instructions for the LLM (Large Language Model) when working on this project. It should be shared with the LLM at the beginning of each session.

## Project Context

[Brief description of the project, its purpose, and current status]

## Steel Thread Process

This project follows the Steel Thread Project (STP) methodology, which structures development into discrete "steel threads" - self-contained units of work. Key aspects of this process include:

1. **Work Organization**: Development is organized into steel threads (ST####)
2. **Documentation**: All work is documented through markdown files in a specific structure
3. **Incremental Progress**: Work proceeds incrementally, with clear tracking of progress

## Document Structure

Documentation is organized into these key directories:

- **doc/prj/**: Project documentation
  - **doc/prj/wip.md**: Current work in progress
  - **doc/prj/journal.md**: Historical record of project activities
  - **doc/prj/st/**: Steel thread documents
- **doc/eng/**: Engineering documentation
  - **doc/eng/tpd/**: Technical Product Design documents
- **doc/usr/**: User documentation
  - **doc/usr/user_guide.md**: User guide
  - **doc/usr/reference_guide.md**: Reference guide
  - **doc/usr/deployment_guide.md**: Deployment guide
- **doc/llm/**: LLM-specific content
  - **doc/llm/llm_preamble.md**: This document
  - **doc/llm/*.prompt.md**: Canned prompts for reuse

## Code Style and Conventions

[Define code style and conventions for the project, including:]

- **Programming Language Style**: [Language-specific conventions]
- **Markdown Style**: Consistent headings; clear sections; numbered lists for steps
- **Comments**: Include purpose at file top; comment complex logic
- **Variable Naming**: [Conventions for variable naming]
- **Indentation**: [Number of spaces/tabs for indentation]
- **Error Handling**: [Error handling conventions]

## Collaboration Guidelines

When collaborating on this project, please:

1. **Maintain Documentation**: Update relevant documentation as work progresses
2. **Focus on Current Work**: Prioritize tasks in the current steel thread
3. **Provide Explanations**: Explain your reasoning for design decisions
4. **Suggest Improvements**: Offer improvements to the process, code, or documentation
5. **Ask Questions**: Request clarification when objectives or approaches are unclear

## Command Usage

The project includes several STP commands:

- `stp init <project_name> [directory]`: Initialize STP in a project
- `stp st new <title>`: Create a new steel thread
- `stp st done <id>`: Mark a steel thread as complete
- `stp st list [--status <status>]`: List steel threads
- `stp help [command]`: Display help information

## Getting Started

When beginning a new session:

1. Review the current WIP document (doc/prj/wip.md)
2. Check active steel threads in the index (doc/prj/st/steel_threads.md)
3. Focus on the specific steel thread indicated in the WIP

[Additional project-specific instructions or guidelines]
