# Design - ST0037: Language config: replace filesystem probes with explicit config

## Decisions locked in

1. **Field shape.** `languages` field in `intent/.config/config.json`, type `array<string>`. Lowercase canonical names: `elixir`, `rust`, `swift`, `lua`, `shell`. Default empty for new projects (see decision 4).
2. **Polyglot ordering.** Array order is the explicit declaration. First entry is the primary where a primary is needed (e.g. for a TCA-audit dispatch where one critic gets called first); for the pre-commit gate, all entries fire equally. Documented up-front in the field's comment in the migration template.
3. **No detection. Anywhere. Ever.** Once this ST lands, no canon code reads filesystem markers (`mix.exs`, `Cargo.toml`, etc.) to decide language-in-use. Rule packs at `intent/plugins/claude/rules/<lang>/` continue to exist and continue to be loaded by the relevant critic subagents on demand -- the rules' _existence_ and the project's _declared languages_ are independent questions.
4. **Empty `languages: []` is valid.** Means no language-specific essentials are loaded; only `/in-essentials` and `/in-standards`. The pre-commit gate falls back to running `critic-shell` only against staged shell files (matching current "always include shell" behaviour, see migration back-fill rule).
5. **Migration back-fill rule.** `migrate_v2_10_x_to_v2_11_0` runs at first `intent` invocation post-upgrade. If `languages` is absent, it adds it and back-fills:
   - For each `intent/llm/RULES-<lang>.md` present, append `<lang>` (in alphabetical order so the migration is deterministic).
   - If the back-filled set is empty AND `lib/templates/hooks/pre-commit.sh` is installed, append `shell` (preserves current "shell always" behaviour).
   - The migration is idempotent: if `languages` is already present, no-op.
6. **Version bump: v2.11.0.** Schema migration is a breaking change in canon sense (config schema rev), even though the migration is automatic and silent.
7. **Phantom skills get stripped, not shipped.** `/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials` were promised in `in-session/SKILL.md` and `working-with-llms.md` but never authored. The rule library at `intent/plugins/claude/rules/<lang>/` plus the critic-`<lang>` subagent already cover the per-language coding-discipline path. The skill refs come out; the rule-pack pointer goes in.
8. **Scope of `intent lang remove <lang>`.** Removing a language: deletes `intent/llm/RULES-<lang>.md`, deletes `intent/llm/ARCHITECTURE-<lang>.md`, removes the marker-block entry from `intent/llm/RULES.md`, removes the language from `intent/.config/config.json` `languages`. All four operations idempotent.

## Polyglot order example

A Rust + shell project adds them in primacy order:

```bash
intent lang init rust         # languages: ["rust"]
intent lang init shell        # languages: ["rust", "shell"]
```

There is no `intent lang reorder`. If a user wants to change primacy, they remove and re-init in the desired order. This is the simplest form and avoids a fourth verb.

## Item-by-item plan

### Item 1: Schema

Add `languages: []` to the config-template emitted by `bin/intent_init`. The config template lives at `lib/templates/_default/intent.config.json` (verify path during implementation).

### Item 2: Migration

Add `migrate_v2_10_x_to_v2_11_0` to `bin/intent_helpers` alongside `migrate_v2_9_0_to_v2_10_0`. Same shape:

- Read current config schema rev.
- If already at v2.11 schema, no-op.
- Otherwise: load `intent/.config/config.json`, add `languages` field with back-filled value (per decision 5), write config back, bump schema rev.

### Item 3: `intent_init`

Update `bin/intent_init` so freshly-initialised projects get `languages: []`. If the user passes `--lang <lang>` (existing flag from ST0035), seed the array with that one entry.

### Item 4: `intent_lang init <lang>`

Existing flow (install RULES-<lang>.md, ARCHITECTURE-<lang>.md, marker-block entry) is preserved. Adds one step: append `<lang>` to `languages` array via the JSON helper. Idempotent -- if already present, no change.

### Item 5: `intent_lang remove <lang>`

New verb. Reverses Item 4: removes the marker-block entry, deletes the two RULES/ARCHITECTURE files, removes the entry from `languages`. Idempotent. Emits `removed: <lang>` on success or `noop: <lang> not present` if it was never installed.

### Item 6: `get_project_languages()` helper

In `bin/intent_helpers`. Reads `intent/.config/config.json` via the existing `get_config_field` helper, returns one language per line (newline-delimited). Returns 0 lines when `languages` is empty or absent. Used by all consumers (pre-commit hook, future scripts).

### Items 7-9: Skill probe replacements

`intent/plugins/claude/skills/in-session/SKILL.md` -- replace probe table with prose describing the config-driven flow. Mention that `/in-elixir-essentials` is the only per-language essentials skill currently shipped; the rule packs at `intent/plugins/claude/rules/<lang>/` cover Rust, Swift, Lua, and shell, and the critic-`<lang>` subagent applies them on demand.

`intent/plugins/claude/skills/in-review/SKILL.md` -- replace probe block with read-from-config dispatch.

`intent/plugins/claude/skills/in-tca-audit/SKILL.md` -- same.

### Item 10: pre-commit hook

`lib/templates/hooks/pre-commit.sh` -- replace probe block with:

```bash
LANGS=()
while IFS= read -r lang; do
  [ -n "$lang" ] && LANGS+=("$lang")
done < <(get_project_languages)
```

`get_project_languages` is sourced from `bin/intent_helpers` (already available to installed hooks per existing template).

### Item 11: BATS test rework

- `tests/unit/in_session_skill.bats` -- probe-table assertions out, prose-and-config assertions in.
- `tests/unit/pre_commit_hook.bats` -- probe assertions out, config-read assertions in. Use a scratch repo with seeded `languages` field.
- `tests/unit/intent_lang.bats` -- assert `intent lang init <lang>` writes the field; new tests for `intent lang remove <lang>`.
- `tests/unit/intent_init.bats` -- assert new projects get `languages: []`.

### Item 12: Docs

`intent/docs/working-with-llms.md` -- replace the probe-table section (lines ~298-303) with config-driven prose, mirroring the SKILL.md change.

### Item 13: Blog draft

`docs/blog/_drafts/####-claude-context-with-intent.md` -- rewrite the "Language-specific essentials" paragraph (currently describes "language detection runs against the project root") with the config-driven flow.

### Item 14: Strip phantom skill refs

In the same SKILL.md and working-with-llms.md edits, remove the four broken rows that point at `/in-{rust,swift,lua,shell}-essentials`. Replace with a single line acknowledging the rule-pack + critic-subagent path covers those languages.

## Risk register

- **Migration vs. fresh install symmetry.** A fresh `intent init` post-v2.11.0 must produce a config that is byte-identical (modulo dates) to a v2.10.x project after migration. Test path: install fresh v2.11, install v2.10.x and migrate, diff configs.
- **Config-file write atomicity.** `intent_lang init/remove` mutates `intent/.config/config.json`. Use the existing `set_config_field` helper if it exists; if not, write to a temp file and `mv` to ensure atomicity.
- **Pre-commit hook environment.** The hook runs in a fresh shell with `set -e`. `get_project_languages` and its dependency `get_config_field` must work without surrounding `intent` CLI state.
- **Polyglot fleet projects.** Before merging, dry-run the migration against three fleet projects with mixed language presence (Conflab is Elixir + Bash; Anvil is Elixir-only; Utilz is mixed). Confirm back-fill is correct in each.
- **`intent lang init` idempotence drift.** Existing implementation re-creates the marker-block entry every call. Adding the JSON write must not double-write. Test with a `for i in 1 2 3; do intent lang init elixir; done` script.

## Test plan

1. Unit tests (BATS) per Item 11.
2. Migration round-trip test: write a v2.10.x config (no `languages`), run migration, assert `languages` is back-filled correctly.
3. Fresh-install equivalence test: run `intent init` for a v2.11.0 project, assert `languages: []`. Run `intent lang init shell`, assert `languages: ["shell"]`.
4. `intent lang remove` test: install elixir + shell, remove shell, assert `languages: ["elixir"]` and `RULES-shell.md` gone.
5. Pre-commit gate end-to-end: scratch repo with `languages: ["shell"]`, stage a shell file with a known violation, commit, assert blocked. Same with `languages: ["elixir"]` and an Elixir file.
6. `intent doctor` clean post-migration in a real fleet project.

## Out of scope

- `intent lang reorder` -- see polyglot example, deferred.
- Shipping the four missing essentials skills. Decision 7: strip the refs, don't ship the placeholders.
- Cross-language critic dispatch from the rule library directly (skipping the subagent layer). Architecturally adjacent but a separate concern.
- Migration that prunes orphan `RULES-<lang>.md` files (e.g. files present without a `languages` entry). Migration only adds; it does not remove. If a project has stale rule files, `intent lang remove` is the cure.

## Alternatives Considered

- **Option A (rejected): Auto-write the languages field at session start by detecting filesystem markers.** Same regression in different clothing. The user's design intent: explicit declaration only.
- **Option C (rejected): Use a top-level `languages.toml` or similar separate file.** No reason to invent a new file when `intent/.config/config.json` exists and is already the per-project config home.
- **Shipping `/in-rust-essentials` etc. as new skills (rejected, see decision 7):** would address the dead-link problem but introduces four new files that mirror in-elixir-essentials but with thinner content. The rule library + critic subagents already cover those languages; new skills would mostly duplicate existing rule pack pointers.
