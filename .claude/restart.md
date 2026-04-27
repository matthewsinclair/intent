# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, plus per-language skills. Releases the `UserPromptSubmit` strict gate via the per-project sentinel.
2. **Verify the working tree.** `git status` should be clean. ST0035 14 of 19 Done; WP-15 (canary rollout) WIP.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/15/info.md`** -- WP-15 spec (note: `intent upgrade --dry-run` doesn't exist; Sites subdir doesn't exist; Pplr out of scope -- spec tidy-up is a sub-task before more canaries).
5. **Read `intent/st/ST0035/WP/15/canary-reports/laksa.md`** -- the Laksa canary report doubles as a template for the next canary (12-point verification commands at the bottom of the table).

## State (2026-04-27, end of session)

**Intent v2.10.0. ST0035 14 of 19 Done. Laksa canary done (1 of 16 in-scope projects). Conflab + Lamplight deferred (busy); Pplr out of scope. Tests 785/785; doctor clean. Pre-commit chain block now wired in Intent itself (auto-inserted, not manual paste).**

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 785/785 green (was 781; +4 new chain-block + REVIEW scenarios).
- **Doctor**: clean.
- **Backup tag**: deleted (`wp08-pre-relocate` removed; v2.10.0 stable).

## What landed this session (newest first)

- `2e90556` -- WP-15 Laksa canary report.
- `a729ec64` (in **Laksa**) -- `chore: apply ST0035 + ST0036 canon (v2.10.0 rollout canary)`.
- `f5d9df9` -- housekeeping: untrack `.claude/settings.local.json`; gitignore `/AGENTS.md.bak`.
- `d0d0dc6` -- populate Intent's RULES.md + ARCHITECTURE.md (no longer verbatim \_default).
- `9315bb6` -- canon-installer rough edges (auto-insert chain block; markered idempotence; REVIEW only on verbatim \_default; AGENTS.md footer; rule-count rendering).
- `9a6387b` -- WP-14 Intent self-dogfood verification.

## Resume target -- next canary (ST0035/WP-15 continued)

User direction: do other fleet projects one at a time before Conflab/Lamplight (busy). Pplr out of scope.

Candidates: **Molt**, **Utilz**, **Arca**, **Prolix**, **MicroGPTEx**, **Sites**. Recipe (Laksa-tested):

1. Clean tree on the canary project (reset any stale `.intent/config.json` bumps).
2. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent claude upgrade )` -- canon dry-run.
3. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent upgrade )` -- chain migration + canon-apply.
4. 12-point verification (commands in Laksa's canary report).
5. Add `/AGENTS.md.bak` to project `.gitignore` if missing.
6. Commit + push to `local` (NOT upstream).
7. Write `intent/st/ST0035/WP/15/canary-reports/<project>.md`.
8. Commit the report in Intent.

After 2-3 more canaries, consider switching to batch mode for the rest of the ecosystem.

## Risks for next session

- **WP-15 spec drift**: info.md mentions `intent upgrade --dry-run` (doesn't exist), Sites subdir (Laksa doesn't have one), and "Conflab + Lamplight + Laksa" as the canary set (now 16 projects minus Pplr). Worth a 5-minute tidy-up before the next canary.
- **Per-project pre-flight**: any project with stale `.intent/config.json` bumps or other pending edits should be reset / committed before canon-apply. Laksa had a stale 2.8.2 -> 2.9.0 bump that would have collided with the chain migration.
- **CLAUDE.md drift in older projects**: pre-existing user CLAUDE.md (STP-era text) is preserved by the canon. Refresh is a separate decision; track per-project for the WP-17 dogfood journal.

## Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims; auto-detection rejected.
- Document first, code next.
- Pre-flight every canary: clean tree before applying.

## Lessons from this session

- **Auto-insert beats manual paste.** The "print snippet, ask user to paste" flow left a known-deferred state on every project. Replacing with a markered idempotent insert removed the deferred bucket entirely. Marker-pair detection makes re-runs a guaranteed no-op.
- **REVIEW warnings should be conditional.** Firing the same warning unconditionally was noise. `cmp -s` against the `_default` template makes it meaningful: warning only fires when the user really hasn't customised yet.
- **Linter-vs-generator oscillation is a real bug.** AGENTS.md `---` footer needed a trailing blank line to match prettier. Without it, every regen flipped the file. MD5 sanity surfaced it on second re-apply.
- **Pre-flight on canary projects matters.** Laksa had a stale manual config bump; resetting to HEAD let the migration write the canonical version end-to-end. Canary discipline: clean tree -> clean migration.
- **Dogfood the canon yourself first.** The chain-block auto-insert was tested in Intent before being shipped to Laksa. Intent's own commit triggered the chain block, which validated the path before any downstream project was touched.

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
