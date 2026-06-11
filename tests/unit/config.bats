#!/usr/bin/env bats
# Test configuration loading and PROJECT_ROOT detection

load "../lib/test_helper.bash"

@test "PROJECT_ROOT is detected correctly" {
  # Create nested directory structure
  project_dir=$(create_test_project "Root Project")
  mkdir -p "$project_dir/subdir/deeper"
  
  # From project root
  cd "$project_dir"
  run run_intent info
  assert_success
  assert_output_contains "Location:        $project_dir"
  
  # From subdirectory
  cd "$project_dir/subdir"
  run run_intent info
  assert_success
  assert_output_contains "Location:        $project_dir"
  
  # From deeper subdirectory
  cd "$project_dir/subdir/deeper"
  run run_intent info
  assert_success
  assert_output_contains "Location:        $project_dir"
}

@test "config.json is loaded correctly" {
  project_dir=$(create_test_project "Config Test Project")
  cd "$project_dir"
  
  # Update config with custom values
  cat > "intent/.config/config.json" << EOF
{
  "intent_version": "2.0.0",
  "project_name": "Custom Project Name",
  "author": "custom_author",
  "created_date": "2025-01-15T10:00:00Z"
}
EOF
  
  run run_intent info
  assert_success
  assert_output_contains "Name:            Custom Project Name"
  assert_output_contains "Author:          custom_author"
}

@test "legacy stp structure is detected" {
  # Create legacy structure in a test directory
  legacy_dir="${TEST_TEMP_DIR}/legacy_project"
  mkdir -p "$legacy_dir/stp/prj/st"
  mkdir -p "$legacy_dir/stp/.config"
  echo "stp_version: 2.0.0" > "$legacy_dir/stp/.config/version"
  
  cd "$legacy_dir"
  
  # Intent should detect this as a legacy project
  run run_intent doctor
  assert_success
  # Should show it found a project needing upgrade
  assert_output_contains "doctor:"
}

@test "missing config shows appropriate error" {
  # Create directory without intent/.config/config.json
  mkdir empty_dir
  cd empty_dir
  
  run run_intent st list
  assert_failure
  assert_output_contains "not in an Intent project directory"
}

@test "config values with shell metacharacters are never evaled" {
  # ST0042 T1 regression guard: load_intent_config used to eval jq-built
  # key="value" assignments, so a config value containing $(...) executed
  # arbitrary shell on any project-scoped command. Values must load verbatim.
  project_dir=$(create_test_project "Eval Inert Project")
  cd "$project_dir"
  mkdir -p intent/st

  local marker="$TEST_TEMP_DIR/eval_pwned"
  cat > "intent/.config/config.json" << EOF
{
  "intent_version": "2.11.11",
  "project_name": "Eval Inert Project",
  "author": "\$(touch $marker)",
  "created_date": "2025-01-15T10:00:00Z"
}
EOF

  run run_intent st list
  assert_success
  [ ! -f "$marker" ] || fail "config value was evaled: marker file created"

  run run_intent info
  assert_success
  [ ! -f "$marker" ] || fail "config value was evaled by info: marker file created"
  assert_output_contains 'Author:          $(touch'
}
