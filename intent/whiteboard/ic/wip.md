---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 14:14Z
status: active
focus: "hv's three rulings actioned. config get/set authored. EXP-03 raised BEFORE WP-09 opens: two ACs say the MCP tier generates from the table and the table cannot answer the first question either generator asks."
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

**Queue clear; nothing owed.** Landed today: config keys (`58c48fc`), vc's rulings (`593878a`), view completeness (`c1fa48c`), cc's follow-ons (`f5622f0`), inventory refusal (`a886f75`), three measurement rules (`bd2bab5`), `config get`/`set` (`b91b086c`), EXP-03 (`e1a9c31`).

- **EXP-03, raised BEFORE WP-09 opens.** AC-09.1 says the MCP tool tier generates from this table; AC-09.4 forbids a hand-maintained command list. **No row says whether it is exposed on MCP, or whether it reads or mutates** -- measured across all 103. A generator must then either expose everything (`intent mcp` as a tool inside an MCP server) or carry a skip list, **which is a hand-maintained command list one command from the AC forbidding them.** Fields proposed; **classifying 103 rows is a safety judgement and is vc's and cc's**, so I authored the exposure and not the classification.
- **`config get`/`config set` authored** on hv's ruling. Load-bearing decision: **an unknown key is REFUSED, never created**, with the valid-key set **derived from the declared schema**. Values carry their declared type -- `set backup.enabled false` writes JSON `false`, since the string form turns "disable" into "enable".
- **EXP-01 predicted a defect and then it happened to me.** It called the zero-emphasis state luck; I wrote four emphasis spans and broke the skew check. **A register that predicts and does not prevent has done the cheaper half.** Fixed-point refusal now closes it.

## Open with others -- nothing owed by this node

1. **vc:** `--list` on `intent backup` is **proposed by me, not ruled** -- strike it if the contract wants the bare trigger. **hv:** does "configurable from `intent config`" mean a writable `config set`? I did not invent one; cc is unblocked either way.
2. **vc + hv:** the machine guards **every** edge into `Cancelled` with "reason recorded"; v2 `st cancel` takes **no `--reason`** (measured, flags empty). Either the row becomes `corrected` or the guard is aspirational. **A ratified guard is not reconciled by editing the surface it binds.**
3. **cc:** `st reopen` has a file-system half -- `st done` RELOCATES the thread directory, so reopen must move it back; a half-applied reopen leaves a thread findable under neither status.
4. **cc:** `TBC` must not become a state; `intent_st:941` pins render order as a five-element array literal that now grows.
5. **vc:** the inventory TSV cannot simply be committed -- **it no longer exists.** Recovering content-checking for those 26 artefacts needs a re-probe at `69d42a7`. Offered; awaiting their word. Tell dc when it lands so they re-report coverage rather than assume.
6. **ALL THREE hv QUESTIONS ANSWERED (2026-08-15).** Public-repo: closed, not a user risk, dev apparatus is not shipped surface. `config set`: yes, new surface -- authored at `b91b086c`. `-s|--start`: cc's and vc's call, row left unchanged pending it. **Raise hv asks directly or via vc -- the hv inbox is durability, not a queue.**

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **A STRUCTURED QUERY IS A NEEDLE AND REPORTS ON THE SUBTREE IT TRAVERSED.** My `jq '.families[].entries[]'` audit missed **all three** old-model strings -- they live in the top-level `new_surface[]` array. A grep caught what the structured query could not, **because I queried the shape I REMEMBERED instead of the shape the file HAS.** jq-only would have reported this lane clean, with a method behind it, and been wrong.
- **A GUARD WITH NO POSITIVE CONTROL CANNOT TELL "NOTHING IS WRONG" FROM "NOTHING RAN".** Four credential sweeps returned clean and all four were VACUOUS (`$FILES` unquoted in zsh is one argument). **Run a control that MUST match, first.** One-off sweeps need it most: nothing downstream will ever contradict them.
- **I REASON FROM THE NAME RATHER THAN FROM THE THING.** `st_zero`, `wp scope` -- both caught by a peer. **Open the definition before arguing about the label.** Worked this time: I hypothesised `st_list_all_vocabulary.bats` would deviate under six states, **read it, and it does not** -- it asserts behaviour, not the vocabulary set.
- **ic cannot certify a green suite.** matts owns the authoritative run. Everything from this node is evidence; label it that way.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`, four sessions live. `native/**` and `bin/.devbin/**` are safe.
- **THIS REPOSITORY IS PUBLIC** (`matthewsinclair/intent`) -- true, and **hv has ruled it is not a user-facing risk, which corrects how I framed it.** A consumer installs `intent`/`intentd` from a tap and never receives our boards, sweeps, registers or session identifiers. **This repo's dev/PM apparatus is NOT shipped surface**; the only audience for it is someone reading the repo to see how Intent works, which is intended. What remains is ordinary: no secrets in commits, and write knowing it is readable. **The same distinction cuts the other way for project config, which IS user-facing** -- Intent dogfooding itself is exactly what makes one `config.json` look like a dev artefact.
- **A CONTROL REFUSES; DOCUMENTATION REMINDS; ONLY ONE IS LOAD-BEARING.** Three nodes broke three rules _while enforcing them_; only the mechanisms that REFUSED held.
- **ASSERT THE FIXTURE REACHED THE BRANCH BEFORE READING ITS VERDICT** (dc). A staged set can be empty and the run silently takes the full-sweep branch.
- **A VERIFICATION IS ONLY AS CURRENT AS THE THING IT READ**, and nothing tells you when that expires. I committed against two Rust paths verified minutes before the tree moved again.
- **A needle reports on the set it MATCHED, never the set it was aimed at.** Count what it matches before building on it -- the `GENERATED` banner would have covered 1 file in 30.
- **`set -euo pipefail` + grep's no-match exit 1 kills a pipeline silently**; every pipeline whose emptiness is legitimate needs `|| true`.
- **`--only` commits what you NAME, and a move is TWO facts** (dc). **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.**
- **This shell is zsh**: no word-splitting of unquoted parameters.
- **Never enumerate remotes through `head`**, and **a result right by coincidence certifies the method**, which is worse than a wrong one.
