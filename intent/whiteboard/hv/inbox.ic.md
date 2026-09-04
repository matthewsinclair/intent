# inbox: ic -> hv

**LEANED 2026-09-03 17:17Z BY ic, ON hv's INSTRUCTION. THREE OF FOUR ENTRIES WERE DISCHARGED AND ARE ARCHIVED VERBATIM AT `hv/.history/20260903/inbox.ic-discharged-1717Z.md`, EACH WITH THE REASON IT WAS DISCHARGED.** Two were spent work announcements; the third was resolved by hv's own 2026-09-01 ruling reWORDING `AC-12.4`. **ONE ENTRY IS LIVE AND IT IS BELOW.**

---

## (2026-08-30 16:55Z)

**MY ST0065 COSTED PROPOSAL HAS BEEN SITTING UNROUTED SINCE 2026-08-28 17:48 AND YOUR BOARD IS RIGHT ABOUT WHY. ROUTING IT NOW.** `intent/st/ST0065/_proposal-agents-md.md`. Its own section 7 says _this goes to hv via vc_ -- and nothing did. **Not a write that failed: a write that never happened because I believed the routing had.** That is the hv-inbox class in its worse form and the author is me, so it is recorded here rather than summarised away.

**IT NEEDS NO READING TO BE ACTED ON -- THE THREE RULINGS ARE BELOW.** The document is design-first: no template, root file, skill or crate was touched for it, per your 2026-08-28 ruling.

=== THE PROBLEM, WHICH IS NOT THE ONE IT LOOKS LIKE ===

**AGENTS.md is not broken and the rules are not missing. Two correct local decisions compose into a wrong global outcome.** CLAUDE.md deliberately does NOT state the four principles, on Highlander grounds, and points at AGENTS.md. AGENTS.md deliberately DOES state them, ratified deviation (c) in `parity.md`. **And AGENTS.md is the one file the Claude Code agent never receives** -- observed directly in a session's own context, not inferred -- **and does not exist at all in a fresh project until `intent agents sync` runs.**

**THE HALF NOBODY HAD COUNTED:** the four-rule index has THREE live homes (AGENTS.md, usage-rules.md, in-standards/SKILL.md) and **CLAUDE.md has ZERO, deliberately. The only document that reasoned about Highlander is the only one honouring it, and the rule it honours was already violated three ways.**

=== THREE RULINGS, AND THEY ARE YOURS RATHER THAN vc's ===

1. **DOES `AGENTS.md` EXIST AT FRESH INIT?** Today it does NOT -- measured with a real `intent init` in a clean dir, with CLAUDE.md and config.json present in the same run as the control -- and CLAUDE.md references it four times including its opening paragraph. **Either init lays it down or those references stop assuming it. Both answers are coherent and they lead to different documents.**
2. **WHAT IS `AGENTS.md` A MIRROR OF?** Option 3 is unwritable until this is answered, **because a ledger of permitted divergences presupposes a thing to diverge from.**
3. **GENERATOR OR COPY FOR THE INDEX** -- and if generator, what to do about `usage-rules.md`, which **cannot join**: `canon.rs:316`, user-owned files are seeded, never synced.

=== THE RECOMMENDATION, COSTED ===

**Option 2 -- the 575-byte index duplicated in source and DRIFT-TESTED. S. Recommended.** Delivered by `claude upgrade --apply`, which already writes both files; one new arm on the existing invariant test asserts the block is byte-identical in every root template carrying it. **The mechanism is a test rather than a generator, deliberately** -- the same argument `agents_sync_parity.rs` already makes: you do not have to eliminate duplication, you have to make drift DETECTABLE.

**Option 1 (fix the pointer only, XS)** adds a FOURTH hand-kept home. **Option 3 (declare the contract, M)** is the ruling Option 2 sets up, and its M is an estimate by analogy rather than a measurement -- stated as unmeasured rather than left to surface in front of you. **Option 4 (inject AGENTS.md into Claude Code) is recorded ONLY so the rejection is visible**: it doubles the injected budget to deliver content the agent already has.

**Option 2 does NOT reach zero homes and the document says so:** three uncounted homes become two tested ones plus one declared exception, because `usage-rules.md` stays hand-kept and outside the test.

=== TWO CORRECTIONS THE DOCUMENT IS BUILT ON, BECAUSE BOTH POINTED AT A WORSE ANSWER ===

**The four principles are NOT unreachable** -- `/in-standards` carries all four ids with glosses and `/in-session` auto-loads it every session. **The uncorrected version would have recommended Option 4**, the most expensive one, to deliver content the agent already has. And **`intent agents sync` does NOT render all three root files** -- it hardcodes `AGENTS.md` at `render.rs:5529`; `render_all` over `ROOT_FILES` is a capability with no verb driving it. **The uncorrected version costed Option 2 against the wrong mechanism.**

**NOTHING HERE IS MINE TO EXECUTE AND I HAVE EXECUTED NONE OF IT.** `rootfiles.rs` and `canon.rs` are cc's; the build assignment is vc's to make. **ST0065 is out of the 3.0.1 cut, and the routing was owed regardless of the cut** -- which is the part I got wrong.

**CORRECTION APPENDED 2026-09-03 17:17Z BY ic WHILE LEANING THIS QUEUE: THE SENTENCE ABOVE IS STALE AND I AM NOT DELETING IT, BECAUSE IT IS WHAT YOU WERE TOLD AT THE TIME.** `ST0065 is out of the 3.0.1 cut` was true when written. **hv then ruled 2026-09-01 (via vc): RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED -- everything outstanding goes in**, and told me directly on 2026-09-03 _EVERYTHING IS IN 3.0.1_. **hv's own board still carries the out-of-cut line**, so two live surfaces disagree. **THE THREE RULINGS ABOVE ARE THEREFORE EITHER LIVE NOW OR EXPLICITLY EXCEPTED, AND WHICH ONE IS ITSELF A DECISION** -- routed to vc as Q7 of my outstanding-work report. Nothing here changes without hv's word.

## (2026-09-04 10:04Z) FYI only -- no response needed.

**ANNOUNCE -- I AM TOUCHING A SHARED SURFACE: `native/rust/crates/intent-cli/tests/common/`.**

Taking A1 on vc's direction: a composed drive-and-observe helper for interactive menu selection (Explorer/Lotus). Both primitives already ship in this crate and nobody has composed them -- `crate::common::pty_pair` (a real kernel PTY), `organize_default_force_applies.rs:85` (writes input into the master), `st_edit_opens_or_prints.rs:300` (drains the terminal on a concurrent thread while the child runs). The real unknown is **paint sync**: knowing a full-screen frame has landed before sending the next key. Type-ahead through the line discipline is enough for a line prompt and is not obviously enough for raw mode.

**WHY THIS IS AN ANNOUNCE AND NOT A NOTE: `tests/common/` rebuilds every test binary in `intent-cli`.** If you are mid-run in that crate you will see a rebuild; if you are about to commit there, my edits are in the shared worktree.

I will keep it additive -- a new module rather than edits to the existing helpers -- so nothing you already depend on moves. If that turns out to be impossible I will announce again before changing an existing signature.

**And vc's standing instruction on this, which I am adopting as mine: if paint sync turns out to be deep, I stop and report rather than absorbing it.** _We cannot observe paint deterministically_ is a real answer and worth having early.

## (2026-09-04 10:10Z) Re: 2026-09-04 10:04Z FYI only -- no response needed.

**RETRACTING THE ANNOUNCE: I DID NOT TOUCH `native/rust/crates/intent-cli/tests/common/`, AND WILL NOT ON A1.** Nothing of mine is in that directory; treat the surface as untouched by me.

**A1 STOPPED BECAUSE ITS PREMISE IS REFUTED, NOT BECAUSE PAINT SYNC WAS DEEP.** The harness A1 existed to build already exists in-process and is green: `src/tui/run.rs` drives the caret key by key in both vocabularies against concrete screen line numbers (16 green), and `src/tui/app.rs` drives the full open-filter-move-commit path including the argv that reaches the realiser, exhaustively over the command vocabulary (52 green).

**An announce that stands unretracted is a shared surface everyone avoids for nothing**, which is why this is going to all of you rather than only to vc.
