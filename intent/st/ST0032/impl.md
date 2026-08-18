# Implementation - ST0032: Fix Intent's Elixir Credo Checks

## File Change Summary

### New files

| File                                                            | Purpose                                                           |
| --------------------------------------------------------------- | ----------------------------------------------------------------- |
| `lib/scripts/configure_credo.exs`                               | Standalone Elixir script to patch `.credo.exs` in target projects |
| `lib/templates/credo_checks/elixir/bracket_access_on_struct.ex` | New Credo check: catches `struct[:field]` runtime crashes         |

### Deleted files

| File                                                     | Why                                                               |
| -------------------------------------------------------- | ----------------------------------------------------------------- |
| `lib/templates/credo_checks/elixir/boolean_operators.ex` | 100% false positives -- can't distinguish boolean vs truthy/falsy |
| `lib/templates/credo_checks/elixir/dependency_graph.ex`  | Compiler catches circular deps; requires unpopulated rules file   |

### Modified files

| File                                                           | Changes                                                                                              |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `lib/templates/credo_checks/elixir/map_get_on_struct.ex`       | Add struct-variable tracking, stop flagging all `Map.get` with atom keys                             |
| `lib/templates/credo_checks/elixir/missing_impl_annotation.ex` | Fix prewalk accumulator: flat issues list instead of `{behaviours, issues}` tuple                    |
| `lib/templates/credo_checks/elixir/debug_artifacts.ex`         | Add `excluded_paths` param for modules with legitimate `IO.puts`                                     |
| `lib/templates/credo_checks/elixir/thick_coordinator.ex`       | Exclude `*_web.ex` entrypoint modules with `use` in `quote` blocks                                   |
| `bin/intent_st_zero`                                           | D5a: call configure script, update check count 7->6, add .credo.exs verification                     |
| `bin/intent_audit`                                             | Remove `--checks-dir`, update rules (drop R8/D11, add R16), call configure script, remove wrong hint |
| `intent/llm/MODULES.md`                                        | Update credo template count, add configure_credo.exs entry                                           |
| `lib/help/audit.help.md`                                       | Update rule list                                                                                     |
| `lib/help/stzero.help.md`                                      | Update D5a description                                                                               |

## configure_credo.exs Design

The script:

1. Runs in target project cwd, invoked as `elixir /path/to/configure_credo.exs [--remove-stale]`
2. Uses `Mix.install([])` (no deps)
3. If `.credo.exs` missing: generates default config from embedded template
4. Parses existing `.credo.exs` via `Code.eval_file/1`
5. Patches the default config map:
   - Adds `requires: ["credo_checks/"]` (appends to existing list if present)
   - Adds all 6 check modules to `checks:` list (deduplicates)
   - With `--remove-stale`: removes `BooleanOperators` and `DependencyGraph` entries
6. Writes back with `inspect(config, pretty: true, limit: :infinity)`
7. Prints machine-readable status line for bash callers

## Template Fixes Detail

### map_get_on_struct.ex

Current: flags ALL `Map.get(x, :atom_key)` regardless of whether `x` is a struct.

Fix: two-phase AST walk:

- Phase 1: collect struct bindings -- find `%Module{} = var` and `var = %Module{}` patterns, build a `MapSet` of variable names known to be structs
- Phase 2: only flag `Map.get(var, :key)` when `var` is in the struct-variable set

### missing_impl_annotation.ex

Current: `Macro.prewalk` accumulator is `{[], []}` but `traverse/3` returns `{ast, {behaviours, issues}}`. The destructuring at line 28 captures the whole `{behaviours, issues}` tuple as `issues`.

Fix: change prewalk accumulator to `[]` (flat list). `traverse/3` returns `{ast, acc ++ module_issues}`. Behaviours stay local to `scan_module/2`.

### debug_artifacts.ex

Current: flags all `IO.inspect`, `IO.puts`, `dbg()` in `lib/` with no exceptions.

Fix: add `param_defaults: [excluded_paths: []]`. Read via `Params.get/3`. Skip files matching any excluded path pattern. Allows target projects to configure: `{Mix.Checks.DebugArtifacts, [excluded_paths: ["lib/my_app/cli", "lib/mix"]]}`.

### thick_coordinator.ex

Current: flags `*_web.ex` entrypoint modules because they contain `use Phoenix.Controller` inside macro `quote` blocks.

Fix: before running `is_coordinator?/1`, strip content inside `quote do...end` blocks from the source text. This way, macro delegation code doesn't trigger false positives.
