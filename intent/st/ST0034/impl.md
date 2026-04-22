# Implementation - ST0034: Agentic Software Engineering Suite

## Progress Tracker

Status as of: 2026-04-22 (WP02 closed; WP01 + WP02 done)

| WP   | Title                              | Status      | Notes                                                                                                   |
| ---- | ---------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------- |
| WP01 | Architecture and rule schema       | Done        | Closed 3625b18; schema + archetype + attribution landed                                                 |
| WP02 | Extension system foundation        | Done        | 6 sessions: callback refactor, ext dispatcher, validator, scaffolding, rules CLI, BATS. 526 tests pass. |
| WP03 | Skill and subagent rationalisation | Not started | Depends WP01                                                                                            |
| WP04 | Agnostic rule pack                 | Not started | Depends WP01                                                                                            |
| WP05 | Elixir rule pack                   | Not started | Depends WP01, WP04                                                                                      |
| WP06 | Rust/Swift/Lua rule packs          | Not started | Depends WP01, WP04                                                                                      |
| WP07 | Critic subagent family             | Not started | Depends WP03, WP04, WP05, WP06                                                                          |
| WP08 | Worker-bee extraction              | Not started | Depends WP02                                                                                            |
| WP09 | Migration and upgrade chain        | Not started | Depends WP02, WP08                                                                                      |
| WP10 | Documentation                      | Not started | Depends WP02, WP07, WP08                                                                                |
| WP11 | Release and fleet upgrade          | Not started | Depends WP01-WP10                                                                                       |

## Implementation Notes

This section accumulates implementation detail, gotchas, and decisions taken during the actual work. Populate as WPs progress.

### Architectural decisions deferred to implementation

- **`rules/index.json` generation schema**: finalise during WP01 after writing archetype rule. Need to see real frontmatter shape before locking the index schema.
- **`critic-<lang>` prompt length**: target under 300 lines per agent.md. Long prompts dilute focus. Final prompt length decided empirically in WP07.
- **`.intent_critic.yml` location**: project root vs. `.intent/` subdir. Default to project root for discoverability; consider fallback to `.intent/critic.yml` if project root is crowded.
- **Rust/Swift/Lua detection heuristics** (in-review stage-2): Cargo.toml, Package.swift are reliable; Lua needs entry-file or `.luarc.json` inspection. Document ambiguity cases in WP07.

## Code Examples

### Rule schema archetype (WP01 as-built)

Shipped at `intent/plugins/claude/rules/_schema/archetype/strong-assertions/` with the final frontmatter shape (upstream_id removed — IN-EX-TEST-001 is Intent-original since upstream's `test-shape-not-values` is telemetry-scoped). Runnable `good_test.exs` / `bad_test.exs` both exit 0; Critic is the enforcer. See commit `3625b18`.

```yaml
---
id: IN-EX-TEST-001
title: Strong assertions against concrete values
language: elixir
category: test
severity: critical
summary: Shape assertions pass for any value of the right type ...
principles: [honest-data, public-interface]
applies_to: ["test/**/*_test.exs"]
references: [IN-AG-HIGHLANDER-001]
related_rules: [IN-EX-TEST-002]
...
version: 1
---
```

### Callback signature (WP02 Session 1 as-built)

`intent/plugins/claude/lib/claude_plugin_helpers.sh` grew `plugin_get_source_roots` + `plugin_source_path_in_root` with conditional defaults so existing plugins that override still win, plus `plugin_resolve_source_file`, `plugin_list_source_origins`, `plugin_root_tag`, `plugin_detect_shadow`. Commit `1d3924f`.

```bash
if ! declare -f plugin_get_source_roots >/dev/null 2>&1; then
  plugin_get_source_roots() {
    echo "$INTENT_HOME/intent/plugins/claude/${PLUGIN_CMD}"
  }
fi
```

Subagent plugin override (Session 1 as-built):

```bash
plugin_get_source_roots() {
  local canon_root="$INTENT_HOME/intent/plugins/claude/subagents"
  if [ "${INTENT_EXT_DISABLE:-}" = "1" ]; then
    echo "$canon_root"
    return 0
  fi
  local ext_base="${INTENT_EXT_DIR:-$HOME/.intent/ext}"
  if [ -d "$ext_base" ]; then
    local ext_dir ext_name ext_subagents
    for ext_dir in "$ext_base"/*/; do
      [ -d "$ext_dir" ] || continue
      ext_name="$(basename "$ext_dir")"
      case "$ext_name" in .*|_*) continue ;; esac
      ext_subagents="${ext_dir%/}/subagents"
      [ -d "$ext_subagents" ] && echo "$ext_subagents"
    done
  fi
  echo "$canon_root"
}
```

### Migration function skeleton (WP09 deliverable)

```bash
migrate_v2_8_2_to_v2_9_0() {
  # 1. Version stamp
  jq '.intent_version = "2.9.0"' .intent/config.json > tmp && mv tmp .intent/config.json

  # 2. Bootstrap user ext dir
  if [ ! -d "$HOME/.intent/ext" ]; then
    mkdir -p "$HOME/.intent/ext"
    generate_ext_readme "$HOME/.intent/ext/README.md"
  fi

  # 3. Seed worker-bee (idempotent)
  if [ ! -d "$HOME/.intent/ext/worker-bee" ] && [ -d "$INTENT_HOME/lib/templates/ext-seeds/worker-bee" ]; then
    cp -r "$INTENT_HOME/lib/templates/ext-seeds/worker-bee" "$HOME/.intent/ext/worker-bee"
  fi

  # 4-5. Prune installed elixir + worker-bee (fail-forward)
  for agent in elixir worker-bee; do
    if [ -f "$HOME/.claude/agents/${agent}.md" ]; then
      rm "$HOME/.claude/agents/${agent}.md"
      if [ -f "$HOME/.intent/agents/installed-agents.json" ]; then
        jq "(.installed) |= map(select(.name != \"${agent}\"))" \
          "$HOME/.intent/agents/installed-agents.json" > tmp \
          && mv tmp "$HOME/.intent/agents/installed-agents.json"
      fi
    fi
  done
}
```

## Technical Details

### Bash 3.x compatibility constraints

- No `declare -A` (associative arrays).
- No `${VAR^}` (case modification).
- No `readarray` / `mapfile`.
- No process substitution in function bodies that need POSIX-safe execution.
- Use newline-separated output + `for x in $(cmd); do` for iteration.

### Highlander enforcement during this ST

- Before creating a new module: register in `intent/llm/MODULES.md` first.
- Before creating a new rule: check for duplication with agnostic + other language packs via `grep -F`.
- Before creating a new skill: check that content doesn't duplicate rule prose.

### Fail-forward discipline

- No deprecation stubs when renaming or removing anything.
- Migration actively deletes installed copies of removed subagents.
- Risk register does not include "breaking N fleet installations" as a blocking concern.

### MIT attribution requirements

- `intent/plugins/claude/rules/_attribution/elixir-test-critic.md` must contain:
  - Full MIT license text.
  - Copyright line: "Copyright 2026 Manuel Zubieta".
  - Source URL: `https://github.com/iautom8things/elixir-test-critic`.
  - Pinned commit hash (captured at WP05 start).
  - List of Intent rules that reference upstream via `upstream_id:`.

## Challenges & Solutions

### Challenge: Upstream heading wording and slug accuracy drift

**Context**: WP01. Drafting the rule schema, I had been writing `## When It Applies` and referencing upstream slugs like `test-critic-strong-assertions` that do not exist. `elixir-test-critic` uses `## When This Applies` verbatim and its real slugs are `no-process-sleep`, `async-by-default`, `start-supervised`, `test-shape-not-values` (telemetry-scoped).
**Solution**: Fetched the upstream tree via GitHub API at the pinned commit `1d9aa40700dab7370b4abd338ce11b922e914b14`, replaced every fabricated slug with a verified upstream slug, and rewrote heading references to match. IN-EX-TEST-001 is now explicitly Intent-original (no `upstream_id`) because upstream's `test-shape-not-values` has a narrower scope.
**Rationale**: Attribution tiers are only credible if upstream IDs resolve. Fabricated slugs break the Tier-2 contract.

### Challenge: `bad_test.exs` exit-code contract

**Context**: WP01. Initial draft of WP01 acceptance criteria said `bad.exs` should fail `mix test`. Upstream convention is the opposite: both `good_test.exs` and `bad_test.exs` exit 0; the Critic reads source to detect the antipattern.
**Solution**: Rewrote the archetype so `bad_test.exs` uses shape-only assertions (`assert is_struct/...`) with a `# BAD PRACTICE:` comment. ExUnit passes; the Critic (not ExUnit) is the enforcer.
**Rationale**: Aligns with upstream and with the `critic-<lang>` responsibility boundary. A failing ExUnit run would be noise, not signal.

### Challenge: Callback overwrite order in library bootstrap

**Context**: WP02 Session 1. Adding `plugin_get_source_roots` defaults in `claude_plugin_helpers.sh`. Plugins define callbacks **before** sourcing the library. A naive definition in the library would overwrite any plugin override.
**Solution**: Wrapped library defaults in `if ! declare -f <name> >/dev/null 2>&1; then ... fi` so the default only takes effect when the plugin did not define its own. Zero behavioural change for existing plugins (none override), opt-in pathway for new ones.
**Rationale**: Keeps the existing "define callbacks, then source" ordering intact — anything else would force the subagents/skills plugins to invert their structure.

### Challenge: `set -e` killed `ext_show` mid-iteration

**Context**: WP02 Session 2. Running `intent ext show valid-ext` printed the Subagents section then truncated. `ext_show_shadow_warning` returned the exit status of its final `[ -f ] && echo` compound, which is 1 when there is no shadow, which under `set -e` aborts the script.
**Solution**: Wrapped the file test in `if ... fi` and added explicit `return 0` at the end of the function.
**Rationale**: Bash's `set -e` + last-command-return semantics means helper functions need to exit 0 deliberately when their "nothing to do" path is the common case.

### Challenge: Fixture `intent_compat.min` vs current VERSION

**Context**: WP02 Session 3. Fixtures declared `intent_compat.min = "2.9.0"` matching the target release. Current VERSION is 2.8.2, so the validator correctly flagged every fixture as incompatible.
**Solution**: Lowered fixture `intent_compat.min` to `"2.8.0"` so all four fixtures run under the current Intent version. The fixture still represents a plausible extension — most real extensions will declare lower min bounds for broader compat.
**Rationale**: Fixtures exist to exercise validator code paths, not to model future-version behaviour. Bumping them to 2.9.0 at release time is cleaner than skipping the compat check in tests.

### Template

### Challenge: [description]

**Context**: What WP surfaced this.
**Solution**: What was done.
**Rationale**: Why this over alternatives.
