---
id: IN-SH-CODE-003
language: shell
category: code
severity: warning
title: set -euo pipefail at top of every bash script
summary: >
  Every bash script opens with `set -euo pipefail` (or at least `set -e`
  with documented rationale). Errors exit immediately, undefined
  variables fail loudly, pipeline failures propagate. Default bash
  behaviour is "keep running past every error", which is wrong for 99%
  of scripts.
principles:
  - no-silent-errors
applies_when:
  - "Any new bash script intended for production, CI, or release"
  - "Any script more than a handful of lines whose failure could leave the system partially changed"
  - "Scripts that build, deploy, migrate, or modify any external state"
applies_to:
  - "**/*.sh"
  - "**/*.bash"
  - "bin/*"
does_not_apply_when:
  - "Interactive scripts (.bashrc, .bash_profile) where strict modes break unrelated interactive behaviours"
  - "Legacy scripts where retrofit would require auditing every unset-var usage (document and track)"
  - "Bash 3.x compatibility constraints where `-o pipefail` has known edge cases — Intent itself uses just `set -e` for macOS bash 3.x"
tags:
  - shell
  - bash
  - bash-specific
  - error-handling
  - strict-mode
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SH-CODE-004
  - IN-SH-CODE-005
aliases: []
status: active
version: 1
---

# set -euo pipefail at top of every bash script

Bash's default is to run every command and ignore every error. That is wrong for any script that matters.

## Problem

By default, `bash script.sh` executes each line in order and continues regardless of individual command exit codes. A failed `mkdir` in step 1 does not stop step 2 from writing to the directory that was not created; a failed `rm` does not stop step 3 from assuming the file is gone; a pipeline `cmd1 | cmd2` reports only `cmd2`'s exit code, so a broken `cmd1` is silently lost.

The triple `set -euo pipefail` is the bash strict mode:

- `-e` (errexit): exit immediately on any command returning non-zero (with well-known exceptions inside conditionals).
- `-u` (nounset): treat references to undefined variables as errors.
- `-o pipefail`: a pipeline fails if _any_ stage fails, not just the last.

Without these, a script that "succeeds" might have silently done nothing. With them, the script crashes loudly at the first real problem, preserving invariants.

## Detection

Static signals:

- Bash scripts that do not begin with a `set -...` directive in the first 10 lines after the shebang.
- `set -e` alone, without `-u` or `-o pipefail`, with no comment explaining the choice.
- `set +e` or `set +u` scattered in the body without a comment explaining why the strictness is being relaxed.

ShellCheck: indirectly via SC2148 (missing shebang) and SC2154 (unset variable) — the latter is effectively unreachable unless `-u` is on.

## Bad

```bash
#!/bin/bash
# Deploy the thing.

mkdir /tmp/release
cp -r dist/* /tmp/release/
rsync -a /tmp/release/ user@host:/opt/app/
rm -rf /tmp/release
```

A failed `mkdir` does not stop `cp`; a failed `rsync` does not stop `rm`. A typo in a variable (`$RELESE` instead of `$RELEASE`) silently expands to empty and `rm -rf /` ... we hope not, but the door is open.

## Good

```bash
#!/bin/bash
set -euo pipefail

# Deploy the thing.
target="${1:?target host required}"

mkdir -p /tmp/release
cp -r dist/* /tmp/release/
rsync -a /tmp/release/ "$target:/opt/app/"
rm -rf /tmp/release
```

Any failure aborts the script immediately. Missing `$1` fails loudly with a message. `"$target"` is quoted (see IN-SH-CODE-001) and undefined would have triggered `-u`.

## When This Applies

- Every new bash script committed to a project.
- CI / deployment / migration scripts especially — these touch external state and need loud failure.
- Functions sourced into other scripts inherit the caller's settings; prefer explicit `set` at the top of any standalone script that can also be sourced.

## When This Does Not Apply

- `.bashrc` / `.bash_profile` / interactive shell startup: strict modes break tab-completion handlers, prompt customisations, and unrelated interactive behaviours.
- Legacy scripts where retrofit would unearth a backlog of unset-var usage. Document the decision, file a ticket, migrate incrementally.
- Bash 3.x compatibility constraints (macOS default). `-o pipefail` has known edge cases in bash 3.x on some corner inputs; Intent itself uses `set -e` alone and documents why (see `CLAUDE.md` / `MEMORY.md` on bash 3.x constraints).

## Further Reading

- "Unofficial Bash Strict Mode" (Aaron Maxwell) (<http://redsymbol.net/articles/unofficial-bash-strict-mode/>)
- Google Shell Style Guide — Error Handling (<https://google.github.io/styleguide/shellguide.html>)
- ShellCheck wiki SC2148 (<https://www.shellcheck.net/wiki/SC2148>)
- IN-SH-CODE-004 — zsh equivalent (`setopt err_exit no_unset pipe_fail`)
- IN-SH-CODE-005 — no-silent-exit-codes is the complementary discipline for explicitly-checked failures
- IN-AG-NO-SILENT-001 — the agnostic principle this rule concretises
