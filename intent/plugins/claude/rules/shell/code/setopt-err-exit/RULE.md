---
id: IN-SH-CODE-004
language: shell
category: code
severity: warning
title: setopt err_exit pipe_fail no_unset in zsh
summary: >
  Zsh's equivalent of bash strict mode is `setopt err_exit pipe_fail
  no_unset`. Semantics diverge from bash on specific corner cases
  (subshell behaviour, trap interactions) — name each option
  explicitly rather than assume bash-compatible defaults.
principles:
  - no-silent-errors
applies_when:
  - "New zsh script (shebang `#!/bin/zsh` or `#!/usr/bin/env zsh`)"
  - "Zsh functions stored in `$fpath` for autoload"
  - "Cross-shell scripts that need to behave strictly under both bash and zsh"
applies_to:
  - "**/*.zsh"
does_not_apply_when:
  - "Interactive zsh startup (`.zshrc`) where strict modes break unrelated behaviours"
  - "Zsh-emulation-in-bash mode (`emulate sh`) where `set -e` is already active"
tags:
  - shell
  - zsh
  - zsh-specific
  - error-handling
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SH-CODE-003
  - IN-SH-CODE-005
aliases: []
status: active
version: 1
---

# setopt err_exit pipe_fail no_unset in zsh

Zsh is not bash. `set -euo pipefail` works in zsh, but the idiomatic form uses `setopt` and exposes the semantic differences.

## Problem

Zsh accepts `set -euo pipefail` for bash compatibility, but its native form is `setopt err_exit pipe_fail no_unset`. The two forms are not identical:

- Zsh's `err_exit` has different interactions with traps, `&&` / `||` chains, and subshells than bash's `-e`. Migration guides document edge cases where a bash script "just works" but a zsh script with `set -e` does not.
- `pipe_fail` is zsh-native and behaves the same as bash's `-o pipefail` in the common case.
- `no_unset` behaves the same as bash's `-u` but catches zsh-specific variable forms (`${(e)var}` parameter flags) that bash does not have.

For scripts genuinely targeting zsh (shebang `#!/bin/zsh`), use `setopt`. For cross-shell scripts, either pick a dominant shell or add a dialect check at the top.

Zsh also has `setopt` groups worth knowing: `WARN_CREATE_GLOBAL` catches accidental globals in functions; `NO_NOMATCH` (controversial) controls whether unmatched globs error. Both are topic-adjacent and worth considering per script.

## Detection

Static signals:

- Zsh scripts (shebang `#!/bin/zsh`) that begin with `set -e` only, or no `set`/`setopt` at all.
- Cross-shell scripts that set `set -e` and rely on identical behaviour under zsh without documentation.
- Zsh scripts that use bash-specific syntax (arrays with bash semantics, `[[ ]]` with bash-only regex flavour) without an `emulate bash` directive.

## Bad

```zsh
#!/bin/zsh
# A deployment script.

release_dir=/tmp/release
mkdir $release_dir
cp -r dist/* $release_dir/
```

No strict mode, no explicit dialect. Behaves differently on macOS vs Linux, differently under various zsh versions. Unquoted variables (IN-SH-CODE-001 violation) compound the problem.

## Good

```zsh
#!/bin/zsh
setopt err_exit pipe_fail no_unset

# A deployment script.
release_dir="/tmp/release"
target="${1:?target host required}"

mkdir -p "$release_dir"
cp -r dist/* "$release_dir/"
rsync -a "$release_dir/" "$target:/opt/app/"
```

Explicit zsh strict mode. Script fails loudly on any step. Required args checked.

## When This Applies

- Every `#!/bin/zsh` or `#!/usr/bin/env zsh` script.
- Zsh autoload functions under `$fpath`.
- Zsh completion definitions where a bug would fire repeatedly per tab-key press.

## When This Does Not Apply

- Interactive `.zshrc` — users want tolerant startup behaviour; one failing theme line shouldn't leave them at a broken prompt.
- `emulate sh` / `emulate bash` contexts where the emulation mode already manages relevant options.
- Scripts explicitly targeting both shells that set `set -e` and document the accepted behavioural differences.

## Further Reading

- Zsh manual — `setopt` (<https://zsh.sourceforge.io/Doc/Release/Options.html>)
- Zsh migration guide — differences from bash (<https://zsh.sourceforge.io/FAQ/zshfaq03.html>)
- IN-SH-CODE-003 — bash equivalent (`set -euo pipefail`) with rationale
- IN-SH-CODE-005 — no-silent-exit-codes applies across both shells
- IN-AG-NO-SILENT-001 — agnostic principle this concretises
