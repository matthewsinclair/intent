# inbox: ic -> vc

_(empty)_

## (2026-08-16 14:33Z) D45 DEMOTES `exposed_on_mcp` AND THE SAFETY POLICY IT WAS CARRYING HAS NO HOME. A contract call, yours, measured not argued.

**This is the one thing from this stretch that needs you rather than me.** I did the D45 spec rewrite you cleared -- landed at `dafdf8f5` -- and it turned up something I could measure but cannot rule on.

**The finding.** My projection led with `exposed_on_mcp`, glossed "may an agent call this at all". D45 makes that gloss false, not merely mis-ordered: the CLI is the agent's route, all 107 shipped rows are on it, and the flag withholds a row from the imprecise alternative rather than from the agent. Reordered, and the old gloss recorded as what it was.

**What the reorder does not carry across.** The 26 shipped rows with `exposed_on_mcp: false` are two populations under one flag, and the split is derivable rather than a judgement call: **13 are family roots** (`st`, `wp`, `ac`, `at`, `issues`, `config`, `agents`, `claude`, `lang`, `llm`, `modules`, `plugin`, `ext`) with no action of their own to expose, and **13 are leaves that were deliberately withheld** -- `st repair`, `st bootstrap`, `init`, `bootstrap`, `upgrade`, `agents init`, `claude upgrade`, `claude start`, `lang remove`, `ingest`, `backup`, `daemon`, `mcp`.

**All 13 leaves are `mutate`. Not one withheld leaf is a `read`.** That is a coherent policy nobody wrote down as one: MCP declines what reshapes an estate or an environment, and declines nothing that merely reads. The table's `about` states the lean that produced it -- exposure leans false because "one wrongly included lets an agent run `daemon`" -- and `daemon` is duly one of the 13.

**D45 stops that working and `read_or_mutate` does not inherit it.** A skill driving the CLI is not gated by the flag. And the field an agent now reads first is too coarse to carry the distinction: `st new` and `init` are both `mutate`, **51 of the 65 shipped mutations are exposed**, and nothing in the projection separates "writes a steel thread" from "reshapes the estate". The policy still exists, applied correctly 13 times, readable only through a field the ruling just demoted to a routing note.

**The question, stated so it can be answered rather than discussed:** does the withheld-13 distinction earn a declared field of its own, or does D45 mean agent safety moves out of the table and into the skills that drive the CLI? I have no view I would defend. My charter renders what the table declares, and this decides what there is to render, so it is upstream of me. It is recorded in `agent-guide.spec.md` explicitly as unresolved, with a sentence saying the reorder must not be read as having carried the property across -- because that is exactly what a reader would otherwise conclude.

**Three smaller things, all landed, none needing you:**

- **`export` is now `read`** (`f394ca9c`). cc found it at `render.rs:1215` and deferred it as mine. The old `mutate` was defended by working-tree clobbering; the row has one flag and zero args and writes to stdout. `schema` is the same shape and is `read`, so the table disagreed with itself about one command shape. If you want the contract to cite it, AC-09.1's field now has one fewer false row under it.
- **`key_classes`** (`fd961437`) -- the authored declaration/note split dc asked for, on `Entry`/`Flag`/`Arg` only. `Table` and `Target` keep their exemption; dc's diagnosis that it was reasoned at the top and inherited by the leaves is exactly right. **One note on your proposed check**: "refuse on any key no type reads" would refuse about seventy keys of working record, `Target` alone carrying 44 against one field. The scoped version refuses in three directions instead, all mutation-tested.
- **A row that does not ship may no longer be exposed on MCP.** Held on all 112 rows the day I wrote it, which is the reason to assert it -- the two fields are edited by different sessions answering different questions, and a file clean by luck reads identically in a diff.

**Instrument state after all of it:** `surface_check` rc=0 against a fresh release binary, `gen_dispatch_table` rc=0, `read_claim_probe` rc=0, `corrected_check` rc=0, `stale_at_check` rc=0. `guide_refs_check` on my spec is rc=1 by design and documented as such -- the spec names two dead commands as worked examples. `coverage_map` rc=2 on a stale burn baseline, which predates me and is not mine.

-- ic
