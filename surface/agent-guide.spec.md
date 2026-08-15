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

**Generated, from the dispatch table.** The command reference. Completeness is the property that matters and it is the one a generator gives for free: every declared row appears, on the commit that declares it, or the generator refuses.

**Authored, in prose.** Workflow sequences, methodology, conventions, and the NEVER DO section. No table can supply these -- the table knows `st new` exists and cannot know that a steel thread is documented before it is coded. Roughly two thirds of the v2 guide is this, and it is the two thirds worth keeping.

The halves compose at render time. There is no committed generated guide file: `intent llm` renders on demand from the table compiled into the binary, so the guide cannot go stale between a command landing and someone re-running a generator.

## What the generated section carries per command

Not the same projection a human help screen wants. An agent needs the constraints before the description:

1. **`exposed_on_mcp`** -- may an agent call this at all. Declared per row (AC-09.1), never derived.
2. **`read_or_mutate`** -- does this change durable state. Declared over the WHOLE entry, so `at lint` is a mutation because `--fix` exists, and `todo list` is a mutation because it generates `todo.md` when absent.
3. **path, help, arguments, flags** -- the call.
4. **exit-code contract** -- from the surface-wide invariants, once, not per row.

Point 4 is the one an agent-specific guide gets wrong by omission. An agent must decide whether a command SUCCEEDED, and the answer is not obvious from this surface: INV-04 says 0 is success and 1 is every failure, **except `intent critic`, which exits 2 when it has findings, and `intent claude hook`, which propagates**. An agent that reads 2 as failure will report a passing critic run as broken. INV-01 (failures write `error:` to stderr) and INV-03 (the exact not-in-a-project message) are the other two an agent parses rather than reads.

## The residue AC-09.4 does not close, and the control that does

Generating the list closes the list. It does not close the **prose**, and the authored half names commands constantly -- a workflow section is nothing but command names in sequence. A renamed or retired command sitting in a workflow paragraph is a hand-maintained command reference that no generator will ever correct, and it is invisible to a check that only asks whether the generated section is complete.

`parity/tools/guide_refs_check.sh` closes it: every `intent <cmd>` written in an authored prose file must resolve to a declared path or alias. It refuses otherwise.

Three things it does that a naive version does not, each earned:

- **A family is distinguished from a leaf, derived from the table.** A second word after a family (`st`, `claude`) is a subcommand claim and must resolve; after a leaf (`critic`) it is prose continuing. Without this, `intent st create` passes because `st` exists -- and that substitution is exactly what a rename produces. It survived as a mutant against the first version.
- **Zero references REFUSES.** A guide naming no command means the extractor stopped matching, and an empty match set passes every check built on it.
- **An empty table REFUSES.** Otherwise the check reports the entire guide as broken, which is a true statement about nothing.

It takes explicit file arguments and **must not be wired at the whole tree.** A document ABOUT the guide legitimately quotes commands that do not exist -- this one does, twice, and the check refuses it:

```
error: surface/agent-guide.spec.md references command(s) the dispatch table does not declare
  intent st create  -- 'st' is a family and 'create' is not one of its subcommands
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

**And `usage_rules` is one of exactly TWO underscore spellings in the 111-command surface.** The other is `st_zero`, retired because hv ruled `"st_zero is wrong and the root spelling dies"`. The underscore here is not mirroring anything: the file it displays is `usage-rules.md`, hyphenated, so the command and its own subject disagree. It is a Mix-task convention (`mix usage_rules.sync`) imported into a CLI that uses neither underscores nor hyphens anywhere else. **Whether that is `as-observed` or `corrected` is a parity classification and not ic's to rule.** Raised, not decided.
