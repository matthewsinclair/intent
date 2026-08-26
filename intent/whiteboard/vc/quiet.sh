#!/usr/bin/env bash
# quiet.sh -- is native/rust quiet for a shared build? TWO reads at the moment; exit 0 only when both are empty.
#   read 1: dirt under native/rust (git status --porcelain)
#   read 2: a live `cargo` EXECUTABLE (pgrep -x on the process name) -- never `pgrep -f` on arguments: the peer Claude
#           sessions' argv carries the restart prompt, which mentions cargo, so an argument match reports them forever
#           (measured 2026-08-26: "91 cargo processes" were three claude sessions and my own shell's echo lines).
#   a process list is a statement about the past the instant it returns; this narrows the race, it cannot close it.
cd ~/Devel/prj/Intent || exit 2
dirt=$(git status --porcelain -- native/rust); procs=$(pgrep -lx cargo)
printf 'dirt under native/rust: %s path(s)\n' "$(printf '%s' "$dirt" | grep -c .)"; [ -n "$dirt" ] && printf '%s\n' "$dirt" | sed 's/^/   /'
printf 'live cargo executables: %s\n' "$(printf '%s' "$procs" | grep -c .)"; [ -n "$procs" ] && printf '%s\n' "$procs" | sed 's/^/   /'
[ -z "$dirt" ] && [ -z "$procs" ] && { echo "QUIET at $(date -u +'%H:%M:%SZ')"; exit 0; } || { echo "NOT quiet at $(date -u +'%H:%M:%SZ')"; exit 1; }
