# Tasks - ST0054: Update Intent for latest usage-rules.md format

## Tasks

- [ ] WP-01: Rewrite the `working-with-llms.md` "mix usage_rules.sync interop" section (approx lines 476-489) to the v1.x model -- config-driven setup, the three delivery modes, the two-artifacts distinction, and the `.claude/skills` coexistence policy. Remove all pre-v1.0 claims.
- [ ] WP-02: Update `/in-standards` Step 3 (and any peer skill that enumerates deps usage rules) to also reference topical `deps/*/usage-rules/*.md` sub-rule folders.
- [ ] WP-03: Fix `lib/templates/llm/_usage-rules.md` -- resolve the staleable `[[INTENT_VERSION]]` line and add the note distinguishing Intent's project-contract file from the library's per-dep files.
- [ ] Verify: grep the Intent tree for other references to the pre-v1.0 usage_rules model (`--link-to-folder`, `link_style`, "without configuration", arg-style `usage_rules.sync <file>`) and reconcile.
- [ ] Ship as Intent 2.16.1; Laksa re-integrates via `intent claude upgrade --apply` (Option A -- stay Intent-native).

## Task Notes

- No generator/code change: `intent agents sync` neither calls nor should call `mix usage_rules.sync`.
- Evidence for the as-built claims: `working-with-llms.md:476-489`; `in-standards/SKILL.md:30-35`; `_usage-rules.md:5`.
- Evidence for v1.x behaviour: the `usage_rules` README plus `deps/usage_rules/lib/mix/tasks/usage_rules.sync.ex` (config key, markers, skills generation), and CHANGELOG v1.0.0 ("rebuild usage rules to be config-driven and support agent skills").

## Dependencies

- WP-01 depends on the coexistence policy (design.md "Design Decision") being ratified -- WP-01 states that policy.
- Laksa re-integration depends on 2.16.1 shipping.
