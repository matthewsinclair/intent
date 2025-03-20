#\!/bin/bash

# Create required directories
mkdir -p stp/prj/st/COMPLETED stp/prj/st/NOT-STARTED stp/prj/st/CANCELLED

# Move files based on their status
echo "Organizing files based on status..."

# Find all ST files
for file in stp/prj/st/ST*.md; do
  if [ -f "$file" ]; then
    # Extract ID and status
    id=$(basename "$file" .md)
    status=$(grep -m 1 "^\- \*\*Status\*\*:" "$file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
    
    # If empty or not found, try YAML frontmatter
    if [ -z "$status" ]; then
      status=$(grep -m 1 "^status:" "$file" | sed "s/^status: *//")
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

echo "Organization complete\!"
