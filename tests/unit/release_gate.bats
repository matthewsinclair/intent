#!/usr/bin/env bats
# The cut's gates on the release commit, bin/.devbin/cut/gate.
#
# ST0091 AT-01.5 cites this file for AC-01.2.
#
# devbin's release core runs `release.gates` on each leg after its commit step,
# so each check runs here as the core runs it: in a scratch repository with
# PROJECT_ROOT pointed at it, with stubs standing in for the two ST0056 checks,
# for cargo and the devbin twins, and for the release pair, where the subject is
# what the gate does with their answers. The cargo steps themselves, the pair
# build and the sealed green are the core's and cargo's own work, and the
# judge's run of the cut drives them.

load "../lib/test_helper.bash"

GATE="${INTENT_PROJECT_ROOT}/bin/.devbin/cut/gate"
CONFIG="${INTENT_PROJECT_ROOT}/bin/.devbin/config.yaml"

# make_repo -- $REPO committed, holding the two ST0056 checks as stubs that
# exit STUB_CONTRACT_RC and STUB_REFERENCE_RC, a native tree, and a `bin/devbin`
# whose `check <twin>` fails for STUB_TWIN_RED alone. The twin is committed
# because the rust gate runs it in a worktree it adds at HEAD. And the stub
# cargo, first on PATH for `check`.
#
# It unsets DEVBIN_GATE_CACHE, because devbin's release legs hand the gates their
# real kept cache in that variable, and this suite runs as one of those gates.
# Inherited, it makes the stubbed gate rename the leg's own kept target into a
# test worktree, where it is lost with the test. A test about the cache exports
# its own after this.
make_repo() {
  unset DEVBIN_GATE_CACHE
  REPO="${TEST_TEMP_DIR}/repo"
  local tools="$REPO/intent/st/ST0056/parity/tools"
  mkdir -p "$tools" "$REPO/native/rust" "$REPO/bin"
  git -C "$REPO" init -q
  git -C "$REPO" config user.name t
  git -C "$REPO" config user.email t@t
  printf '#!/usr/bin/env bash\necho contract findings\nexit "${STUB_CONTRACT_RC:-0}"\n' >"$tools/contract_check.sh"
  printf '#!/usr/bin/env bash\necho reference diff\nexit "${STUB_REFERENCE_RC:-0}"\n' >"$tools/reference_current_check.sh"
  printf '[workspace]\n' >"$REPO/native/rust/Cargo.toml"
  cat >"$REPO/bin/devbin" <<'EOF'
#!/usr/bin/env bash
printf 'twin %s\n' "$2" >>"$STUB_LOG"
[ "$2" != "${STUB_TWIN_RED:-}" ]
EOF
  chmod +x "$REPO/bin/devbin"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m base

  # cargo, as the rust gate runs it. `build` exits STUB_BUILD_RC. `test
  # --workspace` names each of STUB_RED as a failed arm under a target, as
  # cargo prints them, and fails if there is one. `test -p intentd` is the
  # re-run: it is logged, and exits STUB_RERUN_RC.
  STUBS="${TEST_TEMP_DIR}/stubs"
  STUB_LOG="${TEST_TEMP_DIR}/stub.log"
  : >"$STUB_LOG"
  mkdir -p "$STUBS"
  cat >"$STUBS/cargo" <<'EOF'
#!/usr/bin/env bash
case "$1 $2" in
  "build -p") exit "${STUB_BUILD_RC:-0}" ;;
  "test --workspace")
    [ -n "${STUB_RED:-}" ] || exit 0
    echo "     Running tests/suite.rs (target/debug/deps/suite-0)"
    for t in $STUB_RED; do echo "test $t ... FAILED"; done
    exit 1 ;;
  "test -p")
    echo rerun >>"$STUB_LOG"
    exit "${STUB_RERUN_RC:-0}" ;;
  *) echo "stub cargo: unexpected arguments: $*" >&2; exit 2 ;;
esac
EOF
  chmod +x "$STUBS/cargo"
  printf '#!/usr/bin/env bash\necho "rustc 0.0.0 (stub)"\n' >"$STUBS/rustc"
  chmod +x "$STUBS/rustc"
  export STUB_LOG
}

# kept_target_cargo -- a cargo whose `build -p` records where CARGO_TARGET_DIR
# is and what it holds, and then, when STUB_KILL names a signal, sends it to the
# gate itself, as a stopped run does.
kept_target_cargo() {
  cat >"$STUBS/cargo" <<'EOF'
#!/usr/bin/env bash
case "$1 $2" in
  "build -p")
    printf 'target %s\nkept %s\n' "$CARGO_TARGET_DIR" "$(cat "$CARGO_TARGET_DIR/kept-marker" 2>/dev/null)" >>"$STUB_LOG"
    if [ -n "${STUB_KILL:-}" ]; then
      # The gate is the nearest ancestor running cut/gate: bash may exec the
      # subshell's last command, so it is not always the parent's parent.
      p="$PPID"
      until ps -o command= -p "$p" | grep -q 'cut/gate rust'; do
        p="$(ps -o ppid= -p "$p" | tr -d ' ')"
        [ "$p" -gt 1 ] || { echo "stub cargo: no cut/gate ancestor" >&2; exit 2; }
      done
      kill -"$STUB_KILL" "$p"
    fi
    exit 0 ;;
  "test --workspace") exit 0 ;;
  *) echo "stub cargo: unexpected arguments: $*" >&2; exit 2 ;;
esac
EOF
  chmod +x "$STUBS/cargo"
}

# check <name> -- one gate as the core runs it. $output is its stdout alone;
# its stderr is in $ERR.
check() {
  ERR="${TEST_TEMP_DIR}/check.err"
  run env PROJECT_ROOT="$REPO" PATH="$STUBS:$PATH" bash -c '"$1" "$2" 2>"$3"' _ "$GATE" "$1" "$ERR"
}

err_has() {
  grep -qF -- "$1" "$ERR" || fail "expected the gate's stderr to name '$1'; it was: $(cat "$ERR")"
}

# write_pair <commit> -- the release pair, each binary carrying the source-commit
# marker the build embeds, and an intent that answers `doctor` with
# STUB_DOCTOR_RC.
write_pair() {
  local dir="$REPO/native/rust/target/release"
  mkdir -p "$dir"
  printf '#!/usr/bin/env bash\n# [intent-source-commit:%s]\n[ "$1" = doctor ] || exit 2\necho doctor report\nexit "${STUB_DOCTOR_RC:-0}"\n' "$1" >"$dir/intent"
  printf '[intent-source-commit:%s]\n' "$1" >"$dir/intentd"
  chmod +x "$dir/intent"
}

@test "AT-01.5: contract: agreement passes, and a finding and a missing verdict each refuse" {
  make_repo
  check contract
  assert_success
  assert_output_contains "describe the same fields"
  export STUB_CONTRACT_RC=1
  check contract
  assert_failure
  err_has "disagree"
  export STUB_CONTRACT_RC=3
  check contract
  assert_failure
  err_has "no verdict (rc=3)"
}

@test "AT-01.5: reference: current pages pass, and stale pages and a missing verdict each refuse with the remedy" {
  make_repo
  check reference
  assert_success
  export STUB_REFERENCE_RC=1
  check reference
  assert_failure
  err_has "intent/docs/releasing.md"
  export STUB_REFERENCE_RC=2
  check reference
  assert_failure
  err_has "no verdict (rc=2)"
}

@test "AT-01.5: rust: a green run passes after every twin, and leaves no worktree behind" {
  make_repo
  check rust
  assert_success
  for twin in format clippy panics panics-intentd ownership doc; do
    grep -qx "twin $twin" "$STUB_LOG" || fail "the twin '$twin' did not run: $(cat "$STUB_LOG")"
  done
  [ "$(git -C "$REPO" worktree list | wc -l)" -eq 1 ] || fail "the gate left a worktree: $(git -C "$REPO" worktree list)"
}

@test "AT-01.5: rust: a failed build and a failed twin are each refused by the step that failed" {
  make_repo
  export STUB_BUILD_RC=1
  check rust
  assert_failure
  err_has "cargo build -p intentd failed"
  unset STUB_BUILD_RC
  export STUB_TWIN_RED=clippy
  check rust
  assert_failure
  err_has "bin/devbin check clippy failed"
}

# Issue 0682: the binaries find their install by walking up from their own path
# to lib/templates/, so the target must sit inside the worktree. The kept target
# is renamed in for the run and back after it.
@test "AT-01.5: rust: with DEVBIN_GATE_CACHE the kept target is the worktree's own for the run, and is kept again after it" {
  make_repo
  kept_target_cargo
  export DEVBIN_GATE_CACHE="${TEST_TEMP_DIR}/gate-cache"
  mkdir -p "$DEVBIN_GATE_CACHE/rust-target"
  echo "rustc 0.0.0 (stub)" >"$DEVBIN_GATE_CACHE/rust-target/rustc-version"
  echo from-last-run >"$DEVBIN_GATE_CACHE/rust-target/kept-marker"
  check rust
  assert_success
  grep -qE "^target $DEVBIN_GATE_CACHE/intent-gate-tree\.[^/]+/tree/native/rust/target$" "$STUB_LOG" ||
    fail "cargo's target was not the worktree's own under the cache: $(cat "$STUB_LOG")"
  grep -qx "kept from-last-run" "$STUB_LOG" || fail "the run did not build in the kept target: $(cat "$STUB_LOG")"
  [ "$(cat "$DEVBIN_GATE_CACHE/rust-target/kept-marker")" = from-last-run ] || fail "the kept target was not returned"
  [ "$(git -C "$REPO" worktree list | wc -l)" -eq 1 ] || fail "the gate left a worktree: $(git -C "$REPO" worktree list)"
  run bash -c 'ls -d "$1"/intent-gate-tree.* 2>/dev/null' _ "$DEVBIN_GATE_CACHE"
  assert_output ""
}

# The stopped run of 2026-11-12 19:16Z: a gate killed mid-build still returns
# the kept target and removes its worktree, for TERM and for INT.
@test "AT-01.5: rust: a gate stopped by TERM or INT mid-build returns the kept target and leaves no worktree" {
  make_repo
  kept_target_cargo
  export DEVBIN_GATE_CACHE="${TEST_TEMP_DIR}/gate-cache"
  local sig
  for sig in TERM INT; do
    mkdir -p "$DEVBIN_GATE_CACHE/rust-target"
    echo "rustc 0.0.0 (stub)" >"$DEVBIN_GATE_CACHE/rust-target/rustc-version"
    echo "before-$sig" >"$DEVBIN_GATE_CACHE/rust-target/kept-marker"
    export STUB_KILL="$sig"
    # A background job starts with INT ignored, as under the lanes runner, and
    # bash cannot trap a signal ignored at its entry. A ^C at a terminal is not
    # ignored, so the gate is started with INT at its default, as it is there.
    ERR="${TEST_TEMP_DIR}/check.err"
    run env PROJECT_ROOT="$REPO" PATH="$STUBS:$PATH" perl -e '$SIG{INT} = "DEFAULT"; exec @ARGV or die "exec: $!"' \
      bash -c '"$1" "$2" 2>"$3"' _ "$GATE" rust "$ERR"
    assert_failure
    grep -qx "kept before-$sig" "$STUB_LOG" || fail "the $sig run did not reach its build: $(cat "$STUB_LOG")"
    [ "$(cat "$DEVBIN_GATE_CACHE/rust-target/kept-marker" 2>/dev/null)" = "before-$sig" ] ||
      fail "after $sig the kept target was not returned: $(ls -R "$DEVBIN_GATE_CACHE")"
    [ "$(git -C "$REPO" worktree list | wc -l)" -eq 1 ] || fail "after $sig the gate left a worktree: $(git -C "$REPO" worktree list)"
    run bash -c 'ls -d "$1"/intent-gate-tree.* 2>/dev/null' _ "$DEVBIN_GATE_CACHE"
    assert_output ""
  done
}

# Issue 0481: the core re-runs nothing, so the one re-run is the gate's own.
@test "AT-01.5: rust: a red confined to the load-sensitive family re-runs intentd once, and a red outside it does not" {
  make_repo
  export STUB_RED="daemon_watch::a_rename_is_seen" STUB_RERUN_RC=0
  check rust
  assert_success
  [ "$(grep -c '^rerun$' "$STUB_LOG")" -eq 1 ] || fail "expected one re-run: $(cat "$STUB_LOG")"
  export STUB_RERUN_RC=1
  check rust
  assert_failure
  err_has "cargo test --workspace failed"
  : >"$STUB_LOG"
  export STUB_RED="daemon_watch::a_rename_is_seen canon::a_plain_red" STUB_RERUN_RC=0
  check rust
  assert_failure
  run grep -c '^rerun$' "$STUB_LOG"
  assert_output "0"
}

@test "AT-01.5: provenance: a pair naming the release commit passes, another commit and no pair each refuse naming the gate that builds it" {
  make_repo
  check provenance
  assert_failure
  err_has "bin/devbin gate pair"
  write_pair 0123456789abcdef0123456789abcdef01234567
  check provenance
  assert_failure
  err_has "names 0123456789abcdef0123456789abcdef01234567"
  write_pair "$(git -C "$REPO" rev-parse HEAD)"
  check provenance
  assert_success
  assert_output_contains "name the release commit"
}

@test "AT-01.5: doctor: the pair's clean doctor passes, and a finding or no pair refuses" {
  make_repo
  check doctor
  assert_failure
  err_has "bin/devbin build all"
  write_pair "$(git -C "$REPO" rev-parse HEAD)"
  check doctor
  assert_success
  export STUB_DOCTOR_RC=1
  check doctor
  assert_failure
  err_has "intent doctor reported issues"
}

# AC-01.2's own words: the checks run as declared release.gates, and the pair
# and what reads it on macOS alone. Read from config.yaml, never listed here.
@test "AT-01.5: release check names every gate config.yaml declares, and the options that read the pair are held to macOS" {
  local gates line row opt held
  gates="$(awk '
    /^release:/ { r = 1; next }
    r && /^[^ #]/ { r = 0 }
    r && /^  gates:/ { g = 1; next }
    g && /^    - / { sub(/^    - /, ""); print; next }
    g { g = 0 }
  ' "$CONFIG")"
  [ -n "$gates" ] || fail "the release block in $CONFIG declares no gate"
  row="$(cd "$INTENT_PROJECT_ROOT" && bin/devbin release check 2>/dev/null | awk '$1 == "gates"')"
  while IFS= read -r line; do
    [[ "$row" == *"$line"* ]] || fail "release check's gates row does not name '$line': $row"
  done <<<"$gates"
  # The options under commands.gate that declare platforms, each with its list.
  held="$(awk '
    /^  gate:/ { c = 1; next }
    c && /^  [a-z]/ { c = 0 }
    c && /^      [a-z]+:$/ { opt = $1; sub(/:$/, "", opt); next }
    c && /^          - / { sub(/^          - /, ""); print opt " " $0 }
  ' "$CONFIG")"
  for opt in pair provenance doctor; do
    grep -qx "$opt macos" <<<"$held" || fail "gate $opt is not held to macOS: $held"
  done
  ! grep -qvE '^(pair|provenance|doctor) macos$' <<<"$held" || fail "a gate that does not read the pair is held to a platform: $held"
}
