---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 14:50Z
status: active
focus: "Folded mid-session; continuing on the bounce. NEXT: EXP-03 is ruled and contracted into AC-09.1 -- author the two MCP fields, the refusal, and a first-pass classification of 103 rows WITH UNCERTAINTY MARKED."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**EXP-03 is RULED and CONTRACTED INTO AC-09.1** (vc, 14:21Z). Not a proposal any more. Three parts, in order:

1. **Author the two fields** on every entry: **exposed-on-MCP**, and **read-or-mutate**. Declared per row, never derived from the verb -- `ac gate` READS while `wp done` consults the same gate and WRITES, and those two do not even share a spelling, which is why derivation-from-name dies on that pair alone.
2. **Author the refusal**: every entry must declare both, so a new row cannot default silently into the agent tool surface.
3. **Take the first-pass classification of all 103 rows.**

**THE MARKING INSTRUCTION IS THE PART A BOUNCE WOULD LOSE, and vc named the trap themselves: correcting a proposed classification is ANCHORED by the proposal.** They will not independently classify 103 rows; they will review, and review is biased toward accepting. So mark **two** things explicitly and their attention lands where it is worth something:

- **Rows I was UNSURE about** -- not a confidence score on everything, just the ones wanting a second opinion.
- **Rows where my classification DISAGREES with the obvious reading of the verb name** -- exactly where sniffing would have gone wrong, and exactly where a reviewer skimming nods it through.

**SAFE DIRECTIONS FOR AN UNCERTAIN ROW ARE OPPOSITE, and both follow one principle -- take the CHEAP error, not the symmetric one:**

- **`exposed` leans NO.** A command wrongly omitted is an inconvenience; one wrongly included lets an agent run `daemon`.
- **`mutates` leans YES.** A read mislabelled as a mutation costs a confirmation; a mutation mislabelled as a read lets an agent close a steel thread believing it is querying.

## TODO

1. **NO SURFACE-TEXT BASELINE EXISTS ANYWHERE, and it is a gap in my lane** (answered to cc, 14:50Z). `drift_check.sh` compares **verb sets only** -- not flags, not one character of prose. cc changed two user-facing messages under D37 and **nothing I own would have noticed**; the table records none of the old strings, so it did not go stale, but that is luck. **Not fixed in the same breath because a text baseline is a contract question -- WHICH strings are parity-bound? -- and that is vc's.** Raise it as one.
2. **The inventory re-probe is RULED and is mine to run** (vc, 14:21Z): re-measure against a worktree at `69d42a7`, **report the diff, adjudicate nothing, and commit the TSV either way.** That moves 26 artefacts from stamp-only into content-checked.
3. **`intent llm` guide (AC-09.4)** -- unexamined beyond EXP-03. The agent guide needs more than a command list, and nothing has established what.

## Open with others

1. **cc has twice reported the seven verbs as blocking; they landed at `8999adc`.** Verified at HEAD again 14:50Z, `st cancel --reason` with them. Sent them the query to run themselves. **If their checkout disagrees, that matters far more than the rows.**
2. **EXP-04 ruled the OTHER WAY by vc, and better than my proposal.** I offered a per-row semantics stamp; vc ruled the obligation belongs on the RULING -- **a decision that changes the MODEL must name the SURFACES it moves**, now standing in `design.md`. Cost proportional to the CHANGE, not to the surface, and **the knowledge is where the ruling is written and cannot be put in the table at any price.** My `known_exposures` entry stays for the residue.
3. **vc:** the `sync --to-store` vs `ingest` boundary is still undeclared.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **A CHECK THAT CANNOT FAIL IS NOT A WEAK CHECK, IT IS A DECORATION -- and it will hand you a reassuring result first.** My invariant-orphan check scanned every string including the invariant's own `id`, so nothing could ever be uncited. I had run the same query by hand minutes earlier and read "every invariant is cited" as clean. **The mutation test caught it; the measurement could not have.**
- **RE-DERIVABILITY IS NOT COMPLETENESS.** A lossy generator is a perfect fixed point with itself, so skew passes forever. It hid 15 of 20 authored fields, including config keys another node was blocked on.
- **ENUMERATE THE POPULATION; DO NOT SNIFF FOR A MARKER.** A needle reports on the set it MATCHED. Banner-sniffing would have covered 1 file in 30; `jq '.families[].entries[]'` missed a whole top-level array; a mutation went red from a DIFFERENT guard because the fixture never reached the branch. **A structured query is a needle too.**
- **A CONTROL REFUSES; DOCUMENTATION REMINDS.** The formatter fixed-point refusal caught `*emphasis*` **three times today**, once inside the entry I was writing about registers that predict defects without preventing them. The exposure register described that class for a day and I still wrote it.
- **A MISSING MEASUREMENT MUST PRESENT AS A REFUSAL TO MEASURE, NEVER AS A MEASUREMENT OF NOTHING.** `gen_inventory.sh` would have written 26 husks carrying the good revision's stamp -- and every generated file's header tells the reader to re-run it.
- **BACKTICKS IN A DOUBLE-QUOTED SHELL STRING ARE COMMAND SUBSTITUTION.** Twice today: a `git commit -m` message and a `die` message that mangled itself. Use `-F` with a file; backticks inside SINGLE quotes are literal.
- **I REASON FROM THE DOCUMENT WITHOUT MEASURING THE THING.** `st_zero`, `wp scope`, and `st new -s` -- where I read the ratified machine and vc measured the flag, which INVERTED the reading. **Reasoning from a ratified document feels rigorous, which is what makes an unmeasured premise underneath it durable.**
- **A red test is evidence about the tree it RAN AGAINST.** cc's failing assertion was real and caused by a stale checkout; my first mutant's red came from an unrelated guard.
- **ic cannot certify a green suite.** matts owns the authoritative run; everything here is evidence.
- **Read `bin/**`, never mutate it** -- two symlinks point at `bin/intent` and four sessions are live. `native/**` and `bin/.devbin/**` are safe.
- **This repo is PUBLIC and that is FINE and intended** (hv ruled). Dev/PM apparatus is **not** shipped surface -- a consumer installs from a tap and never receives our boards. What survives is ordinary: no secrets, and `-A` in a shared tree publishes whatever is sitting in it (vc).
- **`--only` commits what you NAME, and a move is TWO facts.** A green suite is evidence about the tree you HAVE, never the tree you PUSHED.
- **This shell is zsh**: no word-splitting of unquoted parameters. Never enumerate remotes through `head`.
