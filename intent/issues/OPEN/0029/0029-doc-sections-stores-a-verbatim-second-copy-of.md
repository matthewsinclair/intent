---
id: "0029"
title: doc_sections stores a verbatim second copy of every file's text; contentless FTS5 cuts the DB by 64 percent
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0029: doc_sections stores a verbatim second copy of every file's text; contentless FTS5 cuts the DB by 64 percent

## Tags

rust, intentsvcs, fts5, sqlite, search, measured, WP-13

## Summary

`doc_sections` is declared as an FTS5 virtual table **with no `content=` option**:

```sql
CREATE VIRTUAL TABLE doc_sections USING fts5 (
  owner_type UNINDEXED, owner_id UNINDEXED, file UNINDEXED, seq UNINDEXED,
  heading, level UNINDEXED, body,
  tokenize = 'porter unicode61'
)
```

In that mode SQLite stores **a full verbatim copy of every indexed document** in the `%_content` shadow table, in addition to the inverted index. The documents in question are markdown files that already exist on disk in the same project.

**Measured on Lamplight, via `dbstat`:** `doc_sections_content` is **69.5% of the entire database** and the real inverted index (`doc_sections_data`) is 27.9%. Switching to contentless FTS5 takes the same corpus from **82.49 MB to 29.62 MB -- a 64% reduction**, and inverts the ratio-to-source-text from **1.95x to 0.70x**.

## Reproduction

Measured 2026-08-15 by building the real schema over two real corpora, not by estimating.

**Where the bytes are** (Lamplight, 5,788 markdown files, 42.35 MB source):

```
doc_sections_content   60.1 MB   69.5%   <- verbatim second copy of the markdown
doc_sections_data      24.1 MB   27.9%   <- the actual inverted index
doc_sections_docsize    1.0 MB    1.1%
file_index              0.9 MB    1.0%
```

**The same corpus, contentless (`content=''`) plus a small `doc_section_loc` side table carrying file/seq/level/heading:**

```
stored-content FTS5 : 86,495,232 bytes (82.49 MB)   1.95x source
contentless FTS5    : 31,064,064 bytes (29.62 MB)   0.70x source
reduction           : 64%
```

The expansion ratio is linear across corpora and so this projects: Intent measured **1.97x** on 5.28 MB, Lamplight **1.95x** on 42.35 MB.

## Root Cause

Not a bug -- the table does exactly what its declaration asks for. The declaration asks for the wrong thing: **it treats FTS5 as the store of the prose rather than as an index over prose that is stored elsewhere.**

Under the reversed D01 the DB is the durable SSOT, which makes "the DB holds the text" sound correct, and it is _not_ the reason to keep the content here. D34 rules that the DB is per-machine and rebuilt from the extract, and the extract already carries prose bodies (`thread.json`, the authored `.md`). So the FTS content copy is a **third** copy of the same bytes -- extract, working file, and index -- and only the first two are load-bearing.

## Impact

**Not a correctness defect. Nothing is wrong today and no test should be failing.** Grading it honestly at medium rather than high on that basis.

What it costs:

- **Per-machine DB size**: 82 MB against 30 MB for Lamplight, and that is the DB an operator now backs up on a schedule (D35) and rebuilds on a clone (D34). It multiplies through every one of those operations.
- **It is 98.6% of the database.** Any statement about DB size, backup cost, rebuild time or snapshot retention is really a statement about this table.
- **It does NOT change the D34 ruling** and must not be read as reopening it. 29.62 MB still exceeds any sane commit-per-change budget -- ~3 GiB/year of history at Lamplight's commit rate -- so the DB stays out of git either way. This is about the cost of the artefact, not about where truth lives.

## Proposed Fix

Switch `doc_sections` to contentless FTS5 (`content=''`), with a companion table holding the unindexed locator columns (`file`, `seq`, `level`, `heading`) keyed by rowid -- which is the shape the probe measured, so the 29.62 MB figure already includes that table's cost.

**The real tradeoff, stated rather than buried: `snippet()` and `highlight()` stop working**, because FTS5 no longer has the text to excerpt. A search result that wants a snippet must re-read the file at the recorded path and offset. That is a genuine cost to `intent search`'s output quality and to its implementation, and it is **cc's call, not the reporter's** -- the measurement says what it saves, not that it is worth it.

Two things to weigh alongside:

- **AC-03.6 requires prose bodies to round-trip byte-identical out of the store and be retrievable by full-text query.** Contentless FTS5 cannot serve the retrieval half from the index. If the bodies are retrievable from the modelled `body` fields instead, the AC is still satisfiable through a different path -- but that path has to be named, and if the AC is currently green it is green through the copy this issue proposes to delete.
- **An external-content FTS5 table (`content='<table>'`) is the middle option**: it keeps `snippet()` working by pointing FTS5 at a real table, so the text is stored once rather than twice. That may dominate both alternatives and was not measured here.

**Whatever mode wins, the fix MUST also update the FTS ratio quoted in `.gitignore`** (vc, 2026-08-15). That comment now justifies the D34 ignore on the ceiling rather than on binary dirtiness, and cites **"FTS5 expansion is ~1.95x"** as the measurement doing the work. Halving the table falsifies that number while leaving the rule correct -- which is the D29 shape exactly: a true rule resting on a reason that has quietly become false, in a file nobody re-reads. The two move together or not at all. Recorded on both ends -- this issue and dc's inbox -- so neither depends on somebody remembering at the time.

## Related

- D34 -- the DB is per-machine truth and never committed; this is why its size is a local cost rather than a transport one
- D35 -- the rolling backup, whose cost scales directly with this table
- AC-03.6 -- prose bodies byte-identical out of the store AND retrievable by full-text query; check this before changing the mode
- WP-13 -- project search, which widens the corpus to the whole project and multiplies this
- Found while grounding hv's question about whether the DB could be committed; the size breakdown was a by-product of that measurement rather than its object

## Resolutions

{{TBC}}
