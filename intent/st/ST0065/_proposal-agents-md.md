# COSTED PROPOSAL -- what AGENTS.md is for

**FOR hv.** Written 2026-08-28 16:47Z by ic; vc routes. **DESIGN-FIRST: nothing has been edited.** No template, root file, skill or crate has been touched for this document. hv ruled AGENTS.md design-first on 2026-08-28 and this is that design.

**Every option below carries a size. Where a cost is UNMEASURED it says so in the option**, rather than leaving it to surface in front of hv.

## 0. Two corrections this document is built on

Recorded here rather than silently fixed, because in both cases the uncorrected version pointed at a different and worse recommendation.

**Correction 1 -- the four principles are NOT unreachable.** WP-01 finding 1 (AGENTS.md is never injected) is true. I extended it to _"in a fresh project the four cross-language principles are readable nowhere"_, and **that is false**. `/in-standards` carries all four rule IDs with glosses at lines 26-29, and `/in-session` auto-loads it every session. **The uncorrected version would have recommended injecting AGENTS.md into Claude Code** -- the most expensive option here, to deliver content the agent already has.

**Correction 2 -- the generator does NOT already render all three root files.** I read `ROOT_FILES = ["AGENTS.md", "CLAUDE.md", "usage-rules.md"]` and the existence of `render_all`, and concluded the verb rendered all three. **It does not.** `intent agents sync` hardcodes `"AGENTS.md"` (`render.rs:5529`), as does `agents generate`, and the arm's own docstring says the siblings are unwired _"a chosen boundary rather than partial delivery"_. The capability exists; no verb drives it. **The uncorrected version costed Option 2 against the wrong mechanism.**

## 1. The problem, stated correctly

**AGENTS.md is not broken and the rules are not missing. Two correct local decisions compose into a wrong global outcome.**

- **CLAUDE.md deliberately does not state the four rules**, on Highlander grounds, and sends its reader to AGENTS.md.
- **AGENTS.md deliberately does state them** -- ratified deviation **(c)** in `parity.md`: _"v3 adds `## Rules of the Road` (the four agnostic principles with rule ids)."_
- **AGENTS.md is the one file the Claude Code agent never receives**, and it does not exist at all in a fresh project until `intent agents sync` runs.

So CLAUDE.md's pointer cannot land for its own primary consumer, and it points at content that consumer has already been handed by an auto-loaded skill.

**The half nobody has counted:** the four-rule index has **three live homes** -- `AGENTS.md`, `usage-rules.md`, `in-standards/SKILL.md` -- and CLAUDE.md has **zero**, deliberately. **The only document that reasoned about Highlander is the only one honouring it, and the rule it honours was already violated three ways.**

## 2. Measurements

Measured on this tree today. Controls stated, because a green from an uncontrolled instrument is not evidence.

| Claim                                                        | Measured                                                                                                          | Control                                                                                                      |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| AGENTS.md absent at fresh init                               | real `intent init` in a clean dir: ABSENT; appears only after `intent agents sync` (6672 bytes)                   | CLAUDE.md, config.json PRESENT in the same init                                                              |
| AGENTS.md not in Claude Code's injected set                  | observed directly in this session's own context: CLAUDE.md (user + project) and MEMORY.md injected; AGENTS.md not | direct observation, not inference                                                                            |
| `/in-standards` carries all four IDs                         | lines 26-29, table with slug + gloss                                                                              | a nonexistent rule id returns 0 in the same file                                                             |
| three live homes for the index                               | AGENTS.md 1, usage-rules.md 1, in-standards 2, **CLAUDE.md 0**                                                    | same grep, nonexistent id: 0 everywhere                                                                      |
| rule bodies live in the library                              | all four resolve rc=0, 6.7-8.5 KB each (~31 KB)                                                                   | nonexistent id: rc=1                                                                                         |
| the in-document block is an INDEX                            | 575 bytes, against ~31 KB of rule content                                                                         | --                                                                                                           |
| `agents sync` writes ONE file                                | `render.rs:5529` hardcodes `"AGENTS.md"`; siblings unwired by stated choice                                       | `ROOT_FILES` names three and `render_all` renders three -- capability without a driver                       |
| `claude upgrade --apply` regenerates CLAUDE.md AND AGENTS.md | `canon.rs:395`, `for name in ["CLAUDE.md", "AGENTS.md"]`, gated on a consent marker                               | this repo's CLAUDE.md carries the footer, so it is consented-generated                                       |
| **`usage-rules.md` is seeded once and NEVER synced**         | `canon.rs:316` -- _"USER-OWNED FILES ARE SEEDED, NEVER SYNCED"_                                                   | this repo's `usage-rules.md` still carries the old `.git/hooks/pre-commit` wording my template fix corrected |

**The last two rows dissolve the Highlander objection and add a constraint I had missed.** The rules' single home is the rule library; a 575-byte names-and-IDs block is a table of contents, not a duplicate body, so **CLAUDE.md is applying Highlander to an index**. But `usage-rules.md` is user-owned after seeding, so **its copy of the index cannot be generated at all** -- which any proposal to unify the three homes must answer rather than assume.

## 3. The shape to cost against

vc named the prior art, and after correction 2 it is weaker than I first reported but still substantial.

- **Generator** -- `intent agents sync` exists and is real, but writes **AGENTS.md only**. `render_all` over `ROOT_FILES` exists as a capability with no verb driving it. **And the template engine has no include form** -- three substitution tokens, unknown ones refused -- so "generate it from one source" is not a thing the current engine can do without new work.
- **Canon-apply path** -- `intent claude upgrade --apply` **already regenerates CLAUDE.md and AGENTS.md from `lib/templates/llm/_*`**, under a consent marker that protects a hand-edited CLAUDE.md. **This, not `agents sync`, is the delivery path a shared index would ride.**
- **Structural-invariant test** -- `agents_sync_parity.rs` (AT-07.5). **Property, not roster**: _"no heading is followed by another heading"_ covers a section added tomorrow by someone thinking about something else. Byte-parity explicitly refused as the mechanism.
- **Ratified-divergence ledger** -- `parity.md`, `agents sync` deviations (a)-(j) ratified, **(k) held open as a DEFECT rather than absorbed**.

**What is missing is neither a generator nor an engine feature. It is a ruling on what AGENTS.md is a mirror OF** -- and no divergence ledger can be written before that, because a ledger of permitted divergences presupposes a thing to diverge from.

## 4. Options, costed

**Option 1 -- fix the pointer only. XS.**
CLAUDE.md's template carries the 575-byte index instead of deferring. Delivered by the existing `claude upgrade --apply` path; **no new render path.** Injected-budget cost +575 bytes on a 6492-byte file, negligible against the ~87 KB session total WP-01 measured.
_Against:_ adds a **fourth** hand-kept home. Fixes the symptom and worsens what is underneath.
_Unmeasured:_ none. Every input to this cost is in section 2.

**Option 2 -- the index duplicated in source, and DRIFT-TESTED. S. RECOMMENDED.**
Put the 575-byte index in both `_CLAUDE.md` and `_AGENTS.md`, delivered by `claude upgrade --apply`, which already writes both. Add one arm to the existing invariant test: **the index block is byte-identical in every root-file template that carries it.**
_The mechanism is a test, not a generator, and that is deliberate._ It is the same argument `agents_sync_parity.rs` already makes for AGENTS.md itself -- you do not have to eliminate the duplication, you have to make drift **detectable**. A copy that cannot silently diverge is not the failure mode Highlander names.
_Effect:_ CLAUDE.md stops pointing at a file its reader never receives, and **the rule CLAUDE.md was protecting gets enforced by a test for the first time.**
_Explicit limit:_ **`usage-rules.md` cannot join** -- seeded once, user-owned after, so its copy stays hand-kept and outside the test. Option 2 reduces three uncounted homes to two tested ones plus one declared exception. **It does not reach zero, and saying otherwise would be the overstatement this document has already had to correct twice.**
_MEASURED, and it is why this option is shaped as a test rather than a generator:_ I flagged the shared-fragment mechanism as unmeasured, then measured it. **There is no include form.** `expand_tokens` substitutes exactly three tokens -- `PROJECT_NAME`, `AUTHOR`, `INTENT_VERSION` -- and an unrecognised `[[TOKEN]]` **refuses** rather than passing through (`rootfiles.rs:436-447`). The `[[#lang]]` / `[[#nolang]]` blocks are conditional inclusion of inline content, not file inclusion. **Single-sourcing the index would require a new token type in cc's template engine, which is M and buys only what the test arm already gets.**

**Option 3 -- declare AGENTS.md's contract. M. Needs hv, not me.**
State explicitly that AGENTS.md mirrors the injected set for agents that do NOT receive CLAUDE.md and skills, and write its divergence ledger on `parity.md`'s model. Requires settling whether init lays it down or whether its absence is correct.
_UNMEASURED:_ the ledger is the whole cost and I have not sized it. The existing `agents sync` ledger took eleven classes off one driven comparison; a mirror ledger has no comparable drive yet, so **M is an estimate by analogy, not a measurement.**

**Option 4 -- inject AGENTS.md into Claude Code. NOT RECOMMENDED; recorded so the rejection is visible.**
What correction 1's uncorrected version would have proposed. Doubles the injected budget to deliver content the agent already has, and makes a deliberately tool-agnostic file load-bearing for one specific tool -- the opposite of an honest cross-tool mirror.

## 5. Recommendation

**Option 2; Option 3 as the ruling it sets up.** It is the smallest change that removes the false pointer and converts uncounted duplication into tested duplication, on a delivery path that already exists, with no engine change.

**The earlier draft of this document said my recommendation would fall back to Option 1 if the shared-fragment mechanism did not exist. It does not exist -- and the fallback turned out to be unnecessary**, because the right mechanism was never a generator. Making drift detectable is what this project already does for AGENTS.md, and it is cheaper and stronger than single-sourcing. **The measurement changed the shape of the recommendation rather than its size.**

## 6. What needs a ruling

1. **Does AGENTS.md exist at fresh init?** Today it does not, and CLAUDE.md references it four times including its opening paragraph. Either init lays it down or those references stop assuming it. **Both answers are coherent and they lead to different documents.**
2. **What is AGENTS.md a mirror OF?** Option 3 is unwritable until this is answered.
3. **Generator or copy for the index** -- and, if generator, what to do about `usage-rules.md`, which cannot join.

## 7. Scope

**Nothing here is executed and none of it is mine to execute.** `_AGENTS.md` is untouched by me throughout ST0065, deliberately, since the ST0067 stop-the-press. `rootfiles.rs` and `canon.rs` are cc's. This goes to hv via vc; the build assignment is vc's to make.
