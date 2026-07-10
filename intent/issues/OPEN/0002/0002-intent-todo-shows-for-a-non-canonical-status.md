---
id: "0002"
title: intent todo shows [?] for a non-canonical status string
date: 2026-07-10
reporter: matts
status: OPEN
severity: low
---

# 0002: intent todo shows [?] for a non-canonical status string

## Tags

todo, st, canonical-status, status-synonyms

## Summary

`intent todo` renders `[?]` (unknown-status marker) for a steel thread whose `info.md` frontmatter carries a non-canonical status string -- eg `status: NOT-STARTED` (the directory-name form) instead of the canonical `status: Not Started`. `intent st` tolerates the same value because it routes status through `canonical_status` (bin/intent_st), but `intent todo`'s status -> marker mapping does not, so the thread falls through to `[?]` instead of bucketing under TODO with `[ ]`.

## Reproduction

    # In an ST info.md, hand-set the directory-name form:
    status: NOT-STARTED        # instead of "Not Started"
    $ intent todo
    - [?] STxxxx: ...          # expected: - [ ] STxxxx (TODO)

Observed 2026-07-10 on ST0046, whose frontmatter had been hand-edited to `status: NOT-STARTED`. Normalising it to `Not Started` fixed the marker.

## Root Cause

`intent todo`'s bucket/marker logic keys on the raw frontmatter `status:` value without passing it through the shared `canonical_status` synonym table that `intent st` uses. A synonym `intent st` accepts is therefore unknown to `intent todo` -- a Highlander gap (two status readers, one canonicalises, one does not).

## Impact

Cosmetic + confusing: a valid, tracked thread appears with an unknown-status marker in the flat view, and the two commands disagree about the same thread's status.

## Proposed Fix

Route `intent todo`'s status read through `canonical_status` (the single synonym table), so `todo` and `st` agree. Optionally, also normalise a non-canonical `status:` on write in `intent st` so the on-disk value is always canonical.

## Related

- ST0055 -- surfaced while normalising fleet issue trees / diagnosing ST0046

## Resolutions

{{TBC}}
