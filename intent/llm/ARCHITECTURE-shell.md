# ARCHITECTURE-shell.md

Shell (bash + zsh) architectural patterns and conventions for this project.

## Script Layout

Conventional CLI tool layout:

- `bin/<command>` -- top-level entry script (executable, with shebang)
- `bin/<command>_helpers` -- shared helpers (sourced, not executed)
- `bin/<command>_<subcommand>` -- per-subcommand dispatcher when the surface grows
- `lib/` -- shared libraries (sourced)
- `lib/templates/` -- generated-content source (substituted, not executed)
- `tests/` -- test scripts (BATS or shunit2)

## Script Organisation

- Each script starts with `#!/bin/bash` (or `#!/usr/bin/env bash`) and `set -euo pipefail`.
- Helpers and constants are sourced from a shared file -- never duplicated.
- Top-level scripts are thin coordinators: parse args, source helpers, dispatch to a function, exit.

## Bash 3.x Compatibility (macOS)

- macOS ships bash 3.2 by default. Avoid bash 4+ features in scripts that may run there:
  - No `declare -A` (associative arrays)
  - No `${VAR^}` / `${VAR^^}` (case modifiers)
  - No `mapfile` / `readarray`
  - No `${VAR@Q}` (quoting modifier)
- Use `case` + parallel arrays + explicit upper-casing helpers as alternatives.

## Error Handling

- `set -euo pipefail` is mandatory at the top of every script.
- Use a shared `error()` helper that prints to stderr and exits with non-zero status.
- Reserve `|| true` for genuinely benign failures; document with an inline comment.
- `trap` for cleanup of temporary files and lock acquisitions.

## Testing

- BATS (Bash Automated Testing System) is the canonical framework.
- Tests live under `tests/unit/` (one `.bats` file per module being tested).
- Shared test helpers live in `tests/lib/test_helper.bash`.
- Each test runs in a temporary directory; never against the user's home or project state.

## Build / CI

- `bash -n <script>` syntax check is gating in CI.
- `shellcheck` is gating when configured.
- Pre-commit critic gate runs `intent critic shell` on staged shell files (per Intent canon).
