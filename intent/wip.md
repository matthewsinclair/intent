---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.0
---
# Work In Progress

This file serves as a placeholder for kicking off new sessions.

See the following files for detailed information about the project:

- [Technical Product Design](../eng/tpd/technical_product_design.md)
- [Steel Threads Overview](st/steel_threads.md)

Read CLAUDE.md then wait for instruction.

#### Todo

  Detailed Restart Prompt for Intent v2.0.0 Implementation (ST0016)

  Session Context

  You are continuing work on ST0016: Rename STP CLI to INTENT (v2.0.0). This is a major refactoring that has made significant
  progress but needs completion.

  Project Location: /Users/matts/Devel/prj/IntentCurrent Branch: mainLast Working State: Tests passing (83/83), repository
  restructured, new commands implemented

  Current Implementation Status

  ✅ Completed (Phases 0-2)

  1. Repository Structure:
    - Executables moved from stp/bin/ to top-level bin/
    - All scripts renamed from stp_*to intent_*
    - Backward compatibility symlink stp → intent created
    - Project now uses Intent v2.0.0 structure
  2. New Commands Implemented:
    - intent bootstrap - Sets up global config at ~/.config/intent/config.json
    - intent doctor - Configuration diagnostics with --fix mode
    - intent upgrade - Migrates STP projects to Intent v2.0.0
  3. Configuration System:
    - JSON-based configs (.intent/config.json local, ~/.config/intent/config.json global)
    - Config loading hierarchy: env vars → local → global → defaults
    - No external dependencies (pure bash JSON parsing)
  4. Test Infrastructure:
    - Examples directory with test projects (v0.0.0, v1.2.0, v1.2.1, hello-world)
    - BATS test suite with 83 passing tests
    - Test helper infrastructure (tests/lib/test_helper.bash)

  ⏳ In Progress (Phase 3)

  Current Tasks (from backlog):

- task-69: Test migrations on example projects
- task-70: Execute self-migration to new structure

  ❌ Not Started (Phases 4-7)

- Command updates for remaining subcommands
- Documentation updates (README, migration guide)
- Bootstrap and release preparation

  Critical Information

  Directory Structure (Current)

  /Users/matts/Devel/prj/Intent/
  ├── .intent/              # Local config
  │   └── config.json
  ├── bin/                  # Executables (moved from stp/bin/)
  │   ├── intent
  │   ├── intent_*
  │   └── stp -> intent
  ├── intent/               # Project artifacts (was stp/)
  │   ├── st/              # Steel threads
  │   ├── docs/
  │   └── llm/
  ├── backlog/             # Backlog.md integration
  ├── examples/            # Test projects
  ├── tests/               # BATS test suite
  └── CLAUDE.md            # Project instructions

  Known Issues

  1. Backlog Task Status: Many completed tasks still marked as "todo" in backlog
  2. Test Coverage: Reduced from 186 to 83 tests (see tests/RECOVERY_PLAN.md)
  3. Git History: Shows "borked" commits during Phase 2, but latest state is working

  Key Implementation Notes

  1. Fail-Forward Approach: No rollback mechanisms - fix issues as they arise
  2. Self-Hosting: This project uses Intent on itself (meta usage)
  3. JSON Parsing: Using sed/grep, no jq dependency
  4. Error Handling: All version detection must fail with clear messages
  5. Testing First: Always write tests before implementation

  Your Mission

  Primary Objectives

  1. Complete Phase 3: Test migrations on all example projects

# Run migration tests

  cd examples/v0.0.0-project && ../../bin/intent upgrade --dry-run
  cd examples/v1.2.0-project && ../../bin/intent upgrade --dry-run
  cd examples/v1.2.1-project && ../../bin/intent upgrade --dry-run
  2. Execute Self-Migration (task-70):
    - Backup current state first
    - Run intent upgrade on the Intent project itself
    - Verify all functionality still works
  3. Update Backlog Tasks:
    - Mark completed tasks as done
    - Create new tasks for remaining work

  Secondary Objectives (if time permits)

  4. Recover Test Coverage:
    - Follow tests/RECOVERY_PLAN.md
    - Priority: doctor tests, migrate tests, integration tests
  5. Documentation Updates:
    - Update README.md for v2.0.0
    - Create migration guide
    - Update command help text

  Commands to Get Started

# Verify current state

  ./tests/run_tests.sh              # Should show 83/83 passing
  ./bin/intent doctor               # Check configuration
  ./bin/intent st show ST0016       # Review steel thread

# Check backlog status

  ./bin/intent task list ST0016     # List all ST0016 tasks
  ./bin/intent bl list --plain | grep ST0016  # Alternative view

# Start work

  ./bin/intent task sync ST0016     # Sync task status

  Important Files to Review

  1. intent/st/ST0016/info.md - Overview and current status
  2. intent/st/ST0016/tasks.md - Detailed task breakdown
  3. intent/st/ST0016/impl.md - Technical implementation details
  4. tests/RECOVERY_PLAN.md - Plan to restore missing tests
  5. CLAUDE.md - Project-specific instructions

  Success Criteria

- All example projects can be successfully migrated
- The Intent project itself is migrated to v2.0.0 structure
- All tests pass after migration
- Documentation reflects v2.0.0 changes
- Clean git history with meaningful commits

  Remember: This is a fail-forward implementation. If something breaks, fix it and move forward. Do not implement rollback
  mechanisms.

  Important Reminders

- The tool is being renamed from "stp" to "intent"
- The Steel Thread Process (STP) methodology remains unchanged
- ST#### numbering continues as before
- This is a fail-forward implementation - no rollback needed
- Test everything thoroughly before proceeding to next phase

  Start by reading all ST0016/*.md files, then begin with Phase 0. Good luck!

  This restart prompt provides comprehensive instructions for implementing Intent v2.0.0, following the exact specifications in
  ST0016. It emphasizes the critical Phase 0 (test infrastructure first) approach and provides all necessary context for a fresh
  session.
