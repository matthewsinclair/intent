---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# init

@short:
Initialize STP in a project

@desc:
The 'init' command sets up the Steel Thread Project (STP) structure in a
new or existing project. It creates the necessary directory structure,
initializes template documents, and configures the project for STP use.

This command should be run once at the beginning of a project or when
adding STP to an existing project.

@usage:
stp init <project_name> [directory]

Arguments:
  project_name  Name of the project (required)
  directory     Target directory (optional, defaults to current directory)

The command creates the following directory structure:
  prj/                # Project documentation
    st/               # Steel threads
    wip.md            # Work in progress
    journal.md        # Project journal
  eng/                # Engineering docs
    tpd/              # Technical Product Design
  usr/                # User documentation
  llm/                # LLM-specific content
  _templ/             # Templates
  bin/                # STP scripts

Examples:
  stp init "My Project"                 # Initialize in current directory
  stp init "My Project" ./my-project    # Initialize in specified directory
