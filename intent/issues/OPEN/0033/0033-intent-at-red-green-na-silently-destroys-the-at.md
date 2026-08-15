---
id: "0033"
title: intent at red/green/na silently DESTROYS the AT row's note, which is where the discriminating case is written down
date: 2026-08-15
reporter: matts
status: OPEN
severity: high
---

# 0033: intent at red/green/na silently DESTROYS the AT row's note, which is where the discriminating case is written down

## Tags

acceptance, data-loss, silent-failure

## Summary

The AT row grammar admits a trailing note -- `status: to-write|red|green|n/a[ -- <note>]`, documented at `bin/intent_acceptance:10-11` and `:152-153`. **`intent at red|green|na` rewrites the row and does not carry the note across.** It is deleted, in place, with no warning and a success message.

Measured on ST0056, 2026-08-15, across the four rows a single session's status changes touched:

| row      | before | after | lost |
| -------- | ------ | ----- | ---- |
| AT-02.7  | 779    | 107   | 672  |
| AT-03.10 | 364    | 102   | 262  |
| AT-06.4  | 663    | 101   | 562  |
| AT-06.7  | 707    | 105   | 602  |

**2,098 characters of authored contract content, destroyed by four invocations of the documented, correct command.**

## Reproduction

```
$ grep '^- AT-02.7 ' acceptance.md | wc -c
779
$ intent at red ST0056 AT-02.7
ok: AT-02.7 -> red
$ grep '^- AT-02.7 ' acceptance.md | wc -c
107
```

Recoverable only from git, and only if the note was committed before the setter ran. A note written and then set green in the same session is gone with no copy anywhere.

## Root Cause

`at_status()` is `at_field "$1" 5` (`bin/intent_acceptance:208`) -- the row is split on `--` and the status is field 5. **The note is field 6.** The status write path reconstructs the row from the fields it knows about, so field 6 is not written back. The grammar documents the note; the field-index model of the row does not carry it.

## Impact

**The damage is worse than the byte count, because of WHICH content lives in a note.** The AT note is where the discriminating case is recorded -- "the discriminating case is a store written BEFORE a schema change, and a test that opens a freshly-created store passes on the defect". That sentence is the entire defence against writing a vacuous test. It is written when the AT is created (`to-write`), and it is destroyed by the first status transition, which is **exactly when someone is about to go and write the test.**

So the tool deletes the specification at the precise moment it is needed, and reports success. The next person writes the test against the row's bare path and has no idea what it was supposed to discriminate.

Three aggravating properties:

- **Silent.** `ok: AT-02.7 -> red`. No warning, no diff, no count.
- **Invisible in review.** The row still parses, still lints clean, and still reads as a complete row. `intent at lint` reports `ok`.
- **It hits the careful user hardest.** A row with no note loses nothing. The loss is proportional to how much thought was written down.

Shipped in v2.19.0, which is also the release that introduced the AT row grammar and its `--fix` migrator -- so the note field has never survived a status change in any version that had notes.

## Proposed Fix

Preserve field 6 across a status write: capture the note before rewriting and re-append it, or rewrite only the status token in place (a targeted `sed` on ` -- status: <old>` -> ` -- status: <new>`) rather than reconstructing the row from parsed fields. The in-place token rewrite is preferable -- it cannot lose a field it never parses, and it is the same shape `at_fix_line` already uses for the emphasis and parenthetical-note normalisations.

Add a guard asserting a row with a note survives a full `to-write -> red -> green` cycle byte-identical apart from the status token.

Not fixed at the time of filing: `bin/**` is not mutated in place in this repo while sessions are live, and ST0056/WP-04 replaces this tooling with the Rust facade. **The note that must not be lost in that port: the row is not a tuple of five fields.**

## Related

- ST0056 -- found while setting AT statuses during WP-02 and WP-06 verification; four notes destroyed and restored from git
- 0032 -- same file, same session: the gate ORs its covering ATs instead of ANDing them

## Resolutions

{{TBC}}
