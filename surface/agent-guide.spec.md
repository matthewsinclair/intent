# The `intent llm` agent guide -- what it contains and where each half comes from

> AUTHORED spec (AC-09.4, WP-09). Owner: ic. This describes the guide; it is not the guide. The dispatch table beside it is the SSOT the generated half renders from.

AC-09.4: _"`intent llm` renders the agent guide from the dispatch table; no hand-maintained command list exists."_

## Why the AC exists, measured rather than argued

The v2 guide is `usage-rules.md` at the project root, 389 lines, displayed by `intent llm usage_rules`. Measured against the surface on 2026-08-15:

| property                                    | measured                             |
| ------------------------------------------- | ------------------------------------ |
| commands the surface declares               | 111                                  |
| of those, named anywhere in the guide       | 54                                   |
| named in the guide, not in the surface      | **0**                                |
| whole families never documented as commands | **3** -- `issues`, `modules`, `lang` |

**So the failure mode of a hand-maintained command list is not drift into falsehood. It is silent omission**, and omission is the worse of the two for an agent. A wrong command earns an error the agent can react to; a missing command reads as a capability the tool does not have, and the agent quietly builds a workaround. `intent issues` is how this project tracked the fifteen issues that shipped in v2.19.0, and the string "issues" does not occur anywhere in the guide -- not as a command, not as a word. Neither does "modules". `lang` appears six times, every one of them as a `<lang>` placeholder inside some other command's arguments, and never as the six-subcommand family it is.

The guide also never names `intent llm` -- the command that prints it.

Nothing was stale, which is the part worth understanding: the list was maintained honestly and still ended up describing less than half the surface, because **the act that invalidates a hand-written list is not the act that updates it.** Adding a command is a different commit, usually a different day, from remembering the guide exists.

## The two halves

**Generated, from the dispatch table.** The command reference. Completeness is the property that matters and it is the one a generator gives for free: every **shipped** row appears, on the commit that declares it, or the generator refuses.

**"Shipped", not "declared", and the difference is five commands.** This sentence read _"every declared row appears"_ until it was measured against the table on 2026-08-16, and the table declares 112 rows of which **5 do not ship** -- `st organize`, `organize`, `treeindex`, `help`, `st_zero`. `is_shipped()` is `disposition != "retire" && target.state != "retire"`, and a guide built on the unqualified sentence would have told an agent to call **`intent st_zero`, which hv explicitly ruled dead** ("st_zero is wrong and the root spelling dies"). The table is a PARITY REGISTER before it is a command list: it records what v2 had in order to rule on it, so a row's existence is a statement that the question was ASKED, never that the answer was yes. Completeness is therefore over the shipped set in both directions -- a shipped row missing from the guide refuses the build, and a retired row present in it is the same defect wearing the opposite sign.

**Authored, in prose.** Workflow sequences, methodology, conventions, and the NEVER DO section. No table can supply these -- the table knows `st new` exists and cannot know that a steel thread is documented before it is coded. Roughly two thirds of the v2 guide is this, and it is the two thirds worth keeping.

The halves compose at render time. There is no committed generated guide file: `intent llm` renders on demand from the table compiled into the binary, so the guide cannot go stale between a command landing and someone re-running a generator.

## What the generated section carries per command

Not the same projection a human help screen wants. An agent needs the safety constraint before the description, and the call before the routing:

1. **`read_or_mutate`** -- does this change durable state. Declared over the WHOLE entry, so `at lint` is a mutation because `--fix` exists, and `todo list` is a mutation because it generates `todo.md` when absent.
2. **path, help, arguments, flags** -- the call. Flags are the `keep` set only, which is `Flag::ships()`; see below for what that deliberately excludes.
3. **`exposed_on_mcp`** -- whether the OTHER route also carries this row. Declared per row (AC-09.1), never derived. It is not a gate on the agent; see immediately below.
4. **surface-wide facts, stated ONCE and not per row** -- the exit-code contract, and `--help`.

### `exposed_on_mcp` was first in this list, and D45 moved it

It sat at position 1, glossed **"may an agent call this at all"**, until D45 (hv, 2026-08-16): _"the CLI is the precise surface and the MCP layer is the imprecise one. A skill drives `intent` directly."_ Under that ruling the gloss is not merely mis-emphasised, it is **false**. The agent's default route is the CLI; all 107 shipped rows are on it; `exposed_on_mcp: false` withholds a row from the imprecise alternative, not from the agent.

**Leading with a false gate is the specific error this document opens by measuring.** A guide whose first per-row fact is "may you call this" teaches its reader that parts of the surface are closed, and 26 of the 107 shipped rows carry the flag that would read as closed. That is a larger silent-omission surface than the one the v2 guide had, arrived at by generating rather than forgetting -- which is worth saying plainly, because the generated half's whole claim is that completeness comes for free. Completeness of the ROW SET comes for free. **The truth of each rendered field does not**, and no generator will ever check it.

### What the demotion leaves without a home

The 26 withheld rows are two populations under one flag, and the split is derivable rather than a matter of opinion: **13 are family roots** (`st`, `wp`, `ac`, `at`, `issues`, `config`, `agents`, `claude`, `lang`, `llm`, `modules`, `plugin`, `ext`) which have no action of their own to expose, and **13 are leaves that were deliberately withheld** -- `st repair`, `st bootstrap`, `init`, `bootstrap`, `upgrade`, `agents init`, `claude upgrade`, `claude start`, `lang remove`, `ingest`, `backup`, `daemon`, `mcp`.

**All 13 of those leaves are `mutate`, and not one withheld leaf is a `read`.** The withholding is a coherent policy that nobody wrote down as one: MCP declines the commands that reshape an estate or an environment, and declines nothing that merely reads. The table's `about` states the lean that produced it -- `exposed_on_mcp` leans false because "one wrongly included lets an agent run `daemon`" -- and `daemon` is duly one of the 13.

**D45 makes that policy stop working, and `read_or_mutate` does not inherit it.** If a skill drives `intent` directly then the flag prevents nothing, and the field an agent now reads first is too coarse to carry the distinction: `st new` and `init` are both `mutate`, 51 of the 65 shipped mutations are exposed, and nothing in the projection separates "writes a steel thread" from "reshapes the estate". The policy still exists, correctly applied to all 13 rows, readable only through a field the ruling just demoted to a routing note.

**This is a contract question and it is vc's, not mine.** My charter is the projection, and the projection can only render what the table declares; whether the withheld-13 distinction earns a field of its own -- or whether D45 means agent safety moves to the skills that drive the CLI -- decides what there is to render. Recorded here rather than resolved so that the next person to read this section does not conclude from the reordering that the safety property was carried across. **It was not. It was measured, named, and left with its owner.**

### Which flags, and the one that is missing if nobody asks

The table declares 94 flags in four dispositions, and **three different sets are defensible**, so the projection has to name one: `keep` (66) ships and the renderer must emit it; `intrinsic` (10) ships and clap supplies it; `pending` (6) and `retire` (12) do not ship at all.

Per row the answer is `keep`, which is exactly `Flag::ships()`. **`doctor` is the proof that this is right rather than merely convenient**: its four flags are `--fix` (retire), `--verbose` and `--quiet` (pending), and `--help` (intrinsic), so it renders with NO flags -- and that is accurate, because v3's `doctor` takes none. The withheld three are not silently dropped either; `doctor` names them itself (AC-06.8), so the one surface that knows they are pending is the one an agent is already looking at.

**`--help` is where the per-row answer alone fails, and it fails silently in the direction of omission.** `Flag::ships()` is false for `intrinsic` -- correctly, since its whole meaning is "the renderer is not expected to read it" -- so a guide built on `ships()` and nothing else **never tells an agent that `--help` works on anything**. For the help screen that is harmless, because clap prints its own. **A guide is a document, and nothing else in it will say so.** Routing `--help` to the surface-wide section is also strictly more accurate than rendering it per row: only 10 rows declare it, clap supplies it to all 112, so the per-row projection would under-report it even if `ships()` admitted it. Same shape as the exit-code contract, for the same reason -- a fact true of the whole surface is stated once and is wrong when distributed.

The general form, since it has now cost me four times: **a predicate answers the question it was written for, and reusing it means checking that the new question is the same one.** `Flag::ships()` answers _"must the renderer emit this"_. The guide asks _"does this flag exist at runtime"_. The two coincide for 66 of 76 flags, which is precisely why the substitution reads as obviously fine.

Point 4 is the one an agent-specific guide gets wrong by omission. An agent must decide whether a command SUCCEEDED, and the answer is not obvious from this surface: INV-04 says 0 is success and 1 is every failure, **except `intent critic`, which exits 2 when it has findings, and `intent claude hook`, which propagates**. An agent that reads 2 as failure will report a passing critic run as broken. INV-01 (failures write `error:` to stderr) and INV-03 (the exact not-in-a-project message) are the other two an agent parses rather than reads.

## The residue AC-09.4 does not close, and the control that does

Generating the list closes the list. It does not close the **prose**, and the authored half names commands constantly -- a workflow section is nothing but command names in sequence. A renamed or retired command sitting in a workflow paragraph is a hand-maintained command reference that no generator will ever correct, and it is invisible to a check that only asks whether the generated section is complete.

`parity/tools/guide_refs_check.sh` closes it: every `intent <cmd>` written in an authored prose file must resolve to a declared path or alias. It refuses otherwise.

Four things it does that a naive version does not, each earned:

- **A reference that RESOLVES can still be dead.** `KNOWN` is every DECLARED path, and five declared rows do not ship, so the check passed `treeindex`, `st_zero` and `organize` as _"3 distinct command reference(s), all declared"_ -- **while its own error message named the case, "a renamed or RETIRED command named in prose".** The file asserted the capability in the sentence where it lacked it, and the gap was found by measuring the claim rather than reading it. Retirement is a separate test with its own message, because "no such command" and "declared, but retired" send an author to different places -- the first to look for a typo, the second to a ruling. `KNOWN` deliberately keeps the retired paths: filtering them breaks family detection, and the retired `st organize` would be reported as _"not one of st's subcommands"_, which is false.
- **A family is distinguished from a leaf, derived from the table.** A second word after a family (`st`, `claude`) is a subcommand claim and must resolve; after a leaf (`critic`) it is prose continuing. Without this, `intent st create` passes because `st` exists -- and that substitution is exactly what a rename produces. It survived as a mutant against the first version.
- **Zero references REFUSES.** A guide naming no command means the extractor stopped matching, and an empty match set passes every check built on it.
- **An empty table REFUSES.** Otherwise the check reports the entire guide as broken, which is a true statement about nothing.

It takes explicit file arguments and **must not be wired at the whole tree.** A document ABOUT the guide legitimately names commands an agent must not call -- this one does twice, once of each kind, and the check refuses it:

```
error: surface/agent-guide.spec.md names command(s) that will not answer -- a renamed or retired command in prose is a hand-maintained command reference, and nothing regenerates it:
  intent st create  -- 'st' is a family and 'create' is not one of its subcommands
  intent st_zero  -- declared, but RETIRED -- it does not ship, so nothing answers this call
```

That is the control working, on real prose rather than a synthetic mutant, and it is also the reason the target list is named rather than globbed. A check pointed at everything gets turned off.

## Not decided here

**The authored half has not been rewritten for v3, deliberately.** Its subject matter is the v3 workflows, and `sync`, `export`, `ingest` and `backup` are still settling -- `sync --to-store` against `ingest` is an open boundary. Prose written now would be authored against a surface that moves, and would arrive at WP-09 already needing the treatment this document exists to prevent. The control is built and proven now because it is independent of what the prose says; the prose waits for the workflows.

**Where the authored half lives: RULED by vc 2026-08-15 -- DO NOT SPLIT. One authored file, compiled into the binary.**

I had argued the measurement above for splitting, and it does not carry. **The cause it identifies is temporal** -- the act that invalidates a hand-written list is not the act that updates it -- **and a temporal cause is indifferent to how many readers a document has and to how many files it lives in.** Splitting does not make the update-act coincide with the add-act; it gives the same failure a second place to happen, plus a routing decision about which file a new convention belongs in, made at authoring time. That is precisely the decision the measurement shows nobody remembers to make.

vc also checked the assumption that would have flipped it, and it went the other way: `bin/intent_llm:55` resolves `$INTENT_HOME/usage-rules.md`, the INSTALL rather than the project, and the file is consumer-facing throughout. **There is no dual role to split apart** -- a consumer gets the installed tool's copy, correctly.

**The two-readers tension is real and it is a RENDER question, not a STORAGE question**, which is the answer this document already gives for the other axis: the halves compose at render time. One authored file; `intent llm` projects it differently per reader. `llm usage_rules` is the human projection and keeps its name -- `--symlink` and the Elixir `usage_rules.sync` habit both depend on it. `llm guide` is the agent projection, declared in the table on 2026-08-15 with ratification outstanding.

**Falsifiable trigger, so this is not a forever-ruling:** split when a concrete sentence must be TRUE for one reader and FALSE for the other. Not "differently emphasised", not "longer than a human wants" -- contradictory. Neither of us could construct one; the closest candidate, the exit-code contract, is additive rather than contradictory and is already routed to the generated half.

## Still open

**The command that renders this guide had no row until 2026-08-15**, though this family's own notes said it was regenerated from the table at WP-09. The family knew and the surface did not -- an AC whose subject has no row cannot be tested at all. `llm guide` now exists; **its ratification is outstanding.**

**`usage_rules` is `as-observed`. RULED 2026-08-16 (hv, via vc) and this section previously said it was undecided.** It is one of exactly two underscore spellings in the surface; the other, `st_zero`, is retired.

**The ruling turns on the underscore having an EXTERNAL warrant rather than being drift.** It mirrors `mix usage_rules.sync`, the Mix task whose output the command consumes -- **so the spelling tracks the ecosystem convention that motivates the command's existence**, and `corrected` means a v2 behaviour that is simply WRONG and gets fixed rather than reproduced. A deliberate convention does not meet that bar. My own caution was taken as the confirming half: `--symlink` and the Elixir habit both depend on the current spelling, so `corrected` would cost something, and **a classification that costs something should have to earn it.**

**Two things recorded so nobody re-opens this from the wrong end.** The `st_zero` precedent does NOT carry: it died because hv ruled the ROOT spelling dies, which is a ruling about a top-level command rather than a general rule against underscores, and `usage_rules` is a verb under `llm`. **And if hv later wants a hyphen-consistent CLI, that is `deviate` with a D-number, never `corrected`** -- because `corrected` asserts the old spelling had no reason, and it had one.
