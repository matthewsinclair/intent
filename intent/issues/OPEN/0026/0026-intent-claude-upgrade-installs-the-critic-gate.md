---
id: "0026"
title: intent claude upgrade installs the critic gate without honouring core.hooksPath, and reports success
date: 2026-08-15
reporter: matts
status: OPEN
severity: high
---

# 0026: intent claude upgrade installs the critic gate without honouring core.hooksPath, and reports success

## Tags

canon, hooks, false-green, fleet

## Summary

`intent claude upgrade` installs the critic pre-commit gate by writing `.git/hooks/pre-commit` (and `.git/hooks/pre-commit.intent` in the chained form) at a hard-coded path. Git does not read that directory when `core.hooksPath` is set. In any consumer project that has redirected `core.hooksPath` -- Husky, the Python `pre-commit` framework, a monorepo with a shared hook directory -- the gate is written where git will never look, and the installer prints `INSTALLED` or `CHAINED` in green.

The gate is installed and inert, and the tool reports that it is fine. This is worse than silence: a consumer who checks is told the thing they checked is working.

## Reproduction

```
git init /tmp/probe && cd /tmp/probe
mkdir -p .githooks && git config core.hooksPath .githooks
intent init && intent claude upgrade --apply
# installer reports .git/hooks/pre-commit INSTALLED (green)
# git runs .githooks/ -- the gate never fires
```

Measured on 2026-08-15 across every tracked file in the Intent repo:

```
core.hooksPath mentioned in:  bin/.devbin/cmd/hooks         (added 2026-08-15, dc)
                              intent/whiteboard/dc/wip.md   (dc's board)
                              -- and nowhere else

intent/plugins/claude/bin/intent_claude_upgrade   no hooksPath handling; hard-codes .git/hooks
bin/intent_doctor                                  no hooksPath handling, and no hook check at all
```

## Root Cause

`intent_claude_upgrade` treats `.git/hooks` as the hook directory rather than resolving `git config --get core.hooksPath` first. The canon-print status is derived from what the installer WROTE, not from where git will READ, so the status line cannot be wrong in the installer's own terms and cannot be right in the consumer's.

Two shapes underneath it, both general:

- **A status word in colour is a claim.** `INSTALLED` in green is an assertion about the world, and it was derived from an assertion about the installer.
- **An installer that reports where it wrote rather than where the tool will read is not reporting installation at all.**

## Impact

The critic gate is the mechanism Intent tells consumers protects their code at commit time. On any affected project it does nothing, silently, while reporting success. Nothing downstream contradicts it: `intent doctor` has no hook check whatsoever, so the installer and the diagnostic share one blind spot -- cross-checking them yields agreement and no information, which is the shape that lets a defect survive review.

**The defect is certain; the exposure is unmeasured.** No fleet survey has been run for a redirected `hooksPath` and this issue does not claim one. The Intent repo itself is unaffected (`git config --get core.hooksPath` is unset; git reads `.git/hooks` here).

## Proposed Fix

1. `intent_claude_upgrade` resolves `core.hooksPath` (falling back to `.git/hooks`) and installs into the resolved directory.
2. Its status line reports the RESOLVED path, so the output names where git will actually look.
3. `intent doctor` grows a hook check reporting three states -- WIRED (present and invoking the runner), UNWIRED (present, executable, invoking something else), ABSENT -- and honours `core.hooksPath` for the same reason. `bin/.devbin/cmd/hooks` is a working reference implementation of exactly this, including reading the guard names from the runner rather than a roster that can rot.

Whether Intent should additionally ADOPT `core.hooksPath` pointed at a tracked directory is a separate architectural question, open for hv: it would shrink the per-clone action from copying files to one config command and make hook bodies reviewable. **This issue is orthogonal to that decision.** Adopt it and the installer must honour `hooksPath` or it orphans its own output; reject it and the installer must still honour it, because consumers set it for their own reasons and are already being misinformed.

## Related

- ST0035 -- shipped the critic pre-commit gate (WP-06)
- Surfaced 2026-08-15 while dc built `int hooks` after finding that `.git/hooks` is never tracked, so a fresh clone receives every guard and nothing invoking them. dc declined to redirect `core.hooksPath` on lane grounds -- it would silently orphan this installer's output -- and wrote it up rather than deciding it. Chasing the stated reason is what turned up the defect.

## Resolutions

{{TBC}}
