# RULES.md

Project-specific rules for Intent. Extends -- never replaces -- the four agnostic principles enforced across every Intent project.

## Canon (cross-language)

Every Intent project enforces these. Full rule files at `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure-Functional-Idiomatic-Coordination; pattern match, pipe, tag, compose.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

The terse DO / NEVER summary lives in `usage-rules.md` at the project root. Language-specific concretisations live at `intent/plugins/claude/rules/<lang>/` (`elixir`, `rust`, `swift`, `lua`, `shell` ship in canon).

## Intent dev rules

These extend the canon for Intent itself (a Bash CLI). Each one concretises one of the four agnostic principles for the bash + plugin reality of this codebase.

1. **Module Highlander check** (concretises `IN-AG-HIGHLANDER-001`) -- before creating any new module, helper, or template, consult `intent/llm/MODULES.md`. If a row already covers the concern, extend the existing module instead of creating a new one.
2. **Register before you code** (concretises `IN-AG-HIGHLANDER-001`) -- when a genuinely new module is required, add the row to `MODULES.md` first, then create the file. The registry is canonical, not retrospective.
3. **Thin scripts** (concretises `IN-AG-THIN-COORD-001`) -- business logic lives in dedicated modules under `bin/` or `intent/plugins/`, never inline in command dispatch or heredocs. Dispatch, parse, call, render. Anything else is a code smell.
4. **No silent failures** (concretises `IN-AG-NO-SILENT-001`) -- every error path uses `error()` from `bin/intent_helpers`. Background discards (`2>/dev/null`) only when the failure is genuinely informational (probing optional dependencies, for example); never to hide a real fault.
5. **Single template source** (concretises `IN-AG-HIGHLANDER-001`) -- all generated content originates from `lib/templates/` via `sed` substitution. Inline heredocs that duplicate template content are a Highlander violation; refactor to read the template.
6. **Fail-forward migrations** (concretises `IN-AG-NO-SILENT-001`) -- migrations actively prune deprecated artefacts. No backwards-compat shims, no preservation stubs, no commented-out code. Old code is deleted, not parked.

## Bash environment constraints

- macOS bash 3.x compatibility -- no `declare -A`, no `${VAR^}` case modifiers, no `mapfile` / `readarray`. Use explicit alternatives.
- BSD `mktemp` differs from GNU `mktemp` -- prefer `mktemp -d` with no template, or pass an absolute template path with at least three `X`s.
- `set -euo pipefail` is the default; helpers that legitimately tolerate non-zero exits use `|| true` explicitly so the intent is visible.
- 2-space indentation in all bash scripts. No tabs anywhere in tracked files.

## Markdown discipline

- NEVER manually wrap lines. Markdown has no need for hard wraps; the linter (prettier on commit) reflows content as needed.
- Tables must be column-aligned in source. The linter enforces this; treat it as a one-way ratchet, not a debate.
- No emojis in canonical files unless the user explicitly requests them. No em dashes in skill files (multi-byte truncation bug in Claude Code's list display).

## Testing discipline

- Every `bin/` and plugin module lives behind one or more BATS scenarios in `tests/unit/`. New behaviour without a test is incomplete.
- Tests assert on behaviour and deltas, not absolute counts. `assert_count == 781` is brittle; `assert_passing > before_count` is durable.
- Test fixtures isolate `HOME` and `TEST_TEMP_DIR` so installed user state cannot bleed in. See `tests/lib/test_helper.bash` for the shared setup pattern.

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
