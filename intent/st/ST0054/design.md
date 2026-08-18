# Design - ST0054: Update Intent for latest usage-rules.md format

## Approach

Documentation, skill, and template alignment -- no generator code changes. Intent's `intent agents sync` does not call `mix usage_rules.sync` and should not start; `AGENTS.md` stays Intent-generated. The fix is to make Intent's _guidance_ describe the `usage_rules` v1.x reality and to state Intent's official coexistence policy for the `.claude/skills` overlap. Three edit targets plus one policy decision.

## The usage_rules v1.x model (reference)

- Config lives in `mix.exs` `project/0` under a `:usage_rules` keyword list: `file:` (eg `"AGENTS.md"` / `"CLAUDE.md"`), `usage_rules:` (`:all`, or a list of atoms / `"pkg:sub"` / regex, optionally `{pkg, link: :at | :markdown}`), and `skills:` (`location:`, `deps:`, `build:`, `package_skills:`).
- `mix usage_rules.sync` takes NO task arguments -- only Igniter globals (`--yes`, `--dry-run`, `--check`, `--verbose`). The v0.x form `mix usage_rules.sync AGENTS.md --all --link-to-folder deps` and the `link_to_folder` / `link_style` / `inline` options are removed.
- Config is the source of truth: content is written between `<!-- usage-rules-start -->` and per-package markers and regenerated each run; packages dropped from config are pruned; content outside the markers is preserved (a markerless file is appended to, not clobbered).
- Skills: `SKILL.md` files (default `.claude/skills/<name>/`) carry `managed-by: usage-rules` frontmatter; `build:` composes one skill from several deps (eg `ash-framework` from `[:ash, ~r/^ash_/]`); `package_skills:` copies skills a package ships; names are normalised to the agentskills.io spec.
- Aggregation into a file is NOT deprecated; skills are a second delivery channel for the same per-dep source. No MCP involvement.

## Change targets (as-built -> required)

### 1. `intent/docs/working-with-llms.md` -- "For Elixir projects: mix usage_rules.sync interop" (approx lines 476-489)

As-built describes the pre-v1.0 model:

- "it scans the project's dependencies for `deps/<dep>/usage-rules.md` files and gathers them into a project-level `AGENTS.md`" -- the old arg-driven behaviour.
- "if you're already running `mix usage_rules.sync` for your deps, adding Intent's `usage-rules.md` to the mix works without configuration" -- FALSE under v1.x (requires the `:usage_rules` config key; nothing is auto-included).
- Coexistence framed only at the `AGENTS.md`-file level; silent on the skills-level collision.

Required:

- Describe the v1.x config-driven model (the `:usage_rules` key; no CLI args; the three delivery modes inline / link / skills).
- Distinguish the two `usage-rules.md` artifacts by name up front (Intent project-contract vs library per-dep files).
- Document the `.claude/skills` collision and state the official policy (see Design Decision below).
- Remove every pre-v1.0 claim (zero-config auto-include, arg-driven gather).

### 2. `intent/plugins/claude/skills/in-standards/SKILL.md` -- Step 3 (lines 30-35)

As-built lists only `deps/ash/usage-rules.md`, `deps/ash_postgres/usage-rules.md`, `deps/phoenix_live_view/usage-rules.md`, and "Any other `deps/*/usage-rules.md` or `deps/*/AGENTS.md`".

Required: also reference the topical sub-rule folders `deps/*/usage-rules/*.md` (eg `deps/usage_rules/usage-rules/{elixir,otp}.md`) that v1.x deps ship, so on-demand loading is not limited to the single file. Any peer skill that enumerates deps usage rules (eg `/in-elixir-essentials`, `/in-ash-ecto-essentials`) gets the same treatment.

### 3. `lib/templates/llm/_usage-rules.md` -- template (line 5)

As-built: "This project uses Intent v[[INTENT_VERSION]]" is interpolated only at install/upgrade, so it goes stale between upgrades (Laksa's installed copy reads v2.10.0 while the project is on 2.16.0).

Required: either drop the hard-coded version line or make it unambiguously refresh-on-upgrade; and add a one-line note distinguishing this file (the project DO / NEVER contract) from the library's per-dep `usage-rules.md`, so the name collision does not recur.

## Design Decision -- .claude/skills coexistence policy

Intent projects stay Intent-native: Intent's curated skills (`in-ash-ecto-essentials`, `in-phoenix-liveview`, `in-elixir-essentials`) plus `/in-standards`' on-demand `deps/*/usage-rules{,/**}.md` reads are the source of truth for dep-rule delivery. In an Intent project, leave the library's `skills:` generation OFF (do not wire `mix usage_rules.sync` skill-gen) so two tools are not writing the same `.claude/skills` domains. A project that specifically wants library-generated skills must scope them to non-overlapping names and owns that reconciliation. The interop doc states this explicitly.

## Alternatives Considered

- **Intent adopts usage_rules skill-gen** (retire the curated `in-*-essentials` skills, let the library generate `ash-framework` / `phoenix-framework`): rejected -- abandons Intent's curated, session-loaded skills and hands a core part of the Intent experience to an external tool's output.
- **Intent shells out to `mix usage_rules.sync` during `intent agents sync`**: rejected -- couples Intent's generator to an Elixir-only Hex tool and to a config the project may not want; keeps `AGENTS.md` single-source in Intent.
- **Do nothing / mark the doc stale**: rejected -- the collision is live (both tools target `.claude/skills`), so silence produces duplicated, drifting skills.

## Out of Scope

- Refreshing existing projects' root `usage-rules.md` (eg Laksa's) -- that follows from the template fix via `intent claude upgrade` after this ST ships.
- Any change to `intent agents sync`'s generator or to `AGENTS.md`'s generated structure.
