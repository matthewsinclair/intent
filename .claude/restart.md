# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show `a64a09b` (regression guard) and `46672e4` (idempotence fix) as the latest commits.
3. **Read `intent/wip.md` and `intent/restart.md`.** Pay attention to the "Pending: ST0037" section in `wip.md` — that is the operative work for this next session.

## State (2026-04-28, end of session — ST0037 queued for v2.11.0)

v2.10.1 shipped earlier today (tag `v2.10.1` at `c453cbb`, both remotes, GitHub release live). CI run `25050807366` green on the latest pushed commit (`a64a09b`). Tests 839/839 local. `intent doctor` clean.

This session ran past ship into v2.10.1 follow-up, blog drafting + detrope, and the discovery of a regression in language detection.

## Resume target — ST0037

The user's design intent: **languages-in-use is a per-project CONFIG decision, not filesystem detection**. Filesystem-probe-based detection is alive in four canon sites; none of them reads explicit config. The imperative-config mechanism (`intent lang init`) exists but no consumer reads its output. Decision agreed: **Option B** — explicit `languages: []` field in `intent/.config/config.json`, with a back-fill migration. User direction: "fix this PROPERLY, no half-measures."

After `/in-session`:

1. `intent st new "Language config: replace filesystem probes with explicit config"`.
2. Read `intent/wip.md` "ST0037 scope (13 items)" for the full plan.
3. Document first (the new ST's `info.md` and `design.md`), code next.
4. Execute the 13 items.
5. Bump VERSION to 2.11.0 and add CHANGELOG section.
6. Update both blog drafts in `docs/blog/_drafts/`.

T-shirt: **M**.

## What landed this session (newest first)

- `a64a09b` — test: regression guard for skill-renderer positional-token trap (`tests/unit/skill_renderer_trap.bats`).
- `46672e4` — fix: anchor `AGENTS.md` version probe to `_Generated` footer.
- `01c60a9` — chore: session wrap (from earlier session).

Plus two blog drafts in `docs/blog/_drafts/`:

- `####-critic-shell-on-intent.md` — light freshening of the inception draft, detrope-clean.
- `####-claude-context-with-intent.md` — substantial v2.10.x rewrite of the supercharge-Claude post, detrope-clean except for the language-detection paragraph that ST0037 unblocks.

## Lessons from this session (top three)

- **Detrope discipline scales when applied per-pass.** Three passes per blog draft, with audit after each, eliminates most AI tells. Watch for: bold-first bullets, "X, not Y" / "X rather than Y" / "X instead of Y" forms, three-parallel-verb tricolons, symmetric framing pairs, slogan closers, magic adverbs, action cliches.

- **Prose-level discomfort can surface system-level regressions.** The user flagged a tropey language-detection paragraph; investigation surfaced a regression in the underlying mechanism. Sometimes the bug is one layer down from where it's first noticed.

- **Idempotence is a property of probes.** The AGENTS.md version probe was greedy; once content moved around, the probe started reading the wrong value every run. Anchor probes to specific markers, never generic patterns.

## Risks for post-compact

- v2.11.0 schema migration adds a `languages: []` field. Migration must back-fill from existing `intent/llm/RULES-<lang>.md` presence; existing fleet projects shouldn't need user action.
- BATS test rework on probe sites: `in_session_skill.bats`, `pre_commit_hook.bats`, `intent_lang.bats`. Probe assertions out, config-read assertions in.
- Polyglot order: array order = explicit; first entry = primary where primary matters. Document the convention up front in the design doc.
- Pre-commit hook reads JSON config — use `get_config_field` from `intent_helpers`, don't re-implement.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next.
- Pre-flight every canary: clean tree before applying.
- SKILL.md inline bash with `$N` positional fields gets mangled. Use a script file invoked by path.
- **NEW**: Languages-in-use is a CONFIG decision (`languages: []` field). NOT filesystem detection. Probe sites are regressions.
