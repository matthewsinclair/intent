# Pre-commit critic hook

Intent's canonical pre-commit gate runs the repository guards, then `intent critic <lang> --staged --severity-min <sev> --format text` for each language the project declares, then `intent doctor`. Findings at or above the configured severity threshold block the commit, and so does a critic that refuses because a rule the project armed cannot be enforced, and so does a `doctor` that exits 1. The hook is the primary cadence for rule enforcement (design decision D8 in ST0035's `design.md`; `intent st edit ST0035 design --path` writes the thread's files if they are not on disk and prints the path): local, deterministic, offline, zero-latency feedback.

## Installation

`intent claude upgrade --apply` installs the gate as a carrier and a chain block in the repository's hooks directory. That is the directory `git rev-parse --git-path hooks` names: `.git/hooks/` by default, or the directory `core.hooksPath` points at (Intent's own repository uses a tracked `.githooks/`). It makes both executable (`--skip-settings` leaves `.claude/settings.json` alone while still installing the gate). The installer writes no ignore rule, so a project with a tracked hooks directory must list the `*.intent` carriers in its own `.gitignore`, as Intent's does. Where this page says `.git/hooks/`, read that directory.

- **`pre-commit.intent`, the carrier.** A copy of `lib/templates/hooks/pre-commit-shim.sh` from the Intent install. It reads the install root from `~/.local/share/intent/home` and execs that install's `lib/templates/hooks/pre-commit.sh`. The gate body is never copied into a project, so a gate fix reaches every project when the install is updated.
- **`pre-commit`, the chain block.** A block between `# intent-chain-block:start` and `# intent-chain-block:end` that runs `pre-commit.intent` when it is executable, and otherwise refuses the commit and names the path. A missing hook is created as a shebang plus the block. An existing hook keeps every line it has: the block is inserted after the hook's leading run of shebang, blank and `set` lines. The first comment or command ends that run, so a `set` line below a header comment stays below the block. A block already there is brought to the current form in place, with every line outside it kept. A hook whose block cannot be rewritten safely is left as it is and reported as `held:` with the reason. That covers two chain blocks, the retired `# >>> intent-chain-block >>>` marker, or an opener with no `# intent-chain-block:end`. Repair it by hand, then run the command again.

The same run installs the post-pull carriers `post-merge.intent`, `post-checkout.intent` and `post-rewrite.intent`, each reached from a chain block in its own hook. Each is a copy of `lib/templates/hooks/post-pull.sh`, which runs `intent sync --apply` to bring this clone's store up to what a pull, checkout or rewrite put on disk, and always exits 0. When a post-pull carrier is absent or not executable in a checkout that holds an Intent store (`intent/.cache/intent.db`), its block says so on stderr and exits 0: git has already changed the tree, and a failing `post-checkout` would make `git checkout` and `git worktree add` report failure. A checkout with no store, such as a fresh worktree, stays silent: there is nothing to bring up to date, and the first `intent` command there builds the store. They are not part of the commit gate; `docs/concepts/working-in-a-team.md` describes them.

`$XDG_DATA_HOME/intent/home` (by default `~/.local/share/intent/home`) is written by `intent bootstrap`. A carrier installed by a build before 3.0.2 reads `~/.intent/home` instead, which 3.0.2 moves; `intent claude upgrade --apply` in that project reinstalls the carrier. When it is absent, empty, or names a directory without `lib/templates/`, the carrier refuses every commit and names what it found, and `intent claude upgrade --apply` warns about it at install time. When it names an install with no pre-commit gate (no `lib/templates/hooks/pre-commit.sh`), the carrier also refuses every commit and names what it found. Check what the carrier resolves without running the gate:

```bash
.git/hooks/pre-commit.intent --where
```

`intent bootstrap --check` gives the same answer from the binary, with the same labels and the same exit code (0 when the pointer names an install, 1 when it does not; an install missing `lib/templates/hooks/pre-commit.sh` still answers `OK` and 0, and every commit then refuses), and needs no project. It also says when the gate runs from a different install from the `intent` you ran, and when the pointer names a versioned Homebrew keg that the next `brew upgrade` deletes. When the two differ, the gate body comes from the pointer's install. The guard runner and its roster come from the running `intent`'s install, the `INTENT_HOME:` line of `intent info`. So a commit runs one install's gate over another install's guards. `intent info` shows the same answer on its `Gate root:` line.

Git hooks are not versioned; every fresh clone needs `intent claude upgrade --apply` once.

Manual install of the same carrier and chain block:

```bash
root="$(intent info | sed -n 's/^ *INTENT_HOME: *//p')"
cp "$root/lib/templates/hooks/pre-commit-shim.sh" .git/hooks/pre-commit.intent
chmod +x .git/hooks/pre-commit.intent
```

`intent info` reports the install root under the `INTENT_HOME:` label. The root is resolved from the path of the running `intent` executable; the `INTENT_HOME` environment variable is not read. Then add the chain block to `.git/hooks/pre-commit` (and `chmod +x` it), ahead of your own checks:

```bash
#!/usr/bin/env bash
# intent-chain-block:start (generated by intent claude upgrade)
_intent_chain="$(git rev-parse --git-path hooks 2>/dev/null || :)/pre-commit.intent"
if [ -x "$_intent_chain" ]; then
  "$_intent_chain" "$@" || exit $?
elif [ -e "$_intent_chain" ]; then
  echo "pre-commit: GATE NOT EXECUTABLE -- the Intent gate did NOT run: no guard, no critic, no doctor." >&2
  echo "pre-commit:   the carrier has no execute bit: $_intent_chain" >&2
  echo "pre-commit:   remedy: intent claude upgrade --apply" >&2
  exit 1
else
  echo "pre-commit: GATE ABSENT -- the Intent gate did NOT run: no guard, no critic, no doctor." >&2
  echo "pre-commit:   this hook declares a gate it cannot find: $_intent_chain" >&2
  echo "pre-commit:   the carrier is gitignored, so no clone or worktree receives it." >&2
  echo "pre-commit:   remedy: intent claude upgrade --apply" >&2
  exit 1
fi
# intent-chain-block:end
# ... your checks ...
```

## Configuration

The hook reads `severity_min` from `.intent_critic.yml` at the project root (`critical`, `warning`, `recommendation` or `style`; anything else, or no file, means `warning`). `show_all: true` is shorthand for `severity_min: style`, and an explicit `severity_min` wins over it. `intent critic` reads the file's `disabled` list, and the opt-in PostToolUse advisory reads `post_tool_use_advisory`. A direct `intent critic` run does not read `severity_min`; pass `--severity-min` to it (see `docs/known-defects.md`, `intent#0288`). `intent claude upgrade --apply` seeds it from `lib/templates/_intent_critic.yml` when it is absent, and overwrites an existing one only with `--force`:

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

`--no-verify` bypasses the `pre-commit` and `commit-msg` hooks and leaves no trace in the commit; other hooks, such as `post-commit`, still run. Use sparingly. When the gate blocks on critic findings, a critic refusal or a `doctor` finding, or refuses because `intent` cannot run, it prints a one-line reminder of this escape hatch.

## Fail-open cases

The gate exits `0` (letting the commit through) when it cannot apply:

- `git` not on `PATH`, or the hook is not running inside a git worktree.
- No `intent/.config/config.json` at the worktree root (the hook is in a non-Intent repo). The repository guards have already run by this point; the critic and `intent doctor` arms are both skipped.
- A critic exits with a code other than `0`, `1` or `3` (eg `2`, the critic could not run). That language is reported `UNENFORCED` in a digest line that names it with a denominator (`N of M declared language(s) went UNENFORCED`).
- The guard runner cannot be located. The hook prints `NO guard ran for this commit` and why, then carries on to the critic. The project's declared guards run inside that runner, so they do not run either, and the commit is not blocked. The refusals under Project guards apply once the runner is found.
- `intent doctor` exits with a code other than `0` or `1`. Estate health is reported `UNENFORCED` for that commit.

Each case prints a stderr line saying what was skipped. The gate is a quality check, not an availability check.

It does **not** fail open when `intent` cannot run in an Intent project: see Troubleshooting. And the carrier refuses (exit `1`) when `~/.local/share/intent/home` does not lead to a gate, before `pre-commit.sh` runs at all.

## Repository guards

Before the critic, the gate runs the guard runner, `lib/templates/hooks/pre-commit-guards.sh`. The runner holds the guard roster; `pre-commit.sh` names no guard. Each guard applies only when its subject path exists in the repository, so a project without a whiteboard is not checked by the whiteboard guards. List the roster, each guard's applicability path, and whether it applies here:

```bash
root="$(intent info | sed -n 's/^ *INTENT_HOME: *//p')"
bash "$root/lib/templates/hooks/pre-commit-guards.sh" --list-guards
```

Each guard file's header says what it refuses and why.

- **The runner runs every applicable guard before deciding.** Stopping at the first refusal would cost one commit attempt per defect. Each guard prints its own report; the runner aggregates the verdict and prints a `guards: N ran, M skipped` summary.
- **No guard edits a file.** A guard that silently repaired a value would hide the class from the person who needs to learn it. The whiteboard header guard prints the corrected line so the fix is a copy-paste.
- **The runner and the guards are read from the install, not from the project.** The hook finds the install root through `intent info`, so a new or updated guard reaches every project when the install is updated, with nothing re-run in the project. In an Intent source checkout (a repository carrying `lib/templates/hooks/pre-commit-guards.sh` and `VERSION`) the gate reads the guards from that checkout instead and says so on stderr. When an applicable guard file is missing, the runner says so and names what went unchecked rather than passing in silence.

## Project guards

A project's own guards are declared in `intent/.config/config.json`, and the runner runs them after Intent's roster:

```json
{
  "guards": [
    { "run": ["bin/hooks/inbox-guard"] },
    { "run": ["bin/hooks/docs-check", "--strict"], "when": "docs" }
  ]
}
```

- **`run` is an argv, not a command line.** It is never handed to a shell. `run[0]` is a path relative to the project root; the remaining entries are its arguments.
- **`when` is optional.** It names a path, and the guard is skipped as not applicable when that path does not exist, like the shipped guards. A guard with no `when` runs on every commit.
- **A guard answers with its exit code, and the runner reads a project's guards the way it reads its own.** `0` means the guard ran and passed. `3` means not applicable: the guard is counted under `skipped` and never blocks, which is how a guard whose subject is not a path settles its own applicability. Any other code blocks the commit and is counted under `ran`: `1` is a refusal, and `2` is a shell's own error, such as a script that does not parse, so a guard that crashes blocks rather than passing. A guard that already exits `3` for a reason of its own stops blocking under this gate, so a refusal exits `1`.
- **The declaration is tracked, so a fresh clone runs the same guards as the checkout it came from.** A guard wired by hand into `.git/hooks/pre-commit` is not: `.git/hooks` is never cloned, and neither is `core.hooksPath`, which lives in `.git/config`.
- **A declared guard that cannot run blocks the commit** (once the guard runner is found; see above) and says why: its body is missing, it is not executable, it is not tracked (a clone would not receive it), or the `guards` array cannot be read (it is not valid JSON, `run` is not a non-empty array of strings, or `jq` is absent). The summary line counts project guards separately: `project: N ran, M skipped (not applicable)`.
- `--list-guards` prints the declared guards after the shipped ones, and its fifth column says whose each row is: `intent` or `project`.
- `intent lang init` and `intent lang remove` rewrite `config.json` and keep the `guards` array as it was written.

`intent doctor` reports these wiring states as advisories. They are not counted, and only `intent doctor --verbose` prints them; a default run, and so the gate, shows only how many there are: a line in an untracked pre-commit chain, outside Intent's chain block, that runs something no guard declares; a declared guard that the chain also runs by hand, so it runs twice; and a tracked `.githooks/pre-commit` or `bin/hooks/pre-commit` while `core.hooksPath` is unset, so git never runs it.

## Declared formatters

`staged-format-guard.sh` is a shipped guard that checks the formatting of the bytes a commit actually stages, for the formats a project declares:

```json
{
  "formatters": ["markdown", "elixir", "rust"]
}
```

- **The declaration is a project's; the body is canon's.** Unlike a `guards` entry, whose `run[0]` must be a tracked file inside the project, this guard's body is read live from the install like every other shipped guard. A project opts in by naming formats, and a project that names none gets no line from the guard: it exits `3` and is counted under `skipped (not applicable)` in the `guards:` summary.
- **The vocabulary is closed**: `markdown` (prettier), `elixir` (mix format), `rust` (rustfmt). A name outside it refuses the commit rather than being ignored, because a typo means the check its author intended is not running. A fourth format is a change to the guard in canon, never an estate-supplied command — a command a project supplies could be a formatter in write mode behind a door whose name promises a check.
- **It judges and never writes.** No `git add`, no `--write`, and every probe it creates is removed on the passing, refusing and interrupted paths. The refusal names the files and prints that formatter's own command as the remedy, for a human to run.
- **It reads the staged blob, not the working tree.** A hunk-scoped commit deliberately stages bytes the worktree does not hold; a check that read the worktree would refuse a tree that is not being committed, or pass a staged blob nobody checked.
- **A missing formatter is UNENFORCED, not unformatted.** A declared formatter whose tool is not on PATH is named in the verdict and blocks nothing, and the same distinction is kept for a Rust file rustfmt could not parse: "I could not check" is never reported as "this is wrong". An Elixir or Markdown file its formatter cannot parse is reported as not formatted.
- **Staged deletions and binary blobs are skipped**, and the Rust edition is resolved from the `rustfmt.toml` or `Cargo.toml` above each file rather than assumed. When nothing declares one, the verdict says rustfmt's default was used, so nobody is left to assume it matched the crate.

## Language detection

The hook reads the explicit `languages` array from `intent/.config/config.json` and dispatches one critic per entry:

```bash
jq -r '(.languages // []) | .[]' intent/.config/config.json
```

An empty or absent array means no language critics run, and so does a machine without `jq`; either way the gate prints a `0 of 0 declared language(s)` line naming which of the two it was. This replaced filesystem-marker detection (`mix.exs` ⇒ elixir, and so on) in v2.11.0 / ST0037, on the grounds that file presence is not evidence of language-in-use. Declare with `intent lang init <lang>`; remove with `intent lang remove <lang>`.

`intent critic` owns the code-versus-prose classification from its single registry, so a prose discipline (`author`, `content`) returns a clean no-op here rather than needing the hook to know anything about languages.

`shell` is **not** appended automatically — under the marker probe it was, and under explicit configuration it is a declaration like any other. A polyglot project that wants staged bash and zsh checked must list `shell` in `languages`. Each language's critic runs independently; a critic exit of `1` or `3` from any language blocks the commit.

## Estate health: `intent doctor`

After the critics, the gate runs `intent doctor` and reads its exit code and nothing else. It does not parse doctor's output or decide which of its classes matter: doctor splits its own classes into counted findings and advisories, and it carries that split in its exit code, which the release preflight also reads.

- Exit `0` prints `intent doctor gate: estate clean.` and the commit proceeds.
- Exit `1` prints doctor's findings, then `intent doctor: the estate disagrees with the store -- commit refused.` Each finding names what clears it. Regenerating a view discards a hand edit, so copy out anything you meant to keep first.
- Any other exit prints `intent doctor did not check (exit N) -- estate health is UNENFORCED in this commit.` and lets the commit through. This includes `4`, an estate doctor could not judge, and an older binary without the verb.

There is no flag that skips this arm; `git commit --no-verify` bypasses it along with every other arm. The critic arm and the doctor arm both report before either one refuses, so a commit carrying both kinds of defect shows both. The guards are different: a guard refusal ends the gate at once, so neither the critic nor the doctor arm runs in that attempt, and no `--no-verify` reminder is printed. The merge result of a pull request is checked by the same verb in CI: see `doctor-on-the-merge-result` in `.github/workflows/pr-checks.yml`.

## CI integration

The same command works in CI — no separate tooling. Example GitHub Actions step:

```yaml
- name: intent critic gate
  run: |
    args=()
    while IFS= read -r f; do args+=(--files "$f"); done < <(git diff --name-only --diff-filter=ACM origin/main -- '*.ex' '*.exs')
    if [ "${#args[@]}" -gt 0 ]; then
      intent critic elixir "${args[@]}" --severity-min warning --format text
    fi
```

Or, for the union of all languages the project uses, iterate over `LANGS` the same way the hook does. Any non-zero exit fails the step.

Exit codes of `intent critic`, and what the gate does with each:

| Exit | `intent critic`                                                    | Gate                                      |
| ---- | ------------------------------------------------------------------ | ----------------------------------------- |
| `0`  | Clean. No findings at or above threshold.                          | Proceeds.                                 |
| `1`  | Findings at or above threshold.                                    | Blocks.                                   |
| `2`  | Could not run (eg an unknown language, or no rules loaded).        | Fails open; the language is `UNENFORCED`. |
| `3`  | Refused: a rule the project armed needs a tool this machine lacks. | Blocks.                                   |

The hook itself exits only `0` or `1`.

## Troubleshooting

- **"commit blocked by findings" but my rule is a false positive**: disable the rule in `.intent_critic.yml`:

  ```yaml
  disabled:
    - IN-EX-TEST-001 # reason: <one-line justification>
  ```

  Always comment the reason. Future readers need to know why the project opted out.

- **"REFUSED: a rule this project armed could not be enforced here"**: install the tool the rule names (eg `shellcheck` for the shell pack), or disable that rule for the project.

- **Commit is slow**: `intent critic --staged` only reads staged files. If a single commit touches many files, individual findings may stack up. Use `intent critic <lang> --staged --severity-min critical` temporarily while iterating.

- **Hook not running**: check `ls -la "$(git rev-parse --git-path hooks)"/pre-commit*` — both must exist and be executable. Git skips a missing hook silently and a non-executable one with only a `hint:` line, and the chain block refuses the commit when `pre-commit.intent` is absent (`GATE ABSENT`) or not executable (`GATE NOT EXECUTABLE`), and names the path.

- **"cannot locate the Intent install" from `pre-commit (intent shim)`**: `~/.local/share/intent/home` is absent or empty. Run `intent bootstrap`, then re-commit. `.git/hooks/pre-commit.intent --where` shows what the carrier resolves, and `intent bootstrap --check` answers the same from any directory.

- **"'intent' CLI is not runnable, and this IS an Intent project"**: install Intent, or add the directory holding the `intent` executable to PATH in your shell rc. The message names which state it found (no `intent` on PATH, a dangling link, a directory, or a file without the executable bit) and the remedy for it. **The hook REFUSES the commit rather than skipping** (hv, 2026-08-27): reaching that message means `intent/.config/config.json` is present, so the project declared the gate, and a declared gate that cannot run is a failure rather than a repo it does not apply to. It fails open only for a repo that is _not_ an Intent project, which is tested separately and first. `git commit --no-verify` bypasses one commit if you need to land work before fixing the install.

- **Chain with an existing hook**: `intent claude upgrade --apply` inserts the chain block into your existing `.git/hooks/pre-commit` and keeps the rest of it; see "Installation" above.

## See also

- `intent/docs/critics.md` — critic contract, `.intent_critic.yml` schema, headless runner surface.
- `intent/docs/working-with-llms.md` — full canon: hooks, skills, critics, extensions.
- `lib/templates/hooks/pre-commit.sh` — the gate.
- `lib/templates/hooks/pre-commit-shim.sh` — the carrier installed as `.git/hooks/pre-commit.intent`.
- `lib/templates/hooks/pre-commit-guards.sh` — the guard runner and roster.
- `lib/templates/hooks/staged-format-guard.sh` — the staged-blob format check, declared per project by `formatters`.
- `lib/templates/_intent_critic.yml` — install default for per-project config.
