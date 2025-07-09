#!/bin/bash
# update_frontmatter.sh - Updates frontmatter in STP files
# This script ensures all STP files include the stp_version field in YAML frontmatter
# Updated for v1.2.1 directory structure

# Exit on error
set -e

# Current STP version
CURRENT_VERSION="1.2.1"

# Function to display error messages
error() {
  echo "Error: $1" >&2
  exit 1
}

# Function to add/update YAML frontmatter in a file
update_file_frontmatter() {
  local file="$1"
  local temp_file="${file}.tmp"
  
  if [ -f "$file" ]; then
    echo "Updating $file"
    
    # Check if file already has YAML frontmatter
    if grep -q "^---" "$file"; then
      # Update existing frontmatter
      awk '
        BEGIN { in_frontmatter = 0; has_version = 0; printed_version = 0; }
        /^---/ {
          if (in_frontmatter == 0) {
            in_frontmatter = 1;
            print "---";
            next;
          } else {
            in_frontmatter = 0;
            if (!has_version) {
              print "stp_version: '"$CURRENT_VERSION"'";
              printed_version = 1;
            }
            print "---";
            next;
          }
        }
        in_frontmatter && /^stp_version:/ {
          print "stp_version: '"$CURRENT_VERSION"'";
          has_version = 1;
          printed_version = 1;
          next;
        }
        { print; }
      ' "$file" > "$temp_file"
    else
      # Add new frontmatter
      echo "---" > "$temp_file"
      echo "stp_version: $CURRENT_VERSION" >> "$temp_file"
      
      # Try to extract author information from the file
      local author=$(grep -m 1 "^\- \*\*Author\*\*:" "$file" | sed "s/^\- \*\*Author\*\*: //")
      if [ -z "$author" ]; then
        author="STP System"
      fi
      
      # Add verblock if not present
      echo "verblock: \"$(date '+%d %b %Y'):v0.1: $author - Added metadata\"" >> "$temp_file"
      echo "---" >> "$temp_file"
      cat "$file" >> "$temp_file"
    fi
    
    # Replace the original file
    mv "$temp_file" "$file"
  fi
}

# Scan for files to update
echo "Starting frontmatter update process..."
echo "Current STP version: $CURRENT_VERSION"
echo ""

# Update files in stp/usr/
echo "Updating user documentation..."
for file in stp/usr/*.md; do
  if [ -f "$file" ]; then
    update_file_frontmatter "$file"
  fi
done

# Update files in stp/eng/
echo "Updating engineering documentation..."
for file in stp/eng/tpd/*.md; do
  if [ -f "$file" ]; then
    update_file_frontmatter "$file"
  fi
done

# Update files in stp/llm/
echo "Updating LLM documentation..."
for file in stp/llm/*.md; do
  if [ -f "$file" ]; then
    update_file_frontmatter "$file"
  fi
done

# Update files in stp/prj/
echo "Updating project documentation..."
for file in stp/prj/*.md; do
  if [ -f "$file" ]; then
    update_file_frontmatter "$file"
  fi
done

# Update steel threads (now in directories)
echo "Updating steel threads..."
# Check main directory
for dir in stp/prj/st/ST*/; do
  if [ -d "$dir" ]; then
    # Update all .md files in the directory
    for file in "$dir"*.md; do
      if [ -f "$file" ]; then
        update_file_frontmatter "$file"
      fi
    done
  fi
done

# Also check status subdirectories
for status_dir in stp/prj/st/COMPLETED/ stp/prj/st/NOT-STARTED/ stp/prj/st/CANCELLED/; do
  if [ -d "$status_dir" ]; then
    for dir in "$status_dir"ST*/; do
      if [ -d "$dir" ]; then
        for file in "$dir"*.md; do
          if [ -f "$file" ]; then
            update_file_frontmatter "$file"
          fi
        done
      fi
    done
  fi
done

# Update steel_threads.md separately
update_file_frontmatter "stp/prj/st/steel_threads.md"

# Update steel threads index
if [ -x "./stp/bin/stp_st" ]; then
  echo ""
  echo "Running sync to update steel_threads.md..."
  ./stp/bin/stp_st sync --write
fi

echo ""
echo "Frontmatter update complete."