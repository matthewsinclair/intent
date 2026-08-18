#!/bin/bash
# rig_stub_migrator.sh -- a migrator whose behaviour is CHOSEN, for driving interrupt_rig.sh.
#
# The rig's own header says an explicit `MIGRATE_CMD` exists so its machinery can
# be proven against a stub whose behaviour is known. This is that stub, and it is
# committed rather than improvised because the proving was done ONCE, by hand,
# before use -- and `MODULES.md` still asserts all three directions hold while one
# of them has been broken since `b96188d1`. A claim nothing re-runs is a claim.
#
# Every mode exists to make ONE arm of the rig go red. Run it through
# `rig_selftest.sh`, which predicts each outcome before it runs and scores it after.
#
#   STUB_MODE=pass         deterministic content -- the re-run reaches the clean end state
#   STUB_MODE=diverge      per-run nonce in every file -- the two arms cannot agree
#   STUB_MODE=nosentinel   arm B stops after 10% -- the kill sentinel never appears
#   STUB_MODE=nowrite      writes nothing at all -- there is no delta to kill inside
#   STUB_MODE=escape       re-execs into a NEW SESSION, so a group kill cannot reach it
#
#   STUB_N       files to write (default 40)
#   STUB_DELAY   seconds between writes (default 0.02) -- wide enough for a kill to land
#
# WHICH ARM IT IS IN IS READ FROM THE DIRECTORY NAME, because the rig names them:
# `a-clean` and `b-interrupted`. A stub that cannot tell the arms apart cannot
# produce a divergence that only affects the interrupted one, and that divergence
# is the whole subject of two of the five cases.
set -uo pipefail

# THE STUB PLAYS ALL THREE OF THE RIG'S PARAMETERISED COMMANDS, because they are
# one concern -- "a command the rig runs, whose behaviour is chosen" -- and three
# files would be three places to look for the same idea. The role is the first
# argument; without one it is the migrator.
#
#   rig_stub_migrator.sh          the migration       STUB_MODE
#   rig_stub_migrator.sh read     the liveness probe  STUB_READ_MODE   ok|dead
#   rig_stub_migrator.sh store    the store read      STUB_STORE_MODE  see below
#
# STUB_STORE_MODE, and every value exists to drive one arm of the store block:
#   same        identical bytes, `events: []`          -- STORE: IDENTICAL
#   differ      a per-arm value, `events: []`          -- STORE: DIFFERENT, exit 1
#   live        `events` non-empty                     -- NOT JUDGED, exit 2
#   nokey       no `events` key at all                 -- premise refusal, exit 2
#   dead_b      fails in b-interrupted only            -- exactly one arm answers, exit 1
ROLE="${1:-migrate}"
ARM_NOW="$(basename "$PWD")"

if [ "$ROLE" = "read" ]; then
  case "${STUB_READ_MODE:-ok}" in
    dead) echo "stub-read: this estate cannot be opened" >&2; exit 4 ;;
    *)    echo "stub-read: ok"; exit 0 ;;
  esac
fi

if [ "$ROLE" = "store" ]; then
  case "${STUB_STORE_MODE:-same}" in
    live)   printf '{"threads":[],"events":[{"id":"01J0","op":"thread.create"}]}\n' ;;
    nokey)  printf '{"threads":[]}\n' ;;
    differ) printf '{"threads":["%s"],"events":[]}\n' "$ARM_NOW" ;;
    dead_b) [ "$ARM_NOW" = "b-interrupted" ] && { echo "stub-store: no store here" >&2; exit 5; }
            printf '{"threads":[],"events":[]}\n' ;;
    *)      printf '{"threads":[],"events":[]}\n' ;;
  esac
  exit 0
fi

MODE="${STUB_MODE:-pass}"
N="${STUB_N:-40}"
DELAY="${STUB_DELAY:-0.02}"
ARM="$(basename "$PWD")"
OUT="intent/.rigstub"

# A NEW SESSION, so `kill -9 -$PGID` on the rig's process group does not reach
# here. POSIX::setsid fails for a process that already leads its group -- under
# the rig's `set -m` the SUBSHELL leads it, not this, so the call succeeds.
# This mode exists for one reason: to prove the settle assertion after the kill
# is coupled to the thing it measures.
if [ "$MODE" = "escape" ] && [ -z "${STUB_ESCAPED:-}" ]; then
  export STUB_ESCAPED=1
  exec perl -e 'use POSIX qw(setsid); setsid() or die "setsid: $!"; exec @ARGV or die "exec: $!"' -- "$0" || {
    echo "stub: could not detach into a new session -- the escape case cannot run" >&2
    exit 3
  }
fi

[ "$MODE" = "nowrite" ] && { echo "stub: mode=nowrite, wrote nothing, exiting 0"; exit 0; }

mkdir -p "$OUT" || { echo "stub: cannot create $OUT" >&2; exit 1; }

# The nonce is what makes `diverge` diverge. It is per-PROCESS, so the clean arm,
# the killed run and the re-run each stamp a different value -- which is what a
# non-idempotent migrator does, and what the verdict tool must call DIFFERENT.
NONCE=""
[ "$MODE" = "diverge" ] && NONCE=" nonce=$$-$SECONDS"

LIMIT="$N"
if [ "$MODE" = "nosentinel" ] && [ "$ARM" = "b-interrupted" ]; then
  LIMIT=$(( N / 10 ))
  [ "$LIMIT" -lt 1 ] && LIMIT=1
fi

echo "stub: mode=$MODE arm=$ARM writing $LIMIT of $N files, ${DELAY}s apart"
i=1
while [ "$i" -le "$LIMIT" ]; do
  printf 'stub file %04d%s\n' "$i" "$NONCE" > "$OUT/f_$(printf '%04d' "$i").txt"
  i=$((i + 1))
  sleep "$DELAY"
done
echo "stub: wrote $LIMIT files"
exit 0
