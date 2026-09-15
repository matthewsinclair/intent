#!/usr/bin/env bats
# artefact_source_commit keeps a REFUSAL apart from a MISSING MARKER, and the stage
# record never writes the second for the first (vc's ruling, 2026-09-15).
#
# Until then the function sent `strings`' stderr to /dev/null and returned 1 either
# way, so a binary `strings` could not read and a binary carrying no marker gave the
# same answer, and `int macos stage` recorded `NO MARKER` for bytes nobody had read.
#
# THE ARTEFACTS ARE TEXT FILES, as in artefact_currency_verdict.bats: the function is
# `strings` and nothing else, so a text file carrying the marker is a faithful
# subject. The refusal is produced the one way that needs no broken toolchain: a
# PATH carrying every tool the function calls except `strings`.

load "../lib/test_helper"

ARTEFACT_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/artefact.lib"
MACOS="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/macos"

path_without_strings() {
  local dir="${TEST_TEMP_DIR}/nostrings" tool
  mkdir -p "$dir"
  for tool in mktemp grep head tr rm; do ln -sf "$(command -v "$tool")" "$dir/$tool"; done
  printf '%s' "$dir"
}

@test "artefact_source_commit: a value at rc 0, silence at rc 1 for no marker, a refusal at rc 2 when strings cannot read" {
  printf 'bytes\n[intent-source-commit:abc123]\n' >"${TEST_TEMP_DIR}/marked"
  printf 'bytes and nothing else\n' >"${TEST_TEMP_DIR}/unmarked"

  run bash -c ". '$ARTEFACT_LIB'; artefact_source_commit '${TEST_TEMP_DIR}/marked'"
  [ "$status" -eq 0 ]
  [ "$output" = "abc123" ]

  run bash -c ". '$ARTEFACT_LIB'; artefact_source_commit '${TEST_TEMP_DIR}/unmarked'"
  [ "$status" -eq 1 ]
  [ -z "$output" ]

  local nostrings
  nostrings="$(path_without_strings)"
  run bash -c "PATH='$nostrings'; . '$ARTEFACT_LIB'; artefact_source_commit '${TEST_TEMP_DIR}/marked'"
  [ "$status" -eq 2 ]
  [[ "$output" == *"strings could not read"* ]]
  [[ "$output" == *"remedy:"* ]]
}

@test "int macos stage record: a strings refusal writes no NO MARKER row, and a missing marker still does" {
  local fns="${TEST_TEMP_DIR}/macosfns.sh" stage="${TEST_TEMP_DIR}/dist" nostrings
  sed -n '/^BINARIES=/p;/^artefact_commit_lines() {/,/^}/p' "$MACOS" >"$fns"
  mkdir -p "$stage"
  printf 'bytes\n[intent-source-commit:abc123]\n' >"$stage/intent-test-triple"
  printf 'bytes and nothing else\n' >"$stage/intentd-test-triple"

  run bash -c ". '$ARTEFACT_LIB'; . '$fns'; artefact_commit_lines '$stage' test-triple"
  [ "$status" -eq 0 ]
  [[ "$output" == *"intent-test-triple"*"abc123"* ]]
  [[ "$output" == *"intentd-test-triple"*"NO MARKER"* ]]

  nostrings="$(path_without_strings)"
  run bash -c "PATH='$nostrings'; . '$ARTEFACT_LIB'; . '$fns'; artefact_commit_lines '$stage' test-triple"
  [ "$status" -eq 2 ]
  [[ "$output" != *"NO MARKER"* ]]
  [[ "$output" == *"strings could not read"* ]]
}
