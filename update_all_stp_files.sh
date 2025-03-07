#!/bin/bash
# One-time script to update all steel thread files with proper YAML frontmatter

for file in stp/prj/st/ST*.md; do
  if [ "$file" != "stp/prj/st/steel_threads.md" ]; then
    echo "Processing $file..."
    
    # Extract key information
    STATUS=$(grep -m 1 "^\- \*\*Status\*\*:" "$file" | sed "s/^\- \*\*Status\*\*: //")
    CREATED=$(grep -m 1 "^\- \*\*Created\*\*:" "$file" | sed "s/^\- \*\*Created\*\*: //")
    COMPLETED=$(grep -m 1 "^\- \*\*Completed\*\*:" "$file" | sed "s/^\- \*\*Completed\*\*: //")
    
    # Format dates for YAML frontmatter
    if [ -n "$CREATED" ] && [[ "$CREATED" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
      CREATED_COMPACT=$(echo "$CREATED" | tr -d '-')
    else
      CREATED_COMPACT=""
    fi
    
    if [ -n "$COMPLETED" ] && [ "$COMPLETED" != " " ] && [[ "$COMPLETED" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
      COMPLETED_COMPACT=$(echo "$COMPLETED" | tr -d '-')
    else
      COMPLETED_COMPACT=""
    fi
    
    # Check if file already has YAML frontmatter
    if grep -q "^---" "$file"; then
      # Update existing frontmatter
      tmp_file=$(mktemp)
      awk -v status="$STATUS" -v created="$CREATED_COMPACT" -v completed="$COMPLETED_COMPACT" '
        BEGIN { in_frontmatter = 0; has_version = 0; has_status = 0; has_created = 0; has_completed = 0; }
        /^---/ {
          if (in_frontmatter == 0) {
            in_frontmatter = 1;
            print "---";
            next;
          } else {
            in_frontmatter = 0;
            if (!has_version) {
              print "stp_version: 1.0.0";
            }
            if (!has_status && status != "") {
              print "status: " status;
            }
            if (!has_created && created != "") {
              print "created: " created;
            }
            if (!has_completed && completed != "") {
              print "completed: " completed;
            }
            print "---";
            next;
          }
        }
        in_frontmatter && /^stp_version:/ {
          print "stp_version: 1.0.0";
          has_version = 1;
          next;
        }
        in_frontmatter && /^status:/ {
          if (status != "") {
            print "status: " status;
          } else {
            print $0;
          }
          has_status = 1;
          next;
        }
        in_frontmatter && /^created:/ {
          if (created != "") {
            print "created: " created;
          } else {
            print $0;
          }
          has_created = 1;
          next;
        }
        in_frontmatter && /^completed:/ {
          if (completed != "") {
            print "completed: " completed;
          } else {
            print $0;
          }
          has_completed = 1;
          next;
        }
        { print; }
      ' "$file" > "$tmp_file"
      
      mv "$tmp_file" "$file"
      echo "  Updated frontmatter"
    else
      echo "  Error: No frontmatter found in $file"
    fi
  fi
done

echo "All files processed."