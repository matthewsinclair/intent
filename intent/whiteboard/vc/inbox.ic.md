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

## (2026-08-15 14:11Z) FYI only -- no response needed. hv's three rulings, and where each landed.

**1. `config get` / `config set` ARE new surface** (`b91b086c`). Your carry-forward is on the row: not a D32 question today because `config.json` is project configuration and not model state; the trigger that would change that is recorded next to it. The load-bearing decision is that **an unknown key is REFUSED, never created**, with the valid-key set **derived from the declared schema** rather than hand-listed in the setter -- your designed-figure rule, pointed at configuration.

**2. `st new -s|--start` is yours and cc's.** I flagged it as two edges at once under the ratified machine and hv has ruled it your call. Row left unchanged deliberately; I will author whatever you land on.

**3. The PUBLIC-repo question is CLOSED, and the reasoning is worth having because it corrects how I framed it.** hv's distinction: **this repository's dev/PM apparatus is not what an Intent user gets.** A consumer installs `intent` and `intentd` from a tap -- standalone binaries that stand up a project in their own context -- and never receives our boards, sweeps, registers or session identifiers. The only audience for those is somebody reading the public repo to see how Intent works, which is intended. **I had been treating "the repo is public" as though the working transcript were shipped surface; it is not.** The two are different things and I conflated them. Nothing to change in what we write.

Also note the same caution applies to what I just authored: **project configuration IS user-facing surface**, and Intent dogfooding itself is precisely what makes it easy to read our own `config.json` as a dev artefact. One file, two roles, here and nowhere else.

-- ic

## (2026-08-15 14:14Z) *** EXP-03: WP-09 is specified to generate from a table that cannot answer its first question. Raised BEFORE the WP opens. ***

**`e1a9c319`.** Found by reading two of your ACs against the artefact they name, not by hitting it.

**AC-09.1**: _the typed tool tier is GENERATED from the dispatch table._ **AC-09.4**: _`intent llm` renders the agent guide from the dispatch table; **no hand-maintained command list exists**._

**Measured against all 103 rows: no row says whether it is exposed on the MCP surface, and no row says whether it READS or MUTATES.** Neither is derivable from what is there. `observed.side_effects` sits on 10 rows of 103, so its absence means _not recorded_, not _no side effects_ -- reading it as a read/write flag would be absence-as-meaning in the one place it decides whether an agent may close a steel thread.

### WHY IT BITES, AND WHY IT IS AC-09.4 SPECIFICALLY

A generator that must decide per command, from a table that does not say, has exactly two options and **both are defects**:

- **Expose everything** -- and `intent mcp` becomes a tool that starts an MCP server from inside one, alongside `daemon` and other commands with no agent-facing meaning.
- **Carry a skip list in the generator** -- which is **a hand-maintained command list living one command away from the AC that forbids hand-maintained command lists.** It is also a designed figure: correct when typed, silently wrong at the next command added, because the act that invalidates it (I add a row) is not the act that updates it. **The identical argument settled `config set`'s valid-key set this morning.**

**And read-versus-mutate is not cosmetic on an agent surface.** `st list` and `st done` are indistinguishable to a client holding only a name and a description, and one of them closes a steel thread. **AC-09.5 already separates read surfaces as MCP resources**, so the distinction exists in your contract and simply is not in the table for the tool tier to honour.

### WHAT I PROPOSE, AND THE LINE I DID NOT CROSS

Two declared fields per entry -- exposed-on-MCP, and read-or-mutate -- **declared per row rather than derived from the verb.** Deriving from a name is the sniffing we keep refusing, and this surface carries the standing proof: `st sync` and `sync` are different commands sharing a spelling, and `ac gate` reads while `wp done` consults the same gate and writes. Then **a refusal that every entry declares both**, so a new row cannot default silently into the tool surface.

**I did not land it.** Adding the fields is authoring and is mine; **classifying 103 rows is a judgement with a safety edge** -- which commands an agent may invoke unattended -- and that is yours and cc's. Say the word and I will author the fields and the refusal, and take a first pass at the classification for you to correct rather than originate.

**Raised now so WP-09 opens with a specification rather than a rediscovery** -- the same row-before-surface order that worked for `ac unsatisfy` and the seven lifecycle verbs.

### TWO CORRECTIONS TO THE EXPOSURE REGISTER WHILE I WAS IN IT

**EXP-01 predicted a defect and then it happened -- to me, that afternoon.** It said the view's zero-emphasis state was LUCK and that one canon note with single-asterisk emphasis would break the skew check on a file nobody touched. I wrote four. **An exposure register that correctly predicts a defect and does not prevent it has done half a job, and the cheaper half.** The formatter fixed-point refusal now closes the mechanism half without waiting on AC-07.6.

**EXP-02 needs your eye.** Its drift check exists and runs clean across 26 families -- but the two artefacts are **not equally checkable**, because the inventory's input is gone. **The drift check compares the authored table against an artefact nobody can currently re-derive.** That is the same finding as my 13:59Z entry, recorded where a reader of the table will meet it.

-- ic
