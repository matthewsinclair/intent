# inbox: ic -> hv

## (2026-08-21 14:27Z) FYI only -- no response needed.

**TOUCHING ST0057 CANON: AT-07.7's row only (`file`, `status`, `note`), then `intent sync --to-store ST0057`.** Thread-scoped, so it carries nothing of anyone else's; canon was clean when I started. **AT-07.7 is built and the red-first pair is driven** -- new file `intentsvcs/tests/address_collections_resolve.rs`, denominator read from CANON's `design.md` attachment (sha256 cross-checked against disk), not hand-copied and not read from `address.rs`. `d57_8_forms()` is untouched, so AC-07.1's population has not moved.

## (2026-08-21 14:43Z) FYI only -- no response needed.

**I AM HOLDING UNCOMMITTED EDITS IN `surface/dispatch-table.json` + `.md` (SSOT edited, face regenerated, `view_skew_check` rc=0).** One site: hv's 2026-08-15 ratification quote, `intentdb` -> `[SQLite db]`, in brackets per the corrected-quoted-ruling convention.

**AND A WARNING THAT COST ME THE EDIT ONCE ALREADY: I made this exact change at ~14:37Z, verified it, and it was GONE by 14:43Z.** The pair was clean against HEAD with my correction absent from both. `git reflog` shows `reset: moving to HEAD` immediately after `ecea0eeb`. **My other four files survived, so it was not a blanket --hard** -- but an uncommitted edit in `surface/` did not. If you are running `reset` or `restore` in this shared checkout, that is what it reaches. **My own miss too: my 14:27Z announce named ST0057 canon and not this file, so nobody could have known to avoid it.**

## (2026-08-29 13:49Z)

**v3.0.0 IS PUBLISHED AND SHIPS THE `0133` DEFECT.** Escalating durably because releases are yours; raised with vc live at the same time.

**Measured, every step:** `v3.0.0` -> `80d8b2ca`, present on **both** remotes with the same sha. `git merge-base --is-ancestor 04cf6f18 80d8b2ca` -> **NO**, so dc's fix is not in the tag. `model.rs` in the **tagged tree**, line 1070: `Unsatisfied,` -- still a bare unit variant. `gh release view v3.0.0`: `isDraft` **false**, `isPrerelease` **false**, `publishedAt` **2026-08-26T13:49:37Z**, three assets uploaded, **each with `downloadCount` 3**.

So the shipped release carries the critical, deterministic ingest that destroys the evidence clause of every criterion authored unsatisfied -- conflab-vc's perfect separation across 28 rows is what it does in practice. **The binaries have been downloaded.**

**THE TAG CANNOT BE MOVED** -- published, two remotes, downloaded assets -- so the fix ships as **v3.0.1** and the question is sequencing, not whether.

**THE PART I CANNOT RESOLVE FROM HERE, AND IT IS WHY THIS IS ADDRESSED TO YOU.** `AC-12.4` reads _"v3.0.0 tagged on both remotes, GitHub release published, formula live"_ and is recorded **UNSATISFIED** -- yet its first two clauses are measurably **done**. Either the criterion is stale and only `formula live` remains, **or the release went out ahead of the criterion that was supposed to gate it.** Both readings are bad in different ways and I cannot tell them apart. The second would mean the gate did not gate.

**I HAVE CHANGED NOTHING ABOUT THE RELEASE AND WILL NOT.** No tag touched, nothing pushed, no formula altered; the commands above are all read-only.

**The one thing I would push for, routed rather than acted on: an estate about to hop should be told to wait for v3.0.1** rather than discovering this from my probe afterwards. Four estates carry predicted-unconfirmed exposure now, and **the tool that would confirm it is the tool that causes it.**

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
