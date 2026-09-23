#!/usr/bin/env bats
#
# The shim locates the real gate, or REFUSES and says which failure it hit.
#
# **EVERY ARM OVERRIDES `HOME`.** The shim reads `$XDG_DATA_HOME/intent/home`, by default under `$HOME`, which on
# this machine is a real operator's real pointer. An arm that forgot the
# override would read it, and a WRITING arm would have rewritten it.

load "../lib/test_helper.bash"

SHIM="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit-shim.sh"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-shim-test-XXXXXX)"
  FAKE_HOME="${TEST_TEMP_DIR}/home"
  mkdir -p "${FAKE_HOME}/.local/share/intent"
}

teardown() {
  [ -d "${TEST_TEMP_DIR}" ] && rm -rf "${TEST_TEMP_DIR}"
}

# An install root: `lib/templates` is the marker `install.rs` itself uses, so
# the fixture and the binary agree on what an install IS by construction.
make_install() {
  local root="$1"
  mkdir -p "${root}/lib/templates/hooks"
  cat > "${root}/lib/templates/hooks/pre-commit.sh" <<GATE
#!/usr/bin/env bash
echo "REAL GATE RAN: \$*"
exit ${2:-0}
GATE
  chmod +x "${root}/lib/templates/hooks/pre-commit.sh"
}

@test "pointer ABSENT: refuses, and blames the installer rather than Intent" {
  HOME="${FAKE_HOME}" run bash "$SHIM"
  assert_failure
  assert_output_contains "cannot locate the Intent install"
  assert_output_contains "is absent"
  # **THE CLAIM IS THE DIRECTION OF BLAME, NOT THE SENTENCE AROUND IT.** This
  # asserted "never completed, not that Intent is missing" until 2026-09-12,
  # and batch 1 rewrote the clause before it so the remedy names `intent
  # bootstrap` -- the shim now says the machine has not run it. The test went
  # red on wording it was never about, so it now pins the half that carries the
  # claim this test is named for.
  assert_output_contains "not that Intent is missing"
  assert_output_contains "refusing rather than skipping"
}

@test "pointer EMPTY: refuses, and says empty rather than absent" {
  : > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM"
  assert_failure
  assert_output_contains "is empty"
  refute_output_contains "is absent"
}

@test "pointer resolves somewhere that is NOT an install: refuses and QUOTES the path" {
  # **THE ARM THE WHOLE DESIGN IS FOR.** The afternoon that produced this file
  # was a pointer resolving to a plausible WRONG tree, and a message that says
  # "cannot find the install" without saying where it looked sends the reader
  # to reinstall when the fault is one stale line in a file.
  mkdir -p "${TEST_TEMP_DIR}/not-an-install"
  echo "${TEST_TEMP_DIR}/not-an-install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM"
  assert_failure
  assert_output_contains "is not an install"
  assert_output_contains "${TEST_TEMP_DIR}/not-an-install"
  assert_output_contains "NOT repairing it here"
}

@test "a stale pointer is NOT auto-repaired -- the file is left exactly as found" {
  # A self-healing pointer would hide the incomplete install the operator needs
  # to know about, and would mean this shim choosing an install root on their
  # behalf. Asserted on BYTES, because "it still refuses" would pass even if
  # the shim had rewritten the file to something else broken.
  echo "/nowhere/at/all" > "${FAKE_HOME}/.local/share/intent/home"
  local before; before="$(cat "${FAKE_HOME}/.local/share/intent/home")"
  HOME="${FAKE_HOME}" run bash "$SHIM"
  assert_failure
  [ "$(cat "${FAKE_HOME}/.local/share/intent/home")" = "$before" ]
}

@test "root IS an install but the gate is missing: a DIFFERENT refusal, naming the file" {
  # Distinct from the wrong-pointer arm on purpose: the root is an install, so
  # "your pointer is wrong" would be false and would send someone at the wrong
  # repair.
  mkdir -p "${TEST_TEMP_DIR}/install/lib/templates/hooks"
  echo "${TEST_TEMP_DIR}/install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM"
  assert_failure
  assert_output_contains "has no pre-commit gate"
  assert_output_contains "pre-commit.sh"
  refute_output_contains "is not an install"
}

@test "happy path: the real gate runs and receives the arguments" {
  make_install "${TEST_TEMP_DIR}/install"
  echo "${TEST_TEMP_DIR}/install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM" --some-arg
  assert_success
  assert_output_contains "REAL GATE RAN: --some-arg"
}

@test "the gate's exit status IS the shim's -- exec, not a forwarded \$?" {
  # A wrapper forwarding `$?` by hand is one `set -e` interaction away from
  # turning a refusal into a pass. 42 rather than 1, so a coincidental failure
  # cannot be mistaken for a passthrough.
  make_install "${TEST_TEMP_DIR}/install" 42
  echo "${TEST_TEMP_DIR}/install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM"
  [ "$status" -eq 42 ]
}

@test "\$INTENT_HOME in the environment is IGNORED, even when it names a real install" {
  # **THE PROPERTY MOST LIKELY TO BE 'FIXED' BACK IN BY A LATER READER.**
  # `install.rs` refuses the environment deliberately: a stale INTENT_HOME in
  # somebody's shell would make a v3 binary exec v2's scripts. A shim that read
  # it would hand back exactly what the binary refuses, one layer down.
  #
  # The env var here points at a PERFECTLY GOOD install, so nothing but the
  # rule itself makes this refuse.
  make_install "${TEST_TEMP_DIR}/install"
  HOME="${FAKE_HOME}" INTENT_HOME="${TEST_TEMP_DIR}/install" run bash "$SHIM"
  assert_failure
  assert_output_contains "cannot locate the Intent install"
}

@test "a pointer through Homebrew's opt link finds the gate after an upgrade replaces the keg (issue 0527)" {
  # From 3.2.1 bootstrap records <prefix>/opt/intent/libexec rather than the
  # versioned keg, which `brew upgrade` deletes. The upgrade is driven as brew
  # performs it: a new keg, the opt link moved to it, the old keg cleaned up.
  local prefix="${TEST_TEMP_DIR}/brew"
  make_install "${prefix}/Cellar/intent/9.9.9/libexec"
  mkdir -p "${prefix}/opt"
  ln -s ../Cellar/intent/9.9.9 "${prefix}/opt/intent"
  echo "${prefix}/opt/intent/libexec" > "${FAKE_HOME}/.local/share/intent/home"
  make_install "${prefix}/Cellar/intent/9.9.10/libexec"
  rm "${prefix}/opt/intent"
  ln -s ../Cellar/intent/9.9.10 "${prefix}/opt/intent"
  rm -rf "${prefix}/Cellar/intent/9.9.9"
  HOME="${FAKE_HOME}" run bash "$SHIM" --after-upgrade
  assert_success
  assert_output_contains "REAL GATE RAN: --after-upgrade"
}

@test "--where reports what it resolved, and runs no gate" {
  make_install "${TEST_TEMP_DIR}/install"
  echo "${TEST_TEMP_DIR}/install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM" --where
  assert_success
  assert_output_contains "state:    OK"
  assert_output_contains "${TEST_TEMP_DIR}/install"
  refute_output_contains "REAL GATE RAN"
}

@test "--where on a broken pointer reports UNUSABLE and exits non-zero" {
  mkdir -p "${TEST_TEMP_DIR}/not-an-install"
  echo "${TEST_TEMP_DIR}/not-an-install" > "${FAKE_HOME}/.local/share/intent/home"
  HOME="${FAKE_HOME}" run bash "$SHIM" --where
  assert_failure
  assert_output_contains "UNUSABLE"
}
