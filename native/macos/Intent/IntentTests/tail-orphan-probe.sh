#!/bin/bash
# ST0064 AC-01.4 -- does a pipeline survive its RUNTIME's death, per signal?
#
# THE TOPOLOGY IS THE POINT AND IT IS THREE LEVELS, matching the real one:
#
#   RUNTIME  (stands in for the menubar app)
#     -> WRAPPER  (the log verb's shell)
#          -> TAIL  (the pipeline that must not outlive the app)
#
# A two-level probe cannot express the remedy at all, because the remedy is
# about what the WRAPPER does when the RUNTIME dies.
#
# ARMS, and the second one exists so the first one means something:
#
#   guarded  the wrapper reads its own stdin and the runtime holds the write
#            end. The runtime's death closes it HOWEVER IT DIES -- SIGKILL
#            included, which no handler can intercept -- the read returns EOF,
#            and the wrapper takes its own process group down. This is the
#            remedy ruled for `0281` on 2026-09-09 (option (i)).
#   plain    the runtime spawns the tail directly. This MUST leak. It is the
#            control: a probe whose arms cannot disagree has not been shown to
#            measure anything, and "no tail running" is also what you get from
#            a probe that never started one.
#
# THE PRECONDITION IS ASSERTED BEFORE ANY RESULT IS TRUSTED, and it is the
# constraint `0281`'s ruling carries: the runtime must be its own process-group
# leader. If it is not, `kill -TERM 0` in the wrapper names somebody else's
# group -- so the probe REFUSES rather than reporting a clean run it cannot
# justify. That refusal fired on this probe's first execution and was a real
# bug in it ($$ inside a subshell reports the PARENT's pid; $BASHPID is the
# subshell's own), which is the argument for the assertion being here at all.
set -u
ARM="${1:?arm: guarded|plain}"; SIG="${2:?signal: TERM|INT|KILL}"; DIR="${3:?state dir}"
rm -f "$DIR"/tail.pid "$DIR"/rt.pid "$DIR"/rt.pgid

set -m   # job control -- the runtime subshell becomes its own group leader
if [ "$ARM" = "guarded" ]; then
  (
    echo $BASHPID > "$DIR/rt.pid"
    ps -o pgid= -p $BASHPID | tr -d ' ' > "$DIR/rt.pgid"
    exec 3> >(
      tail -f /dev/null &
      echo $! > "$DIR/tail.pid"
      while read -r _; do :; done
      kill -TERM 0 2>/dev/null
    )
    sleep 120
  ) &
else
  (
    echo $BASHPID > "$DIR/rt.pid"
    ps -o pgid= -p $BASHPID | tr -d ' ' > "$DIR/rt.pgid"
    tail -f /dev/null &
    echo $! > "$DIR/tail.pid"
    sleep 120
  ) &
fi
set +m

for _ in $(seq 1 25); do
  [ -s "$DIR/tail.pid" ] && [ -s "$DIR/rt.pid" ] && [ -s "$DIR/rt.pgid" ] && break
  sleep 0.2
done
[ -s "$DIR/tail.pid" ] || { echo "probe-error: no tail pid -- the arm never started"; exit 2; }
RT=$(cat "$DIR/rt.pid"); TL=$(cat "$DIR/tail.pid"); PG=$(cat "$DIR/rt.pgid")

[ "$RT" = "$PG" ] || {
  echo "probe-refuse: runtime pid=$RT pgid=$PG -- not a group leader, so a group kill names someone else"
  kill -9 "$RT" "$TL" 2>/dev/null; exit 2; }
kill -0 "$TL" 2>/dev/null || { echo "probe-error: tail not alive before the signal"; exit 2; }

kill "-$SIG" "$RT" 2>/dev/null
for _ in $(seq 1 15); do kill -0 "$TL" 2>/dev/null || break; sleep 0.4; done

if kill -0 "$TL" 2>/dev/null; then echo "LEAKED"; else echo "clean"; fi
kill -9 "$TL" "$RT" 2>/dev/null
exit 0
