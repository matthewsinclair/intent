## Intent LLM Guidance Upgrade -- Intent

Mode: APPLY

Phase 1: Diagnosis

Checking version...
intent_version: 2.10.0 (current)

Checking LLM guidance files...
AGENTS.md: UP TO DATE (version: 2.10.0)
RULES.md: EXISTS (0
0 rules/sections)
ARCHITECTURE.md: EXISTS

Checking for deprecated files...
AGENTS-phx.md: not present
llm_preamble.md: not present
usage-rules.md: not present

Checking for deprecated skill names...
No deprecated skill names found

Checking installed subagents...
critic-elixir: UP TO DATE
critic-lua: UP TO DATE
critic-rust: UP TO DATE
critic-shell: UP TO DATE
critic-swift: UP TO DATE
diogenes: UP TO DATE
intent: UP TO DATE
socrates: UP TO DATE

Checking installed skills...
in-ash-ecto-essentials: UP TO DATE
in-autopsy: UP TO DATE
in-cost-analysis: UP TO DATE
in-debug: UP TO DATE
in-detrope: UP TO DATE
in-elixir-essentials: UP TO DATE
in-elixir-testing: UP TO DATE
in-essentials: UP TO DATE
in-finish: UP TO DATE
in-handoff: UP TO DATE
in-next: UP TO DATE
in-phoenix-liveview: UP TO DATE
in-plan: UP TO DATE
in-review: UP TO DATE
in-session: UP TO DATE
in-standards: UP TO DATE
in-start: UP TO DATE
in-tca-audit: UP TO DATE
in-tca-finish: UP TO DATE
in-tca-init: UP TO DATE
in-tca-remediate: UP TO DATE
in-tca-synthesize: UP TO DATE
in-verify: UP TO DATE

Checking canon artefacts...
.claude/settings.json: UP TO DATE
.claude/scripts/session-context.sh: UP TO DATE
.claude/scripts/require-in-session.sh: UP TO DATE
.claude/scripts/post-tool-advisory.sh: UP TO DATE
.git/hooks/pre-commit: CHAINED (existing hook preserved)
.intent_critic.yml: PRESENT (user-owned)
CLAUDE.md (root): UP TO DATE
usage-rules.md (root): PRESENT (user-owned)
intent/llm/MODULES.md: PRESENT
intent/llm/DECISION_TREE.md: PRESENT
intent/.treeindex/.treeindexignore: PRESENT (project-owned)

Phase 2: Upgrade Plan

Manual review needed:
! REVIEW intent/llm/RULES.md against template for missing rules
! REVIEW intent/llm/ARCHITECTURE.md for completeness

Phase 3: Applying Changes

---

Upgrade applied successfully.

Remaining manual tasks:
! REVIEW intent/llm/RULES.md against template for missing rules
! REVIEW intent/llm/ARCHITECTURE.md for completeness

Next steps:

1. Review generated files and customize for your project
2. Run 'intent doctor' to verify configuration
3. Commit changes
