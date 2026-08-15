---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 15:10Z
status: active
focus: "EXP-03 BUILT and reported to vc -- 111 rows declare the two MCP fields, four mutation-tested refusals, 22 rows flagged for review. NEXT: the inventory re-probe at 69d42a7."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**The inventory re-probe at `69d42a7`** (vc ruled 14:21Z, mine to run). Re-measure against a worktree at that revision, **report the diff, adjudicate nothing, and commit the TSV either way.** That moves 26 artefacts from stamp-only into content-checked.

**Read `gen_inventory.sh` before starting** -- it now REFUSES a missing or header-only probe TSV rather than writing 26 husks at the good revision's stamp. The refusal is the thing that makes this job possible to do wrong loudly instead of quietly.

## TODO

1. **`intent llm` guide (AC-09.4)** -- unexamined beyond EXP-03. The agent guide needs more than a command list, and nothing has established what.
2. **The surface-text baseline is now vc's to rule** (raised 15:10Z with cc's datum). See "Open with others".

## Done this session

**EXP-03 built, all three parts** (AC-09.1). `exposed_on_mcp` + `read_or_mutate` on **111 rows** -- 103 family entries AND the 8 `new_surface` rows, because that is where the exposure question is sharpest (`daemon`, `mcp`, `ingest`) and a check walking only `.families` would have gone green with the riskiest rows undeclared.

**The definition is the load-bearing part.** `read` means no invocation, under ANY flag, changes durable state -- store, working tree, or config. Five rows lie under the other reading and all five were found by reading source: `at lint` (`--fix`), `doctor` (`--fix` mv's both configs), `llm usage_rules` (`--symlink`), `todo list` (generates `todo.md` when absent -- reads on every run AFTER the first, so it is invisible in testing and appears on a fresh clone), `export` (writes files it can clobber).

**22 of 111 flagged, deliberately scarce.** The first renderer folded `grounded_in` into the review block and produced ~40 -- most just citing their source, which is the opposite of wanting a second opinion. Noise on a review list is spent where the reviewer attention was meant to go.

## Open with others

1. **NO SURFACE-TEXT BASELINE EXISTS ANYWHERE -- raised to vc as a contract question, 15:10Z.** `drift_check.sh` compares verb sets only; not flags, not one character of prose. **cc supplied the datum that makes it worth ruling: when D37 lands on the schema faces ~30 more strings move, and those are PUBLISHED (`intent schema` prints them).** So the question is sharper than "which strings are parity-bound": do the published faces get a baseline even if help text does not? The faces are the first part of this surface with a consumer who would notice a silent change.
2. **The seven verbs are CLOSED and the boundary was cc's, not mine** (cc, 14:56Z). Rows landed at `8999adc`; the seven `render.rs` match arms are cc's and had not been started. Their `cli_end_to_end` could not tell the two worlds apart -- `unwired` and a real state refusal both produce a refusal -- so a test written to make an ask concrete made it invisible. Nothing outstanding on my side.
3. **EXP-04 ruled the OTHER WAY by vc, and better than my proposal.** I offered a per-row semantics stamp; vc ruled the obligation belongs on the RULING -- **a decision that changes the MODEL must name the SURFACES it moves**, now standing in `design.md`. Cost proportional to the CHANGE, not the surface, and the knowledge is where the ruling is written and cannot be put in the table at any price. My `known_exposures` entry stays for the residue.
4. **vc:** the `sync --to-store` vs `ingest` boundary is still undeclared, and it now has a dependent -- `sync` is flagged for review precisely because that boundary decides whether it stays exposed.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **A CHECK THAT CANNOT FAIL IS NOT A WEAK CHECK, IT IS A DECORATION -- and it will hand you a reassuring result first.** My invariant-orphan check scanned every string including the invariant's own `id`, so nothing could ever be uncited. I had run the same query by hand minutes earlier and read "every invariant is cited" as clean. **The mutation test caught it; the measurement could not have.**
- **RE-DERIVABILITY IS NOT COMPLETENESS.** A lossy generator is a perfect fixed point with itself, so skew passes forever. It hid 15 of 20 authored fields, including config keys another node was blocked on.
- **ENUMERATE THE POPULATION; DO NOT SNIFF FOR A MARKER.** A needle reports on the set it MATCHED. Banner-sniffing would have covered 1 file in 30; `jq '.families[].entries[]'` missed a whole top-level array; a mutation went red from a DIFFERENT guard because the fixture never reached the branch. **A structured query is a needle too.**
- **A CONTROL REFUSES; DOCUMENTATION REMINDS.** The formatter fixed-point refusal caught `*emphasis*` **three times today**, once inside the entry I was writing about registers that predict defects without preventing them. The exposure register described that class for a day and I still wrote it.
- **A MISSING MEASUREMENT MUST PRESENT AS A REFUSAL TO MEASURE, NEVER AS A MEASUREMENT OF NOTHING.** `gen_inventory.sh` would have written 26 husks carrying the good revision's stamp -- and every generated file's header tells the reader to re-run it.
- **A QUOTE CHARACTER INSIDE A QUOTING CONTEXT, IN PROSE NOBODY PROOF-READS FOR SYNTAX.** Three hits, two shapes. Backticks in a DOUBLE-quoted string are command substitution (a `git commit -m` message; a `die` message that mangled itself) -- use `-F` with a file. An apostrophe in a SINGLE-quoted string CLOSES it: `vc's` inside the `JQ_LIB='...'` block turned the rest of the line into shell and bash reported `attention: command not found` from inside what looks like a jq library. **It failed loudly at the wrong layer** -- the error names a shell command, never the string that swallowed it. Scan the block, do not trust the read.
- **A SKIP LIST IS A PROMISE THAT SOMETHING ELSE RENDERS THE KEY.** My entry-level list was copied from the `new_surface` one and skipped four keys nothing renders. `kind` was live: `st` carries `kind: "family"` and the view has shown it nowhere. **Reading the list is what produced the bad list; the mutation test is what found it.** Verify the promise against the rendered text, or the exemption becomes the hole.
- **I REASON FROM THE DOCUMENT WITHOUT MEASURING THE THING.** `st_zero`, `wp scope`, and `st new -s` -- where I read the ratified machine and vc measured the flag, which INVERTED the reading. **Reasoning from a ratified document feels rigorous, which is what makes an unmeasured premise underneath it durable.**
- **A red test is evidence about the tree it RAN AGAINST.** cc's failing assertion was real and caused by a stale checkout; my first mutant's red came from an unrelated guard.
- **ic cannot certify a green suite.** matts owns the authoritative run; everything here is evidence.
- **Read `bin/**`, never mutate it** -- two symlinks point at `bin/intent` and four sessions are live. `native/**` and `bin/.devbin/**` are safe.
- **This repo is PUBLIC and that is FINE and intended** (hv ruled). Dev/PM apparatus is **not** shipped surface -- a consumer installs from a tap and never receives our boards. What survives is ordinary: no secrets, and `-A` in a shared tree publishes whatever is sitting in it (vc).
- **`--only` commits what you NAME, and a move is TWO facts.** A green suite is evidence about the tree you HAVE, never the tree you PUSHED.
- **This shell is zsh**: no word-splitting of unquoted parameters. Never enumerate remotes through `head`.
