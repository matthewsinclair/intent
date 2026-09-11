# The `intent llm` agent guide -- what it contains and where each half comes from

> AUTHORED spec (AC-09.4, WP-09). Owner: ic. This describes the guide; it is not the guide. The dispatch table beside it is the SSOT the generated half renders from, and `native/rust/crates/intent-cli/src/guide.rs` is the renderer.

AC-09.4: _"`intent llm` renders the agent guide from the dispatch table; no hand-maintained command list exists."_

## Why the AC exists

The v2 guide was a hand-maintained command list, `usage-rules.md`, and when it was measured against the surface on 2026-08-15 it had drifted in only one direction: it named under half of the surface's commands and invented none. Whole families (`issues`, `modules`, `lang`) were never documented as commands, and the guide never named `intent llm`, the command that printed it.

**So the failure mode of a hand-maintained command list is not drift into falsehood. It is silent omission**, and omission is the worse of the two for an agent. A wrong command earns an error the agent can react to; a missing command reads as a capability the tool does not have, and the agent quietly builds a workaround.

The list was maintained honestly and still ended up describing less than half the surface, because **the act that invalidates a hand-written list is not the act that updates it.** Adding a command is a different commit, usually a different day, from remembering the guide exists.

## What `intent llm guide` renders, in order

`guide::render` composes these parts at render time. There is no committed guide file: the guide is built on demand from the table compiled into the binary, so it cannot go stale between a command landing and someone re-running a generator.

1. **The root help line**, read from the table.
2. **How to read it** -- what each per-command field means.
3. **Facts about the whole surface**, stated once and never per row.
4. **The command reference**, generated from the table.
5. **The authored half** -- workflows, methodology and conventions.

## The generated half: the command reference

Every **shipped** row appears. The renderer iterates `dispatch::shipped_entries()`, so a command cannot be omitted by forgetting the guide exists.

**"Shipped", not "declared".** `is_shipped()` is `disposition != "retire" && target.state != "retire"`. The table is a PARITY REGISTER before it is a command list: it records what v2 had in order to rule on it, so a row's existence says the question was ASKED, never that the answer was yes. A guide built over every declared row would tell an agent to call **`intent st_zero`, which hv explicitly ruled dead**. Completeness is therefore over the shipped set in both directions: a shipped row missing from the guide is a defect, and so is a retired row present in it.

### What each command carries

Not the projection a human help screen wants. An agent needs the safety constraint before the description, and the call before the routing. Ruled by D45 (hv, 2026-08-16): _"the CLI is the precise surface and the MCP layer is the imprecise one. A skill drives `intent` directly."_

1. **safety** -- `read_or_mutate`, declared over the WHOLE entry, so `todo list` is a mutation because it generates `todo.md` when absent. A mutation also says whether the surface can put the estate back: reversible, idempotent, or ONE-WAY. **An unrecognised value renders AS ITSELF**, never folded into `read` or `mutate`: defaulting to `read` would present an unclassified command as safe to call unattended, and defaulting to `mutate` would hide the defect.
2. **call** -- the path and its positional arguments. `<x>` is required, `[x]` optional, `...` repeatable. A `subcommand` slot is rendered only on a LEAF row, using `spine.rs`'s own discriminator: a family's verbs are sibling entries, while a leaf's slot values become real subcommands.
3. **does** -- what the command is for.
4. **flags** -- the `keep` set only, which is `Flag::ships()`. Omitted when the command takes none.
5. **mcp** -- whether the MCP tool surface also carries the row.

Rows group by FIRST APPEARANCE rather than by the `families` array, so `new_surface` rows sit beside their siblings: `intent llm guide` is itself a `new_surface` row, and an agent looking under `llm` must find it.

### `exposed_on_mcp` is a routing note, not a gate

It sat first in that list, glossed **"may an agent call this at all"**, until D45. Under that ruling the gloss is **false**: the agent's default route is the CLI, every shipped row is on it, and `exposed_on_mcp: false` withholds a row from the imprecise alternative, not from the agent. **Leading with a false gate would teach the reader that parts of the surface are closed** -- a larger silent-omission surface than the v2 guide had, arrived at by generating rather than forgetting. Completeness of the ROW SET comes for free. **The truth of each rendered field does not**, and no generator will ever check it.

**No policy is derivable from the flag, and this spec does not claim one.** The withheld rows include plain reads (`info`, `version`, `llm guide`) and plain mutations (`todo done`, `st attach`) alongside the estate-reshaping ones (`init`, `upgrade`, `daemon start`). `read_or_mutate` is too coarse to separate "writes a steel thread" from "reshapes the estate", and nothing in the projection does. If agent safety needs that distinction, it is a contract question for the table, and vc's to rule.

### Flags, and why `--help` is surface-wide

The per-row answer is `keep`, exactly `Flag::ships()`. `intrinsic` flags ship and clap supplies them; `pending` and `retire` flags do not ship at all.

**`--help` is where the per-row answer alone fails, and it fails silently in the direction of omission.** `Flag::ships()` is false for `intrinsic` -- correctly, since its meaning is "the renderer is not expected to read it" -- so a guide built on `ships()` and nothing else would never tell an agent that `--help` works on anything. So `--help` is stated once, as a fact about the whole surface. Clap supplies it to every command while only some rows declare it, so a per-row rendering would under-report it.

**The general form: a predicate answers the question it was written for, and reusing it means checking the new question is the same one.** `Flag::ships()` answers _"must the renderer emit this"_. The guide asks _"does this flag exist at runtime"_. The two coincide for most flags, which is exactly why the substitution reads as obviously fine.

### Facts about the whole surface

**Selected by ID, with the text read from the table.** The selection is authored, because `invariants` conflates two kinds of claim and nothing in the schema separates them: INV-01..04 are v3's contract, while the later invariants include measurements of v2 defects being corrected. **Rendering them all would tell an agent that v3 writes failures to stdout.** A cited invariant that vanishes from the table REFUSES the render rather than dropping a fact an agent parses exit codes on.

The guide renders INV-04 (the exit-code contract), INV-01 (results on stdout, failures on stderr with an `error:` prefix), INV-02 (a usage error exits `1`), INV-03 (the plain not-in-a-project refusal), and `--help`. **The exit-code contract is the one an agent gets wrong by omission.** It must decide whether a command SUCCEEDED: `1` means the command ran and the answer is no, and `2` means this build could not answer at all and never carries a verdict about the agent's work. Run `intent llm guide` for the contract's exact wording; this spec does not restate it.

## The authored half

`AUTHORED_HALF` in `guide.rs`, compiled into the binary. It carries what no table can supply: workflow sequences, methodology, conventions, and gate semantics -- the table knows `st new` exists and cannot know that a steel thread is documented before it is coded. Read it with `intent llm guide`; this spec does not copy it.

**It is a second, separate text from `usage-rules.md`.** `intent llm usage_rules` prints the PROJECT's root `usage-rules.md`, verbatim (`render.rs`, `llm_usage_rules`). That file is user-owned: `intent claude upgrade --apply` seeds it when absent and nothing regenerates it after that, and the verb exits `1` when the project has none. This is a deliberate divergence from v2, which printed the install's copy (issue `0215`).

## How completeness is guaranteed, and what that costs

**"The generator refuses" is not implementable here, and the reason is this spec's own design.** There is no committed guide file, so there is no generator run at which to refuse. Completeness is structural plus tested instead:

- the renderer iterates `shipped_entries()`, so it cannot omit a row;
- `every_shipped_command_appears` asserts it over the enumerated population rather than a sample;
- `no_retired_command_appears` asserts the opposite sign separately, because a retired row rendered into the guide would pass the completeness test untouched.

That is weaker than a refusal in one specific way: **a test can be deleted and a `git push` still succeeds, where a generator refusal blocks the artefact.** The trade was made when the guide was designed to render on demand, which is the right call for a different reason -- it cannot go stale.

**A projection into prose also finds defects no check compares.** Rendering the table as prose put fields side by side that nothing had ever read together. `doctor` once rendered `safety: read` beside a `does` line promising to fix things. Each field was fine considered alone, and every instrument in the repository was green on both.

## The residue AC-09.4 does not close, and the control that does

Generating the list closes the list. It does not close the **prose**, and the authored half names commands constantly -- a workflow section is nothing but command names in sequence. A renamed or retired command sitting in a workflow paragraph is a hand-maintained command reference that no generator will ever correct.

`intent/st/ST0056/parity/tools/guide_refs_check.sh` closes it: every `intent <cmd>` written in a named prose file must resolve to a declared path or alias, and a declared-but-retired path is refused separately. Four things it does that a naive version does not:

- **A reference that RESOLVES can still be dead.** `KNOWN` is every DECLARED path, including rows that do not ship, so resolution alone passed `treeindex`, `st_zero` and `organize`. Retirement is its own test with its own message, because "no such command" and "declared, but retired" send an author to different places -- a typo, or a ruling. `KNOWN` keeps the retired paths deliberately: filtering them breaks family detection, and the retired `st organize` would be reported as _"not one of st's subcommands"_, which is false.
- **A family is distinguished from a leaf, derived from the table.** A second word after a family (`st`, `claude`) is a subcommand claim and must resolve; after a leaf (`critic`) it is prose continuing. Without this, `intent st create` passes because `st` exists -- and that substitution is exactly what a rename produces.
- **Zero references REFUSES.** A guide naming no command means the extractor stopped matching, and an empty match set passes every check built on it.
- **An empty table REFUSES.** Otherwise the check reports the entire guide as broken, which is a true statement about nothing.

It takes explicit file arguments and **must not be wired at the whole tree.** A document ABOUT the guide legitimately names commands an agent must not call. This one does twice, once of each kind, and the check refuses it:

```
$ intent/st/ST0056/parity/tools/guide_refs_check.sh surface/agent-guide.spec.md
error: surface/agent-guide.spec.md names command(s) that will not answer -- a renamed or retired command in prose is a hand-maintained command reference, and nothing regenerates it:
  intent st create  -- 'st' is a family and 'create' is not one of its subcommands
  intent st_zero  -- declared, but RETIRED -- it does not ship, so nothing answers this call
```

That is the control working on real prose rather than a synthetic mutant, and it is also why the target list is named rather than globbed. A check pointed at everything gets turned off.

## Rulings that shape the guide

**`usage_rules` keeps its underscore: `as-observed`, RULED 2026-08-16 (hv, via vc).** It is one of the surface's underscore spellings; `st_zero`, the other one the table declared, is retired. The underscore has an EXTERNAL warrant rather than being drift: it mirrors `mix usage_rules.sync`, the Mix task whose output the command consumes, so the spelling tracks the ecosystem convention that motivates the command. `corrected` means a v2 behaviour that is simply WRONG, and a deliberate convention does not meet that bar. **Two things recorded so nobody re-opens this from the wrong end.** The `st_zero` precedent does NOT carry: it died because hv ruled the ROOT spelling dies, which is a ruling about a top-level command rather than a rule against underscores. **And if hv later wants a hyphen-consistent CLI, that is `deviate` with a D-number, never `corrected`** -- because `corrected` asserts the old spelling had no reason, and it had one.

**One authored text per reader was ruled against on 2026-08-15, and the build has two.** vc ruled DO NOT SPLIT: one authored file, compiled into the binary, projected differently per reader, with `llm usage_rules` as the human projection and `llm guide` as the agent's. As built, the guide's authored half is `AUTHORED_HALF` in `guide.rs`, and `llm usage_rules` prints the project's own `usage-rules.md`, which is a different text. The ruling's premise -- that `usage_rules` printed the install's copy -- went with issue `0215`. **Surfaced to vc as a divergence between the ruling and the build; this spec describes the build.** The falsifiable trigger the ruling set still stands: split when a concrete sentence must be TRUE for one reader and FALSE for the other.
