#!/usr/bin/env bash
# exposure.sh -- count `satisfied: yes` forms in PRE-migration acceptance.md files.
# Usage: exposure.sh <project> <ref>    (ref = <migration-sha>^ for landed, HEAD for unmigrated)
# BASH ON PURPOSE: the Bash tool's shell is zsh, which does not word-split $var;
# an inline loop over a newline list ran once with a bogus path and printed zeros
# (fourth instance today). A script file runs under bash.
set -uo pipefail
P="$1"; REF="$2"; cd "$P" || exit 1
hand=0; comp=0; bare=0; files=0
while IFS= read -r f; do
  [ -n "$f" ] || continue
  files=$((files+1))
  c=$(git show "$REF:$f" 2>/dev/null) || continue
  h=$(printf '%s\n' "$c" | grep -E 'satisfied: yes \(' | grep -vcE 'satisfied: yes \(computed')
  k=$(printf '%s\n' "$c" | grep -cE 'satisfied: yes \(computed')
  b=$(printf '%s\n' "$c" | grep -cE 'satisfied: yes *$')
  hand=$((hand+h)); comp=$((comp+k)); bare=$((bare+b))
done < <(git ls-tree -r --name-only "$REF" -- intent/st 2>/dev/null | grep '/acceptance.md$')
printf '%-28s files=%-3s hand-parenthetical=%-4s (computed)=%-4s bare=%-4s\n' "$(basename "$P")" "$files" "$hand" "$comp" "$bare"
