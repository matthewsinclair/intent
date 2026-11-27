# RULES.md

Project-specific rules for Intent. They extend the agnostic principles and never replace them.

## Canon (cross-language)

The cross-language principles (Highlander, PFIC, Thin Coordinator and No Silent Errors) are in `AGENTS.md` under "Rules of the Road", with their rule IDs. The agnostic pack also carries two procedural rules: `IN-AG-RED-CONTROL-001` (a control is only a control if it can go red) and `IN-AG-FIAT-001` (fiat close is the human's verb). `intent claude rules list --lang agnostic` lists the pack, and `intent claude rules show <id>` reads a rule.

The terse DO / NEVER summary is in `usage-rules.md` at the project root. Language-specific rules are at `intent/plugins/claude/rules/<lang>/`. `intent claude rules list --lang <lang>` lists a pack, and `intent lang list` names the languages a project can declare.

## Intent dev rules

These extend the canon for Intent itself: a Rust workspace at `native/rust/`, the Swift menubar app at `native/macos/`, the shell assets under `lib/templates/` and the bash dev tooling under `bin/`.

1. **Module Highlander check** (concretises `IN-AG-HIGHLANDER-001`): before you create a module, helper or template, search the registry with `intent modules find <name>`. If a row already covers the concern, extend that module.
2. **Register before you code** (concretises `IN-AG-HIGHLANDER-001`): when a new module is necessary, add its row to `intent/llm/MODULES.md` first, then create the file.
3. **Thin coordinators** (concretises `IN-AG-THIN-COORD-001`): behaviour lives in `intentsvcs`. `intent-cli`'s `render.rs` parses, calls the facade and renders.
4. **No silent failures** (concretises `IN-AG-NO-SILENT-001`): every error is typed and carries a remedy (`intentsvcs::remedy::Remedy`), rendered as an `error:` line and a `remedy:` line. In the shell assets, discard stderr (`2>/dev/null`) only when the failure is informational, eg a probe for an optional dependency.
5. **Single template source** (concretises `IN-AG-HIGHLANDER-001`): all generated content comes from `lib/templates/`, expanded in Rust (`rootfiles::substitute` for the root files; `init` fills the templates it seeds). A literal that duplicates template content is a Highlander violation: read the template.
6. **Fail-forward migrations** (concretises `IN-AG-NO-SILENT-001`): migrations prune deprecated artefacts. No backwards-compatibility shims, no preservation stubs, no commented-out code. Old code is deleted.

## Bash environment constraints

- The shell assets under `lib/templates/` run on any machine's `/bin/bash`, so they keep to bash 3.2 (`tests/unit/templates_bash32.bats` holds this): no `declare -A`, no `${VAR^}` case modifiers, no `mapfile` / `readarray`. The dev tooling under `bin/.devbin/` requires bash 5 and is outside this rule.
- BSD `mktemp` differs from GNU `mktemp`: use `mktemp -d` with no template, or pass an absolute template path with at least three `X`s.
- The shell assets run under `set -u`, and some add `-o pipefail`. A helper that tolerates a non-zero exit says so with `|| true`.
- 2-space indentation in all bash scripts. No tab indentation in tracked files Intent authors, except a literal tab used as a field delimiter, TSV data and Xcode-generated project files.

## Markdown discipline

- NEVER wrap lines by hand. The pre-commit gate runs `prettier --check` on the staged Markdown and refuses the commit until it is formatted (`bin/devbin fmt md <path>`).
- Tables are column-aligned in source. prettier aligns them.
- No emojis in canonical files unless the user asks for them. No em dashes in skill files: Claude Code's list display truncates multi-byte characters.

## Testing discipline

- Every change under `native/rust/` lands with a Rust test (a colocated `#[cfg(test)]` module or `native/rust/crates/<crate>/tests/`). Shell assets have BATS scenarios in `tests/unit/`. New behaviour without a test is incomplete.
- Tests assert on behaviour and deltas, never on a count written down by hand. Derive an expected number from the thing it counts.
- Test fixtures isolate `HOME` and `TEST_TEMP_DIR` so installed user state cannot leak in. `tests/lib/test_helper.bash` has the shared setup.

## Instrument discipline: classifying a population defect

When an instrument misses a defect it should have caught, its population is wrong in one of three ways, and each needs a different remedy. Ask the questions in order:

1. **Can an artefact the instrument already reads show the defect?** Then the population is right and the QUESTION is too weak. Deepen the predicate and add no source. This class is the cheapest to fix and the hardest to see, because every count the instrument prints is correct.
2. **If not, is the missing artefact the same kind as those in the population, outside the selector?** Then the selector is too narrow. Declare the boundary and emit the complement. Do not only widen it: a reach that can widen silently was never declared.
3. **Neither?** Then a source is missing. This class is the expensive one: price it before you start.

In each class, the instrument's declared reach exceeded its actual reach and nothing said so. When two extractions agree, you have tested neither: ask what MUST be in the answer, and look for it.

## Commit discipline

- A commit message explains the why. The diff is authoritative for the what.

## Pointers

- Module registry: `intent/llm/MODULES.md`
- Code-placement flow chart: `intent/llm/DECISION_TREE.md`
- Architecture overview: `intent/llm/ARCHITECTURE.md`
- Critic dispatch contract: `intent/docs/critics.md`
- Rule-library authoring: `intent/docs/rules.md`
- Extension authoring: `intent/docs/writing-extensions.md`
