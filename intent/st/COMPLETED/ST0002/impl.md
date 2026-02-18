# Implementation - ST0002: Core Script Framework

## Implementation Notes

### Core Script Design

The core script framework follows a modular design where:

1. The main `stp` script:
   - Validates input parameters
   - Determines the STP_HOME directory
   - Dispatches to the appropriate command implementation
   - Handles basic error conditions

2. Command implementations:
   - Each command is implemented in a separate script named `stp_<command>`
   - Commands receive parameters directly from the main script
   - Commands handle their own parameter validation
   - Commands provide specific error messages

3. Help system:
   - Implemented in `stp_help`
   - Reads help documentation from `.help` directory
   - Provides both general and command-specific help

### Script Environment

The scripts use environment variables to maintain configuration:

- `STP_HOME`: Location of the STP installation
- `STP_PROJECT`: Current project name
- `STP_AUTHOR`: Default author name
- `STP_EDITOR`: Preferred text editor

The main script can determine `STP_HOME` automatically if not set.

### Error Handling

Error handling follows these principles:

- Exit with non-zero status on error
- Provide clear error messages to stderr
- Check prerequisites before operations
- Validate input parameters
- Handle script permissions issues

### Help Documentation

Help documentation follows a standard format with sections:

- `@short`: Brief one-line description
- `@desc`: Detailed description
- `@usage`: Usage information, parameters, and examples
