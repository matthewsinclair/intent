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

# ---- issue 0008: the declared-language predicate --------------------------
# Languages-in-use is a configuration decision, not a filesystem detection, so
# anything asking "is this a <lang> project?" reads the declared array rather
# than probing for a marker file. The absent cases must answer false: a
# generator that cannot prove a language is in use should say nothing about it
# rather than assert a default that will be wrong somewhere.

haslang() {
  /bin/bash -c "source '$INTENT_HOME/bin/intent_helpers'; has_project_language '$1' '$2' && echo yes || echo no"
}

@test "has_project_language is true only for a declared language" {
  local cfg="${BATS_TEST_TMPDIR}/config.json"
  echo '{"languages":["shell","elixir"]}' > "$cfg"
  run haslang shell "$cfg"
  assert_output "yes"
  run haslang elixir "$cfg"
  assert_output "yes"
  run haslang rust "$cfg"
  assert_output "no"
}

@test "has_project_language matches whole entries, never substrings" {
  local cfg="${BATS_TEST_TMPDIR}/config.json"
  echo '{"languages":["shell"]}' > "$cfg"
  run haslang she "$cfg"
  assert_output "no"
  run haslang shellscript "$cfg"
  assert_output "no"
}

@test "has_project_language is false for an empty, absent or unreadable config" {
  local cfg="${BATS_TEST_TMPDIR}/config.json"
  echo '{"languages":[]}' > "$cfg"
  run haslang shell "$cfg"
  assert_output "no"

  echo '{}' > "$cfg"
  run haslang shell "$cfg"
  assert_output "no"

  run haslang shell "${BATS_TEST_TMPDIR}/does-not-exist.json"
  assert_output "no"
}
