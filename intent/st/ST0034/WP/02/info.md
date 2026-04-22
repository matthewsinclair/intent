---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-02
title: "Extension system foundation"
scope: Large
status: WIP
---

# WP-02: Extension system foundation

## Objective

Implement the `~/.intent/ext/<name>/` extension mechanism end to end: discovery, precedence rules, shadow warnings, and the `intent ext` command surface (list/show/validate/new). Extend the existing plugin callback pattern with an additive `plugin_get_source_roots` callback so that subagents and skills discovered from user extensions integrate with today's install/sync/uninstall flow unchanged.

## Context

Today Intent has a single plugin root (`$INTENT_HOME/intent/plugins/*/plugin.json`) and hardcoded routing for `claude` and `agents` plugins in `bin/intent:78-109`. Users cannot add skills, subagents, or rule packs without forking Intent. WP02 delivers the first-class mechanism for user-local extensions, with an explicit design principle that **canon vs user-ext discovery is layered, not replaced**: canon remains the default; user extensions shadow canon by name with a visible warning on every list/show/install.

This WP also delivers the `intent claude rules` command surface (validator + index generator) because the rule library lives under the `claude` plugin and needs the same discovery machinery to list/validate rules from both canon and extensions.

Bash 3.x compatibility is non-negotiable (macOS default). All iteration uses newline-separated output and `for x in $(cmd)` patterns.

## Deliverables

### New CLI dispatcher

- `bin/intent_ext` (new file, modelled on `bin/intent_plugin`) — implements `list`, `show`, `validate`, `new` subcommands
- `bin/intent` — new `ext)` case in the router; add `ext` to the GLOBAL_COMMANDS set; help entry

### Callback refactor (additive, no breaking changes)

- `intent/plugins/claude/lib/claude_plugin_helpers.sh` — new optional callback `plugin_get_source_roots` with a documented default that returns canon-only (preserves v2.8.2 behaviour for plugins that don't override)
- `claude_plugin_helpers.sh` — new library helper `plugin_resolve_source_file NAME` that walks roots in precedence order and returns the first hit
- `claude_plugin_helpers.sh` — new helper `plugin_detect_shadow NAME` that returns shadow provenance if a name exists in multiple roots
- `claude_plugin_helpers.sh` header comment updated to document the new callback and helpers alongside the existing 8

### Subagent and skill plugins use multi-root discovery

- `intent/plugins/claude/bin/intent_claude_subagents` — override `plugin_get_source_roots` to include `~/.intent/ext/<name>/subagents/` and `$INTENT_EXT_DIR`; emit shadow warning in `list`, `show`, `install`
- `intent/plugins/claude/bin/intent_claude_skills` — same pattern for skills

### New rules CLI surface

- `intent/plugins/claude/bin/intent_claude_rules` — new script: `list`, `show`, `validate`, `index` subcommands
- `intent claude rules list` — enumerates rules across canon + extensions, tagged by provenance
- `intent claude rules show <id>` — prints one rule's Markdown body
- `intent claude rules validate [<id>|<path>]` — frontmatter schema check, cross-reference resolution, MIT attribution presence
- `intent claude rules index` — regenerates `intent/plugins/claude/rules/index.json` from RULE.md frontmatter
- Registered in `intent/plugins/claude/plugin.json` `commands` array

### Extension manifest schema

- `intent/plugins/claude/ext-schema/extension.schema.json` — JSON Schema for `extension.json` validation
- Fields: `schema`, `name`, `version`, `description`, `author`, `license`, `homepage`, `intent_compat` (min/max), `contributes` (subagents, skills, rules arrays), optional `checksums`

### Help files

- `lib/help/ext.help.md` — full help for the `intent ext` command
- `lib/help/rules.help.md` — help for `intent claude rules` (under the `claude` plugin)

### Fixtures and tests

- `tests/fixtures/extensions/valid-ext/` — complete valid extension (one subagent + one skill + one rule)
- `tests/fixtures/extensions/malformed-ext/` — missing `schema` field in `extension.json`
- `tests/fixtures/extensions/shadow-ext/` — subagent named `intent` (collides with canon)
- `tests/fixtures/extensions/traversal-ext/` — `contributes` path with `..` escape attempt
- `tests/unit/ext_commands.bats` — list/show/validate/new behaviour
- `tests/unit/ext_discovery.bats` — precedence, shadowing, `INTENT_EXT_DISABLE=1`

### Documentation draft

- `intent/docs/writing-extensions.md` — skeleton (full worked example lands in WP10 after worker-bee extraction in WP08)

### MODULES.md updates

- Register `bin/intent_ext`, `bin/intent_claude_rules`, `claude_plugin_helpers.sh:plugin_resolve_source_file`, `ext-schema/`, new help files

## Approach

1. **Callback refactor first.** Modify `claude_plugin_helpers.sh`: add `plugin_get_source_roots` default, `plugin_resolve_source_file` helper, `plugin_detect_shadow` helper. Update header comment. Run existing BATS suite — must remain green (zero behavioural change for default single-root case).

2. **Extend subagents plugin.** Override `plugin_get_source_roots` in `intent_claude_subagents` to include `~/.intent/ext/`. Modify `list` output to tag each item with provenance (`[canon]` / `[ext:<name>]` / `[ext:<name>, shadows canon]`). Emit shadow warnings to stderr on `list`, `show`, `install`.

3. **Mirror for skills.** Same pattern in `intent_claude_skills`.

4. **Implement `bin/intent_ext`.** Four subcommands with bash 3.x safety. Each:
   - `list`: enumerate `~/.intent/ext/*/extension.json`; show name, version, status (valid/malformed/shadow).
   - `show <name>`: parse `extension.json`, display manifest fields, list contributions, flag shadows.
   - `validate [<name>]`: JSON schema check (via `ajv` if available, else jq fallback); path-traversal check; intent_compat check; contribution-path existence; optional checksum verification.
   - `new <name> --subagent|--skill|--rule-pack`: scaffold directory + `extension.json` + placeholder contributed files.

5. **Router update.** Add `ext)` case in `bin/intent` that `exec`s `bin/intent_ext`. Add `ext` to `GLOBAL_COMMANDS`. Add one-line help entry.

6. **Implement `bin/intent_claude_rules`.** Four subcommands:
   - `list`: enumerate RULE.md across canon + extensions, show `id | severity | language | source-provenance`.
   - `show <id>`: resolve ID to RULE.md, print Markdown body.
   - `validate`: frontmatter schema check, cross-reference resolution (`references:` IDs exist), attribution presence where `upstream_id:` is set.
   - `index`: regenerate `intent/plugins/claude/rules/index.json` from frontmatter (jq pipeline).

7. **Write extension fixtures.** Each fixture is a minimal but complete extension testing one scenario.

8. **Write BATS tests.** Use `INTENT_EXT_DIR` env override to point at fixtures. Test discovery, precedence, shadow warnings, validation failures, scaffolding.

9. **Draft `writing-extensions.md`.** Skeleton structure only; worked example fills in at WP10.

10. **MODULES.md registrations.** Every new module, helper, fixture directory, test file.

## Acceptance Criteria

### Callback refactor

- [ ] `plugin_get_source_roots` callback documented in `claude_plugin_helpers.sh` header
- [ ] Default implementation returns canon root only (no behavioural change when no plugin overrides)
- [ ] `plugin_resolve_source_file NAME` walks roots in precedence order and returns first hit
- [ ] `plugin_detect_shadow NAME` returns shadow provenance string (empty if no shadow)
- [ ] All 8 original callbacks unchanged; no removed functionality

### Subagent and skill discovery

- [ ] `intent claude subagents list` shows ext subagents with `[ext:<name>]` tag
- [ ] `intent claude subagents list` shows shadowed subagents with `[ext:<name>, shadows canon]` tag and emits warning to stderr
- [ ] `intent claude subagents install <name>` resolves source via `plugin_resolve_source_file` (ext wins over canon with warning)
- [ ] Symmetric behaviour for `intent claude skills list/install`

### `intent ext` commands

- [ ] `intent ext list` returns empty-set status (no error) when `~/.intent/ext/` is absent
- [ ] `intent ext list` enumerates valid extensions, showing name + version + status
- [ ] `intent ext list` flags malformed extensions as "malformed" without crashing
- [ ] `intent ext show <name>` prints parsed manifest + contribution list + shadow warnings
- [ ] `intent ext show <missing-name>` returns non-zero with a clear error
- [ ] `intent ext validate` checks every extension; reports schema violations, path traversal, missing contributed files, version incompatibility
- [ ] `intent ext validate <name>` validates a single extension
- [ ] `intent ext new my-ext --subagent` scaffolds a valid skeleton that passes `intent ext validate my-ext`
- [ ] `intent ext new my-ext --skill` and `--rule-pack` scaffold valid skeletons

### `intent claude rules` commands

- [ ] `intent claude rules list` enumerates rules across canon + extensions, with provenance
- [ ] `intent claude rules show <id>` resolves and displays the RULE.md content
- [ ] `intent claude rules validate` checks frontmatter schema, cross-references, attribution presence
- [ ] `intent claude rules index` regenerates `index.json` deterministically (running twice produces identical output)

### Safety

- [ ] Path-traversal attempts (`..` in `contributes` path) rejected by validator
- [ ] `INTENT_EXT_DISABLE=1` env var suppresses ext discovery entirely
- [ ] `INTENT_EXT_DIR` env var overrides default `~/.intent/ext/` (used by tests)
- [ ] No auto-execution of extension code; extensions are content-only in v2.9.0

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP02 for the authoritative list. WP02-specific surfaces:

- [ ] `tests/unit/ext_commands.bats` — `intent ext list|show|validate|new` command surface (argument parsing, error paths, output format)
- [ ] `tests/unit/ext_discovery.bats` — multi-root search order (`$INTENT_EXT_DIR` → `~/.intent/ext/` → canon), shadow warning emission on collisions, `INTENT_EXT_DISABLE=1` escape hatch
- [ ] `tests/unit/rule_validator.bats` — `intent claude rules validate` against the archetype (must pass) and against fixtures with known frontmatter errors (each error path exercised)
- [ ] `tests/unit/rule_index.bats` — `intent claude rules index` shell+jq pipeline: index.json is regenerable and byte-identical on re-run
- [ ] Fixtures committed under `tests/fixtures/extensions/{valid-ext,malformed-ext,shadow-ext,traversal-ext}/`
- [ ] Fixtures committed under `tests/fixtures/rules/{valid,missing-frontmatter,bad-id,duplicate-id,unresolved-reference,unknown-field}/`
- [ ] All tests run on macOS bash 3.x (no `declare -A`, no `readarray`, no `${VAR^}`)

### Tests to update

- [ ] Existing 469-test baseline remains green after WP02 commits (pristine invariant)
- [ ] No existing test is modified; WP02 is additive from the BATS suite's perspective

### Discoverability

- [ ] `intent help` mentions `ext` command
- [ ] `intent ext --help` shows `lib/help/ext.help.md`
- [ ] `intent plugin list` output includes the `claude` plugin's new `rules` subcommands
- [ ] MODULES.md has entries for every new module

## Dependencies

- **WP01** (schema): required. The rule schema and `extension.json` schema must exist before validators can be written.

## Implementation Notes

### Exact file paths to modify

- `/Users/matts/Devel/prj/Intent/bin/intent` (lines 78-134, add `ext` route; line ~41 area for GLOBAL_COMMANDS)
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/lib/claude_plugin_helpers.sh` (lines 7-21 for callback documentation; add new callback + helpers)
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/bin/intent_claude_subagents` (add `plugin_get_source_roots` override; modify `list` output)
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/bin/intent_claude_skills` (same pattern)

### New files to create

- `/Users/matts/Devel/prj/Intent/bin/intent_ext`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/bin/intent_claude_rules`
- `/Users/matts/Devel/prj/Intent/intent/plugins/claude/ext-schema/extension.schema.json`
- `/Users/matts/Devel/prj/Intent/lib/help/ext.help.md`
- `/Users/matts/Devel/prj/Intent/lib/help/rules.help.md`
- `/Users/matts/Devel/prj/Intent/tests/unit/ext_commands.bats`
- `/Users/matts/Devel/prj/Intent/tests/unit/ext_discovery.bats`
- `/Users/matts/Devel/prj/Intent/tests/fixtures/extensions/{valid,malformed,shadow,traversal}-ext/`
- `/Users/matts/Devel/prj/Intent/intent/docs/writing-extensions.md` (skeleton)

### Callback signature (to be added in `claude_plugin_helpers.sh`)

```bash
# plugin_get_source_roots  -- echo newline-separated root directories
#                             to search, in precedence order (highest first).
#                             If undefined, defaults to a single root
#                             (the canon subagents/skills dir).
plugin_get_source_roots() {
  echo "$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
}
```

The subagents/skills plugin override looks like:

```bash
plugin_get_source_roots() {
  [ "${INTENT_EXT_DISABLE:-}" = "1" ] && { echo "$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"; return 0; }
  [ -n "${INTENT_EXT_DIR:-}" ] && echo "$INTENT_EXT_DIR"
  if [ -d "$HOME/.intent/ext" ]; then
    for e in "$HOME/.intent/ext"/*/${PLUGIN_CMD}; do
      [ -d "$e" ] && echo "$e"
    done
  fi
  echo "$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
}
```

### Shadow warning format

```
warning: 'worker-bee' in ~/.intent/ext/worker-bee/ shadows canon subagent
  to use canon: set INTENT_EXT_DISABLE=1
```

Emitted once per `list`, `show`, `install` invocation per name. Not emitted on every task execution.

### JSON Schema strictness

`extension.schema.json` uses `additionalProperties: false` at the top level but `additionalProperties: true` on `contributes` entries (to allow future fields like `priority`, `tags`). Strict mode catches typos in top-level fields during `intent ext validate`.

### Path traversal detection

For every path in `contributes.<type>[].path`:

1. Resolve relative to extension root.
2. Check that resolved path starts with the extension root.
3. Reject if any segment is `..` or if any symlink escapes the root.

### `intent ext new` scaffolding

With `--subagent`:

- Creates `<name>/extension.json` with `contributes.subagents = [{name: <name>, path: "subagents/<name>"}]`
- Creates `<name>/subagents/<name>/agent.md` with placeholder frontmatter
- Creates `<name>/subagents/<name>/metadata.json`
- Creates `<name>/README.md`

With `--skill` and `--rule-pack`, analogous scaffolding.

### Helper file generation

`generate_ext_readme()` (new function, likely in `intent_helpers`) produces `~/.intent/ext/README.md` on migration. Content explains:

- What `~/.intent/ext/` is for.
- How to list extensions: `intent ext list`.
- How to create one: `intent ext new <name>`.
- Reference to worker-bee as the worked example.

## Risks and Edge Cases

### Callback compatibility

The new `plugin_get_source_roots` must be truly optional. If a plugin doesn't define it, the library-side helper `plugin_resolve_source_file` falls back to calling `plugin_get_source_file NAME` directly. **Regression test**: existing BATS suite must pass without any plugin overriding the new callback.

### Malformed manifests

`extension.json` with syntax errors must not crash discovery. `jq` parse errors are caught and surfaced as "malformed" status, not propagated.

### Symlink escape

An extension could contain a symlink to `/` or `$HOME/.ssh/`. The validator walks symlinks and rejects any whose target falls outside the extension root.

### Circular references

`references:` in a rule could cycle. WP02 validator detects cycles and reports them; does not follow them blindly.

### Bash 3.x

- No `readarray`.
- `while read` loops instead of `mapfile`.
- Quoting discipline: every path is `"$path"`, never `$path`.

### XDG compliance

`~/.intent/ext/` is the default. If `XDG_CONFIG_HOME` is set, consider `$XDG_CONFIG_HOME/intent/ext/` as an alias — but default to `~/.intent/ext/` for consistency with existing `~/.intent/agents/` pattern.

### Extension name collisions across roots

Two extensions under `~/.intent/ext/` with overlapping `contributes.subagents[].name` — e.g. ext A contributes `foo` and ext B contributes `foo`. Validator detects and reports; discovery uses the first match by directory-listing order (deterministic but user-controlled via dir names).

## Testing Approach

### Unit tests (BATS)

- `ext_commands.bats` exercises each subcommand against fixtures.
- `ext_discovery.bats` exercises precedence, shadow warnings, env-var overrides.
- Use `INTENT_EXT_DIR` to point at fixture dir; isolates tests from user's actual `~/.intent/ext/`.

### Regression tests

- Full existing BATS suite must pass (`tests/run_tests.sh`).
- `intent claude subagents list` output format unchanged when no extensions present.

### Manual verification

- Create `~/.intent/ext/test-ext/` by hand; verify `intent ext list` sees it.
- Scaffold with `intent ext new test2 --subagent`; verify output passes `intent ext validate test2`.
- Simulate shadow: create an ext subagent named `intent`; verify warning on `intent claude subagents list`.

## Size and Estimate

- **Size**: L (Large, 4-6 sessions). This is the most complex WP.
- **Session 1**: Callback refactor + subagent/skill plugin multi-root support + regression test.
- **Session 2**: `bin/intent_ext` dispatcher + `list`/`show` subcommands + fixtures.
- **Session 3**: `validate` subcommand + extension.schema.json + path-traversal guards.
- **Session 4**: `new` subcommand scaffolding + help files.
- **Session 5**: `bin/intent_claude_rules` + rules schema validation.
- **Session 6**: BATS tests, edge-case hardening, MODULES.md pass.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] Full BATS suite green
- [ ] `intent doctor` clean on Intent repo
- [ ] `intent ext validate` clean on all four fixtures (valid passes, malformed/shadow/traversal fail as expected)
- [ ] MODULES.md entries for every new module
- [ ] No dead code
- [ ] `writing-extensions.md` skeleton exists (full content in WP10)

## As-Built (in progress)

### Session 1 — callback refactor + plugin opt-in (commit `1d3924f`)

| Path                                                 | Change                                                                                                                                                                                                                                                                                                                                |
| ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `intent/plugins/claude/lib/claude_plugin_helpers.sh` | Added `plugin_get_source_roots`, `plugin_source_path_in_root`, `plugin_resolve_source_file`, `plugin_list_source_origins`, `plugin_root_tag`, `plugin_detect_shadow`. Conditional defaults (canon-only) so plugins that override still win. `plugin_install` / `plugin_sync` switched to the resolver; install emits shadow warnings. |
| `intent/plugins/claude/bin/intent_claude_subagents`  | Opt-in: ext roots (`INTENT_EXT_DIR` / `~/.intent/ext/`) precede canon. `plugin_copy_to_target` routes through resolver. `list` gains an Extensions section with `[ext:<name>]` / `[ext:<name>, shadows canon]` tags + stderr warnings. `show` runs `plugin_detect_shadow` and resolves across roots.                                  |
| `intent/plugins/claude/bin/intent_claude_skills`     | Same pattern for SKILL.md layout.                                                                                                                                                                                                                                                                                                     |

BATS: 469/469 passed after commit.

### Session 2 — `intent ext` dispatcher + fixtures (commit `4a6dc4e`)

| Path                                       | Change                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bin/intent_ext`                           | New global dispatcher. `list` enumerates `~/.intent/ext/` (honours `INTENT_EXT_DIR` / `INTENT_EXT_DISABLE`) and classifies each extension as `valid` / `malformed` / `missing-manifest`. `show <name>` parses `extension.json` and prints manifest fields, contribution counts, per-contribution shadow warnings. `validate` and `new` were explicit stubs in Session 2. |
| `bin/intent`                               | `ext` added to `GLOBAL_COMMANDS`.                                                                                                                                                                                                                                                                                                                                        |
| `bin/intent_help`                          | `ext` listed in core section and skip list (no double-listing).                                                                                                                                                                                                                                                                                                          |
| `lib/help/ext.help.md`                     | Synopsis, commands, env-var table, examples.                                                                                                                                                                                                                                                                                                                             |
| `tests/fixtures/extensions/valid-ext/`     | Happy path: subagent + skill + rule, full manifest with `intent_compat.min = 2.8.0`.                                                                                                                                                                                                                                                                                     |
| `tests/fixtures/extensions/malformed-ext/` | `extension.json` missing required `schema` field.                                                                                                                                                                                                                                                                                                                        |
| `tests/fixtures/extensions/shadow-ext/`    | Contributes a subagent named `intent` to collide with canon.                                                                                                                                                                                                                                                                                                             |
| `tests/fixtures/extensions/traversal-ext/` | `contributes.subagents[0].path = "../../../escape-target"` for traversal rejection.                                                                                                                                                                                                                                                                                      |

BATS: 469/469 passed after commit.

### Session 3 — validator + schema (this session, uncommitted)

| Path                                                     | Change                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `intent/plugins/claude/ext-schema/extension.schema.json` | New JSON Schema (draft 2020-12). Top-level `additionalProperties: false`, required `schema` / `name` / `version`, regex on name and version, semver pattern on `intent_compat.*`. Contribution items require `name` + `path`.                                                                                                                                                                           |
| `bin/intent_ext` (`ext_validate` no longer a stub)       | Per-extension check: JSON parse, required top-level fields, schema literal equality, manifest name matches directory, semver on version, `intent_compat.min` vs current Intent VERSION, unknown-key rejection, traversal guard (rejects absolute paths and any segment matching `..`), contribution-file existence (`agent.md` / `SKILL.md` / `RULE.md`). Summary line with counts; exit 1 if any fail. |
| `tests/fixtures/extensions/*/extension.json`             | `intent_compat.min` dropped from `"2.9.0"` to `"2.8.0"` so fixtures validate under current VERSION. Bumped back at WP11 release time.                                                                                                                                                                                                                                                                   |

Smoke: `intent ext validate` reports `2 ok, 2 failed` across fixtures (valid-ext and shadow-ext pass; malformed-ext fails on missing schema; traversal-ext fails on `..` rejection). BATS 469/469 still green.

### Session 4 — `ext new` scaffolding (this session, uncommitted)

| Path             | Change                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bin/intent_ext` | `ext_new` replaces the Session 4 stub. Accepts `<name> --subagent \| --skill \| --rule-pack`. Validates name regex (`^[a-z0-9][a-z0-9-]*$`), refuses to overwrite an existing target, builds `extension.json` via `jq -n` with the current Intent VERSION as `intent_compat.min`, generates `README.md` with type-specific "next steps", and scaffolds the placeholder contribution (`subagents/<name>/agent.md` + `metadata.json`, `skills/<name>/SKILL.md`, or `rules/agnostic/<name>-sample/RULE.md` with a generated `IN-AG-EXT-<UPPER>-001` id). Scaffolds pass `intent ext validate <name>` on first try for all three types. |

Smoke: created all three types under a tmp `INTENT_EXT_DIR`, validated each, confirmed overwrite refusal and unknown-flag rejection. BATS 469/469 still green.

### Session 5 — `intent claude rules` CLI (this session, uncommitted)

| Path                                            | Change                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `intent/plugins/claude/bin/intent_claude_rules` | New script. `list` enumerates canon + ext RULE.md files (filters: `--lang`, `--severity`). `show <id>` resolves and prints with a provenance header. `validate [<id>\|<path>]` checks frontmatter (required scalars, id regex, severity enum), required H2 sections, H1 presence, `upstream_id`+attribution coupling, and cross-reference resolution for `references:`. `index` regenerates `rules/index.json` from canon only, sorts rules by id, pretty-prints through `jq -S` for byte-stable output. Extensions are discovered at runtime and are not folded into the canon index. |
| `bin/intent`                                    | Claude plugin router gains `rules` subcommand.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `intent/plugins/claude/plugin.json`             | `commands` array gains the `rules` entry.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `lib/help/rules.help.md`                        | Full help text with synopsis, commands, env-var table, checks-performed list.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `intent/plugins/claude/rules/index.json`        | First generation. Zero rules today (canon populates in WP04-WP06); schema/version/pin populated correctly.                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `tests/unit/plugin_commands.bats`               | Bumped `Commands (6)` -> `Commands (7)` and added `intent claude rules` assertion. Legitimate additive change, not a regression workaround.                                                                                                                                                                                                                                                                                                                                                                                                                                            |

Smoke: `list` shows 0 canon + 1 fixture-ext rule. `show` resolves ext id with provenance tag. `validate` passes ext fixture; archetype validate surfaces the expected forward-reference error (`IN-AG-HIGHLANDER-001` is WP04 work). `index` run twice produces byte-identical output. BATS 469/469 green.

### Session 6 remaining

- BATS suites: `tests/unit/ext_commands.bats`, `tests/unit/ext_discovery.bats`, `tests/unit/rule_validator.bats`, `tests/unit/rule_index.bats`.
- `intent/docs/writing-extensions.md` skeleton (full worked example lands in WP10).
- MODULES.md registrations for every new module added by WP02.
- Exit checklist closeout; WP02 done.
