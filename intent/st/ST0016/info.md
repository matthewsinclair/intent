---
verblock: "16 Jul 2025:v0.2: Matthew Sinclair - Updated with JSON config and new commands"
intent_version: 2.0.0
status: WIP
created: 20250311
completed: 20250708
---
# ST0016: Rename STP CLI to INTENT (v2.0.0)

## Objective

Major refactoring to rename STP to INTENT with clean separation of concerns:

- Tool executables move to top-level bin/
- Project artifacts in intent/ (flattened structure)
- Robust upgrade path with comprehensive testing
- Full backwards compatibility
- New `intent bootstrap` command for easy setup
- New `intent doctor` command for diagnostics

## Context

The STP project has evolved significantly, and we've identified architectural issues that conflate tool executables with project artifacts. This refactoring addresses these concerns by:

1. **Separating tool from usage**: Moving executables to top-level bin/ while keeping project artifacts in intent/
2. **Flattening structure**: Removing unnecessary nesting (prj/st/ becomes st/)
3. **Modern configuration**: JSON-based local/global config system
4. **Robust migration**: Comprehensive upgrade command with testing (fail-forward approach)
5. **Clear naming**: "intent" better reflects the tool's purpose while maintaining ST#### methodology

Key architectural improvements:

- Clean separation between the intent tool (bin/, lib/) and its usage (intent/, backlog/)
- Configurable directory names via JSON configuration
- Full backwards compatibility for existing STP projects
- Model project for testing and examples

## Related Steel Threads

- ST0001: Directory Structure (established initial structure)
- ST0014: Directory Structure for Steel Threads (introduced directory-based STs)
- ST0015: Enhanced Steel Thread Templates (future enhancements)

## Context for LLM

This is a major version 2.0.0 refactoring that renames the CLI from "stp" to "intent" while maintaining full backwards compatibility. The Steel Thread Process methodology remains unchanged (ST#### numbering continues).

Key points:

1. The tool repository itself uses intent on itself (meta usage)
2. bin/ and lib/ are tool components, not project artifacts
3. intent/ and backlog/ are project artifacts from using the tool
4. .intent/config.json is local config, ~/.config/intent/config.json is global (XDG standard)
5. Comprehensive testing via model projects before release

### Implementation Phases

0. **Test Infrastructure**: Create all example projects and test suite FIRST
1. **New Commands**: Implement `intent bootstrap` and `intent doctor`
2. **Configuration System**: JSON-based local/global configs with proper loading hierarchy
3. **Repository Restructuring**: Move executables and flatten directories
4. **Migration Implementation**: Robust upgrade command with backup (no rollback - fail forward)
5. **Command Updates**: Update all subcommands for new structure
6. **Documentation**: Update all references and guides
7. **Bootstrap & Release**: Final testing and v2.0.0 release

See design.md for detailed phase descriptions and impl.md for technical implementation details.
