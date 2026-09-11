# inbox: dc -> hv

## (2026-09-11 08:26Z)

**LEANED ON hv's INSTRUCTION, AFTER THE FULL CONTENT WAS SENT TO vc.** Eight entries are now four open items. The four closed entries are archived: the 08-21 and 08-25 FYIs; AC-00.3, which is ruled and satisfied; and the 09-05 build announcement, whose push has since happened. The same archive holds the full text of every open item: `intent/whiteboard/dc/.history/20260911/hv-inbox-dc-prelean-0824Z.md` (sha256 starting `baf771eba80a141a`). I re-drove every state below at 2026-09-11 08:22Z. Drive it again before acting on it.

**1. ONE RULING OVER SEVEN THINGS** (first put 2026-09-10 09:33Z, widened 09:51Z). Three shipped doors answer rc=2 `not implemented yet`: `st bootstrap`, `agents template`, `claude prime`. Four bats suites (`agent_commands`, `skills_commands`, `rule_index`, `claude_prime`) are dispositioned `keep` in `intent/st/ST0056/parity/register.md` with their burn recorded. All seven carry a disposition that outlived its subject. For each one: retire it, re-point it, or say the subject is less retired than I measured. Seven is the minimum, not the count: 57 of 127 paths were never probed. **This is the biggest single lever on CI's red.** CI Ubuntu run 34523263681 has 216 failures: 186 are this class, 15 are wrong-binary (see 3e), and 15 are unattributed (1 driven and environment-dependent, 14 unmeasured). That split is of Ubuntu's 216 only; macOS's 214 are not enumerated. Ruling this unblocks AC-12.1 and AC-00.6.

**2. `int local status`: BEFORE OR AFTER THE CUT?** (first put 2026-09-10 09:33Z). Delivered at `8c5a82fa`. It counts daemons by executable and reports store holders as a separate number. Where it sits relative to the cut is the only open question.

**3. FIVE ITEMS FROM 2026-09-02 16:32Z, NONE RULED.**

- **(a) ST0057 AT-07.5 needs a stop-the-daemon window.** The AT reads green, but its arm A refuses to run while any intentd is up, so nobody can reproduce that green on demand.
- **(b) The v3.0.0 -> v3.0.1 store migration takes the schema from 13 to 17 and cannot be undone, and nothing shipped says so.** Before the cut it needs a release note, a refusal message, or a backup step.
- **(c) Issues 0207, 0203 and 0208 are all open, and the remedies are yours.** AC-04.6 waits on 0207.
- **(d) The `overhead` meter reports a figure about the wrong subject (dc W46).** You have not weighed in on it.
- **(e) Flip, then burn, in that order.** The flip means rebinding the default `INTENT_BIN`: `tests/unit/test_helper.bash:21` points it at v2, and that is what causes the 15 wrong-binary failures in item 1. A burn taken before the flip inherits all 15. Neither step needs a build. This unblocks AC-06.1.

**4. AN OPEN QUESTION, NOT A FINDING: WHAT DOES A TEST RUN COST THIS MACHINE?** (first put 2026-09-10 09:51Z). `native/rust/target` holds 247,053 files and has no `.metadata_never_index`, and Dropbox's `fileproviderd` runs high while the `local` remote lives in Dropbox. Proving cause needs a suite run under observation. Nothing is asked of you until someone has that measurement.

-- dc
