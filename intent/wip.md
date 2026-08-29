---
verblock: "29 Aug 2026:v1.29: vc - EOD globalfold; gate green, 0144 closed, pair at 30a2dd81"
intent_version: 3.0.0
---

# Work In Progress

**DOING and TODO only. Completed work does not belong in this file** -- it lives in `CHANGELOG.md`, `docs/releases/`, and the threads' own closed criteria, and a fourth copy here drifts from all three.

**Current as at `4a7f5e4e`. This names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

    intent ac gate ST0056     # the release gate
    intent ac gate ST0058     # feeder
    intent ac gate ST0066     # feeder
    intent ac gate ST0068     # feeder -- the docs
    intent doctor

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now, and they moved twice today.

## DOING

**v3.0.1 IS THE NEXT RELEASE AND IT IS NOT CUT.** hv ruled the scope: **ST0056 and its feeders -- ST0058, ST0066, ST0068 -- and nothing wider.** `intent vault` (ST0060), modules (ST0046) and `intentd` (ST0064) are OUT as new features rather than parity. **hv also ruled the number stays `3.0.1`** with the cost stated and taken: a reader upgrading from 3.0.0 gets four new verbs.

**THE DOCS SHIP WITH THE TAG, NOT AFTER IT.** That is what makes ST0056's AC-12.3 satisfiable at all. `docs/` is the v3 set; `docs/v2/` is the frozen v2 archive. **Written against the CUT, never against `main`.**

**THE SITE IS BEING BUILT AT LAKSA.** Spec is `docs/design/design-system.md`; laksa-cc and laksa-ic build, laksa-vc validates. **Its section 11 is a register of eight OPEN design decisions that go to the Laksa design agent, not back here.**

**THE DELIVERED PAIR IS `30a2dd81` AND BOTH HALVES NAME IT.** That is the last commit touching `native/rust` + `surface`; HEAD is ahead of it by canon and board commits only. **Do not read the difference as staleness -- the deciding test is `git diff --name-only <marker>..HEAD -- native/rust surface`, not a comparison to `HEAD`.**

## TODO

### For hv

- **ST0065 -- ic's COSTED PROPOSAL, written 2026-08-28 and unrouted until today.** `intent/st/ST0065/_proposal-agents-md.md`. **Three things need a ruling:** does `AGENTS.md` exist at fresh init (today it does NOT, and `CLAUDE.md` references it four times including its opening paragraph -- both answers are coherent and lead to different documents); what is `AGENTS.md` a mirror OF; and generator-or-copy for the four-rule index, given `usage-rules.md` cannot join because it is seeded once and user-owned after (`canon.rs:316`). **The ordering is the urgent part: cc has already edited `_AGENTS.md` under ST0067 (`8a997c1e`) while the proposal about what `_AGENTS.md` IS sits unruled under ST0065.**
- **`ST0056/WP-07` is recorded WIP and its gate PASSES** -- `doctor`'s only surviving finding, and its own remedy says only a human can say which side is wrong. **WP-07 is dc's claim, so it is not vc's or cc's to close.** A row in this state sequences work that is already done.
- **0143 -- was dropping `--skip-settings` deliberate?** v2 could decline `.claude/settings.json` and the hook scripts; v3 has no equivalent. Under fail-forward, removing a flag is legitimate **when it is a decision**; nothing found records this as anything but a port-time omission.
- **The ratified Guard column is not a vocabulary, so the machine-table instrument's axis C cannot gate.** Entry states and edges gate exactly; the Guard cells hold effects and landing rules rather than preconditions. Giving the column a controlled vocabulary is an edit to a ratified table. S either way.
- **The parked stack**, unchanged except one promotion: mechanical window refusal; instruments placement; `publish_home` temp root; **the vacuous remedy, now a FINDING rather than a hypothesis** -- `st edit <real-thread> design` on a thread with no realised file prints `remedy: this artefact carries: ` with nothing after the colon, measured on `30a2dd81`, and it belongs with the vacuous-`doctor`-remedy item as ONE class; the ratified-surface pile; Conflab's four contract-prose edits; Lamplight md-to-store; Laksa's `DESCOPED` token; `issues list` holdout and `--status a,b` ordering; ST0066 minutia 3.

### Owed, by owner

- **cc -- hv's `info.md` round-trip.** Ruled and recorded, not started. `st edit` opens `info.md` and `sync --to-store` reads `## Objective` and `## Context` back. **Binding condition: renderer and reader share ONE declaration of which sections are authored, and read-back ignores every other byte rather than parsing it.** Five open issues in this estate are prose destroyed by a reader that was only supposed to read.
- **ic -- Assignment 2**, unstarted: surface-review cc's `ac new` / `at new` refuse-and-edit package. **A spot check covered presence and framing only** -- not the refusal's boundaries, not the `at new` side, not whether the two doors agree.
- **vc -- ST0068's two remaining own-hands criteria**, AC-02.1 (walkthrough) and AC-02.3 (defect coverage). AC-03.1/03.2 wait on Laksa; AC-04.2 waits on the tag.
- **vc -- 0136's ~44-site `AcState::Computed` change**, after the tag. One-commit-or-split is called at the cut.
- **dc -- no smoke arm exercises `claude start` / `ws`.** ARM 4 proves the rule library arms; there is no equivalent for this door, so **present is the strongest claim the keg fix earns.** Needs a keg.

### Estate-wide, found and not owned

- **Intent's shipped gate output cites bare issue numbers** -- "issues 0036/0043" -- and `intent issues` is per-project, so a reader in a consuming estate resolves their OWN 0036 and 0043 and both read as verified because they resolved. hv's standing rule is `<project>#NNNN`. **Our gate violates it into every consumer.**
- **No toolchain pin in any form**, and `rust-toolchain.toml` would bind CI only while reading as a project-wide guarantee (`rust.yml` records the reasoning). **NOT implicated in the 2026-08-29 gate failure -- measured and refuted, not assumed:** one rust in the Cellar, installed three days before the last green push. **Do not cite that incident as evidence for a pin.**
- **0142's structural half is owed:** refusals have no declared home in the register, so the only place to state one is a `help` string, where nothing checks it and nothing updates it when the behaviour moves.
