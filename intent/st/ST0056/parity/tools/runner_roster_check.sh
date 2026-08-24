#!/bin/bash
# runner_roster_check.sh -- every parity instrument declares whether anything runs it.
#
# Issue 0059: eleven instruments lived in this directory, three had an execution
# site, and eight had none. Each of those eight had been written, reasoned about
# at length in its own header, registered in MODULES.md, mutation-tested, and in
# three cases CITED BY A GREEN ACCEPTANCE ROW -- with every individual record
# correct, because an instrument's existence and an instrument's execution are
# recorded in different places and NOTHING JOINED THEM. This is that join.
#
# WHY THE JOIN IS A THIRD AXIS AND NOT A COLUMN ON AN EXISTING ONE. MODULES.md
# records what a tool IS, and in prose what its POSTURE is ("REPORTS, never
# gates") -- that is the script's own internal contract, and it is true whether
# or not anything ever calls the script. `precommit` records what the GATE runs.
# Those two can each be perfectly accurate while the answer to "does anything
# run this?" is nowhere. So this file adds the missing fact -- a DISPOSITION --
# and then measures it against the runner instead of trusting it.
#
# THE POPULATION HAD ALREADY MOVED BEFORE THE FIX WAS WRITTEN, WHICH IS THE
# ARGUMENT FOR THE REFUSAL BELOW. 0059 measured eleven instruments at 0f87fc2c
# and its table lists eleven. By the time the remedy was built there were TWELVE
# -- `ratified_in_check.sh` (renamed `rulings_check.sh` 2026-08-23) landed
#    hours after the census, unwired, and no
# artefact in the repository noticed. A correct measurement of a population that
# has silently acquired a member is the failure this check has to survive, so it
# does not compare against a remembered count: it enumerates the directory every
# run and REFUSES on any file it has no row for. The roster fails on the day a
# tool is added, which is the only day anybody is in a position to classify it.
#
# WHY IT ASKS THE RUNNER RATHER THAN REIMPLEMENTING ITS RULE. `precommit`'s own
# header states the doctrine, and states it because it was learned the hard way:
# `int hooks` derived the guard roster by grepping the runner's source, anchored
# on a PATH SHAPE, and under-reported a three-guard gate as two the same day.
# `--list-guards` exists so a reader can ask. This asks.
#
# DECLARED AND INVOKED ARE MEASURED SEPARATELY, BECAUSE THE GUARD-0 ROT IS
# EXACTLY THEIR DISAGREEMENT. A guard can be named in `--list-guards` and
# implemented nowhere, or invoked in the body and named in no roster; both are
# real and both have happened in this file. One measurement cannot see either.
#
# Exit codes follow the family: 0 clean, 1 a finding, 2 cannot measure. This one
# GATES on its findings, unlike its report-only siblings, and the difference is
# principled rather than a mood. A report-only check is report-only because most
# of its hits are a legitimate mid-ladder state -- a command not wired yet, a
# row not ratified yet. There is no legitimate state in which a tool exists and
# its disposition is undeclared: the fix is one line in the roster below, and
# the whole point is to force it at the moment the tool arrives.

set -uo pipefail

die() { echo "error: $1" >&2; exit 2; }

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" || die "cannot resolve my own directory"
# tools -> parity -> ST0056 -> st -> intent -> repo root. FIVE. The count is
# taken from guide_refs_check.sh's header rather than recounted here: the
# version that was one short resolved to `intent/` and died on every single
# invocation, including all four mutants, which the harness then scored as kills.
ROOT="$(cd "$HERE/../../../../.." && pwd)" || die "cannot resolve the repository root from $HERE"

# RUNNER is overridable for the mutation proof below, following the family's
# existing `TABLE=` and `BIN=` precedent. It exists because the calibration on
# line ~140 is a guard on MY OWN NEEDLE, and a guard that has never been fired
# is an unrun claim -- the failure this whole check is about, one level down.
RUNNER="${RUNNER:-$ROOT/bin/.devbin/cmd/precommit}"
DISPATCH="$ROOT/bin/int"

[ -d "$HERE" ] || die "no tools directory at $HERE"
[ -f "$RUNNER" ] || die "no repo-local runner at $RUNNER -- there is nothing to measure a disposition against"
[ -x "$DISPATCH" ] || die "no devbin dispatcher at $DISPATCH -- the runner only answers through it"

# THE ROSTER. One row per `.sh` in EITHER thread's tools directory:
# <name> <gated|manual|not-an-instrument> <reason>.
#
# `gated`  -- the repo-local pre-commit gate runs it, bare, on every commit.
# `manual` -- it is an instrument and the gate does not run it; the reason says why.
# `not-an-instrument` -- it is not a checker at all: a sourced library, a
#             generator, an extractor, a capture driver, a stub that exists to be
#             driven, or a transformer that rewrites a tree.
#
# A REASON IS REQUIRED ON EVERY KIND, INCLUDING `not-an-instrument` (hv, ruled
# 2026-08-21). This EXTENDS the emptiness rule that already governed `manual`
# rather than adding new machinery -- the check below is one more arm, not a new
# mechanism. The reason it now reaches the third kind: a bare
# `not-an-instrument` costs nothing to write, so a genuine instrument can be
# declared out of the population by anyone who finds this guard inconvenient,
# and the guard goes blind again WITH A SIGNATURE ON IT -- which is worse than
# an undeclared file, because a declaration reads as a decision somebody made
# rather than as a gap. Requiring the reason makes the mislabel expensive to
# write and visible in the diff that adds it.
#
# WHAT COUNTS AS AN INSTRUMENT, because hv's ruling widened the population and
# left the word undefined, and a builder inventing that boundary at build time
# is a builder choosing what this guard is allowed to see. An instrument
# ANSWERS A QUESTION ABOUT THE ESTATE AND RETURNS A VERDICT.
#
# THE MECHANICAL DISCRIMINATOR IS vc's AND IT BEAT MINE (vc, 2026-08-21):
# **does the file's own header cite an AT or AC row it covers?** If it does it is
# definitionally an instrument -- its verdict gates a criterion -- and the
# classification is asserted by the author at the site rather than guessed here.
# Driven over the 33 files this widening admitted: 12 cite, 21 do not.
#
# I TRIED exit-1 PRESENCE FIRST AND IT MISFIRES, which is recorded because the
# next person will reach for it too: `gen_dispatch_table.sh` and `gen_pertest.sh`
# both carry `exit 1` and are generators -- that exit is a refusal to EMIT on bad
# input, never a finding about the estate. **The citation test corrected three of
# my own classifications** -- `estate_census.sh`, `estate_corpus.sh` and
# `of_n_population.sh` all cite criteria and I had written them off as producers.
# All three would have been silently removed from the population.
#
# EVERY ST0057 ROW IS STRUCTURALLY `manual` TODAY, AND NOT ONE OF THEM IS A
# JUDGEMENT ABOUT THAT TOOL. The runner pins `TOOLS` to ST0056's directory at
# `bin/.devbin/cmd/precommit:116`, so it cannot invoke an ST0057 instrument at
# all -- a `gated` row for one would fail check B immediately, correctly,
# reporting that nothing in the runner's body invokes it. **This widening admits
# ST0057's seven tools to the ROSTER; it does not make them GATEABLE.** Making
# one gateable needs the runner to grow a second tools path, which is a change
# to `cmd/precommit` and therefore dc's. Filed rather than assumed, so the next
# reader does not take seven `manual` rows as seven separate rulings.
#
# AND THE CLASS ARRIVED INSIDE THE EDIT THAT DOCUMENTS IT -- TWICE, THE SECOND
# TIME AS MY OWN CORRECTION, WHICH IS THE INSTRUCTIVE HALF.
#
# I first wrote `no_daemon_required.sh`'s row citing the `pgrep -f` needle defect
# as the reason it cannot be gated. ic told me the needle was fixed. **I did
# re-derive -- and I re-derived the WRONG SUBJECT.** I measured that `pgrep -x`
# is the correct needle (true, reproducible, and I drove both directions) and
# offered that as proof THE FILE CONTAINS IT. Different subject entirely.
# `c0749463` CREATED that file WITH the defect -- 302 insertions, `pgrep -f` at
# `:106` and `:233` -- HEAD still carries it, and the fix is UNCOMMITTED in ic's
# worktree. **So the reason I struck as expired had not expired: the CORRECTION
# was the error.** Caught by ic and vc independently, neither of whom I asked.
#
# AND THE ROW MOVED A THIRD TIME WITHIN THE HOUR: ic committed the fix at
# `6edbd24f`, so the correction ABOVE went stale too. **Three states in one day
# for one row's reason -- blocked-by-the-needle, wrongly-fixed, live-at-HEAD,
# fixed-and-committed.** The lesson is not to re-derive faster. It is that **a
# reason churning that fast should not be load-bearing at all**: this row now
# rests on the structural fact (`cmd/precommit:116` pins `TOOLS` to ST0056's
# directory) which has not moved once, and the needle is recorded as history
# beside it rather than as the reason.
#
# THE LESSON IS NOT `re-derive`. IT IS RE-DERIVE THE SUBJECT THE CLAIM IS ABOUT.
# A true measurement of a different property is the most persuasive wrong
# evidence there is, precisely because the measuring was real and careful.
#
# AND `FIXED` IS NOT A STATE (vc, 2026-08-21). Worktree, index, HEAD and pushed
# are four states. Every node on this estate holds dirty files and commits only
# on hv's word, so a peer saying "I fixed it" is reporting the FIRST while the
# reader hears the THIRD -- that gap is this estate's default condition, not an
# edge case. **A commit citation pointing the wrong way does not fail the next
# checker; it CONFIRMS to them that the fix is in**, which is worse than a
# citation that cannot be resolved at all, and the cheap does-the-file-carry-the-id
# split does not reach it.
#
# UNDER UNCERTAINTY THE ROW GOES `manual`, NEVER `not-an-instrument`, and the
# asymmetry is the point (vc's framing): a wrong `manual` costs a reason, stays
# visible, and anyone can correct it; a wrong `not-an-instrument` removes the
# file from the population, so it generates no signal ever again and nothing will
# surface the mistake. Fail toward the loud error.
#
# Every cost below was MEASURED on 2026-08-17 at 55e540df, not estimated, per
# the runner's own rule that coverage is reported as measured and never as
# designed. Re-time before moving a row on cost grounds.
# EVERY TIMING BELOW IS MEASURED AND NAMES WHAT IT IS A MEASUREMENT OF (ic,
# 2026-08-18). They were previously bare -- eleven figures, not one naming a
# machine, a revision or a tree size -- which dc counted after making the error
# those figures invite: comparing a fresh measurement against 3077ms read out of
# this string, taken on another machine at another time over a smaller tree.
# MEASURED-AGAINST-RECORDED IS NOT A COMPARISON AT ALL, and a bare figure here is
# an invitation to it. dc corrected their own row and correctly declined to
# restate the other ten, since re-stating a figure you did not take repeats the
# error; these are re-MEASURED, not re-stated.
#
# METHOD: /usr/bin/time -p wall clock (NOT the shell builtin over a subshell,
# which under-reported dc's by roughly half), 5 runs each, min-max, darwin/arm64,
# over 8f652d1b..08bed4b2 -- HEAD moved four whiteboard/canon commits during the
# sweep and the figures were stable across it, which is stated rather than
# resolved by pinning one sha the numbers were not all taken at.
#
# THE RE-MEASUREMENT WAS WORTH DOING BECAUSE THE OLD FIGURES WERE WRONG IN BOTH
# DIRECTIONS, so no single correction factor explains them and no reader could
# have adjusted for them: nine overstated by up to 39%, and ratified_in_check.sh
# UNDERSTATED by 49% -- it reads the dispatch table, which has grown. Costs that
# grow silently are the ones a stale figure hides, and the total gate cost is
# ~4.6s measured against ~5.4s recorded. THE AFFORDABILITY ARGUMENT FOR ADMITTING
# canon_commit_check.sh WAS CONDUCTED AGAINST THE STALE NUMBERS.
#
# Re-time before moving a row on cost grounds, and record the span you measured
# over. One sub-figure is NOT mine and is left attributed: the 263ms dispatcher
# component of runner_roster_check.sh is the original author's and I did not
# reproduce the breakdown, only the total.
ROSTER='
class_vocab_check.sh       gated   30-40ms, two committed files, one-line verdict
corrected_check.sh         gated   50-60ms, static, reports and never gates
generator_inputs_check.sh  gated   140-180ms, index-only, whole-set by design
provenance_check.sh        gated   290-310ms, three greps over the stamped artefacts
residue_class_check.sh     gated   40-50ms and a single line, the cheapest here
rulings_check.sh           gated   240-290ms, static read of the dispatch table
runner_roster_check.sh     gated   470-490ms total (re-measured); the original row attributed 263ms of a then-782ms total to asking the runner through the dispatcher -- THAT COMPONENT IS NOT RE-MEASURED AND 263 OF 480 IS NOT THE SAME CLAIM AS 263 OF 782, so treat the breakdown as unverified, which is the price of not re-grepping its source; it is a *_check.sh and rosters itself
self_provenance_check.sh   gated   530-540ms re-measured at c51f10d5 on one machine, up from 470-510ms: each binary line now carries a sha256, which costs two hashes of ~9MB and buys the only token on the line that distinguishes one build from another -- THE MARKER DOES NOT, and three distinct binaries carrying dirty-18197aaf in one day is what proved it. 27 blobs read from the INDEX; whole-set because the failure is staging one of two facts, so a path trigger would have to fire on the path that is not there
stale_at_check.sh          gated   50ms and a single line, reports presence only
view_skew_check.sh         gated   the slowest gated one, path-triggered. RE-MEASURED 2026-08-21 (cc): 3336-4100ms, n=7, mode ~3407, max/mode 1.2x, at 0dea9abb ON ONE MACHINE at loadavg 32.4 over 16 cores, timed inside ONE python3 process with perf_counter around subprocess.run, harness floor 5.7ms. THE SUPERSEDED 2860-2940ms IS NOT WITHDRAWN AND IS NOT WRONG: it was a different sitting, and the two together are the finding. EVERY TIMING ROW IN THIS TABLE CARRIED A REVISION AND A MACHINE AND NO LOAD STATE, WHICH IS UNCOMPARABLE ON A MACHINE FOUR SESSIONS SHARE ALL DAY (cc named the axis, dc measured it: 21 claude processes and 6 rustc live at the time). dc published 65-74ms for provenance_fields_check.sh while correcting a bad figure and it read 53-62ms twenty minutes later with nothing about the tool changed -- so a POINT was never available and re-timing cannot fix that, only naming the condition can. STATE A RANGE ACROSS SITTINGS, and if you need a cost argument, run it in the condition the gate will run in
canon_commit_check.sh      gated   ADMITTED 2026-08-21 by dc on the hv release. THE TWO HOLDING REASONS ARE BOTH DEAD AND WERE RE-DERIVED HERE RATHER THAN TAKEN ON REPORT: (1) no narrow attachment-sync verb -- `sync --to-store [ID]...` takes positional ids and landed at 212b0075, so the only order a gate permits (sync the one thread, then commit file and canon together) is open; (2) --staged unsupported -- :254 parses the flag and :364 reads `git diff-index --cached HEAD`, so it judges the INDEX. Dispatched ALWAYS --staged and NEVER path-triggered: the tool narrows internally and a path trigger would be a second copy of that narrowing. RE-TIMED 2710-2760ms narrowed at ecea0eeb ON ONE MACHINE against 2.49-2.55s recorded -- ~8% slower, which REFRESHES the affordability case rather than overturning it, but it is now the most expensive guard here and the gate total moves ~4.6s -> ~7.3s on every commit; the comment at :102 warned that the original affordability argument was conducted against stale numbers, and this is that re-measurement. NO APOSTROPHES IN THIS TABLE: ROSTER is a single-quoted shell string, so one of that character terminates it and breaks the parse -- this row did exactly that when first written, twice, the second time inside the sentence warning about it. Reads git only: no worktree, no binary, no clock. LAYOUT-AGNOSTIC as of 2026-08-18: it detects the canon layout per revision (nested intent/st/<ID>/thread.json or flat intent/.canon/st/<ID>.json), REFUSES a half-migrated tree rather than counting one half, and reads the thread id from canon CONTENT rather than from the path -- because a path strip whose pattern is absent returns the string UNCHANGED at rc=0, which emitted ST0056.json as a steel-thread id and looked entirely plausible. 2.49-2.55s narrowed / 11.3-11.5s --exhaustive, measured at 4ba598f1 ON ONE MACHINE (the superseded figures, 2.1-2.3s and 9.5-9.7s at f2a2675f, were two-machine; that difference is part of the figure). ~1.8x SLOWER ON PURPOSE: scoped used to count narrowing FILTER KEYS rather than attachments examined, which overstated on the nested layout and printed EXAMINED 2 of 1 with the other -1 on the flat one; correcting it costs one extra pass over every canon. STATED WITH SUBJECT AND REVISION BECAUSE THE BARE FIGURES IN THE ROWS ABOVE CANNOT BE COMPARED AGAINST ANYTHING: measuring your own tool and comparing it to one of them is measured-against-recorded across unknown machines, trees and dates, which is how the first timing claim for THIS row went wrong by half. Driven five ways on a purpose-built rig spanning both layouts (nested clean 0, flat clean 0, flat diverged 1, canon with no id 2, both layouts present 2) plus nine real commits verdict-identical to the superseded version, and it caught an unplanted divergence on its first whole-tree run
provenance_fields_check.sh manual  new 2026-08-21 (dc), covering AC-11.7 via AT-11.7. MANUAL AND NOT GATED, WITH A NAMED RELEASE CONDITION, BECAUSE ITS SUBJECT FAILS TODAY: dist-provenance.txt carries a bare commit and no artefact hash, so gating it now delivers a PERMANENTLY-RED gate -- the same reason thread_view_skew_check.sh was new rather than late. RELEASE CONDITION: promote to gated once int macos stage emits the labelled partition (artefact_sha256 + answers IDENTITY, source commit + answers CURRENCY, drift HELD + release condition); the writer fix is bin/.devbin/cmd/macos and is dc lane, unstarted. TWO ARMS, DIFFERENT SUBJECTS, REPORTED SEPARATELY. FIELDS asks whether the record is well-formed. SET asks whether the record describes the BYTES beside it and whether those bytes agree with EACH OTHER -- added 2026-08-21 after cc found, on real bytes, that a per-record check cannot see a property of the SET: intent named dirty-483e65e4 and intentd named dirty-5819417b beside a record naming 26fe1aea, three disagreements, every file individually well-formed. NOT A COPY OF artefact_commit_blockers in bin/.devbin/cmd/macos: that pivots on the TAG at release time and only at release time, this pivots on the RECORD at any time. COST MOVED AND THIS ROW HAS NOW CARRIED THREE FIGURES, WHICH IS WHY THE HARNESS IS NAMED AND NOT JUST THE NUMBER: it runs strings over target/release/intent at 9478896 bytes and intentd at 373136, measured 53-74ms across two sittings at b4918a35/bb3dce99 -- median 67 over 7 runs, then median 56 over 7 runs twenty minutes later -- timed inside ONE python3 process with perf_counter around subprocess.run, harness floor (bash -c true) median 4.9ms, ON A MACHINE AT LOAD AVERAGE 28 OVER 16 CORES with 21 claude processes and 6 rustc live. Still no git, no worktree, no clock. THE EARLIER 20ms IS SUPERSEDED BY THE SET ARM AND THE 82-88ms IS WITHDRAWN AS AN ARTEFACT OF ITS OWN HARNESS: that run spawned two python3 processes PER ITERATION to read the clock, at 23-35ms each, so roughly 50ms of the figure was the instrument measuring itself -- a TRUE measurement of script-plus-harness offered as the size of the script. THE LOAD CONDITION IS cc AND IT CORRECTS ME TWICE OVER (cc, 2026-08-21 17:03Z, their stamp and their measurement). I put it to them that their two rows might carry the same 50ms artefact; they RAN IT rather than accepting it, and their harness floor is 5ms rather than 50, with both figures coming back HIGHER than recorded -- and an inflation artefact would have made a clean re-measurement LOWER. The hypothesis was mine, the measurement did not exist, and it was true only of my own row. What DOES generalise is theirs: a figure carrying a revision and a machine but no LOAD STATE is still uncomparable when four sessions share the machine all day, which is how this estate works. It is a defect in every timing row in this table, mine included, and re-timing cannot fix it -- only naming the condition can. THAT IS WHY THIS ROW NOW STATES A RANGE ACROSS SITTINGS RATHER THAN A POINT: the point figure moved 15% in twenty minutes with nothing about the tool changing. SELF-TEST IS THE POINT: --self-test drives SEVEN controls, each shown able to fire alone -- a record with no source commit (the f2e4d1f9005d0334 currency failure, correct hash and 158 commits behind), a record with no artefact hash (the dirty-18197aaf identity failure, one marker over intent at 9008848 bytes and intentd at 373136), a compliant record that must PASS, a coherent set matching its record that must PASS, a set whose members disagree checked against a record naming NO commit so only the set pivot can speak, a coherent set the record is not about so only the record pivot can speak, and an empty set that must report NOT EXAMINED rather than passing silently. REACH: FIELDS checks fields EXIST and are LABELLED; SET checks the record and the artefacts name ONE commit between them; NEITHER can check that an embedded marker is HONEST about the bytes carrying it, which is the drift field and drift is held.
guard_home_check.sh        gated   new 2026-08-22 (dc), closing the exposure under hv guard-resolution directive whose PREMISE ic falsified: the guards do NOT resolve out of the frozen v2 checkout, because pre-commit.sh overrides GUARD_HOME to the repo root when the repo is itself an Intent install. hv asked for a mechanism rather than a variable and the mechanism ALREADY EXISTED with nothing watching it. GATES THE TRACKED TEMPLATE, NEVER THE INSTALLED COPY: pre-commit.intent is gitignored by design so a fresh clone has none, and a check keyed to it would fail in every clone, which is the ARM C shape AC-01.5 spent two days on. BYTE-IDENTITY WAS THE CANARY OFFERED AND IT IS THE WRONG ONE: an ACTIVE tree and a FROZEN one are supposed to diverge, so that check goes red on the first legitimate guard edit and is cry-wolf by construction; identity is not the property to protect, it is the reason the fallback is currently INVISIBLE. Two arms asserted separately because they fail differently -- a missing condition means the override can never fire, a missing assignment means it fires and does nothing -- and one combined grep would send the reader at the wrong half. Mutation-proven four ways: green, condition removed (rc=1), assignment removed (rc=1), and a tree that is not a tool tree (NOT APPLICABLE, rc=0). 17-22ms n=9 median 19, timed inside ONE python3 process with perf_counter around subprocess.run, harness floor median 5ms, ON ONE MACHINE at loadavg 47.7 over 16 cores -- the cheapest gated instrument here. Reads two greps over one committed file: no git, no binary, no clock. REACH: it checks the override EXISTS and assigns the repo root; it does not run the hook, does not compare guard bodies across trees, and cannot tell whether the branch CONDITION is correct for any particular install
conservation_check.sh      manual  takes a MIGRATED tree as an argument and no such tree exists until WP-10 lands, so there is no bare invocation for a gate to make; it refuses with exit 2 rather than passing when handed an unmigrated one, which is the behaviour a gate would have to bypass on every commit
thread_view_skew_check.sh  manual  new 2026-08-20, awaiting admission by dc -- cc built it, dc rosters. COST, RE-MEASURED 2026-08-21 (cc): 142-175ms, n=15, mode ~151, max/mode 1.2x, at 0dea9abb ON ONE MACHINE at loadavg 32.4 over 16 cores, one python3 process with perf_counter around subprocess.run, harness floor 5.7ms, over 268 views. Consistent with the 130-150ms recorded at f0c2805c; both stand, as two sittings rather than a correction. It would still be the CHEAPEST gated instrument here and ~22x faster than view_skew_check.sh, and the RATIO is what this figure is for -- a bare number cannot be compared against anything, which is why the harness and the load are named beside it. A 3x TAIL WAS OBSERVED ONCE IN 22 RUNS AND HAS NOT RECURRED, AND IT IS RECORDED AS AN OBSERVATION RATHER THAN AS A PROPERTY. One 7-run sitting read 160 163 165 167 170 188 513; dc and cc both reasoned from it that the distribution is bimodal under contention and that a gate pays the tail rather than the median -- a rule that would DECIDE DIFFERENTLY about promotion. Fifteen runs at HIGHER load produced no second mode and no tail. THE 513 IS REAL AND THE GENERALISATION WAS NOT TESTED: two nodes agreed because both liked the reasoning, and the agreement is what stopped either of them re-running it. IF THE TAIL IS REAL IT WILL RECUR AND THIS ROW CAN SAY SO THEN; a promote-to-gated argument does not get to rest on a number nobody can reproduce. IT COVERS THE POPULATION THE SIBLING NEVER DID: the CHECKABLE in view_skew_check.sh is ONE triple under surface/, so gated skew coverage is 1 of 269 and the missing 268 are the thread covers, acceptance contracts and WP covers. It forms no verdict -- views::skew is the single home for the question and this parses one answer rather than computing a second. UNGATEABLE BEFORE b082b488, which is why it is new rather than late: doctor reported every dehydrated view as MISSING, 235 findings at rc=1 on a healthy tree, so wiring it then would have delivered a permanently-red gate. Driven ten arms on a purpose-built rig: clean, planted skew with and without --changed, inherited-only, absent binary, changed output shape, doctor exiting 2, a 0-view denominator, and a MENTION/SUBJECT pair carrying identical decoy lines to opposite verdicts. Refuses at exit 2 rather than passing when it cannot read the summary line, because a text-reading gate whose needle stops matching goes green forever and nothing says so. Needs the v3 binary at native/rust/target/release/intent (v3 is off PATH by ruling); absent, it names what goes unchecked and exits 0
drift_check.sh             manual  compares a STAMPED inventory against live canon, so gating it would block a dispatch-table edit until somebody re-runs a 27-family measurement sweep -- a measurement, not a fix
guide_refs_check.sh        manual  takes required prose-file arguments, so there is no bare invocation for a gate to make
same_end_state_check.sh    manual  takes three tree arguments, so there is no bare invocation for a gate to make; it refuses an absent, EMPTY or UNCHANGED subject rather than comparing nothing, refuses two subjects that are one directory rather than comparing a tree with itself, and reports a differing SQLite store (or its -wal/-shm sidecar) as NOT JUDGED BY THIS TOOL, naming the path and the reason, because comparing the content of a container needs the sqlite3 shell and that would make the verdict depend on the machine
implemented_check.sh       manual  invokes every declared row in a fresh throwaway project with a sandboxed HOME
surface_check.sh           manual  probes --help across 100+ paths, so every commit would pay for a full surface sweep
burn.sh                        not-an-instrument produces burn-baseline.tsv by running each BATS file twice; a data producer other tools read, forms no verdict
canon_clone_completeness.sh    manual            ST0057 AT-01.2; checks by CLONING the repository, so gating it would clone on every commit
canon_concurrent_diff.sh       manual            ST0057 AT-01.4; edits two threads and inspects the resulting diffs, so it mutates a tree and has no bare gate-safe invocation
canon_ignore_dispatch_rig.sh   manual            ST0057 AT-01.5; plants an ignore rule and requires a real commit to be REFUSED, so it cannot run inside the commit it is testing
coverage_map.sh                not-an-instrument renders which command families the BATS estate covers; a map for a reader, forms no verdict
critic_global_rig.sh           manual            half A of the pre-commit critic gate; the gate already runs the critic directly, so gating the rig as well would pay the critic cost twice per commit
estate_census.sh               manual            ST0056; CITES AC-10.5 -- takes a v2 estate as its subject and none exists in this tree, so there is no bare invocation for a gate to make
estate_corpus.sh               manual            ST0056; CITES AC-10.5 -- captures the v2 estate the migrator is exercised against, and that estate does not exist in this tree, so there is no bare invocation for a gate to make
extract_flags.sh               not-an-instrument takes a script argument and prints its flags; a field extractor its callers consume, forms no verdict
extract_verbs.sh               not-an-instrument takes a script argument and prints its subcommands; a field extractor its callers consume, forms no verdict
fixture_probe.sh               manual            the second predicate beside burn, classifying per file for the parity inventory; CLASSIFIED manual RATHER THAN not-an-instrument BECAUSE I COULD NOT TELL, and manual keeps it inside the adjudicated population where a wrong not-an-instrument would blind this guard with a signature on it
gen_dispatch_table.sh          not-an-instrument generator; its exit 1 is a refusal to EMIT on bad input and never a finding about the estate, which is exactly why exit-1 presence cannot classify this table
gen_inventory.sh               not-an-instrument generator; emits the parity inventory that generator_inputs_check.sh then audits
gen_pertest.sh                 not-an-instrument generator; its exit 1 is a usage refusal, not a finding
gen_register.sh                not-an-instrument generator; emits the test register
intentfiles_reviewable.sh      manual            ST0057; needs a realisation change already in the working tree to review, so there is no bare invocation for a gate to make
interrupt_rig.sh               manual            produces the two trees the cutover gate compares by SIGKILLing a migration mid-run; it DOES form a verdict, but it takes minutes and needs a migrated estate
lib_classify.sh                not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_corpus.sh                  not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_mdfmt.sh                   not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_surface.sh                 not-an-instrument sourced, not executed; ships 644 and defines functions only
no_daemon_required.sh          manual            ST0057 AT-07.5; CITES AC-07.5. Manual for a STRUCTURAL reason that stands alone and has never moved: cmd/precommit:116 pins TOOLS to ST0056s directory, so the runner cannot invoke ANY ST0057 instrument. THE NEEDLE IS NOW FIXED AND COMMITTED -- ic landed pgrep -x intentd at 6edbd24f, verified at HEAD (:128 live, zero live pgrep -f). THIS ROW HAS BEEN CORRECT-THEN-STALE THREE TIMES IN ONE DAY, WHICH IS WHY IT RESTS ON THE STRUCTURAL FACT AND NOT ON THE NEEDLE: the needle blocked gating (true), then I recorded it fixed at c0749463 (false, that commit CREATED the defect), then I recorded it live-at-HEAD-with-the-fix-uncommitted (true when written, stale within the hour). A reason that moves three times in a day is not a reason to cite, it is a reason to stop citing
of_n_labels_its_derivation.sh  manual            ST0056 AT-00.12 covering AC-00.11; mode 2 of the criterion over a different population, driven on demand rather than per commit
of_n_population.sh             manual            ST0056; CITES AC-00.11 through AT-00.11 and AT-00.12 -- enumerates the population those two rows drive, run when that population changes rather than per commit
partition.sh                   manual            ST0057; answers which tracked files under intent/ have no store row, which is hv standing question -- a measurement, not a per-commit gate
probe.sh                       not-an-instrument runtime surface capture recording exit code and output bytes per invocation; forms no verdict
probe_toplevel.sh              not-an-instrument the driver that produces probes/toplevel.tsv; a capture driver, forms no verdict
read_claim_probe.sh            manual            witnesses whether a row declaring read_or_mutate read actually leaves the filesystem alone; runs the binary once per row, far too slow to gate
realise_plan.sh                not-an-instrument computes what intent organize would do and what it would refuse, an executable reading of realisation.md; a plan, not a verdict
retarget.sh                    not-an-instrument rewrites a sacrificial worktree to thread invocations through INTENT_BIN; a transformer that forms no verdict, and it must never run against the live tree
rig_selftest.sh                manual            drives interrupt_rig.sh against stubs whose behaviour is known and scores each arm against a prediction written before the run; a proof OF the rig, run when the rig changes
rig_stub_migrator.sh           not-an-instrument a migrator whose behaviour is CHOSEN, existing to be driven BY interrupt_rig.sh; a stub subject rather than a checker
sparse_tree_equals_manifest.sh manual            ST0057; runs organize and compares the tree against .intentfiles, so it mutates a tree and has no bare gate-safe invocation
sync_issue_loss.sh             manual            ST0056; the REPRO for intent#0069 attribution -- asks whether a thread-scoped sync --to-store destroys ISSUES. It does NOT (issues 3 to 3, both v3 binaries), which is what exonerated the sync as the cause of the 2026-08-24 loss. MANUAL because it stands up a throwaway project and runs a mutating verb in it, so there is no gate-safe bare invocation. Kept rather than deleted after the answer came back negative: a NO-CHANGE result is a result, and the next reader who suspects sync should re-run this rather than re-derive it
upgrade_issue_loss.sh          manual            ST0056; the REPRO for intent#0070 -- intent upgrade destroys every issue in an ALREADY-MIGRATED v3 project. Two arms, both v3 binaries: issues 5 to 0, threads intact. v2 is CLEAN across a real 2.10.0 to 2.19.0 migration, which bounds the defect to v3 and makes it a v3-only fix under the hv freeze scope. THE FIRST v2 ARM WAS A FALSE CLEAN and the script now forces the real path: v2 printed already at 2.19.0 and SHORT-CIRCUITED, so 5 to 5 measured only that a no-op destroys nothing. ASSERTS ON COUNTS READ FROM THE STORE, NEVER ON sync REPORTING AGREE -- a regression test that asserts via the agreement report inherits intent#0069 and the reporting defect then hides this one. MANUAL because it mutates a throwaway project; the fix is cc lane and the red arm must be THIS script so the test and the diagnosis do not share an author
'

# ---------------------------------------------------------------------------
# Populations. Each is enumerated, never remembered, and an empty one refuses.
# ---------------------------------------------------------------------------

# 1. PRESENT -- what THIS COMMIT holds, read from the index and never from the
#    working tree.
#
#    **It globbed the worktree until 2026-08-17, and in a shared clone that
#    froze every node's commits on paths they had never touched** (found by dc,
#    who held the commit and diagnosed it rather than reaching for
#    `--no-verify`). Four sessions work this one checkout, so any peer's
#    untracked mid-work `*_check.sh` was an unrostered tool to this guard --
#    and the only way past it was to wait for its owner to land a roster row.
#    A guard that has to be waited out is one step from a guard that gets
#    bypassed.
#
#    **The purpose survives exactly, which is the thing to check before
#    changing a guard**: a tool that is added AND STAGED is in this commit's
#    index and is still caught on the day it arrives, which is the only day
#    anyone is in a position to classify it. What stops being caught is a file
#    that is not part of the project and is not the committer's business.
#    `git ls-files` honours `GIT_INDEX_FILE`, and git hands a hook a temporary
#    index during a partial commit, so under `--only` this reads HEAD plus the
#    committer's own named paths -- which is the population it should judge.
#    Verified both ways at this tree: worktree glob 15, index read 15, same
#    names; and 15 again from a HEAD-only index built with `read-tree`.
#    THREE BOUNDS LIVED ON THIS LINE AND ONLY ONE OF THEM WAS EVER DELIBERATE
#    (vc named the third, 2026-08-21). It read
#    `git -C "$HERE" ls-files -- "$HERE/*_check.sh"`, which bounded the
#    population to (a) TRACKED files, (b) ONE DIRECTORY, and (c) ONE FILENAME
#    SHAPE. hv's ruling of 2026-08-21 removes (b) and (c). (a) STAYS AND IS A
#    CHOICE, stated here because it previously read as an accident: this is a
#    pre-commit guard, so the index is exactly the population it should judge,
#    and a `git add`ed tool does appear. An untracked file is not part of the
#    commit and is not the committer's business.
#
#    (b) WAS STRUCTURAL RATHER THAN A FILTER ANYONE COULD DROP, and that is why
#    ST0057's whole toolset was invisible: `$HERE` is `dirname
#    "${BASH_SOURCE[0]}"`, so a guard anchored on it can only ever see the
#    directory it lives in. The pathspec is now repo-relative and rooted at
#    $ROOT, which is the only form that can address a second thread.
#
#    KEYED BY BASENAME, SO A COLLISION ACROSS THREADS MUST REFUSE. Two threads
#    carrying the same filename would silently merge into one roster row and one
#    of the two would go unadjudicated while everything read clean. Zero
#    collisions measured at 510d4b10; that is a fact about today, so it is
#    asserted rather than assumed.
PRESENT="$(git -C "$ROOT" ls-files -- 'intent/st/*/parity/tools/*.sh' | sed 's|.*/||' | sort)"

# The INDEX mode of each parity file, keyed by basename: `<basename> <mode>`.
#
# **Read from git, not from `stat`, because the mode git RECORDS is what a
# clone gets.** A file can be 755 on this disk and 644 in the index -- that is
# exactly the state a `mv` over an already-staged file leaves behind, and the
# operator sees a working copy that runs.
MODES="$(git -C "$ROOT" ls-files -s -- 'intent/st/*/parity/tools/*.sh' \
  | sed 's|^\([0-9]*\) [^\t]*\t.*/|\1 |' | awk '{ print $2, $1 }' | sort)"
[ -n "$PRESENT" ] || die "this commit holds no parity .sh under any intent/st/*/parity/tools -- an empty population and a clean roster compare equal, so this is a refusal and not a pass"

COLLIDE="$(printf '%s\n' "$PRESENT" | uniq -d)"
[ -z "$COLLIDE" ] || die "two threads carry a parity file with the same basename ($(printf '%s' "$COLLIDE" | tr '\n' ' ')) -- this roster is keyed by basename, so a collision silently merges two files into one row and leaves one of them unadjudicated"

# 2. ROSTERED -- what this file declares.
# `$1 !~ /^#/` MATCHES THE PARSER LOOP BELOW, WHICH ALREADY SKIPPED `#` ROWS.
# The two disagreed: a commented row would have entered ROSTERED as the literal
# `#` and then been reported as "# has a roster row and NO file". Latent until
# somebody commented a row, which the widening makes likely.
ROSTERED="$(printf '%s\n' "$ROSTER" | awk 'NF && $1 !~ /^#/ { print $1 }' | sort)"
[ -n "$ROSTERED" ] || die "the roster in this file parsed empty -- its format has changed under the parser"

# 3. DECLARED -- what the runner says it checks, asked rather than inferred.
#    Field 2 is a repo-relative path or a dash; only the ones under this
#    directory are ours to have an opinion about.
GUARDS="$("$DISPATCH" precommit --list-guards 2>/dev/null)" || die "the runner refused --list-guards; its roster cannot be read"
[ -n "$GUARDS" ] || die "the runner's --list-guards printed nothing -- it declares guards, so an empty answer is a broken reader and not a gate with no guards"
DECLARED="$(printf '%s\n' "$GUARDS" | awk -F'\t' '$2 != "" && $2 != "-" { n = split($2, p, "/"); print p[n] }' | grep '\.sh$' | sort)"

# 4. INVOKED -- what the runner's body actually calls.
#
#    The needle is `$TOOLS/<name>`, and it is anchored on a variable this file
#    does not own, so it is CALIBRATED rather than trusted. My own watch-out,
#    fired three times already: a needle written from the author's head
#    enumerates the spellings the author remembers. If the runner stops keeping
#    its tools directory in a TOOLS variable, every answer below silently
#    becomes "invoked: none" -- which would read as nine simultaneous
#    regressions rather than as a broken instrument. So the shape is asserted
#    first and its absence REFUSES.
grep -q '^TOOLS=' "$RUNNER" || die "the runner no longer defines TOOLS= -- the invocation needle here is anchored on \"\$TOOLS/<tool>\" and must be re-derived before any answer it gives means anything"
#    THE `_check` IN THIS NEEDLE WAS THE SAME BLINDNESS IN A SECOND PLACE, and
#    it is dropped here because hv's widening is what makes it bite: the roster
#    can now hold a `gated` row for a tool that is not named `*_check.sh`, and
#    the old needle would have reported it as invoked by nothing.
#
#    CALIBRATED ONCE, ON 2026-08-21 AT 510d4b10: narrow and wide both return the
#    same 10 names, so the widening provably does not change today's answer.
#    Measured independently by vc against the same runner. **Recorded here as a
#    one-time calibration rather than asserted at runtime, and that is
#    deliberate** -- the narrow pattern is a strict specialization of the wide
#    one, so a superset assertion holds BY CONSTRUCTION and could never fire. A
#    check that cannot fail is a vacuous green, which is the class this whole
#    directory exists to catch.
INVOKED="$(grep -v '^[[:space:]]*#' "$RUNNER" | grep -o '\$TOOLS/[A-Za-z0-9_]*\.sh' | sed 's|^\$TOOLS/||' | sort -u)"
[ -n "$INVOKED" ] || die "the runner invokes no parity .sh at all through \$TOOLS -- ten are known to be wired, so this is the needle failing and not the gate emptying"

findings=""
add() { findings="${findings}  $1
"; }

has() { printf '%s\n' "$2" | grep -qx -- "$1"; }

# ---------------------------------------------------------------------------
# A. Every tool on disk is rostered, and every rostered tool is on disk.
# ---------------------------------------------------------------------------
while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$ROSTERED" || add "$t exists in a parity tools directory and has NO roster row -- declare it gated, manual or not-an-instrument, with a reason"
done <<EOF
$PRESENT
EOF

while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$PRESENT" || add "$t has a roster row and NO file -- the roster has outlived the instrument"
done <<EOF
$ROSTERED
EOF

# ---------------------------------------------------------------------------
# B. Each disposition matches what the runner actually does.
#
# `gated` is TWO claims, checked separately, because the guard-0 rot recorded in
# the runner's header was precisely their disagreement: named in the roster,
# implemented inline, and `int hooks` reported a three-guard gate as two.
# ---------------------------------------------------------------------------
gated_n=0
manual_n=0
notinstr_n=0
while read -r name disp reason; do
  [ -n "$name" ] || continue
  case "$name" in \#*) continue ;; esac

  # Only judge rows whose file exists; a ghost row was already reported above
  # and would otherwise be counted twice under two different descriptions.
  has "$name" "$PRESENT" || continue

  case "$disp" in
    gated)
      gated_n=$((gated_n + 1))
      [ -n "$reason" ] || add "$name is rostered GATED with no reason -- hv's 2026-08-21 ruling requires a reason on every kind, and a gated row without one cannot be re-timed or re-argued by the next reader"
      has "$name" "$DECLARED" || add "$name is rostered GATED but the runner does not name it in --list-guards -- a guard the gate runs and does not declare is invisible to \`int hooks\`"
      has "$name" "$INVOKED" || add "$name is rostered GATED but nothing in the runner's body invokes it -- this is the 0059 defect itself, in the roster meant to prevent it"
      ;;
    manual)
      manual_n=$((manual_n + 1))
      [ -n "$reason" ] || add "$name is rostered MANUAL with no reason -- an unlabelled instrument wearing a label"
      has "$name" "$INVOKED" && add "$name is rostered MANUAL and the runner invokes it -- the roster is wrong in the direction that reads as safe"
      has "$name" "$DECLARED" && add "$name is rostered MANUAL and the runner declares it in --list-guards -- \`int hooks\` is reporting a guard that does not run"
      ;;
    not-an-instrument)
      notinstr_n=$((notinstr_n + 1))
      # THE REASON IS THE WHOLE CONTROL ON THIS KIND. `gated` and `manual` are
      # both checked against the runner below, so a wrong one is caught by
      # machinery. `not-an-instrument` REMOVES the file from every such check,
      # so nothing downstream can ever contradict it -- the only thing standing
      # between a mislabel and permanent invisibility is a human reading the
      # reason in the diff that adds it.
      [ -n "$reason" ] || add "$name is rostered NOT-AN-INSTRUMENT with no reason -- this kind removes the file from every check below, so an unreasoned one is how this guard goes blind with a signature on it"
      has "$name" "$INVOKED" && add "$name is rostered NOT-AN-INSTRUMENT and the runner INVOKES it -- either it is an instrument and the row is a mislabel, or the gate is running something that forms no verdict"
      has "$name" "$DECLARED" && add "$name is rostered NOT-AN-INSTRUMENT and the runner declares it in --list-guards -- \`int hooks\` is reporting a guard that is not a checker"
      ;;
    *)
      add "$name has disposition '$disp', which is none of gated, manual or not-an-instrument"
      ;;
  esac
done <<EOF
$ROSTER
EOF

# ---------------------------------------------------------------------------
# D. An INSTRUMENT ships executable.
#
# **THIS FILE HELD THE CONVENTION IN PROSE AND DID NOT CHECK IT.** Four rows
# below read *sourced, not executed; ships 644 and defines functions only* --
# so 644 versus 755 is how this roster tells a library from an instrument, in
# its own words, with nothing watching the join. That is the class this whole
# file exists to close, sitting inside it.
#
# **MEASURED BEFORE IT WAS ASSERTED: 35 of 35. Every `gated` (11) and every
# `manual` (24) is 100755 in the index, no exceptions.**
#
# **`not-an-instrument` IS DELIBERATELY NOT CHECKED, and the measurement is why:
# 4 are 644 and 13 are 755.** Sourced libraries ship 644; generators and
# extractors are run directly and ship 755. There is no invariant there, and
# inventing one to make the check symmetrical would be an unearned claim in a
# file whose subject is unearned claims.
#
# # It cost two commits in one day, by two authors, and the gate stayed green
#
# `runner_roster_check.sh` itself went 755 -> 644 at `d8dd6dc6` (cc) and again
# at `19d77f61` (dc), **with a repair commit sitting between them that restored
# the state and left the mechanism** -- a fix indistinguishable from one that
# worked until the next occurrence. Nothing failed either time: the runner
# invokes it as `bash "$TOOLS/..."`, so a 644 instrument runs and the gate
# prints green forever. **What regressed was MEANING -- at 644 the roster
# classifies itself, by its own convention, as a library.**
#
# **THE MECHANISM IS ESTABLISHED ONCE AND INFERRED ONCE, AND THIS CHECK DOES
# NOT DEPEND ON IT.** dc drove six idioms (2026-08-21): the mode survives when
# the INODE survives. `write_text`, `open(w)` and `fileinput` truncate in place
# and PRESERVE it; `sed > tmp && mv`, `open(tmp) + os.replace` and `mkstemp +
# os.replace` create a new file at the umask and rename it over the old, so the
# original mode is never consulted -- 644, or 600 from `mkstemp`. dc's drop is
# `sed > tmp && mv`, tested. **cc's is NOT established**: cc's own record blamed
# `write_text`, and driving it refuted that, so the cause of `d8dd6dc6` is
# unknown. Two occurrences, one mechanism -- which is why this checks the
# SYMPTOM in the committed state and says nothing about how a file got there.
# ---------------------------------------------------------------------------
# `read` splits the two fields itself, so nothing here relies on an unquoted
# expansion -- the shell critic refuses `set -- $row` (IN-SH-CODE-001) and is
# right to: three sites in this repo carry deliberate word-splitting and each
# one is a place a future reader has to be told not to quote. One fewer.
while IFS=' ' read -r name disp _rest; do
  [ -n "$name" ] || continue
  case "$disp" in
    gated | manual) ;;
    *) continue ;;
  esac
  mode="$(printf '%s\n' "$MODES" | awk -v n="$name" '$1 == n { print $2 }')"
  [ -n "$mode" ] || continue
  [ "$mode" = "100755" ] || add "$name is rostered $disp and the INDEX records it $mode -- an instrument ships executable, and at 644 this roster classifies it as a library by its own convention. Nothing else catches this: the runner invokes tools with \`bash\`, so a 644 instrument runs and the gate stays green"
done <<EOF
$(printf '%s\n' "$ROSTER" | awk 'NF && $1 !~ /^#/ { print $1, $2 }')
EOF

# ---------------------------------------------------------------------------
# C. The runner does not run anything the roster has never heard of.
# ---------------------------------------------------------------------------
while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$ROSTERED" || add "the runner invokes $t and the roster has no row for it"
done <<EOF
$INVOKED
EOF

total="$(printf '%s\n' "$PRESENT" | grep -c .)"

if [ -n "$findings" ]; then
  printf 'roster: %s parity file(s) in this commit; %s gated, %s manual, %s not-an-instrument; the roster and the runner DISAGREE\n' \
    "$total" "$gated_n" "$manual_n" "$notinstr_n"
  printf '%s' "$findings"
  printf '  the roster is in %s -- fix the row or fix the runner, whichever is lying.\n' \
    "intent/st/ST0056/parity/tools/runner_roster_check.sh"
  exit 1
fi

printf 'roster: %s parity file(s) in this commit, all rostered; %s gated, %s manual, %s not-an-instrument; every disposition matches the runner\n' \
  "$total" "$gated_n" "$manual_n" "$notinstr_n"
exit 0
