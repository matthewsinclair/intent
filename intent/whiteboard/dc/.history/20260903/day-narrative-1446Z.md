# dc, 2026-09-03 -- narrative for the 14:46Z localfold

Companion to `wip-prefold-1446Z.md` (sha `0a07d12a`, 64,829 bytes, `cmp`-verified verbatim). The board keeps rules; this keeps the reasoning.

## The session had one subject and it was my own instruments

Cold pickup at 07:33Z. hv held me, then handed me `dvb fullcycle` mid-turn, then authorised the release note. **Every substantive thing that went wrong today was an instrument I built for somebody else to read, and in each case the wrong answer was the reassuring one.**

## `dvb fullcycle` -- nothing broken, and the tempting figure was the wrong one

hv pasted `8.85x` against `1.72x` and asked for a review. Both verdict files were zero bytes and I nearly reported that as a fault; `print_run_verdict`'s own comment says **empty IS the green verdict**.

The decomposition: `clean rust` 117s + `build all` 121s + `test rust` 60s = 298s against 303s cycle wall. **238 of 303 seconds, 79%, were clean and build.** The test phase's own 1.72x is quiet on the meter's published scale.

devbin `0049` is CLOSED, 2026-09-02, explicitly as _wording fixed, number unchanged_. The vendored `runlog` is `cmp`-identical to upstream, so this is current devbin behaving as designed.

**What is NOT in 0049 and is the sharper statement:** the two lines are ONE denominator with TWO numerators. `34.23s across 14 suites` is identical in both because the outer log CONTAINS the inner phase's output -- the fullcycle log carries exactly 14 `finished in` lines, the same 14. So `8.85 / 1.72 = 5.14` is exactly `303 / 59`. **The outer line carries no information beyond the wall clock already printed three lines above it.** And structurally: **a nesting gate can never report quieter than its own test phase**, because the numerator is strictly larger and the denominator is the same file re-parsed. Not "wrong subject" -- monotonically inflated by construction.

**The proof was in the same run.** `clean rust` took 117s and printed NO overhead line, because it parsed zero `finished in` lines and the meter returns early. **The meter is silent when it sees nothing and confident when it sees something, and it never asks whether what it saw covers what it timed.**

## The release note -- four rounds, and I was the defect in three of them

### Round 1: the two limbs

Limb (i) PFIC and limb (ii) the `13 -> 17` schema. Both re-measured. **`intent upgrade` is the v2-to-v3 migration door, not a canon refresh** -- `Facade::upgrade`'s own doc comment, which deliberately bypasses `open` because the migrator runs on an unmigrated project by definition. So a project already on v3.0.0 does not pick corrected canon up from a tool upgrade, which is why the limb needed a reader-facing CHECK rather than a reassurance.

### Round 2: two blind checks, caught before commit

**Draft 1** told the reader `grep PFIC AGENTS.md`. **The shipped v3.0.0 `AGENTS.md` does not carry the fork** -- its only PFIC line is a skills description. The reader sees a clean line and stops looking.

**Draft 2** grepped `Pure-Functional-Idiomatic-Coordination`. That finds five of six; the sixth is `in-review`'s checklist -- **the home that produced the false green** -- which shipped the idiom list with no name at all.

Caught by extracting all six homes as `v3.0.0` actually shipped them and requiring the check to find every one.

### Round 3: the control was half a control (vc)

vc drove my instruction against the CORRECTED tree. **The repaired `in-review` line quotes the old gloss verbatim in order to explain the repair**, so a reader applying my rule concludes the one home that mattered is still forked, re-runs the remediation, sees it still there, and concludes the remediation does not work.

Driving it myself found a SECOND false positive vc did not name: **`in-standards/SKILL.md:61`**, a red-flag row about when pattern matching applies. Legitimate, unchanged since v3.0.0, in the file a reader greps first.

**The method defect is the keeper: I positive-controlled against the BROKEN population and only that. A control built from the defective corpus cannot exhibit a false positive on the fix, by construction.** W60 amended rather than a W61 added -- a second entry leaves the incomplete cure standing beside the complete one.

vc's one-layer-up reading: the whiteboard header guard **never scans prose, because nodes report this class by quoting it**. My note is a prose instrument with exactly that problem and I gave it no exemption.

### Round 4: the count, and I held the disproof

The note said SIX homes. **Four more -- `templates/{rust,lua,shell,swift}/RULES.md` -- were in my FIRST grep of the session, in the tool output, before any of this started.** I took `six` out of ic's commit message and never reconciled it against the measurement I had just run. A figure transcribed from a narrative into a shipped artefact, in a note whose whole subject is a claim that drifted from its owner.

**The count is gone rather than corrected.** Ten would rot the same way. The reader's instrument is the grep; the count belongs in the derivation.

**And the remediation gap is wider than vc found.** `intent/llm/RULES.md` itself -- the plain one, a home ic FIXED here -- is orphaned. `intent claude upgrade` never touches `intent/llm/` at all, and `intent lang show rust` says in its own words that `lang init` _installs nothing into the project_. It is not a v2-residue edge case affecting four language files; **it is the whole `intent/llm/` directory in every estate, including this one.**

## The four templates, and why checking the citation changed the fix

hv authorised the fix. Each forked line also carried a `Concretised by` citation, and vc had reported those as correct. **They are not.** `IN-RS-CODE-002` is _Ownership before clone_; the authoritative `concretised_by` on the PFIC rule says `IN-RS-CODE-003`.

The block is numbered **positionally** -- `-001` through `-004` against the four principles in order. Against the owning rules' own fields: **3 of 16 correct, and two of those three are coincidences of the numbering.** Thin Coordinator has NO non-Elixir concretisation at all, so all four cite a rule for a mapping that does not exist.

**So the fix drops the citation rather than correcting it.** The library owns the mapping and `intent claude rules show` prints it; a hand-written citation is a second home for a fact with an owner. Scope held: hv authorised the definitions, and the remaining 12 citations are reported rather than swept in, **because correcting them by hand would rebuild the roster that produced them.**

## The two things I got right by discipline rather than by luck

**The 62 commits.** The delivered pair is 62 behind HEAD and **0 of those touch `native/rust`, `surface` or `docs/design`.** Reporting "62 behind" would have been W22 in my own hand on the morning I picked up. Three nodes drove the same range separately and got the same zeros.

**The index lock.** cc's `git commit` held it twice. Both times: re-issue the SAME command, never recompose, never touch the lock. Measured that pid 58195 was live in its pre-commit gate rather than assuming staleness.

## What this fold cut

Watch-outs was **45,521 of 64,829 bytes -- 70% of the board.** The header's own standing rule is that a rule is never dropped here, only its narrative, and that is what this fold enforces on the section that had stopped obeying it.
