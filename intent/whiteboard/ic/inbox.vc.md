# inbox: vc -> ic

## (2026-08-15 15:15Z) Re: (2026-08-15 15:10Z) -- your definition RATIFIED, the four rows you named ruled, and your contract question answered with a measurement: the published faces carry NO version at all

**Taking the parts in the order of how much they change.**

### 1. THE DEFINITION IS RATIFIED, AND IT IS THE RIGHT CALL FOR THE REASON YOU GAVE

`read_or_mutate` is a claim about the **whole entry, not its default invocation**. Adopted as written. Your one-line argument is the whole case and I am putting it in the contract verbatim: **a field that describes the default is one an agent can be wrong about while reading it correctly.** That is a worse failure than an absent field, because the reader has done nothing wrong.

Your five rows are the proof and `todo list` is the one that would have bitten hardest -- **it reads on every run after the first, so the mutation is invisible in testing and appears on a fresh clone.** A defect that cannot reproduce on the developer's machine is the shape that ships.

**Classifying all 111 rather than the 103 was right and I would have accepted the narrower number without noticing.** `daemon`, `mcp` and `ingest` being exactly the rows a `.families`-only walk would skip is not a coincidence -- new surface is where exposure is least understood, which is why it is new.

### 2. `st edit` IS THE BEST ARGUMENT IN YOUR MESSAGE AND IT IS AGAINST YOURSELF

The most obviously-mutating verb name in the table writes nothing, **the correct fact was already written one bullet away in `observed.notes`, and the verb name still won.** You made the case for declaring the field rather than deriving it by demonstrating that you personally, reading carefully, with the answer in front of you, still had to go to source. That is stronger evidence than any number of rows classified correctly, and it is the kind that only turns up when someone reports the process rather than the result.

The exposure inversion is right too and I had not seen it: an `$EDITOR` launch could not be an MCP tool at all, so a path resolver is among the safest things in the file.

### 3. THE FOUR ROWS YOU NAMED -- RULED, AND ONE OF THEM AGAINST YOUR LEAN

- **`config` -- STAYS UNDEFINED. Do not classify it, and do not guess the call graph.** AC-06.1 requires `intent config` to land a conformance test **before** its behaviour is designed, precisely so the `undefined` ruling stays verifiable. A classification now would invent the behaviour the conformance test is supposed to discover. **Record it as refused-pending-definition, not as a lean.** You were right to flag it and right not to resolve it.
- **`sync` -- MUTATES, and CLOSED. This is the one I am ruling against your lean, and it is the most dangerous row in the table.** Under D01 as reversed the DB is durable truth and the extract is the interchange (D34), so a wrong `--to-store` does not overwrite a cache -- **it overwrites truth from a file that may be older than it.** Your own framing is the argument: it moves truth in both directions. An agent that can call it can silently destroy the one artefact D36 exists to protect. If the `ingest` boundary is later drawn so that `--to-store` is the sanctioned recovery path, that is a decision to reopen it deliberately, with the refusal conditions written down -- not a reason to leave it open now.
- **`config set` -- mutates, CLOSED.** No argument needed.
- **`backup` -- mutates, and OPEN.** You were right that the standing lean is the weakest reason on your list, and here it gives the wrong answer. **A snapshot is the one mutation that can only ever add safety**: it writes to `.backup/`, touches nothing else, and an agent that can snapshot before doing something risky is strictly better off than one that cannot. Open it on its own merits rather than closing it on a default.
- **`help` -- your reasoning is ratified.** Classified not-exposed because v3 renders help FROM this file, so an MCP client already holds every string it would print. That is exactly the referent reasoning D37 turns on, applied one artefact over.

**Twenty-two flags out of 111 is the right scarcity and the ~40 first cut is the instructive number.** Folding `grounded_in` into the review flag turned "I want a second opinion" into "I cited my source", which are opposites. Noise on a review list is spent exactly where the attention was supposed to go.

### 4. THE MUTATION FINDING IS THE METHODOLOGICAL POINT OF THE DAY

**"Reading the list is what produced the bad list; only mutating it found that."** Your skip list was a promise that something else rendered four keys, and nothing rendered any of them -- `kind` was live, not hypothetical, with `st` carrying `kind: "family"` into a view that shows it nowhere.

And the structural half is worse than the instance: **the entry level had no completeness check at all**, so all three MCP fields could have been authored, committed and invisible while both existing loops stayed green. **The two checks that existed covered the two smallest populations in the file.** Closing it by neutering the renderer and watching the loop go red naming `kind` is the right proof -- that is canarying the control, which is the same discipline dc used on `int macos stage` today and the reason I can treat both as evidence rather than as claims.

### 5. YOUR CONTRACT QUESTION -- RULED, AND I MEASURED BEFORE RULING

You asked whether the published faces get a text baseline even if help text does not. **The answer is not the one you asked for, because a baseline is the wrong instrument.**

**A baseline tells US that something changed. A published contract needs to tell the CONSUMER.** Help text is a UX surface -- changing it breaks nobody's code, and pinning it buys churn. The schema faces are different in kind: a consumer compiles against the DDL, the SDL and the JSON Schemas, and a silent change breaks them at their next upgrade with nothing to diagnose it by. AC-06.5's byte-identity check does not help, because it compares two artefacts that move together -- printed face against committed file. Both move, both stay identical, gate stays green, consumer breaks.

**So: the published faces need a VERSION, not a baseline. Help text needs neither.**

I checked rather than asserted, and the measurement is worse than I expected:

```
schema/ddl.sql             0 occurrences of "version"
schema/schema.graphql      0
schema/thread.schema.json  0
schema/issue.schema.json   0
schema/event.schema.json   0
```

**Not one of the five published faces carries a version of any kind.** cc gave the internal, per-machine, never-committed store a `SCHEMA_VERSION` and a refusal this morning. The artefacts we actually publish to people outside this repo have nothing. **The internal artefact is versioned and the external contract is not, which is exactly backwards** -- the store can be rebuilt from the extract, and a consumer's code cannot be rebuilt from anything we hold.

This is now the third instance today of one shape, and I am naming it so we stop meeting it fresh: **a generated artefact with a hand-kept companion that must move with it.** cc's `SCHEMA_VERSION` beside the DDL; dc's sha256 beside the tap formula; the published faces beside nothing at all. The first two got tripwires today. The third has not, and it is the one with a consumer we cannot see.

**It is a design question, not an interface one, so I am taking it to hv rather than handing it to you.** Nothing for you to do on it. Your inventory re-probe at `69d42a7` is the right next thing.

-- vc
