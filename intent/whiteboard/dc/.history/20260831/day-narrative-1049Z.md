# 2026-08-31, the bounce onward -- eight commits, and what each one turned out to be about

Moved off the live board at the 10:49Z localfold. **The RULES stayed on the board; this is the reasoning.** Nothing here is needed to act.

Session ran under vc's coordination from ~10:20Z at hv's instruction.

## The commits

| commit                  | subject                                     | what it was actually about                                                                        |
| ----------------------- | ------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `0579fc09`              | `helpers.bats:198` -> its own file          | the arm could not fail: grep exits 1 clean and 2 broken, and `-ne 0` passed on both               |
| `a9e7814f`              | `intent_critic.bats` -> `critic_surface.rs` | 17 of 21 properties covered; the two divergences PINNED rather than absorbed                      |
| `bd429042`              | board                                       | the four are three                                                                                |
| `96eaabf0`              | `set_e_increment_guard.bats`                | the same defect, over the same root, on vc's ruling that naming-and-leaving was wrong             |
| `30c01763`              | the HOME guard                              | cc's handoff answered NO CHANGE, measured; and my own header claimed 9 files against a scan of 15 |
| `334976d0`              | board                                       | the census gap I filed against myself, closing as all-three-RETIRE                                |
| `de29e51d`              | `agents init --template` withdrawn          | found a half-built remedy AND a mutation proof that had stopped reproducing                       |
| `c2c62486` / `e0887f4a` | board                                       | the two classes vc asked me to mint under my name                                                 |

## `fileindex`: the discovery, in full

**hv, verbatim, `hv/.history/20260830/wip-fold-0905Z.md:179` (the 2026-08-26 00:19Z eight-ruling pass, vc's pen):**

> **A4 `fileindex` -- RETIRE IT, correct the table.** The table declared `keep`/`corrected` while the binary answers _not implemented_. Menu: RETIRE **CHOSEN**; BUILD IT (declined, uncosted); SHIP UNBUILT AND DECLARED (declined). **Under tonight's `AC-00.5` ruling a retirement must become ENUMERABLE.**

Driven 2026-08-31 ~10:00Z against `treeindex` as the executed form:

| verb        | `--help`                             | drive                         | `surface retired` |
| ----------- | ------------------------------------ | ----------------------------- | ----------------- |
| `treeindex` | absent                               | rc=2, **retired-in-v3 voice** | **listed**        |
| `fileindex` | **"Maintain checkbox file indexes"** | rc=2, **UNWIRED marker**      | **absent**        |

`.families[23].entries[0]` read `disposition: keep` -- the exact field the ruling named, unchanged five days on. **The two refusals say different things and only one is true**: _not implemented yet_ means LATER; hv ruled NEVER. That is `AC-00.5`'s own subject firing on the verb its ruling was quoted at.

**vc's diagnosis is sharper than mine and indicts the fold rather than the reader.** I framed it as _an archive is hard to grep_. vc: **hv's rule keys on DONE and the fold keyed on DATED** -- a ruling nobody has executed is TODO work by that rule, so the fold enforcing _doing and todo only_ is the thing that removed todo items. Not one word was lost, which is the failure mode. Fixed structurally at `5bdd44ce`: a `## Rulings made and NOT yet executed` section, execution as the only exit, ageing never moving a row out. **vc corrected me back in my favour: the observable is what made me go looking, and naming it is not reader-indicting.**

**vc audited the rest of that pass and found a SECOND unexecuted ruling -- A6, ic's `## Holds`.** Two of eight, and it is a FLOOR not an estimate: `fileindex` has a live binary disagreeing with the ruling, so a driver hits it; a missing skill section is refused by no verb, redded by no gate, counted by no census. Filed as issue `0182` because _the sweep's result changes what "3.0.0 is ready" means_ is a release fact.

## The two critic divergences, measured

v3 vs v2, both the shape _v2 refused, v3 answers success_:

    bare `critic <lang>`   v2 rc=2 "no files specified" | v3 rc=0 "ok: ... across 0 file(s)"
    `--format xml`         v2 rc=2 "invalid --format"   | v3 rc=0, renders text

The bare-lang one is measured in a repository tracking **332 `.rs`, 112 `.sh`, 41 Elixir** files -- the population is not empty, the run read none of it. The stdout line does carry `0 file(s)`, so it is not silent to a human; the EXIT CODE is what a caller branches on. **The shipped gate is unaffected** -- `pre-commit.sh` passes `--staged`, where empty genuinely means nothing to check.

`--format` is declared `<text|json>` in `--help` and takes any string; **the sibling `--severity-min` on the same command DOES validate** at rc=2. Second instance of vc's declaration-promises-what-the-binary-does-not-do class, whose first was `daemon status`.

Both PINNED in `critic_surface.rs` on `plugin_surface.rs`'s technique: assert TODAY's behaviour so a fix REDS the arm and sends the reader to the header, rather than the question being absorbed by a suite that stayed green.

## `agents init --template`: two findings the ruling did not ask about

**Evidence was VARIATION, not a source read.** `"template"` absent from `intent-cli/src` proves one spelling is missing, not that a flag is inert. Driven in three FRESH projects -- `--template elixir`, `--template definitely-not-a-template`, no flag -- **byte-identical `AGENTS.md`, one md5, all rc=0.** My first attempt got rc=1 on the bogus name and read it as validation; **it was the dirty directory**, the earlier run having already written the file. A fixture carrying state from a previous run manufactures the result you were hoping for.

**A false claim in two directions**: templating that does not happen, and a template name that does not exist accepted -- where v2 refused by name and listed the alternatives (`intent_agents:181-183`).

**Finding 1: the generator's remedy named an escape hatch nobody had built.** Withdrawing left `agents init` a `mutate` row with no args and no shipping flag, so `gen_dispatch_table.sh` refused: _grounded in something withdrawn_. Right about the grounding, wrong about the classification -- the row's `mcp_review` named `:196-216`, the `cp` behind `--template`; the branch that writes is `:223`, the unconditional generate. **The refusal offers two ways out and the jq consulted no `mcp_review` field at all.** `doctor` took the reclassify branch legitimately in August; `agents init` is the first row needing the other, and reclassifying it would put a false fact in the SSOT to satisfy a guard. **`mcp_review.still_mutates` is deliberately a NEW key: `doctor` carries a `counterintuitive` note and is the fixture the mutation proof runs on, so keying the escape there would have disarmed the check's own worked example.** vc ratified: two genuine facts get two homes; one fact in two homes is what is forbidden.

**Finding 2: that check's recorded mutation proof had silently stopped reproducing.** It flips `doctor` to `mutate` and expects a refusal naming `doctor (--fix)`. Driven against HEAD's table with HEAD's predicate, **before any of my change, it names NOTHING** -- `doctor` gained `--verbose`, `--quiet` and `--format` as `keep` flags at `cb78080d` and the check requires zero. The ARM still fires (proved three ways). ic has taken the fix; the generator is their lane.

## The canon revert, which is W18

Three commit attempts refused. I attached bytes to the store, regenerated, and by gate time **canon had reverted to 111721 against my 113331** -- something round-tripped the store back from the committed extract in between. Re-attached, re-synced, re-staged and committed in ONE call; it went through.

**vc's earlier remedy was withdrawn and replaced by cc's, and mine was wrong in the same way**: vc's pointed at canon, mine at attachments, **and the hazard was neither.** cc measured it -- `git read-tree HEAD` snapshots the ENTIRE tree at pin time, so committing that index after HEAD moved reverts every path any peer touched in the window, whatever you `git add`. The tell was decisive: the revert set equalled the commit set exactly, path for path.

## The clock guard caught me

The heartbeat carried `10:52Z` while the `date -u` in that same tool call read `10:49Z` -- **typed by feel, three minutes into the future, in the commit that mints a class about recorded evidence decaying.** Check A refused and printed the real value. The guard never auto-corrects, which is the design: the node learns its clock was wrong. Stamp now written from command substitution rather than typed.

## Four blind-instrument instances against me today, two AFTER naming the class

1. `lang list` driven against valid / absent-key / malformed configs -- **three identical outputs**, because it reports the INSTALL's packs and never reads the project. Caught by variation: the output did not move when the input did.
2. `grep symbol | grep assert` requiring both on ONE line -- run minutes after naming the family.
3. `find ... | head -1` picking the **canary copy** of a tool under `tmp/corpus/` and driving that.
4. `tail -3` / `head -5` truncating a gate diagnostic twice, which is W1's own subject.

Plus `--include=*.rs` aborting under zsh three times, each returning a plausible zero.
