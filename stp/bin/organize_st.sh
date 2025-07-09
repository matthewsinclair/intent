#!/bin/bash
# Script to organize steel thread directories by status
# Updated for v1.2.1 directory structure

# Create required directories
mkdir -p stp/prj/st/COMPLETED stp/prj/st/NOT-STARTED stp/prj/st/CANCELLED

# Move files based on their status
echo "Organizing files based on status..."

# Find all ST directories
for dir in stp/prj/st/ST*/; do
  if [ -d "$dir" ]; then
    # Extract ID from directory name
    id=$(basename "$dir")
    
    # Look for status in info.md file
    info_file="${dir}info.md"
    if [ ! -f "$info_file" ]; then
      echo "Warning: $id has no info.md file"
      continue
    fi
    
    # Check both YAML frontmatter and document body for status
    yaml_status=$(grep -m 1 "^status:" "$info_file" | sed "s/^status: *//")
    body_status=$(grep -m 1 "^\- \*\*Status\*\*:" "$info_file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    
    # Prioritize YAML frontmatter status
    if [ -n "$yaml_status" ]; then
      status="$yaml_status"
    elif [ -n "$body_status" ]; then
      status="$body_status"
    else
      status="Not Started"
    fi
    
    echo "Directory: $id - Status: $status"
    
    # Move directory to appropriate location
    case "$status" in
      "Completed")
        echo "Moving $id to COMPLETED"
        mv "$dir" "stp/prj/st/COMPLETED/$id"
        ;;
      "Not Started")
        echo "Moving $id to NOT-STARTED"
        mv "$dir" "stp/prj/st/NOT-STARTED/$id"
        ;;
      "Cancelled")
        echo "Moving $id to CANCELLED"
        mv "$dir" "stp/prj/st/CANCELLED/$id"
        ;;
      *)
        # In Progress or On Hold stay in the main directory
        echo "$id stays in main directory"
        ;;
    esac
  fi
done

# Also check subdirectories to make sure directories are in the right place
for subdir in stp/prj/st/COMPLETED/ stp/prj/st/NOT-STARTED/ stp/prj/st/CANCELLED/; do
  if [ ! -d "$subdir" ]; then
    continue
  fi
  
  subdir_name=$(basename "$subdir")
  
  # Find all ST directories in this subdirectory
  for dir in "$subdir"ST*/; do
    if [ -d "$dir" ]; then
      # Extract ID from directory name
      id=$(basename "$dir")
      
      # Look for status in info.md file
      info_file="${dir}info.md"
      if [ ! -f "$info_file" ]; then
        echo "Warning: $id has no info.md file"
        continue
      fi
      
      # Check both YAML frontmatter and document body for status
      yaml_status=$(grep -m 1 "^status:" "$info_file" | sed "s/^status: *//")
      body_status=$(grep -m 1 "^\- \*\*Status\*\*:" "$info_file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
      
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
      
      # Move the directory if it's in the wrong location
      if [ "$subdir_name" == "COMPLETED" ] && [ "$status" != "Completed" ]; then
        echo "Moving $id from COMPLETED to $target_dir"
        mv "$dir" "$target_dir/$id"
      elif [ "$subdir_name" == "NOT-STARTED" ] && [ "$status" != "Not Started" ]; then
        echo "Moving $id from NOT-STARTED to $target_dir"
        mv "$dir" "$target_dir/$id"
      elif [ "$subdir_name" == "CANCELLED" ] && [ "$status" != "Cancelled" ]; then
        echo "Moving $id from CANCELLED to $target_dir"
        mv "$dir" "$target_dir/$id"
      fi
    fi
  done
done

echo "Organization complete!"
