---
verblock: "27 Apr 2026:v0.2: matts - Phase 0 elaboration"
wp_id: WP-19
title: "Per-language canon: intent lang command + intent init --lang flag"
scope: Medium
status: Not Started
---

# WP-19: Per-language canon -- `intent lang` command + `intent init --lang` flag

## Objective

Replace the canon-installer's (now-removed) auto language detection with explicit, user-driven language setup. Provide a new `intent lang init <lang>` subcommand that idempotently installs a language's canon defaults into a project, and an `intent init --lang <lang1>,<lang2>,...` flag that invokes it for each named language sequentially during fresh project initialisation.

## Context

ST0036/WP-08 surfaced two facts during the canon-installer remediation pass:

1. The canon-installer's hard-coded "Elixir template" path (`intent/plugins/agents/templates/elixir/RULES.md`, `ARCHITECTURE.md`) is wrong-shaped for non-Elixir projects. Intent itself (a bash CLI) got Elixir templates installed.
2. A first-cut "auto-detect the project language" patch was tried (`detect_project_language`, single-token return). It failed the multi-language reality test: most real projects are polyglot (Elixir + Swift + Rust + Lua + Bash + HTML/CSS/JS content). Picking any single "primary" doesn't reflect the truth and would just shift the wrong-shape problem.

The ST0036/WP-08 fix landed the **always-`_default`** canon-installer behaviour: canon-installer now installs language-agnostic `_default` templates for `RULES.md`, `ARCHITECTURE.md`, and (when missing) `AGENTS.md`. This sidesteps the language question entirely at canon-install time.

WP-19 delivers the proper home for language-specific setup: an explicit `intent lang init <lang>` command that the user invokes (or that `intent init --lang <langs>` invokes on their behalf at fresh-init time). Re-runs are safe (idempotent). Multi-language is the default case.

## Design (from user direction, 2026-04-27)

```
intent lang init <lang> [<lang2> ...]      # idempotent per-language canon install
intent lang list                            # which languages have canon templates available
intent lang show <lang>                     # what does intent lang init <lang> install?
intent init --lang <lang1>,<lang2>,...      # invokes intent lang init for each on init
```

`intent lang init <lang>` is the workhorse. Per-language responsibilities (concretised by each language's pack):

- Install `intent/llm/RULES-<lang>.md` from `intent/plugins/agents/templates/<lang>/RULES.md` (existing language-specific templates -- Elixir already has one; Rust/Swift/Lua/Shell to add or stub).
- Install `intent/llm/ARCHITECTURE-<lang>.md` if a language-specific architecture template exists (Elixir already has one).
- Append a "Language Pack" section to `intent/llm/RULES.md` (generic \_default file) listing the language and pointing at its rule pack at `intent/plugins/claude/rules/<lang>/`.
- Set up any language-specific dependency hooks (e.g. for Elixir: copy the Credo check templates from `lib/templates/credo_checks/elixir/` into `.credo.exs`-aware locations; for Rust/Swift/Lua: equivalent if applicable).
- Idempotent: re-running with the same language is a no-op (or refresh-with-preservation for files with user-section markers, mirroring the canon-installer pattern).

`intent init --lang <lang1>,<lang2>,...` runs `intent lang init <langN>` for each language in order. No auto-detection. The user names what they want. Default (no `--lang`) is "no per-language setup" -- the project gets the agnostic canon and that's it.

## Deliverables

1. **`bin/intent_lang`** -- new CLI command. Subcommands: `init`, `list`, `show`, `help`. Dispatcher follows existing `bin/intent_*` patterns.
2. **`intent_lang_init` helper** in `bin/intent_lang` (or extracted to `bin/intent_helpers` if shared). Per-language idempotent installer.
3. **`bin/intent_init`** updated -- adds `--lang <list>` parsing; invokes `intent lang init` for each named language post-init.
4. **`intent/plugins/agents/templates/<lang>/`** filled out for the five canon languages: `elixir/` (already exists), `rust/`, `swift/`, `lua/`, `shell/`. Each gets `RULES.md` + `ARCHITECTURE.md` minimum. Existing Elixir templates serve as the reference.
5. **`intent/llm/RULES.md`** template (the agnostic `_default` one from WP-08) gains a "Language Packs" placeholder/section; `intent lang init` appends to it.
6. **MODULES.md** -- register `bin/intent_lang` and the new per-language template files.
7. **Help files** -- `lib/help/lang.help.md` covering all subcommands.
8. **BATS** coverage:
   - `tests/unit/intent_lang.bats` -- per-subcommand scenarios + idempotence + multi-lang invocation.
   - `tests/unit/intent_init_lang_flag.bats` -- `intent init --lang` end-to-end.

## Approach

### Phase A: Command + helper scaffolding

1. Create `bin/intent_lang` dispatcher (init/list/show/help subcommands).
2. Wire it into `bin/intent` main dispatch.
3. Add `lib/help/lang.help.md`.
4. MODULES.md registration.

### Phase B: Per-language template scaffolding

For each of {rust, swift, lua, shell}: stub `RULES.md` + `ARCHITECTURE.md` in `intent/plugins/agents/templates/<lang>/`. Modeled on the existing `_default/` and `elixir/` templates. Initial content: header + pointer at the language's rule pack at `intent/plugins/claude/rules/<lang>/`. Per-language deep content fills in over time.

### Phase C: `intent lang init` core

1. Implement the install loop: for each named language, install `RULES-<lang>.md` and `ARCHITECTURE-<lang>.md` if their templates exist; append a Language Pack entry to the agnostic `intent/llm/RULES.md`.
2. Idempotence: re-run produces no diff (file checksums + section-marker preservation).
3. Multi-lang invocation: `intent lang init elixir rust shell` runs all three.

### Phase D: `intent init --lang` integration

1. Parse `--lang <comma-or-space-separated>` in `bin/intent_init`.
2. After init completes, invoke `intent lang init <each>` sequentially.
3. Failure of one language's init does not abort the others (fail-forward); summary at the end reports per-language status.

### Phase E: Tests + docs

1. BATS scenarios for each Phase B/C/D behaviour.
2. Update `intent/docs/working-with-llms.md` with the multi-language story (replaces any auto-detection language).
3. Cross-link from `RULES.md` template documentation.

## Acceptance Criteria

- [ ] `intent lang` subcommand registered and discoverable via `intent help`.
- [ ] `intent lang init <lang>` idempotent: re-run produces zero diff.
- [ ] `intent lang init lang1 lang2 lang3` runs all three; no order-dependent failures.
- [ ] All five canon languages have at least stub `RULES.md` + `ARCHITECTURE.md` templates in `intent/plugins/agents/templates/<lang>/`.
- [ ] `intent init --lang elixir,rust` end-to-end: project initialised, both languages installed, agnostic `intent/llm/RULES.md` lists both as Language Packs.
- [ ] `intent init` with no `--lang` flag works as today (no per-language setup; agnostic canon only).
- [ ] No auto-detection anywhere in canon-installer or init flow. User explicitly names languages.
- [ ] BATS coverage for every subcommand + the `--lang` flag (~10-15 new scenarios).
- [ ] `bin/intent_lang` registered in MODULES.md.
- [ ] `lib/help/lang.help.md` exists and covers all subcommands.
- [ ] `intent/docs/working-with-llms.md` updated with the explicit-language-setup story.

### Tests to add / update

- `tests/unit/intent_lang.bats` (new) -- ~8 scenarios.
- `tests/unit/intent_init_lang_flag.bats` (new) -- ~4 scenarios.
- Existing `tests/unit/intent_claude_upgrade.bats` -- already covers "canon-installer uses \_default" (WP-08); no changes needed.

## Dependencies

- **Blocked by**: ST0036/WP-08 (always-`_default` canon-installer behaviour must be in place; that's what makes per-language setup an opt-in concern rather than the default). WP-08 ships in this same conversation.
- **Blocks**: nothing in ST0035/ST0036 explicitly. Fleet rollout (ST0035/WP-15..17) does NOT depend on WP-19 -- canary projects continue to work fine without per-language setup; users opt in per-project as desired.

## Implementation Notes

- **Why a separate `intent lang` command rather than overloading `intent claude upgrade`?** Different lifecycle: `intent claude upgrade` is the canon-refresh path (run on every Intent version bump); `intent lang init` is the "I want this language's canon installed" path (run once per language per project, then idempotent re-runs). Bundling would conflate two distinct concerns.
- **Why no auto-detection?** (See user feedback on 2026-04-27.) Real projects are polyglot: Elixir + Swift + Rust + Lua + Bash + HTML/CSS/JS content. Picking one "primary" misrepresents the project shape. Explicit user choice via `--lang` flags is more honest and avoids the wrong-shape problem.
- **Idempotence pattern**: same as canon-installer's `canon_template_matches_installed` -- compare installed file checksum against template (with placeholder substitution); skip if identical. For files with user-section markers, refresh-with-preservation.
- **Fallback on missing language template**: if `intent lang init nonexistent` is called, error cleanly with a list of supported languages. Does not auto-create.
- **Cross-cutting with `intent lang list`**: enumerates the directories under `intent/plugins/agents/templates/` (excluding `_default/`).

## Risks and Edge Cases

- **Risk**: User runs `intent lang init <lang>` on a project that already has `intent/llm/RULES-<lang>.md` from a previous `--template <lang>` `intent init`. Mitigation: idempotence via checksum match; refresh-with-preservation if user-section markers exist.
- **Risk**: Multi-lang invocation has order-dependent state (one language's install reads what another wrote). Mitigation: each `intent lang init <lang>` is self-contained; no cross-language state. Append-only changes to `intent/llm/RULES.md` for the Language Pack section.
- **Edge**: `intent init --lang` with no value (`intent init --lang `) or empty list. Mitigation: treat as "no `--lang` specified".
- **Edge**: `intent init --lang foo,bar` where `bar` is invalid. Mitigation: install `foo`, error on `bar`, summary reports.
- **Edge**: per-language templates that need to merge with non-canon project files (e.g. Elixir's Credo checks need to land in `.credo.exs` -- which may be user-edited). Mitigation: scope WP-19 to Markdown templates only; defer Credo/build-tool integration to a follow-up if needed.

## Verification Steps

1. `intent help` lists the `lang` subcommand.
2. `intent lang init elixir` on a fresh project: `intent/llm/RULES-elixir.md` + `intent/llm/ARCHITECTURE-elixir.md` installed; `intent/llm/RULES.md` Language Packs section lists Elixir.
3. Re-run `intent lang init elixir`: zero diff.
4. `intent lang init rust shell`: both installed; `intent/llm/RULES.md` lists Rust + Shell + Elixir.
5. `intent init --lang elixir,rust` on a fresh dir: project initialised + both languages installed.
6. BATS suite runs green.

## Size and Estimate

- **Size**: M (Medium). ~2-3 sessions.
  - Session 1: dispatcher + help + MODULES.md + Phase A scaffolding.
  - Session 2: per-language template stubs (Phase B) + `intent lang init` core (Phase C) + tests.
  - Session 3: `intent init --lang` integration (Phase D) + remaining tests + docs (Phase E).

## Exit Checklist

- [ ] `bin/intent_lang` exists and dispatches subcommands.
- [ ] `intent lang init <lang>` works for all five canon languages.
- [ ] Idempotent (zero diff on re-run).
- [ ] `intent init --lang` integration end-to-end.
- [ ] BATS coverage for all subcommands + the flag.
- [ ] MODULES.md updated.
- [ ] Help file in place.
- [ ] `intent/docs/working-with-llms.md` updated.
- [ ] Committed: `feat: ST0035/WP-19 per-language canon command + init --lang flag`.
