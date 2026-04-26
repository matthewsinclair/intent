# Pre-commit critic hook

Intent's canonical pre-commit hook runs `intent critic <lang> --staged --severity-min <sev>` for each language it detects in the project. Findings at or above the configured severity threshold block the commit. The hook is the primary cadence for rule enforcement (design decision D8 in `intent/st/ST0035/design.md`): local, deterministic, offline, zero-latency feedback.

## Installation

`intent claude upgrade --apply` installs the hook (copies `lib/templates/hooks/pre-commit.sh` to `.git/hooks/pre-commit`, `chmod +x`). Manual install:

```bash
cp $INTENT_HOME/lib/templates/hooks/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

If a `.git/hooks/pre-commit` already exists, the Intent installer writes to `.git/hooks/pre-commit.intent` instead of overwriting. Chain it from your own hook:

```bash
#!/usr/bin/env bash
# .git/hooks/pre-commit — chain Intent critic after your own checks
# ... your checks ...
exec .git/hooks/pre-commit.intent
```

Git hooks are not versioned; every fresh clone needs the hook installed once.

## Configuration

The hook reads `.intent_critic.yml` at the project root. The install default (from `lib/templates/_intent_critic.yml`):

```yaml
severity_min: warning
disabled: []
post_tool_use_advisory: false
```

`severity_min: warning` blocks on CRITICAL + WARNING findings and lets RECOMMENDATION + STYLE through. Tune per project; see `intent/docs/critics.md` for the full schema.

## Opt-out (per-commit)

```bash
git commit --no-verify -m "..."
```

`--no-verify` bypasses all git hooks. Use sparingly; fleet-wide sweeps can detect over-use later by grepping commit metadata or CI lint history. The hook prints a one-line reminder of this escape hatch whenever it blocks.

## Fail-open cases

The hook deliberately exits `0` (letting the commit through) when the critic infrastructure itself is unavailable:

- `git` not on `PATH` (the hook is a bash script that needs git to resolve the worktree root).
- `intent` CLI not on `PATH`.
- No `intent/.config/config.json` at the worktree root (the hook was copied into a non-Intent repo).

In every case a one-line stderr advisory explains why the gate was skipped. The gate is a quality check, not an availability check — a missing tool shouldn't prevent work from being committed.

## Language detection

Per-invocation the hook enumerates `LANGS` by file marker:

| Marker                            | Language adds |
| --------------------------------- | ------------- |
| `mix.exs`                         | `elixir`      |
| `Cargo.toml`                      | `rust`        |
| `Package.swift`                   | `swift`       |
| `.luarc.json`                     | `lua`         |
| _(always, regardless of markers)_ | `shell`       |

`shell` is always included so staged bash/zsh scripts are checked even in a polyglot project whose primary language is something else. Each language's critic runs independently; the hook aggregates exit codes (any `1` blocks the commit).

## CI integration

The same command works in CI — no separate tooling. Example GitHub Actions step:

```yaml
- name: intent critic gate
  run: |
    intent critic elixir --files $(git diff --name-only origin/main | grep -E '\.exs?$') \
      --severity-min warning --format text
```

Or, for the union of all languages the project uses, iterate over `LANGS` the same way the hook does.

Exit codes (matching `bin/intent_critic` and this hook):

| Exit | Meaning                                                             |
| ---- | ------------------------------------------------------------------- |
| `0`  | Clean. No findings at or above threshold.                           |
| `1`  | Findings at or above threshold. Commit / job fails.                 |
| `2+` | Reserved (the hook itself only emits `0` or `1` after aggregating). |

## Troubleshooting

- **"commit blocked by findings" but my rule is a false positive**: disable the rule in `.intent_critic.yml`:

  ```yaml
  disabled:
    - IN-EX-TEST-001 # reason: <one-line justification>
  ```

  Always comment the reason. Future readers need to know why the project opted out.

- **Commit is slow**: `intent critic --staged` only reads staged files. If a single commit touches many files, individual findings may stack up. Use `intent critic <lang> --staged --severity-min critical` temporarily while iterating.

- **Hook not running**: check `ls -la .git/hooks/pre-commit` — must exist and be executable. `git commit` silently skips missing/unexecutable hooks.

- **"`intent` CLI not on PATH"**: install Intent globally or add `$INTENT_HOME/bin` to PATH in your shell rc. The hook fails open — it lets the commit through — rather than blocking work.

- **Chain with an existing hook**: see the "Installation" section above. Use `exec .git/hooks/pre-commit.intent` at the end of your own hook.

## See also

- `intent/docs/critics.md` — critic contract, `.intent_critic.yml` schema, headless runner surface.
- `intent/docs/working-with-llms.md` — full canon: hooks, skills, critics, extensions.
- `lib/templates/hooks/pre-commit.sh` — the hook script.
- `lib/templates/_intent_critic.yml` — install default for per-project config.
