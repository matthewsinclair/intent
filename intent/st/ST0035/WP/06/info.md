---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-06
title: "Ship .git/hooks/pre-commit template for critic gate"
scope: Small
status: Not Started
---

# WP-06: Ship .git/hooks/pre-commit template for critic gate

## Objective

Author the `.git/hooks/pre-commit` template at `lib/templates/hooks/pre-commit.sh`. The hook invokes `bin/intent_critic <lang> --staged --severity-min <threshold>` on the staged files and blocks the commit if findings at or above threshold are reported. Config loaded from `.intent_critic.yml` (WP07). Hook is installed (chmod +x) into each project's `.git/hooks/` by `intent claude upgrade --apply` (WP11).

## Context

D8 picks pre-commit as the primary critic cadence — local, deterministic, offline, zero-latency feedback. CI and session-end reminders are secondary.

Git hooks live in `.git/hooks/`. They are not versioned by git (`.git/` is the local repo metadata). Intent installs the hook via `intent claude upgrade --apply`; users can also install manually by copying the template. `--no-verify` remains available as the escape hatch; it's logged (via hook output) so sweeps can identify bypasses.

WP06 depends on WP05 (`bin/intent_critic`). It does not depend on WP07 (`.intent_critic.yml`) — the hook works fine with defaults when the config is absent; WP07 is a refinement.

## Deliverables

1. **Template** at `lib/templates/hooks/pre-commit.sh`. Responsibilities:
   - Detect project language (read `.intent/config.json` + look for language markers: `mix.exs`, `Cargo.toml`, etc.).
   - If language is unsupported by critics, `exit 0` (no-op) with an advisory stderr note.
   - If `.intent_critic.yml` exists, load its severity threshold and disabled-rules list. Default threshold: `warning`.
   - Invoke `intent critic <lang> --staged --severity-min <threshold> --format text`.
   - On exit code 1 (findings-above-threshold): print the findings, print a one-line opt-out hint (`git commit --no-verify`), exit 1 (block).
   - On exit code 0: exit 0 (allow).
   - On exit code 2 (intent critic invocation error): print error, exit 0 (fail-open — don't block a commit because the tool is broken).
2. **Template installer** — invoked by `intent claude upgrade --apply` (WP11): copies template to `.git/hooks/pre-commit`, chmod +x. If an existing hook is present, write `.git/hooks/pre-commit.intent` and print instructions for chaining (don't overwrite user's hook).
3. **Multi-language projects**: for projects with multiple languages (e.g., Elixir + shell scripts), run critic per language. Iterate; aggregate exit codes.
4. **MODULES.md registration** for `lib/templates/hooks/pre-commit.sh`.
5. **Documentation**: brief `intent/docs/pre-commit-hook.md` explaining install, opt-out, configuration, CI integration.

## Approach

1. Study existing `.git/hooks/pre-commit.sample` for the hook contract (exit codes, input).
2. Study `bin/intent_critic` CLI (from WP05) for exact invocation.
3. Author the template script:

   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   # Discover project root
   PROJECT_ROOT="$(git rev-parse --show-toplevel)"
   cd "$PROJECT_ROOT"

   # Bail if intent isn't available
   if ! command -v intent >/dev/null 2>&1; then
     echo "intent CLI not found on PATH; skipping critic gate." >&2
     exit 0
   fi

   # Detect language(s)
   LANGS=()
   [ -f mix.exs ] && LANGS+=(elixir)
   [ -f Cargo.toml ] && LANGS+=(rust)
   [ -f Package.swift ] && LANGS+=(swift)
   # ... etc ...
   LANGS+=(shell)  # always check shell scripts if any are staged

   # Load config
   SEVERITY="warning"
   [ -f .intent_critic.yml ] && SEVERITY=$(grep '^severity_min:' .intent_critic.yml | awk '{print $2}' || echo warning)

   # Run critic per language
   FAIL=0
   for lang in "${LANGS[@]}"; do
     intent critic "$lang" --staged --severity-min "$SEVERITY" --format text || FAIL=$?
   done

   # Handle exit code
   if [ "$FAIL" -eq 1 ]; then
     echo "" >&2
     echo "Commit blocked by intent critic findings (severity ≥ $SEVERITY)." >&2
     echo "To bypass (use sparingly): git commit --no-verify" >&2
     exit 1
   elif [ "$FAIL" -eq 2 ]; then
     echo "Critic invocation error; allowing commit to proceed." >&2
     exit 0
   fi

   exit 0
   ```

   (Final content refined during WP06 — script above is the design sketch.)

4. Build the installer component of WP11's `intent claude upgrade --apply`. For WP06 scope, just ship the template + a verification that the installer script (WP11) will correctly copy it.
5. Author docs at `intent/docs/pre-commit-hook.md`: install, configure, opt-out, CI recipe, troubleshooting.
6. MODULES.md update.
7. Commit.

## Acceptance Criteria

- [ ] `lib/templates/hooks/pre-commit.sh` exists and is a valid bash script.
- [ ] `bash -n lib/templates/hooks/pre-commit.sh` (syntax check) returns 0.
- [ ] Template invokes `intent critic` with `--staged` and `--severity-min`.
- [ ] Template reads `.intent_critic.yml` if present; falls back to `severity_min: warning` otherwise.
- [ ] Template detects multiple languages and runs critic per language.
- [ ] Template handles missing `intent` CLI gracefully (exit 0 with advisory).
- [ ] Template handles `intent critic` invocation error gracefully (exit 0 with advisory — fail-open).
- [ ] Template handles findings-above-threshold correctly (exit 1, block commit).
- [ ] Manually tested: in a scratch repo with a staged rule violation, hook blocks commit with expected output.
- [ ] Manually tested: `git commit --no-verify` bypasses hook.
- [ ] `intent/docs/pre-commit-hook.md` exists and covers install / configure / opt-out / CI / troubleshooting.
- [ ] MODULES.md registers `lib/templates/hooks/pre-commit.sh`.
- [ ] Commit follows Intent conventions.

### Tests to add

- **BATS integration test**: stage a known-bad file; invoke hook; assert exit 1 and specific stderr output.
- **BATS integration test**: stage a known-good file; invoke hook; assert exit 0.
- **BATS integration test**: `intent` CLI not on PATH; invoke hook; assert exit 0 with advisory.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP05 (`bin/intent_critic` must exist).
- **Blocks**: WP11 (upgrade installs the hook).

## Implementation Notes

- **Fail-open posture**: if `intent critic` fails (invocation error, missing binary), don't block the commit. The hook is a quality gate, not an availability gate. Document this in the hook's output so users understand why it passed.
- **Multi-language coverage**: iterate languages; the hook runs each critic in turn. For performance, skip a language if no staged files match that language's extensions (the critic runner should already handle this, but a hint in the hook saves spawn time).
- **Always run `shell` critic on staged shell files**: regardless of project primary language, if any staged `.sh` / `.bash` / `.zsh` files exist, run `critic-shell`.
- **`--no-verify` logging**: don't try to log bypasses locally (hook doesn't see bypasses). CI (future) catches them at push time.
- **Don't overwrite existing hooks**: the WP11 installer writes to `.git/hooks/pre-commit.intent` if `.git/hooks/pre-commit` is present and not the same content. Document chaining (source the Intent hook from the user's existing hook).

## Risks and Edge Cases

- **Risk**: `intent critic` binary not on PATH in a given env. **Mitigation**: fail-open (exit 0 with advisory); document PATH requirement in `pre-commit-hook.md`.
- **Risk**: Hook slows down commits unacceptably on large staged changesets. **Mitigation**: `intent critic --staged` only looks at staged files; performance should match WP05 budget. Monitor; if problematic, add `--parallel` or `--timeout` flags later.
- **Risk**: Different bash versions (3.2 macOS vs 5.x Linux). **Mitigation**: test on both. Avoid bash 4+ features (no `declare -A`, no `;;&`).
- **Risk**: User has a pre-existing pre-commit hook. **Mitigation**: installer writes to `.pre-commit.intent`, prints instructions for chaining. Never overwrite.
- **Edge**: Commit with only non-source files (docs, config). Hook runs, critic reports nothing, exit 0.
- **Edge**: Commit amend / revert. Hook fires as usual; no special handling needed.
- **Edge**: Git commit in detached HEAD or worktree. Hook still works (hooks are git-level, not branch-level).

## Verification Steps

1. `bash -n lib/templates/hooks/pre-commit.sh` — syntax valid.
2. In a scratch repo, copy the template to `.git/hooks/pre-commit`, chmod +x, stage a known-bad file, try to commit — confirm block.
3. Same, but stage a known-good file — confirm pass.
4. `git commit --no-verify` path — confirm bypass works.
5. Manual CI recipe test — run the same `intent critic --staged` command in a GitHub Actions-style environment.
6. `tests/run_tests.sh` — all green with new BATS tests.

## Size and Estimate

- **Size**: S (Small). 1–2 sessions.
- Session 1: Template script + language detection + docs.
- Session 2: BATS tests + MODULES.md + commit.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Template committed.
- [ ] Docs committed.
- [ ] BATS tests green.
- [ ] Coordinated with WP05 on `intent critic` CLI.
- [ ] Coordinated with WP11 on installer logic (WP11 consumes this template).
