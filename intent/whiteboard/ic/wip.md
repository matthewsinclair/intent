---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 14:01Z
status: active
focus: "Queue clear. The command inventory turns out to be UNREPRODUCIBLE -- its probe input was never tracked and is gone -- so the drift check's measurement half is stamp-only. Refusal added, raised to vc, rule 13 landed with the evidence."
claims: []
---

# Interface Claude (ic)

## THE CANON -- ratified, supersedes everything earlier

**The db is where the truth lives.** Not a cache of it, not an index over the real files -- the thing itself. Everything on disk is a copy taken out of it or a candidate queued to go in.

1. intentdb = durable SSOT. **Nothing on disk is truth.**
2. All of intentsvcs works FROM the db.
3. Sync runs BOTH ways, manual or daemon-triggered. **Transport is bidirectional; authority is not.**
4. The typed Rust API is the ONLY door in -- conformance by construction. **The gate makes an ingested file trustworthy, not its format.**
5. Re-creation from an extract is a CAPABILITY, not a licence to treat the db as scratch.
6. **Migrations are NORMAL.** "No DB migrations, ever" is DELETED and was never hv's constraint.
7. The real standing requirement is **PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6) -- 1-1 db-entity-to-`.json`/`.md`, lossless, **usable without Intent**. That is what bidirectional sync is FOR.
8. **The three state machines are ratified** (`data-model.md`): thread `Triage -> NotStarted -> Wip -> Completed` + `Hold`/`Cancelled`; wp `NotStarted -> Wip -> Done`; AC one enum `Satisfied|Unsatisfied|Descoped|Withdrawn`. **No terminal states.**

## DOING

**Queue clear. Nothing owed to any node.** Landed: config keys (`58c48fc`), vc's rulings (`593878a`), view completeness (`c1fa48c`), cc's three follow-ons (`f5622f0`), the inventory refusal (`a886f75`), three measurement rules (`bd2bab5`).

- **THE COMMAND INVENTORY CANNOT BE REGENERATED.** `gen_inventory.sh` only READS `probes/toplevel.tsv`; that file lives in throwaway scratch, was **never tracked**, and is gone. So the 26 `cmd-*.md` are stamp-only and **the drift check's MEASUREMENT half cannot be content-checked** -- a weaker claim than "drift ok/26" has been carrying. Corrects my own coverage line: those 26 are not merely un-re-derived, they are un-re-derivABLE from committed state.
- **Re-running would have destroyed them.** awk on a missing file exits 2 producing NOTHING (the dash fallback never runs), and `set -uo pipefail` without `-e` does not stop it -- 26 husks carrying the good revision stamp. **And every file's header tells the reader to re-run it.** Two refusals added, mutation-tested three ways.
- **Recommended to vc, not done unilaterally:** re-run the probe step at `69d42a7` and commit the TSV. A re-measurement is a contract act -- if it disagreed with the committed files I would be both the producer of the discrepancy and its judge.
- **The VIEW was dropping 15 of 20 authored target fields.** Skew cannot see this: it tests re-derivability, and a lossy generator is a fixed point with itself. Completeness + formatter-fixed-point refusals added.

## Open with others -- nothing owed by this node

1. **vc:** `--list` on `intent backup` is **proposed by me, not ruled** -- strike it if the contract wants the bare trigger. **hv:** does "configurable from `intent config`" mean a writable `config set`? I did not invent one; cc is unblocked either way.
2. **vc + hv:** the machine guards **every** edge into `Cancelled` with "reason recorded"; v2 `st cancel` takes **no `--reason`** (measured, flags empty). Either the row becomes `corrected` or the guard is aspirational. **A ratified guard is not reconciled by editing the surface it binds.**
3. **cc:** `st reopen` has a file-system half -- `st done` RELOCATES the thread directory, so reopen must move it back; a half-applied reopen leaves a thread findable under neither status.
4. **cc:** `TBC` must not become a state; `intent_st:941` pins render order as a five-element array literal that now grows.
5. **vc:** the inventory TSV cannot simply be committed -- **it no longer exists.** Recovering content-checking for those 26 artefacts needs a re-probe at `69d42a7`. Offered; awaiting their word. Tell dc when it lands so they re-report coverage rather than assume.
6. **hv, raised directly (their instruction -- the hv inbox is durability, not a queue):** the PUBLIC-repo `session_id` question from 09:12Z is still unanswered; whether `configurable from intent config` means a writable `config set`; and whether `-s|--start` may still jump two edges under the ratified machine.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **A STRUCTURED QUERY IS A NEEDLE AND REPORTS ON THE SUBTREE IT TRAVERSED.** My `jq '.families[].entries[]'` audit missed **all three** old-model strings -- they live in the top-level `new_surface[]` array. A grep caught what the structured query could not, **because I queried the shape I REMEMBERED instead of the shape the file HAS.** jq-only would have reported this lane clean, with a method behind it, and been wrong.
- **A GUARD WITH NO POSITIVE CONTROL CANNOT TELL "NOTHING IS WRONG" FROM "NOTHING RAN".** Four credential sweeps returned clean and all four were VACUOUS (`$FILES` unquoted in zsh is one argument). **Run a control that MUST match, first.** One-off sweeps need it most: nothing downstream will ever contradict them.
- **I REASON FROM THE NAME RATHER THAN FROM THE THING.** `st_zero`, `wp scope` -- both caught by a peer. **Open the definition before arguing about the label.** Worked this time: I hypothesised `st_list_all_vocabulary.bats` would deviate under six states, **read it, and it does not** -- it asserts behaviour, not the vocabulary set.
- **ic cannot certify a green suite.** matts owns the authoritative run. Everything from this node is evidence; label it that way.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`, four sessions live. `native/**` and `bin/.devbin/**` are safe.
- **THIS REPOSITORY IS PUBLIC** (`matthewsinclair/intent`). The environment brief says "assume private" and is **wrong in the dangerous direction**.
- **A CONTROL REFUSES; DOCUMENTATION REMINDS; ONLY ONE IS LOAD-BEARING.** Three nodes broke three rules _while enforcing them_; only the mechanisms that REFUSED held.
- **ASSERT THE FIXTURE REACHED THE BRANCH BEFORE READING ITS VERDICT** (dc). A staged set can be empty and the run silently takes the full-sweep branch.
- **A VERIFICATION IS ONLY AS CURRENT AS THE THING IT READ**, and nothing tells you when that expires. I committed against two Rust paths verified minutes before the tree moved again.
- **A needle reports on the set it MATCHED, never the set it was aimed at.** Count what it matches before building on it -- the `GENERATED` banner would have covered 1 file in 30.
- **`set -euo pipefail` + grep's no-match exit 1 kills a pipeline silently**; every pipeline whose emptiness is legitimate needs `|| true`.
- **`--only` commits what you NAME, and a move is TWO facts** (dc). **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.**
- **This shell is zsh**: no word-splitting of unquoted parameters.
- **Never enumerate remotes through `head`**, and **a result right by coincidence certifies the method**, which is worse than a wrong one.
