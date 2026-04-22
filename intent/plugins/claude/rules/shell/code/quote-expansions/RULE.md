---
id: IN-SH-CODE-001
language: shell
category: code
severity: critical
title: Always quote variable expansions
summary: >
  Every variable expansion goes in double quotes: `"$var"`, `"${arr[@]}"`,
  `"$(cmd)"`. Bare `$var` undergoes word-splitting and pathname expansion,
  which silently breaks on filenames with spaces, globs, and empty
  arguments.
principles:
  - no-silent-errors
  - honest-data
applies_when:
  - "Passing a variable as an argument to any command"
  - "Using a variable in a string context ([ ], [[ ]], test)"
  - 'Expanding an array: always `"${arr[@]}"`, never `${arr[@]}`'
applies_to:
  - "**/*.sh"
  - "**/*.bash"
  - "**/*.zsh"
  - "bin/*"
does_not_apply_when:
  - "Intentional word-splitting for a single-purpose utility (rare; document with a comment)"
  - "Expansion inside `[[ ]]` where bash performs no word-splitting on the right of `==`, `!=`, `=~` (but keep quoting for consistency)"
tags:
  - shell
  - bash
  - zsh
  - quoting
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SH-CODE-005
aliases: []
status: active
version: 1
---

# Always quote variable expansions

Unquoted `$var` is word-splitting plus pathname expansion. Every time. In both bash and zsh.

## Problem

Bash and zsh (in its POSIX-emulation mode) perform word-splitting and pathname expansion on unquoted variable references. `rm $file` where `$file` is `"My Documents"` becomes `rm My Documents` — two arguments, neither of which exist. `cp $src $dst` where `$src` is empty becomes `cp $dst` — a `cp` with one argument that typically fails in a confusing way. `for f in $files` where `$files` contains a `*` expands every matching filename in `$PWD`, often with catastrophic results.

Double quoting is the universal fix. `"$var"` is expanded but not split, not globbed. `"${arr[@]}"` expands an array to one argument per element, preserving empty elements. `"$(cmd)"` captures command output without splitting on internal newlines.

The mistake is not catching this until production: the dev's home directory has no spaces, the CI runner has no spaces, the user's `~/My Stuff` has spaces, and the script silently produces wrong output for months.

## Detection

Static signals:

- Bare `$var` in command arguments: `rm $file`, `mv $src $dst`, `echo $msg`.
- `$(...)` unquoted in assignments or arguments: `result=$(cmd)` is fine for scalars; `args=$(cmd)` fed to another command is risky.
- `${arr[@]}` without surrounding quotes — this is _especially_ dangerous because quietly collapses array elements.
- Test expressions: `[ -f $file ]` fails on filenames with spaces; use `[ -f "$file" ]` or `[[ -f $file ]]`.

Linters: `shellcheck` flags these with SC2086, SC2046, SC2206, SC2068.

## Bad

```bash
#!/bin/bash
file="$1"
other_files=$(find . -name '*.log')

rm $file
for f in $other_files; do
  echo Processing $f
  cp $f $BACKUP_DIR
done
```

Fails the moment any filename contains a space, a glob character, or the variable is empty.

## Good

```bash
#!/bin/bash
file="$1"

# Collect filenames safely with find -print0 + read -d ''.
mapfile -d '' other_files < <(find . -name '*.log' -print0)

rm "$file"
for f in "${other_files[@]}"; do
  echo "Processing $f"
  cp "$f" "$BACKUP_DIR"
done
```

Every expansion is quoted. Filenames with spaces, globs, and other special characters flow through untouched.

## When This Applies

- Every new shell script or shell function.
- Every variable reference in an argument position, test, or string context.
- Arrays in particular: `"${arr[@]}"` is the invariant. Unquoted is always wrong.

## When This Does Not Apply

- Intentional word-splitting: rare, document with a `# SC2086: deliberate splitting of whitespace-separated list` comment to acknowledge the risk and turn off the shellcheck alert locally.
- `[[ ]]` right-hand sides of comparison operators. Bash does not split there, but keep quotes for consistency — readers who scan for violations learn to skip any `[[ ]]` context.
- `case "$var" in ... esac` patterns — inside the `in ... esac`, word-splitting rules differ; still, keep the matching variable quoted.

## Further Reading

- BashFAQ #50: "I'm trying to put a command in a variable" (<https://mywiki.wooledge.org/BashFAQ/050>)
- ShellCheck SC2086 (<https://www.shellcheck.net/wiki/SC2086>), SC2046, SC2068
- Google Shell Style Guide — Quoting (<https://google.github.io/styleguide/shellguide.html#quoting>)
- IN-AG-NO-SILENT-001 — the silent-error class this rule prevents
