# GitHub Actions Workflows

This directory holds the workflows GitHub runs for the Intent project: the bats suite (`tests.yml`), the Rust workspace gates (`rust.yml`) and the pull-request checks (`pr-checks.yml`).

## Workflows

### 1. Intent Tests (`tests.yml`)

**Triggers**: push to `main` and pull requests targeting `main`, except when every changed path is under `intent/whiteboard/**` (`paths-ignore`).

**What it does**:

- Builds the v3 release binaries (`intent` and `intentd`) with cargo, then runs the bats suite against them on Ubuntu and macOS
- Runs ShellCheck over `bin/intent*` (non-blocking)
- Aggregates the Ubuntu and macOS legs into one pass/fail

**Jobs**:

- `test-linux` (Test on Ubuntu): installs bats-core v1.12.0 from its GitHub release tarball and jq from apt, clones the bats libraries into `tests/lib`, builds `-p intent-cli -p intentd` in release mode, then runs `tests/run_tests.sh`
- `test-macos` (Test on macOS): the same, with bats-core, jq and shellcheck from Homebrew
- `shellcheck` (Shell Script Analysis): runs `shellcheck` on every file under `bin/` named `intent*` that `file` reports as a shell script; findings never fail the job. No tracked file under `bin/` matches that selector (`find bin -type f -name 'intent*'`), so the job has nothing to check.
- `test-summary` (Test Summary): runs after `test-linux` and `test-macos` whatever their result, and fails unless both succeeded. It does not wait on `shellcheck`.

`tests/run_tests.sh` with no argument runs every `.bats` file under `tests/` (excluding `tests/lib/`) in one `bats` invocation and exits non-zero if any test fails, so a failing test fails the job.

### 2. rust (`rust.yml`)

**Triggers**: push to `main` and pull requests, only when a changed path is under `native/rust/**`, `schema/**` or `surface/**`, or is `rust.yml` itself.

**Job** `rust`, on a `macos-latest` and `ubuntu-latest` matrix with `fail-fast: false`, working in `native/rust`:

- Installs the stable toolchain with rustfmt and clippy, restores the cargo cache, and records `rustc --version` and `cargo --version` to the job summary
- Installs shellcheck (macOS only) and `prettier@3` (both legs); the test suite needs both on PATH
- Runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace --no-fail-fast`

### 3. PR Checks (`pr-checks.yml`)

**Triggers**: pull request events `opened`, `synchronize` and `reopened`.

**Jobs**:

- `validate-steel-thread`: builds the v3 `intent` binary, takes the first `ST####` in the PR description and runs `intent st show` on it. A referenced thread that does not exist fails the job; a description with no reference passes with a suggestion to add one.
- `check-documentation`: warns when the diff against `origin/main` touches a path containing `bin/` and no path ending `.md` or containing `usr/` or `doc/`. Never fails.
- `test-coverage`: warns when the diff against `origin/main` touches a path containing `bin/` and nothing under `tests/`. Never fails.
- `commit-message-check`: warns for each commit subject in `origin/main..HEAD` shorter than 10 or longer than 72 characters. Never fails.
- `pr-size-check`: reports additions plus deletions, warns above 1000 changed lines and notes above 500. Never fails.

## Local Testing

The bats suite drives `native/rust/target/release/intent` (set `INTENT_BIN` to drive another binary), so build it first:

```bash
# Build the binaries the bats suite drives
cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd

# Run every bats file
./tests/run_tests.sh

# Run one bats file
./tests/run_tests.sh tests/unit/pre_commit_hook.bats

# The Rust gates, as rust.yml runs them
cd native/rust
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --no-fail-fast
```

Suites that commit through the pre-commit hook (eg `pre_commit_hook.bats`) also need an `intent` on PATH, because the hook refuses a commit in an Intent project when it cannot run `intent`. The Rust tests need `shellcheck` and `prettier` on PATH.

## Workflow Maintenance

### Dependencies

- **GitHub Actions**: `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`
- **Bats**: bats-core v1.12.0 built from its release tarball on Ubuntu; Homebrew's `bats-core` on macOS
- **Bats libraries**: bats-support, bats-assert and bats-file, cloned from GitHub into `tests/lib` if not present, retrying a failed clone
- **Tools**: jq on both bats legs; shellcheck installed on macOS (Ubuntu runners ship it); prettier 3 installed from npm for `rust.yml`

### Test Environments

- **Ubuntu**: `ubuntu-latest`
- **macOS**: `macos-latest`

### Behaviour

- ShellCheck findings in the `shellcheck` job are non-blocking
- A failing bats test fails its leg, and `test-summary` fails unless both legs pass
- All `bin/*` files and `tests/*.sh` are made executable before the suite runs

## Adding New Tests

1. **Bats tests** go in `tests/unit/` as a `.bats` file that loads `../lib/test_helper.bash`
2. **Rust integration tests** go under a crate's `tests/` and are declared as a `#[path]` module in that crate's `tests/suite.rs` (every crate sets `autotests = false`; see `intent/docs/notes/tn001-one-test-target-per-crate.md`)
3. **Run tests locally** before pushing
4. **Reference the steel thread** in your PR description (eg "Implements ST0042"); the check fails if the referenced thread does not exist
5. **Update documentation** if adding new commands or changing behavior

## Troubleshooting

### Common Issues

1. **Tests pass locally but fail in CI**
   - Check for environment-specific paths
   - Ensure all dependencies are properly installed in the workflow
   - Verify file permissions are set correctly

2. **ShellCheck warnings**
   - These are non-blocking but should be addressed
   - Run `shellcheck` locally to see specific issues

### Debugging Workflows

- Check the Actions tab in GitHub for detailed logs
- Each step shows its output when expanded
- Failed steps are highlighted in red
- Use `echo` statements for debugging in workflows
