#!/usr/bin/env bats
# devbin_hosting -- what `bin/devbin hosting`'s default sweep may drive (issue 0472).
#
# The default sweep is labelled READ-ONLY and drives READONLY_VERBS. It carried
# `agents sync`, which rewrites AGENTS.md, and the retired `fileindex`, which can
# only refuse; MUTATING_VERBS carried the retired `claude prime`.

load "../lib/test_helper.bash"

HOSTING="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/hosting"

# verbs <ARRAY> -- the quoted entries of one array declaration in the handler.
verbs() {
  sed -n "/^$1=(/,/)/p" "$HOSTING" | grep -o '"[^"]*"' | tr -d '"'
}

@test "hosting: no writer is in the read-only sweep" {
  run verbs READONLY_VERBS
  [ "$status" -eq 0 ]
  [ -n "$output" ]
  [[ "$output" != *"agents sync"* ]]
  [[ "$output" != *"upgrade"* ]]
  [[ "$output" != *"init"* ]]
}

@test "hosting: agents sync is a mutating verb" {
  run verbs MUTATING_VERBS
  [[ "$output" == *"agents sync"* ]]
}

@test "hosting: neither list drives a verb v3 retired" {
  run bash -c "sed -n '/^READONLY_VERBS=(/,/)/p;/^MUTATING_VERBS=(/,/)/p' '$HOSTING'"
  [[ "$output" != *"fileindex"* ]]
  [[ "$output" != *"claude prime"* ]]
}
