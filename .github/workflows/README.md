# GitHub Actions Workflows

This directory holds the workflows GitHub runs for the Intent project: the bats suite (`tests.yml`), the Rust workspace gates (`rust.yml`), the pull-request checks (`pr-checks.yml`) and the Linux packages (`linux-release.yml`).

## Workflows

### 1. Intent Tests (`tests.yml`)

**Triggers**: push to `main`, push to a release candidate `candidate/**`, pull requests targeting `main`, and `workflow_dispatch`, except when every changed path is under `intent/whiteboard/**` or `intent/.canon/**` (`paths-ignore`). Never a tag (ST0091 WP-04): devbin's release core pushes the release commit to `candidate/vX.Y.Z` and reads this workflow's run there (`release.ci.workflow`) before it tags, so the commit that ships is green on every leg before it is public.

**Concurrency**: a newer run of the same workflow for the same event and ref cancels the one in progress.

**What it does**:

- Builds the v3 release binaries (`intent` and `intentd`) with cargo, then runs the bats suite against them on Ubuntu and macOS
- Runs ShellCheck over the repository's shell scripts (`bin/devbin`, `bin/int`, and the shell scripts under `bin/.devbin/cmd/` and `lib/templates/hooks/`), non-blocking
- Aggregates the Ubuntu and macOS legs into one pass/fail

**Jobs**:

- `test-linux` (Test on Ubuntu): installs bats-core v1.12.0 from its GitHub source archive and jq from apt, puts `bin/` and `native/rust/target/release` on PATH, sets a CI git identity, clones the bats libraries into `tests/lib`, installs the stable Rust toolchain (rustfmt, clippy), Erlang/OTP 29 and Elixir 1.20 (`erlef/setup-beam`) and prettier 3 (npm), blanks the XDG_* variables, builds the pair, then runs `tests/run_tests.sh`.
- `test-macos` (Test on macOS): only for a run by hand (`workflow_dispatch`). A push to main, a pull request and a release candidate run Linux alone (hv, fleet decision 30 and hv decision 97): the cut's macOS leg on rhadamanth is a release's macOS proof. Otherwise the same, with bats-core, jq, shellcheck and bash from Homebrew; Homebrew's bash goes first on PATH because devbin's handlers refuse macOS's `/bin/bash` 3.2
- `rust`: only for a candidate or a run by hand, it calls `rust.yml` (`workflow_call`), so a release's cargo verdict is inside the one run the cut reads.
- `shellcheck` (Shell Script Analysis): runs `shellcheck` on `bin/devbin`, `bin/int`, and every file under `bin/.devbin/cmd/` and `lib/templates/hooks/` that `file` reports as a shell script; findings never fail the job. The sourced libraries under `bin/.devbin/cmd/shared/` carry no shebang, so `file` reports them as text and they are not checked.
- `test-summary` (Test Summary): runs after `test-linux`, `test-macos` and `rust` whatever their result. It fails unless `test-linux` succeeded, and unless each of the other two succeeded where it was due: `test-macos` for a run by hand, and `rust` for a candidate or a run by hand. A leg skipped where it was not due is no failure. It does not wait on `shellcheck`.

`tests/run_tests.sh` with no argument runs every `.bats` file under `tests/` (excluding `tests/lib/`) through devbin's lanes runner, the copy vendored at `bin/.devbin/lib`, and exits non-zero if any test fails or does not run, so a failing test fails the job. On a runner without GNU parallel it runs one lane, one test at a time, and its first line says so.

### 2. rust (`rust.yml`)

**Triggers**: push to `main` and pull requests, only when a changed path is under `native/rust/**`, `schema/**`, `surface/**` or `.github/actions/fleet-rust/**`, or is `rust.yml` itself; `workflow_call` from `tests.yml` for a release candidate or a run by hand, whatever changed; and `workflow_dispatch`.

**Concurrency**: a newer run of the same workflow for the same event and ref cancels the one in progress.

**Job** `rust`, working in `native/rust`, with `fail-fast: false`. It runs on `ubuntu-latest`, and on `macos-latest` as well only for a run by hand (hv decision 97, as for `test-macos`):

- Installs the fleet's Rust (`.github/actions/fleet-rust`, the one home of the version CI follows) with rustfmt and clippy, restores the cargo cache, and records `rustc --version` and `cargo --version` to the job summary
- Installs shellcheck (macOS only) and `prettier@3` (both legs); the test suite needs both on PATH
- Runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, IN-RS-CODE-001's own step `cargo clippy -p intentsvcs -p intent-cli --lib -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic` (the library targets only, never `--all-targets`) and its step over intentd's binary `cargo clippy -p intentd --bins -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic` (testkit is left out on purpose), IN-RS-CODE-002's step `cargo clippy --workspace --lib --bins -- -D clippy::needless_pass_by_value` (issue 0649), `cargo doc --no-deps --document-private-items` under `RUSTDOCFLAGS=-Dwarnings` so every intra-doc link must resolve (issue 0451; the same command as devbin's `doc` gate, which `tests/unit/devbin_rust_gates.bats` holds them to), and `cargo test --workspace --no-fail-fast`

### 3. PR Checks (`pr-checks.yml`)

**Triggers**: pull request events `opened`, `synchronize` and `reopened`.

No concurrency group: every run completes.

**Jobs**:

- `validate-steel-thread`: builds the v3 `intent` binary and runs `.github/scripts/pr-thread-ref.sh`, which asks the binary's id door, `intent id`, for the first word of the PR description that names a thread, an ordinal id (`ST0001`) or a git-style one (`ST3f9a2c1`); the door's answer is the existence check. A referenced thread that does not exist fails the job; a description with no reference passes with a suggestion to add one. The description reaches the step as an environment variable and is read only as text, so nothing written in it runs.
- `check-documentation`: warns when the diff against `origin/main` touches a path under `bin/` or `native/` and no path ending `.md` or containing `usr/` or `doc/`. The job fetches the full history so that `origin/main` is there, and it fails only when that diff cannot be taken.
- `test-coverage`: warns when the diff against `origin/main` touches a path under `bin/` or `native/` and no path containing `tests/`. Like `check-documentation`, it fetches the full history and fails only when that diff cannot be taken.
- `commit-message-check`: warns for each commit subject in `origin/main..HEAD` shorter than 10 or longer than 72 characters. Never fails.
- `pr-size-check`: reports additions plus deletions, warns above 1000 changed lines and notes above 500. Never fails.
- `doctor-on-the-merge-result`: builds the v3 `intent` binary and runs `intent doctor` on the pull request's merge result (`refs/pull/<n>/merge`), from a depth-1 checkout under a runner HOME that holds no Intent state, so the store loads from the committed canon as a collaborator's clone would. Doctor's exit code is the verdict: a counted finding fails the job. It is the merge-result twin of the pre-commit gate's doctor arm, which judges only the author's own tree. Unlike the gate's arm, which refuses only on exit 1 and lets any other exit through as UNENFORCED, this job fails on any non-zero exit, including 4 (an estate doctor cannot judge).

### 4. linux release (`linux-release.yml`)

**Triggers**: push to a release candidate `candidate/**` (ST0091 WP-04), and `workflow_dispatch`. Never a tag, and it writes no release object: devbin's release core reads this run at the release commit at its object step and attaches the packages (`release.assets.workflow`), then writes `SHA256SUMS` over every asset.

**Concurrency**: a newer run of the same workflow for the same event and ref cancels the one in progress.

**Job** `package`, on an x86_64 (`ubuntu-latest`) and an aarch64 (`ubuntu-24.04-arm`) leg with `fail-fast: false`:

- Installs the fleet's Rust (`.github/actions/fleet-rust`) with the leg's musl target and `musl-tools`, and records `rustc --version` and `cargo --version` to the job summary
- Runs `bin/int linux doctor`, `build`, `package`, `smoke`, and `smoke --in oraclelinux:7-slim`: the same subcommands hv's Ubuntu VM runs by hand, so both prove one set of steps (`bin/.devbin/cmd/linux`)
- `build` refuses a binary that is not static; `smoke` unpacks the package, reaches `intent` through a link on PATH, and checks that it starts, resolves its install root to the unpacked directory, and runs a hook script from the support tree
- Uploads `intent-<triple>.tar.gz` as the run's artifact

## Local Testing

Build and test through `bin/devbin`, never with cargo or `tests/run_tests.sh` directly (`AGENTS.md`). Where `intent estate status` shows that the estate has a controller, ask it for a slot first (`intent estate slots ask`).

```bash
# Build the pair the bats suite drives
bin/devbin build all

# Run every bats file
bin/devbin test shell

# Run one bats file
bin/devbin test shell tests/unit/pre_commit_hook.bats

# Every cargo step of rust.yml, in a private worktree
bin/devbin gate rust
```

The bats suite drives `native/rust/target/release/intent`; set `INTENT_BIN` to drive another binary. `bin/devbin build all` builds a tree with uncommitted changes under its build inputs into `native/rust/target/private/release`. It verifies a clean tree's pair, and only then promotes it into `native/rust/target/release`.

Suites that commit through the pre-commit hook (eg `pre_commit_hook.bats`) also need an `intent` on PATH, because the hook refuses a commit in an Intent project when it cannot run `intent`. The Rust tests need `shellcheck` and `prettier` on PATH. The bats suite also needs prettier, rustfmt, elixir and npx on PATH (CI installs jq as well), and on macOS a bash of version 5 or later ahead of `/bin/bash`: an arm whose tool is missing fails rather than skips. `INTENT_ALLOW_MISSING_<TOOL>=1` (eg `INTENT_ALLOW_MISSING_ELIXIR=1`) waives one deliberately.

## Workflow Maintenance

### Dependencies

- **GitHub Actions**: `actions/checkout@v7`, `actions/upload-artifact@v7`, `dtolnay/rust-toolchain@master` (through the local `.github/actions/fleet-rust`, which pins the fleet's version), `Swatinem/rust-cache@v2`, `erlef/setup-beam@v1`
- **Bats**: bats-core v1.12.0 installed with its own `install.sh` from the GitHub source archive on Ubuntu; Homebrew's `bats-core` on macOS
- **Bats libraries**: bats-support, bats-assert and bats-file, cloned from GitHub into `tests/lib` if not present, retrying a failed clone
- **Tools**: jq on both bats legs; shellcheck installed on macOS (Ubuntu runners ship it); prettier 3 from npm on every leg of both `tests.yml` and `rust.yml`; Erlang/OTP 29 and Elixir 1.20 on both bats legs; Homebrew's bash on the macOS bats leg

### Test Environments

- **Ubuntu**: `ubuntu-latest`
- **macOS**: `macos-latest`

### Behaviour

- ShellCheck findings in the `shellcheck` job are non-blocking
- A failing bats test fails its leg, and `test-summary` then fails
- All `bin/*` files and `tests/*.sh` are made executable before the suite runs

## Adding New Tests

1. **Bats tests** go in `tests/unit/` as a `.bats` file that loads `../lib/test_helper.bash`
2. **Rust integration tests** go under a crate's `tests/` and are declared as a `#[path]` module in that crate's `tests/suite.rs` (every crate sets `autotests = false`; see `intent/docs/notes/tn001-one-test-target-per-crate.md`)
3. **Run the tests through `bin/devbin`** before you push (Local Testing, above)
4. **Reference the steel thread** in your PR description (eg "Implements ST0042"); the check fails if the referenced thread does not exist
5. **Update the documentation** when you add a command or change behaviour
