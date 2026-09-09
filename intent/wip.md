---
verblock: "08 Sep 2026:v1.36: vc - aggressive global fold: deleted this file's CORRECTION ARCHAEOLOGY, which is what done work looks like in a DOING-and-TODO file. Records of edits to this document are not state; the corrected state is. The cause is fixed too -- /in-finish ordered 'What was accomplished this session' into this file, and /in-essentials ordered state into the entry point that forbids it."
intent_version: 3.0.0
---

# Work In Progress

**DOING and TODO ONLY, NO EXCEPTIONS (hv, 2026-08-30). Completed work does not belong in this file** -- it lives in `done.md`, `intent/history/`, `CHANGELOG.md`, and the threads' own closed criteria. **A fifth copy here drifts from all four.** Pre-fold verbatim: `intent/.history/20260903/wip-preglobalfold-1731Z.md`.

**NO FIGURE IN THIS FILE IS EVIDENCE. RUN THE VERBS:**

    intent ac gate ST0056     # the release gate
    intent ac gate ST0058     # feeder
    intent ac gate ST0066     # feeder
    intent ac gate ST0068     # feeder -- the docs
    intent doctor

## DOING

### THE v3.0.1 GATE -- SIX ROWS CANNOT MOVE UNTIL hv CUTS, AND EVERY OTHER ROW IS ENGINEERING

**DO NOT PUT A COUNT IN THIS HEADING.** A total is a state; the verbs below are the only thing that can answer it. **A number printed beside its own regeneration command gets read INSTEAD of running the command**, so the two readers -- the one who obeys and the one who does not -- get different answers and the document rewards the lazier.

**WHAT IS DURABLE IS THE NAMED SET, NOT THE COUNT.** Six rows are unfalsifiable without a published artefact: `ST0056` AC-00.5, AC-07.7, AC-11.1, AC-11.4, AC-12.4 and `ST0068` AC-04.2. **Each re-verified still unsatisfied 2026-09-08 -- nothing regressed, the count was simply always short.** **None is engineering anybody is withholding: a cut is the instrument that measures them, and no node can measure them for hv.** Every row the gate reports beyond those six is engineering somebody can start.

**`ST0068` AC-03.1 + AC-03.2 ARE HELD ON AN INDEPENDENT READ AT LAKSA, AND THE PING IS vc's TO INITIATE.** It went out 2026-09-08, two days after the hold's own condition expired -- recorded because a hold that quietly outlives its condition is the failure the hold rules exist to prevent. `ST0056` AC-00.6's `bin/` coupling is discharged (the deletion is AT the cut and was never owed before the tag); `ST0068` AC-02.1 is satisfied; AC-02.3 carries its command and its declared scope limit.

**RUN IT. DO NOT TRANSCRIBE IT:**

    intent ac gate ST0056
    intent ac gate ST0068

### THE SHIPPED PAIR IS BEHIND THE TREE, AND THAT IS THE FIRST THING A COLD NODE SHOULD MEASURE

**Behaviours differ between this tree and the binary five sessions actually run.** Do not read a list of which -- the list rots the moment a sixth thing is fixed. **Read the distance:**

    intent --version                                              # the marker the BINARY was built from
    git rev-list --count <marker>..HEAD -- native/rust surface     # how far the tree has moved past it

**NO FIGURE HERE, AND WHAT THE COMMAND RETURNS IS AN UPPER BOUND RATHER THAN A DISTANCE (`0285`).** The marker is an ANCESTOR of the commit whose code is actually in the binary, so the count includes commits ALREADY COMPILED IN and can only OVERSTATE what is unbuilt. **An empty result is therefore conclusive and a non-empty one is not** -- for a non-empty answer the cargo fingerprint is the only instrument. Run the command; read it as a ceiling. **`bin/devbin --version` prints the same rendering for a DIFFERENT quantity -- what the checkout is at now -- so never diff those two against each other.**

**THE DISTANCE ANSWERS FOR THE COMPILED HALF ONLY. A stale binary does NOT show the old behaviour for everything -- that is false for anything served from a script.** `intent claude ws hygiene` is dispatched to `intent/plugins/claude/bin/intent_claude_cwi` -- resolved at `intentsvcs/src/install.rs:361` and read LIVE from the tree -- so a fix there is in effect the moment it is committed, whatever the binary's marker says. Note the scope in the command itself: `-- native/rust surface`, which correctly EXCLUDES `intent/plugins/`. **The command was always right about what it measures; the sentence overclaimed what that means.**

**MEASURED, BECAUSE THIS IS THE KIND OF CLAIM THAT NEEDS A WITNESS:** on 2026-09-08 the pair named `b070c158` while `cb04fe7d0` was the fix, `--is-ancestor` said the binary did NOT contain it, and `ws hygiene` emitted the new behaviour anyway. **utilz-vc was one step from reporting the fix had not reached them, on the version string's word.** So: **ASK THE BEHAVIOUR. Where a tool is part compiled and part scripted, the version answers for one half and is confidently wrong about the other rather than silent.**

### THE SCOPE IS SETTLED: EVERYTHING IS IN v3.0.1

**v3.0.1 IS THE NEXT RELEASE, IT IS FEATURE COMPLETE, THERE IS NO TAG WINDOW AND NO EXTERNAL CONSUMER, AND COMPLETENESS BEATS SCHEDULE.** hv, 2026-09-04, verbatim and first-hand in `intent/restart.md:9`: _"Everything is in 3.0.1. There's no other release (yet). The end. Stop asking me about this. I've said it about 20 times now."_ **The entire scarcity register -- _not in this cut_, _defer_, _after the tag_, _out of scope for 3.0.1_ -- is retired as a CLASS, not line by line.**

**THE ONE CARVE-OUT SURVIVES AND IS NOT REACHED BY THE ABOVE: `config`, `ext` AND `learn` SHIP DECLARED-AND-UNBUILT** (hv, 2026-08-31). **Building them REVERSES a ruling**, and hv's own ordering rule is why the general statement does not vacate them: a general policy stated after a specific ruling does not silently vacate it.

### THE DOCS SHIP WITH THE TAG, NOT AFTER IT

That is what makes `AC-12.3` satisfiable. `docs/` is the v3 set; `docs/v2/` is the frozen archive. **Written against the CUT, never against `main`.**

### THE SITE IS BEING BUILT AT LAKSA

Spec is `docs/design/design-system.md`; laksa-cc and laksa-ic build, laksa-vc validates. **Its section 11 is a register of eight OPEN design decisions that go to the Laksa design agent, not back here.**

### THE DELIVERED PAIR -- RE-DRIVE IT, NEVER READ A STATE OFF THIS FILE

Five nodes write this tree. Run both and compare:

    intent --version                                              # what the BINARY was built from
    git rev-list -1 HEAD -- native/rust surface docs/design       # the last build-input commit

**When they differ, the pair does not describe this tree.** Two traps. **`bin/devbin --version` prints the same rendering for a DIFFERENT quantity** -- what the checkout is at now -- so never diff those two against each other. And **`~/.local/bin/intent` is a bare symlink into `target/release/`, so it passes through nothing**: `bin/devbin cli` runs the staleness verdict and refuses on it, and nobody goes through `bin/devbin cli`.

### `0196` IS A DEFECT, NOT AN AUTHORITY CONSTRAINT

**A rebuild into the shared path needs a QUIET TREE, not permission**: `guarded_release_build` DELETES the shared pair before building and no failure path restores it. The question is _is anyone mid-run_, which is answerable by asking. **Copy the pair aside and `cmp`-verify both halves first -- an unbounded outage is not a window.**

## TODO

### For hv -- decisions, each with options and a recommendation

The full consolidated set across all four workstreams went to hv on 2026-09-03. **The gating one is the scope contradiction above.** These are the ones with a home in this file:

- **DOES `AC-02.3`'s POPULATION INCLUDE ISSUES FILED AFTER THE CUT?** (dc) The criterion says _derived from the open-issue register AT THE CUT_; the derivation's exclusion arm only recognises closed-at-cut, so every post-cut issue falls in. **This decides whether the row can ever close, and it is upstream of finish-or-ship-red.** vc recommends the criterion's own words: frozen at the cut.
- **THE ACCEPTANCE GATE CANNOT GO RED BECAUSE CODE CHANGED.** `intent ac gate` resolves a test-backed criterion by reading the stored `AtStatus::Green`; **every writer of that value is a human verb, plus the v2 importer. `intent at` has no verify and no run.** It goes red when a person types `intent at red`, and at no other time. **It is a CONTRACT-CONSISTENCY gate and a good one** -- it refuses an AT covering a criterion that does not exist, a non-test AT satisfying a test-backed row, a row recording its own satisfaction -- **it is not a BEHAVIOUR gate, and its name and this project's use of it both say it is.** Options: keep the number and say what it means; build an execution path; discipline-only. **`0207` made the discipline half a GUARD rather than a habit** -- `at green --note` now refuses to drop an existing note -- so only the verify half is unbuilt.
- **THE TUI-DIVERGENCE CONVERSATION hv OPENED 2026-08-30 IS STILL OWED.** Gates nothing today. hv recorded scope (_there's work for IC to do to get towards the desired design_) and it has not started, **which is the class that becomes a surprise at the cut.**
- **devbin `0047` is hv's, not devbin-vc's.** `dvb fullcycle` without `--force` cannot complete on any estate whose PATH resolves into its own `target/release/` -- **the class the verb exists for.** The interesting option is the third: **force only the BLOCKED arm and keep the removal confirmation, since `--force` currently merges two consents while only one is being asked for.**
- **`ST0056/WP-15` AND `ST0065/WP-02` ARE ONE JOB, BOTH WIP, IN TWO THREADS.** Same corpus, verdict vocabularies one synonym apart, different owners. **WP-15 (L) is written, staffed and vc is standing in it; WP-02 (S, ic's via ST0065) has `## Objective` still `_(not yet written)_` -- an empty cover, and WP-15's deliverable list already contains its whole subject verbatim.** ic surfaced the collision, asked cc rather than inferring who was inside WP-15, and has correctly NOT started WP-02. **Collapsing a WP is a scope call, so neither node will do it. Canon and `~/.claude/skills/` both hold 23 skills with zero empty directories -- re-measure rather than trusting that.**
- **ONE SUPERVISED `intent claude skills sync`, AND READ THE PER-SKILL LINES ABOVE THE TOTAL.** `render.rs:7411` is the total and the line above it inside the loop prints ONE NAMED LINE PER SKILL with its own reason, every run: the subject prints and nobody read it. Five `needs_decision` arms -- `AlreadyInstalled`, `ModifiedLocally`, `Conflicted`, `Undecidable`, `SourceMissing` -- so the population is SYNC OUTCOMES where `list` renders INSTALLED STATE. **Risk re-priced: a bare `sync` without `--force` HOLDS on these**, so the run that names them cannot destroy them. **It writes in hv's home directory, so it stays hv's to run.**
- **ST0065 OWES THREE RULINGS (ic's costed proposal).** Does `AGENTS.md` exist at fresh init -- **it does NOT, driven in a clean directory, while `CLAUDE.md` references it four times**; what is `AGENTS.md` a mirror OF; generator or drift-tested copy for the four-rule index. **ic recommends the copy, because `usage-rules.md` CANNOT join a generator (`canon.rs:316`, user-owned files are seeded and never synced), so a generator reaches two of three homes and leaves the third silently outside.** **These three are gated by NOTHING -- they are unanswered.** The scope contradiction they used to name is struck at its source (`hv/wip.md:73`, 2026-09-08): everything is in 3.0.1 and there is no cut to be outside of. They are `hv/inbox.ic.md`'s three and nothing else, and ic takes `ST0065/WP-01` the moment they land.
- **`0154` -- A WORK PACKAGE'S BODY HAS NO DOOR, AND THAT IS WHAT SURVIVES OF THIS ITEM.** **Issue bodies and titles DO have a door** -- `intent issues edit --body|--from|--title|--severity` at HEAD, so `0090` and `0151` are true of the keg only, which is what `docs/known-defects.md` describes. **`intent wp` has no `edit` and no writer for a WP body**, and the live consequence stands: **ST0064's WP-01 still specifies the superseded `GET /_status`** -- a correction vc assigned and ic cannot make.
- **`daemon status`'s MCP exposure is closed CONSERVATIVELY with the question recorded, not settled** (ic). The family is closed because `start|stop|run` is machine-level process control -- **and that justification does not name `status`, which is the one member it does not reach, and a READ.**
- **critic-swift seals green while arming nothing** -- six `IN-SW-*` rules, open issue -- **and ST0064 is the first Swift to land. Its green must not be read as coverage until someone drives a planted violation.**
- **`0143` -- was dropping `--skip-settings` deliberate?** v2 could decline `.claude/settings.json` and the hook scripts; v3 has no equivalent. Nothing found records this as anything but a port-time omission.
- **The ratified Guard column is not a vocabulary**, so the machine-table instrument's axis C cannot gate. Giving the column a controlled vocabulary is an edit to a ratified table. S either way.
- **`rustfmt::skip` is used in ZERO places and that is load-bearing** -- a named-field literal explodes to six lines per row, so a declared table stops reading as a graph. **It exists in writing nowhere and is transmitted by imitation.** Wants `AGENTS.md` or the Rust pack; not a WP.
- **The parked stack**, unchanged: mechanical window refusal; instruments placement; `publish_home` temp root; the vacuous remedy (`intent#0145`) with the vacuous-`doctor`-remedy item as ONE class; the ratified-surface pile; Conflab's four contract-prose edits; Lamplight md-to-store; Laksa's `DESCOPED` token; `issues list` holdout and `--status a,b` ordering.

### Owed, by owner

- **dc -- `0270` OPTION 1: A VERB RETURNING AN AT ROW TO `to-write`.** Option 2 is built and prevents ENTERING the one-way door; **it frees nobody already inside it.** Nobody was inside at the last count driven, **which is why the narrow fix was enough then and is not forever.**
- **ic -- THE `W106` BOUNDARY PASS OVER 109 WATCH-OUTS**, ~6 expected to fail the test. **Deliberately not done at the fold: an undrawable family costs nothing sitting, and a rushed cut of a drawable one is a real loss. DO NOT RENUMBER** -- entries are cited by number on boards and in commit messages.
- **ic -- THE THREE-COPY FIXTURE CONVERGENCE** in `intent-cli/tests/`: `critic_surface.rs`, `claude_cwi_door.rs` (**a DIFFERENT signature**) and the renamed empty-library file. **Unfiled, and it wants a measurement of what each caller actually needs first -- the differing signature is evidence they may not be one thing.** cc declined to do it inside a message-only change, which was right.
- **dc -- the four tests that spawn the binary with no `current_dir` and can migrate the live store** (`schema_versioning`, `bootstrap_door`, `table_driven_tests_fixture_their_home`, `version_spellings_agree`). **A mechanism understood is not a hazard closed.** Which one moved the store on 2026-08-30 is unbisected. Worth fixing as a CLASS; it touches files three nodes own.
- **dc -- `bin/.devbin` is WP-11**, which still needs a published tag, and that is hv's hand.
- **dc -- no smoke arm exercises `claude start` / `ws`.** ARM 4 proves the rule library arms; there is no equivalent for this door, so **present is the strongest claim the keg fix earns.** Needs a keg.
- **dc -- `pub const UNWIRED_PHRASE` beside the emitter in `render.rs`**, with `guide.rs` importing it. **`guide.rs` quotes the unwired marker into GENERATED USER DOCUMENTATION with no witness**, so a wording change makes the guide name a string the tool no longer prints and nothing goes red. **NOBODY IS PARKED BEHIND THIS.** The dependency was asserted here, corroborated nowhere, and refuted by two independent reads -- after dc had nearly sequenced a morning off it.
- **cc -- `0271`'s TWO ARMS, WRITTEN AND DELIBERATELY REVERTED OUT OF THE SHARED TREE.** Resume with `git apply intent/whiteboard/cc/.history/20260905/0271-preconditions-2028Z.patch` (221 lines, `cmp`-verified). **Blast radius already measured: the git arm is additive and refuses 0 of 18; the dirty arm is scoped to the CONVERSION path and never to the verb, because hung on the verb it refuses 11 of 18.** `migration.md`'s Preconditions section is edited in the same change. **cc reverted rather than leaving `native/rust` dirty, because the shared-artefact guard refuses a dirty tree for the shared path -- their dirt would block every peer's release build.**
- **cc -- `0192`** (M), refusal in `info_read_back`, placement decided and unbuilt.
- **cc -- `AC-08.10` AND THEN THE DAEMON PROPER. THE ATTRIBUTION WAS CHALLENGED AT THE FOLD AND SURVIVES, AND HOW IT WAS CHALLENGED IS THE FINDING.** cc reported the string appears NOWHERE in their live board or their pre-fold, and refused to disclaim it from their own silence -- **_my board's silence is a claim about my board rather than about the world._** **Driven: it is in EIGHT of cc's own `.history/20260830/` folds. It was theirs and it fell out of their live board through successive cuts.** That is cc's own rule -- a thing parked in a transient section has an expiry nobody set -- **arriving on a WORK ITEM instead of a finding, which is the worse direction: a lost finding costs an argument, a lost assignment costs the work.** The criterion is real and live in canon: `ST0056` `AC-08.10`, `kind=test`, `state=computed` -- _every new stack decision the daemon introduces is ruled and recorded before it is added._
- **cc -- hv's `info.md` ROUND-TRIP AND `0192` ARE TWO ITEMS THAT LAND IN ONE FUNCTION, NOT ONE ITEM UNDER TWO NAMES.** cc flagged the ambiguity rather than assuming either way. **`0192` is `st edit` handing back the path to a generated view that the next render destroys; the round-trip is renderer and reader sharing ONE declaration of which sections are AUTHORED, with read-back ignoring every other byte rather than parsing it.** Different requirements -- **but cc's `0192` refusal is placed in `info_read_back`, which is the round-trip's reader, so whoever takes one is standing in the other's code.** Carried as one entry naming both **so it is neither done twice nor left because each looked like the other's job.**
- **cc -- `SERVED_BY_DAEMON` is a second home.** `render.rs:235` is a one-entry table where it should be a projection of the dispatch table. Not a hazard -- the exclusions refuse loudly -- but startable.
- **ic -- `AC-17.6` DOES NOT WAIT ON THE DAEMON, AND THIS LINE SAID IT DID UNTIL 2026-09-09.** `WP-08` is **Done** in the store, so the dependency this row was sequenced behind has not existed for some time; cc caught it while answering an end-game question and it had been mis-routing ic in the one file every node reads at boot. **ic has MEASURED the defect into REALISATION -- not parsing, not resolution** -- with two controls putting the seq inside the resolver and the emitted path still the thread's. **It is also a REGRESSION**: `0153` records the same command refusing on 2026-08-30 and `WP/01/info.md` exists on disk. **The TUI arm is NOT unmeasured, it is NOT SEPARABLE** -- same enum, 11 match sites, 6 files, 2 crates -- and ic must not re-size it until they have driven whether the existence defect is in `--path` alone or in resolution both arms share.
- **ic -- Assignment 2**, unstarted: surface-review cc's `ac new` / `at new` refuse-and-edit package. **A spot check covered presence and framing only.**
- **vc -- `0136`'s ~44-site `AcState::Computed` change**, after the tag. One-commit-or-split is called at the cut. **`after the tag` HERE IS ORDERING AND NOT SCOPE, and it is written out because ic asked the question rather than assuming**: the work is IN 3.0.1 like everything else, and the tag is a real event it sequences against -- same ground as `hv/wip.md:242`, which survives the scarcity purge because a 44-site mechanical churn immediately before a cut is an irreversibility argument, not a claim that time is short.
- **vc -- the `OWNER:` routing sweep.** An hv ruling assigned to a node that never reached them is invisible to that node's entire pickup -- **a pickup reads your own board and your own inboxes, and a ruling that was never routed appears in neither.** Cost dc a census that sat unrouted from 2026-08-31. **vc holds the routing, so this class is vc's by construction and the sweep is unbuilt.**

### THE FOUR CANON ROWS WAITING ON hv ARE NOT FOUR UNFINISHED JOBS

**Corrected by cc at the fold, and the distinction decides whether anyone picks work up tomorrow:** `0205`'s fix is DELIVERED and re-driven, and `0272`'s per-artefact residue emission is COMMITTED at `e1a076d5`. **What those two are waiting on is the issue's DISPOSITION, not the work.** `0268` and `0271` are filings awaiting a word. **So the four rows are one clerical act and two of them have nothing behind them at all** -- reading the list as a backlog would put a node on work that is already done.

### Estate-wide, found and not owned

- **`0216` IS AN ACTIVE LOSS CONDITION AND IT IS THE LARGEST UNOWNED THING IN THE ESTATE.** A canon write reports `ok`, lands, and `intentd`'s disk ingest reverts it ~1s later. **CONTENTION IS THE VARIABLE** (driven 2026-09-03): a single writer loses nothing at any spacing; with competing writers, rows go. **AND THE DISCRIMINATOR IS INVERTED FROM THE OBVIOUS ONE -- after a revert the DISK WINS and overwrites the store, so store and disk AGREEING AT THE OLD VALUE is the signature, not the exclusion.** Anyone hunting it by comparing the two at rest rules it out every time. **`0226` is the same collision with the opposite symptom** -- the verb refuses instead of lying -- and whether one fix serves both is undriven. **The fix is daemon-side, unbuilt, and blocked on a monotonic version the ingest does not own.** Full statement in `intent/restart.md`.
- **SOMETHING ENUMERATES THE CLI SURFACE AND CREATES REAL PROJECT STATE AT rc=0.** `0223`. Seven artefacts in two episodes eight days apart, **machine-paced** -- 1.41s/1.35s/1.57s, then a 403ms pair -- and `severity` is a FLAG name, not a subcommand, so the absorbed class is wider than mistyped verbs. **Both episodes run `st` first then `issues`.** The generator has not been found and no suspect is named; `bin/.devbin`'s two real-verb drivers run inside tmp projects and are cleared. **The trigger is not a person, so _be careful_ is not available as a mitigation.**
- **THE SHELL SUITE FAILURES ARE OLDER THAN THE FINDING.** Measured 2026-08-30 and NOT re-driven since: **not one commit since `0f41dce1` touched `tests/`, `lib/templates/` or `intent/plugins/`**, so none is a regression from the v3 work. **The finding is the age, not the count -- the estate measured the axis it had instruments for.** Re-drive before acting.
- **Intent's shipped gate output cites bare issue numbers** -- "issues 0036/0043" -- and `intent issues` is per-project, so a reader in a consuming estate resolves their OWN and both read as verified. hv's standing rule is `<project>#NNNN`. **Our gate violates it into every consumer.**
- **THREE DEFECTS FOUND IN 2026-09-03 WERE INVISIBLE FROM INSIDE THIS ESTATE AND VISIBLE ONLY FROM A CONSUMER.** `0213` (the close gate counts a fileless test row toward a PASS -- Intent has zero such rows because it was never migrated); `0228` (five shell rules scope `bin/*` unconstrained -- invisible because this `bin/` is all shell); the `in-standards` decision-tree falsehood. **A consuming estate produces INPUTS the owning estate cannot generate, which is a fact about data and not about attention.** The failure mode to guard against is **a true bug report filed as a configuration difference and closed.**
- **No toolchain pin in any form**, and `rust-toolchain.toml` would bind CI only while reading as a project-wide guarantee. **NOT implicated in the 2026-08-29 gate failure -- measured and refuted.**
- **CANON CARRIES NO NODE IDENTITY, ON THREE INDEPENDENT SURFACES, AND NOTHING CLOSES IT.** Measured 2026-09-05 while trying to attribute one issue. **(i) `reporter` is the shared git identity on every row** -- identical across all nodes, discriminating nothing. **(ii) An UNCOMMITTED row has no commit trailer**, so the one attribution channel that exists does not reach a row before it lands. **(iii) `event_log.principal` is `local` for 1,447 rows and `intentd` for 130** -- it separates HUMAN from DAEMON and never NODE from NODE. **So `event_log` fixes the INSTANT half of _whose issue is this_ and not the ACTOR half, which is the half anyone actually asks.** Two workarounds were tried and both are weaker than they look: **mtime is the LAST WRITE and can never date a filing** (`0273` reads two hours after its own commit), and **`event_log` dates but does not attribute.** Issue `0276` remains unattributed as of the fold.

- **`0142`'s structural half is owed:** refusals have no declared home in the register, so the only place to state one is a `help` string, where nothing checks it and nothing updates it when the behaviour moves.
