# Session Restart Context

## Project

Intent v2.5.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

v2.5.0 released. ST0026 (Steel Thread Zero) is the active steel thread -- fully elaborated with 11 work packages, no code written yet.

## ST0026: Steel Thread Zero

**Goal**: Make it structurally impossible for code quality violations to accumulate. Prevention over remediation.

**Context**: Two audits found 797 combined violations (laksa-web ST0058: 408, Lamplight ST0098: 389). ST0026 bakes rules, registries, and enforcement into every project from commit one.

**11 Work Packages** (see `intent/st/ST0026/WP/*/info.md` for details):

| WP    | Title                 | Key deliverables                          | Status      |
| ----- | --------------------- | ----------------------------------------- | ----------- |
| WP-01 | Skill Rename          | Rename intent-_ to in-_ (6 skills)        | Not Started |
| WP-02 | Workflow Skills       | 5 new /in-\* skills (start,plan,etc.)     | Not Started |
| WP-03 | LLM Templates         | CLAUDE.md, MODULES.md, DECISION_TREE.md   | Not Started |
| WP-04 | Memory Injection      | `intent claude prime` command             | Not Started |
| WP-05 | Archetype Templates   | 9 Elixir module templates + ARCHETYPES.md | Not Started |
| WP-06 | Automated Enforcement | 6 Credo checks + `intent audit quick`     | Not Started |
| WP-07 | Health & Learnings    | `intent audit health` + `intent learn`    | Not Started |
| WP-08 | Guardrails            | Module checklist + dependency graph check | Not Started |
| WP-09 | Retrofit Installation | `intent st zero install` (brownfield)     | Not Started |
| WP-10 | Integrator Command    | `intent init --with-st0000` (greenfield)  | Not Started |
| WP-11 | TN004 Tech Note       | Port audit process doc from laksa-web     | Not Started |

**Execution order**: WP-01 -> WP-02, then WP-03 + WP-05 in parallel, then WP-04, then WP-06-08, then WP-09-10. WP-11 anytime.

## Architecture

- Commands: `bin/intent_<name>`, auto-routed by `bin/intent`
- Plugin commands: `intent/plugins/claude/bin/intent_claude_*`
- Shared plugin library: `intent/plugins/claude/lib/claude_plugin_helpers.sh`
- Shared helpers: `bin/intent_helpers`
- Skills: `intent/plugins/claude/skills/<name>/SKILL.md` (currently 6, all `intent-*` prefix)
- Templates: `lib/templates/` (ST, WP, LLM)
- Tests: `tests/unit/` (17 .bats files, 339 tests)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)

## Deferred Work (ST0025)

Lower-priority Highlander violations: template consolidation (CLAUDE.md in 3 places), correctness fixes (upgrade bypasses install lifecycle), legacy cleanup. See `intent/st/COMPLETED/ST0025/design.md`.

## Conventions

- NO Claude attribution in commit messages -- ever
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Markdown linter auto-formats on save
- Run `tests/run_tests.sh` before committing
