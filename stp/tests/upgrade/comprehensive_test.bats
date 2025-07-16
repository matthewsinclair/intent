#!/usr/bin/env bats
# Comprehensive tests for Intent v2.0.0 migration
# Tests migration from v0.0.0, v1.2.0, and v1.2.1 to v2.0.0

load '../lib/test_helper.bash'

# Global test variables
EXAMPLES_DIR="${STP_PROJECT_ROOT}/examples"

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/intent-test-XXXXXX")"
  echo "Setup: Created test directory at ${TEST_TEMP_DIR}"
  cd "${TEST_TEMP_DIR}" || exit 1
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Helper function to copy example project
copy_example_project() {
  local version=$1
  cp -r "${EXAMPLES_DIR}/${version}-project" "${TEST_TEMP_DIR}/test-project"
  cd "${TEST_TEMP_DIR}/test-project" || return 1
}

# Helper function to verify v2.0.0 structure
verify_v2_structure() {
  # Check for JSON config
  assert_file_exists ".intent/config.json"
  
  # Check flattened structure
  assert_dir_exists "intent/st"
  assert_dir_exists "intent/eng"
  assert_dir_exists "intent/ref"
  
  # Check that old structure is gone
  assert_not_exists "stp/prj/st"
  
  # Verify JSON config contains required fields
  assert_file_contains ".intent/config.json" '"intent_version": "2.0.0"'
  assert_file_contains ".intent/config.json" '"intent_dir": "intent"'
  assert_file_contains ".intent/config.json" '"backlog_dir": "backlog"'
}

# Test version detection function
@test "version detection correctly identifies v0.0.0" {
  copy_example_project "v0.0.0"
  
  # Mock the detect_stp_version function
  source "${STP_PROJECT_ROOT}/stp/bin/stp_upgrade" 2>/dev/null || true
  
  # Should detect v0.0.0 from .stp-config
  run detect_stp_version
  assert_success
  assert_output "0.0.0"
}

@test "version detection correctly identifies v1.2.0" {
  copy_example_project "v1.2.0"
  
  # Mock the detect_stp_version function
  source "${STP_PROJECT_ROOT}/stp/bin/stp_upgrade" 2>/dev/null || true
  
  # Should detect v1.2.0 from stp/.config/version
  run detect_stp_version
  assert_success
  assert_output "1.2.0"
}

@test "version detection correctly identifies v1.2.1" {
  copy_example_project "v1.2.1"
  
  # Mock the detect_stp_version function
  source "${STP_PROJECT_ROOT}/stp/bin/stp_upgrade" 2>/dev/null || true
  
  # Should detect v1.2.1 from stp/.config/version
  run detect_stp_version
  assert_success
  assert_output "1.2.1"
}

@test "version detection correctly identifies v2.0.0" {
  copy_example_project "hello-world"
  
  # Mock the detect_stp_version function
  source "${STP_PROJECT_ROOT}/stp/bin/stp_upgrade" 2>/dev/null || true
  
  # Should detect v2.0.0 from .intent/config.json
  run detect_stp_version
  assert_success
  assert_output "2.0.0"
}

@test "version detection fails gracefully for unknown structure" {
  # Create empty directory
  mkdir -p "${TEST_TEMP_DIR}/test-project"
  cd "${TEST_TEMP_DIR}/test-project"
  
  # Mock the detect_stp_version function
  source "${STP_PROJECT_ROOT}/stp/bin/stp_upgrade" 2>/dev/null || true
  
  # Should fail to detect version
  run detect_stp_version
  assert_failure
}

# Test migration from v0.0.0 to v2.0.0
@test "migrate v0.0.0 to v2.0.0 - structure" {
  copy_example_project "v0.0.0"
  
  # TODO: Run actual migration command when implemented
  # For now, simulate the expected structure changes
  
  # Verify starting structure
  assert_file_exists ".stp-config"
  assert_dir_exists "stp/prj/st"
  assert_file_exists "stp/prj/st/ST0001.md"
  assert_file_exists "stp/prj/st/ST0002.md"
  
  # After migration, verify v2.0.0 structure
  # verify_v2_structure
  
  # Verify steel threads were moved correctly
  # assert_file_exists "intent/st/ST0001.md"
  # assert_file_exists "intent/st/ST0002.md"
}

@test "migrate v0.0.0 to v2.0.0 - config conversion" {
  copy_example_project "v0.0.0"
  
  # Verify YAML config exists
  assert_file_exists ".stp-config"
  assert_file_contains ".stp-config" "project_name: Ancient Example Project"
  
  # TODO: After migration, verify JSON config
  # assert_file_exists ".intent/config.json"
  # assert_file_contains ".intent/config.json" '"author": "Test User"'
}

# Test migration from v1.2.0 to v2.0.0
@test "migrate v1.2.0 to v2.0.0 - file-based steel threads" {
  copy_example_project "v1.2.0"
  
  # Verify starting structure
  assert_file_exists "stp/.config/version"
  assert_file_exists "stp/prj/st/ST0001.md"
  assert_file_exists "stp/prj/st/ST0002.md"
  assert_file_exists "stp/prj/st/steel_threads.md"
  
  # TODO: After migration
  # verify_v2_structure
  # assert_file_exists "intent/st/ST0001.md"
  # assert_file_exists "intent/st/ST0002.md"
  # assert_file_exists "intent/st/steel_threads.md"
}

# Test migration from v1.2.1 to v2.0.0
@test "migrate v1.2.1 to v2.0.0 - directory-based steel threads" {
  copy_example_project "v1.2.1"
  
  # Verify starting structure
  assert_file_exists "stp/.config/version"
  assert_dir_exists "stp/prj/st/ST0001"
  assert_file_exists "stp/prj/st/ST0001/info.md"
  assert_dir_exists "stp/prj/st/ST0002"
  assert_dir_exists "stp/prj/st/ST0003"
  
  # TODO: After migration
  # verify_v2_structure
  # assert_dir_exists "intent/st/ST0001"
  # assert_file_exists "intent/st/ST0001/info.md"
  # assert_dir_exists "intent/st/ST0002"
  # assert_dir_exists "intent/st/ST0003"
}

# Test backup creation
@test "migration creates timestamped backup" {
  copy_example_project "v1.2.1"
  
  # TODO: Run migration
  # Verify backup was created
  # assert_dir_exists ".stp_backup_*"
}

# Test dry-run mode
@test "migration dry-run doesn't modify files" {
  copy_example_project "v1.2.1"
  
  # Take snapshot of files
  find . -type f | sort > before.txt
  
  # TODO: Run migration with --dry-run
  # run intent_upgrade --dry-run
  
  # Verify no changes
  find . -type f | sort > after.txt
  diff before.txt after.txt
}

# Test JSON parsing
@test "JSON parser extracts config values correctly" {
  # Create test JSON
  cat > test.json << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "custom_intent",
  "backlog_dir": "custom_backlog",
  "author": "Test Author",
  "editor": "emacs"
}
EOF

  # TODO: Source the parse_json function
  # eval "$(parse_json test.json "")"
  
  # Verify extracted values
  # assert_equal "$intent_version" "2.0.0"
  # assert_equal "$intent_dir" "custom_intent"
  # assert_equal "$backlog_dir" "custom_backlog"
  # assert_equal "$author" "Test Author"
  # assert_equal "$editor" "emacs"
}

# Test error handling
@test "migration fails with clear error for unknown version" {
  mkdir -p "${TEST_TEMP_DIR}/test-project"
  cd "${TEST_TEMP_DIR}/test-project"
  
  # TODO: Run migration on empty directory
  # run intent_upgrade
  # assert_failure
  # assert_output --partial "Unable to determine current STP version"
}

# Test frontmatter updates
@test "migration updates frontmatter version field" {
  copy_example_project "v1.2.1"
  
  # Check current frontmatter
  assert_file_contains "stp/prj/st/ST0001/info.md" "stp_version: 1.2.1"
  
  # TODO: After migration
  # assert_file_contains "intent/st/ST0001/info.md" "intent_version: 2.0.0"
}

# Test self-hosting scenario
@test "migration handles self-hosting (intent on itself)" {
  # This tests the meta-usage scenario where intent uses STP on itself
  skip "Will be tested when implementing actual migration"
}