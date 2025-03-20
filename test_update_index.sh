#\!/bin/bash

# Update the steel threads index file
echo "Updating steel threads index..."

INDEX_FILE="stp/prj/st/steel_threads.md"
TMP_FILE=$(mktemp)

# Create a copy of the original file
cp "$INDEX_FILE" "$TMP_FILE"

# Generate index content
ST_INDEX_CONTENT=$(
  # Header
  echo "| ID | Title | Status | Created | Completed |"
  echo "|----|-------|--------|---------|-----------|"
  
  # Find all ST files in all directories
  for file in $(find "stp/prj/st" -name "ST*.md" | sort -r); do
    if [ -f "$file" ]; then
      # Extract ID from filename
      ID=$(basename "$file" .md)
      
      # Create relative link for the file
      DIR_PATH=$(dirname "$file")
      RELATIVE_PATH="${DIR_PATH#stp/prj/st/}"
      
      if [ "$RELATIVE_PATH" = "$DIR_PATH" ] || [ -z "$RELATIVE_PATH" ]; then
        # Files in the same directory
        ID_LINK="[$ID](./$ID.md)"
      else
        # Files in subdirectories
        ID_LINK="[$ID](./$RELATIVE_PATH/$ID.md)"
      fi
      
      # Extract metadata from file
      TITLE=$(grep "^# $ID:" "$file" | sed "s/^# $ID: //")
      STATUS=$(grep -m 1 "^\- \*\*Status\*\*:" "$file" | sed "s/^\- \*\*Status\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
      CREATED=$(grep -m 1 "^\- \*\*Created\*\*:" "$file" | sed "s/^\- \*\*Created\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
      COMPLETED=$(grep -m 1 "^\- \*\*Completed\*\*:" "$file" | sed "s/^\- \*\*Completed\*\*: //" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
      
      echo "| $ID_LINK | $TITLE | $STATUS | $CREATED | $COMPLETED |"
    fi
  done
)

# Create a temporary section file
SECTION_FILE=$(mktemp)
{
  echo "<\!-- BEGIN: STEEL_THREAD_INDEX -->"
  echo "$ST_INDEX_CONTENT"
  echo "<\!-- END: STEEL_THREAD_INDEX -->"
} > "$SECTION_FILE"

# Replace content between markers
awk '
  BEGIN { replacing = 0; }
  /<\!-- BEGIN: STEEL_THREAD_INDEX -->/ { replacing = 1; system("cat '"$SECTION_FILE"'"); next; }
  /<\!-- END: STEEL_THREAD_INDEX -->/ { replacing = 0; next; }
  \!replacing { print; }
' "$INDEX_FILE" > "$TMP_FILE"

# Update the file
mv "$TMP_FILE" "$INDEX_FILE"
rm "$SECTION_FILE"

echo "Index updated successfully\!"
