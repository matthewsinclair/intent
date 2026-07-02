#!/usr/bin/env bats
# ST0051: generated FILES take their width from config `dft_width` (default
# 120); STDOUT takes the live terminal width; an explicit `--width N` overrides
# both. The bug this pins: `intent st sync --write` hardcoded 80 in bin/intent_st,
# so the steel_threads.md slug column truncated (`add-modules-properly-t...`).

load "../lib/test_helper.bash"

# get_default_width, sourced and run inside a project dir (helpers.bats pattern).
default_width_in() {
  /bin/bash -c "cd '$1' && source '$INTENT_HOME/bin/intent_helpers' && get_default_width"
}

# Longest content line inside the steel_threads.md index block. The dash
# separator row is full-width with no trailing-space ambiguity, so max line
# length == table width.
index_width() {
  awk '/BEGIN: STEEL_THREAD_INDEX/{f=1;next} /END: STEEL_THREAD_INDEX/{f=0} f{if(length>m)m=length} END{print m+0}' "$1"
}

@test "get_default_width reads dft_width from config" {
  local d
  d=$(create_test_project "Width Cfg")
  jq '. + {dft_width: 200}' "$d/intent/.config/config.json" > "$d/c.json"
  mv "$d/c.json" "$d/intent/.config/config.json"
  run default_width_in "$d"
  assert_output "200"
}

@test "get_default_width defaults to 120 when dft_width is absent" {
  local d
  d=$(create_test_project "Width Default")
  run default_width_in "$d"
  assert_output "120"
}

@test "st sync --write sizes the file from dft_width (120), not the terminal" {
  local d
  d=$(create_test_project "Width File")
  cd "$d"
  run_intent st new "a deliberately long steel thread title whose slug would truncate at eighty columns"
  export COLUMNS=40
  run run_intent st sync --write
  assert_success
  local w
  w=$(index_width intent/st/steel_threads.md)
  [ "$w" -ge 115 ] || fail "index width $w; expected ~120 (bug hardcoded 80; terminal was 40)"
}

@test "st list renders to stdout at the terminal width" {
  local d
  d=$(create_test_project "Width Stdout")
  cd "$d"
  run_intent st new "another sufficiently long steel thread title for measuring stdout width"
  export COLUMNS=200
  run run_intent st list --status all
  assert_success
  local w
  w=$(printf '%s\n' "$output" | awk '{if(length>m)m=length} END{print m+0}')
  [ "$w" -ge 150 ] || fail "stdout width $w; expected ~200 (terminal), not the 120 file default"
}

@test "explicit --width overrides both the file and stdout paths" {
  local d
  d=$(create_test_project "Width Override")
  cd "$d"
  run_intent st new "yet another long steel thread title to exercise the width override path"
  run run_intent st sync --write --width 150
  assert_success
  local w
  w=$(index_width intent/st/steel_threads.md)
  [ "$w" -ge 140 ] || fail "index width $w; expected ~150 (explicit override)"
}
