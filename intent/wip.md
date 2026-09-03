---
verblock: "03 Sep 2026:v1.33: vc - aggressive globalfold; DOING and TODO only, four stale premises struck against drives, and the 3.0.1 scope is CONTESTED not settled"
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

### THE 3.0.1 SCOPE IS CONTESTED AND ONLY hv CAN SETTLE IT

**This file carried _scope is ST0056 and its feeders, nothing wider_ as settled fact. It is not settled.** `hv/wip.md:84` records hv first-hand, 2026-09-01 08:32Z: **_"Everything here outstanding is going into 3.0.1... I want 3.0.1 to be feature complete and I don't care how long it takes."_** The narrow-scope statements on `hv/wip.md:40` and `:73` are **vc pen notes from 2026-08-31, one day earlier**, and were never struck.

**So three documents carry a scope the ruler superseded, and nodes have been working off different halves.** Under the narrow reading `ST0060` (`intent vault`), `ST0046` (modules) and `ST0065` are out; under hv's ruling they are in. **Put to hv 2026-09-03 with options; unanswered. Do not resolve it by picking the half your work needs.**

**Not contested, and true under either reading:** `intentd` is IN via WP-08, and **ST0064 narrows to the macOS menubar app and IS IN** -- hv ruled that first-hand on 2026-08-31 (_the intentd and the menubar app that controls it are linked and need to both land_).

### WP-08 IS THE DAEMON, IT IS cc's, AND IT IS THE LONGEST POLE

Everything downstream of a shipped `intentd` waits on it. **This file deliberately names no per-criterion state for it** -- the row-by-row figure moved three times on 2026-08-30 alone. `intent ac status ST0056` and cc's board are the two homes that track it.

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
- **`ST0056/WP-15` AND `ST0065/WP-02` ARE ONE JOB.** Same corpus, same verdict vocabulary one synonym apart, different threads, different owners. WP-15 is Not Started, scope L. **vc recommended re-scoping WP-15 to EXECUTION rather than cancelling -- AND THAT RECOMMENDATION MAY ITSELF BE STALE**: the retirement it would execute has since executed, and `AC-15.3`'s premise on `hv/wip.md` (_25 skills, two empty orphans_) is expired. **Re-measure before acting: canon and `~/.claude/skills/` both hold 23 with zero empty directories (driven 2026-09-03).**
- **ONE SUPERVISED `intent claude skills sync`, AND READ THE PER-SKILL LINES ABOVE THE TOTAL.** **The old framing here -- _a standing count with no visible subject_ -- WAS FALSE and is struck.** `render.rs:7411` is the total and the line above it inside the loop prints ONE NAMED LINE PER SKILL with its own reason, every run: the subject prints and nobody read it. Five `needs_decision` arms -- `AlreadyInstalled`, `ModifiedLocally`, `Conflicted`, `Undecidable`, `SourceMissing` -- so the population is SYNC OUTCOMES where `list` renders INSTALLED STATE. **Risk re-priced: a bare `sync` without `--force` HOLDS on these**, so the run that names them cannot destroy them. **It writes in hv's home directory, so it stays hv's to run.**
- **ST0065 OWES THREE RULINGS (ic's costed proposal).** Does `AGENTS.md` exist at fresh init -- **it does NOT, driven in a clean directory, while `CLAUDE.md` references it four times**; what is `AGENTS.md` a mirror OF; generator or drift-tested copy for the four-rule index. **ic recommends the copy, because `usage-rules.md` CANNOT join a generator (`canon.rs:316`, user-owned files are seeded and never synced), so a generator reaches two of three homes and leaves the third silently outside.** **These three are gated by the scope contradiction above.**
- **`0154` -- A WORK PACKAGE'S BODY HAS NO DOOR, AND THAT IS WHAT SURVIVES OF THIS ITEM.** **Struck as stale: issue bodies and titles now DO have one** -- `intent issues edit --body|--from|--title|--severity` exists at HEAD, so `0090` and `0151` are no longer true of HEAD (they remain true of the keg, which is what `docs/known-defects.md` describes). **`intent wp` has no `edit` and no writer for a WP body**, and the live consequence stands: **ST0064's WP-01 still specifies the superseded `GET /_status`** -- a correction vc assigned and ic cannot make.
- **`daemon status`'s MCP exposure is closed CONSERVATIVELY with the question recorded, not settled** (ic). The family is closed because `start|stop|run` is machine-level process control -- **and that justification does not name `status`, which is the one member it does not reach, and a READ.**
- **critic-swift seals green while arming nothing** -- six `IN-SW-*` rules, open issue -- **and ST0064 is the first Swift to land. Its green must not be read as coverage until someone drives a planted violation.**
- **`0143` -- was dropping `--skip-settings` deliberate?** v2 could decline `.claude/settings.json` and the hook scripts; v3 has no equivalent. Nothing found records this as anything but a port-time omission.
- **The ratified Guard column is not a vocabulary**, so the machine-table instrument's axis C cannot gate. Giving the column a controlled vocabulary is an edit to a ratified table. S either way.
- **`rustfmt::skip` is used in ZERO places and that is load-bearing** -- a named-field literal explodes to six lines per row, so a declared table stops reading as a graph. **It exists in writing nowhere and is transmitted by imitation.** Wants `AGENTS.md` or the Rust pack; not a WP.
- **The parked stack**, unchanged: mechanical window refusal; instruments placement; `publish_home` temp root; the vacuous remedy (`intent#0145`) with the vacuous-`doctor`-remedy item as ONE class; the ratified-surface pile; Conflab's four contract-prose edits; Lamplight md-to-store; Laksa's `DESCOPED` token; `issues list` holdout and `--status a,b` ordering.

### Owed, by owner

- **dc -- the four tests that spawn the binary with no `current_dir` and can migrate the live store** (`schema_versioning`, `bootstrap_door`, `table_driven_tests_fixture_their_home`, `version_spellings_agree`). **A mechanism understood is not a hazard closed.** Which one moved the store on 2026-08-30 is unbisected. Worth fixing as a CLASS; it touches files three nodes own.
- **dc -- `bin/.devbin` is WP-11**, which still needs a published tag, and that is hv's hand.
- **dc -- no smoke arm exercises `claude start` / `ws`.** ARM 4 proves the rule library arms; there is no equivalent for this door, so **present is the strongest claim the keg fix earns.** Needs a keg.
- **dc -- `pub const UNWIRED_PHRASE` beside the emitter in `render.rs`**, with `guide.rs` importing it. **`guide.rs` quotes the unwired marker into GENERATED USER DOCUMENTATION with no witness**, so a wording change makes the guide name a string the tool no longer prints and nothing goes red. cc is parked behind this.
- **cc -- `AC-08.10` and then the daemon proper.** And hv's `info.md` round-trip, ruled and not started: **renderer and reader share ONE declaration of which sections are authored, and read-back ignores every other byte rather than parsing it.**
- **cc -- `SERVED_BY_DAEMON` is a second home.** `render.rs:235` is a one-entry table where it should be a projection of the dispatch table. Not a hazard -- the exclusions refuse loudly -- but startable.
- **ic -- `AC-17.6` waits on cc's WP-08**: `browsed()` is an unconditional stub that refuses even with the daemon up. **The remaining work is ONE ROW, not an XL.**
- **ic -- Assignment 2**, unstarted: surface-review cc's `ac new` / `at new` refuse-and-edit package. **A spot check covered presence and framing only.**
- **vc -- `0136`'s ~44-site `AcState::Computed` change**, after the tag. One-commit-or-split is called at the cut.
- **vc -- the `OWNER:` routing sweep.** An hv ruling assigned to a node that never reached them is invisible to that node's entire pickup -- **a pickup reads your own board and your own inboxes, and a ruling that was never routed appears in neither.** Cost dc a census that sat unrouted from 2026-08-31. **vc holds the routing, so this class is vc's by construction and the sweep is unbuilt.**

### Estate-wide, found and not owned

- **`0216` IS AN ACTIVE LOSS CONDITION AND IT IS THE LARGEST UNOWNED THING IN THE ESTATE.** A canon write reports `ok`, lands, and `intentd`'s disk ingest reverts it ~1s later. **CONTENTION IS THE VARIABLE** (driven 2026-09-03): a single writer loses nothing at any spacing; with competing writers, rows go. **AND THE DISCRIMINATOR IS INVERTED FROM THE OBVIOUS ONE -- after a revert the DISK WINS and overwrites the store, so store and disk AGREEING AT THE OLD VALUE is the signature, not the exclusion.** Anyone hunting it by comparing the two at rest rules it out every time. **`0226` is the same collision with the opposite symptom** -- the verb refuses instead of lying -- and whether one fix serves both is undriven. **The fix is daemon-side, unbuilt, and blocked on a monotonic version the ingest does not own.** Full statement in `intent/restart.md`.
- **SOMETHING ENUMERATES THE CLI SURFACE AND CREATES REAL PROJECT STATE AT rc=0.** `0223`. Seven artefacts in two episodes eight days apart, **machine-paced** -- 1.41s/1.35s/1.57s, then a 403ms pair -- and `severity` is a FLAG name, not a subcommand, so the absorbed class is wider than mistyped verbs. **Both episodes run `st` first then `issues`.** The generator has not been found and no suspect is named; `bin/.devbin`'s two real-verb drivers run inside tmp projects and are cleared. **The trigger is not a person, so _be careful_ is not available as a mitigation.**
- **THE SHELL SUITE FAILURES ARE OLDER THAN THE FINDING.** Measured 2026-08-30 and NOT re-driven since: **not one commit since `0f41dce1` touched `tests/`, `lib/templates/` or `intent/plugins/`**, so none is a regression from the v3 work. **The finding is the age, not the count -- the estate measured the axis it had instruments for.** Re-drive before acting.
- **Intent's shipped gate output cites bare issue numbers** -- "issues 0036/0043" -- and `intent issues` is per-project, so a reader in a consuming estate resolves their OWN and both read as verified. hv's standing rule is `<project>#NNNN`. **Our gate violates it into every consumer.**
- **THREE DEFECTS FOUND IN 2026-09-03 WERE INVISIBLE FROM INSIDE THIS ESTATE AND VISIBLE ONLY FROM A CONSUMER.** `0213` (the close gate counts a fileless test row toward a PASS -- Intent has zero such rows because it was never migrated); `0228` (five shell rules scope `bin/*` unconstrained -- invisible because this `bin/` is all shell); the `in-standards` decision-tree falsehood. **A consuming estate produces INPUTS the owning estate cannot generate, which is a fact about data and not about attention.** The failure mode to guard against is **a true bug report filed as a configuration difference and closed.**
- **No toolchain pin in any form**, and `rust-toolchain.toml` would bind CI only while reading as a project-wide guarantee. **NOT implicated in the 2026-08-29 gate failure -- measured and refuted.**
- **`0142`'s structural half is owed:** refusals have no declared home in the register, so the only place to state one is a `help` string, where nothing checks it and nothing updates it when the behaviour moves.
