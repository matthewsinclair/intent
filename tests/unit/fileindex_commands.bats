#!/usr/bin/env bats
# Test suite for intent_fileindex command

load ../lib/test_helper

# Test basic functionality
@test "fileindex: shows help with -h flag" {
  run "${INTENT_BIN_DIR}/intent_fileindex" -h
  assert_failure
  assert_output_contains "Usage: intent_fileindex [OPTIONS] [STARTDIR] [FILESPEC]"
  assert_output_contains "Create and manage file indexes with checkbox states"
}

@test "fileindex: shows help with --help flag" {
  run "${INTENT_BIN_DIR}/intent_fileindex" --help
  assert_failure
  assert_output_contains "Usage: intent_fileindex [OPTIONS] [STARTDIR] [FILESPEC]"
}

# Test standalone mode
@test "fileindex: lists files in current directory (standalone mode)" {
  # Create test files
  touch test1.ex test2.exs test3.txt
  
  run "${INTENT_BIN_DIR}/intent_fileindex"
  assert_success
  assert_output_contains "[ ] ./test1.ex"
  assert_output_contains "[ ] ./test2.exs"
  refute_output_contains "test3.txt"
}

@test "fileindex: respects custom filespec" {
  # Create test files
  touch test1.py test2.py test3.rb
  
  run "${INTENT_BIN_DIR}/intent_fileindex" . "*.py"
  assert_success
  assert_output_contains "[ ] ./test1.py"
  assert_output_contains "[ ] ./test2.py"
  refute_output_contains "test3.rb"
}

@test "fileindex: recursive search with -r flag" {
  # Create nested structure
  mkdir -p subdir/nested
  touch file1.ex subdir/file2.ex subdir/nested/file3.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -r
  assert_success
  assert_output_contains "[ ] ./file1.ex"
  assert_output_contains "[ ] ./subdir/file2.ex"
  assert_output_contains "[ ] ./subdir/nested/file3.ex"
}

@test "fileindex: non-recursive by default" {
  # Create nested structure
  mkdir -p subdir
  touch file1.ex subdir/file2.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex"
  assert_success
  assert_output_contains "[ ] ./file1.ex"
  refute_output_contains "subdir/file2.ex"
}

# Test output options
@test "fileindex: output to file with -f" {
  touch test1.ex test2.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -f output.txt
  assert_success
  assert_file_exists output.txt
  assert_file_contains output.txt "[ ] ./test1.ex"
  assert_file_contains output.txt "[ ] ./test2.ex"
}

@test "fileindex: output to file with --file" {
  touch test1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" --file output2.txt
  assert_success
  assert_file_exists output2.txt
  assert_file_contains output2.txt "[ ] ./test1.ex"
}

# Test index file functionality
@test "fileindex: creates index file with -i" {
  touch file1.ex file2.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  assert_file_exists test.index
  
  # Check JSON header
  assert_file_contains test.index '"generator": "intent-fileindex"'
  assert_file_contains test.index '"context": "standalone"'
  
  # Check file entries
  assert_file_contains test.index "[ ] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: preserves checkbox states in index" {
  touch file1.ex file2.ex
  
  # Create initial index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Manually mark one file as checked
  sed -i.bak 's/\[ \] \.\/file1\.ex/[x] .\/file1.ex/' test.index
  
  # Run again
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Verify state was preserved
  assert_file_contains test.index "[x] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: adds new files to existing index" {
  touch file1.ex
  
  # Create initial index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  assert_file_contains test.index "[ ] ./file1.ex"
  
  # Add new file
  touch file2.ex
  
  # Run again
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Both files should be present
  assert_file_contains test.index "[ ] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: removes deleted files from index" {
  touch file1.ex file2.ex
  
  # Create initial index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Mark one as checked
  sed -i.bak 's/\[ \] \.\/file2\.ex/[x] .\/file2.ex/' test.index
  
  # Remove a file
  rm file1.ex
  
  # Run again
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Only file2 should remain
  refute_output_contains "file1.ex"
  assert_file_contains test.index "[x] ./file2.ex"
}

# Test verbose mode
@test "fileindex: verbose mode shows processing details" {
  touch file1.ex file2.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -v
  assert_success
  assert_output_contains "Processing:"
  assert_output_contains "Summary: Processed 2 files"
}

@test "fileindex: verbose mode with index file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -v -i test.index
  assert_success
  assert_output_contains "Index updated: ./test.index"
}

# Test Intent project integration
@test "fileindex: detects Intent project and uses lib/ default" {
  # Create test Intent project
  local project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  # Create lib directory with files
  mkdir -p lib src
  touch lib/app.ex lib/server.ex src/other.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex"
  assert_success
  assert_output_contains "[ ] lib/app.ex"
  assert_output_contains "[ ] lib/server.ex"
  refute_output_contains "src/other.ex"
}

@test "fileindex: uses .intent/indexes/ for index files in Intent project" {
  local project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  mkdir -p lib
  touch lib/app.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i myproject.index
  assert_success
  assert_file_exists ".intent/indexes/myproject.index"
  assert_file_contains ".intent/indexes/myproject.index" '"context": "intent_project"'
}

@test "fileindex: --no-intent flag disables Intent integration" {
  local project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  touch file1.ex
  mkdir -p lib
  touch lib/app.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" --no-intent
  assert_success
  assert_output_contains "[ ] ./file1.ex"
  refute_output_contains "lib/app.ex"
}

@test "fileindex: --intent-dir specifies Intent project" {
  # Create Intent project in different location
  local project_dir=$(create_test_project "Test Project" "$TEST_TEMP_DIR/other-project")
  
  # Run from outside the project
  mkdir -p "$TEST_TEMP_DIR/work"
  cd "$TEST_TEMP_DIR/work"
  
  mkdir -p "$project_dir/lib"
  touch "$project_dir/lib/app.ex"
  
  # Need to cd to project directory since script looks for files relative to pwd
  cd "$project_dir"
  
  run "${INTENT_BIN_DIR}/intent_fileindex" --intent-dir "$project_dir"
  assert_success
  assert_output_contains "[ ] lib/app.ex"
}

@test "fileindex: --index-dir overrides default index directory" {
  touch file1.ex
  mkdir -p indexes
  
  run "${INTENT_BIN_DIR}/intent_fileindex" --index-dir indexes -i test.index
  assert_success
  assert_file_exists "indexes/test.index"
}

# Test error conditions
@test "fileindex: handles non-existent directory" {
  run "${INTENT_BIN_DIR}/intent_fileindex" nonexistent
  assert_failure
  assert_output_contains "Error: Directory 'nonexistent' does not exist"
}

@test "fileindex: handles empty directory" {
  mkdir empty
  
  run "${INTENT_BIN_DIR}/intent_fileindex" empty
  assert_success
  # Should complete without error but with no output
}

# Test complex scenarios
@test "fileindex: handles files with spaces in names" {
  touch "file with spaces.ex"
  
  run "${INTENT_BIN_DIR}/intent_fileindex"
  assert_success
  assert_output_contains '[ ] ./file with spaces.ex'
}

@test "fileindex: sorts files consistently" {
  touch z.ex a.ex m.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex"
  assert_success
  
  # Extract just the filenames and check order
  echo "$output" | grep -E "^\[.\]" > actual_order.txt
  assert_file_contains actual_order.txt "[ ] ./a.ex"
  assert_file_contains actual_order.txt "[ ] ./m.ex"  
  assert_file_contains actual_order.txt "[ ] ./z.ex"
  
  # Verify a.ex comes before z.ex in the output
  local a_line=$(grep -n "a.ex" actual_order.txt | cut -d: -f1)
  local z_line=$(grep -n "z.ex" actual_order.txt | cut -d: -f1)
  [ "$a_line" -lt "$z_line" ] || fail "Files not sorted correctly"
}

@test "fileindex: handles mixed file extensions correctly" {
  touch app.ex app.exs config.json
  
  run "${INTENT_BIN_DIR}/intent_fileindex" . "*.{ex,exs}"
  assert_success
  assert_output_contains "[ ] ./app.ex"
  assert_output_contains "[ ] ./app.exs"
  refute_output_contains "config.json"
}

# Test integration with intent command
@test "fileindex: works through intent command" {
  touch test.ex
  
  run run_intent fileindex
  assert_success
  assert_output_contains "[ ] ./test.ex"
}

@test "fileindex: help available through intent help" {
  run run_intent help fileindex
  assert_success
  assert_output_contains "Create and manage file indexes with checkbox states"
  assert_output_contains "@usage:"
  assert_output_contains "intent fileindex [OPTIONS]"
}

# Toggle functionality tests
@test "fileindex: toggle file state from unchecked to checked" {
  touch file1.ex file2.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Toggle file1.ex
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -X ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  
  # Verify state persisted
  assert_file_contains test.index "[x] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: toggle file state from checked to unchecked" {
  touch file1.ex
  
  # Create index and manually mark as checked
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  sed -i.bak 's/\[ \] \.\/file1\.ex/[x] .\/file1.ex/' test.index
  
  # Toggle back to unchecked
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -X ./file1.ex
  assert_success
  assert_output "[ ] ./file1.ex"
}

@test "fileindex: toggle with --toggle flag" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index --toggle ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
}

@test "fileindex: toggle requires index file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -X ./file1.ex
  assert_failure
  assert_output_contains "Error: Toggle mode requires an index file"
}

@test "fileindex: toggle with non-existent file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -X ./nonexistent.ex
  assert_failure
  assert_output_contains "Error: File './nonexistent.ex' not found in index"
}

@test "fileindex: toggle with non-existent index" {
  run "${INTENT_BIN_DIR}/intent_fileindex" -i nonexistent.index -X ./file.ex
  assert_failure
  assert_output_contains "Error: Index file"
  assert_output_contains "does not exist"
}

@test "fileindex: toggle preserves file order in index" {
  touch a.ex b.ex z.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Toggle middle file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -X ./b.ex
  assert_success
  
  # Check order is preserved
  grep -E "^\[.\]" test.index > actual_order.txt
  assert_file_contains actual_order.txt "[ ] ./a.ex"
  assert_file_contains actual_order.txt "[x] ./b.ex"
  assert_file_contains actual_order.txt "[ ] ./z.ex"
  
  # Verify order
  local line1=$(grep -n "a.ex" actual_order.txt | cut -d: -f1)
  local line2=$(grep -n "b.ex" actual_order.txt | cut -d: -f1)
  local line3=$(grep -n "z.ex" actual_order.txt | cut -d: -f1)
  [ "$line1" -lt "$line2" ] || fail "File order not preserved"
  [ "$line2" -lt "$line3" ] || fail "File order not preserved"
}

# Check functionality tests
@test "fileindex: check file from unchecked to checked" {
  touch file1.ex file2.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Check file1.ex
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  
  # Verify state persisted
  assert_file_contains test.index "[x] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: check already checked file remains checked" {
  touch file1.ex
  
  # Create index and manually mark as checked
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  sed -i.bak 's/\[ \] \.\/file1\.ex/[x] .\/file1.ex/' test.index
  
  # Check already checked file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  
  # Verify still checked
  assert_file_contains test.index "[x] ./file1.ex"
}

@test "fileindex: check with --check flag" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index --check ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
}

@test "fileindex: check requires index file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -C ./file1.ex
  assert_failure
  assert_output_contains "Error: Check mode requires an index file"
}

@test "fileindex: check with non-existent file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./nonexistent.ex
  assert_failure
  assert_output_contains "Error: File './nonexistent.ex' not found in index"
}

@test "fileindex: check with non-existent index" {
  run "${INTENT_BIN_DIR}/intent_fileindex" -i nonexistent.index -C ./file.ex
  assert_failure
  assert_output_contains "Error: Index file"
  assert_output_contains "does not exist"
}

# Uncheck functionality tests
@test "fileindex: uncheck file from checked to unchecked" {
  touch file1.ex file2.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Manually mark file1 as checked
  sed -i.bak 's/\[ \] \.\/file1\.ex/[x] .\/file1.ex/' test.index
  
  # Uncheck file1.ex
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -U ./file1.ex
  assert_success
  assert_output "[ ] ./file1.ex"
  
  # Verify state persisted
  assert_file_contains test.index "[ ] ./file1.ex"
  assert_file_contains test.index "[ ] ./file2.ex"
}

@test "fileindex: uncheck already unchecked file remains unchecked" {
  touch file1.ex
  
  # Create index (files are unchecked by default)
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Uncheck already unchecked file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -U ./file1.ex
  assert_success
  assert_output "[ ] ./file1.ex"
  
  # Verify still unchecked
  assert_file_contains test.index "[ ] ./file1.ex"
}

@test "fileindex: uncheck with --uncheck flag" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Mark as checked first
  sed -i.bak 's/\[ \] \.\/file1\.ex/[x] .\/file1.ex/' test.index
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index --uncheck ./file1.ex
  assert_success
  assert_output "[ ] ./file1.ex"
}

@test "fileindex: uncheck requires index file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -U ./file1.ex
  assert_failure
  assert_output_contains "Error: Uncheck mode requires an index file"
}

@test "fileindex: uncheck with non-existent file" {
  touch file1.ex
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -U ./nonexistent.ex
  assert_failure
  assert_output_contains "Error: File './nonexistent.ex' not found in index"
}

@test "fileindex: uncheck with non-existent index" {
  run "${INTENT_BIN_DIR}/intent_fileindex" -i nonexistent.index -U ./file.ex
  assert_failure
  assert_output_contains "Error: Index file"
  assert_output_contains "does not exist"
}

# Combined tests
@test "fileindex: check and uncheck preserve file order" {
  touch a.ex b.ex c.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  
  # Check middle file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./b.ex
  assert_success
  
  # Check first file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./a.ex
  assert_success
  
  # Uncheck middle file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -U ./b.ex
  assert_success
  
  # Check order is preserved
  grep -E "^\[.\]" test.index > actual_order.txt
  assert_file_contains actual_order.txt "[x] ./a.ex"
  assert_file_contains actual_order.txt "[ ] ./b.ex"
  assert_file_contains actual_order.txt "[ ] ./c.ex"
  
  # Verify order
  local line1=$(grep -n "a.ex" actual_order.txt | cut -d: -f1)
  local line2=$(grep -n "b.ex" actual_order.txt | cut -d: -f1)
  local line3=$(grep -n "c.ex" actual_order.txt | cut -d: -f1)
  [ "$line1" -lt "$line2" ] || fail "File order not preserved"
  [ "$line2" -lt "$line3" ] || fail "File order not preserved"
}

@test "fileindex: sequential check, uncheck, and toggle operations" {
  touch file1.ex
  
  # Create index
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index
  assert_success
  assert_file_contains test.index "[ ] ./file1.ex"
  
  # Check the file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  assert_file_contains test.index "[x] ./file1.ex"
  
  # Uncheck the file
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -U ./file1.ex
  assert_success
  assert_output "[ ] ./file1.ex"
  assert_file_contains test.index "[ ] ./file1.ex"
  
  # Toggle the file (should become checked)
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -X ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  assert_file_contains test.index "[x] ./file1.ex"
  
  # Check the file (should remain checked)
  run "${INTENT_BIN_DIR}/intent_fileindex" -i test.index -C ./file1.ex
  assert_success
  assert_output "[x] ./file1.ex"
  assert_file_contains test.index "[x] ./file1.ex"
}