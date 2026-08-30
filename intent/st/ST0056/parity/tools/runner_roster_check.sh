#!/bin/bash
# runner_roster_check.sh -- every parity instrument declares whether anything runs it.
#
# THE JOIN. MODULES.md records what a tool IS and what posture its own header
# claims; `precommit` records what the GATE runs. Both can be perfectly accurate
# while "does anything run this?" is recorded nowhere. Issue 0059: eight of
# eleven instruments had no execution site, every individual record correct.
# This file adds the missing fact -- a DISPOSITION -- and measures it against
# the runner instead of trusting it.
#
# IT ENUMERATES THE DIRECTORY EVERY RUN AND REFUSES ON ANY FILE IT HAS NO ROW
# FOR. Never a remembered count: a population silently acquiring a member is the
# failure this check has to survive, and it did acquire one between 0059's
# census and this remedy. The roster fails on the day a tool is added, which is
# the only day anybody can classify it.
#
# IT ASKS THE RUNNER RATHER THAN REIMPLEMENTING ITS RULE. Deriving a guard
# roster by grepping the runner's source anchored on a path shape under-reported
# a three-guard gate as two. `--list-guards` exists so a reader can ask.
#
# DECLARED AND INVOKED ARE MEASURED SEPARATELY, because their disagreement IS
# the rot: a guard named in `--list-guards` and implemented nowhere, or invoked
# in the body and named in no roster. Both have happened here; one measurement
# sees neither.
#
# Exit codes: 0 clean, 1 a finding, 2 cannot measure. THIS ONE GATES, unlike its
# report-only siblings, and the difference is principled. Report-only checks are
# report-only because most hits are a legitimate mid-ladder state. There is no
# legitimate state in which a tool exists and its disposition is undeclared --
# the fix is one line below, and the point is to force it when the tool arrives.

# `-e` IS DELIBERATELY OFF and that is recorded rather than left to be inferred
# (critic-shell asked for the rationale, correctly). This is a findings
# COLLECTOR: it probes, calls `add`, and keeps going so one run reports every
# disagreement. Under `errexit` the first non-zero probe would abort and the
# roster would report ONE finding and exit, which reads as a smaller problem
# than it has.
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
# THE ROSTER. One row per `.sh` in EITHER thread's tools directory:
# <name> <gated|manual|not-an-instrument> <reason>.
#
# `gated`  -- the repo-local pre-commit gate runs it, bare, on every commit.
# `manual` -- it is an instrument and the gate does not run it; the reason says why.
# `not-an-instrument` -- not a checker: a sourced library, generator, extractor,
#             capture driver, stub-to-be-driven, or a transformer of a tree.
#
# A REASON IS REQUIRED ON EVERY KIND, INCLUDING `not-an-instrument` (hv, ruled
# 2026-08-21). A bare label costs nothing to write, so a genuine instrument can
# be declared out of the population by anyone who finds this guard inconvenient
# -- and the guard goes blind WITH A SIGNATURE ON IT, which reads as a decision
# somebody made rather than as a gap. The reason makes the mislabel expensive to
# write and visible in the diff that adds it.
#
# WHAT COUNTS AS AN INSTRUMENT: it ANSWERS A QUESTION ABOUT THE ESTATE AND
# RETURNS A VERDICT. THE MECHANICAL DISCRIMINATOR IS vc's: **does the file's own
# header cite an AT or AC row it covers?** If it does it is definitionally an
# instrument -- its verdict gates a criterion -- and the classification is
# asserted by the author at the site rather than guessed here. Driven over the
# 33 files hv's widening admitted: 12 cite, 21 do not.
#
# EXIT-1 PRESENCE MISFIRES AS A DISCRIMINATOR AND THE NEXT PERSON WILL REACH FOR
# IT. `gen_dispatch_table.sh` and `gen_pertest.sh` both carry `exit 1` and are
# generators -- a refusal to EMIT on bad input, never a finding about the estate.
# The citation test corrected three of my own classifications, all of which
# would have been silently removed from the population.
#
# EVERY ST0057 ROW IS STRUCTURALLY `manual` AND NONE OF THEM IS A JUDGEMENT
# ABOUT THAT TOOL. `cmd/precommit:116` pins `TOOLS` to ST0056's directory, so
# the runner cannot invoke an ST0057 instrument at all and a `gated` row would
# correctly fail check B. Making one gateable needs a second tools path in
# `cmd/precommit`, which is dc's. Seven `manual` rows are one structural fact,
# not seven rulings.
#
# UNDER UNCERTAINTY THE ROW GOES `manual`, NEVER `not-an-instrument` (vc). A
# wrong `manual` costs a reason, stays visible, and anyone can correct it; a
# wrong `not-an-instrument` removes the file from the population, so it
# generates no signal ever again and nothing surfaces the mistake. Fail loud.
#
# A REASON THAT CHURNS SHOULD NOT BE LOAD-BEARING. One row's reason moved three
# times in a day; it now rests on the structural fact (`cmd/precommit:116`),
# which has never moved, with the volatile detail recorded beside it as history.
#
# AND RE-DERIVE THE SUBJECT THE CLAIM IS ABOUT, not merely re-derive. A true
# measurement of a different property is the most persuasive wrong evidence
# there is, precisely because the measuring was real and careful.
#
# `FIXED` IS NOT A STATE (vc). Worktree, index, HEAD and pushed are four. Every
# node here holds dirty files and commits only on hv's word, so a peer saying "I
# fixed it" reports the FIRST while the reader hears the THIRD -- the default
# condition, not an edge case. A commit citation pointing the wrong way does not
# fail the next checker, it CONFIRMS to them that the fix is in.
#
# EVERY TIMING BELOW IS MEASURED AND NAMES WHAT IT MEASURES. RE-TIME BEFORE
# MOVING A ROW ON COST GROUNDS, and record the span. Bare figures invite
# comparing a fresh measurement against a recorded one taken on another machine
# over a smaller tree, which is not a comparison at all. Stale figures were
# wrong in BOTH directions, so no correction factor recovers them.
# METHOD: /usr/bin/time -p wall clock (not the shell builtin over a subshell,
# which under-reported by roughly half), 5 runs each, min-max, darwin/arm64,
# over 8f652d1b..08bed4b2. One sub-figure is left attributed to its original
# author: the 263ms dispatcher component of runner_roster_check.sh.
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
canon_commit_check.sh          gated             ADMITTED 2026-08-21 by dc on the hv release, both holding reasons RE-DERIVED rather than taken on report: sync --to-store takes positional ids, so the only order a gate permits (sync the one thread, then commit file and canon together) is open, and --staged is supported and judges the INDEX. Dispatched ALWAYS --staged and NEVER path-triggered -- the tool narrows internally and a path trigger would be a second copy of that narrowing. NO APOSTROPHES IN THIS TABLE: ROSTER is a single-quoted shell string, so one of that character terminates it and breaks the parse. This row did exactly that when first written, twice, the second time inside the sentence warning about it. LAYOUT-AGNOSTIC: it detects the canon layout per revision, REFUSES a half-migrated tree rather than counting one half, and READS THE THREAD ID FROM CANON CONTENT RATHER THAN FROM THE PATH -- because a path strip whose pattern is absent returns the string UNCHANGED at rc=0 and looks entirely plausible. ~1.8x SLOWER ON PURPOSE: scoped used to count narrowing FILTER KEYS rather than attachments EXAMINED, which printed EXAMINED 2 of 1 with the other -1. COST 2710-2760ms narrowed and 11.3-11.5s --exhaustive ON ONE MACHINE -- the most expensive guard here, moving the gate total from ~4.6s to ~7.3s on every commit. Reads git only: no worktree, no binary, no clock. Driven five ways across both layouts plus nine real commits, and it caught an unplanted divergence on its first whole-tree run
provenance_fields_check.sh     manual            new 2026-08-21 (dc), covering AC-11.7 via AT-11.7. MANUAL WITH A NAMED RELEASE CONDITION BECAUSE ITS SUBJECT FAILS TODAY: dist-provenance.txt carries a bare commit and no artefact hash, so gating now delivers a PERMANENTLY-RED gate. RELEASE CONDITION: promote to gated once int macos stage emits the labelled partition (artefact_sha256 answering IDENTITY, source commit answering CURRENCY, drift HELD with its release condition); the writer fix is bin/.devbin/cmd/macos, dc lane, unstarted. TWO ARMS, DIFFERENT SUBJECTS, REPORTED SEPARATELY: FIELDS asks whether the record is well-formed, SET asks whether the record describes the BYTES beside it and whether those bytes agree with EACH OTHER. A PER-RECORD CHECK CANNOT SEE A PROPERTY OF THE SET -- found on real bytes, two binaries naming two different commits beside a record naming a third, every file individually well-formed. NOT A COPY of artefact_commit_blockers in cmd/macos: that pivots on the TAG at release time only, this pivots on the RECORD at any time. COST 53-74ms across two sittings, strings over a 9.5MB and a 373KB binary, timed in ONE python3 process with perf_counter around subprocess.run, harness floor 4.9ms, at loadavg 28 over 16 cores. A RANGE ACROSS SITTINGS RATHER THAN A POINT, because the point moved 15 percent in twenty minutes with nothing about the tool changing. AN EARLIER 82-88ms IS WITHDRAWN AS AN ARTEFACT OF ITS OWN HARNESS: it spawned two clock processes per iteration, so roughly 50ms was the instrument measuring itself. AND A FIGURE NAMING A REVISION AND A MACHINE BUT NO LOAD STATE IS STILL UNCOMPARABLE WHEN FOUR SESSIONS SHARE THE MACHINE ALL DAY (cc) -- a defect in every timing row here, and only naming the condition fixes it. SELF-TEST drives SEVEN controls, each shown able to fire alone, including an empty set that must report NOT EXAMINED rather than pass silently. REACH: FIELDS checks fields EXIST and are LABELLED, SET checks that record and artefacts name ONE commit between them, and NEITHER can check that an embedded marker is HONEST about the bytes carrying it -- that is the drift field and drift is held
guard_home_check.sh            gated             new 2026-08-22 (dc), closing the exposure under an hv guard-resolution directive WHOSE PREMISE ic FALSIFIED: the guards do NOT resolve out of the frozen v2 checkout, because pre-commit.sh overrides GUARD_HOME to the repo root when the repo is itself an Intent install. hv asked for a mechanism rather than a variable and THE MECHANISM ALREADY EXISTED WITH NOTHING WATCHING IT. GATES THE TRACKED TEMPLATE, NEVER THE INSTALLED COPY: pre-commit.intent is gitignored by design so a fresh clone has none, and a check keyed to it would fail in every clone. BYTE-IDENTITY WAS THE CANARY OFFERED AND IT IS THE WRONG ONE -- an ACTIVE tree and a FROZEN one are SUPPOSED to diverge, so that check reddens on the first legitimate guard edit and is cry-wolf by construction. Two arms asserted separately because they fail differently: a missing condition means the override can never fire, a missing assignment means it fires and does nothing, and one combined grep would send the reader at the wrong half. Mutation-proven four ways. COST 17-22ms, n=9, median 19, harness floor 5ms, at loadavg 47.7 over 16 cores -- the cheapest gated instrument here. Two greps over one committed file: no git, no binary, no clock. REACH: it checks the override EXISTS and assigns the repo root; it does not run the hook, does not compare guard bodies across trees, and cannot tell whether the branch CONDITION is correct for any particular install
conservation_check.sh      manual  takes a MIGRATED tree as an argument and no such tree exists until WP-10 lands, so there is no bare invocation for a gate to make; it refuses with exit 2 rather than passing when handed an unmigrated one, which is the behaviour a gate would have to bypass on every commit
thread_view_skew_check.sh      gated             ADMITTED 2026-08-27 by dc -- cc built it, dc rosters. RE-MEASURED FOR THE PROMOTION RATHER THAN PROMOTED ON THE 2026-08-20 FIGURE: 182-212ms, n=15, median 188, harness floor ~31ms so net ~157ms, at loadavg 17.9 over 16 cores, over 288 views (the row below says 268; the population grew). NO SECOND MODE IN 15 RUNS, so the 3x tail recorded below remains unreproduced and the promotion does not rest on it -- which is the standard the row itself set. DISPATCHED --changed WITH THE STAGED PATHS, like its sibling, and the flag narrows WHAT BLOCKS rather than what is checked: every view is examined either way, and without it a stale view no node wrote would stop all five. THE FAIL-OPEN IS RETIRED 2026-08-27 UNDER vc PEN (authority vc, hv AFK): an absent binary now REFUSES at exit 2. The old arm named the gap and exited 0, which was the defect rather than the mitigation -- exit 0 is the SAME STATUS a run examining every view returns, so no caller reading the exit code could tell 288 views checked from 0 checked, and the warning text was the only carrier of the difference. That is IN-AG-NO-SILENT-001 rather than a preference. TWO REFUSAL STATES, SEPARATED FOR THE BLOCKED READER RATHER THAN FOR THE GUARD: a release directory present means a build has run in this tree, so the absence is a REBUILD IN PROGRESS and the message says TRANSIENT, wait about 60 seconds and retry; no release directory means never built and waiting will not help. Both name the remedy (bin/devbin build all). A refusal that reads like a defect gets worked around, and a guard that gets worked around is worth less than the silent one it replaced. DRIVEN THREE ARMS AT FIXTURE DEPTH plus the real tree: binary present rc=0 at 288 views, release dir without binary rc=2 transient, no release dir rc=2 not transient. DRIVEN TWO-SIDED AT FIXTURE DEPTH, because ROOT is NOT overridable in this tool -- an env-var drive silently runs against the real tree and reports a success that measured nothing, which is how the first attempt at this evidence went. The stated population (a machine with no v3 build) is not the one that occurs here: every machine carries a build and the real absent window is a REBUILD, where cargo removes the artefact before writing. Routed to hv with the same ordering question cc raised on the critic gate; this is the third instrument on that one ruling. ONE STALE PREMISE LEFT IN PLACE DELIBERATELY: the comment justifies absolute-path resolution by v3 being off PATH, a ruling retired 2026-08-22. The behaviour stays correct for a better reason (it names THIS tree binary rather than whatever PATH resolves), so the fix is a reword and belongs to whoever next edits that file. COVERS THE POPULATION THE SIBLING NEVER DID: the CHECKABLE in view_skew_check.sh is ONE triple under surface/, so gated skew coverage is 1 of 269 and the missing 268 are thread covers, acceptance contracts and WP covers. Forms no verdict -- views::skew is the single home for the question and this parses one answer rather than computing a second. UNGATEABLE BEFORE b082b488, which is why it is new rather than late: doctor reported every dehydrated view as MISSING, 235 findings at rc=1 on a healthy tree. COST 142-175ms, n=15, mode ~151, harness floor 5.7ms, at loadavg 32.4 over 16 cores, over 268 views -- it would be the cheapest gated instrument here and ~22x faster than view_skew_check.sh. A 3x TAIL WAS OBSERVED ONCE IN 22 RUNS, HAS NOT RECURRED, AND IS RECORDED AS AN OBSERVATION RATHER THAN AS A PROPERTY: one sitting read 160 163 165 167 170 188 513, two nodes reasoned from it that the distribution is bimodal under contention and that a gate pays the tail rather than the median -- a rule that would DECIDE DIFFERENTLY about promotion -- and fifteen runs at HIGHER load produced no second mode. THE AGREEMENT IS WHAT STOPPED EITHER OF THEM RE-RUNNING IT. A promote-to-gated argument does not get to rest on a number nobody can reproduce. Refuses at exit 2 rather than passing when it cannot read the summary line, because a text-reading gate whose needle stops matching goes green forever and nothing says so. Driven ten arms, including a MENTION/SUBJECT pair carrying identical decoy lines to opposite verdicts. Needs the v3 binary at native/rust/target/release/intent; absent, it REFUSES at exit 2 and distinguishes a transient rebuild window from a tree that was never built
machine_table_check.sh     gated   210-240ms, re-measured at 021ba259 after collapsing axis C from a shell function per row (620-720ms, about ten processes per row) to one awk per machine -- METHOD /usr/bin/time -p, 5 runs, min-max, darwin/arm64. Reads TWO COMMITTED FILES from the index and nothing else: no worktree, no binary, no clock. WHOLE-SET AND NEVER PATH-TRIGGERED, because the failure it exists for is committing ONE side of a two-file fact, so a path trigger would have to fire on the path that is not there. GATES ON AXIS A (entry states against initial:) AND AXIS B (expanded from-to-verb edges), both a controlled vocabulary on both sides. AXIS C (guards) REPORTS AND NEVER GATES: the Guard column of the ratified document is free prose, and in Machine 3 it holds effects and landing rules rather than preconditions, so 6 of 37 rows are UNMEASURED -- printed as loudly as a finding rather than defaulted to clean, which would let a genuinely missing guard read as green. Driven to twelve arms with every plant verified to have landed, including the real drift it was built for (hv fiat ruling in the code, absent from the table) and a negative control (an UNTABLED field mutating stays silent); the one-table-header arm found a defect in it before it landed
drift_check.sh             manual  compares a STAMPED inventory against live canon, so gating it would block a dispatch-table edit until somebody re-runs a 27-family measurement sweep -- a measurement, not a fix
guide_refs_check.sh        manual  takes required prose-file arguments, so there is no bare invocation for a gate to make
instrument_reach_census.sh manual  new 2026-08-30 (vc), covering AC-00.16 via AT-00.20. MANUAL BECAUSE IT EXITS 1 BY DESIGN: the row is RED and the instrument says so in its exit code, so a gated row would block every commit in the estate on a criterion nobody has satisfied yet. It measures ONE PRECONDITION -- that an instrument declares what it does NOT cover -- and reports 40 of 216 Rust integration-test files doing so. Declaring reach is what makes the union computable and is NOT the union, which is what AC-00.16 asks for. RELEASE CONDITION: promote to gated when something owns the union and this can assert a property rather than report a census. ITS OWN REACH IS PRINTED IN ITS OUTPUT RATHER THAN DESCRIBED HERE, because it is subject to the class it measures: it reads native/rust/crates/*/tests/*.rs and NOT the BATS suite, the shell instruments beside it, lib/templates/hooks, the critic rule library, or colocated cfg(test) modules -- and the vc instance of this class was a shipped SHELL surface sitting outside the reach of a Rust scanner. An instrument measuring declared reach while hiding its own would be the fourth instance. PARTITION CLOSES IN THE OUTPUT and the script refuses with exit 2 if it does not, per AC-00.11 and AC-00.12.
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
gen_cut_surface.sh             not-an-instrument generator; emits docs/reference/cut-surface.md. It REPORTS a difference between two revisions and never judges the estate, so its exit 1 is a refusal to emit -- an unresolvable rev, or a rev carrying no register -- and never a finding
gen_dispatch_table.sh          not-an-instrument generator; its exit 1 is a refusal to EMIT on bad input and never a finding about the estate, which is exactly why exit-1 presence cannot classify this table
gen_inventory.sh               not-an-instrument generator; emits the parity inventory that generator_inputs_check.sh then audits
gen_pertest.sh                 not-an-instrument generator; its exit 1 is a usage refusal, not a finding
gen_reference.sh               not-an-instrument generator; emits the per-family command reference under docs/reference/ plus its index. It REPORTS what the register declares at a revision and never judges the estate, so its exit 1 is a refusal to emit -- an unresolvable rev, or a rev carrying no register -- and never a finding. THE ONE THING IT DELIBERATELY WILL NOT DO: publish observed.exit as the v3 exit codes. That field is v2 behaviour measured on v2, and on 28 rows it is the v2 DEFECT, so reprinting it would document in the voice of a reference manual the thing the rewrite exists to remove. Exit codes are emitted only where target.state warrants the claim -- 61 of 118 shipped rows -- and each page states the shortfall as register coverage rather than quietly omitting a section. IDEMPOTENT THROUGH THE FORMATTER, DRIVEN RATHER THAN ASSERTED: prettier rewrote all 16 pages before the fix and rewrites none after, so the control flips. It escapes < outside code spans, because a raw <name> is parsed as an HTML tag and renders as nothing -- silent text loss with no other symptom. AND IT DOES NOT ATTRIBUTE REFUSALS TO COMMANDS: the register carries no refusal field, so a per-command refusal list could only be guessed from variant names, and a guess dressed as a reference is worse than the stated gap
gen_register.sh                not-an-instrument generator; emits the test register
intentfiles_reviewable.sh      manual            ST0057; needs a realisation change already in the working tree to review, so there is no bare invocation for a gate to make
interrupt_rig.sh               manual            produces the two trees the cutover gate compares by SIGKILLing a migration mid-run; it DOES form a verdict, but it takes minutes and needs a migrated estate
lib_classify.sh                not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_corpus.sh                  not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_mdfmt.sh                   not-an-instrument sourced, not executed; ships 644 and defines functions only
lib_staged.sh                  not-an-instrument sourced, not executed; ships 644 and defines functions only. THE ONE HOME for reading a gating input from the INDEX rather than the working tree. Converged from four callers under an hv Highlander ruling (issue 0125), after the same defect was found and fixed IN PLACE three times in eleven days by three different nodes -- runner_roster_check.sh and residue_class_check.sh on 2026-08-17, then corrected_check.sh, class_vocab_check.sh and rulings_check.sh on 2026-08-28, where one mid-edit dispatch-table row refused a peer commit on a file that peer had never touched. hv declined fix-the-three-leave-the-fourth: two homes for this mechanism is what produced the second episode. REACH LIMIT, stated because the name invites the opposite assumption: it handles FILES, since git show of a directory is not a thing, so the two issue-estate directory scans in rulings_check.sh still read the working tree. AND IT DOES NOT CLOSE THE CLASS -- nothing makes a new instrument source it, which is the lesson lib_surface.sh already recorded in its own header after being built as the one home and sitting beside two callers that never sourced it. Whether the gate should REFUSE an instrument that reads a gating path off disk is a roster question rather than a library one, and it is raised with vc rather than answered in the library
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
shared_artefact_build_guard.sh gated             new 2026-08-25 (dc), covering AC-11.6 via AT-11.6. GUARDS THE SHARED BUILD PATH: native/rust/target/release/ is shared by every node with no per-node isolation, and until now the only control over what entered it was the convention do not rebuild while I am measuring. THE FOUNDING EPISODE IS REAL AND COST THE WHOLE ESTATE: a build at 19:33Z on 2026-08-18 from a tree carrying uncommitted WP-01 source produced a working-looking binary that emptied the store for every node until rebuilt. GATED ON DAY ONE RATHER THAN HELD, and the precedent is guard_home_check.sh not provenance_fields_check.sh: that row went manual because its SUBJECT fails today so gating would deliver a permanently-red gate, and this subject is green on the live tree because every arm runs in a disposable fixture. It builds nothing, and it reads and writes nothing under target/. ARMS 1-6 ARE DYNAMIC, 7-8 ARE STATIC, AND THE SPLIT IS THE REACH LIMIT: 1-6 drive the predicate in scratch repos (dirty refuses and names the path, untracked refuses, clean passes, dirt OUTSIDE native/rust does NOT refuse, unreachable git refuses rather than reading as clean, and the guard scope equals the markers DIRT_SCOPE); 7-8 READ THE SOURCE of cmd/local to establish that the verdict is taken BEFORE cargo and that a refusal redirects rather than blocks. ORDER IS THE CRITERION AND ONLY AN ORDER ARM SEES IT: verify_pair already refused a dirty marker before any of this existed, but ran AFTER cargo had overwritten the shared binaries, so it named the damage instead of preventing it. ARM 7 FAILED ITS OWN FIRST DRAFT AGAINST CORRECT CODE by grepping the whole file and matching the header sentence WHY build IS NOT cargo build --release -- prose about an invocation read as one -- so it is now scoped to the cmd_build body with comments stripped, which also closes the opposite direction where a comment mentioning the verdict would have passed the arm while the call site was late or absent. MUTATION-PROVEN FOUR WAYS, EACH REDDING A DIFFERENT ARM SET: verdict moved after cargo reds 7 alone; scope diverged from DIRT_SCOPE reds 6 alone; predicate forced to answer ok reds 1, 2 and 5 while 3 STAYS GREEN, which is what says the arms test different things; redirect removed reds 8 alone. COST 285-411ms, n=12, median 319, harness floor 1.8ms, at loadavg 33.4 over 16 cores -- mid-pack among gated instruments and cheaper than self_provenance_check.sh, runner_roster_check.sh and view_skew_check.sh. The cost is five scratch git repos, which is also why it is not cheaper. REACH: THE REDIRECT HAS NEVER EXECUTED. Arms 7 and 8 establish that cmd/local CONTAINS the ordering and the redirect; no arm runs int local build on a dirty tree to confirm the private target dir is actually written and the shared one actually is not. That needs a real cargo build and is the one claim here taken from the source rather than from behaviour
staged_reads_check.sh          manual            new 2026-08-29 (cc), the enforcement half of issue 0125 -- vc ruled BUILD IT and declined leaving it as a documented limit, on the ground that a header comment relies on every future instrument author reading it, which is exactly what failed for lib_surface.sh. ASKS WHETHER EVERY GATED INSTRUMENT READS ITS GATING INPUT FROM THE INDEX. lib_staged.sh is the one home for that and a sourced library closes the class only for the callers that source it; this is the thing that makes them. MANUAL RATHER THAN GATED FOR THE provenance_fields_check.sh REASON AND NOT FOR TIMIDITY: its subject fails today, 7 of 14 gated instruments name a repo path and route none of it through staged_copy, so gating now delivers a permanently-red gate and a guard that must be bypassed to commit is one step from a guard nobody keeps. RELEASE CONDITION: promote to gated once every instrument it names is either converged onto staged_copy or carries a one-line exemption saying why its read cannot mislead a peer -- a directory scan, an untracked build artefact, a path named but never opened are all legitimate, and the exemption is the record that somebody DECIDED. PLACEMENT IS AN OPEN QUESTION FOR dc: vc described this as a roster arm and it is built as a separate instrument instead, because the roster asks whether every tool declares a disposition and this asks how a tool reads -- two concerns -- but the roster is dc territory and the call is dc. REPORTS, NEVER GATES: exit 0 clean or with findings, non-zero only when it cannot measure. REACH, and the name oversells it: SYNTACTIC, so it cannot tell a load-bearing read from an incidental one, cannot see a path built any other way than an assignment reaching under the repo root, and cannot see a DIRECTORY scan because git show of a directory is not a thing. The idiom it does see is the one all four instruments in 0125 used. A BEHAVIOURAL VERSION WOULD BE STRICTLY BETTER AND THIS IS NOT ONE -- an attempt at one was made and is not reported as evidence anywhere, because the STAGED control did not flip either, so the perturbations were comment appends no instrument asserts on and the probe was inert. DRIVEN FIVE ARMS ON PLANTED FIXTURES, to every verdict it has: a defective tool is named, a compliant one passes ALONE rather than only in company, an exempted one is skipped, a bare exempt marker with no reason does NOT silence it, an EMPTY gated population refuses rather than reporting a clean estate, and a roster naming absent tools refuses rather than reading as clean
sparse_tree_equals_manifest.sh manual            ST0057; runs organize and compares the tree against .intentfiles, so it mutates a tree and has no bare gate-safe invocation
sync_issue_loss.sh             manual            ST0056; the REPRO for the intent#0069 attribution -- asks whether a thread-scoped sync --to-store destroys ISSUES. It does NOT (3 to 3, both v3 binaries), which is what exonerated sync as the cause of the 2026-08-24 loss. MANUAL because it stands up a throwaway project and runs a mutating verb in it, so there is no gate-safe bare invocation. KEPT RATHER THAN DELETED AFTER A NEGATIVE RESULT: a no-change result is a result, and the next reader who suspects sync should re-run this rather than re-derive it
upgrade_issue_loss.sh          manual            ST0056; the REPRO for intent#0070 -- intent upgrade destroys every issue in an ALREADY-MIGRATED v3 project. TWO ARMS, BOTH v3 BINARIES, against a synthetic project made by the v3 init verb: issues 5 to 0, threads intact. Fix landed at 3f367cf8 with this script as the red arm, and intent#0070 is closed. CORRECTED 2026-08-24: THIS ROW ONCE SAID the script now forces the real path, ABOUT A v2 ARM THE COMMITTED SCRIPT HAS NEVER CONTAINED -- one commit, two arm lines, zero v2 tokens, probe positive-controlled. The v2 finding was real and driven and it is what bounds the defect to v3, but the arm lived in a scratchpad and died with a compact, so THE ROW DESCRIBED THE INVESTIGATION WHILE CLAIMING TO DESCRIBE THE RUNNER. cc landed the missing population separately as v2_estate_issue_carry.sh. A ROW CARRYING ONE TRUE SENTENCE AND ONE FALSE ONE IS HARDER TO CATCH THAN A WHOLLY WRONG ROW, BECAUSE THE TRUE HALF IS WHAT A READER CHECKS FIRST AND IT HOLDS (cc) -- and THIS CHECKER IS BLIND TO IT, verifying row-to-file EXISTENCE both ways and nothing about whether a row DESCRIPTION matches its runner. ASSERTS ON COUNTS READ FROM THE STORE, NEVER ON sync REPORTING AGREE, or it inherits intent#0069 and the reporting defect hides this one. MANUAL because it mutates a throwaway project
v2_estate_issue_carry.sh       manual            ST0056; the NEGATIVE CONTROL for the intent#0070 fix and the arm that must never go red. upgrade_issue_loss.sh drives the positive arm -- an ALREADY-MIGRATED v3 project, where issues were destroyed and must now survive. This drives the other population: a REAL 2.19.0-built v2 estate migrated by v3, where issues were ALWAYS carried and must stay that way. The fix adds a union in migrate::plan that tops issues up from committed canon, and a FIRST migration has no canon to top up from, so the union must add NOTHING here -- a fix that made the already-migrated path work by changing what a first migration carries would pass the positive arm and be wrong. Only this arm can see that, which is why it is not a duplicate: the two populations do not overlap. IF IT EVER REPORTS ANYTHING BUT carried all N, the counts name which defect it is -- MORE than the estate held means the union is contributing where there is no canon, FEWER means legacy::scan has stopped reaching the v2 issue estate. Driven at both v3 binaries, pre-fix and post-fix, 5 of 5 both times with byte-identical output. ASSERTS ON COUNTS READ FROM THE STORE, never on sync reporting AGREE, for the same reason the positive arm does. MANUAL because it needs a v2 binary to BUILD the estate and then mutates a throwaway project, so there is no gate-safe bare invocation
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

# LATENT rather than live, and fixed anyway. `printf | grep -q` under
# `set -uo pipefail` is the idiom dc found in the two whiteboard guards: grep -q
# exits on the first match, printf takes SIGPIPE (141), and pipefail promotes it
# to the pipeline's status -- so a MATCH reads as FALSE. It cannot fire here,
# because SIGPIPE needs the write to outrun the 64KB pipe buffer and the largest
# `$2` in this file is ~927 bytes; printf always completes. The fix is taken on
# the same grounds dc took it in canon-ignore-guard.sh: a herestring is not a
# pipeline, so pipefail has nothing to promote, and the immunity is definitional
# rather than measured. NO TEST, deliberately -- an arm that fires only against
# a fabricated 64KB roster asserts about a population that does not exist.
#
# `-F` IS A SECOND FIX AND IS NAMED RATHER THAN SLIPPED IN. The old form let
# `$1` be a regex, so a `.` in a tool name matched any character; every call
# site means a literal filename comparison.
has() { grep -qxF -- "$1" <<<"$2"; }

# IS THE MISSING HALF STAGED RATHER THAN ABSENT? (discriminator: cc, 2026-08-29)
#
# `PRESENT` above is `git ls-files`, which reads whatever index it is handed.
# **Under a `git commit --only <paths>` hook that index is a TEMPORARY one
# holding HEAD plus the named paths** -- so a tool that is staged in the real
# index, but not named on that commit line, is simply absent from the tree being
# judged. The roster row (read from the WORKTREE) survives; the file vanishes;
# and the check correctly reports a severed two-file fact about a tree that
# exists only because somebody committed by path.
#
# **THE DETECTION WAS RIGHT AND THE ATTRIBUTION WAS WRONG, AND THE ATTRIBUTION
# IS THE ACTIONABLE HALF.** The old wording -- "the roster has outlived the
# instrument", "fix the row or fix the runner, whichever is lying" -- sends a
# reader to DELETE the roster row to get moving, which lands a real defect to
# clear a false one, in a file its author is mid-change on. Measured 2026-08-29:
# this refused two nodes' unrelated commits and both correctly declined both to
# touch the row and to reach for `--no-verify`. It cost them an hour between
# them and it will fire again on the next instrument anyone adds, because the
# triggering sequence is ordinary: stage the tool, edit the roster, and let any
# peer commit anything at all in between.
#
# **IT STILL GATES, AND THAT IS NOT A COMPROMISE.** The commit genuinely severs
# the pair; letting it through would land a roster row naming a file the commit
# does not carry. Only the sentence changes.
#
# `GIT_INDEX_FILE` is unset in a SUBSHELL so the ambient index is asked and the
# caller's environment is untouched -- every other read in this file must keep
# judging the tree being committed.
in_flight() {
  ( unset GIT_INDEX_FILE
    git -C "$ROOT" ls-files -- 'intent/st/*/parity/tools/*.sh' 2>/dev/null \
      | sed 's|.*/||' | grep -qxF -- "$1" )
}

# ---------------------------------------------------------------------------
# A. Every tool on disk is rostered, and every rostered tool is on disk.
# ---------------------------------------------------------------------------
inflight=0
while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$ROSTERED" || add "$t exists in a parity tools directory and has NO roster row -- declare it gated, manual or not-an-instrument, with a reason"
done <<EOF
$PRESENT
EOF

while IFS= read -r t; do
  [ -n "$t" ] || continue
  if has "$t" "$PRESENT"; then
    continue
  elif in_flight "$t"; then
    inflight=1
    add "$t has a roster row and its file is STAGED but not in the tree being committed -- the two halves are IN FLIGHT, not severed. DO NOT DELETE THE ROW. Land the tool and its roster row in ONE commit: \`git commit --only\` builds HEAD plus the named paths, so a path-scoped commit by ANY node drops a staged sibling it did not name. If the tool is not yours, its author is mid-landing -- wait."
  else
    add "$t has a roster row and NO file anywhere -- absent from the tree being committed AND unstaged. The roster has outlived the instrument."
  fi
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
  if [ "$inflight" -eq 1 ]; then
    printf '  AT LEAST ONE FINDING ABOVE IS AN IN-FLIGHT PAIR, NOT A LIE. Nothing needs correcting in the roster: a tool and its row are landing together and a path-scoped commit split them. Wait, or land the pair.\n'
  else
    printf '  the roster is in %s -- fix the row or fix the runner, whichever is lying.\n' \
      "intent/st/ST0056/parity/tools/runner_roster_check.sh"
  fi
  exit 1
fi

printf 'roster: %s parity file(s) in this commit, all rostered; %s gated, %s manual, %s not-an-instrument; every disposition matches the runner\n' \
  "$total" "$gated_n" "$manual_n" "$notinstr_n"
exit 0
