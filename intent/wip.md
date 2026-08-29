---
verblock: "29 Aug 2026:v1.28: vc - DOING and TODO only; done work removed"
intent_version: 3.0.0
---

# Work In Progress

**DOING and TODO only. Completed work does not belong in this file** -- it lives in `CHANGELOG.md`, `docs/releases/`, and the threads' own closed criteria, and a fourth copy here drifts from all three.

**Current as at `1d5adfe1`. This names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

    intent ac gate ST0056     # the release gate
    intent ac gate ST0058     # feeder
    intent ac gate ST0066     # feeder
    intent ac gate ST0068     # feeder -- the docs
    intent doctor

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now, and they moved twice today.

## DOING

**v3.0.1 IS THE NEXT RELEASE AND IT IS NOT CUT.** hv ruled the scope: **ST0056 and its feeders -- ST0058, ST0066, ST0068 -- and nothing wider.** `intent vault` (ST0060), modules (ST0046) and `intentd` (ST0064) are OUT as new features rather than parity; ST0064 has zero criteria, so including it would have gated the release on unspecified work. **hv also ruled the number stays `3.0.1`** with the cost stated and taken: it carries `ac edit`, `at edit`, `issues edit` and an `st edit` behaviour change, so a reader upgrading from 3.0.0 gets four new verbs.

**THE DOCS SHIP WITH THE TAG, NOT AFTER IT.** That is what makes ST0056's AC-12.3 satisfiable at all. `docs/` is the v3 set -- install, getting started, concepts, working with agents, migration, and a generated command reference; `docs/v2/` is the frozen v2 archive. **Written against the CUT, never against `main`.**

**THE SITE IS BEING BUILT AT LAKSA.** Kickoff sent on hv's instruction. Spec is `docs/design/design-system.md`; laksa-cc and laksa-ic build, laksa-vc validates. **Its section 11 is a register of eight OPEN design decisions that go to the Laksa design agent, not back here.**

## TODO

### For hv

- **0143 -- was dropping `--skip-settings` deliberate?** v2 could decline `.claude/settings.json` and the hook scripts; v3 has no equivalent, so a project that wants Intent without Claude Code lifecycle hooks cannot say so. Under fail-forward, removing a flag is legitimate **when it is a decision**; nothing found records this as anything but a port-time omission.
- **The ratified Guard column is not a vocabulary, so the machine-table instrument's axis C cannot gate.** Entry states and edges gate exactly; the Guard cells hold effects and landing rules rather than preconditions. Giving the column a controlled vocabulary is an edit to a ratified table. S either way.
- **The parked stack**, unchanged: mechanical window refusal; instruments placement; `publish_home` temp root; the vacuous `doctor` remedy; the ratified-surface pile; Conflab's four contract-prose edits; Lamplight md-to-store; Laksa's `DESCOPED` token; `issues list` holdout and `--status a,b` ordering; ST0066 minutia 3.

### Owed, by owner

- **cc -- hv's `info.md` round-trip.** Ruled and recorded, **not started**; hv stood cc down for the day. `st edit` opens `info.md` and `sync --to-store` reads `## Objective` and `## Context` back. **Binding condition: renderer and reader share ONE declaration of which sections are authored, and read-back ignores every other byte rather than parsing it** -- with an arm that plants a byte in a region neither authority owns and asserts it survives the round trip. Five open issues in this estate are prose destroyed by a reader that was only supposed to read. **cc's own sizing is withdrawn as unmeasured: `legacy::scan` already parses those four fields, but it is the v2 migration parser aimed at v2's shape. Measure the gap before costing this.**
- **ic -- Assignment 2**, unstarted: surface-review cc's `ac new` / `at new` refuse-and-edit package. **A spot check covered presence and framing only** -- not the refusal's boundaries, not the `at new` side, not whether the two doors agree.
- **vc -- three `st edit` defects**, routed to cc. The register's DEFAULT for a path-printer is the one file it must refuse; the refusal's remedy names `intent st` and **no `intent st` verb writes `objective` or `context`**; and **`st edit <unknown-id>` never reports an unknown thread**, giving two different wrong stories depending on the file argument. The third first.
- **vc -- 0136's ~44-site `AcState::Computed` change**, after the tag. One-commit-or-split is called at the cut.

### Estate-wide, found today and not owned

- **Intent's shipped gate output cites bare issue numbers** -- "issues 0036/0043" -- and `intent issues` is per-project, so a reader in a consuming estate resolves their OWN 0036 and 0043 and both read as verified because they resolved. hv's standing rule is `<project>#NNNN`. **Our gate violates it into every consumer.**
- **No toolchain pin in any form.** No `.tool-versions`, and no `rust-toolchain.toml` either -- the Cargo-native one, and the only one `cargo` would honour. The build takes whatever `rustc` is on PATH.
- **0142's structural half is owed:** refusals have no declared home in the register, so the only place to state one is a `help` string, where nothing checks it and nothing updates it when the behaviour moves.
