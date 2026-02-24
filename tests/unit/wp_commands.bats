#!/usr/bin/env bats
# Tests for intent wp commands

load "../lib/test_helper.bash"

# ====================================================================
# Basic Validation
# ====================================================================

@test "wp requires a command" {
  project_dir=$(create_test_project "WP Test")
  cd "$project_dir"

  run run_intent wp
  assert_failure
  assert_output_contains "Work package command is required"
}

@test "wp new requires STID and title" {
  project_dir=$(create_test_project "WP New Test")
  cd "$project_dir"

  run run_intent wp new
  assert_failure
  assert_output_contains "Usage: intent wp new"
}

@test "wp new requires title argument" {
  project_dir=$(create_test_project "WP New Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test
EOF

  run run_intent wp new ST0001
  assert_failure
  assert_output_contains "Usage: intent wp new"
}

@test "wp new errors on non-existent ST" {
  project_dir=$(create_test_project "WP New Error Test")
  cd "$project_dir"

  run run_intent wp new ST9999 "Some WP"
  assert_failure
  assert_output_contains "Steel thread not found"
}

@test "wp done requires STID/WP specifier" {
  project_dir=$(create_test_project "WP Done Test")
  cd "$project_dir"

  run run_intent wp done
  assert_failure
  assert_output_contains "Usage: intent wp done"
}

@test "wp show requires STID/WP specifier" {
  project_dir=$(create_test_project "WP Show Test")
  cd "$project_dir"

  run run_intent wp show
  assert_failure
  assert_output_contains "Usage: intent wp show"
}

@test "wp done requires WP number in specifier" {
  project_dir=$(create_test_project "WP Done Specifier Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test
EOF

  run run_intent wp done ST0001
  assert_failure
  assert_output_contains "Work package number is required"
}

# ====================================================================
# wp new
# ====================================================================

@test "wp new creates WP directory and info.md" {
  project_dir=$(create_test_project "WP New Create Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp new ST0001 "First Work Package"
  assert_success
  assert_directory_exists "intent/st/ST0001/WP/01"
  assert_file_exists "intent/st/ST0001/WP/01/info.md"
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "First Work Package"
}

@test "wp new creates sequential WP numbers" {
  project_dir=$(create_test_project "WP Sequential Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp new ST0001 "First WP"
  assert_success
  assert_directory_exists "intent/st/ST0001/WP/01"

  run run_intent wp new ST0001 "Second WP"
  assert_success
  assert_directory_exists "intent/st/ST0001/WP/02"

  run run_intent wp new ST0001 "Third WP"
  assert_success
  assert_directory_exists "intent/st/ST0001/WP/03"
}

@test "wp new works with bare number STID" {
  project_dir=$(create_test_project "WP Bare Number Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0011
  cat > intent/st/ST0011/info.md << 'EOF'
---
status: WIP
---
# ST0011: Test Thread
EOF

  run run_intent wp new 11 "Bare Number WP"
  assert_success
  assert_directory_exists "intent/st/ST0011/WP/01"
  assert_file_contains "intent/st/ST0011/WP/01/info.md" "Bare Number WP"
}

@test "wp new works with full STID format" {
  project_dir=$(create_test_project "WP Full STID Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0011
  cat > intent/st/ST0011/info.md << 'EOF'
---
status: WIP
---
# ST0011: Test Thread
EOF

  run run_intent wp new ST0011 "Full STID WP"
  assert_success
  assert_directory_exists "intent/st/ST0011/WP/01"
}

@test "wp new creates WP directory if it doesn't exist" {
  project_dir=$(create_test_project "WP Auto Dir Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  # No WP/ dir exists yet
  [ ! -d "intent/st/ST0001/WP" ]

  run run_intent wp new ST0001 "Auto Dir WP"
  assert_success
  assert_directory_exists "intent/st/ST0001/WP/01"
}

@test "wp new handles special characters in title" {
  project_dir=$(create_test_project "WP Special Chars Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp new ST0001 "Add foo/bar & baz"
  assert_success
  assert_file_exists "intent/st/ST0001/WP/01/info.md"
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "Add foo/bar & baz"
}

@test "wp new generates correct frontmatter" {
  project_dir=$(create_test_project "WP Frontmatter Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp new ST0001 "Frontmatter Test"
  assert_success

  assert_file_contains "intent/st/ST0001/WP/01/info.md" "wp_id: WP-01"
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "title: \"Frontmatter Test\""
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "status: Not Started"
}

@test "wp new finds ST in COMPLETED directory" {
  project_dir=$(create_test_project "WP Completed ST Test")
  cd "$project_dir"

  mkdir -p intent/st/COMPLETED/ST0001
  cat > intent/st/COMPLETED/ST0001/info.md << 'EOF'
---
status: Completed
---
# ST0001: Completed Thread
EOF

  run run_intent wp new ST0001 "WP in Completed ST"
  assert_success
  assert_directory_exists "intent/st/COMPLETED/ST0001/WP/01"
}

# ====================================================================
# wp done
# ====================================================================

@test "wp done marks WP status as Done" {
  project_dir=$(create_test_project "WP Done Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "Test WP"
status: WIP
---
# WP-01: Test WP
EOF

  run run_intent wp done ST0001/01
  assert_success
  assert_output_contains "Marked work package as Done"
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "status: Done"
}

@test "wp done errors on non-existent WP" {
  project_dir=$(create_test_project "WP Done Error Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp done ST0001/99
  assert_failure
  assert_output_contains "Work package not found"
}

@test "wp done works with bare number specifier" {
  project_dir=$(create_test_project "WP Done Bare Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0011/WP/01
  cat > intent/st/ST0011/info.md << 'EOF'
---
status: WIP
---
# ST0011: Test Thread
EOF
  cat > intent/st/ST0011/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "Test WP"
status: WIP
---
# WP-01: Test WP
EOF

  run run_intent wp done 11/01
  assert_success
  assert_output_contains "Marked work package as Done"
  assert_file_contains "intent/st/ST0011/WP/01/info.md" "status: Done"
}

@test "wp done hints when all WPs complete" {
  project_dir=$(create_test_project "WP All Done Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  mkdir -p intent/st/ST0001/WP/02
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "First WP"
status: Done
---
# WP-01: First WP
EOF
  cat > intent/st/ST0001/WP/02/info.md << 'EOF'
---
wp_id: WP-02
title: "Second WP"
status: WIP
---
# WP-02: Second WP
EOF

  run run_intent wp done ST0001/02
  assert_success
  assert_output_contains "All WPs complete"
  assert_output_contains "intent st done ST0001"
}

# ====================================================================
# wp start
# ====================================================================

@test "wp start marks WP status as WIP" {
  project_dir=$(create_test_project "WP Start Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "Test WP"
status: Not Started
---
# WP-01: Test WP
EOF

  run run_intent wp start ST0001/01
  assert_success
  assert_output_contains "Marked work package as WIP"
  assert_file_contains "intent/st/ST0001/WP/01/info.md" "status: WIP"
}

@test "wp start errors on non-existent WP" {
  project_dir=$(create_test_project "WP Start Error Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp start ST0001/05
  assert_failure
  assert_output_contains "Work package not found"
}

# ====================================================================
# wp list
# ====================================================================

@test "wp list shows all WPs for a steel thread" {
  project_dir=$(create_test_project "WP List Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  mkdir -p intent/st/ST0001/WP/02
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "First WP"
scope: Small
status: Done
---
# WP-01: First WP
EOF
  cat > intent/st/ST0001/WP/02/info.md << 'EOF'
---
wp_id: WP-02
title: "Second WP"
scope: Medium
status: WIP
---
# WP-02: Second WP
EOF

  run run_intent wp list ST0001
  assert_success
  assert_output_contains "First WP"
  assert_output_contains "Second WP"
  assert_output_contains "Done"
  assert_output_contains "WIP"
}

@test "wp list shows no work packages message when empty" {
  project_dir=$(create_test_project "WP List Empty Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp list ST0001
  assert_success
  assert_output_contains "No work packages found"
}

@test "wp list works with bare number STID" {
  project_dir=$(create_test_project "WP List Bare Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0011/WP/01
  cat > intent/st/ST0011/info.md << 'EOF'
---
status: WIP
---
# ST0011: Test Thread
EOF
  cat > intent/st/ST0011/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "Bare Number List Test"
scope: Small
status: Not Started
---
# WP-01: Bare Number List Test
EOF

  run run_intent wp list 11
  assert_success
  assert_output_contains "Bare Number List Test"
}

@test "wp list handles WPs with missing frontmatter" {
  project_dir=$(create_test_project "WP List Fallback Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  # WP without frontmatter title, only has heading
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
status: WIP
---
# WP-01: Heading Only Title
EOF

  run run_intent wp list ST0001
  assert_success
  assert_output_contains "Heading Only Title"
}

# ====================================================================
# wp show
# ====================================================================

@test "wp show displays WP info.md content" {
  project_dir=$(create_test_project "WP Show Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001/WP/01
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF
  cat > intent/st/ST0001/WP/01/info.md << 'EOF'
---
wp_id: WP-01
title: "Show Test WP"
status: WIP
---
# WP-01: Show Test WP

## Objective

Test the show command.
EOF

  run run_intent wp show ST0001/01
  assert_success
  assert_output_contains "Show Test WP"
  assert_output_contains "Test the show command"
}

@test "wp show errors on non-existent WP" {
  project_dir=$(create_test_project "WP Show Error Test")
  cd "$project_dir"

  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
status: WIP
---
# ST0001: Test Thread
EOF

  run run_intent wp show ST0001/42
  assert_failure
  assert_output_contains "Work package not found"
}

# ====================================================================
# wp help
# ====================================================================

@test "wp help displays usage information" {
  project_dir=$(create_test_project "WP Help Test")
  cd "$project_dir"

  run run_intent wp help
  # help exits with 1 (usage pattern)
  assert_output_contains "Usage: intent wp"
  assert_output_contains "new"
  assert_output_contains "done"
  assert_output_contains "start"
  assert_output_contains "list"
  assert_output_contains "show"
}

@test "unknown wp command shows error" {
  project_dir=$(create_test_project "WP Unknown Test")
  cd "$project_dir"

  run run_intent wp foobar
  assert_failure
  assert_output_contains "Unknown wp command"
}
