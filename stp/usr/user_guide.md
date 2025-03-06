---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# User Guide

This user guide provides task-oriented instructions for using the Steel Thread Project (STP) system. It explains how to accomplish common tasks and provides workflow guidance.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Working with Steel Threads](#working-with-steel-threads)
5. [Documentation Management](#documentation-management)
6. [LLM Collaboration](#llm-collaboration)
7. [Troubleshooting](#troubleshooting)

## Introduction

Steel Thread Project (STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs). STP provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

### Purpose

STP helps developers:

- Organize and track development work
- Create and maintain project documentation
- Collaborate effectively with LLMs
- Preserve context across development sessions

### Core Concepts

- **Steel Thread**: A self-contained unit of work focusing on a specific piece of functionality
- **Documentation Structure**: Organized markdown files capturing project information
- **LLM Collaboration**: Patterns for effective work with language models

## Installation

### Prerequisites

- POSIX-compatible shell (bash, zsh)
- Git (optional, for version control)
- Text editor with markdown support

### Installation Steps

1. **Global Installation**:

   ```bash
   # Clone the STP repository
   git clone https://github.com/username/stp.git ~/stp

   # Add STP bin directory to PATH
   echo 'export STP_HOME=~/stp' >> ~/.bashrc
   echo 'export PATH=$PATH:$STP_HOME/bin' >> ~/.bashrc

   # Reload shell configuration
   source ~/.bashrc
   ```

2. **Project-Specific Installation**:

   ```bash
   # From your project directory
   git clone https://github.com/username/stp.git .stp

   # Create a local alias for the project
   alias stp='./.stp/bin/stp'
   ```

## Getting Started

### Initializing a Project

To set up STP in a new or existing project:

```bash
# Navigate to project directory
cd my-project

# Initialize STP
stp init "Project Name"
```

This creates the STP directory structure with template documents.

### Directory Structure

After initialization, you'll have this structure:

```
my-project/
└── doc/                    # Project documentation
    ├── prj/                # Project documentation
    │   ├── st/             # Steel threads
    │   ├── wip.md          # Work in progress
    │   └── journal.md      # Project journal
    ├── eng/                # Engineering docs
    │   └── tpd/            # Technical Product Design
    ├── usr/                # User documentation
    └── llm/                # LLM-specific content
```

## Working with Steel Threads

### Creating a Steel Thread

To create a new steel thread:

```bash
stp st new "Implement Feature X"
```

This creates a new steel thread document (e.g., `prj/st/ST0001.md`) and adds it to the index.

### Viewing Steel Threads

To list all steel threads:

```bash
stp st list
```

To view a specific steel thread:

```bash
stp st show ST0001
```

### Completing a Steel Thread

When all tasks in a steel thread are done:

```bash
stp st done ST0001
```

This updates the status and completion date.

## Documentation Management

[Instructions for working with and maintaining documentation]

## LLM Collaboration

[Guidelines for effectively collaborating with LLMs]

## Troubleshooting

[Common issues and their solutions]

---

# Context for LLM

This document template is for creating a user guide explaining how to use the STP system. When implementing this guide:

1. Replace placeholder sections with specific, detailed instructions
2. Include examples for common workflows
3. Add screenshots or diagrams if helpful
4. Ensure all commands and paths are accurate
5. Include troubleshooting information for common issues

The final user guide should be comprehensive but accessible, aimed at helping users get started with the STP system and use it effectively.
