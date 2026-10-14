#!/usr/bin/env bats
# `bin/int linux doctor` on a host with no Rust (issue 0595).
#
# doctor exists for one case above all: a fresh Linux machine that cannot build
# yet. On exactly that machine it exited 127 and printed nothing on either
# stream. host_triple's `rustc -vV 2>/dev/null | sed` failed under pipefail and
# helpers' inherit_errexit before its die could speak, and doctor asked
# host_triple for a triple before it listed a single lack.
#
# THE ARM'S PATH IS BUILT, NOT INHERITED. It holds the tools the dispatcher runs
# before the handler, each linked from this host's own PATH, bash among them
# (the dispatcher's floor is bash 5), and a `uname` that answers Linux x86_64.
# It holds no rustc, cargo, rustup or musl-gcc, whatever this host has
# installed. So the arm runs the same on the Mac, on Linux CI and on kovacs,
# where hv's zshrc puts ~/.cargo/bin on every interactive PATH.

load "../lib/test_helper.bash"

# What the dispatcher runs before the handler reaches doctor, measured on
# 2026-10-14 by adding each tool a bare PATH reported missing until doctor ran.
# cat, grep and head are spares, in case another host's dispatcher path differs.
DISPATCH_TOOLS="bash dirname readlink awk date tr sed rm cat grep head"

# no_rust_path -- sets NO_RUST_PATH to a directory holding DISPATCH_TOOLS and a
# uname that says Linux x86_64, and nothing else, for use as the whole PATH.
no_rust_path() {
  local t p
  NO_RUST_PATH="$TEST_TEMP_DIR/no-rust-bin"
  mkdir -p "$NO_RUST_PATH"
  for t in $DISPATCH_TOOLS; do
    p="$(command -v "$t")" || fail "the arm's PATH needs $t, and this host has none"
    ln -s "$p" "$NO_RUST_PATH/$t"
  done
  cat >"$NO_RUST_PATH/uname" <<'EOF'
#!/bin/sh
case "$1" in
  -s) echo Linux ;;
  -m) echo x86_64 ;;
  *) echo Linux ;;
esac
EOF
  chmod +x "$NO_RUST_PATH/uname"
}

@test "invariant: linux doctor with no rustc on PATH lists every lack and refuses, where it exited 127 in silence (0595)" {
  no_rust_path
  # From the project root, because the dispatcher refuses a cwd outside the
  # project it belongs to, and setup() stands in a temp dir.
  cd "$INTENT_PROJECT_ROOT"
  run env PATH="$NO_RUST_PATH" bin/int linux doctor
  [ "$status" -eq 1 ] || fail "doctor exited $status, and 0595 was 127 with no output: [$output]"
  assert_output_contains "missing: rustc is not on PATH"
  assert_output_contains "missing: rustup is not on PATH"
  assert_output_contains "missing: musl-gcc is not on PATH"
  assert_output_contains "this host cannot build x86_64-unknown-linux-musl yet"
}
