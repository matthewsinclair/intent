# v2-authored AT rows -- the corpus v3 did not produce

**Supplied by vc, 2026-08-17, for issue 0056's permanent regression test. Captured, not written.**

## Why this exists at all

Issue 0056's closing section states the requirement and the reason:

> A regression test must assert **byte-equality against a fixture written in v2's spelling** -- not that ingest-then-emit is stable, which it already is.

**`n-a` re-ingests to `Na` and re-emits to `n-a`, so from pass 2 the round trip is a fixed point.** A stability test seeded from v3's own output reports a clean pass and is RIGHT. The general form, which outlives this issue: _a stability property measured from the system's own output confirms that the system is consistent with itself, which is exactly what a one-time normalisation preserves._

**So the seed must be a corpus v3 did not produce.** Two weaker candidates were rejected on the way here, and the reasons matter more than the choice:

- **`display()` itself.** `view_determinism.rs`'s first regression test asserted `view.contains(format!("status: {}", status.display()))` -- comparing the renderer to the function that _defines_ the renderer's spelling. `display()` returning `n@a` passed it. True, and unable to discriminate.
- **A hand-authored seed.** vc generated one for the manual pairing on 2026-08-17: a real v2.19.0 project, real v2 templates, linting clean at 4/4. **Rejected as a committed fixture by its own author**, because it was written _for this test_ -- one degree closer to the thing under test than a fixture may be. Its authority would be vc's judgement about v2's grammar rather than v2's own output.

**The rows below have an authority neither node supplied: they were authored under v2, they have lived in this repository's committed canon, and every one of them predates issue 0056.**

## Provenance

| source                                       | commit     | authored   | statuses   | rows |
| -------------------------------------------- | ---------- | ---------- | ---------- | ---- |
| `intent/st/COMPLETED/ST0054/acceptance.md`   | `f28938c2` | 2026-08-13 | `n/a`      | 3    |
| `intent/st/NOT-STARTED/ST0046/acceptance.md` | `f28938c2` | 2026-08-13 | `to-write` | 9    |
| `intent/st/COMPLETED/ST0045/acceptance.md`   | `ee44f63b` | 2026-08-14 | `green`    | 2    |

Issue 0056 was filed **2026-08-17**. `AtStatus::display()` landed at **`d14cd0b5`**, the same day. **Every row here was last written before either event**, so no row can have been shaped by the defect or by its fix.

## `red` IS ABSENT AND THAT IS A DECLARED GAP, NOT AN OVERSIGHT

Measured across the estate at `3ce298c3`, constrained to column-0 `- AT-` rows: **140 `green`, 62 `to-write`, 22 `n/a`, 4 `red`.**

**All four `red` rows live in `intent/st/ST0056/acceptance.md`**, the thread being edited today -- so there is no uncontaminated source for that variant and this fixture does not fabricate one.

**A test standing on this corpus must SAY so rather than read as four-of-four.** `red` is one of the three variants where the wire tag and the authored form COINCIDE, and **that coincidence is the entire hiding mechanism of issue 0056** -- echoing the wrong source is correct for `green` and `red` and wrong only for `n/a`. So a reader must not infer from a green that all four were compared against authored bytes. **Three compared against the corpus; one against a transcription; stated as such.**

## The discriminating row, named so it is not optimised away

**`ST0046` AT-01.1's note contains the row separator.**

```
-- status: to-write -- red-first; modules check -- unregistered fixture flagged
```

The note is introduced by a spaced `--` and then **contains another spaced `--`**. v2's grammar handles this by never parsing the note (`AT_G_NOTE='( -- .*)?'` is greedy to end-of-line). **Any v3 reader that splits on `--` rather than anchoring the status will round-trip this row wrongly**, and it will do so silently, because the two halves still look like a note. **Nine of nine ST0046 rows carry the same shape** -- corrected from "eight of the nine" by cc, 2026-08-17, and re-measured here: every `to-write` row is `status: to-write -- red-first; modules <check|sync> -- <text>`. It changes nothing about the hazard and it is recorded because a count in a fixture README is a claim someone will scope a test to.

This is the row most likely to fail and therefore the row worth keeping. **Do not "simplify" the fixture by dropping it.**

## What a test built on this should assert

**Parse each line through the v2 reader, render it through `views`, and require the output byte to equal the input byte.** Not `contains`, not a status-token comparison: the whole line. A status check would pass on a row whose coverage list or note had been reordered, and the covers list and the note are the two fields issue 0033 already destroys elsewhere.

The rows live in `rows.txt`, one per line, verbatim, no trailing edits. Regenerate by re-capturing from the commits above -- never by rendering them.

## Verification of the capture itself

- **Captured from the commits, not from the worktree** (`git show <sha>:<path>`), then diffed against the worktree: **identical**, so the provenance claim holds in both directions and the file is not a snapshot of an uncommitted edit.
- **All three source threads lint clean under the v2 that scores this repository**, measured at `3ce298c3`:

```
ST0054   lint: ST0054 ok -- 3 AT row(s) conform
ST0046   lint: ST0046 ok -- 9 AT row(s) conform
ST0045   lint: ST0045 ok -- 2 AT row(s) conform
```

14 rows: 9 `to-write`, 3 `n/a`, 2 `green`.
