---
verblock: "30 Aug 2026:v1.30: vc - globalfold; DOING and TODO only, everything else to history"
intent_version: 3.0.0
---

# Work In Progress

**DOING and TODO ONLY, NO EXCEPTIONS (hv, 2026-08-30). Completed work does not belong in this file** -- it lives in `done.md`, `intent/history/`, `CHANGELOG.md`, and the threads' own closed criteria. **A fifth copy here drifts from all four.** The pre-fold body is at `intent/history/202608-wip-prefold.md`.

**Current as at `66e08a74`. This names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

    intent ac gate ST0056     # the release gate
    intent ac gate ST0058     # feeder
    intent ac gate ST0066     # feeder
    intent ac gate ST0068     # feeder -- the docs
    intent doctor

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now.

## DOING

**v3.0.1 IS THE NEXT RELEASE AND IT IS NOT CUT.** Scope is hv's: **ST0056 and its feeders -- ST0058, ST0066, ST0068 -- and nothing wider.** `intent vault` (ST0060) and modules (ST0046) are OUT as new features rather than parity. **`intentd` is IN** -- WP-08 builds it inside ST0056 -- and **ST0064 narrows to the macOS menubar app, which stays out of 3.0.1.** The number stays `3.0.1` with the cost stated and taken: a reader upgrading from 3.0.0 gets four new verbs.

**WP-08 IS THE DAEMON AND IT IS cc's.** It is the longest pole in the cut and everything downstream of a shipped `intentd` waits on it. **THIS FILE DELIBERATELY NAMES NO PER-CRITERION STATE FOR IT** -- the row-by-row figure moved three times on 2026-08-30 alone, and a transcription here would have been wrong within the hour each time. `intent ac status ST0056` and cc's own board are the two homes that track it.

**THE DOCS SHIP WITH THE TAG, NOT AFTER IT.** That is what makes `AC-12.3` satisfiable. `docs/` is the v3 set; `docs/v2/` is the frozen archive. **Written against the CUT, never against `main`.**

**THE SITE IS BEING BUILT AT LAKSA.** Spec is `docs/design/design-system.md`; laksa-cc and laksa-ic build, laksa-vc validates. **Its section 11 is a register of eight OPEN design decisions that go to the Laksa design agent, not back here.**

**THE DELIVERED PAIR IS `0751c42b` AND BOTH HALVES NAME IT.** That is the last commit touching `native/rust` + `surface`, and `intent --version` reports it. **Do not read HEAD being ahead as staleness -- the deciding test is `git diff --name-only <marker>..HEAD -- native/rust surface`.**

## TODO

### For hv

- **`ST0056/WP-15` AND `ST0065/WP-02` ARE ONE JOB.** Same corpus, same verdict vocabulary one synonym apart, different threads, different owners. WP-15 is Not Started and scope L; ST0065/WP-02 is wip, catalogued, retirements already executing. **Menu with options and a recommendation is on `hv/wip.md`.** vc recommends re-scoping WP-15 to EXECUTION rather than cancelling: the triage half is done, the criteria are the part worth keeping, and `AC-15.3` is live -- **canon holds 23 skills and `~/.claude/skills/` holds 25, two of them empty orphans invisible to `claude skills list`.**
- **`0154` -- NO ENTITY'S AUTHORED PROSE IS EDITABLE AFTER CREATION.** `0090` (issue bodies), `0151` (titles), and no door to a WP body. **The live consequence is ST0064's WP-01 still specifying the superseded `GET /_status`** -- a correction vc assigned and ic cannot make. **A fix reaching only `body`, or only `issues`, closes the instances already filed and leaves the case that prompted them.**
- **ST0065 -- ic's costed proposal still owes three rulings.** Does `AGENTS.md` exist at fresh init (today it does NOT, and `CLAUDE.md` references it four times); what is `AGENTS.md` a mirror OF; generator or copy for the four-rule index, given `usage-rules.md` cannot join because it is seeded once and user-owned after (`canon.rs:316`). **ST0065 is out of the 3.0.1 cut.**
- **`daemon status`'s MCP exposure is closed CONSERVATIVELY with the question recorded, not settled** (ic). The family is closed because `start|stop|run` is machine-level process control -- **and that justification does not name `status`, which is the one member it does not reach, and a READ.**
- **critic-swift seals green while arming nothing** -- six `IN-SW-*` rules, open issue -- **and ST0064 is the first Swift to land. Its green must not be read as coverage until someone drives a planted violation.**
- **`0143` -- was dropping `--skip-settings` deliberate?** v2 could decline `.claude/settings.json` and the hook scripts; v3 has no equivalent. Nothing found records this as anything but a port-time omission.
- **The ratified Guard column is not a vocabulary, so the machine-table instrument's axis C cannot gate.** Giving the column a controlled vocabulary is an edit to a ratified table. S either way.
- **`rustfmt::skip` is used in ZERO places and that is load-bearing** -- a named-field literal explodes to six lines per row, so a declared table stops reading as a graph. **It exists in writing nowhere and is transmitted by imitation.** Wants `AGENTS.md` or the Rust pack; not a WP.
- **The parked stack**, unchanged: mechanical window refusal; instruments placement; `publish_home` temp root; the vacuous remedy (`intent#0145`) with the vacuous-`doctor`-remedy item as ONE class; the ratified-surface pile; Conflab's four contract-prose edits; Lamplight md-to-store; Laksa's `DESCOPED` token; `issues list` holdout and `--status a,b` ordering.

### Owed, by owner

- **dc -- the four tests that spawn the binary with no `current_dir` and can migrate the live store** (`schema_versioning`, `bootstrap_door`, `table_driven_tests_fixture_their_home`, `version_spellings_agree`). **cc explained the mechanism -- the store is project-relative, so `HOME` never covered it -- and a mechanism understood is not a hazard closed.** Which one moved the store on 2026-08-30 is unbisected. Worth fixing as a CLASS, and it touches files three nodes own.
- **dc -- `bin/.devbin` is WP-11**, which still needs a published tag, and that is hv's hand.
- **dc -- no smoke arm exercises `claude start` / `ws`.** ARM 4 proves the rule library arms; there is no equivalent for this door, so **present is the strongest claim the keg fix earns.** Needs a keg.
- **dc -- `pub const UNWIRED_PHRASE` beside the emitter in `render.rs`**, with `guide.rs` importing it. **`guide.rs` quotes the unwired marker into GENERATED USER DOCUMENTATION with no witness**, so a wording change makes the guide name a string the tool no longer prints and nothing goes red. cc is parked behind this.
- **cc -- `AC-08.10` and then the daemon proper.** And hv's `info.md` round-trip, ruled and not started: **renderer and reader share ONE declaration of which sections are authored, and read-back ignores every other byte rather than parsing it.**
- **ic -- WP-17 piece 3** (the TUI event loop, then the editor handoff where `AC-17.10`'s re-read lives), then ST0064 behind cc's daemon arm. **`ratatui` goes in WITH the first thing that draws, announced before `Cargo.lock` moves.**
- **ic -- Assignment 2**, unstarted: surface-review cc's `ac new` / `at new` refuse-and-edit package. **A spot check covered presence and framing only.**
- **vc -- `0136`'s ~44-site `AcState::Computed` change**, after the tag. One-commit-or-split is called at the cut.

### Estate-wide, found and not owned

- **THE SHELL SUITE HAS SIX FAILURES AND THEY ARE THREE TO FOUR DAYS OLD.** Measured 2026-08-30: **not one commit since `0f41dce1` touched `tests/`, `lib/templates/` or `intent/plugins/`**, so none is a regression from the v3 work. `rule_pack_agnostic` expects four agnostic rules and there are five since `red-control` landed 2026-08-26; `guard_dispatch` refuses `critic-guard.sh`, which landed deliberately inert and unrostered; `intent_bin_retarget_guard` catches `critic_arming_census.bats:290`; `shipped_surface_drift` reports ten files diverged from `Intentv2`; `devbin_fmt_md`'s PREMISE test correctly reports that README lost the fence it protects; `critic_dispatch` says the hook no longer fails open with no `intent` on PATH. **The finding is the four days, not the six: the estate measured the axis it had instruments for.**
- **Intent's shipped gate output cites bare issue numbers** -- "issues 0036/0043" -- and `intent issues` is per-project, so a reader in a consuming estate resolves their OWN and both read as verified. hv's standing rule is `<project>#NNNN`. **Our gate violates it into every consumer.**
- **No toolchain pin in any form**, and `rust-toolchain.toml` would bind CI only while reading as a project-wide guarantee. **NOT implicated in the 2026-08-29 gate failure -- measured and refuted.**
- **`0142`'s structural half is owed:** refusals have no declared home in the register, so the only place to state one is a `help` string, where nothing checks it and nothing updates it when the behaviour moves.
