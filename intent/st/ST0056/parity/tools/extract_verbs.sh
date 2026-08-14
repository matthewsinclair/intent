#!/bin/bash
# extract_verbs.sh <script> -- candidate subcommands from a script's MAIN dispatch.
#
# Finds the first `case` whose scrutinee names a command-ish variable, then takes
# its arms until the matching `esac`, tracking nesting so inner `case` blocks are
# not harvested as verbs.
#
# A flat grep cannot do this. Arm indentation is inconsistent across bin/
# (intent_todo:384 sits at column 0, intent_st:295 is nested), so an
# indentation-anchored pattern silently undercounts -- it scored the 1621-line
# intent_st at zero subcommands. Nested `case` blocks on $1, $status and $FILE_TYPE
# would otherwise contribute their arms as if they were verbs.
#
# Output is CANDIDATES ONLY. Each one is verified by running it; the dispatch is
# evidence of intent, the runtime is evidence of behaviour.

set -u
f="$1"

awk '
  # Arm at nesting depth 1 -- a real verb. Handles alternation (red|green|na).
  function emit(line) {
    sub(/^[[:space:]]*/, "", line)
    sub(/\).*$/, "", line)
    gsub(/"/, "", line)
    gsub(/'"'"'/, "", line)
    if (line == "*" || line == "") return
    n = split(line, parts, "|")
    for (i = 1; i <= n; i++) if (parts[i] != "*" && parts[i] != "") print parts[i]
  }

  depth == 0 {
    # Look for the main dispatch: case "$...COMMAND/NOUN/VERB/ACTION/CMD..."
    if ($0 ~ /^[[:space:]]*case[[:space:]]/ && $0 ~ /\$/) {
      v = $0
      if (v ~ /COMMAND/ || v ~ /NOUN/ || v ~ /VERB/ || v ~ /ACTION/ || v ~ /CMD/) {
        depth = 1
        next
      }
    }
    next
  }

  depth > 0 {
    if ($0 ~ /^[[:space:]]*case[[:space:]]/) { depth++; next }
    if ($0 ~ /^[[:space:]]*esac/)            { depth--; if (depth == 0) exit; next }
    if (depth == 1 && $0 ~ /^[[:space:]]*[a-zA-Z0-9_|"'"'"'-]+\)/) emit($0)
  }
' "$f"
