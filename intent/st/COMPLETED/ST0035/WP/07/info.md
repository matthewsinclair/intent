---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-07
title: "Ship .intent_critic.yml default template"
scope: ExtraSmall
status: Done
---

# WP-07: Ship .intent_critic.yml default template

## Objective

Ship the canonical `.intent_critic.yml` configuration template at `lib/templates/_intent_critic.yml`. Installed per-project by `intent claude upgrade --apply` (WP11). Read by `bin/intent_critic` (WP05) and `.git/hooks/pre-commit` (WP06) for threshold and rule-disable overrides.

## Context

The critic subagent contract (`intent/docs/critics.md`) already specifies `.intent_critic.yml` as the per-project config. Current fields: `severity_min`, `disabled` (rule IDs). WP07 ships the default template and confirms the shape.

Defaults (open decision #3 and #4 resolved):

- `severity_min: warning` — pre-commit blocks on CRITICAL + WARNING, logs RECOMMENDATION + STYLE.
- `disabled: []` — no rules disabled by default; user opts out per rule as needed.
- `post_tool_use_advisory: false` — advisory critic on PostToolUse is off by default (open decision #4 default).

Users edit this file to tune per project.

## Deliverables

1. **Template** at `lib/templates/_intent_critic.yml`. Contents:

   ```yaml
   # Intent critic configuration
   # Controls pre-commit critic gate behaviour and advisory critic triggers.
   # See intent/docs/pre-commit-hook.md and intent/docs/critics.md for reference.

   # Minimum severity to block on. Findings at or above this level fail the
   # pre-commit gate. Valid: critical | warning | recommendation | style.
   severity_min: warning

   # Rule IDs to disable project-wide. Use for false positives or
   # intentional deviations. Each entry should be commented with justification.
   disabled: []
   # Example:
   # disabled:
   #   - IN-EX-CODE-012  # Project uses Process.sleep in test fixtures by design.

   # Whether to trigger advisory critic on PostToolUse (after Write/Edit
   # tool use on source files). Off by default; turning on runs a lightweight
   # advisory check per tool use.
   post_tool_use_advisory: false
   ```

2. **MODULES.md registration** for the template.
3. **Cross-references** from `intent/docs/critics.md` (config schema) and `intent/docs/pre-commit-hook.md` (WP06 author) pointing at this file.

## Approach

1. Confirm the existing `.intent_critic.yml` schema in `intent/docs/critics.md` — extend if needed.
2. Author the template with well-commented defaults.
3. Verify YAML parses (`yq` if available, else a simple bash-grep confirmation for the three top-level fields).
4. MODULES.md update.
5. Commit.

## Acceptance Criteria

- [ ] `lib/templates/_intent_critic.yml` exists.
- [ ] `yq e '.' lib/templates/_intent_critic.yml` (or `python3 -c "import yaml; yaml.safe_load(open(...))"`) parses cleanly.
- [ ] Template documents `severity_min`, `disabled`, `post_tool_use_advisory` with inline comments.
- [ ] Defaults match design decisions: `warning`, `[]`, `false`.
- [ ] `intent/docs/critics.md` links to this template.
- [ ] `intent/docs/pre-commit-hook.md` (WP06) links to this template.
- [ ] MODULES.md registers the template.
- [ ] Commit follows Intent conventions.

### Tests to add

- Minimal smoke test: BATS test that sources the template via a parser and confirms the three keys exist. If Intent's bash env doesn't have `yq`/`python3`, skip; the test file's presence is a documentation artefact.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP05 (consumes this config).
- **Blocks**: WP11 (upgrade installs the template per project).

## Implementation Notes

- **Bash 3.x YAML parsing**: `bin/intent_critic` and the pre-commit hook need to read this file. They only need scalar top-level fields (`severity_min`, `post_tool_use_advisory`) and a list (`disabled`). A minimal grep/awk parser handles this; full YAML lib not required.
- **Comment every default**: config files should explain _why_ the default is what it is. The comments are documentation, not noise.
- **Don't over-specify**: ship only the fields consumers use. Adding speculative fields creates drift.
- **File name is `.intent_critic.yml` (dotted, underscore)** — matches existing critic subagent contract. Don't change naming here.

## Risks and Edge Cases

- **Risk**: Users edit the file and produce invalid YAML that crashes the critic. **Mitigation**: `bin/intent_critic` catches parse errors and emits a clear "can't read config; using defaults" advisory rather than crashing.
- **Risk**: YAML flavour incompatibility (YAML 1.1 vs 1.2). **Mitigation**: stick to trivial scalars and lists. Avoid anything funky.
- **Edge**: Users comment out all fields. Hook falls back to compiled-in defaults (documented in `bin/intent_critic --help`).

## Verification Steps

1. `cat lib/templates/_intent_critic.yml` — visible and readable.
2. YAML parser test (yq / python).
3. MODULES.md audit.
4. `grep -l "_intent_critic.yml\|.intent_critic.yml" intent/docs/` — both docs link.
5. `bin/intent_critic` (WP05) run with this config — confirm defaults applied.

## Size and Estimate

- **Size**: XS (Extra Small). Single session.

## Exit Checklist

- [ ] Template committed.
- [ ] YAML parses cleanly.
- [ ] MODULES.md updated.
- [ ] Cross-refs in docs.
- [ ] Committed.
