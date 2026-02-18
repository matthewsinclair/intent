# Implementation - ST0006: Help System

## Implementation Notes

### Help File Structure

Help files are stored in the `.help` directory and follow a consistent format:

- Filename pattern: `command.help.md`
- Structure:

  ```
  @short: Brief one-line description
  @desc:
  Detailed multi-line description

  @usage:
  command [options] <arguments>

  @examples:
  command example1
  command example2
  ```

### Help Command Implementation

The `stp help` command:

1. Without arguments, displays a list of all available commands with short descriptions
2. With a command argument (eg `stp help init`), displays detailed help for that command
3. Dynamically discovers available commands by scanning for `stp_*` scripts

### Dynamic Command Discovery

The system:

- Scans the bin directory for `stp_*` scripts to identify available commands
- Extracts short descriptions from corresponding help files
- Formats the output in a consistent and readable way
- Handles the case of missing help files gracefully

### Multi-line Description Handling

The help system properly formats multi-line descriptions, maintaining:

- Proper indentation
- Paragraph structure
- Code blocks and examples
- Lists and other formatting
