#!/usr/bin/env bash
# quiet.sh -- is native/rust quiet for a shared build? both reads, at the moment, exit 0 only when both are empty.
cd ~/Devel/prj/Intent || exit 2
dirt=$(git status --porcelain -- native/rust); procs=$(pgrep -fl 'cargo (build|test|clippy|fmt|clean|check|run|doc|bench)' | grep -v pgrep)
printf 'dirt under native/rust: %s path(s)\n' "$(printf '%s' "$dirt" | grep -c .)"; [ -n "$dirt" ] && printf '%s\n' "$dirt" | sed 's/^/   /'
printf 'cargo processes: %s\n' "$(printf '%s' "$procs" | grep -c .)"; [ -n "$procs" ] && printf '%s\n' "$procs" | cut -c1-120 | sed 's/^/   /'
[ -z "$dirt" ] && [ -z "$procs" ] && { echo "QUIET at $(date -u +'%H:%M:%SZ')"; exit 0; } || { echo "NOT quiet at $(date -u +'%H:%M:%SZ')"; exit 1; }
