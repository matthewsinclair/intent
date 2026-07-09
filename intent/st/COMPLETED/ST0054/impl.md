# Implementation - ST0054: Update Intent for latest usage-rules.md format

## Implementation

Documentation, skill, and template alignment only -- no generator code change. `intent agents sync` neither calls nor should call `mix usage_rules.sync`; `AGENTS.md` stays Intent-generated. The v1.x model was ground-truthed against `../Laksa/deps/usage_rules/README.md` (usage_rules 1.2.6), not just design.md's summary.

### WP-01 -- `intent/docs/working-with-llms.md` interop section

Rewrote the "For Elixir projects" section to the v1.x model:

- Opens by naming the two `usage-rules.md` artifacts (Intent's hand-authored project contract vs the library's per-dep `deps/<pkg>/usage-rules.md` + topical `usage-rules/<topic>.md` files).
- Describes the config-driven model: the `:usage_rules` mix.exs key (`file:` / `usage_rules:` / `skills:`), `mix usage_rules.sync` taking no task args, config as source of truth (marker-bounded regeneration, prune-on-drop).
- Documents the three delivery modes (inline / link / skills), including that `skills:` writes `SKILL.md` with `managed-by: usage-rules` frontmatter via `deps:` / `build:` / `package_skills:`.
- States the `.claude/skills` coexistence policy: Intent projects stay Intent-native (curated skills + on-demand deps reads are source of truth), leave the library `skills:` gen off; file aggregation coexists as additive; keep one `AGENTS.md` generator.
- Removed every pre-v1.0 claim (arg-driven gather, "works without configuration", zero-config auto-include).

### WP-02 -- topical sub-rule folders

Added a `deps/*/usage-rules/*.md` sub-rule-folder reference to `/in-standards` Step 3 and the two peer skills that enumerate deps usage rules (`/in-elixir-essentials`, `/in-ash-ecto-essentials`), so on-demand loading is not limited to the single top-level file. Propagated to the installed skills (`intent claude skills sync` -- 3 updated).

### WP-03 -- `lib/templates/llm/_usage-rules.md`

Dropped the staleable `This project uses Intent v[[INTENT_VERSION]]` line (a one-time init stamp that drifts, since `usage-rules.md` is hand-authored and never regenerated on upgrade) -- now version-agnostic, pointing at `intent doctor` for the live version. Added a paragraph distinguishing this project-contract file from the library's per-dep `usage-rules.md`.

### Verify sweep

Grepped the tree for pre-v1.0 references (`link_to_folder`, `link_style`, "without configuration", zero-config, auto-gather framing). Two current-guidance hits reconciled: the doc ToC entry (renamed heading) and line ~124's "discovers them and gathers them into the project-level AGENTS.md" (softened to "can aggregate them into a target file when configured"). The only surviving match is the intentional migration note stating the v0.x options are gone.

## Challenges & Solutions

- **usage_rules v0.2-vs-v1.0 version nuance.** The README's migration notes attribute config-driven setup to v0.2 and the removal of `link_to_folder` / `link_style` / `inline` to v1.0; info.md frames it as the v1.0 rebuild. The prose uses "v1.x" as the umbrella (Laksa is on 1.2.6) to avoid over-precision on which point release introduced which piece.
