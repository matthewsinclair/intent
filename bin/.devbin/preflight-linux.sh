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
#   - XDG_CONFIG_HOME, XDG_DATA_HOME, XDG_STATE_HOME and XDG_RUNTIME_DIR set empty;
#   - bats-support, bats-assert and bats-file cloned into tests/lib;
#   - bin/ and native/rust/target/release first on PATH; a CI git identity;
#   - `cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd`;
#   - `cd tests && ./run_tests.sh` with BATS_LIB_PATH set;
#   - AND SIGPIPE IGNORED, as GitHub's runners start every step. An interactive shell has it at default, and
#     that difference hid issue 0628's class: under it, `comm | head -1` dies quietly; ignored, it fails.
# The git identity goes through GIT_CONFIG_GLOBAL, a file in the run's own directory, so the user's ~/.gitconfig
# is never touched. Everything lands in ~/work/intent-preflight/<sha>/, and the summary names every red arm. Its
# first line and where it is copied come from bin/.devbin/cmd/shared/preflight.lib in the clone, the file
# `int prepush` reads them from.
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
# CARGO_TARGET_DIR, outside this clone and never this repository's shared native/rust/target, and it is linked
# where the suite looks for the binaries (native/rust/target/release is on PATH and is INTENT_BIN's default).
# Runs are sequential, so it is never shared by two builds at once. One job by default: kovacs has two CPUs and
# little memory (ORDER 3's setting).
mkdir -p "$ROOT/target"
ln -s "$ROOT/target" native/rust/target || die "cannot link native/rust/target to $ROOT/target"
printf '\n# ---- build (cargo build --release -p intent-cli -p intentd), jobs %s ----\n' "${PREFLIGHT_JOBS:-1}" >>"$log"
printf 'preflight: building %s (jobs %s)\n' "$sha" "${PREFLIGHT_JOBS:-1}" >&3
CARGO_TARGET_DIR="$ROOT/target" CARGO_BUILD_JOBS="${PREFLIGHT_JOBS:-1}" \
  cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd >>"$log" 2>&1 ||
  die "the build failed at $sha -- see $log"
printf 'preflight: running the bats suite, SIGPIPE ignored\n' >&3

printf '\n# ---- run_tests.sh, SIGPIPE ignored as on the runner ----\n' >>"$log"
env XDG_CONFIG_HOME= XDG_DATA_HOME= XDG_STATE_HOME= XDG_RUNTIME_DIR= \
  GIT_CONFIG_GLOBAL="$gitconfig" \
  PATH="$PWD/bin:$PWD/native/rust/target/release:$PATH" \
  BATS_LIB_PATH="$PWD/tests/lib" \
  bash -c "trap '' PIPE; cd tests && ./run_tests.sh" >>"$log" 2>&1
rc=$?

oks="$(grep -c '^ok ' "$log")"
reds="$(grep -c '^not ok ' "$log")"
summary="$ROOT/$sha.summary"
{
  preflight_summary_line "$sha" "$rc" "$oks" "$reds"
  grep '^not ok ' "$log"
  printf 'log: %s\n' "$log"
} >"$summary"
cat "$summary" >&3

# THE SUMMARY GOES BACK to the machine the checkout is on, red or green, where `int prepush` reads it before a
# push to upstream. A copy that fails is said, with the command that makes it, and never passes as a clean run.
host="${SRC%%:*}"
if ! { ssh "$host" mkdir -p "$PREFLIGHT_SUMMARY_REL" && scp -q "$summary" "$host:$PREFLIGHT_SUMMARY_REL/$sha.summary"; }; then
  printf 'preflight: the summary did not reach %s; copy it with:\n  ssh %s mkdir -p %s && scp %s %s:%s/%s.summary\n' \
    "$host" "$host" "$PREFLIGHT_SUMMARY_REL" "$summary" "$host" "$PREFLIGHT_SUMMARY_REL" "$sha" >&2
  exit 3
fi
printf 'preflight: summary copied to %s:%s/%s.summary\n' "$host" "$PREFLIGHT_SUMMARY_REL" "$sha" >&3
exit "$rc"
