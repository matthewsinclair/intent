---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-29 23:10Z
status: active
focus: "BOUNCED after compact and RE-MEASURED, and both blockers I handed over were wrong. `0148` is nine paths, eight mine, and it clears on TWO CELLS in dc's data-model.md. The store outage has a working escape already on disk. All three peers were paused holding for me on facts that had moved; all three are told."
claims: [ST0056, ST0057, ST0058, ST0060, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body verbatim at `.history/20260829/wip-fold-2253Z.md`; the day's folds are all in that directory.

## DOING

**`0148`, waiting on dc.** Nine paths, eight of them mine. The one that is not is dc's `data-model.md`, and **it is in the knot because MY OWN scoped sync put it there** -- canon's sha for that file matches dc's worktree and differs from HEAD. Two cells (632-633) spell the fiat verb `fc --because` where `transitions.rs` and dc's own paragraph at 641 both say `ac.fc`. dc has the finding. Nothing else in the knot needs anyone.

## OPEN

1. **OWED BY ME, and the first is the one hv assigned:** the `AC-00.4` rewrite (`hv/wip.md:72` -- _vc holds that rewrite_, under the AC-11.1 / AC-07.5 precedent, because AC-00.4 would otherwise pass VACUOUSLY for the WP kind); the four missing `data-model.md` property tables (`Attachment`, `Legacy`, `Related`, `Envelope`) that WP-16's checker reds on; **ST0068 AC-02.1 and AC-02.3**; and **WP-15**, the 26-skill triage.
2. **FOR hv -- `AC-08.9` and `AC-08.10` want ratifying.** Both minted by me on cc's finding. The web view existed in three sessions' instructions, on two boards, and in NO criterion, so the gate would have computed WP-08 satisfied without seeing it.
3. **FOR hv -- the ST/WP `fc` EDGE.** `st.done` is `Guard::GatePass` and a fiat close is exactly where the gate does not pass, so reaching `completed` needs an `st.fc` edge with a `ReasonRecorded` guard. hv's _leaves that machine untouched_ was about the STATE SET. **It is a row in a ratified table, so it is not mine.** dc is unblocked meanwhile.
4. **FOR hv -- `intent/wip.md:22` CONTRADICTS the no-3.1.0 ruling.** It still records `intentd` (ST0064), `vault` (ST0060) and modules (ST0046) as OUT. ST0064 is a separate Triage thread for the same daemon WP-08 builds. **Does ST0064 fold into WP-08, or does WP-08 build the daemon and ST0064 keep only the menubar app?** I lean the second.
5. **A SYNC WELDS A PEER'S FILE INTO YOUR COMMIT, and that is a second, worse half of the sweep.** Everyone knows `sync --to-store` pulls peers' uncommitted attachment bytes into canon. The half nobody had stated: canon then NAMES those bytes, `canon_commit_check` gates on canon naming bytes the commit does not carry, so **the peer's file becomes mandatory in your commit and their unfinished work becomes your blocker.** I did this to myself and then reported it as an ownership problem for four hours.
6. **MY CANON IS STALE FOR MY OWN WRITE.** Canon's `design.md` sha matches HEAD and differs from my worktree -- the sync ran BEFORE the D56 rewrite, and `tui-design.md` is not in canon at all. `0148` needs a fresh scoped `sync --to-store ST0056` before it lands, and that sync will re-sweep whatever is dirty then.
7. **hv's parked stack**, unchanged: the ratified Guard column (the checker now reports 5 DISAGREE + 6 UNMEASURED on that axis, never gating); 0143's `--skip-settings`; ST0065's three questions; ST0066 minutia 3; the vacuous-remedy class.

## Watch-outs

**Mechanisms only. Incidents are in the fold archives.**

1. **A true result from an instrument that could not have answered differently.** The dominant class. Positive-control the INSTRUMENT, not the subject.
2. **AN INSTRUMENT THAT READS INDEX-ELSE-HEAD PASSES VACUOUSLY WHEN NOTHING IS STAGED.** `machine_table_check.sh` run bare compares HEAD to HEAD and prints _agree exactly_. **I nearly cleared dc's blocker on that green.** It is 1 with a specific and reusable shape: a gate that reports on a COMMIT cannot be exercised by a working tree. Stage first, then believe it.
3. **Relaying is authoring.** A number you pass on is a number you asserted.
4. **A claim outlives its basis and nothing announces it.** Three of mine expired across one compact: `0148` is five paths, no node can assemble it, the CLI is dead. All three were on peers' boards, holding all three of them.
5. **A TERMINATION CONDITION IS AN UNDECLARED FILTER.** `cargo test` aborting after a failing target; a 1500ms probe reporting `emacs -nw` broken when only the deadline was. **The flag that makes a run authoritative shrinks its denominator and nothing says so.**
6. **A RULE FAILS IN THE ARTEFACT THAT STATES IT.** Now seven. The newest is dc's, and it is the tidiest: a table cell spelling the verb one way, eight lines above dc's own paragraph spelling it the other, in the document that ratifies the machine.
7. **A TWO-WAY COMPARISON CANNOT ATTRIBUTE A DIFFERENCE TO ONE SIDE; THE FIX IS ALWAYS A THIRD INPUT.** canon-vs-worktree cannot say who moved. canon-vs-worktree-vs-HEAD can, and it is what identified my own stale `design.md` and dc's swept `data-model.md` in one pass.
8. **A CAPABILITY THAT WORKS AND IS UNLABELLED IS INDISTINGUISHABLE FROM ONE THAT DOES NOT EXIST.** `target/debug/intent` answered all evening while four nodes recorded the CLI as dead.
9. **RULE BEFORE YOU READ THE BOARD AND YOU WILL RULE ON SETTLED GROUND.** Twice: the four-kinds question, and the ST/WP fork where **I ruled the DECLINED option and dropped the composer that carries the whole protection.**
10. **A LINE NUMBER FROM A FILTERED STREAM IS A CLAIM ABOUT THE STREAM.** I gave hv `CHANGELOG.md:26` off an `awk` pipe; it was line 33.
11. **`$?` AFTER A PIPE IS THE LAST STAGE'S.** Fourth firing, mine this time: I read `rc=0` off a piped `machine_table_check` whose true rc was 1, in the same turn I was measuring a divergence.
12. **A GREP PATTERN THAT CANNOT MATCH THE OUTPUT FORMAT RETURNS 0 AND READS AS ABSENCE.** `^\s*AC-` against a stream that prints `ac: AC-17.7`. The unanchored positive control in the same command is the only reason I did not report zero criteria.
13. **THE SUITE AND THE FORMATTER ARE SEPARATE GATES AND THE SUITE CANNOT SEE THE FORMATTER** -- fmt/clippy live in CI, so there is no local alarm. `cargo fmt --check` before declaring a lane ready.
14. **THE ONE ACT EVERYBODY BELIEVES IS READ-ONLY IS WHAT MUTATES SHARED DURABLE STATE.** `cargo test` opens the live root and runs the migration ladder; 22 `intentsvcs` files do it. dc's finding, cc's sharpening, and the reason the AC-08.2 harness takes a temp fixture.
15. **Canon is the SSOT for rows, not for prose.** A record may QUOTE what was said; it must POINT AT, never reproduce, what is currently true.
16. **A false ABSENCE closes the question; every other wrong answer invites a second look.**
17. **Design against the LARGEST real subject.** The strawman found ten defects on ST0056 that were invisible on ST0058.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **`D56`: `intentd` emits JSON ONLY and every renderer is generic.** Ruled by hv 2026-08-29, REVERSING the same day's axum+maud server-rendered ruling. **The deciding argument is ST0064's SwiftUI menubar app, which cannot consume HTML** -- an HTML face would have meant two servers or two truths. The daemon resolves the form DSL server-side; no renderer knows what a criterion is. Conflab's `daemon_bridge.js` is the cited precedent. **Recorded in `design.md` and `tui-design.md` §10a, not here.**
- **The daemon's pages INHERIT `docs/design/design-system.md`.** Its founding principle is _the site renders like the tool_ and these pages ARE the tool. Self-hosted, same-origin, no CDN -- a constraint, because the daemon serves localhost.
- **Announce a write to a SHARED file to everyone; announce a write to a CLAIM to the claim-holder.** Claims predict who cares, not who is blocked.
- **An opening announcement states the PROPERTY that ends it, not a promise to send a second message.** cc's convention, adopted after two nodes in two days announced an opening and never its close.
- **Rule 13 canon edit path:** edit the extract, `intent sync --to-store <ID>` scoped, then `--to-disk <ID>`, then commit file and canon TOGETHER. Sync reads the WORKTREE.
- **Docs are written against the CUT, never against `main`.**
- **Em dash in prose pages; `--` in generated reference pages.**
- **Design decisions go to the Laksa design agent, not settled here** (hv), each carrying the decision, its constraint, and what breaks if it goes the other way.
