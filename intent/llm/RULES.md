# RULES.md

Project-specific rules for Intent. Extends -- never replaces -- the agnostic principles enforced across every Intent project.

## Canon (cross-language)

Every Intent project enforces these. Full rule files at `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure Function, Impure Coordination. Read it with `intent claude rules show IN-AG-PFIC-001`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

The terse DO / NEVER summary lives in `usage-rules.md` at the project root. Language-specific concretisations live at `intent/plugins/claude/rules/<lang>/`; `intent claude rules list --lang <lang>` enumerates a pack and `intent lang list` names the packs a project can declare.

## Intent dev rules

These extend the canon for Intent itself (a Rust workspace at `native/rust/`, plus the shell assets under `lib/templates/`). Each one concretises one of the agnostic principles for this codebase.

1. **Module Highlander check** (concretises `IN-AG-HIGHLANDER-001`) -- before creating any new module, helper, or template, consult `intent/llm/MODULES.md`. If a row already covers the concern, extend the existing module instead of creating a new one.
2. **Register before you code** (concretises `IN-AG-HIGHLANDER-001`) -- when a genuinely new module is required, add the row to `MODULES.md` first, then create the file. The registry is canonical, not retrospective.
3. **Thin coordinators** (concretises `IN-AG-THIN-COORD-001`) -- behaviour lives in `intentsvcs`; `intent-cli`'s `render.rs` parses, calls the facade, and renders. Dispatch, parse, call, render. Anything else is a code smell.
4. **No silent failures** (concretises `IN-AG-NO-SILENT-001`) -- every error is typed and carries a remedy (`intentsvcs::remedy::Remedy`), rendered as an `error:` line and a `remedy:` line. In the shell assets, background discards (`2>/dev/null`) only when the failure is genuinely informational (probing optional dependencies, for example); never to hide a real fault.
5. **Single template source** (concretises `IN-AG-HIGHLANDER-001`) -- all generated content originates from `lib/templates/`, expanded in Rust (`rootfiles::substitute` for the root files; `init` fills the templates it seeds). Literals that duplicate template content are a Highlander violation; refactor to read the template.
6. **Fail-forward migrations** (concretises `IN-AG-NO-SILENT-001`) -- migrations actively prune deprecated artefacts. No backwards-compat shims, no preservation stubs, no commented-out code. Old code is deleted, not parked.

## Bash environment constraints

- macOS bash 3.x compatibility -- no `declare -A`, no `${VAR^}` case modifiers, no `mapfile` / `readarray`. Use explicit alternatives.
- BSD `mktemp` differs from GNU `mktemp` -- prefer `mktemp -d` with no template, or pass an absolute template path with at least three `X`s.
- `set -euo pipefail` is the default; helpers that legitimately tolerate non-zero exits use `|| true` explicitly so the intent is visible.
- 2-space indentation in all bash scripts. No tabs anywhere in tracked files.

## Markdown discipline

- NEVER manually wrap lines. Markdown has no need for hard wraps. The pre-commit hook runs `prettier --check` on staged Markdown and refuses the commit until `prettier --write` has been run.
- Tables must be column-aligned in source. The linter enforces this; treat it as a one-way ratchet, not a debate.
- No emojis in canonical files unless the user explicitly requests them. No em dashes in skill files (multi-byte truncation bug in Claude Code's list display).

## Testing discipline

- Every change under `native/rust/` lands with a Rust test (a colocated `#[cfg(test)]` module or `native/rust/crates/<crate>/tests/`); shell assets sit behind BATS scenarios in `tests/unit/`. New behaviour without a test is incomplete.
- Tests assert on behaviour and deltas, not absolute counts. `assert_count == <N>` is brittle; `assert_passing > before_count` is durable.
- Test fixtures isolate `HOME` and `TEST_TEMP_DIR` so installed user state cannot bleed in. See `tests/lib/test_helper.bash` for the shared setup pattern.

## Instrument discipline -- classifying a population defect

When an instrument misses a defect it should have caught, the population is wrong in one of the ways below -- the question is too weak, the selector is too narrow, or a source is missing -- and **each wants a different remedy. Reaching for one remedy across them is the mistake this section exists to prevent.** Run the questions in order:

1. **Can the defect be exhibited by an artefact the instrument ALREADY READS?** Then the population is right and complete, and the QUESTION is too weak. **Deepen the predicate. Add no source.** This is the cheapest class to fix and the hardest to notice, because every count the instrument prints is correct. Worked case: `bin/.devbin/cmd/canon` compared `git ls-files` against the disk listing, which answers MEMBERSHIP; a tracked canon file whose committed body differed from the store passed clean, and a clone got the stale body.
2. **If not -- is the missing artefact the SAME KIND as those in the population, merely outside the selector?** Then the selector is narrow. **Declare the boundary and emit the complement.** Do NOT simply widen it: a reach that can be widened silently is a reach that was never declared, and moving the boundary while keeping the silence reproduces the defect one directory over. Worked case: `of_n_labels_its_derivation.sh` defaulted its population to its own directory and `.sh` only while its docstring claimed every instrument emitting an `N of M` at all; the two limits are INDEPENDENT, so widening the directory would still miss extensionless commands.
3. **Neither?** Then a source is missing outright -- a corpus nobody is reading names things no existing source names. **This is the expensive one; price it before starting.** Worked case: `uninstall --all` was named by the BATS suite and by canon nowhere, so a population drawn from canon could not see it, and neither deepening nor widening the canon-versus-table pair reached it.

**What they share is not a source, it is an OBLIGATION**: in each, an instrument's declared reach exceeded its actual reach and nothing said so. That is `ST0056 AC-00.16`. One criterion, a remedy per case.

**AND WHEN TWO EXTRACTIONS AGREE, YOU HAVE TESTED NEITHER.** Agreement between a wrong instrument and a right one is the standard validation move and it confirms the wrong answer. Measured 2026-09-04: grepping `surface/dispatch-table.json` for `"--x"` string literals reads the table's PROSE rather than its declarations, and returns the same count as the correct extraction over `flags[].spellings[]`. **Ask what MUST be in the answer and look for it** -- that census was caught only because `--daemon` was missing from a list it had to be in.

## Commit discipline

- T-shirt sizing only (XS / S / M / L / XL / XXL). No clock-time estimates.
- Commit messages explain the why, not the what. The diff is authoritative for "what".
- No Claude attribution. No `Co-Authored-By` lines for AI assistance. Author the commit; that is the contract.

## Pointers

- Module registry: `intent/llm/MODULES.md`
- Code-placement flow chart: `intent/llm/DECISION_TREE.md`
- Architecture overview: `intent/llm/ARCHITECTURE.md`
- Critic dispatch contract: `intent/docs/critics.md`
- Rule-library authoring: `intent/docs/rules.md`
- Extension authoring: `intent/docs/writing-extensions.md`
