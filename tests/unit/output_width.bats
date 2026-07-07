#!/usr/bin/env bats
# Terminal-fit table widths: `st list`, `st sync`, and `wp list` render through
# the shared render_table (bin/intent_helpers), which fills the terminal width
# (or an explicit `--width`), with content-fit as the floor so nothing is ever
# truncated. `st sync` composes `st list`, so their output is identical.
# `get_default_width` / `dft_width` remain defined for other generated content.

load "../lib/test_helper.bash"

# get_default_width, sourced and run inside a project dir (helpers.bats pattern).
default_width_in() {
  /bin/bash -c "cd '$1' && source '$INTENT_HOME/bin/intent_helpers' && get_default_width"
}

# Longest content line inside the steel_threads.md index block.
index_width() {
  awk '/BEGIN: STEEL_THREAD_INDEX/{f=1;next} /END: STEEL_THREAD_INDEX/{f=0} f{if(length>m)m=length} END{print m+0}' "$1"
}

# Longest line of a captured stdout blob.
stdout_width() {
  printf '%s\n' "$1" | awk '{if(length>m)m=length} END{print m+0}'
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

@test "st list stdout fills the terminal width and never truncates the slug" {
  local d
  d=$(create_test_project "Width Stdout")
  cd "$d"
  run_intent st new "another sufficiently long steel thread title for measuring stdout width"

  export COLUMNS=250
  run run_intent st list --status all
  assert_success
  local wide
  wide=$(stdout_width "$output")
  printf '%s\n' "$output" | grep -q 'another-sufficiently-long' || fail "slug truncated at COLUMNS=250"
  [ "$wide" -ge 200 ] || fail "stdout width $wide at COLUMNS=250; expected to fill the terminal"

  export COLUMNS=130
  run run_intent st list --status all
  assert_success
  local narrow
  narrow=$(stdout_width "$output")
  [ "$wide" -gt "$narrow" ] || fail "width did not track terminal: 250->$wide 130->$narrow"
}

@test "st list and st sync produce identical output" {
  local d
  d=$(create_test_project "Width Same")
  cd "$d"
  run_intent st new "a title"
  export COLUMNS=160
  run run_intent st list
  assert_success
  local list_out="$output"
  run run_intent st sync
  assert_success
  [ "$output" = "$list_out" ] || fail "st list and st sync differ"
}

@test "st sync --write index fills the terminal width" {
  local d
  d=$(create_test_project "Width File")
  cd "$d"
  run_intent st new "a deliberately long steel thread title whose slug would truncate at eighty columns"
  export COLUMNS=200
  run run_intent st sync --write
  assert_success
  local wide
  wide=$(index_width intent/st/steel_threads.md)
  export COLUMNS=60
  run run_intent st sync --write
  assert_success
  local narrow
  narrow=$(index_width intent/st/steel_threads.md)
  [ "$wide" -gt "$narrow" ] || fail "index width did not track terminal: 200->$wide 60->$narrow"
}

@test "explicit --width overrides the terminal width (list and sync alike)" {
  local d
  d=$(create_test_project "Width Override")
  cd "$d"
  run_intent st new "yet another long steel thread title to exercise the width override path"
  export COLUMNS=250

  run run_intent st list --status all --width 120
  assert_success
  local wl
  wl=$(stdout_width "$output")
  { [ "$wl" -ge 110 ] && [ "$wl" -le 130 ]; } || fail "st list width $wl with --width 120 (COLUMNS=250); expected ~120"

  run run_intent st sync --width 120
  assert_success
  local ws
  ws=$(stdout_width "$output")
  { [ "$ws" -ge 110 ] && [ "$ws" -le 130 ]; } || fail "st sync width $ws with --width 120; expected ~120"
}
