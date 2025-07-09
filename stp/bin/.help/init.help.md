---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# init

@short:
Initialize STP in a project

@desc:
The 'init' command sets up the Steel Thread Process (STP) structure in a
new or existing project. It creates the necessary directory structure,
initializes template documents, and configures the project for STP use.

This command should be run once at the beginning of a project or when
adding STP to an existing project.

@usage:
stp init [options] <project_name> [directory]

Options:
  -d, --dirs    Comma-separated list of directories to copy (default: eng,llm,prj,usr)
  -a, --all     Copy all directories, including bin, _templ, tests

Arguments:
  project_name  Name of the project (required)
  directory     Target directory (optional, defaults to current directory)

The command creates the following directory structure by default:
  prj/                # Project documentation
    st/               # Steel threads
    wip.md            # Work in progress
  eng/                # Engineering docs
    tpd/              # Technical Product Design
  usr/                # User documentation
  llm/                # LLM-specific content

When using the --all option or specifying with --dirs, additional directories may be included:
  _templ/             # Templates (only with --all or --dirs "_templ")
  bin/                # STP scripts (only with --all or --dirs "bin")
  tests/              # Tests (only with --all or --dirs "tests")

Examples:
  stp init "My Project"                                # Initialize with default directories
  stp init "My Project" ./my-project                   # Initialize in specified directory
  stp init --dirs "eng,llm,prj,usr,bin" "My Project"   # Specify which directories to include
  stp init --all "My Project"                          # Include all directories
