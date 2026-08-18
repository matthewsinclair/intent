# Design - ST0032: Fix Intent's Elixir Credo Checks

## Approach

Standalone Elixir script (`lib/scripts/configure_credo.exs`) that runs in the target project's working directory. Uses `Code.eval_file(".credo.exs")` to parse the config as a native Elixir data structure, modifies it to add `requires: ["credo_checks/"]` and register all custom check modules, then writes it back with `inspect/2`. Called from bash scripts (`intent_st_zero`, `intent_audit`) instead of the current wrong `elixirc_paths` hint.

## Design Decisions

### 1. Standalone Elixir script over Igniter

Igniter requires being a dependency in the target project's `mix.exs` and operates through Mix tasks. It cannot be invoked via `Mix.install/2` in a standalone script because `Igniter.new()` expects a loaded Mix project environment. `.credo.exs` is a simple map literal -- `Code.eval_file` + `inspect` is sufficient and has zero external dependencies.

### 2. Code.eval_file over string/regex manipulation

`.credo.exs` is valid Elixir that evaluates to a `%{configs: [...]}` map. Parsing it as data avoids fragile regex patterns on nested structures. Writing back with `inspect(data, pretty: true, limit: :infinity)` produces clean, correctly-formatted output.

### 3. Warning-only for mix.exs cleanup

Projects that followed the old wrong hint may have `"credo_checks"` in `elixirc_paths`. The script prints a warning but does NOT auto-modify `mix.exs` because `mix.exs` contains arbitrary code (function definitions, conditionals) that a data-level script shouldn't touch.

### 4. Removing --checks-dir from intent audit

Once `.credo.exs` has `requires: ["credo_checks/"]`, Credo loads checks natively. The `--checks-dir` flag becomes redundant. Removing it means `intent audit quick` and direct `mix credo --strict` produce identical results -- the entire point of this fix.

### 5. Rule ID R16 for bracket_access_on_struct

Continuing the existing numbering: R2, R6, R7, R11, R15 are taken. R8 (boolean_operators) and D11 (dependency_graph) are retired. R16 is the next available.

### 6. Struct-variable tracking for map_get and bracket_access checks

Both `map_get_on_struct` and `bracket_access_on_struct` need to distinguish struct variables from plain maps. Approach: walk function bodies collecting variables bound to `%Module{}` patterns, then only flag access operations on those variables. This eliminates the false-positive problem where every `Map.get(x, :atom_key)` was flagged.

## Architecture

```
intent st zero apply D5a
  |
  +--> cp templates to credo_checks/     (existing, unchanged)
  +--> rm deprecated templates           (new: boolean_operators, dependency_graph)
  +--> elixir configure_credo.exs        (new: patches .credo.exs)

intent audit quick
  |
  +--> ensure_checks_installed()
  |      +--> cp missing templates       (existing)
  |      +--> rm deprecated templates    (new)
  |      +--> elixir configure_credo.exs (new, on first install)
  |
  +--> mix credo --strict                (changed: no --checks-dir)
```

## Alternatives Considered

### Igniter as target project dependency

Adds `{:igniter, ...}` to the target's `mix.exs`, then runs a Mix task. More powerful AST modification but adds a transitive dependency to every target project. Rejected: overkill for a map literal.

### String/regex manipulation in bash

Patch `.credo.exs` with sed/awk entirely from bash. No Elixir needed but fragile on nested Elixir data structures, hard to maintain, and can't handle edge cases like existing `requires:` lists. Rejected: brittleness.

### Keep --checks-dir as the mechanism

Instead of patching `.credo.exs`, keep using `--checks-dir` everywhere and document it. Rejected: doesn't solve the core problem (direct `mix credo` misses checks).
