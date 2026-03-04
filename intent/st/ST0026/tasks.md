# Tasks - ST0026: Steel Thread Zero

## WP-01: Skill Rename (intent-_ to in-_)

- [ ] Rename 6 skill source directories from `intent-*` to `in-*`
- [ ] Update self-references inside SKILL.md files (autopsy script path, essentials install example)
- [ ] Update `lib/help/claude.help.md` examples
- [ ] Update project `CLAUDE.md` skill references
- [ ] Update `lib/templates/llm/_CLAUDE.md` template
- [ ] Update MEMORY.md skill names
- [ ] Update any test file references
- [ ] Uninstall old skills, install new via `intent claude skills sync`
- [ ] Run full test suite

## WP-02: Workflow Skills

- [ ] Create `in-start/SKILL.md` (session start procedure)
- [ ] Create `in-plan/SKILL.md` (planning kickoff procedure)
- [ ] Create `in-standards/SKILL.md` (coding standards primer)
- [ ] Create `in-next/SKILL.md` (next step identification)
- [ ] Create `in-finish/SKILL.md` (session finish procedure)
- [ ] Install all 5 new skills
- [ ] Test each skill invocation in Claude Code

## WP-03: LLM Templates (D2, D3, D6)

- [ ] Rewrite `lib/templates/llm/_CLAUDE.md` (enhanced v2.6.0 template)
- [ ] Update `create_claude_md()` in `bin/intent_helpers` to use enhanced template
- [ ] Update heredoc in `bin/intent_init` to use template
- [ ] Create `lib/templates/llm/_MODULES.md`
- [ ] Create `lib/templates/llm/_DECISION_TREE.md`
- [ ] Test `intent init` creates all 3 files correctly
- [ ] Verify variable substitution works

## WP-04: Memory Injection (D8)

- [ ] Create `lib/templates/prime/operational-knowledge.md` (bundled knowledge)
- [ ] Create `intent/plugins/claude/bin/intent_claude_prime` script
- [ ] Implement memory path computation (project path -> ~/.claude/projects/HASH/memory/)
- [ ] Implement source file reading with graceful missing-file handling
- [ ] Implement MEMORY.md synthesis (condensation, not verbatim)
- [ ] Implement `--refresh` flag (full overwrite)
- [ ] Implement `--dry-run` flag (show output, don't write)
- [ ] Implement `--from <project>` flag (import learnings)
- [ ] Add to `intent/plugins/claude/plugin.json`
- [ ] Add prime section to `lib/help/claude.help.md`
- [ ] Verify output stays under 200 lines
- [ ] Test with missing source files
- [ ] Test with complete source files

## WP-05: Archetype Templates (D4)

- [ ] Create `lib/templates/archetypes/elixir/` directory
- [ ] Create `ash_domain.ex.eex`
- [ ] Create `ash_resource.ex.eex`
- [ ] Create `phoenix_controller.ex.eex`
- [ ] Create `live_view.ex.eex`
- [ ] Create `service.ex.eex`
- [ ] Create `cli_command.ex.eex`
- [ ] Create `genserver.ex.eex`
- [ ] Create `oban_worker.ex.eex`
- [ ] Create `test.ex.eex`
- [ ] Create `lib/templates/llm/_ARCHETYPES.md` reference document
- [ ] Verify templates are syntactically valid Elixir

## WP-06: Automated Enforcement (D5a, D5b)

- [ ] Create `lib/templates/credo_checks/elixir/boolean_operators.ex`
- [ ] Create `lib/templates/credo_checks/elixir/missing_impl_annotation.ex`
- [ ] Create `lib/templates/credo_checks/elixir/debug_artifacts.ex`
- [ ] Create `lib/templates/credo_checks/elixir/map_get_on_struct.ex`
- [ ] Create `lib/templates/credo_checks/elixir/thick_coordinator.ex`
- [ ] Create `lib/templates/credo_checks/elixir/highlander_suspect.ex`
- [ ] Create `bin/intent_audit` command script
- [ ] Implement `intent audit quick` subcommand
- [ ] Implement `--rule` filter
- [ ] Implement `--fix` auto-fix for R8, R11, R15
- [ ] Implement `--json` output mode
- [ ] Create `lib/help/audit.help.md`
- [ ] Test against known-clean and known-dirty codebases

## WP-07: Health Check & Learnings (D7, D10)

- [ ] Implement `intent audit health` subcommand in `bin/intent_audit`
- [ ] Implement `--report` flag (save markdown)
- [ ] Implement `--diff` flag (git-based changed files only)
- [ ] Implement timestamp tracking in `.intent/last-health-check`
- [ ] Create `bin/intent_learn` command script
- [ ] Implement `intent learn "description"` (append to .intent/learnings.md)
- [ ] Implement `--category` flag (footgun/worked/failed)
- [ ] Implement `--list` flag
- [ ] Test learnings integration with `intent claude prime`

## WP-08: Guardrails (D9, D11)

- [ ] Create `bin/intent_modules` command script
- [ ] Implement `intent modules check`
- [ ] Implement `intent modules check --register`
- [ ] Implement `intent modules find`
- [ ] Create Claude Code hook for `Write` to `lib/**/*.ex`
- [ ] Create `lib/templates/llm/_DEPENDENCY_GRAPH.md` template
- [ ] Implement dependency graph check (scan alias/import/use vs declared deps)
- [ ] Integrate with `intent audit quick`

## WP-09: Retrofit Installation (D12)

- [ ] Create `bin/intent_st_zero` (or extend `bin/intent_st`)
- [ ] Implement `intent st zero install` with 4 phases
- [ ] Implement module auto-discovery algorithm
- [ ] Implement gap analysis report
- [ ] Implement proposal generation
- [ ] Implement interactive apply with confirmation
- [ ] Implement `--audit-only` flag
- [ ] Implement `--deliverable` filter
- [ ] Implement `--dry-run` flag
- [ ] Test on Intent project itself (eating our own dogfood)

## WP-10: Integrator Command (D1)

- [ ] Modify `bin/intent_init` to accept `--with-st0000` flag
- [ ] Implement `bootstrap_st0000()` function
- [ ] Create `bin/intent_st_zero` for `intent st zero` subcommand
- [ ] Implement `intent st zero --check` compliance verification
- [ ] Test greenfield: `intent init --with-st0000` creates fully equipped project
- [ ] Test existing project: `intent st zero` delegates to retrofit
- [ ] Run `intent audit quick` on bootstrapped project (expect zero violations)

## WP-11: TN004 Tech Note (Total Codebase Audit)

- [ ] Copy TN004 from `../Laksa/laksa-web/intent/eng/notes/tn004-total-codebase-audit.md`
- [ ] Replace project-specific references (laksa-web, Lamplight, ST0058, ST0098) with generic examples
- [ ] Generalize stats and specific module names
- [ ] Extract universal lessons from Appendix F into main text
- [ ] Add Prerequisites section
- [ ] Add cross-references to ST0026 deliverables
- [ ] Drop project-specific appendix data
- [ ] Place at `intent/docs/tn004-total-codebase-audit.md`

## Dependencies

```
WP-01 -> WP-02
WP-03 -> WP-04 -> WP-10
WP-05 -> WP-04
WP-06 -> WP-07
WP-03 -> WP-08
WP-06 -> WP-08
WP-03 + WP-04 + WP-06 -> WP-09 -> WP-10
```
