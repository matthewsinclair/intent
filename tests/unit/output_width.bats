#!/usr/bin/env bats
# Content-fit table widths (supersedes ST0051's destination-based width).
# `st list`, `st sync`, and `wp list` size each column to its widest cell via
# the shared render_table (bin/intent_helpers), so the table width is a
# function of the DATA -- independent of the terminal, of config `dft_width`,
# and of `--width` (now inert). No slug is ever truncated. `get_default_width` /
# `dft_width` remain defined for any other generated content.

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

@test "st sync --write index is content-fit (independent of terminal width)" {
  local d
  d=$(create_test_project "Width File")
  cd "$d"
  run_intent st new "a deliberately long steel thread title whose slug would truncate at eighty columns"

  COLUMNS=40 run run_intent st sync --write
  assert_success
  local narrow
  narrow=$(index_width intent/st/steel_threads.md)

  COLUMNS=200 run run_intent st sync --write
  assert_success
  local wide
  wide=$(index_width intent/st/steel_threads.md)

  # Content-fit: the file sizes to the data, so its width is identical at 40 and
  # 200 columns (the old bug padded to a fixed width and truncated the slug).
  [ "$narrow" -eq "$wide" ] || fail "index width varies with terminal: $narrow (40) vs $wide (200)"
}

@test "st list stdout is content-fit and never truncates the slug" {
  local d
  d=$(create_test_project "Width Stdout")
  cd "$d"
  run_intent st new "another sufficiently long steel thread title for measuring stdout width"

  COLUMNS=40 run run_intent st list --status all
  assert_success
  local narrow
  narrow=$(stdout_width "$output")
  # The full slug is present -- content-fit never truncates.
  printf '%s\n' "$output" | grep -q 'another-sufficiently-long' || fail "slug truncated at 40 cols"

  COLUMNS=200 run run_intent st list --status all
  assert_success
  local wide
  wide=$(stdout_width "$output")

  # Width is a function of the data, so it does not change with the terminal.
  [ "$narrow" -eq "$wide" ] || fail "stdout width varies with terminal: $narrow (40) vs $wide (200)"
}

@test "explicit --width is inert (content-fit ignores it)" {
  local d
  d=$(create_test_project "Width Override")
  cd "$d"
  run_intent st new "yet another long steel thread title to exercise the width override path"

  run run_intent st list --status all
  assert_success
  local plain="$output"

  run run_intent st list --status all --width 150
  assert_success

  # --width no longer sizes the table; the output is identical with and without.
  [ "$output" = "$plain" ] || fail "--width changed the output; content-fit should ignore it"
}
