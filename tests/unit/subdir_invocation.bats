#!/usr/bin/env bats
# ST0033: intent subcommands must work from any subdirectory of a project,
# not just the project root. Outside any project, they must fail cleanly
# without creating .intent/ or intent/ at cwd.

load "../lib/test_helper.bash"

@test "st list works from intent/st subdirectory" {
  project_dir=$(create_test_project "Subdir List Test")
  export EDITOR=echo
  cd "$project_dir"
  run run_intent st new "First"
  assert_success

  cd "$project_dir/intent/st"
  run run_intent st list --status all
  assert_success
  assert_output_contains "ST0001"
}

@test "st list works from a deeply nested directory" {
  project_dir=$(create_test_project "Deep Subdir Test")
  export EDITOR=echo
  cd "$project_dir"
  run run_intent st new "Deep"
  assert_success

  mkdir -p "$project_dir/intent/docs/a/b/c"
  cd "$project_dir/intent/docs/a/b/c"
  run run_intent st list --status all
  assert_success
  assert_output_contains "ST0001"
}

@test "st new from subdirectory creates ST at project root, not cwd" {
  project_dir=$(create_test_project "Subdir New Test")
  export EDITOR=echo
  mkdir -p "$project_dir/intent/docs"
  cd "$project_dir/intent/docs"

  run run_intent st new "From Subdir"
  assert_success

  # ST must be under the real project root
  assert_directory_exists "$project_dir/intent/st/NOT-STARTED/ST0001"
  # and nothing stray at the subdirectory
  [ ! -d "$project_dir/intent/docs/.intent" ]
  [ ! -d "$project_dir/intent/docs/intent" ]
}

@test "wp list works from subdirectory" {
  project_dir=$(create_test_project "Subdir WP Test")
  export EDITOR=echo
  cd "$project_dir"
  run run_intent st new "Host"
  assert_success
  run run_intent wp new ST0001 "Package One"
  assert_success

  cd "$project_dir/intent/st"
  run run_intent wp list ST0001
  assert_success
  assert_output_contains "Package One"
}

@test "doctor works from subdirectory" {
  project_dir=$(create_test_project "Subdir Doctor Test")
  mkdir -p "$project_dir/intent/llm"
  cd "$project_dir/intent/llm"

  run run_intent doctor
  assert_success
}

@test "outside any project, commands fail cleanly without creating .intent" {
  # setup() put us in a fresh empty $TEST_TEMP_DIR already
  run run_intent st list
  assert_failure
  assert_output_contains "not in an Intent project"

  [ ! -d "$TEST_TEMP_DIR/.intent" ]
  [ ! -d "$TEST_TEMP_DIR/intent" ]
}

@test "INTENT_ORIG_CWD is exported to subcommands" {
  project_dir=$(create_test_project "Orig Cwd Test")
  mkdir -p "$project_dir/intent/docs/sub"
  cd "$project_dir/intent/docs/sub"

  # Use a harness that prints the env var via intent's own helper path.
  # Run any project command and confirm INTENT_ORIG_CWD propagates.
  # We check by inspecting the env of a shell subprocess started via bash -c
  # from within the project: the simplest assertion is behavioural -- treeindex
  # accepts a relative path resolved against the original cwd.
  mkdir -p "$project_dir/intent/docs/sub/target"
  run run_intent treeindex target
  # Either success, or an error that is NOT "Directory does not exist"
  # (treeindex may fail for other reasons in a minimal fixture, but the
  # original-cwd relative path resolution must not be what blocks it).
  refute_output_contains "Directory does not exist"
}
