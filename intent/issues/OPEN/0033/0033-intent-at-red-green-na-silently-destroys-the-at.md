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

### CORRECTED 2026-08-15 (vc, on reading the write path instead of inferring it)

**The original root cause below named a mechanism that does not exist, and the correction makes the defect smaller, sharper, and fixable in one line.** It is kept verbatim underneath because the wrong version is what produced the wrong Proposed Fix, and the pair is the lesson.

The read path is not a split and the note is not unreadable. `at_field` matches the WHOLE row against an anchored regex -- `AT_GRAMMAR_TEST` / `AT_GRAMMAR_NONTEST` (`bin/intent_acceptance:180-181`) -- and returns one numbered capture group. The group numbers are deliberately aligned across both arms so one accessor serves both, and the source says so at `:171-172`: **1 id, 2 reference, 3 covers, 5 status, 6 note.** `AT_G_NOTE` (`:179`) is `( -- .*)?`, so **the note IS captured**, delimited by a space-hyphen-hyphen-space separator and nothing else (`:169`). Any caller could have it today for `at_field "$1" 6`.

**The whole defect is one greedy `.*` in the setter**, `bin/intent_acceptance:1341`:

```
replace_line "$acc" "s|^(- ${esc} .*) -- status:.*|\1 -- status: ${target}|"
```

`-- status:.*` matches from the status separator to end-of-line. **The note is inside that match, so it is consumed and replaced rather than never-read.** The row is not reconstructed from parsed fields at any point -- the setter never calls a field accessor at all.

That distinction is the whole repair: the parser had the note in hand and the writer overwrote it, so nothing needs to be modelled, taught, or ported. One anchored pattern has to stop being greedy past the status token.

### Original filing (WRONG about the mechanism -- see above)

`at_status()` is `at_field "$1" 5` (`bin/intent_acceptance:208`) -- the row is split on a space-hyphen-hyphen-space separator and the status is field 5. **The note is field 6.** The status write path reconstructs the row from the fields it knows about, so field 6 is not written back. The grammar documents the note; the field-index model of the row does not carry it.

**Also corrected here:** the original sentence wrote that separator as a code span with the spaces inside the backticks. The markdown linter strips the padding from such a span on save (CommonMark drops one space from each side), so a claim whose entire subject is that the delimiter is made of spaces silently became a claim about a bare double hyphen. **Written out in words above so the formatter cannot rewrite the fact.**

## Impact

**The damage is worse than the byte count, because of WHICH content lives in a note.** The AT note is where the discriminating case is recorded -- "the discriminating case is a store written BEFORE a schema change, and a test that opens a freshly-created store passes on the defect". That sentence is the entire defence against writing a vacuous test. It is written when the AT is created (`to-write`), and it is destroyed by the first status transition, which is **exactly when someone is about to go and write the test.**

So the tool deletes the specification at the precise moment it is needed, and reports success. The next person writes the test against the row's bare path and has no idea what it was supposed to discriminate.

Three aggravating properties:

- **Silent.** `ok: AT-02.7 -> red`. No warning, no diff, no count.
- **Invisible in review.** The row still parses, still lints clean, and still reads as a complete row. `intent at lint` reports `ok`.
- **It hits the careful user hardest.** A row with no note loses nothing. The loss is proportional to how much thought was written down.

Shipped in v2.19.0, which is also the release that introduced the AT row grammar and its `--fix` migrator -- so the note field has never survived a status change in any version that had notes.

**A FIFTH INSTANCE, 2026-08-16, and it is the one that should settle the severity: it happened to the node that holds this issue, on the same day, using the documented command, with the defect in working memory.**

AT-03.12's test had landed at `0e82b116` while its row still read `to-write`. Moving it to green requires passing through `red` (green is reachable only from red), so **two invocations of the correct verb, and the row went from 1,560 bytes to 106.**

```
$ git show e5a4fed7:.../acceptance.md | grep '^- AT-03.12' | wc -c
    1561
$ intent at red ST0056 AT-03.12 && intent at green ST0056 AT-03.12
ok: AT-03.12 -> red
ok: AT-03.12 -> green
$ grep '^- AT-03.12' .../acceptance.md | wc -c
     111
```

**1,447 characters destroyed**: the three arms the criterion required, the reason `todo_watermark.rs` catching the defect as a side effect would not have been sufficient, the fresh-clone precondition, and the explicit refusal of AT-02.8 and AT-04.5 as coverage. **Every one of those is a decision that took argument to reach and that nothing else in the tree records.**

Recovered in full from `git show`, so nothing is lost -- **and recovery was possible only because the row happened to have been committed before the transition.** A note written and moved in the same session is unrecoverable.

**Three things this instance adds that the first four did not.**

- **The transition graph MULTIPLIES the loss.** `to-write -> green` is refused, so recording a passing test costs two rewrites rather than one. **The status machine's correctness and this defect compound: the safer the graph, the more often the note is destroyed.**
- **Knowing about the defect did not prevent it.** There is no point in the workflow where the tool asks, and the success message is indistinguishable from a lossless one. **A defect that a fully-informed operator walks into is not a training problem.**
- **`intent at lint` reported `ok -- 112 AT row(s) conform` immediately afterwards.** The contract's own linter cannot see 1,447 characters leave, which is the invisible-in-review property above, observed rather than predicted.

**This raises the practical severity above `high` in one specific sense worth recording**: the four earlier instances were found by comparing against git. **Nobody is comparing routinely, so the measured instance count is a lower bound on a defect that leaves no trace in the file it damages.**

## Proposed Fix

### CORRECTED 2026-08-15 (vc) -- THE ORIGINAL FIX BELOW RECOMMENDED THE SHAPE THAT IS THE BUG

**The original recommendation was "rewrite only the status token in place (a targeted `sed`) rather than reconstructing the row from parsed fields", and called that shape preferable because "it cannot lose a field it never parses".** `bin/intent_acceptance:1341` already IS a targeted in-place `sed` that parses no fields, and it is the line that destroys the note. **The recommended cure was a description of the disease**, and anyone following it would have written the defect a second time and reasonably believed they had fixed it.

It read as sound because every clause of it was true. The setter does rewrite in place; it does not parse fields; that shape genuinely cannot lose a field it never parses. **What none of that establishes is the thing being claimed -- an in-place rewrite loses whatever its own pattern matches, and this one matches to end-of-line.** The property was real and simply not the property that mattered.

**The actual fix: stop the setter's trailing wildcard at the status token and re-emit what follows.** Shape, not a tested patch:

```
s|^(- ${esc} .*) -- status: [^ ]+(( -- .*)?)$|\1 -- status: ${target}\2|
```

The status vocabulary contains no spaces, so `[^ ]+` bounds it, and the note re-emits through the trailing group. **Verify against a row with a note, a row without one, and both grammar arms before landing it** -- the leading group is greedy and a note that itself contains the status separator is the case to check.

### Original proposal (kept -- it is the instance)

Preserve field 6 across a status write: capture the note before rewriting and re-append it, or rewrite only the status token in place (a targeted `sed` on a status-separator match) rather than reconstructing the row from parsed fields. The in-place token rewrite is preferable -- it cannot lose a field it never parses, and it is the same shape `at_fix_line` already uses for the emphasis and parenthetical-note normalisations.

### Guard

Add a guard asserting a row with a note survives a full `to-write -> red -> green` cycle byte-identical apart from the status token.

Not fixed at the time of filing: `bin/**` is not mutated in place in this repo while sessions are live, and ST0056/WP-04 replaces this tooling with the Rust facade. **The note that must not be lost in that port: the note is captured and then overwritten, so the port must not treat "the writer never saw it" as the thing being fixed.**

## Related

- ST0056 -- found while setting AT statuses during WP-02 and WP-06 verification; four notes destroyed and restored from git
- 0032 -- same file, same session: the gate ORs its covering ATs instead of ANDing them

## Resolutions

{{TBC}}
