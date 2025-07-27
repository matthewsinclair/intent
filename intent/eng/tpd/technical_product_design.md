---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Complete update for Intent v2.0.0 (As-Built)"
intent_version: 2.0.0
---
# Intent Technical Product Design v2.0.0 (As-Built)

## Preamble to Claude

This document is a Technical Product Design (TPD) for the Intent system (formerly known as STP - Steel Thread Process). When processing this document, please understand:

1. This is the AS-BUILT documentation for Intent v2.0.0, reflecting the actual implementation
2. Intent underwent a complete rebrand from STP to Intent in July 2025
3. The system is designed to facilitate collaboration between developers and LLMs
4. This document contains:
   - Actual v2.0.0 architecture and implementation
   - JSON-based configuration system
   - Complete command reference for intent_* commands
   - Migration tools and processes
   - Lessons learned from development

5. The code is developed through "steel threads" which are incremental implementation stages
6. Steel threads are organized as directories under intent/st/ containing:
   - info.md: Main information and metadata (required)
   - design.md: Design decisions and approach (optional)
   - impl.md: Implementation details (optional)
   - tasks.md: Task tracking or Backlog.md integration (optional)
7. The system consists of shell scripts and markdown templates
8. Configuration uses JSON format (.intent/config.json) instead of YAML
9. The system integrates with Backlog.md for task management with enhanced filtering
10. Intent is self-hosting - this project is built using Intent v2.0.0
11. Key commands include:

- `intent st list`: List all steel threads with status filtering
- `intent st new`: Create a new steel thread
- `intent st show`: Display steel thread contents
- `intent st edit`: Edit steel thread files
- `intent bl`: Enhanced Backlog.md wrapper with status filtering
- `intent task`: Manage Backlog tasks linked to steel threads
- `intent status`: Synchronize steel thread status with task completion
- `intent init`: Initialize a new Intent project
- `intent bootstrap`: Global Intent setup and configuration
- `intent doctor`: Diagnose and fix configuration issues
- `intent upgrade`: Migrate any STP version to Intent v2.0.0
- `intent help`: Unified help system for all commands

# Intent v2.0.0 Technical Product Design

This document serves as the central index for the Technical Product Design (TPD) of Intent v2.0.0. The TPD has been forensically updated to reflect the actual as-built state of the system after the migration from STP to Intent. Sections marked with "[AS-BUILT]" indicate deviations from the original design.

## Table of Contents

1. [Introduction](./1_introduction.md)
2. [Requirements](./2_requirements.md)
3. [Architecture](./3_architecture.md)
4. [Detailed Design](./4_detailed_design.md)
5. [Implementation Strategy](./5_implementation_strategy.md)
6. [Deployment and Operations](./6_deployment_and_operations.md)
7. [Technical Challenges and Mitigations](./7_technical_challenges_and_mitigations.md)
8. [Appendices](./8_appendices.md)

## Migration Notes

Intent v2.0.0 represents a complete rebrand and restructuring from the Steel Thread Process (STP) to Intent:

- Directory structure flattened: `stp/prj/st/` → `intent/st/`
- Commands renamed: `stp_*` → `intent_*`
- Configuration migrated: YAML → JSON
- New features: bootstrap, doctor, upgrade commands
- Enhanced Backlog.md integration with status filtering
- Self-hosting success: Intent is built using Intent

## Current Status

- **Version**: 2.0.0 (Released July 2025)
- **Tests**: 86/86 passing (reduced from 186 during migration)
- **Commands**: 12 primary commands with full functionality
- **Projects Using Intent**: Intent itself (self-hosting)

## Links

- [Current Steel Threads](../../st/)
- [Intent Blog Series](../../../docs/blog/)
- [Migration Guide](./6_deployment_and_operations.md#migration)
