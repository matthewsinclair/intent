---
id: IN-SH-CODE-006
language: shell
category: code
severity: warning
title: One helper function per concern across bin/
summary: >
  Helper functions live in one place. When a new script in a multi-script
  project needs an error-printer, a version-getter, or a config-parser, it
  sources the shared library rather than re-implementing the function.
  Shell codebases drift fastest when every script carries its own copy
  of `error()`.
principles:
  - highlander
applies_when:
  - "Writing a new shell dispatcher or subcommand in a multi-script project"
  - "Copy-pasting a helper from another script rather than sourcing it"
  - "Adding the same `N+1`th definition of a utility already defined elsewhere"
applies_to:
  - "bin/*"
  - "**/*.sh"
does_not_apply_when:
  - "Truly self-contained one-file utilities with no siblings"
  - "Cases where sourcing a shared library is impossible (cross-host transfers, installer bundles)"
tags:
  - shell
  - bash
  - zsh
  - highlander
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-CODE-006
aliases: []
status: active
version: 1
---

# One helper function per concern across bin/

There can be only one `error()`. Copies drift.

## Problem

Shell scripts accumulate by copy-paste. A new dispatcher command needs `error()`, so the author copies the four-line function from a sibling. Six months later, one copy prints to `stderr` with a prefix, another prints without, a third exits with `1`, a fourth exits with the `$?` it received. The behaviour of "what does `error()` do in script X?" depends on which script and which era of authoring, and nobody notices until the error messages start looking inconsistent in production.

The remedy is the Highlander Rule applied at the shell layer: one helper function per concern, one location, every script sources it. Intent's own dev tooling does this with `bin/.devbin/lib/helpers`, which `bin/devbin` sources before it dispatches. Any new `error()` / `get_version()` / `ensure_intent_home()` function has a canonical home, and scripts that need it source it.

The secondary benefit: changes to the helper propagate to every caller at once. No more "fix this bug in four different scripts".

## Detection

Static signals:

- Multiple scripts defining a function with the same name (`grep -l '^error()' bin/*`).
- Nearly identical function bodies across scripts (a `diff` would show one or two trivial differences).
- A new script that does not source the project's shared helper library and yet invokes helpers such as `error` or `get_version`.
- Pull requests adding a utility function to one script while an identical function already exists in the shared library.

**No greppable proxy is authoritative for this rule.** The signal is one function name defined in more than one file, which requires COUNTING across the corpus; a `grep` cannot aggregate, and the headless runner runs each proxy line as one pattern and unions the per-line matches, so nothing counts across files. **No shellcheck lint asks it either** -- cross-file duplication is outside what a per-file parser sees at all, so this is not a limit the runner imposes. Apply this rule via the LLM-driven `critic-shell` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

```bash
#!/bin/bash
# bin/new_tool — yet another dispatcher

error() {
  echo "Error: $1" >&2
  exit 1
}

version() {
  if [ -f "$INTENT_HOME/VERSION" ]; then
    cat "$INTENT_HOME/VERSION"
  else
    echo "unknown"
  fi
}

# ... dispatcher body ...
```

`error` already exists in the shared `lib/helpers`. `version` reinvents its `get_version`. Both will drift from the shared copies within a release or two.

## Good

```bash
#!/bin/bash
# bin/new_tool — yet another dispatcher
set -e

source "$(dirname "${BASH_SOURCE[0]}")/../lib/helpers"

# Everything that follows uses the canonical error(), get_version(), etc.
version="$(get_version)"
if [ -z "$version" ]; then
  error "VERSION missing"
fi

# ... dispatcher body ...
```

`error` and `get_version` come from one place. Bug fixes and refactors happen once.

## When This Applies

- Any multi-script project where helpers cross script boundaries.
- Intent itself: `bin/devbin` sources `bin/.devbin/lib/helpers` rather than each command carrying its own `die`.
- Shell libraries in larger projects (`lib/common.sh`, `scripts/bootstrap.sh`) whose existence is the shared-helper architecture.

## When This Does Not Apply

- Genuinely one-off scripts that have no siblings. A `bin/one-time-backfill` run once and deleted does not benefit from the Highlander discipline.
- Scripts bundled for deployment to a system where sourcing is impossible (installer payload, embedded-systems bootstrap). Use a build step to inline the shared content rather than duplicating by hand.
- Teaching examples where the duplication is pedagogical.

## Further Reading

- IN-AG-HIGHLANDER-001 — the agnostic principle this concretises
- IN-EX-CODE-006 — Elixir module-Highlander counterpart (same logic, different language)
- Intent's `bin/.devbin/lib/helpers` is a worked example of this pattern in practice
- Google Shell Style Guide — Shared Functions (<https://google.github.io/styleguide/shellguide.html>)
