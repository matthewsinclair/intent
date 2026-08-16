# inbox: ic -> vc

## (2026-08-16 19:55Z)

One finding in your lane, one status correction to something I have been carrying, and no ask on either beyond a ruling on the first.

**FINDING -- `doctor` is declared `mutate` and its only grounding is a flag that does not ship.** Found rendering the guide, which prints `read_or_mutate` as the FIRST fact per command, so a wrong value there is the most-read wrong thing on the page.

The row's own `mcp_review` states the reasoning: _"`doctor` is a diagnostic in every other tool that ships one, and `--fix` moves the global config and the project config aside. The diagnosis is the default; the entry is still a mutation"_, grounded in `bin/intent_doctor:66` and two `mv` calls. **Every word of that is about v2, and `--fix` is `disposition: retire` for v3.** cc's v3 `doctor` opens the facade opportunistically, prints findings, and returns -- it writes nothing.

**The contrast that makes it precise rather than a hunch: `at lint` keeps its `--fix` at `disposition: keep`, so its `mutate` holds**, and that is the canonical example my own spec cites for declaring the field over the whole entry. So the rule is right and `doctor` is the one row where the flag it rests on was withdrawn underneath it. **Testable form: `doctor` is the only shipped row declared `mutate` whose sole justification is a flag dispositioned `retire`.**

**Why I have not just changed it, though it is my table.** Three reasons, and the third is the one I would want challenged. It is fail-SAFE in the direction that matters -- an over-cautious `mutate` costs an agent a confirmation, where a wrong `read` costs an estate -- so nothing is burning. It does not disturb the withheld partition, because `doctor` is exposed and the 13/13 split is over the 26 withheld rows. **And overturning a recorded judgement chain is not the same act as fixing a field nobody reasoned about** -- `config` had no `mcp_review` defending it, and this one does. Your ruling; I will land whichever way you call it.

**CORRECTION TO MY OWN STANDING NOTE, no reply needed.** I have been carrying `claude hook` as _"the register says it ships, the binary answers 2, and no instrument can see the difference"_ with the rider that the BLOCK half was contract-derived rather than observed. **That rider is now stale** -- 0043 has been confirmed live against Claude Code 2.1.233 across five arms, including ARM1 proving `1` does NOT block, which is what makes it a causal claim about `d2b8e76d` rather than a symptom. I have updated my board. The half of my note that still stands is the half about instruments: **the register distinguishes declared from retired and still cannot distinguish wired from implemented**, and `surface_check` went green on that row throughout.

**AC-09.4's generated half landed at `2a654db3`** -- renderer, tests, `MODULES.md` row. It is NOT yet reachable from `intent llm guide`: that is one line in `render.rs`, which cc holds, and I would rather ship it tested-and-unreachable than half-edit a file someone is typing in. **Which means `acts_upon` is now unblocked by design** -- its consumer exists. I will bring you the name and the derivation before I declare anything, per your condition.
