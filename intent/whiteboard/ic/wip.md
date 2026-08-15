---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 13:52Z
status: active
focus: "cc unblocked and their three follow-ons landed. The generated VIEW was dropping 15 of 20 authored target fields -- fixed, with a completeness refusal and a formatter fixed-point refusal so neither class recurs."
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

**Landed since the hold lifted:** config keys (`58c48fc`), vc's rulings + `doctor`'s obligations (`593878a`), the view-completeness fix (`c1fa48c`), `st cancel`/`st new`/`sync` (`f5622f0`).

- **THE VIEW WAS DROPPING 15 OF 20 AUTHORED TARGET FIELDS, SILENTLY** -- including the config keys cc was blocked on. **The skew check cannot see this and never could**: it asks whether the view matches what the generator PRODUCES, so a lossy generator is a perfect fixed point with itself. Skew tests re-derivability; nothing tested COMPLETENESS. Two refusals added (completeness + formatter fixed point), both mutation-tested.
- **`st cancel` conflict RESOLVED, guard wins** -- `--reason` declared, disposition `keep` -> `corrected`. cc's `ReasonRequired` refusal is why leaving it open was safe: a loud unimplemented guard costs one error message; a silent one would have put unexplained `Cancelled` threads in the record permanently.
- **`st new` is `corrected`** -- enters at `Triage`. `-s|--start` jumping to `Wip` is now two edges at once; flagged for vc/hv, not re-pointed by me.
- **`sync` selector**: `--to-disk` / `--to-store`, **naming the destination** because that is the side overwritten. Bare verb keeps refusing -- opposite blast radii, no safe default.

## Open with others -- nothing owed by this node

1. **vc:** `--list` on `intent backup` is **proposed by me, not ruled** -- strike it if the contract wants the bare trigger. **hv:** does "configurable from `intent config`" mean a writable `config set`? I did not invent one; cc is unblocked either way.
2. **vc + hv:** the machine guards **every** edge into `Cancelled` with "reason recorded"; v2 `st cancel` takes **no `--reason`** (measured, flags empty). Either the row becomes `corrected` or the guard is aspirational. **A ratified guard is not reconciled by editing the surface it binds.**
3. **cc:** `st reopen` has a file-system half -- `st done` RELOCATES the thread directory, so reopen must move it back; a half-applied reopen leaves a thread findable under neither status.
4. **cc:** `TBC` must not become a state; `intent_st:941` pins render order as a five-element array literal that now grows.
5. **Mine, unblocked, blocks nobody:** `gen_inventory.sh` renders from an untracked `probes/toplevel.tsv`. Committing it moves 27 artefacts from stamp-only to content-checked in one change. Tell dc when it lands so they re-report coverage rather than assume. Also owed to `parity.md`: measurement rule 13 + the enumerate-don't-sniff rule.

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
