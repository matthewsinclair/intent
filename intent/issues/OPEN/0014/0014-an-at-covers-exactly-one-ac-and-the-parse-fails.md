---
id: "0014"
title: "AT coverage is comma-separated only: 'and' as a separator, or an id with an adjacent character, drops the link silently"
date: 2026-08-04
reporter: matts
status: OPEN
severity: medium
---

# 0014: AT coverage is comma-separated only: "and" as a separator, or an id with an adjacent character, drops the link silently

## Tags

acceptance, parsing, silent-failure

> **This issue was filed on 2026-08-04 with the wrong root cause and corrected the same day.** The original text claimed "an AT covers exactly one AC". **That is false** -- multi-AC coverage works and is documented: ST0087's contract preamble states "Multi-AC coverage on an AT is comma-separated", and `AT-04.1 ... -- covers AC-04.1, AC-04.2` links both. The correction was caught by a reviewing node before the wrong rule was applied as a sweep across four steel threads, which would have split acceptance criteria to work around a limit that does not exist. Recorded rather than quietly edited, because a wrong issue is worse than no issue: it is read as a finding.

## Summary

AT-to-AC coverage is derived by scanning the AT's line in `acceptance.md` for `covers AC-NN.N` ids. **Multiple ids are supported and must be comma-separated.** Anything else in that position is dropped without complaint:

1. **`and` as a separator** -- `covers AC-09.2 and AC-04.3` links only the first.
2. **Any character fused to the id** -- `covers AC-09.1's city half` links neither, because the possessive breaks the match.

In both cases `intent ac list` reports the affected AC as `covered-by: -`, which renders **identically to never having written the AT at all**. The AT itself lists as green and looks fine.

The same silence covers at least two neighbouring format rules, reported together because they share the failure mode:

3. **The non-test marker must read exactly `(non-test)`.** `(non-test, boundary)` silently makes the AC test-backed, and it then cannot be satisfied by evidence at all.
4. **The evidence delimiter is the FIRST `--` on the line**, so an AC whose prose uses a dash before `-- evidence:` parses its own sentence as the evidence.

## Reproduction

Observed on Laksa ST0086, 2026-08-04. Written in `acceptance.md`:

```
- AT-09.1 `.../snorkeltoast_catalog_test.exs` -- covers AC-09.1's city and artist halves ... -- status: green
- AT-09.2 `.../snorkeltoast_test.exs` -- covers AC-09.2 and AC-04.3: ... -- status: green
```

Both ATs real, both green, both registered:

```
$ intent at list ST0086
at: AT-09.1  test/laksa_web/themes/snorkeltoast_catalog_test.exs  green
at: AT-09.2  test/laksa_web/themes/snorkeltoast_test.exs          green
```

But:

```
$ intent ac list ST0086 | grep -E "AC-04.3|AC-09"
ac: AC-04.3  covered-by: -        satisfied: no     # "and" separator: dropped
ac: AC-09.1  covered-by: -        satisfied: no     # possessive: dropped
ac: AC-09.2  covered-by: AT-09.2  satisfied: yes    # first id, bare: matched
```

The working form, on ST0087, for contrast:

```
- AT-04.1 `.../site_files_test.exs::success: files reads the bytes` -- covers AC-04.1, AC-04.2 -- status: green

$ intent ac list ST0087 | grep -E "AC-04.1|AC-04.2"
ac: AC-04.1  covered-by: AT-04.1 AT-04.2 AT-04.3  satisfied: yes
ac: AC-04.2  covered-by: AT-04.1 AT-04.2 ...      satisfied: yes
```

## Root Cause

The scan matches ids by adjacency rather than at a token boundary, and treats the comma as the only separator. Both are defensible parsing choices in isolation; the defect is that **a failed match is indistinguishable from an absent AT.** Nothing errors, nothing warns, and the failure surfaces later as a coverage number quietly too low.

`intent ac list` was reported to warn on these; measured on ST0086, it emits nothing matching `warn` at all. If a warning path exists it is not reaching this case.

## Impact

- **Silent under-coverage.** A thread carries ATs that were written, run and greened, while the contract reports the ACs uncovered. The work is done and the record says it is not.
- **It surfaces late and looks like something else.** The first symptom is `intent wp done` refusing to close a package, which reads as "the work is not finished" rather than "the line is phrased wrongly".
- **It punishes the more careful author.** A descriptive AT line -- naming in prose what it covers -- is what breaks the match. A terse line works.
- **It produces confident wrong generalisations.** This issue is the evidence: hitting the two broken forms and not the working one, the natural inference was a one-AC-per-AT limit, which is wrong and was about to be applied as a project-wide sweep. **A silent parse failure does not just lose data; it teaches the reader a false rule.**

## Proposed Fix

Ordered by value, and the first is worth more than the rest combined:

1. **Warn on every AT line whose `covers` clause does not fully parse.** Name the ids that matched and the text that did not:

   ```
   warn: AT-09.2 -- coverage clause partially parsed.
         matched: AC-09.2   unmatched text: "and AC-04.3"
         Multi-AC coverage is comma-separated: covers AC-09.2, AC-04.3
   ```

2. **Match ids at a token boundary**, so `AC-09.1's`, `AC-09.1,` and `(AC-09.1)` resolve. Optionally accept `and` / `&` as separators; the warning matters more than the leniency.
3. **Apply the same "parsed nothing, said nothing" audit** to the `(non-test)` marker and the `--` evidence delimiter above. Three format rules with one shared failure mode is a parser-wide policy question, not three fixes.
4. **Document the comma rule where an author will meet it.** It is stated in one project's contract preamble and nowhere in the tool's own help.

## Related

- 0013 -- `intent ac` has no descope verb. Same family: acceptance-contract state the tool models more simply than the practice needs, failing quietly rather than loudly.
- Found on Laksa ST0086 while closing WP-04, WP-09 and WP-10; the false root cause was caught by that project's validation node.

## Resolutions

{{TBC}}
