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
# EVERY BINARY IS BUILT INSIDE THE CLONE, as CI builds in-tree. `intent` finds the install it belongs to by
# walking up from its own REAL path to a directory holding lib/templates/, and the test binaries cargo test runs
# do the same, so that path must sit inside the clone. One target directory is kept across runs, as CI's
# rust-cache keeps one, at $ROOT/target: after the clone it is MOVED into the clone's native/rust/target, and an
# EXIT trap moves it back on every way the order ends (finish, die, a signal). Both are renames, since the two
# paths are on one filesystem. It is never a redirect or a link. A link to it put every binary's real path
# outside the clone and most of the bats suite went red on "cannot locate the Intent install this binary belongs
# to"; a CARGO_TARGET_DIR redirect did the same to cargo test's binaries (issue 0637, the kovacs run at
# e39e60104). The workspace crates rebuild on each run because their path moved, which also keeps testkit's
# CARGO_MANIFEST_DIR naming this clone; the registry dependencies stay built.
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

# THE LEG'S PATH HOLDS NO INSTALLED intent, because CI's runners have none. A test that runs `intent` from the
# ambient PATH where it should run the build's own is green on any machine with Intent installed and red on
# the runner, as the 3.2.6 tag's macOS test leg was (issue 0637). kovacs has brew's intent in the directory
# its brew rust links into, and stays brew-only, so no directory of the login's PATH can go on the leg's. The
# leg's PATH is instead a directory of links, made for this run, to exactly the tools the leg runs, each
# resolved from the login's PATH, then the system directories: the rust steps run under it as rust.yml's job
# does, and the build and the bats suite put the clone's bin/ and native/rust/target/release in front of it, as
# tests.yml does. cargo finds rustc, rustdoc and its fmt and clippy subcommands on PATH, so they are linked
# too; the rest are what tests.yml installs (bats, jq, prettier with node, the BEAM's erl, elixir and mix) and
# what the runner's image ships (git, shellcheck). An intent the leg's PATH still finds can only be in a system
# directory, and is refused here, naming where, before anything is cloned or built.
legbin="$run/leg-bin"
mkdir -p "$legbin" || die "cannot make $legbin"
for t in cargo rustc rustdoc rustfmt cargo-fmt cargo-clippy clippy-driver git bats jq prettier node erl escript elixir mix shellcheck; do
  p="$(command -v "$t" 2>/dev/null)" || continue
  ln -s "$p" "$legbin/$t" || die "cannot link $t into $legbin"
done
leg_path="$legbin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
if stray="$(PATH="$leg_path" && command -v intent)"; then
  die "an installed intent is in a system directory, at $stray -- CI's runners have none there, so a test that finds it passes here and fails there; remove it from that directory, then run again"
fi
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

# ONE target directory across runs, MOVED into the clone and back, never a redirect or a link (see the header).
# The trap is set before the move and returns the target once the move-in has begun; a move that failed left
# nothing in the clone to return. It moves the target back only when nothing is at $ROOT/target already, since
# mv would otherwise move it inside that directory; a target it cannot return is said, and the next run starts
# cold. Runs are sequential, so it is never shared by two builds at once. One job by default: kovacs has two
# CPUs and little memory (ORDER 3's setting).
intree="$run/Intent/native/rust/target"
moved_in=""
# shellcheck disable=SC2329 # invoked by the EXIT trap below
return_target() {
  [ -n "$moved_in" ] && [ -d "$intree" ] || return 0
  if [ -e "$ROOT/target" ]; then
    printf 'preflight: %s exists, so this run left its target at %s -- the next run starts cold\n' "$ROOT/target" "$intree" >&2
    return 0
  fi
  mv "$intree" "$ROOT/target" || printf 'preflight: could not move %s back to %s\n' "$intree" "$ROOT/target" >&2
}
trap return_target EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM
[ ! -e "$intree" ] || die "the fresh clone already holds native/rust/target, so the kept target cannot move in"
moved_in=1
if [ -d "$ROOT/target" ]; then
  mv "$ROOT/target" "$intree" || die "cannot move $ROOT/target into the clone at $intree"
else
  mkdir -p "$intree" || die "cannot make $intree"
fi

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
# in native/rust, XDG_* empty (the job's env), the runner's locale, SIGPIPE ignored, into the clone's own native/rust/target. Its
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
    PATH="$leg_path" CARGO_BUILD_JOBS="${PREFLIGHT_JOBS:-1}" \
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

# THE POSITIVE CONTROL on where cargo builds: nothing may send it outside the clone. A target-dir variable in the
# environment would redirect every step, and a link at native/rust/target would put every binary's real path
# outside the clone; either is refused here, named, before the first step.
for v in CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR; do
  [ -z "${!v+x}" ] || die "$v is set in the environment (to ${!v}) -- it would build outside the clone; unset it, then run again"
done
[ ! -L "$intree" ] || die "$intree is a symlink (to $(readlink "$intree")) -- every binary's real path would be outside the clone"

# rust.yml's Linux leg, in its order, each line as that file writes it.
rust_step fmt cargo fmt --check
rust_step clippy cargo clippy --workspace --all-targets -- -D warnings
rust_step clippy-lib cargo clippy -p intentsvcs -p intent-cli --lib -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic
rust_step doc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
rust_step test cargo test --workspace --no-fail-fast

printf '\n# ---- build (cargo build --release -p intent-cli -p intentd), jobs %s ----\n' "${PREFLIGHT_JOBS:-1}" >>"$log"
printf 'preflight: building %s (jobs %s)\n' "$sha" "${PREFLIGHT_JOBS:-1}" >&3
PATH="$PWD/bin:$PWD/native/rust/target/release:$leg_path" CARGO_BUILD_JOBS="${PREFLIGHT_JOBS:-1}" \
  cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd >>"$log" 2>&1 ||
  die "the build failed at $sha -- see $log"
printf 'preflight: running the bats suite, SIGPIPE ignored, LANG=C.UTF-8\n' >&3
printf '\n# ---- run_tests.sh, SIGPIPE ignored and LANG=C.UTF-8 as on the runner (env %s) ----\n' "${nolc[*]}" >>"$log"
env "${nolc[@]}" LANG=C.UTF-8 XDG_CONFIG_HOME= XDG_DATA_HOME= XDG_STATE_HOME= XDG_RUNTIME_DIR= \
  GIT_CONFIG_GLOBAL="$gitconfig" \
  PATH="$PWD/bin:$PWD/native/rust/target/release:$leg_path" \
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
