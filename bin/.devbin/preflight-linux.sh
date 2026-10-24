#!/usr/bin/env bash
# preflight-linux.sh <sha> <host>:<path> -- CI's "Test on Ubuntu" leg, run on kovacs BEFORE a push to upstream or
# a cut, so a Linux-only failure is found here and not on GitHub after the fact (hv, 2026-10-23: "could have been
# found with a simple test run on Kovacs"). <host>:<path> is the checkout to clone over ssh, and the machine its
# summary is copied back to, where `int prepush` reads it before a push to upstream. `int prepush` prints the two
# lines that fetch this file from the commit under test and run it, with both arguments filled in. Run it in a
# terminal on kovacs.
#
# What it mirrors, step for step, from .github/workflows/tests.yml's test-linux job:
#   - a FRESH clone at the candidate commit (from the checkout over ssh, so nothing is pushed first);
#   - BEFORE the build, every line of .github/workflows/rust.yml's Linux leg, in its order: fmt, clippy over
#     every target, the library clippy, doc and the workspace's tests. The 3.2.6 candidate passed this order
#     while it ran only the bats suite, and its tag went red on GitHub on a clippy lint that only Linux's libc
#     signatures raise, in an example. The first step that fails ends the run, and the summary's first line
#     names it;
#   - XDG_CONFIG_HOME, XDG_DATA_HOME, XDG_STATE_HOME and XDG_RUNTIME_DIR set empty;
#   - bats-support, bats-assert and bats-file cloned into tests/lib;
#   - bin/ and native/rust/target/release first on PATH; a CI git identity;
#   - `cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd`;
#   - `cd tests && ./run_tests.sh` with BATS_LIB_PATH set;
#   - LANG=C.UTF-8 with LC_ALL unset, the runner's locale; any other LC_* variable this login exports is dropped
#     too, because each one overrides LANG for its own category;
#   - AND SIGPIPE IGNORED, as GitHub's runners start every step. An interactive shell has it at default, and
#     that difference hid issue 0628's class: under it, `comm | head -1` dies quietly; ignored, it fails.
# The git identity goes through GIT_CONFIG_GLOBAL, a file in the run's own directory, so the user's ~/.gitconfig
# is never touched. Everything lands in ~/work/intent-preflight/<sha>/, and the summary names every red arm. Its
# first line and where it is copied come from bin/.devbin/cmd/shared/preflight.lib in the clone, the file
# `int prepush` reads them from.
#
# THE BINARIES ARE COPIED INTO THE CLONE, NEVER LINKED. `intent` finds the install it belongs to by walking up
# from its own REAL path to a directory holding lib/templates/. CI builds in-tree, so that path is inside the
# checkout. This order builds into one target directory kept across runs, outside the clone, and the first run
# linked native/rust/target to it: every binary's real path was then outside the clone, and most of the suite
# went red on "cannot locate the Intent install this binary belongs to" (the gate's "no usable INTENT_HOME").
# So intent and intentd are copied to native/rust/target/release after the build, where they would have been
# built, and where PATH and INTENT_BIN's default look for them.
set -uo pipefail

want="${1:?usage: preflight-linux.sh <sha> <host>:<path> -- the commit to test, and the checkout to clone it from}"
SRC="${2:?usage: preflight-linux.sh <sha> <host>:<path> -- the checkout to clone, on the machine that reads the summary}"
ROOT="${PREFLIGHT_ROOT:-$HOME/work/intent-preflight}"

die() { printf 'preflight: %s\n' "$*" >&2; exit 2; }

# Every tool the leg needs, named when missing rather than failing somewhere inside the suite.
missing=""
for t in git cargo bats jq prettier shellcheck mix rustfmt scp; do
  command -v "$t" >/dev/null 2>&1 || missing="$missing $t"
done
[ -z "$missing" ] || die "missing on this machine:$missing -- install them, then run again (CI's leg has each)"
for sub in clippy fmt; do
  cargo "$sub" --version >/dev/null 2>&1 || die "cargo $sub is missing on this machine -- install it, then run again (rust.yml's leg has it)"
done

run="$ROOT/$want"
rm -rf "$run" || die "cannot clear $run"
mkdir -p "$run" || die "cannot make $run"
log="$run/run.log"
exec 3>&1
{
  printf '# preflight-linux %s -- %s\n' "$want" "$(date -u +'%Y-%m-%d %H:%M:%SZ')"
  printf '# host: %s %s\n' "$(uname -s)" "$(uname -m)"
  printf '# bats %s; cargo %s\n' "$(bats --version)" "$(cargo --version)"
} >"$log"

git clone -q "$SRC" "$run/Intent" >>"$log" 2>&1 || die "clone of $SRC failed -- see $log"
cd "$run/Intent" || die "no clone at $run/Intent"
git checkout -q --detach "$want" >>"$log" 2>&1 || die "$want is not in $SRC's history -- commit it there first"
sha="$(git rev-parse HEAD)"
printf '# commit: %s\n' "$sha" >>"$log"
# shellcheck source-path=SCRIPTDIR source=cmd/shared/preflight.lib
. bin/.devbin/cmd/shared/preflight.lib 2>/dev/null ||
  die "$sha carries no bin/.devbin/cmd/shared/preflight.lib, so it predates the preflight gate -- run the order at a commit that carries the gate"

mkdir -p tests/lib
for lib in bats-support bats-assert bats-file; do
  [ -d "tests/lib/$lib" ] || git clone -q "https://github.com/bats-core/$lib.git" "tests/lib/$lib" >>"$log" 2>&1 ||
    die "could not clone $lib -- see $log"
done
chmod +x bin/* 2>/dev/null
chmod +x tests/*.sh 2>/dev/null

gitconfig="$run/gitconfig"
printf '[user]\n\temail = ci@intent.local\n\tname = Intent CI\n' >"$gitconfig"

# ONE target directory across runs, as CI's rust-cache keeps one: the build is redirected there with
# CARGO_TARGET_DIR, outside this clone and never this repository's shared native/rust/target, and the two
# binaries are copied from it into the clone (see the header for why a link will not do). Runs are sequential,
# so it is never shared by two builds at once. One job by default: kovacs has two CPUs and little memory
# (ORDER 3's setting).
mkdir -p "$ROOT/target"

# The runner's locale: LANG=C.UTF-8 and LC_ALL unset. Every other LC_* this login exports is unset as well,
# since any one of them would override LANG for its own category.
nolc=(-u LC_ALL)
while IFS= read -r v; do
  case "$v" in LC_ALL) ;; LC_*) nolc+=(-u "$v") ;; esac
done < <(compgen -e)

# finish <rc> <head-file> -- writes the summary (the head file's lines, then the log's path), prints it, copies
# it back, and exits <rc>. Every way the order ends after the clone goes through here, red or green.
finish() {
  local rc=$1 summary="$ROOT/$sha.summary" host
  { cat "$2"; printf 'log: %s\n' "$log"; } >"$summary"
  cat "$summary" >&3
  # THE SUMMARY GOES BACK to the machine the checkout is on, red or green, where `int prepush` reads it before
  # a push to upstream. A copy that fails is said, with the command that makes it, and never passes as clean.
  host="${SRC%%:*}"
  if ! { ssh "$host" mkdir -p "$PREFLIGHT_SUMMARY_REL" && scp -q "$summary" "$host:$PREFLIGHT_SUMMARY_REL/$sha.summary"; }; then
    printf 'preflight: the summary did not reach %s; copy it with:\n  ssh %s mkdir -p %s && scp %s %s:%s/%s.summary\n' \
      "$host" "$host" "$PREFLIGHT_SUMMARY_REL" "$summary" "$host" "$PREFLIGHT_SUMMARY_REL" "$sha" >&2
    exit 3
  fi
  printf 'preflight: summary copied to %s:%s/%s.summary\n' "$host" "$PREFLIGHT_SUMMARY_REL" "$sha" >&3
  exit "$rc"
}

# rust_step <name> <command...> -- one of .github/workflows/rust.yml's `run:` lines, as its Linux leg runs it:
# in native/rust, XDG_* empty (the job's env), the runner's locale, SIGPIPE ignored, into the shared target. Its
# output goes to its own log and then the run's. A failure ends the order there: the summary's first line names
# the step and its rc, its second the steps that ran, and the step's error lines follow, so a lint that fails
# here fails before a push. A green summary's second line names every step too, so a green this order made
# reads apart from one made when it ran only the bats suite; preflight_state reads the first line alone.
steps=""
rust_step() {
  local name=$1 rc steplog="$run/rust-$1.log"
  shift
  steps="${steps:+$steps }$name"
  printf 'preflight: rust.yml %s\n' "$name" >&3
  (cd native/rust && env "${nolc[@]}" LANG=C.UTF-8 XDG_CONFIG_HOME= XDG_DATA_HOME= XDG_STATE_HOME= XDG_RUNTIME_DIR= \
    CARGO_TARGET_DIR="$ROOT/target" CARGO_BUILD_JOBS="${PREFLIGHT_JOBS:-1}" \
    bash -c "trap '' PIPE; exec \"\$@\"" rust-step "$@") >"$steplog" 2>&1
  rc=$?
  { printf '\n# ---- rust.yml %s: %s, rc=%s ----\n' "$name" "$*" "$rc"; cat "$steplog"; } >>"$log"
  [ "$rc" -eq 0 ] && return 0
  {
    preflight_step_line "$sha" "rust.yml $name" "$rc"
    printf 'steps: %s\n' "$steps"
    grep -E '^(error|Diff in| +--> |test .* FAILED$)' "$steplog"
  } >"$run/summary.head"
  finish "$rc" "$run/summary.head"
}

# rust.yml's Linux leg, in its order, each line as that file writes it.
rust_step fmt cargo fmt --check
rust_step clippy cargo clippy --workspace --all-targets -- -D warnings
rust_step clippy-lib cargo clippy -p intentsvcs -p intent-cli --lib -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic
rust_step doc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
rust_step test cargo test --workspace --no-fail-fast

printf '\n# ---- build (cargo build --release -p intent-cli -p intentd), jobs %s ----\n' "${PREFLIGHT_JOBS:-1}" >>"$log"
printf 'preflight: building %s (jobs %s)\n' "$sha" "${PREFLIGHT_JOBS:-1}" >&3
CARGO_TARGET_DIR="$ROOT/target" CARGO_BUILD_JOBS="${PREFLIGHT_JOBS:-1}" \
  cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd >>"$log" 2>&1 ||
  die "the build failed at $sha -- see $log"
mkdir -p native/rust/target/release || die "cannot make native/rust/target/release in the clone"
cp -p "$ROOT/target/release/intent" "$ROOT/target/release/intentd" native/rust/target/release/ ||
  die "cannot copy intent and intentd from $ROOT/target/release into the clone"
printf 'preflight: running the bats suite, SIGPIPE ignored, LANG=C.UTF-8\n' >&3
printf '\n# ---- run_tests.sh, SIGPIPE ignored and LANG=C.UTF-8 as on the runner (env %s) ----\n' "${nolc[*]}" >>"$log"
env "${nolc[@]}" LANG=C.UTF-8 XDG_CONFIG_HOME= XDG_DATA_HOME= XDG_STATE_HOME= XDG_RUNTIME_DIR= \
  GIT_CONFIG_GLOBAL="$gitconfig" \
  PATH="$PWD/bin:$PWD/native/rust/target/release:$PATH" \
  BATS_LIB_PATH="$PWD/tests/lib" \
  bash -c "trap '' PIPE; cd tests && ./run_tests.sh" >>"$log" 2>&1
rc=$?

oks="$(grep -c '^ok ' "$log")"
reds="$(grep -c '^not ok ' "$log")"
{
  preflight_summary_line "$sha" "$rc" "$oks" "$reds"
  printf 'steps: %s run_tests.sh\n' "$steps"
  grep '^not ok ' "$log"
} >"$run/summary.head"
finish "$rc" "$run/summary.head"
