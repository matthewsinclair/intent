# ST0011 `completed` NULL -- the evidence (vc, 2026-08-19)

AC-08.5 (ST0057) burning case #1. Measured at canon revision `8bb47e49`; v2 archaeology at `0ec2ac79` (the hoist).
Positional citations avoided: every figure below is reproducible from the two named revisions.

## The population, stated and closing

At `0ec2ac79` BOTH layouts coexisted -- the hoist wrote the flat layout without yet removing the buckets
(`1af21f4e` removed them afterwards).

111 `*/ST<NNNN>/info.md` paths = 55 bucketed (TRUE v2) + 56 flat (HOIST OUTPUT). Closes.
bucketed parents: COMPLETED 52 / CANCELLED 2 / NOT-STARTED 1.
flat-only id: ST0056 (created during the hoist, never had a bucket).

TRAP HIT AND RECORDED: my first sweep keyed a dict by ST id over all 111 paths, so the two copies of each
thread silently collapsed to whichever sorted last -- the FLAT one, ie the hoist's own output. It was
reading the artefact and calling it the source. Caught only because a later count came back 28 for
14 threads. This is the ambiguous-name-to-arbitrary-member class, committed while investigating.

## Finding 1 -- the carry is exonerated, by measurement

bucketed vs flat `completed` at the same revision, 55 compared: 0 differ.
threads holding a v2 frontmatter date and NULL in canon today: 0.

The hoist changed no completion value and lost none. NULL was not introduced by the migration.

## Finding 2 -- exactly one row is genuinely wrong, and it is ST0011

TRUE v2 (bucketed) frontmatter `completed:` EMPTY -- 2 threads:
ST0011 status=Completed frontmatter=EMPTY body prose = 2025-06-03 <-- WRONG
ST0046 status=Not Started frontmatter=EMPTY body prose = none <-- correct

NULL in canon today -- 4 threads: ST0011, ST0046 (not-started), ST0056 (wip), ST0057 (wip).
Three are correct. ST0011 is the one genuinely wrong row, confirming AC-08.5's own text by a
different method than the one that first found it.

## Finding 3 -- the v2 estate held this fact in TWO homes, and ST0011 is where they diverged

v2 `info.md` carried the completion date in frontmatter AND in body prose (`- **Completed**: ...`).
Across the estate: 39 frontmatter-only, 14 both, 1 body-only (ST0011), 2 neither.

Two homes for one fact is silent until they disagree. ST0011 is the single row where they did, and it
took the migration to a single-home model to surface it. The declared home was empty; the undeclared
home held the value; the carry read the declared home and got nothing. Nothing malfunctioned.

## Finding 4 -- the body date is evidence, not a template echo

Of the 14 threads carrying both (28 file-copies): frontmatter and body AGREE in 28 of 28.
body == created in only 2 of 28 -- so the body value varies independently and is a reliable second copy.

Caveat stated rather than buried: for ST0011 itself body == created (2025-06-03), so ST0011 is in the
2-of-28 same-day shape. It is NOT independently corroborated by varying from created.

## The proposed value, and why

ST0011.completed = 2025-06-03

1. Its own body prose says so.
2. 2025-06-03 is a BULK COMPLETION DATE in this estate: ST0001-ST0009 all carry exactly that value in
   frontmatter, against a created date of 2025-03-06. Someone swept a batch to Completed that day.
   ST0011 was created 2025-06-03 and marked completed in the same sweep.
3. The body is established as a reliable second copy (28/28).

So the value is consistent with the estate's own convention rather than inferred from ST0011 alone.
It records the DAY IT WAS MARKED, not the day work stopped -- which is what the other 51 rows record too.

## Why this is still not fixed

Two reasons, and the second is the criterion:

1. cc is mid-move of the 57 + 40 into `intent/.canon/`; no canon writes until the green ping.
2. THERE IS NO FIELD-SETTER VERB. The only route is a hand-edit of canon plus a whole-estate
   `sync --to-store`. That absence IS AC-08.5, and ST0011 is its worked example rather than a chore
   to be quietly discharged. Fixing it by hand without recording the absence would delete the evidence
   for the criterion.
