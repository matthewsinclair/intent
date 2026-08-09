---
id: "0016"
title: Project template bakes an absolute INTENT_HOME path into .claude/settings.json
date: 2026-08-09
reporter: matts
status: OPEN
severity: medium
---

# 0016: Project template bakes an absolute INTENT_HOME path into .claude/settings.json

## Tags

templates, claude, hooks, portability, hygiene

## Summary

`lib/templates/.claude/settings.json` invokes its hook scripts through `[[INTENT_HOME]]` (lines 9 and 21), which is substituted with the installing machine's absolute Intent home at install time. Every project scaffolded from the template therefore carries an absolute path to one user's home directory in a tracked file, so the hooks are machine-specific and the path is published if the project's repository is public.

## Reproduction

Scaffold a project with the Claude template on any machine, then read the project's `.claude/settings.json`: the `SessionStart` and `UserPromptSubmit` hook commands carry the installing machine's absolute Intent home rather than a portable reference. Measured 8 Aug 2026 across sibling projects: two projects carry 2 occurrences each; one project (Cdsync) has been hand-fixed and now guards the invariant with a test asserting no tracked file carries an absolute home-directory path.

## Root Cause

The substitution happens at install time, so the resolved value is frozen into the project's tracked file. Hook resolution is a runtime question answered at write time.

## Impact

- The project's hooks break for any other contributor or machine, since the baked path exists only on the installing machine.
- A public repository publishes the installing user's home-directory path -- a hygiene leak of exactly the kind downstream projects lint against.
- The fix today is a per-project hand-edit that recurs on every scaffold.

## Proposed Fix

Resolve at runtime instead of install time: have the template invoke hooks via an environment variable (an `INTENT_HOME` the shell profile already exports), a PATH-resolved wrapper owned by the installed tool, or a `$HOME`-relative form -- then the template needs no substitution and the scaffolded `settings.json` is byte-identical on every machine.

## Related

- Cdsync ST0003/WP-08 -- where the defect was measured and recorded as an upstream report

## Resolutions

{{TBC}}
