#\!/bin/bash
# Script to organize steel thread files by status

# Create required directories
mkdir -p stp/prj/st/COMPLETED stp/prj/st/NOT-STARTED stp/prj/st/CANCELLED

# Move files based on their status
echo "Organizing files based on status..."

# Find all ST files
for file in stp/prj/st/ST*.md; do
  if [ -f "$file" ]; then
    # Extract ID from filename
    id=$(basename "$file" .md)
    
    # Check both YAML frontmatter and document body for status
    yaml_status=$(grep -m 1 "^status:" "$file" | sed "s/^status: *//")
    body_status=$(grep -m 1 "^\- \*\*Status\*\*:" "$file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    
    # Prioritize YAML frontmatter status
    if [ -n "$yaml_status" ]; then
      status="$yaml_status"
    elif [ -n "$body_status" ]; then
      status="$body_status"
    else
      status="Not Started"
    fi
    
    echo "File: $id - Status: $status"
    
    # Move file to appropriate directory
    case "$status" in
      "Completed")
        echo "Moving $id to COMPLETED"
        mv "$file" "stp/prj/st/COMPLETED/$id.md"
        ;;
      "Not Started")
        echo "Moving $id to NOT-STARTED"
        mv "$file" "stp/prj/st/NOT-STARTED/$id.md"
        ;;
      "Cancelled")
        echo "Moving $id to CANCELLED"
        mv "$file" "stp/prj/st/CANCELLED/$id.md"
        ;;
      *)
        # In Progress or On Hold stay in the main directory
        echo "$id stays in main directory"
        ;;
    esac
  fi
done

# Also check subdirectories to make sure files are in the right place
for subdir in stp/prj/st/*/; do
  subdir_name=$(basename "$subdir")
  if [ "$subdir_name" \!= "COMPLETED" ] && [ "$subdir_name" \!= "NOT-STARTED" ] && [ "$subdir_name" \!= "CANCELLED" ]; then
    continue
  fi
  
  # Find all ST files in this subdirectory
  for file in "$subdir"ST*.md; do
    if [ -f "$file" ]; then
      # Extract ID from filename
      id=$(basename "$file" .md)
      
      # Check both YAML frontmatter and document body for status
      yaml_status=$(grep -m 1 "^status:" "$file" | sed "s/^status: *//")
      body_status=$(grep -m 1 "^\- \*\*Status\*\*:" "$file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
      
      # Prioritize YAML frontmatter status
      if [ -n "$yaml_status" ]; then
        status="$yaml_status"
      elif [ -n "$body_status" ]; then
        status="$body_status"
      else
        status="Not Started"
      fi
      
      # Determine the correct directory
      target_dir="stp/prj/st"
      case "$status" in
        "Completed")
          target_dir="stp/prj/st/COMPLETED"
          ;;
        "Not Started")
          target_dir="stp/prj/st/NOT-STARTED"
          ;;
        "Cancelled")
          target_dir="stp/prj/st/CANCELLED"
          ;;
        *)
          target_dir="stp/prj/st"
          ;;
      esac
      
      # Move the file if it's in the wrong directory
      if [ "$subdir_name" == "COMPLETED" ] && [ "$status" \!= "Completed" ]; then
        echo "Moving $id from COMPLETED to $target_dir"
        mv "$file" "$target_dir/$id.md"
      elif [ "$subdir_name" == "NOT-STARTED" ] && [ "$status" \!= "Not Started" ]; then
        echo "Moving $id from NOT-STARTED to $target_dir"
        mv "$file" "$target_dir/$id.md"
      elif [ "$subdir_name" == "CANCELLED" ] && [ "$status" \!= "Cancelled" ]; then
        echo "Moving $id from CANCELLED to $target_dir"
        mv "$file" "$target_dir/$id.md"
      fi
    fi
  done
done

echo "Organization complete\!"
