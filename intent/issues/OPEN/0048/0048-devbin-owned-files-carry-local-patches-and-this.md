---
id: "0048"
title: devbin-owned files carry local patches and this project has no detector for them
date: 2026-08-17
reporter: matts
status: OPEN
severity: medium
---

# 0048: devbin-owned files carry local patches and this project has no detector for them

## Tags

devbin, vendoring, provenance, tooling

## Summary

`bin/.devbin/` vendors devbin. Its `manifest.sha256` lists the 27 files devbin OWNS, and its own header states the contract: _"Every line below is a file devbin owns. An edit to one is DETECTED, not overwritten. Files not listed here -- config.yaml, cmd/, help/ -- belong to the project."_ **Four of those 27 currently diverge from their recorded checksums, and nothing in this repository reports that.** The detector the manifest header refers to (`devbin doctor` / `devbin upgrade`) lives in the Devbin source repository, not in this project's command surface, so the divergence is invisible from inside the project that carries it.

This is raised as its own item on hv's instruction (2026-08-17). It was going to be a footnote on a run-verdict design discussion, which is precisely how it stayed invisible for as long as it has.

## Reproduction

Measured 2026-08-17 at `2de5f1e5`, walking the manifest and re-hashing each entry:

```
manifest entries: 27   diverged: 4   missing: 0

  bin/.devbin/lib/cmd/check
  bin/.devbin/lib/cmd/docs
  bin/.devbin/lib/resolve
  bin/.devbin/lib/runlog
```

**Two of the four are not from today and were not known to anyone in the discussion that produced this issue.** `lib/cmd/check` and `lib/cmd/docs` were both modified under issue 0044 (`68282648`, and `a18010a8` for `docs`), which are legitimate committed fixes. `lib/resolve` and `lib/runlog` were modified at `55e8857b` for the seal-disagreement refusal. The count reported in conversation before it was measured was **three**, and it was three because it counted the patches one node happened to know about.

**A NOTE ON THE INSTRUMENT, because the first run of this measurement reported all 27 files diverged.** Under zsh, `path` is a special variable tied to `PATH`, so a loop written as `while read -r want path` silently destroys `PATH` on its first iteration; `shasum` and `awk` then cannot be found, every comparison fails, and every file is reported as diverged. **A broken instrument reported maximum alarm and looked exactly like a catastrophic finding.** Anyone re-running this must not name a loop variable `path`, and should keep an unmodified file in the output as a control -- the corrected run shows 23 matching, which is what says the tool works.

## Root Cause

Two separate things, and only the second is a defect.

**Patching a vendored file is a supported state.** The manifest header says an edit is DETECTED rather than overwritten, so devbin's own design anticipates a project needing a local fix. Each of the four edits is defensible on its own terms and each was made for a measured reason.

**Nothing in this project observes the aggregate.** There is no `int` command that reads `manifest.sha256` and compares it to the tree (verified: no handler under `bin/.devbin/cmd/` references the manifest at all). So a divergence is discovered only by someone who happens to run the measurement by hand, or at the moment a devbin upgrade is attempted from the Devbin source repository -- which is the worst time to discover it, because that is when it becomes a merge problem rather than a decision.

The result is a divergence whose SIZE nobody knows, whose MEMBERS nobody has enumerated, and whose existence is recorded only in session transcripts. That is the same shape as an unwired git hook (`int hooks`, three states): the thing is not failing, it is reporting nothing, **and reporting nothing is indistinguishable from being in the expected state.**

## Impact

No user-facing effect today, which is why the severity is medium rather than high.

What it costs is optionality, and it compounds. Every additional local patch makes the eventual reconciliation with upstream Devbin larger and less likely to be attempted; at some point "a project with a few local fixes" becomes "a fork nobody declared". **One of the four patches is a guard against false test verdicts** -- the class that has cost this estate real time twice in one day -- so the changes being lost or silently reverted in a future vendor sync is not a cosmetic risk.

It also means the question "is this project running stock devbin?" cannot be answered by anyone reading the repository, only by someone re-deriving it. Two of the four divergences had already been in the tree for some time and were not known to the people discussing the topic.

**What would raise this to high:** evidence that a devbin upgrade would revert rather than refuse (the manifest header says otherwise, and that has not been driven), or a fifth patch landing before this is resolved.

## Proposed Fix

Two parts, and the first does not depend on the second being decided.

**1. A detector in this project.** An `int` report that walks `manifest.sha256`, re-hashes each entry, and names every file in one of three states -- MATCHES, DIVERGED, MISSING -- plus files under the vendored tree that the manifest does not list at all. It REPORTS rather than refuses: refusing would block every session on a divergence nobody has ruled on yet, and a guard that must be bypassed is a guard nobody keeps. This is the same shape and the same argument as `int hooks`, whose bare form reports. It belongs under `bin/.devbin/cmd/`, which the manifest header explicitly says belongs to the project, so building it adds no further divergence.

**2. hv's ruling on the carry itself.** Whether these patches go upstream to Devbin (which the manifest's design invites, and which keeps this project on stock), or are declared a deliberate local fork with a recorded rationale. The argument for upstreaming FIRST rather than accreting further is that the largest of the patches is also the most general -- a runner that refuses a verdict when a gate's return code and its seal disagree is not Intent-specific -- and the most general change is the one with the best chance of being accepted, which makes it the worst one to land locally and forget.

**Deliberately NOT part of the fix: restamping the manifest.** Rewriting the checksums would make the report clean and erase the only signal that the divergence exists. The manifest is evidence, not a lockfile to be refreshed.

## Related

- ST0056 -- Intent v3.0.0 (the work these patches were made under)
- 0044 -- the issue whose sweep produced two of the four divergences

## Resolutions

{{TBC}}
