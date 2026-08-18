# Design - ST0044: Add in acceptance.md and supporting process

## Approach

The build splits into three layers:

1. Doc + template -- `acceptance.md` as a steel-thread document, sourced from `lib/templates/`.
2. CLI instrumentation -- the `intent ac` / `intent at` command family that reads and mutates the AC/AT records.
3. Gates + process -- the open-gate and close-gate wired into the lifecycle, and the five-step loop mapped onto the skill set.

Layer 1 turned out to be trivial: the `intent st new` doc-set stamping loop (`bin/intent_st`, around line 394) globs `"$TEMPLATE_DIR"/*.md` and substitutes placeholders into each file. Dropping `acceptance.md` into `lib/templates/prj/st/ST####/` makes every new ST carry it -- no code change, on by default. The substance is layers 2 and 3 (instrumentation + gates).

## Design Decisions

- Default-on stamping (owner decision). `acceptance.md` ships in the default doc-set from now on. There is no feature flag and no prove-before-flip gate: the template is a plain file picked up by the existing `*.md` glob, refined in place if the shape needs to change. Editing the template later only affects future STs -- low blast radius.
- AC and AT are separate axes. AC = coverage ("is the whole thing there?"); AT = proof ("does each part work?"). Tracked separately because the agent reliably passes proof and fails coverage.
- Mutation applies to `at`, not `ac`. AC satisfaction is computed from AT coverage, never hand-set. The single carve-out is non-test ACs, satisfied by named evidence via `intent ac satisfy ... --evidence`.
- AT is a state machine: `to-write -> red -> green` (+ `n/a` for non-test). The CLI guards transitions -- `green` is reachable only from `red` -- so red-first witnessing is encoded as mechanism, not prose. `done` / `notdone` survive as aliases for `green` / `red`.
- The close-gate is a new precondition on the existing `intent st done` / `intent wp done`, not a new command. Done is computed from coverage + recorded sign-off, never from a hand-ticked box.
- `acceptance.md` is the single source of truth. AC/AT lines are an addressable, greppable record grammar (bash 3.2; no `declare -A`), and in-place status edits must be linter-stable so the file does not churn.
- Highlander for ACs: one home (`acceptance.md`). `info.md` and per-WP `info.md` reference it and never restate ACs.
- Flat `ac` / `at` nouns alongside `st` / `wp`, matching the existing `intent <noun> <verb> <args>` grammar.

## Architecture

- Template source: `lib/templates/prj/st/ST####/acceptance.md` (single-template-source rule; no inline heredoc). Registered in `intent/llm/MODULES.md`.
- Stamping: no new code -- the `for template in "$TEMPLATE_DIR"/*.md` loop in `bin/intent_st` (around line 394) copies and substitutes it like the other doc-set files.
- Known consistency gap (follow-on, folded into WP-05): `st show <id> acceptance` and `st show <id> all` still hardcode `info|design|impl|tasks`, so the new doc is stamped but not yet viewable via those paths. Out of WP-01's witnessed ATs; flagged for WP-05.
- Parser + gate: a new module in `bin/` (eg `bin/intent_acceptance`) for WP-03/WP-04 that parses the AC/AT grammar, computes coverage, and enforces the transition guard and close-gate. Register in MODULES.md before writing it (project rule 5).
- Command dispatch: add `ac` / `at` nouns to the intent CLI dispatch; output uses the Rust-style prefixes (`ok:`, `error:`), no banners.
- Skills/process: the five-step maps onto existing skills -- `/in-plan` (open-gate), `/in-verify` (red-first witnessing), `/in-review` (two-stage), `/in-finish` + `intent done` (close-gate). Whether to add a dedicated `/in-acceptance` skill or thread the steps through the existing ones is an open fork.

## Alternatives Considered

- A default-off feature flag / env-var seam for the stamp: built first, then dropped. The stamping loop globs `*.md`, so there was nothing to gate -- the file's presence is the switch. Owner chose default-on.
- Binary `done` / `notdone` as the sole AT interface: rejected -- it cannot represent `red`, so it drops the red-first witnessing that is the feature's reason to exist. Retained as aliases over the state setter.
- A separate `acceptance` noun, or nesting under `st`: rejected for verbosity; flat `ac` / `at` chosen.
- CLI runs the cited test to derive red/green (`intent at run`): deferred. Cross-language test execution (bats here, ExUnit/cargo/swift elsewhere) makes it a later iteration; v1 uses manual states plus the red-first transition guard.

## Reconciliation

Written against `info.md` as of this session; ST0044's spec may still be revised externally -- re-read and reconcile when a newer version lands. Retrofit is scoped out: existing closed steel threads do NOT get `acceptance.md`; only the two open threads (ST0043, ST0044) got it, by hand. New STs get it from the template. So there is no upgrade-driven per-ST retrofit, and ST0044 does not couple to ST0043 on that axis.
