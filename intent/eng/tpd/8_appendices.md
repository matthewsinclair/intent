---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Updated for Intent v2.0.0 (As-Built)"
intent_version: 2.0.0
---
# 8. Appendices

[index](<./technical_product_design.md>)

## 8.1 Glossary

| Term           | Definition                                                                                       |
|----------------|--------------------------------------------------------------------------------------------------|
| LLM            | Large Language Model - An AI system capable of understanding and generating human language,      |
|                | such as Claude, GPT, etc.                                                                        |
| Steel Thread   | A self-contained unit of work that represents a logical piece of functionality to be implemented |
| Context Window | The amount of text an LLM can process in a single interaction                                    |
| Intent         | The system described in this document - captures and preserves development intention             |
| STP            | Steel Thread Process - The original name for Intent (pre-v2.0.0)                                |
| TPD            | Technical Product Design - A comprehensive technical specification document                      |
| Backlog.md     | Task management system integrated with Intent for fine-grained task tracking                     |
| Bootstrap      | Automated setup process for Intent global installation                                           |

## 8.2 Command Reference [AS-BUILT]

### 8.2.1 Primary Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `intent init` | Initialize Intent in a project | `intent init "Project Name"` |
| `intent st` | Manage steel threads | `intent st new/list/show/edit` |
| `intent bl` | Enhanced Backlog.md wrapper | `intent bl list/create/done` |
| `intent task` | Manage tasks linked to threads | `intent task create/list/count` |
| `intent status` | Synchronize thread/task status | `intent status show/sync/check` |
| `intent bootstrap` | Global Intent setup | `intent bootstrap [--force]` |
| `intent doctor` | Diagnose configuration | `intent doctor [--fix]` |
| `intent upgrade` | Migrate from STP | `intent upgrade [--backup-dir]` |
| `intent help` | Display help | `intent help [command]` |
| `intent llm` | LLM integration | `intent llm usage_rules` |

### 8.2.2 Configuration Schema

```json
// .intent/config.json
{
  "version": "2.0.0",              // Required: Intent version
  "project_name": "string",        // Required: Project name
  "author": "string",              // Optional: Default author
  "created": "YYYY-MM-DD",         // Auto-generated: Creation date
  "st_prefix": "ST",               // Optional: Steel thread prefix (default: ST)
  "backlog_dir": "backlog",        // Optional: Backlog directory (default: backlog)
  "intent_dir": "intent",          // Optional: Intent directory (default: intent)
  "backlog_list_status": "todo"    // Optional: Default status filter (default: todo)
}
```

### 8.2.3 Global Configuration

```json
// ~/.config/intent/config.json
{
  "author": "Your Name",           // Default author for all projects
  "editor": "vim",                 // Preferred text editor
  "backlog_list_status": "wip"     // Global default status filter
}
```

### 8.2.4 Command Examples

```bash
# Initialize new project
intent init "My Project"

# Create and manage steel threads
intent st new "Implement OAuth2 authentication"
intent st list --status "In Progress"
intent st show ST0015
intent st edit ST0015 design

# Task management with Backlog integration
intent task create ST0015 "Design auth flow"
intent bl list           # Filtered by config
intent bl list --all     # All tasks
intent bl done task-123

# Status synchronization
intent status show ST0015
intent status sync ST0015

# System maintenance
intent doctor --fix
intent upgrade --backup-dir ./backup
intent bootstrap --force
```

### 8.2.5 Environment Variables

| Variable | Description | Default |
|----------|-------------|----------|
| `INTENT_HOME` | Intent installation directory | Required |
| `AUTHOR` | Default author name | $USER |
| `EDITOR` | Preferred text editor | vim |
| `INTENT_DEBUG` | Enable debug output | unset |
| `INTENT_*` | Override any config value | unset |

Example usage:
```bash
export INTENT_HOME=~/intent
export INTENT_BACKLOG_LIST_STATUS=wip  # Override default filter
export INTENT_DEBUG=1                  # Enable debug mode
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
I'll help you create a new steel thread for user authentication. Let's use Intent:

1. First, let's create the steel thread:
   $ intent st new "User Authentication Implementation"

2. This will create ST0005 directory (assuming your last steel thread was ST0004).

3. Let's define the objective and approach in the steel thread document...
```

## 8.5 References

- POSIX Shell Command Language: [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
- Markdown Guide: [Basic Syntax](https://www.markdownguide.org/basic-syntax/)
- Documentation Best Practices: [Write the Docs](https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/)
- LLM Prompt Engineering: [Claude Prompt Design](https://docs.anthropic.com/claude/docs/introduction-to-prompt-design)
- Shell Scripting Guide: [Bash Hackers Wiki](https://wiki.bash-hackers.org/)

## 8.6 Future Enhancements

- **Web Interface**: A lightweight web interface for viewing Intent documentation
- **Notification System**: Notifications for steel thread status changes
- **Analytics**: Statistics on steel thread progress and completion
- **Integration Plugins**: Deeper integration with version control and issue tracking
- **Templating Extensions**: More sophisticated templating with variables
- **Cross-Project References**: References between related projects
- **Document Generation**: Automatic generation of summary reports
- **Collaborative Editing**: Support for collaborative editing of documents
- **AI Integration**: Enhanced LLM workflows and context management
- **Mobile Support**: Mobile-friendly documentation viewing

## 8.7 Integration References

### 8.7.1 Backlog.md Integration

- **Integration Guide**: `/intent/llm/usage-rules.md#task-management-integration` - Comprehensive guide for using the integration
- **Backlog.md Documentation**: [https://github.com/slune-org/backlog](https://github.com/slune-org/backlog)
- **Integration Tests**:
  - `/tests/unit/task_commands.bats` - Task management command tests
  - `/tests/unit/status_commands.bats` - Status synchronisation tests
  - `/tests/unit/backlog_wrapper.bats` - Wrapper command tests
  - `/tests/integration/` - End-to-end tests
- **Implementation Scripts**:
  - `/bin/intent_backlog` - Backlog wrapper implementation
  - `/bin/intent_task` - Task management implementation
  - `/bin/intent_status` - Status synchronisation implementation
  - `/bin/intent_migrate` - Migration tool implementation

### 8.7.2 Integration Architecture

For technical details on the Backlog.md integration architecture, see:
- Section 3.6.1: Backlog.md Integration Architecture
- Section 4.8.1: Backlog.md Integration Implementation Details
- Blog Post: [LLM Collaboration with Intent](../../docs/blog/0004-llm-collaboration-with-intent.md)

## 8.8 AS-BUILT Notes

### 8.8.1 Version History

| Version | Date | Changes |
|---------|------|---------|  
| 2.0.0 | 2025-07-17 | Complete rebrand to Intent, JSON config, enhanced UX |
| 1.2.1 | 2025-07-09 | Added Backlog.md integration |
| 1.0.0 | 2025-06-15 | Initial STP release |

### 8.8.2 Test Coverage

- 86 tests passing (BATS framework)
- Core functionality: 100% covered
- Integration tests: Backlog.md wrapper
- Lost tests documented for recovery

### 8.8.3 Known Limitations

1. **Reduced test coverage**: ~100 tests lost during migration
2. **Limited error recovery**: Some edge cases need handling
3. **Documentation gaps**: Some advanced features undocumented
4. **Platform testing**: Primarily tested on macOS/Linux
