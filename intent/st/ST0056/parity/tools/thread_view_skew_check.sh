#!/bin/bash
# thread_view_skew_check.sh -- the estate's generated THREAD views still match the model.
#
# ST0057. The detection has existed since `views::skew` landed and nothing has
# ever gated on it. Its sibling `view_skew_check.sh` covers the parity APPARATUS
# views and its `CHECKABLE` is ONE triple in `surface/`; `intent doctor` renders
# 268 and compares every one. So the gate's skew coverage is 1 of 269 and the
# missing 268 are the thread covers, the acceptance contracts and the work
# package covers -- the files a hand edit silently loses at the next sync.
#
# WHY THIS COULD NOT BE WIRED BEFORE TODAY, RECORDED SO THE DATE IS NOT A
# MYSTERY. Until `b082b488` `doctor` reported every dehydrated view as MISSING
# -- 235 findings at rc=1 on a healthy estate -- so gating on it would have
# bolted a permanently-red check onto a correct tree, which is the shape that
# gets a check deleted rather than fixed. WP-10 taught `doctor` to ask the
# manifest and the report went to 0. This instrument is only honest downstream
# of that, and it is worth saying that the blocker was never the wiring.
#
# IT COMPUTES NOTHING, AND THAT IS THE DESIGN RATHER THAN LAZINESS. `views::skew`
# is the single home for the question *is this view skewed*. A shell
# reimplementation would be a SECOND opinion, free to disagree with the one
# `doctor` prints to the operator -- and the estate has already paid for that
# shape twice, in a collection verifier carrying its own copy of the classifier's
# eligibility rules and in a roster that agreed with neither dispatcher beneath
# it. This parses one verdict; it does not form one. The one choice it does
# make, where doctor runs, copies the store's own, and the block that makes
# it says why.
#
# IT READS `doctor --format json`, NEVER DOCTOR'S TEXT (ST0095 WP-03). The text
# parse it replaces matched `^residue: <path> -- view-skew -- `, and from
# 9331cb11a (2026-09-07), which grouped the report by class, it matched nothing:
# the summary line still parsed, so its denominator check passed, and this arm
# answered "match" for every tree until WP-03's red control drove a hand edit
# past it. A denominator proves the report was read, not that a finding can be,
# so each field read here refuses when it is missing: `checked.views`, which
# must be above zero, because an empty population and a clean estate compare
# equal, and `findings`, which must be a list. A finding is of the class when
# its `class` field is `view-skew`, compared whole: a path or a message that
# mentions the class is not a finding of it. A class name the build no longer
# prints would still select nothing and pass every tree, the same blindness one
# field over, so a control runs before the read and refuses unless the build
# reports `view-skew` by that name. A project that acknowledges the class has
# its findings moved out of `findings`, so that is a refusal too, never a match.
#
# `--changed <paths...>` BLOCKS ONLY ON WHAT THIS COMMIT TOUCHES and reports the
# rest, which is the clock guard's ratified inherited-breakage rule: a guard that
# must be bypassed to work is a guard nobody keeps, and a repository that
# arrived already skewed would otherwise be unable to commit the fix.
#
# Exit codes: 0 clean, not applicable, or inherited-only; 1 this commit touches
# a skewed view; 2 the instrument cannot answer (no verdict, a report it cannot
# read, a control that failed, or the class acknowledged).

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# tools -> parity -> ST0056 -> st -> intent -> repo root. FIVE, and it is spelled
# out for the same reason the sibling spells it out: a miscount resolves to a
# directory that exists, so it fails as a wrong answer rather than as an error.
ROOT="$(cd "$HERE/../../../../.." && pwd)"

die() { echo "thread-view-skew: $*" >&2; exit 2; }

CHANGED=""
TRIGGERED=0
if [ "${1:-}" = "--changed" ]; then
  shift
  CHANGED=" $* "
  TRIGGERED=1
fi

# THE BINARY IS RESOLVED BY ABSOLUTE PATH because this arm judges the views the
# tree's own build renders, and `intent` on PATH is the installed release.
BIN="$ROOT/native/rust/target/release/intent"
if [ ! -x "$BIN" ]; then
  # REFUSES, IT DOES NOT SKIP (hv ruling 4 as scoped by vc 2026-08-27, authority
  # vc). This arm used to name the gap and exit 0, which was the defect rather
  # than the mitigation: exit 0 is the SAME STATUS a run that examined every
  # view returns, so no caller reading the exit code could tell 288 views
  # checked from 0 views checked. The warning text was the only carrier of the
  # difference, and text is what a gate summary drops. That is
  # IN-AG-NO-SILENT-001, not a preference.
  #
  # THE TWO STATES BELOW ARE SEPARATED BECAUSE THE BLOCKED READER IS THE POINT.
  # A refusal that reads like a defect gets worked around, and a guard that gets
  # worked around is worth less than the silent one it replaced. Whoever meets
  # this at 2am must be able to tell "wait and retry" from "something is broken"
  # WITHOUT opening this file.
  if [ -d "$ROOT/native/rust/target/release" ]; then
    # TRANSIENT. A release directory means a build has run in this tree, so the
    # binary is absent because something removed it. cargo clears the artefact
    # before writing the new one; that window was measured at ~66 seconds on
    # this estate, and a commit landing inside it is ordinary peer concurrency
    # rather than a fault.
    echo "thread-view-skew: REFUSING -- no v3 binary at $BIN, but $ROOT/native/rust/target/release exists, so a build has run here and this is almost certainly A REBUILD IN PROGRESS." >&2
    echo "  THIS IS TRANSIENT AND NOTHING IS BROKEN. The window is about 60 seconds. Wait, then retry the commit." >&2
    echo "  If it persists past a couple of minutes the pair is genuinely gone -- remedy: \`bin/devbin build all\`." >&2
  else
    # NOT TRANSIENT. No release directory at all, so this tree has never been
    # built and waiting will not help.
    echo "thread-view-skew: REFUSING -- no v3 binary at $BIN and no release directory, so this tree has never been built." >&2
    echo "  THIS IS NOT TRANSIENT. Waiting will not help." >&2
    echo "  Remedy: \`bin/devbin build all\`." >&2
  fi
  echo "  Why this refuses rather than skipping: without the binary every generated thread view goes UNCHECKED, and exiting 0 would report that as a pass." >&2
  exit 2
fi

command -v jq >/dev/null 2>&1 ||
  die "jq is not on PATH, and it is what reads doctor's report, so this commit is unchecked rather than clean"

# The class this arm reads, spelled once: the control and the read both take it
# from here.
CLASS="view-skew"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/thread-view-skew.XXXXXX")" ||
  die "no temporary directory for the control, so this commit is unchecked rather than clean"
trap 'rm -rf "$WORK"' EXIT

# THE CONTROL: THE BUILD REPORTS $CLASS BY THAT NAME. doctor checks each key of
# `doctor.acknowledged` against the class names it prints, and a key it does
# not print acknowledges nothing and comes back as a setting it cannot honour.
# So a scratch project carrying this checkout's config, with $CLASS
# acknowledged, must come back with $CLASS among `acknowledged`.
if ! {
  mkdir -p "$WORK/probe/intent/.config" &&
    jq --arg class "$CLASS" '.doctor.acknowledged[$class] = "the control in thread_view_skew_check.sh"' \
      "$ROOT/intent/.config/config.json" > "$WORK/probe/intent/.config/config.json"
}; then
  die "could not make the control's scratch project at $WORK/probe, so this commit is unchecked rather than clean"
fi
CONTROL="$(cd "$WORK/probe" && "$BIN" doctor --format json)"
CRC=$?
if [ "$CRC" -gt 1 ] ||
  ! printf '%s' "$CONTROL" | jq -e --arg class "$CLASS" 'any(.acknowledged[]?; .class == $class)' >/dev/null; then
  die "CONTROL FAILED: a scratch project that acknowledges $CLASS did not come back with it acknowledged (doctor exited $CRC), so $BIN may not report $CLASS by that name, and reading for it would pass every tree; this commit is unchecked rather than clean. The control's report follows:
$CONTROL"
fi

# THIS TOOL NEVER MIGRATES OR MAKES THE STORE IT JUDGES (ST0095 WP-03, AC-03.1).
# `doctor` opens this checkout's store, and a build whose store schema is newer
# migrates a store as it opens it, one way: the release installed from brew
# then refuses it. Every seat in this checkout shares the build above, so one
# rebuild after a schema bump would migrate the main store at the next commit,
# whoever made it. The shipped half of the commit gate runs its own `intent
# doctor` earlier, through PATH: that is the installed release while brew comes
# first on PATH, and this build after `intent use dev`, which then opens the
# store before this tool runs and is out of its reach (issue 0717; vc decision
# 132 keeps the seats on brew).
#
# So the store's schema is read from its header, never by opening it, and the
# build's from the header of the store the control above made. A store older
# than the build, one whose header does not say, or no store at all, which
# doctor would make at this build's schema, is judged as a copy of this
# checkout's intent/ with no store in it, which doctor builds from canon as a
# fresh clone does, and the checkout's store is neither opened nor made. A
# newer store is judged in place: this build cannot open it, so doctor reports
# that as a finding of its own and judges the views with no render record, as
# on the copy.
#
# Measured on this tree on 2026-11-19, at 1184 views: with one thread cover
# edited by hand, the copy and the store in place both name that view and no
# other, and without the edit both name none. A copy has no record of what the
# store rendered, so a view the store re-rendered and nobody has written yet,
# which reads as store-stale in place and is not this tool's class, reads as
# view-skew in the copy: on the copy path, a peer's write still landing can
# block a commit that stages that view. `.backup` and `.cache` stay out of the
# copy, about 3.4 GB of the store's backups and the search index, which doctor
# does not read for this verdict. A header is current once its store's last
# connection has closed: a checkout store whose newer schema still sits in its
# WAL reads as older, which only sends doctor to the copy, and a control store
# whose header reads 0 still holds its stamp in its WAL, which is a refusal,
# since against 0 every store would read as current.
#
# THIS IS A SECOND COPY OF `Store::init`'s DECISION (intentsvcs store.rs), kept
# here for want of a door: the binary cannot say whether opening a store would
# migrate it without opening it. Issue 0717 is that door. WHEN IT LANDS, DELETE
# THIS BLOCK AND ASK IT.

# The store's path below a project root, written once.
STORE_REL="intent/.cache/intent.db"

# schema_in <store> -- the user_version a SQLite header carries (bytes 60 to 63,
# big-endian), or nothing for a file too short to carry one.
schema_in() {
  od -An -tu1 -j60 -N4 "$1" |
    awk 'NF == 4 { print $1 * 16777216 + $2 * 65536 + $3 * 256 + $4 }'
}

STORE="$ROOT/$STORE_REL"
COPY_WHY=""
if [ ! -f "$STORE" ]; then
  COPY_WHY="this checkout has no store, and doctor would make one at the schema $BIN writes"
else
  BUILT="$(schema_in "$WORK/probe/$STORE_REL")"
  [ "${BUILT:-0}" -gt 0 ] 2>/dev/null ||
    die "the store the control made at $WORK/probe/$STORE_REL does not show which schema $BIN writes (its header reads ${BUILT:-nothing}), so this commit is unchecked rather than clean"
  HELD="$(schema_in "$STORE")"
  if [ -z "$HELD" ] || [ "$HELD" -lt "$BUILT" ]; then
    COPY_WHY="$BIN writes store schema $BUILT and this checkout's store is at ${HELD:-a schema its header does not show}"
  fi
fi

DOCTOR_ROOT="$ROOT"
if [ -n "$COPY_WHY" ]; then
  clone=(cp -R)
  [ "$(uname -s)" = Darwin ] && clone=(cp -c -R)
  mkdir -p "$WORK/copy/intent" || die "could not make $WORK/copy/intent to judge a copy in, so this commit is unchecked rather than clean"
  for entry in "$ROOT"/intent/* "$ROOT"/intent/.[!.]*; do
    [ -e "$entry" ] || continue
    case "${entry##*/}" in .backup | .cache) continue ;; esac
    "${clone[@]}" "$entry" "$WORK/copy/intent/" ||
      die "could not copy $entry to judge a copy of intent/, so this commit is unchecked rather than clean"
  done
  DOCTOR_ROOT="$WORK/copy"
  echo "thread-view-skew: $COPY_WHY, so doctor judges a copy of intent/, and this checkout's store is neither opened nor made." >&2
fi
cd "$DOCTOR_ROOT" || die "could not enter $DOCTOR_ROOT to run doctor there, so this commit is unchecked rather than clean"

OUT="$("$BIN" doctor --format json)"
RC=$?
# `doctor` exits 1 for findings of ANY class, so its exit code is not this
# instrument's verdict and must not be read as one. Anything above 1 is the
# command failing rather than reporting, and a failure to run is not a pass.
[ "$RC" -le 1 ] || die "\`$BIN doctor --format json\` exited $RC -- it did not produce a verdict, so this commit is unchecked rather than clean. Its report follows:
$OUT"

VIEWS="$(printf '%s' "$OUT" | jq -er '.checked.views | numbers')" ||
  die "\`doctor --format json\` gave no view count at .checked.views, so this script cannot show it read the views, and this commit is unchecked rather than clean"
[ "$VIEWS" -gt 0 ] || die "\`doctor\` reports $VIEWS views rendered. An empty population and a clean estate compare equal, so this is a refusal and not a pass."

# An acknowledged class's findings leave `findings` for `acknowledged` (issue
# 0065), where the read below does not look, and whether an acknowledged skew
# blocks a commit is not this arm's to rule.
if printf '%s' "$OUT" | jq -e --arg class "$CLASS" 'any(.acknowledged[]?; .class == $class)' >/dev/null; then
  die "this project acknowledges $CLASS in doctor.acknowledged, so doctor reports those findings outside its verdict and this arm cannot say the views match; whether an acknowledged skew blocks a commit needs a ruling"
fi

SKEWED="$(printf '%s' "$OUT" | jq -r --arg class "$CLASS" '
  if (.findings | type) != "array" then error("the report carries no findings list") else . end
  | .findings[] | select(.class == $class)
  | if (.file | type) == "string" then .file else error("a \($class) finding names no file") end
')" || die "could not read the $CLASS findings from \`doctor --format json\` (jq's reason is above), so this commit is unchecked rather than clean"

if [ -z "$SKEWED" ]; then
  echo "thread-view-skew: $VIEWS generated view(s) match the model."
  exit 0
fi

BLOCKING=""
INHERITED=""
while IFS= read -r path; do
  [ -n "$path" ] || continue
  if [ "$TRIGGERED" -eq 1 ] && [ "${CHANGED#* "$path" }" = "$CHANGED" ]; then
    INHERITED="${INHERITED}${path}
"
  else
    BLOCKING="${BLOCKING}${path}
"
  fi
done <<< "$SKEWED"

n_block="$(printf '%s' "$BLOCKING" | grep -c . )"
n_inherit="$(printf '%s' "$INHERITED" | grep -c . )"
n_block="${n_block:-0}"
n_inherit="${n_inherit:-0}"

if [ "$n_inherit" -gt 0 ]; then
  echo "thread-view-skew: $n_inherit view(s) already skewed before this commit -- REPORTED, never blocked:" >&2
  printf '%s' "$INHERITED" | sed 's/^/  /' >&2
fi

if [ "$n_block" -eq 0 ]; then
  echo "thread-view-skew: $VIEWS generated view(s) checked; this commit adds no skew."
  exit 0
fi

echo "BLOCKED: this commit touches a generated view that no longer matches the model." >&2
printf '%s' "$BLOCKING" | sed 's/^/  /' >&2
echo "  Of $VIEWS view(s) rendered from the model, the $n_block above differ from what is on disk." >&2
echo "  A generated view is rendered FROM the model, so a hand edit here is discarded by the next sync." >&2
echo "  Make the change through the CLI so it lands in the model, or run \`intent sync --to-disk\` to" >&2
echo "  discard the edit -- copying anything you meant to keep out of the file FIRST." >&2
exit 1
