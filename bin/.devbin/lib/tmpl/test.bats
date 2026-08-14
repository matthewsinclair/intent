#!/usr/bin/env bats
#
# {{NAME}} -- scaffolded by `{{DEVBIN}} new {{NAME}}`.
#
# The first test asserts the command RESOLVES, which is the thing a scaffold can
# know is true. The rest is yours.
#
# Test names are EVALUATED by bats, so no backticks and no $(...) in them.

setup() {
  # Walk UP to the project root rather than counting `..`. This stub is written
  # into whatever test directory the project already has -- `tests/` in most
  # projects, `tests/unit/` in devbin's own -- and a fixed depth is correct in
  # exactly one of those and silently wrong in the rest. It was `../..`, so a
  # scaffold in a `tests/`-at-root project failed both of these tests out of the
  # box, about nothing.
  if [ -z "${DEVBIN_HOME:-}" ]; then
    DEVBIN_HOME="$BATS_TEST_DIRNAME"
    while [ "$DEVBIN_HOME" != / ] && [ ! -x "$DEVBIN_HOME/bin/devbin" ]; do
      DEVBIN_HOME="$(cd "$DEVBIN_HOME/.." && pwd)"
    done
  fi
  # The project-named launcher when it exists, the dispatcher otherwise. During
  # a migration the project name is still the old launcher and devbin declines
  # to take it (design D10), so bin/devbin is the name always present.
  DEVBIN="$DEVBIN_HOME/bin/{{DEVBIN}}"
  [ -x "$DEVBIN" ] || DEVBIN="$DEVBIN_HOME/bin/devbin"
}

@test "{{NAME}} resolves and is offered in help" {
  run "$DEVBIN" help
  [ "$status" -eq 0 ]
  [[ "$output" == *"{{NAME}}"* ]]
}

@test "{{NAME}} answers --help" {
  run "$DEVBIN" {{NAME}} --help
  [ "$status" -eq 0 ]
}
