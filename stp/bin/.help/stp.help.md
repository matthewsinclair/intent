# stp

@short:
Steel Thread Process - A system for structured development and documentation with LLM collaboration

@desc:
STP (Steel Thread Process) provides a structured process for developing software
in collaboration with Large Language Models (LLMs). It helps manage documentation,
track progress, and maintain context across development sessions.

STP organizes work into "steel threads" - self-contained units of work that
focus on implementing specific pieces of functionality. It provides templates,
scripts, and process guidelines to enhance productivity while ensuring
high-quality documentation.

@usage:
stp <command> [options] [arguments]

Commands:
  init    Initialize STP in a project
  st      Manage steel threads
  help    Display help information

Examples:
  stp init "My Project"         # Initialize STP in the current directory
  stp st new "Implement Auth"   # Create a new steel thread
  stp help st                   # Display help for the 'st' command
