# COSTED PROPOSAL -- what AGENTS.md is for

**DESIGN-FIRST. NOTHING HAS BEEN EDITED.** Written 2026-08-28 16:43Z by ic for vc, then hv. No template, no root file and no skill has been touched for this document. hv ruled AGENTS.md design-first on 2026-08-28 and this is the design.

## 0. The correction this document is built on

**WP-01 finding 1 said AGENTS.md is never injected, and that is true. I then extended it to "in a fresh project the four cross-language principles are readable nowhere", and THAT IS FALSE.** I measured the AGENTS.md side and reasoned about the rest.

`/in-standards` carries all four rule IDs with glosses in a table at lines 26-29, and `/in-session` auto-loads it every session. **A Claude Code agent receives the four principles every time.** The corrected finding is recorded here rather than quietly fixed, because a proposal built on the wrong problem would have proposed the wrong thing -- and what it would have proposed (inject AGENTS.md) is the most expensive option on the list.

## 1. The problem, stated correctly

**AGENTS.md is not broken and the rules are not missing. Two correct local decisions compose into a wrong global outcome.**

- **CLAUDE.md deliberately does not state the four rules**, on Highlander grounds, and sends its reader to AGENTS.md.
- **AGENTS.md deliberately does state them** -- ratified deviation **(c)** in `parity.md`: _"v3 adds `## Rules of the Road` (the four agnostic principles with rule ids)."_
- **And AGENTS.md is the one file the Claude Code agent never receives.**

So CLAUDE.md's pointer cannot land for its own primary consumer. It points at content that consumer has already been handed by an auto-loaded skill, via a file that does not exist in a fresh project.

**The second half is the one nobody has counted.** The four-rule index has **three live homes** -- `AGENTS.md`, `usage-rules.md`, `in-standards/SKILL.md` -- and CLAUDE.md has **zero**, deliberately. **The only document that reasoned about Highlander is the only one honouring it, and the rule it honours was already violated three ways.**

## 2. Measurements

Every row measured on this tree today; the controls are stated because a green from an uncontrolled instrument is not evidence.

| Claim                                       | Measured                                                                                                                 | Control                                            |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------- |
| AGENTS.md absent at fresh init              | real `intent init` in a clean dir: ABSENT; appears only after `intent agents sync` (6672 bytes)                          | CLAUDE.md and config.json PRESENT in the same init |
| AGENTS.md not in Claude Code's injected set | observed directly in this session's own context: CLAUDE.md (user + project) and MEMORY.md are injected, AGENTS.md is not | -- (direct observation, not inference)             |
| `/in-standards` carries all four IDs        | lines 26-29, table with slug + gloss                                                                                     | a nonexistent rule id returns 0 in the same file   |
| three live homes for the index              | AGENTS.md 1, usage-rules.md 1, in-standards 2, **CLAUDE.md 0**                                                           | same grep for a nonexistent id: 0 everywhere       |
| the rule bodies live in the library         | all four resolve, rc=0, 6.7-8.5 KB each (~31 KB total)                                                                   | a nonexistent id: rc=1                             |
| the in-document block is an INDEX           | 575 bytes in AGENTS.md against ~31 KB of rule content                                                                    | --                                                 |

**The last two rows dissolve the Highlander objection.** The rules' single home is the rule library. A 575-byte names-and-IDs block is a table of contents, not a duplicate body. **CLAUDE.md is applying Highlander to an index**, and paying for it with a pointer that does not land.

## 3. The shape to cost against, and the good news in it

vc named the prior art and it is stronger than expected: **the mirror machinery for AGENTS.md already exists.**

- **Generator** -- `intent agents sync`, and `rootfiles.rs` already renders all three root files, not just AGENTS.md.
- **Structural-invariant test** -- `agents_sync_parity.rs` (AT-07.5). **Property, not roster**: _"No heading is followed by another heading"_ covers a section added tomorrow by somebody thinking about something else, which a hand-kept list does not. Byte-parity explicitly refused as a mechanism.
- **Ratified-divergence ledger** -- `parity.md`, `agents sync` deviations, classes (a)-(j) ratified and **(k) held open as a DEFECT rather than absorbed as a deviation.**

**So hv's "AGENTS.md becomes the honest cross-tool mirror" is mostly BUILT.** What is missing is not machinery. It is a stated answer to _who reads AGENTS.md, and what is it a mirror OF_ -- and until that is answered no divergence ledger can be written, because a ledger of permitted divergences presupposes a thing to diverge FROM.

## 4. Options, costed

**Option 1 -- fix the pointer only. XS.**
CLAUDE.md carries the 575-byte index instead of deferring. Injected-budget cost is +575 bytes on a 6492-byte file, negligible against the ~87 KB session total WP-01 measured. Removes a pointer that cannot land.
_Against:_ adds a **fourth** hand-kept home. Fixes the symptom and worsens the thing underneath it.

**Option 2 -- Option 1, generated. S. RECOMMENDED.**
One source for the four-rule index; `agents sync` renders it into AGENTS.md, CLAUDE.md and usage-rules.md. **The generator already writes all three files**, so this is a template change plus one arm on the existing invariant test -- _the index renders identically in every root file that carries it_ -- which is a property, in the shape the existing test already argues for.
_Effect:_ three homes stop being a Highlander violation and become three rendered views of one source. The rule CLAUDE.md was protecting is actually enforced, by a test, for the first time.
_Against:_ does not answer what AGENTS.md is FOR. It makes the current arrangement honest rather than deciding the arrangement.

**Option 3 -- declare AGENTS.md's contract. M. Needs hv, not me.**
State explicitly that AGENTS.md mirrors the injected set for agents that do NOT receive CLAUDE.md and skills, and write its divergence ledger on `parity.md`'s model -- what it may legitimately omit, add, or restate. Requires settling whether init lays it down (today it does not) or whether its absence is correct and the four CLAUDE.md references must stop assuming it.
_Against:_ the ledger is the whole cost, and it cannot be written before the mirror's subject is ruled.

**Option 4 -- inject AGENTS.md into Claude Code. NOT RECOMMENDED, recorded so it is visibly rejected.**
This is what I would have proposed before the section 0 correction. It doubles the injected budget to deliver content the agent already has, and it makes a file whose whole purpose is being _tool-agnostic_ load-bearing for one specific tool -- the opposite of an honest cross-tool mirror.

## 5. Recommendation

**Option 2 now; Option 3 as the ruling it sets up.** Option 2 is the smallest change that both removes the false pointer and converts three homes from an uncounted violation into a tested mirror, using machinery that already exists and a test whose design argument already covers the case. It does not pre-empt Option 3 -- it makes Option 3 cheaper, because once the index is generated, the divergence ledger has one source to be a ledger against.

## 6. What needs a ruling before any of this is built

1. **Does AGENTS.md exist at fresh init?** Today it does not, and CLAUDE.md references it four times including in its first paragraph. Either init lays it down, or those four references must stop assuming it. **This is a ruling, not a preference** -- both answers are coherent and they lead to different documents.
2. **What is AGENTS.md a mirror OF?** Option 3 is unwritable until this is answered.
3. **Does the four-rule index get a generator (Option 2) or a fourth copy (Option 1)?** I recommend the generator, but a copy is a legitimate call if the estate would rather not grow the render path.

## 7. Scope note

**Nothing here is executed and nothing is mine to execute.** `_AGENTS.md` is untouched by me throughout ST0065 -- deliberately, since the stop-the-press on ST0067 -- and `rootfiles.rs` is cc's. This document goes to vc, then hv, and the build assignment is vc's to make.
