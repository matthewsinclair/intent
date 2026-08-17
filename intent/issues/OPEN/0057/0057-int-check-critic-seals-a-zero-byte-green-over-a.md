---
id: "0057"
title: int check critic seals a zero-byte green over a zero-file scan, and every reader defines green by seal emptiness, so an empty population is indistinguishable from a clean one at the artefact
date: 2026-08-17
reporter: matts
status: OPEN
severity: low
---

# 0057: int check critic seals a zero-byte green over a zero-file scan, and every reader defines green by seal emptiness, so an empty population is indistinguishable from a clean one at the artefact

## Tags

critic, gates, runlog, vacuous-green, measured

## Summary

`bin/intent_critic:215-219` exits 0 with `ok: no staged <lang> files to scan` when nothing is staged. The run then seals a **zero-byte `.errors` file**, and every reader in the tooling defines green as exactly that:

```
bin/.devbin/cmd/measured:226   [ -s "$errors" ] || { printf 'GREEN\n'; return 0; }
bin/.devbin/lib/runlog:218     if [ ! -s "$errors" ]; then
bin/.devbin/lib/runlog:229     [ -s "$errors" ] ||
bin/.devbin/lib/runlog:725     if [ -n "$errors" ] && [ -s "$errors" ]; then
```

**A zero-byte seal from a zero-file scan is byte-identical to a zero-byte seal from a full clean scan.** The two states are indistinguishable at the artefact that decides the verdict -- which is a wrong subject producing a well-formed result, at the point where the result is durable.

## Reproduction

On a tree with unstaged changes and nothing staged, at `82d756e2`:

```
$ bin/int check critic
intent: nothing staged -- the critic will examine NO files (stage changes, or pass --files <path>...)
    elixir
ok: no staged elixir files to scan
    rust
ok: no staged rust files to scan
    shell
ok: no staged shell files to scan
verdict: tmp/check/20260817-1330.CRITIC.errors

$ wc -c < tmp/check/20260817-1330.CRITIC.errors
0
```

## Root Cause

The population size is stated where it is not durable-enough to matter and absent where the verdict is read.

**A correction to the first framing of this, because it was wrong and the corrected version is narrower:** the qualifier is NOT terminal-only. `tmp/check/<stamp>.CRITIC.out` carries the `nothing staged` line, and the log is kept beside the seal. So the fact is recoverable by a reader who opens the log.

**But nothing reads the log to decide green.** The `[ -s ]` test above is the definition, in four places, and the qualifier reaches none of them. So the defect is not "the information is lost" -- it is that **the information does not reach the artefact that carries the decision**, which is the same shape as issue 0049: a verdict that does not carry its own subject.

## Impact

Low, and there is no outage path -- the gate is advisory here and the pre-commit hook stages by construction, so the zero-file case is a manual invocation rather than a gate bypass.

What it costs is a summary: **anything that answers "were the gates green" from the seals cannot tell a critic run over 1584 files from a critic run over none.** A node that runs `int check critic` before staging gets a green that means nothing and leaves behind an artefact that will keep meaning nothing to every later reader. It is honest at the moment of reading and silent at every moment after, so it will pass any review conducted at the terminal -- which is where reviews are conducted.

## Proposed Fix

**Put the population in the verdict, which is the pattern `int measured` already establishes for trees.** Either the seal or the verdict line should carry the count the scan actually covered, so `0 files` is a fact a summariser can read rather than a silence it must infer. `ok: no staged <lang> files to scan` is already the population statement; it simply does not reach the artefact.

Deliberately NOT proposed: making the zero-file case non-zero-exit. The command is correct to succeed -- refusing would break the pre-commit path for any commit that touches no files of a given language, which is most commits. **The defect is in what the run records, not in what it decides.**

## Related

- ST0056 -- found while checking a peer's flagged observation rather than by running the gate for its own sake
- 0049 -- the same shape one level up: a verdict that does not record what it was taken against
- 0028 -- a safety mechanism whose artefact does not say what the rule intends

## Resolutions

{{TBC}}
