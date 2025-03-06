---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# 8. Appendices

## 8.1 Glossary

| Term           | Definition                                                                                       |
|----------------|--------------------------------------------------------------------------------------------------|
| LLM            | Large Language Model - An AI system capable of understanding and generating human language,      |
|                | such as Claude, GPT, etc.                                                                        |
| Steel Thread   | A self-contained unit of work that represents a logical piece of functionality to be implemented |
| Context Window | The amount of text an LLM can process in a single interaction                                    |
| STP            | Steel Thread Project - The system described in this document                                     |
| Canned Prompt  | A pre-defined, reusable instruction template for an LLM                                          |
| TPD            | Technical Product Design - A comprehensive technical specification document                      |

## 8.2 Script Reference

### 8.2.1 Core Script (`stp`)

```bash
#!/bin/bash
# STP - Steel Thread Project main script
# Usage: stp <command> [options] [arguments]

# Check if STP_HOME is set
if [ -z "$STP_HOME" ]; then
  # Determine STP_HOME from script location
  STP_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  export STP_HOME
fi

# Check command
if [ $# -eq 0 ]; then
  $STP_HOME/bin/stp_help
  exit 1
fi

COMMAND="$1"
shift

# Check if command script exists
COMMAND_SCRIPT="$STP_HOME/bin/stp_$COMMAND"
if [ ! -f "$COMMAND_SCRIPT" ]; then
  echo "Error: Unknown command '$COMMAND'"
  $STP_HOME/bin/stp_help
  exit 1
fi

# Execute command
$COMMAND_SCRIPT "$@"
```

### 8.2.2 Help Script (`stp_help`)

```bash
#!/bin/bash
# STP Help script
# Usage: stp_help [command]

# Check if STP_HOME is set
if [ -z "$STP_HOME" ]; then
  echo "Error: STP_HOME environment variable is not set"
  exit 1
fi

# Display command-specific help
if [ $# -eq 1 ]; then
  COMMAND="$1"
  HELP_FILE="$STP_HOME/.help/$COMMAND.help.md"
  
  if [ -f "$HELP_FILE" ]; then
    cat "$HELP_FILE"
  else
    echo "Error: No help available for command '$COMMAND'"
    exit 1
  fi
  exit 0
fi

# Display general help
echo "STP - Steel Thread Project"
echo ""
echo "Usage: stp <command> [options] [arguments]"
echo ""
echo "Available commands:"
echo "  init    Initialize STP in a project"
echo "  st      Manage steel threads"
echo "  help    Display help information"
echo ""
echo "For more information on a specific command, run:"
echo "  stp help <command>"
```

### 8.2.3 Init Script (`stp_init`)

```bash
#!/bin/bash
# STP Init script
# Usage: stp_init <project_name> [directory]

# Check arguments
if [ $# -lt 1 ]; then
  echo "Error: Project name is required"
  echo "Usage: stp init <project_name> [directory]"
  exit 1
fi

PROJECT_NAME="$1"
TARGET_DIR="${2:-.}"

# Create directory structure
mkdir -p "$TARGET_DIR"/{prj/st,eng/tpd,usr,llm,_templ}

# Copy templates
cp -r "$STP_HOME"/_templ/* "$TARGET_DIR"/_templ/

# Create initial files
# ...

echo "STP initialized for project: $PROJECT_NAME in $TARGET_DIR"
```

### 8.2.4 Steel Thread Script (`stp_st`)

```bash
#!/bin/bash
# STP Steel Thread script
# Usage: stp_st <command> [options] [arguments]

# Check arguments
if [ $# -lt 1 ]; then
  echo "Error: Steel thread command is required"
  echo "Usage: stp st <command> [options] [arguments]"
  echo "Commands: new, done, list, show"
  exit 1
fi

ST_COMMAND="$1"
shift

case "$ST_COMMAND" in
  "new")
    # Create new steel thread
    # ...
    ;;
  "done")
    # Mark steel thread as done
    # ...
    ;;
  "list")
    # List steel threads
    # ...
    ;;
  "show")
    # Show steel thread details
    # ...
    ;;
  *)
    echo "Error: Unknown steel thread command '$ST_COMMAND'"
    exit 1
    ;;
esac
```

## 8.3 Template Examples

### 8.3.1 Work in Progress Template

```markdown
# Work In Progress

## Current Focus
[Brief description of the current development focus]

## Active Steel Threads
- ST####: [Brief description]
- ...

## Upcoming Work
- [Item 1]
- ...

## Notes
[Any additional notes about the current work]
```

### 8.3.2 Steel Thread Template

```markdown
# ST####: [Title]

- **Status**: [Not Started|In Progress|Completed]
- **Created**: YYYY-MM-DD
- **Completed**: YYYY-MM-DD

## Objective
[Clear statement of what this steel thread aims to accomplish]

## Context
[Background information and context for this steel thread]

## Approach
[Planned approach for implementing this steel thread]

## Tasks
- [ ] Task 1
- [ ] Task 2
- ...

## Implementation Notes
[Notes on implementation details, decisions, challenges, etc.]

## Results
[Summary of results after completion]
```

## 8.4 Process Examples

### 8.4.1 New Steel Thread Process

1. **Identify Need**: Developer identifies a discrete piece of work
2. **Create Steel Thread**: `stp st new "Implement Feature X"`
3. **Define Objective**: Update steel thread document with clear objective
4. **Implement Tasks**: Work through tasks with LLM assistance
5. **Document Progress**: Update documentation as work progresses
6. **Complete Steel Thread**: `stp st done ST####`

### 8.4.2 LLM Collaboration Example

**Developer Prompt:**

```
I need to implement a new feature for user authentication. Let's create a new steel thread for this.
```

**LLM Response:**

```
I'll help you create a new steel thread for user authentication. Let's use the STP process:

1. First, let's create the steel thread:
   $ stp st new "User Authentication Implementation"

2. This will create ST0005.md (assuming your last steel thread was ST0004).

3. Let's define the objective and approach in the steel thread document...
```

## 8.5 References

- POSIX Shell Command Language: [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
- Markdown Guide: [Basic Syntax](https://www.markdownguide.org/basic-syntax/)
- Documentation Best Practices: [Write the Docs](https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/)
- LLM Prompt Engineering: [Claude Prompt Design](https://docs.anthropic.com/claude/docs/introduction-to-prompt-design)
- Shell Scripting Guide: [Bash Hackers Wiki](https://wiki.bash-hackers.org/)

## 8.6 Future Enhancements

- **Web Interface**: A lightweight web interface for viewing STP documentation
- **Notification System**: Notifications for steel thread status changes
- **Analytics**: Statistics on steel thread progress and completion
- **Integration Plugins**: Deeper integration with version control and issue tracking
- **Templating Extensions**: More sophisticated templating with variables
- **Cross-Project References**: References between related projects
- **Document Generation**: Automatic generation of summary reports
- **Collaborative Editing**: Support for collaborative editing of documents
