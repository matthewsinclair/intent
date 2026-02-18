---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.0
---

# 2. Requirements

[index](./technical_product_design.md)

## 2.1 Functional Requirements

### 2.1.1 Documentation Management

| ID     | Requirement                                                                |
| ------ | -------------------------------------------------------------------------- |
| FR-1.1 | The system shall provide templates for all required documentation types    |
| FR-1.2 | The system shall maintain project history and work-in-progress tracking    |
| FR-1.3 | The system shall support steel thread creation, management, and completion |
| FR-1.4 | All documentation shall be in markdown format for maximum portability      |

### 2.1.2 LLM Collaboration

| ID     | Requirement                                                        |
| ------ | ------------------------------------------------------------------ |
| FR-2.1 | The system shall provide context management for LLM interactions   |
| FR-2.2 | The system shall support canned prompts for common LLM tasks       |
| FR-2.3 | The system shall facilitate passing context between LLM sessions   |
| FR-2.4 | The system shall include LLM-specific instructions for consistency |

### 2.1.3 Process Support

| ID     | Requirement                                                                  |
| ------ | ---------------------------------------------------------------------------- |
| FR-3.1 | The system shall support initialisation of STP within existing projects      |
| FR-3.2 | The system shall provide commands for all common STP workflow operations     |
| FR-3.3 | The system shall track completion status of steel threads                    |
| FR-3.4 | The system shall maintain independence from specific version control systems |

### 2.1.4 Backlog Integration [AS-BUILT]

| ID     | Requirement                                                                | Status        |
| ------ | -------------------------------------------------------------------------- | ------------- |
| FR-4.1 | The system shall integrate with Backlog.md for task management             | ✓ Implemented |
| FR-4.2 | The system shall provide wrapper commands to avoid direct Backlog.md usage | ✓ Implemented |
| FR-4.3 | The system shall link tasks to steel threads using naming conventions      | ✓ Implemented |
| FR-4.4 | The system shall synchronize steel thread status based on task completion  | ✓ Implemented |
| FR-4.5 | The system shall support migration of embedded tasks to Backlog.md         | ✓ Implemented |
| FR-4.6 | The system shall support configurable backlog_list_status filtering        | ✓ Implemented |

### 2.1.5 Configuration Management [AS-BUILT]

| ID     | Requirement                                                       | Status        |
| ------ | ----------------------------------------------------------------- | ------------- |
| FR-5.1 | The system shall support project-specific configuration           | ✓ Implemented |
| FR-5.2 | The system shall provide sensible defaults for all configurations | ✓ Implemented |
| FR-5.3 | The system shall validate configurations on startup               | ✓ Implemented |
| FR-5.4 | The system shall support environment variable overrides           | ✓ Implemented |
| FR-5.5 | Configuration shall use JSON format (.intent/config.json)         | ✓ Implemented |
| FR-5.6 | Configuration shall support hierarchy (env→local→global→default)  | ✓ Implemented |

## 2.2 Non-Functional Requirements

### 2.2.1 Usability

| ID      | Requirement                                                   |
| ------- | ------------------------------------------------------------- |
| NFR-1.1 | The system shall be usable with minimal training              |
| NFR-1.2 | The system shall provide clear documentation for all commands |
| NFR-1.3 | The system shall integrate with existing developer workflows  |

### 2.2.2 Performance

| ID      | Requirement                                                        |
| ------- | ------------------------------------------------------------------ |
| NFR-2.1 | The system shall have minimal impact on development performance    |
| NFR-2.2 | The system shall optimise context usage for LLM interactions       |
| NFR-2.3 | Commands shall complete within reasonable time frames (<2 seconds) |

### 2.2.3 Compatibility

| ID      | Requirement                                                                           |
| ------- | ------------------------------------------------------------------------------------- |
| NFR-3.1 | The system shall be compatible with common shell environments (bash, zsh)             |
| NFR-3.2 | The system shall function on major operating systems (Linux, macOS, Windows with WSL) |
| NFR-3.3 | The system shall not interfere with or depend on specific development tools           |

### 2.2.4 Maintainability

| ID      | Requirement                                                    |
| ------- | -------------------------------------------------------------- |
| NFR-4.1 | The system shall be self-contained within project repositories |
| NFR-4.2 | The system shall support upgrading to newer STP versions       |
| NFR-4.3 | The system shall be extensible for project-specific needs      |

## 2.3 Constraints

| ID    | Constraint                                                                           |
| ----- | ------------------------------------------------------------------------------------ |
| CON-1 | The system must use only shell scripts and markdown for maximum portability          |
| CON-2 | The system must not require external dependencies beyond common shell utilities      |
| CON-3 | The system must be agnostic to LLM platforms while supporting specific optimisations |
| CON-4 | The system must respect the context window limitations of LLMs                       |
| CON-5 | [AS-BUILT] The system requires jq for JSON configuration parsing                     |

## 2.4 AS-BUILT Notes

All original requirements have been met or exceeded in Intent v2.0.0. Key additions include:

1. **Enhanced Configuration**: JSON-based configuration with hierarchy support
2. **New Commands**: bootstrap, doctor, upgrade for better user experience
3. **Status Filtering**: Configurable backlog_list_status for focused task views
4. **Self-Hosting**: Intent is developed using Intent itself
5. **Migration Tools**: Comprehensive upgrade path from any STP version
6. **Flattened Structure**: Simplified directory layout under intent/
7. **Command Naming**: Consistent intent\_\* naming pattern
