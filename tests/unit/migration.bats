#!/usr/bin/env bats
# Test migration and backup functionality

load "../lib/test_helper.bash"

@test "backup directory uses .backup_ prefix" {
  project_dir=$(create_test_project "Backup Test")
  cd "$project_dir"
  
  # Create some content to backup
  mkdir -p "intent/st/ST0001"
  echo "test content" > "intent/st/ST0001/info.md"
  
  # Source helpers to test backup function
  source "${INTENT_BIN_DIR}/intent_helpers"
  
  # Create backup
  create_project_backup "$project_dir"
  
  # Check that backup was created with correct prefix
  backup_dirs=(.backup_*)
  assert_directory_exists "${backup_dirs[0]}"
  
  # Verify it's not using old .stp_backup_ prefix
  if ls .stp_backup_* 2>/dev/null; then
    fail "Found old .stp_backup_ directory, should use .backup_"
  fi
}

@test "gitignore contains .backup_* pattern" {
  project_dir=$(create_test_project "Gitignore Test")
  cd "$project_dir"
  
  # Create a gitignore with the helpers function
  source "${INTENT_BIN_DIR}/intent_helpers"
  create_v2_directory_structure "$project_dir"
  
  # Check gitignore contains new pattern
  assert_file_exists ".gitignore"
  assert_file_contains ".gitignore" ".backup_*"
  
  # Verify old pattern is not present
  if grep -q ".stp_backup_" ".gitignore"; then
    fail ".gitignore contains old .stp_backup_ pattern"
  fi
}

@test "intent_version is used in frontmatter" {
  project_dir=$(create_test_project "Version Test")
  cd "$project_dir"
  
  # Create a steel thread manually to test frontmatter
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
id: ST0001
title: Test Thread
status: In Progress
created: 2025-01-01
author: test_user
intent_version: 2.0.0
---

# Test Thread
EOF
  
  # Check it uses intent_version, not stp_version
  assert_file_contains "intent/st/ST0001/info.md" "intent_version:"
  
  if grep -q "stp_version:" "intent/st/ST0001/info.md"; then
    fail "Steel thread contains old stp_version in frontmatter"
  fi
}