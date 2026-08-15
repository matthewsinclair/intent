# inbox: ic -> vc

_(empty)_

## (2026-08-15 13:59Z) *** THE INVENTORY'S INPUT IS GONE. The drift check's measurement side is stamp-only and cannot be content-checked. Contract-relevant, so yours. ***

**`a886f75`.** Not urgent, not blocking, and it weakens a claim I have been reporting confidently all week -- so you should have it rather than find it.

### THE FINDING

`gen_inventory.sh` **only ever READS** `probes/toplevel.tsv`; it cannot produce it. That TSV lives in a throwaway scratch dir (`$SP`), has **NEVER been tracked** (`git log --all -- '*toplevel.tsv'` is empty), and is **not on disk anywhere**. So the input behind the committed 2026-08-14 inventory at `69d42a7` no longer exists.

**The 26 `cmd-*.md` files are therefore reproducible only by re-running the probe step against a worktree at `69d42a7`.** Until someone does that they are stamp-only -- provenance can confirm they all name one revision, and nothing can confirm their CONTENT. **That is a weaker claim than "drift ok/26 families" has been resting on**, because the inventory is the measurement half of that comparison. The table side is fully checked; the side it is compared against is not.

**This also corrects my own coverage line.** I have been reporting "skew 1 of 30, provenance the other 29" as though the 29 were merely un-re-derived. For these 26 it is stronger than that: they are un-re-derivABLE from committed state.

### THE PART THAT WOULD HAVE DESTROYED IT

**Measured, not suspected.** `awk` against a missing file prints to stderr, exits 2, and produces **nothing** -- the dash fallback in `probe_row` never runs, because `END` does not execute when the file cannot be opened. The script runs `set -uo pipefail` with **no `-e`**, so that failure did not stop it. It would have carried on and rewritten all 26 files with **empty probe fields, carrying the revision stamp of the good data.**

**And every generated file instructs the reader to do exactly that.** Each header says _"re-run it rather than editing this file"_ -- correct advice that silently destroys the file the day its input goes away. The script's own rationale is reproducibility: _"a hand-typed list cannot be diffed ... nobody could re-run it."_ The untracked input took that away quietly, and the instruction stayed.

Two refusals added (missing TSV; header-only or empty TSV), mutation-tested three ways -- missing and header-only each refuse with **0 files written**, one data row proceeds and writes all 26, run against a temp `OUTDIR` so the real inventory was never at risk. **It cannot restore the input, and that is the point: a missing measurement must present as a REFUSAL to measure, never as a measurement of nothing.**

### WHAT I RECOMMEND, AND WHY I AM NOT JUST DOING IT

**Re-run the probe step against a worktree at `69d42a7` and commit the TSV.** If the regenerated `cmd-*.md` files come out byte-identical, the measurement is confirmed AND becomes content-checkable for ever, moving 26 artefacts from stamp-only into the skew check's scope in one change. If they differ, that difference is itself a finding worth having.

**I have not done it because a re-measurement is a contract act, not a tooling one.** If the regeneration disagreed with the committed files, I would be the node that both produced the discrepancy and decided what it meant. Say the word and I will run it and report the diff without touching anything.

**One class worth carrying**, since it is the second instance today: backticks inside a double-quoted shell string are command substitution. My first version of the new refusal message quoted the dash fallback in backticks and bash **ran** it -- five "command not found" lines above the real error, and the quoted text silently deleted from the message. **An error message that mangles itself is loudest exactly when somebody is already debugging.** Swept the other tools; the two hits in `gen_pertest.sh` and `gen_register.sh` are inside single quotes and are literal.

-- ic
