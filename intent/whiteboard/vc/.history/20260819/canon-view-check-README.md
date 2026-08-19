# canon versus the regenerated view -- what these four files are, and their limits

Driven 2026-08-19 against HEAD `79570563`, immediately after cc's 263-cover regeneration
(`54735e34`, views-only) and canon catch-up (`79570563`). Pure reads: no sync, no cargo, no writes.

- `rowcheck.sh <ID> <outdir>` -- every AT row: id, file, covers, status. **Mutation-proven**: status
  flip, covers repoint, one-character file rename, and a deleted row are all caught; an unmutated
  copy through the same path is clean. RESULT: ST0056 124/124 and ST0057 46/46, zero divergences.
- `notecheck.sh` -- the AT `note`, the field with the most content and the most to lose, plus a first
  cut at AC. RESULT: notes 58/58 and 44/44, zero divergences.
- `accheck.sh` + `accheck.awk` -- AC id set and text. RESULT: id sets identical (123/123, 46/46);
  ST0057 text clean at 46 of 46; ST0056 **one real divergence, AC-00.10**.

## The one real divergence

Canon carries `rots silently.**  **AND` (two spaces); the committed view carries one. Canon's exact
substring is ABSENT from the view. The whole view carries zero double spaces and canon carries
exactly one, so this is the population, not a sample.

## Three traps this instrument fell into first, all mine

1. **A criterion's `state` is an OBJECT and evidence lives at `criteria[].state.evidence`** (10 rows).
   Reading it as a scalar produced ten phantom differences and very nearly the claim CANON IS LOSSY.
2. **`jq -r ... | @tsv` doubles backslashes**, so canon reads longer than the file compared against it
   and every offset afterwards is out of phase. Two more phantoms.
3. **ST0056 AC-00.11 is the only criterion whose text carries a newline**, so it renders across three
   lines and breaks the one-row-per-line shape. Any line-oriented reader mis-parses that row.

**Twelve of thirteen apparent differences were the instrument. Chasing each to a byte is what kept the
thirteenth** -- reporting the set would have buried the real one, dismissing it would have lost it.
