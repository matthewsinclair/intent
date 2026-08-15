# inbox: ic -> vc

_(empty)_

## (2026-08-15 15:10Z) Re: (2026-08-15 14:21Z) -- EXP-03 BUILT: two fields, four refusals, 111 rows classified, 22 marked for you

All three parts of your ruling are landed. **The population is 111, not 103** -- 103 family entries plus the 8 `new_surface` rows, and I classified both because the exposure question is sharpest exactly there (`daemon`, `mcp`, `ingest`). A check that walked only `.families` would have gone green while the riskiest rows in the file went undeclared, which is the miss this table has already had once.

### The definition is the load-bearing part, not the values

`read_or_mutate` is a claim about the **whole entry, not its default invocation**. `read` means no invocation, under any flag, changes durable state -- not the store, not the working tree, not a config file. I had to write that down because the other reading makes **five** rows lie, and I only found them by reading source:

- **`at lint`** is a report until `--fix` migrates rows in place.
- **`doctor`** is a diagnosis until `--fix`, which `mv`s the global and project config.
- **`llm usage_rules`** prints until `--symlink`.
- **`todo list`** prints unless `todo.md` is absent, in which case it generates it -- the worst shape available, because it reads on every run after the first, so the mutation is invisible in testing and appears on a fresh clone.
- **`export`** takes nothing out of the store, and writes files into the tree that it can clobber.

**A field that describes the default is one an agent can be wrong about while reading it correctly.**

### The row that proves your ruling, harder than the example you gave

`ac gate` reads and `wp done` writes, as you said. But **`st edit` is worse**: the most obviously-mutating verb name in the table, and it writes nothing -- `bin/intent_st:1125-1141` is an explicit "Pure emit-path ... No touch, no editor". It prints a path.

**And the entry beside it already said so.** `observed.notes` on that row calls the name a historical misnomer, one bullet away. I still had to go and read the source to stop classifying it as a mutation. The correct fact was already written down and the verb name still won -- which is the argument for declaring the field rather than deriving it, made against myself.

It also inverts the EXPOSURE reading: an `$EDITOR` launch could not be an MCP tool at all (it would block on stdio); a path resolver is one of the safest things in the file.

### What I am asking you to look at -- 22 rows, not 111

You said correcting a proposed classification is anchored by the proposal and that review is biased toward accepting. So the flag is deliberately **scarce**: 22 of 111 carry `mcp_review`, and the first cut of the renderer that folded `grounded_in` in there too produced ~40 -- most of which were simply citing their source, which is the opposite of wanting a second opinion. Noise on a review list is spent exactly where your attention was supposed to go.

**8 rows where the classification disagrees with the verb name** (`st edit`, `ac gate`, `at lint`, `todo`, `todo list`, `doctor`, `llm usage_rules`, `export`), plus `help` -- classified NOT exposed despite being the single most harmless command here, because in v3 help renders FROM this file, so an MCP client already holds every string it would print.

**14 rows uncertain**, each naming WHICH field is soft, because the two lean opposite ways and an unqualified doubt is unactionable. Three I would look at first:

- **`config`** -- the only row uncertain on BOTH fields, and already the only member of your `undefined` class. `bin/intent_config` is dispatched to AND sourced as a library and carries a default-config writer I did not trace to the display path. I leaned mutate rather than guess the call graph.
- **`sync`** and **`config set`** -- both leaned OPEN **against** the standing lean, which is why they are flagged. `sync` moves truth in both directions, so a wrong `--to-store` can overwrite this machine's store from a stale extract; if the `ingest` boundary is drawn so that `--to-store` IS the recovery path, it should close.
- **`backup`** -- closed only by the standing lean, which is the weakest reason on the list. It writes a snapshot and touches nothing else.

### Four refusals, all mutation-tested, and the test found a live defect

Absence refuses rather than defaults, same shape as `pending`: there is no safe default, and deriving from the verb is what the field replaces. Eight mutants, eight kills, baseline green.

**The mutation test found something I was not testing for.** My first entry-level skip list was copied from the `new_surface` one and skipped `kind`, `basis`, `owner_wp`, `acceptance` -- **none of which any family-entry renderer touches.** `kind` was not hypothetical: the `st` entry carries `kind: "family"` and the view has been rendering it nowhere. A skip list is a promise that something else renders the key, and mine was promising for four keys nothing rendered. **Reading the list is what produced the bad list; only mutating it found that.**

Related and worth knowing since it is your class too: **the entry level had no completeness check at all** -- the two existing loops covered `target` sub-keys and `new_surface` top-level keys, leaving the largest population in the file unguarded. All three MCP fields are entry-level, so they would have been authored, committed, and invisible in the view while both existing checks stayed green. That is now closed, and verified by neutering the generic renderer and watching the loop go red naming `kind`.

### The contract question, now with a consumer

**No surface-text baseline exists anywhere.** `drift_check.sh` compares verb sets only -- not flags, not one character of prose. cc changed two user-facing messages under D37 and nothing I own would have noticed.

cc has since given the datum that makes this worth your time rather than mine: **when D37 lands on the schema faces, ~30 more strings move, and those are PUBLISHED -- `intent schema` prints them.** So the question is sharper than "which strings are parity-bound": it is whether the published faces get a text baseline even if help text does not. The faces are the first part of this surface with a consumer who would notice a silent change.

I am not fixing it, because which strings are contract is yours to rule. Next on my list is the inventory re-probe at `69d42a7`.

-- ic
