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
#            and the wrapper takes its own process group down. Ruled for `0281`
#            on 2026-09-09, option (i).
#   plain    the runtime spawns the tail directly. This MUST leak. A probe
#            whose arms cannot disagree has not been shown to measure anything.
#
# THREE VERDICTS, NOT TWO, AND THE THIRD IS WHY (dc, 2026-09-09):
#
#   clean                 the tail is gone.
#   LEAKED                the tail is ALIVE and its original parent is GONE.
#   probe-indeterminate   the tail is alive and the runtime has not been reaped
#                         yet, so neither statement is available.
#
# **A TWO-VERDICT PROBE FAILS IN THE DIRECTION NOBODY CHECKS.** With only
# clean/LEAKED, a loaded machine that has not finished reaping the runtime
# reports the control arm as `clean` -- and a reader scanning results sees the
# leak as FIXED. That is the worst available misreading. The third verdict is
# unmisreadable by construction: nobody mistakes `probe-indeterminate` for a
# fix.
#
# **AND THE PREDICATE IS `ppid != RUNTIME`, NOT `ppid == 1` (dc's correction).**
# The property meant is *its original parent is gone*; `1` is what that looks
# like on this machine because launchd reaps, and a subreaper would satisfy the
# same property with a different pid. **The claim is asserted, the observation
# is reported.** Hardcoding the observation is a real measurement of an adjacent
# property -- the defect class that cost this estate an entire evening on
# 2026-09-09.
#
# **THE POLL BUDGETS ARE DELIBERATELY NOT SHARED**, because the arms fail in
# opposite directions. The guarded arm can false-FAIL on a loaded machine (the
# wrapper must be scheduled to notice EOF) -- loud and safe. The control arm
# can false-`clean` -- quiet and dangerous. The settle condition is per-arm and
# structural, so neither can report a verdict it has not observed.
#
# **THE RUNTIME MUST BE ITS OWN PROCESS-GROUP LEADER OR THE PROBE REFUSES.**
# That is `0281`'s ruling constraint: `kill -TERM 0` in the wrapper names
# another group otherwise. It fired on this probe's first execution and caught
# a real bug in the probe rather than in the subject ($$ inside a subshell
# reports the PARENT's pid; $BASHPID is the subshell's own).
# **THE INTERPRETER IS PART OF THE CONTRACT AND IT IS NOT THE ONE ON `PATH`
# (dc, 2026-09-09, found by running this file under `xcodebuild`).**
# `TailOrphanTests.swift` sets `executableURL = /bin/bash` explicitly, and on
# macOS `/bin/bash` is **3.2.57** while `bash` on PATH here is Homebrew's
# **5.3.15**. `BASHPID` is bash 4.0+, so the original `$BASHPID` in each arm was
# an UNBOUND VARIABLE under `set -u` at line 39 -- fatal before a single pid
# file was written, which surfaced as `probe-error: the arm never started` on
# all six cells while running perfectly for its author.
#
# **EVERY RESULT THIS PROBE PRODUCED BEFORE THAT FIX RAN ON AN INTERPRETER THE
# TEST NEVER USES.** The substitute below is POSIX and version-independent:
# `sh -c 'echo $PPID'` inside the subshell returns the SUBSHELL's own pid,
# driven under both 3.2.57 and 5.3.15. **Drive this file with `/bin/bash`, never
# with `bash`** -- they are different programs on this machine, and the house
# notes already carry the class (`no declare -A`, `no ${VAR^}`); `BASHPID`
# belongs on that list.
# **THE INT CELL IS CONSTRUCTIBLE HERE BY A CONJUNCTION OF THREE PROPERTIES,
# AND ANY ONE OF THEM CHANGING SILENTLY DISARMS IT (dc, 2026-09-09).**
#
#   1. `set -m` is on, so the runtime is its own process-group leader.
#   2. NO trap is installed in the runtime. A backgrounded subshell that
#      installs `trap ... INT` SURVIVES the signal -- driven, with and without
#      job control -- and the handler never fires. One with no trap DIES.
#   3. The runtime `exec`s into its final program rather than forking it, so it
#      takes that program's default disposition.
#
# **NONE OF THE THREE IS A PROPERTY OF THE WRAPPER, WHICH IS WHAT THIS PROBE
# EXISTS TO TEST.** They are all properties of the RUNTIME construction, so a
# later edit made for an unrelated reason -- dropping job control, reinstating
# a trap, replacing `exec sleep` with `sleep` -- takes the INT cell back to
# `probe-indeterminate` while every other cell keeps reporting normally.
# **That is the failure mode to watch for: not a red, but one cell quietly
# ceasing to answer.**
#
# **THIS IS WHY THE ARMS MUST BUILD THEIR RUNTIME IDENTICALLY.** They did not,
# once: `guarded` had `exec`'d and `plain` had not, so the control varied the
# remedy AND the signal disposition. **A control that varies more than the axis
# under test isolates nothing**, and it produces confident results indefinitely
# rather than failing. That defect was found only because dc asked about INT
# for an unrelated reason.
#
set -u
ARM="${1:?arm: guarded|plain|stubborn (self-test)}"; SIG="${2:?signal: TERM|INT|KILL}"; DIR="${3:?state dir}"
SETTLE_TRIES="${PROBE_SETTLE_TRIES:-40}"   # x 0.4s -- generous; both arms settle structurally
rm -f "$DIR"/tail.pid "$DIR"/rt.pid "$DIR"/rt.pgid "$DIR"/ctl
mkfifo "$DIR/ctl"

set -m   # job control -- the runtime subshell becomes its own group leader
if [ "$ARM" = "stubborn" ]; then
  # RIG SELF-TEST ARM, not a production arm. The runtime IGNORES the signal, so
  # the tail stays alive with its ORIGINAL parent -- the one state in which
  # neither `clean` nor `LEAKED` is available. Without this arm
  # `probe-indeterminate` is unreachable, and a verdict that cannot be produced
  # is decoration rather than a verdict. Driven: on this machine reparenting is
  # effectively instantaneous, so shrinking the poll budget does NOT reach the
  # third state -- only a runtime that survives its signal does.
  (
    trap '' TERM INT
    RT_SELF=$(sh -c 'echo $PPID')          # bash 3.2 has no BASHPID -- see header
    echo "$RT_SELF" > "$DIR/rt.pid"
    ps -o pgid= -p "$RT_SELF" | tr -d ' ' > "$DIR/rt.pgid"
    tail -f /dev/null &
    echo $! > "$DIR/tail.pid"
    sleep 120
  ) &
elif [ "$ARM" = "guarded" ]; then
  (
    RT_SELF=$(sh -c 'echo $PPID')          # bash 3.2 has no BASHPID -- see header
    echo "$RT_SELF" > "$DIR/rt.pid"
    ps -o pgid= -p "$RT_SELF" | tr -d ' ' > "$DIR/rt.pgid"
    # THE RUNTIME MUST BE THE SOLE HOLDER OF THE WRAPPER'S STDIN WRITE END,
    # AND NO DESCENDANT MAY INHERIT IT. Both halves are load-bearing and both
    # were found by driving this under /bin/bash 3.2.57 -- see the header.
    #   `3>&-`      the wrapper must not hold the write end itself, or its own
    #               read never sees EOF.
    #   `exec`      the runtime must not fork a child that inherits fd 3; an
    #               ordinary `sleep` keeps the pipe open after the runtime dies.
    # A FIFO rather than `>(...)`: bash 3.2 process substitution does not close
    # the write end on the runtime's death and the wrapper blocks forever.
    ( tail -f /dev/null &
      echo $! > "$DIR/tail.pid"
      while read -r _; do :; done < "$DIR/ctl"
      kill -TERM 0 2>/dev/null ) 3>&- &
    exec 3> "$DIR/ctl"
    exec sleep 120
  ) &
else
  (
    RT_SELF=$(sh -c 'echo $PPID')          # bash 3.2 has no BASHPID -- see header
    echo "$RT_SELF" > "$DIR/rt.pid"
    ps -o pgid= -p "$RT_SELF" | tr -d ' ' > "$DIR/rt.pgid"
    tail -f /dev/null &
    echo $! > "$DIR/tail.pid"
    # `exec` FOR THE SAME REASON THE GUARDED ARM HAS IT, AND THE REASON IS THE
    # CONTROL RATHER THAN THE MECHANISM. Without it this arm's runtime stays a
    # backgrounded bash subshell, which IGNORES SIGINT, while the guarded arm's
    # runtime has exec'd into `sleep`, which does not. The arms would then
    # differ in TWO ways -- the remedy and the runtime's signal disposition --
    # and a control that varies more than the axis under test isolates nothing.
    # Measured before this line existed: plain/INT returned probe-indeterminate
    # because the runtime simply never died.
    exec sleep 120
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

VERDICT="probe-indeterminate"; PPID_SEEN=""
for _ in $(seq 1 "$SETTLE_TRIES"); do
  if ! kill -0 "$TL" 2>/dev/null; then VERDICT="clean"; break; fi
  PPID_SEEN=$(ps -o ppid= -p "$TL" 2>/dev/null | tr -d ' ')
  # SETTLED only when the original parent is demonstrably gone -- not when a
  # timer expired. `1` is what that looks like here; the claim is `!= RT`.
  if [ -n "$PPID_SEEN" ] && [ "$PPID_SEEN" != "$RT" ]; then VERDICT="LEAKED"; break; fi
  sleep 0.4
done

# **THE VERDICT TOKEN IS THE LAST LINE AND CARRIES NOTHING ELSE; EVIDENCE GOES
# ON ITS OWN LINE ABOVE IT (dc, 2026-09-09, on the first harness run).**
# The evidence was originally appended to the token -- `LEAKED (tail 16960
# alive, ...)` -- and the caller's `XCTAssertEqual(verdict, "LEAKED")` then
# failed on all three control cells while the probe was behaving perfectly.
# **The producer gained information and the consumer still demanded the old
# exact form**, which is producer/consumer drift committed inside the
# instrument built to catch that class.
#
# **THE CHEAP FIX WAS `hasPrefix` IN THE ASSERTION AND IT IS THE WRONG ONE.** A
# prefix match keeps passing if the evidence text later becomes WRONG, because
# nothing reads it. A fixed token plus a separate evidence line lets a caller
# assert the verdict and the evidence INDEPENDENTLY -- the same discipline as a
# machine-readable column set that prose can grow beside without either
# breaking the other.
case "$VERDICT" in
  clean)  echo "clean" ;;
  LEAKED) echo "evidence: tail $TL alive, reparented to ppid=$PPID_SEEN, original parent $RT gone"
          echo "LEAKED" ;;
  *)      echo "evidence: tail $TL alive, ppid=$PPID_SEEN, runtime $RT not yet reaped -- neither verdict is available"
          echo "probe-indeterminate" ;;
esac
kill -9 "$TL" "$RT" 2>/dev/null
[ "$VERDICT" = "probe-indeterminate" ] && exit 3
exit 0
