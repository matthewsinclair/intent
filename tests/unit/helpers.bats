#!/usr/bin/env bats
# Tests for shared helpers in bin/intent_helpers.
#
# normalise_st_id must be octal-safe and accept ST-prefixed short ids. These run
# under /bin/bash (the scripts' shebang) on purpose: the leading-zero octal trap
# (printf %d treating 0044 as octal 36) only bites under that interpreter, so a
# shell that doesn't octalise would pass vacuously.

load "../lib/test_helper.bash"

norm() { /bin/bash -c "source '$INTENT_HOME/bin/intent_helpers'; normalise_st_id '$1'"; }

@test "normalise_st_id pads a bare number" {
  run norm 44
  assert_output "ST0044"
}

@test "normalise_st_id is octal-safe for leading-zero numbers" {
  run norm 0044
  assert_output "ST0044"
  run norm 0011
  assert_output "ST0011"
}

@test "normalise_st_id pads an ST-prefixed short id" {
  run norm ST44
  assert_output "ST0044"
}

@test "normalise_st_id leaves a canonical id unchanged" {
  run norm ST0044
  assert_output "ST0044"
}
