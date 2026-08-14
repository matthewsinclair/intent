#!/bin/bash
# extract_flags.sh <script> -- long/short flags parsed by a script.
# Flags appear as case arms at ANY nesting depth (arg-parsing loops are nested
# inside verb arms), which is the opposite of extract_verbs.sh's depth-1 rule.
# Alternation arms (--write|-w) are split.
set -u
awk '
  /^[[:space:]]*-[-a-zA-Z0-9|_=-]*\)/ {
    line=$0
    sub(/^[[:space:]]*/,"",line); sub(/\).*$/,"",line); gsub(/"/,"",line)
    n=split(line,p,"|")
    for(i=1;i<=n;i++) if (p[i] ~ /^-/) print p[i]
  }
' "$1" | sort -u
