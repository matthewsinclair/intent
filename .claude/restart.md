# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Now folded with orientation: reads restart files + project rules + `intent st list` first, then loads `/in-essentials`, `/in-standards`, plus per-language skills, then releases the `UserPromptSubmit` strict gate via the per-project sentinel. The gate-release block is now in `~/.claude/skills/in-session/scripts/release-gate.sh` (extracted from SKILL.md to dodge a Claude Code skill-renderer bug -- see "Lessons" below).
2. **Verify the working tree.** `git status` should be clean if release engineering completed; otherwise check `intent/wip.md` for the in-flight commit.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read CHANGELOG.md v2.10.1 entry** for the shipped surface.

## State (2026-04-28, end of session -- v2.10.1 SHIPPED)

**Intent v2.10.1 shipped 2026-04-28.** Tag `v2.10.1` at `c453cbb`; pushed to `local` (Dropbox) + `upstream` (GitHub); GitHub release live at https://github.com/matthewsinclair/intent/releases/tag/v2.10.1.

This was the first end-to-end exercise of `scripts/release`, which landed in the same line. One invocation, one confirmation point at the push step.

- **VERSION**: `2.10.1` shipped.
- **Tests**: 836/836 green.
- **Doctor**: clean.
- **Tag**: `v2.10.1` at `c453cbb`; pushed to both remotes.
- **GitHub release**: live at the URL above.

## What landed this session (newest first)

- `c453cbb` -- release: v2.10.1 (cut commit produced by `scripts/release --patch`).
- `09700df` -- chore: promote v2.10.1 changes from Unreleased to in-progress section.
- `cf3dfa8` -- chore: v2.10.x polish bundle (dry-run UX + Diogenes alignment + IN-RS-CODE-005 carve-out).
- `0f7bcef` -- feat: add `scripts/release` for one-shot release cuts. +12 BATS scenarios.
- `f44717e` -- fix: `/in-session` gate releaser (root cause: skill renderer strips `$1` from `awk '{print $1}'` in SKILL.md inline blocks) + `intent doctor` leftover-`.intent/` warning. +14 BATS scenarios.

Plus 1 mirror change in `~/.claude/skills/in-session/`: the same release-gate.sh + SKILL.md edit applied to the user-installed skill so running Claude Code sessions self-heal on next `/in-session`.

## Resume target -- next ST or v2.11 backlog

v2.10.1 shipped. No active ST. Backlog candidates:

- **Blog draft**: `docs/blog/_drafts/####-shell-critic-inception.md`. Now has the v2.10.1 cut as second dogfood datapoint.
- **Homebrew tap**: Conflab's release script has the formula-update pattern.
- **`scripts/release` v2**: `--rollback`, prettier progress output, log-to-file mirror.

## Lessons from this session (top three)

- **Claude Code skill renderer strips `$N` positional-field tokens.** Inline `awk '{print $1}'` in SKILL.md gets injected with the `$1` silently emptied, leading to a malformed downstream value. Move any pipeline using `$N` into a real script file under `scripts/` and have the SKILL invoke it by path. The rendering bug is upstream-of-Intent; the workaround is the only thing under our control.

- **Per-project state-file scoping (`cksum`-keyed) is sound.** The gate-firing bug looked like cross-project state collision but was really a single-key lookup miss. Three concurrent project sessions coexisted in `/tmp/` without overwriting each other.

- **Single confirmation point at the push step is the right human pause.** Local + reversible steps run silently; the externally-visible push gets one y/N. Sprinkling confirmations across the flow would have been noise.

## Risks for next session

- CI status of `c453cbb` was in_progress at publish time. Check `gh run list` to confirm green.
- The skill-renderer bug applies to ALL skills with inline `$N` — there may be other skills in the `in-*` family that need the same script-extraction treatment. Audit when convenient.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims; auto-detection rejected.
- Document first, code next.
- Pre-flight every canary: clean tree before applying.
- SKILL.md inline bash with `$N` positional fields gets mangled. Use a script file invoked by path.
