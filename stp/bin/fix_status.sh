#\!/bin/bash

# Function to modify the update_file_frontmatter function in stp_upgrade
modify_upgrade_script() {
  local file="stp/bin/stp_upgrade"
  local temp_file="${file}.tmp"
  
  if [ -f "$file" ]; then
    # Create a backup
    cp "$file" "${file}.bak"
    
    # Modify the script
    awk '
      /function update_file_frontmatter/,/^}/ {
        if ($0 ~ /echo "status: \\$status" >> "\\$temp_file"/) {
          print "      # Check if there is already a status in the YAML frontmatter";
          print "      original_status=$(grep -m 1 \"^status:\" \"$file\" | sed \"s/^status: *//\")";
          print "      if [ -n \"$original_status\" ]; then";
          print "        echo \"status: $original_status\" >> \"$temp_file\"";
          print "      else";
          print "        echo \"status: $status\" >> \"$temp_file\"";
          print "      fi";
          next;
        }
      }
      { print; }
    ' "$file" > "$temp_file"
    
    # Check if the modification worked
    if grep -q "original_status" "$temp_file"; then
      mv "$temp_file" "$file"
      chmod +x "$file"
      echo "Successfully modified stp_upgrade script to preserve status"
    else
      echo "Failed to modify stp_upgrade script"
      rm "$temp_file"
    fi
  else
    echo "Error: stp_upgrade script not found"
  fi
}

# Function to update the organize_st.sh script to better check file status
update_organize_script() {
  local file="stp/bin/organize_st.sh"
  
  # Add code to check both status formats (YAML frontmatter and document body)
  sed -i.bak 's/status=$(grep -m 1 "^\\\- \\\*\\\*Status\\\*\\\*:" "$file" | sed "s\/^\\\- \\\*\\\*Status\\\*\\\*: \/\/" | sed '"'"'s\/^[[:space:]]*\/\/;s\/[[:space:]]*$\/\/'"'"')/# Try document body status first\
    status=$(grep -m 1 "^\\\- \\\*\\\*Status\\\*\\\*:" "$file" | sed "s\/^\\\- \\\*\\\*Status\\\*\\\*: \/\/" | sed '"'"'s\/^[[:space:]]*\/\/;s\/[[:space:]]*$\/\/'"'"')\
    \
    # If empty or not found, try YAML frontmatter\
    if [ -z "$status" ]; then\
      status=$(grep -m 1 "^status:" "$file" | sed "s\/^status: *\/\/")\
    fi/g' "$file"
  
  # Check if the modification worked
  if grep -q "# Try document body status first" "$file"; then
    echo "Successfully updated organize_st.sh script"
  else
    echo "Failed to update organize_st.sh script"
    mv "${file}.bak" "$file"
  fi
}

# Move ST0013 back to the right location
move_st0013() {
  if [ -f "stp/prj/st/NOT-STARTED/ST0013.md" ]; then
    # Update status in the file
    sed -i.bak 's/status: Not Started/status: In Progress/' "stp/prj/st/NOT-STARTED/ST0013.md"
    sed -i.bak 's/\- \*\*Status\*\*: Not Started/\- \*\*Status\*\*: In Progress/' "stp/prj/st/NOT-STARTED/ST0013.md"
    
    # Move the file back to the main directory
    mv "stp/prj/st/NOT-STARTED/ST0013.md" "stp/prj/st/ST0013.md"
    echo "Moved ST0013 back to main directory"
  elif [ -f "stp/prj/st/ST0013.md" ]; then
    echo "ST0013 is already in the correct location"
  else
    echo "ST0013 file not found"
  fi
}

# Update the steel_threads.md index file
update_index() {
  local file="stp/prj/st/steel_threads.md"
  local temp_file="${file}.tmp"
  
  sed -i.bak 's/\[ST0013\](\.\/NOT-STARTED\/ST0013)/\[ST0013\](\.\/ST0013)/' "$file" 2>/dev/null || true
  echo "Updated index file references"
}

# Main execution
echo "Fixing ST0013 status and location..."
move_st0013
update_index
#modify_upgrade_script
#update_organize_script
echo "Fixes completed\!"
