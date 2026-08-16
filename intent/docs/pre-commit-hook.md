# Pre-commit critic hook

Intent's canonical pre-commit hook runs `intent critic <lang> --staged --severity-min <sev>` for each language it detects in the project. Findings at or above the configured severity threshold block the commit. The hook is the primary cadence for rule enforcement (design decision D8 in `intent/st/COMPLETED/ST0035/design.md`): local, deterministic, offline, zero-latency feedback.

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

## Whiteboard guards

Projects that run the multi-session whiteboard protocol get a second set of checks, ahead of the critic. They are **opt-in by the presence of `intent/whiteboard/`** — a project without a board is not one they have an opinion about, and nothing changes for it.

| Guard                        | Refuses                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------- |
| `whiteboard-clock-guard.sh`  | a timestamp that cannot be a real clock read (future, missing `Z`, backwards) |
| `whiteboard-header-guard.sh` | a header value that has been YAML-escaped — the header block is not YAML      |

Three properties are worth knowing, because they are what make the guards keepable:

- **One concern per guard, and the hook runs all of them before deciding.** Stopping at the first refusal costs a node one commit attempt per defect, and a board with a bad stamp and an escaped value is one editing session. Each guard prints its own report; the hook only aggregates the verdict.
- **Neither ever auto-corrects.** Both print the corrected line so the fix is a copy-paste. A guard that silently repairs the value hides the class from the node that needs to learn it.
- **Only this hook is copied into your project.** The guard bodies are resolved at runtime out of `INTENT_HOME`, so a new or updated guard reaches every project on the next `intent upgrade` without anyone touching `.git/hooks/`. If a board is present and a guard is not found, the hook says so on stderr and names what went unchecked rather than passing in silence.

## Language detection

The hook reads the explicit `languages` array from `intent/.config/config.json` and dispatches one critic per entry:

```bash
jq -r '(.languages // []) | .[]' intent/.config/config.json
```

An empty or absent array means no language critics run. This replaced filesystem-marker detection (`mix.exs` ⇒ elixir, and so on) in v2.11.0 / ST0037, on the grounds that file presence is not evidence of language-in-use. Declare with `intent lang init <lang>`; remove with `intent lang remove <lang>`.

`intent critic` owns the code-versus-prose classification from its single registry, so a prose discipline (`author`, `content`) returns a clean no-op here rather than needing the hook to know anything about languages.

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
