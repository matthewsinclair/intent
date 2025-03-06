---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# help

@short:
Display help information for STP commands

@desc:
The 'help' command provides detailed documentation for STP commands
and features. It can be used to display general usage information
or to get specific help for individual commands.

When used without arguments, it shows a summary of all available commands.
When a command name is provided, it displays detailed help for that command.

@usage:
stp help [command]

Arguments:
  command   The name of the command to get help for (optional)

Examples:
  stp help          # Display general help
  stp help init     # Show help for the 'init' command
  stp help st       # Show help for the 'st' command