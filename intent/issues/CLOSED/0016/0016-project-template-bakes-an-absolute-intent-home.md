---
id: "0016"
title: Project template bakes an absolute INTENT_HOME path into .claude/settings.json
date: 2026-08-09
reporter: matts
status: CLOSED
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

FIXED + CLOSED (2026-08-14), shipped in v2.19.0. Confirmed as filed, **and live in this repository at the time of filing**: our own `.claude/settings.json` carried the absolute home path of the machine that installed it, and this repository is public.

**The root cause is a category error.** Hook resolution is a _runtime_ question and the template answered it at _write_ time, freezing the installing machine's absolute Intent home into a tracked file. Two consequences followed: the hooks worked on exactly one machine and broke silently for every other contributor, and any public repository scaffolded by Intent published one user's home directory path.

**What shipped.** Hooks are named as `intent claude hook <name>`, a thin runner (`intent/plugins/claude/bin/intent_claude_hook`, registered in MODULES.md first) that `exec`s the shipped script. The `exec` is deliberate: **stdin and the exit code must pass through untouched**, because the `UserPromptSubmit` gate signals "block" with exit 2 specifically, and a wrapper that swallowed either would have turned a strict gate into an advisory one without a word. `settings.json` needs no substitution and is now byte-identical on every machine, the canon engine's `[[INTENT_HOME]]` substitution arm is gone with a comment explaining why nothing should reintroduce it, and consumers converge on their next `intent upgrade` (verified end to end against a project carrying another machine's baked path). This repository's own settings.json was fixed in the same commit.

**A guard asserts no tracked file under `lib/templates/.claude/` or `.claude/` carries an absolute home path.**

### The guard could not fail, and only mutation testing found that

The first version of that guard used an **invalid** ERE: `grep` errored with "mismatched [", `|| true` swallowed the error, and the check returned empty regardless of what the files contained. It passed, and it would have passed forever. It was caught because the mutation run reported "restored: 7 ok" while the mutation was still applied -- a result that made no sense and had to be chased. It is now an empirically-verified fixed-string needle.

This is worth recording plainly: a guard that cannot fail, written into the test protecting against silently-broken configuration, is the same defect class as the issue it guards. Every guard in this release was subsequently broken-and-restored to prove it bites.

### Open, for hv

**The per-project `.claude/scripts/*.sh` copies are now unreferenced** -- `settings.json` calls the installed tool, so editing a local copy expecting it to take effect silently does nothing, which is the same trap class this issue removes. Pruning them deletes files from consumer trees, so it is recorded rather than actioned. The troubleshooting section of `working-with-llms.md` documents the trap in the meantime.

**Scope of the guard.** It covers configuration that _functions_ (tracked files under `lib/templates/.claude/` and `.claude/`). 42 other tracked files carry an absolute home path, mostly `intent/.treeindex/**` -- filed separately as **issue 0018** on hv's ruling, since a tracked machine-local cache in a public repo is its own decision.
