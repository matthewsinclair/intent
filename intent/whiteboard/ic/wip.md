---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-28 13:20Z
status: active
focus: "AWAKE AFTER THE COMPACT, NOTHING IN FLIGHT, HOLDING FOR vc. Conflab is on v3 and Phase 4 closed green at 7652c9b4 -- 779/779 views 0 skew, canon 123 == st list 123, flat 4 == declared 4 == WIP 4, buckets 119 unmoved, AC/AT exact, 0 dangling. Inboxes all empty at pickup; index clean; rules 3 and 4 stand. MY INTENT CLAIMS ARE PAUSED, NOT ABANDONED. I START NOTHING UNTIL vc SAYS SO."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. Phase 4 closed by vc; folded and paused.** Full record of the day archived verbatim at `.history/20260828/wip.md` -- retractions, mutation tables and all.

**WHAT LANDED:** Conflab hopped to v3 and every parity leg I was asked to verify passed. Routed out and not mine: `wp list` reach became issue 0103 (cc), doctor's `gate-not-running` false positive and the json-no-counted-marker gap went to dc.

**MY INTENT CLAIMS ARE PAUSED, NOT ABANDONED** -- ST0057/{02,05,07,08,11,14} and ST0061 sat untouched all day under the Conflab reassignment.

## TODO

- **PICKUP DONE AT 13:20Z, AND IT FOUND NOTHING FOR ME.** All four inboxes at the `_(empty)_` sentinel; shared index clean; vc is `active` at 13:14Z and itself holding for hv's three picks (D2 fiat-close, D3 Superseded->Cancelled sequencing, D4 AC-00.8 amend). **I read vc's board rather than trusting my own TODO** -- a held list is a claim about a moment, and this is the re-measurement.
- **THE PHASE 4 INSTRUMENTS DIED WITH THE SESSION.** `parity4.py`, `census_cmp.py`, `dangling.py` were scratchpad-only. Designs and mutation tables are in the archive; **making them durable is hv's call, not a thing to do quietly** -- they would be new apparatus in a tree whose parity tools are gated.

## Watch-outs

**THE DAY'S ONE CLASS, AND IT COST ME THREE RETRACTIONS: I COMPARED NUMBERS WITHOUT ASKING WHAT POPULATION EACH COUNTED.**

- **531 vs 216 WPs** -- canon holds all 531, `wp list` returns rc=0 with zero rows for 71 threads. Reach, not loss. **I caught this one ONLY because a 315 gap looks like catastrophe.**
- **190 vs 54 findings** -- 54 is doctor's COUNTED figure, 190 is the json array including 136 uncounted advisories. **I did NOT catch it, and told my coordinator their correct number was wrong.** Both figures plausible, so nothing looked odd. **THE SIZE OF A DISCREPANCY IS WHAT MAKES IT VISIBLE, NOT ANY VIRTUE IN THE READING** -- so the discipline is _ask what population each number counts_, every time, not _notice when they look wrong_.
- **`1dd65db8` vs `b4d63b44`** -- sha256 of the file vs the commit marker. **I compared an identifier to an identifier without establishing they name the same KIND of thing.**

**A VERIFICATION IS A CLAIM ABOUT A MOMENT, AND I BROKE THIS ONE HAVING WRITTEN IT DOWN.** I measured Conflab's carrier at 08:55Z, accurate and PRE-HOP. Post-hop I reached for that reading to corroborate a doctor finding instead of re-reading a file replaced an hour earlier. **A FALSE POSITIVE AND A STALE MEASUREMENT CORROBORATED EACH OTHER INTO A CONCLUSION NEITHER COULD SUPPORT ALONE.** Two sources agreeing is the control I demand of everyone; here the agreement was the trap. **Independence is not two instruments -- it is two instruments whose errors cannot share a cause, and _both taken before the file changed_ is a shared cause.** Applied correctly two hours later: re-measured `~/.intent/home` for vc rather than handing them the morning value.

**`names no X` AND `does no X` ARE DIFFERENT CLAIMS.** The shim names no `GUARD_RUNNER` **because it delegates** (`:129` resolves the gate, `:152` execs it). **A component that does not NAME a thing may be the one that CALLS it**, and a check written before the delegating layer cannot tell them apart. That is doctor's false positive, and it recruited me because both premises were individually defensible. **A stale reading is a mistake; a true premise with a false conclusion is a trap, and the second recruits careful people** (cc's framing, better than mine).

**A RETRACTION DOES NOT OVERTAKE THE FALSE THING IT IS CHASING.** Mine was two nodes downstream before it reached one. **Send a retraction to every node the claim could have reached, and say what to PULL, not just what was wrong.**

**INSTRUMENTS.**

- **TWO SIBLINGS IN ONE DIRECTORY HAD OPPOSITE `ROOT=` CONTRACTS** -- `view_skew_check.sh:78` honours it, `thread_view_skew_check.sh:70` discards it silently, and ROOT selects only the BINARY while the ESTATE comes from CWD. Both returned rc=1 for incompatible reasons. **Only invocation that is right: `cd` into the estate, absolute path. Only control that catches the wrong tree regardless: assert every reported path is in the estate you meant** -- or read the DENOMINATOR, which is what actually proved it (779 vs Intent's 288).
- **A ROW THAT READS PASS CAN BE THE ONE THAT MATTERS.** A mutation put one thread in two buckets; every leg passed because my own set union absorbed the duplicate. Only tell: per-bucket counts summing to 120 against a union of 119.
- **A BASELINE THAT FAILS FIRST RUN IS THE FIXTURE TALKING, AND THAT IS THE INSTRUMENT WORKING.** Fix the fixture, never the leg, and re-run the whole table.
- **AN EMPTY READ AND A FAILED READ ARE DIFFERENT, AND A REFUSAL AT THE WRONG ALTITUDE ABORTS THE SWEEP IT PROTECTS.** Refusing per-thread on empty AC/AT would have died on the first of Conflab's 119 criteria-less threads. **Population proof belongs at the SWEEP, and the denominator rides on every run.**
- **SPLIT AN INVARIANT FROM A FINGERPRINT.** `declaration == flat` is standing; `declaration == WIP` is true only at a fresh hop and drifts on the next op. Fused, it is a false finding waiting for next week.
- **A PREDICTED-ABSENT ID HAS THREE OUTCOMES, NOT TWO** -- the third (present as a row) falsifies the mechanism claim. **A prediction is only a control if its falsifying outcome is enumerated.**

**MECHANICS THAT BIT TODAY.**

- **`cmd | grep; echo $?` READS grep's CODE** -- I did it inside the verification of a refusal arm, the one place the plausible wrong number goes unnoticed. **Capture to a FILE and measure the file.**
- **`git status --short | head -10` SHOWED ME 7 LINES OF 159** and hid 152 staged deletions a peer had already counted. `| head -N` is `| tail -N`.
- **BACKTICKS IN AN `echo` OPEN A COMMAND SUBSTITUTION** and `<ST>` inside parses as a redirect; the whole call dies.
- **`git commit --only` PLUS A STAGING HOOK LEAVES THE FILE STAGED BEHIND HEAD**, accumulating one stale entry per commit until `git diff --cached --name-only` is permanently non-empty -- defeating the peer-mid-write check. **Append `&& git reset -q -- <paths>`.** Adopted estate-wide.
- **`git status` IS A SNAPSHOT OF WHOEVER IS MID-WRITE.** A peer's staged board cleared itself between two of my reads -- their commit landing. **Wait and re-read; do not diagnose.**
- **I WROTE A PLACEHOLDER STAMP (`12:5xZ`) INTO A RETRACTION**, the one defect all three clock-guard arms are blind to. **The moment of writing a correction is when care goes into the content and out of the mechanics.**
- **`--to-disk` WRITES THE EXTRACT, NOT ONLY VIEWS.** Harmless when canon already agrees; on a dirty canon it reverts a hand edit silently, and "only that view changes" is the sentence that stops anyone checking.

## Decisions

- **(hv, 2026-08-28, first-hand) THE AUTHORITY CHAIN IS A TREE:** hv -> `intent-vc` -> {`intent-{dc,ic,cc}`, `devbin-vc` -> devbin nodes, `conflab-vc` -> conflab nodes}. **My only channel up is `intent-vc`** -- not hv directly, not laterally.
- **(ic, 2026-08-28) MEASURING ANOTHER ESTATE IS NOT DIRECTING ITS NODES.** Phase 4 was Conflab work by an `intent-*` node: vc assigned it, vc got the report, Conflab stayed read-only, and cc's census reached me via vc rather than by my asking across.
- **(vc, 2026-08-28) Conflab post-hop is 123 canon == 123 st list, 4 flat, 119 bucketed and UNMOVED** -- `migrate.rs:47` never relocates a thread out of v2's buckets. Confirmed by measurement, not relay.
