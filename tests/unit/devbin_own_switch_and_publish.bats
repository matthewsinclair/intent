#!/usr/bin/env bats
# Intent keeps its own switch and its own publish (devbin 0332): this
# checkout's config declares `install.switch: intent use` and
# `install.publish: intent install`, so devbin's generic `bin/devbin use` and
# `bin/devbin install` refuse here, each naming Intent's own command, and write
# nothing.
#
# Each arm runs this checkout's own bin/devbin with HOME, the XDG roots and the
# install prefix all inside the test's sandbox, so a refusal that failed could
# only write there. The sandbox is listed before and after, and must not change.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-own-XXXXXX)"
  setup_fake_home
  [ "$HOME" != "$REAL_HOME" ] && [[ "$HOME" == "$TEST_TEMP_DIR"/* ]] || {
    echo "HOME is not the test's sandbox: $HOME" >&2
    return 1
  }
  unset DEVBIN_INSTALL_PREFIX
  export XDG_DATA_HOME="${HOME}/.local/share"
  export XDG_STATE_HOME="${HOME}/.local/state"
  export XDG_CONFIG_HOME="${HOME}/.config"
  export XDG_CACHE_HOME="${HOME}/.cache"
  BEFORE="$(cd "$HOME" && find . | sort)"
}

teardown() {
  teardown_fake_home
  rm -rf "${TEST_TEMP_DIR}"
}

# sandbox_unchanged -- the sandbox HOME holds exactly what it held at setup.
sandbox_unchanged() {
  local after
  after="$(cd "$HOME" && find . | sort)"
  [ "$after" = "$BEFORE" ] || {
    printf 'the sandbox HOME changed:\n%s\n' "$(diff <(printf '%s\n' "$BEFORE") <(printf '%s\n' "$after"))" >&2
    return 1
  }
}

@test "bin/devbin use refuses, naming intent use, and points no link" {
  cd "$INTENT_PROJECT_ROOT"
  run bin/devbin use dev
  [ "$status" -ne 0 ]
  [[ "$output" == *"keeps its own switch, install.switch: run intent use dev"* ]]
  [[ "$output" == *"use points no link of it"* ]]
  sandbox_unchanged
}

@test "bin/devbin install refuses, naming intent install, and publishes nothing" {
  cd "$INTENT_PROJECT_ROOT"
  run bin/devbin install
  [ "$status" -ne 0 ]
  [[ "$output" == *"keeps its own publish, install.publish: run intent install"* ]]
  [[ "$output" == *"install publishes no opt tree of it"* ]]
  sandbox_unchanged
}
