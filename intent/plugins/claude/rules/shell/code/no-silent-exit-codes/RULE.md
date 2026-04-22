---
id: IN-SH-CODE-005
language: shell
category: code
severity: critical
title: Never discard command exit codes
summary: >
  Every command's exit code must be handled: check it with `if`, `||`, or
  rely on `set -e` to propagate. Never append `|| true` or `2>/dev/null`
  without naming the reason. Discarded exit codes are the number one way
  shell scripts lie about success.
principles:
  - no-silent-errors
applies_when:
  - "Running any external command whose failure would be meaningful"
  - "Piping to another command, file, or `/dev/null`"
  - "Using `||` to branch on failure, or `&&` to chain on success"
applies_to:
  - "**/*.sh"
  - "**/*.bash"
  - "**/*.zsh"
  - "bin/*"
does_not_apply_when:
  - "Best-effort cleanup where failure is explicitly tolerated (`|| true` with comment)"
  - "Intentional capture of stderr for diagnostic output that would otherwise clutter"
  - "Nested subshells where the inner exit code is intentionally not propagated"
tags:
  - shell
  - bash
  - zsh
  - error-handling
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SH-CODE-003
  - IN-SH-CODE-004
aliases: []
status: active
version: 1
---

# Never discard command exit codes

A shell script's reliability is the product of every command's reliability. Swallowing a failure breaks the chain invisibly.

## Problem

Shell has several ways to discard an exit code, all of which look harmless:

- `cmd || true` — "ignore failure". Also ignores the reason the command failed.
- `cmd 2>/dev/null` — "silence stderr". The exit code still matters, but the diagnostic that would have helped debug is gone.
- Pipelines — `cmd1 | cmd2` under default bash reports only `cmd2`'s exit code (`set -o pipefail` fixes this; see IN-SH-CODE-003).
- Background jobs — `cmd &` — exit code goes to `wait`'s return, which is frequently discarded.
- `set +e` followed by several commands before `set -e` is restored — in between, every failure is silent.

Each of these has a legitimate use, but each also gets reached for reflexively when someone wants a `set -e` error to go away. That is the wrong reason. The right reason is "I understand the failure mode and am handling it deliberately".

## Detection

Static signals:

- `|| true` or `|| :` without an adjacent comment explaining why.
- `2>/dev/null` without `|| handle_error` or a comment.
- `set +e` blocks longer than 2-3 lines without an explicit rationale.
- Pipelines without `pipefail` set and without explicit `${PIPESTATUS[@]}` inspection.
- Functions that do not `return` the relevant exit code (see: a function that runs a command and then does `return 0` regardless).

ShellCheck: SC2015 (`&&` and `||` mixing pitfall), SC2164 (`cd` without error handling).

## Bad

```bash
#!/bin/bash
set -e

deploy() {
  rsync -a dist/ user@host:/opt/app/ || true
  systemctl restart app 2>/dev/null
  curl -s https://host/health | grep ok
}

deploy
echo "deploy complete"
```

The `rsync` failure is silently ignored. The `systemctl` failure is masked by `/dev/null`. The pipeline reports the `grep` exit code only; a broken `curl` produces `"deploy complete"` regardless.

## Good

```bash
#!/bin/bash
set -euo pipefail

deploy() {
  if ! rsync -a dist/ user@host:/opt/app/; then
    echo "ERROR: rsync failed" >&2
    return 1
  fi

  if ! systemctl restart app; then
    echo "ERROR: restart failed" >&2
    return 1
  fi

  local response
  response=$(curl -s -f https://host/health)
  if [ "$response" != "ok" ]; then
    echo "ERROR: health check returned '$response'" >&2
    return 1
  fi
}

if ! deploy; then
  echo "deploy failed" >&2
  exit 1
fi
echo "deploy complete"
```

Every command's failure is either propagated (via `set -euo pipefail`), deliberately checked (via `if !`), or reported with a specific error message. "Deploy complete" prints only when every step actually completed.

## When This Applies

- Every external command in a production shell script.
- Pipelines, especially those fetching data from services (`curl | jq` is classic; without `pipefail` a broken curl yields silent empty output).
- Cleanup paths — `trap '... cleanup ...' EXIT` should preserve the failure exit code via `$?`.

## When This Does Not Apply

- Explicitly best-effort cleanup: `rm -f /tmp/lock || true # lock file may not exist` with the comment.
- Intentional stderr redirection when stderr is noise: `check_feature_x 2>/dev/null # feature detection; absence is a normal case` with the comment.
- Sourcing a file that might not exist: `source ./optional.sh 2>/dev/null || true` — but consider `[ -f ./optional.sh ] && source ./optional.sh` for clarity.

## Further Reading

- BashFAQ #105: "I'm trying to use `set -e` and it's biting me" (<https://mywiki.wooledge.org/BashFAQ/105>)
- ShellCheck SC2015 (<https://www.shellcheck.net/wiki/SC2015>), SC2164
- Google Shell Style Guide — Error Handling (<https://google.github.io/styleguide/shellguide.html>)
- IN-SH-CODE-003, IN-SH-CODE-004 — strict-mode rules that make this rule easier to enforce
- IN-AG-NO-SILENT-001 — agnostic principle concretising here
