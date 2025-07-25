---
verblock: "09 Jul 2025:v0.4: Matthew Sinclair - Updated for steel thread directory structure"
stp_version: 1.2.1
---
# Technical Product Design

## Preamble to Claude

This document is a Technical Product Design (TPD) for the Steel Thread Process (STP) system. When processing this document, please understand:

1. This is a comprehensive technical specification for a shell-script and markdown-based system
2. The system is designed to facilitate collaboration between developers and LLMs
3. The document contains:
   - System architecture and design principles
   - Process descriptions
   - Template structures
   - Implementation details for existing code
   - Future development plans

4. The code is developed through "steel threads" which are incremental implementation stages
5. Steel threads are now organized as directories (v1.2.1+) containing multiple files:
   - info.md: Main information and metadata
   - design.md: Design decisions and approach
   - impl.md: Implementation details
   - tasks.md: Task tracking (or linked to Backlog)
   - results.md: Results and outcomes
6. The system consists primarily of shell scripts and markdown templates
7. The system is designed to integrate with existing development workflows
8. The system integrates with Backlog.md for fine-grained task management while maintaining separation of concerns
9. Key commands include:
   - `stp st list`: List all steel threads with optional filtering by status
   - `stp st sync`: Synchronize the steel_threads.md index with individual ST directories
   - `stp st show ST0001 design`: Show specific file from steel thread directory
   - `stp st edit ST0001 impl`: Edit specific file from steel thread directory
   - `stp st organize`: Organize steel thread directories by status
   - `stp upgrade`: Upgrade STP files to the latest format and standards
   - `stp bl`: Wrapper for Backlog.md commands to avoid git errors
   - `stp task`: Manage Backlog tasks linked to steel threads
   - `stp status`: Synchronize steel thread status based on task completion
   - `stp migrate`: Migrate embedded tasks from steel threads to Backlog
   - `stp llm usage_rules`: Display usage patterns and workflows for LLMs

# Steel Thread Process Technical Product Design

This document serves as the central index for the Technical Product Design (TPD) of the Steel Thread Process (STP) system. The TPD is organized into sections that detail the architecture, implementation, and roadmap for the system.

## Table of Contents

1. [Introduction](./1_introduction.md)
2. [Requirements](./2_requirements.md)
3. [Architecture](./3_architecture.md)
4. [Detailed Design](./4_detailed_design.md)
5. [Implementation Strategy](./5_implementation_strategy.md)
6. [Deployment and Operations](./6_deployment_and_operations.md)
7. [Technical Challenges and Mitigations](./7_technical_challenges_and_mitigations.md)
8. [Appendices](./8_appendices.md)

## Links

[Steel Threads](../../prj/st/steel_threads.md)
