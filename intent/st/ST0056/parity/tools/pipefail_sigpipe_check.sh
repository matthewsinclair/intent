#!/bin/bash
# pipefail_sigpipe_check.sh -- no instrument ends a pipeline in an early-exiting
# reader, because under `set -o pipefail` that reads FALSE on a TRUE input.
#
# THE DEFECT. `grep -q` exits the moment it matches. Its writer is then killed by
# SIGPIPE and exits 141. Under pipefail the pipeline takes the WORST status, so
# 141 becomes the pipeline status and the test reads FALSE -- on an input that
# matched. In a guard that is a finding LOST; where the test is negated with `!`
# it is a finding INVENTED, which is the same bug wearing the opposite sign.
#
# IT IS INVISIBLE BY CONSTRUCTION. A lost race looks exactly like a clean run:
# same exit code, same silence, same green. Nothing downstream of the symptom can
# tell the two apart, which is why this is a guard and not a code review note.
#
# EXPOSURE IS NOT UNIFORM AND THIS TOOL DOES NOT PRETEND OTHERWISE. SIGPIPE needs
# the write to outrun the pipe buffer, so a small payload completes before the
# reader exits and the same line is harmless. The population here is EXACT; the
# per-site exposure is NOT MEASURED and is not claimed. That is deliberate: a
# check that only flagged sites it could prove lose today would go quiet the
# moment a body grew, which is the direction this defect actually arrives from.
#
# MEASURED 2026-09-02: the sweep hv ruled found 36 live sites across 15 files,
# up from 24 reported in August with nobody adding one on purpose. The
# population is not static, which is the argument for a guard over a sweep.
#
# THE REMEDY IS A HERESTRING, and for a MULTI-STAGE pipeline it is not enough:
#
#   before   printf '%s' "$x" | grep -q PAT
#   after    grep -q PAT <<<"$x"
#
#   before   printf '%s' "$x" | tr A B | grep -q PAT          # tr is SIGPIPEd
#   after    grep -q PAT <<<"$(tr A B <<<"$x")"               # substitution reads to EOF
#
# A herestring on the FIRST stage of a multi-stage pipeline fixes NOTHING: the
# early-exiting reader is still downstream of another writer. Four of the 36
# sites were that shape, and the obvious remedy applied uniformly would have
# left all four still losing.
#
# ONE SITE CHANGED ITS ANSWER UNDER THE NAIVE REMEDY and is worth knowing about
# before anyone fixes a finding from this tool: where the final stage is
# `grep -qv`, an EMPTY capture becomes ONE EMPTY LINE under a herestring, which
# `-v` matches, flipping false to true. Test emptiness explicitly there.
#
# HOW TO DESCRIBE THE IDIOM WITHOUT TRIPPING THIS TOOL: put it in a comment.
# Comment lines are excluded, deliberately and by first-character test, because
# a guard that punished its own documentation would be uncomfortable to explain
# and would push the explanation out of the file that needs it. This header is
# itself the worked example -- every form above sits in a comment.
#
# THIS TOOL CAN SEE ITSELF. Its detection pattern is built so that it does not
# contain the sequence it looks for, so it is a member of its own population
# rather than an exception to it. A checker that must be excluded from its own
# check has an untested assumption at exactly the point it is least visible.
#
# REACH -- what this cannot see, stated because a guard that implies more than
# it checks is worse than one that checks nothing:
#   - It reads LINES. A pipeline split across a line continuation with the
#     `grep` on the next line is invisible to it.
#   - It knows nothing about whether `pipefail` is in force at the SITE. It
#     reports the file-level setting as context and gates regardless, because a
#     file one `set -o pipefail` away from the defect is not a file that is safe.
#   - `grep -q` is the only early-exiting reader it knows. `head`, `grep -m` and
#     an exiting `awk` have the same shape and are NOT its subject: they are
#     usually deliberate, and widening a fix quietly is its own defect class.
#   - It says nothing about whether a flagged predicate is otherwise correct.
#
# Exit 0 clean, 1 findings (this gates), 2 not evaluable.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../../../.." && pwd)"

die() { echo "pipefail-sigpipe: error: $1" >&2; exit 2; }

cd "$REPO_ROOT" || die "cannot reach the repository root"

# The sequence is assembled rather than written, so this file is inside its own
# population. Written literally, the tool would flag itself on every run and the
# only available fix would be an exemption -- which is the thing it exists to
# stop other files from needing.
PIPE='|'
GREPQ="grep[[:space:]]+-[A-Za-z]*q"
# `||` IS NOT A PIPE, and the first version of this tool read it as one. That
# gave two false positives immediately -- `[ ... ] || grep -q PAT file` reads a
# FILE with no pipeline anywhere, and `[ ... ] || grep -q PAT <<<"$x"` is the
# FIXED form being reported as the defect. A guard whose findings include the
# remedy it recommends teaches the reader to stop believing it, so the pipe must
# be a SINGLE one: preceded by the start of the line or by a non-pipe character.
PAT="(^|[^$PIPE])[$PIPE][[:space:]]*$GREPQ"

# THE POPULATION IS DECLARED AND ITS SIZE IS ASSERTED. A grep over an absent
# root exits 2 and prints nothing, which reads as a clean estate; the count
# below is what makes "no findings" mean something. Both directories hold
# instruments this repository owns and commits.
POP_DIRS=(intent/st/ST0056/parity/tools intent/st/ST0057/parity/tools lib/templates/hooks)

files=()
for d in "${POP_DIRS[@]}"; do
  [ -d "$d" ] || continue
  while IFS= read -r f; do
    [ -n "$f" ] && files+=("$f")
  done < <(find "$d" -maxdepth 1 -type f -name '*.sh' | sort)
done

[ "${#files[@]}" -gt 0 ] || die "the declared population is EMPTY -- ${POP_DIRS[*]} held no .sh files, so a clean verdict here would be an artefact of finding nothing to read"

# ---------------------------------------------------------------------------
# scan_file <path> -- prints "<path>:<lineno>:<line>" per finding.
#
# A comment line is one whose first non-blank character is `#`. That is a
# FIRST-CHARACTER test and not a search for `#` anywhere, because a `#` inside a
# live command is not a comment and excluding those lines would blind the tool
# to real code.
# ---------------------------------------------------------------------------
scan_file() {
  local path="$1" n=0 line stripped
  while IFS= read -r line || [ -n "$line" ]; do
    n=$((n + 1))
    stripped="${line#"${line%%[![:space:]]*}"}"
    case "$stripped" in '#'*) continue ;; esac
    # Herestring, not a pipe: this tool is a member of its own population and
    # would otherwise be its own first finding.
    if grep -Eq "$PAT" <<<"$line"; then
      printf '%s:%s:%s\n' "$path" "$n" "$stripped"
    fi
  done < "$path"
}

# ---------------------------------------------------------------------------
# THE TWO-SIDED CONTROL, RUN BEFORE ANY VERDICT IS REPORTED.
#
# An instrument that has only ever produced one of its two answers has not been
# shown to be measuring anything. So the scanner is driven against a planted
# defect (must be RED) and a planted fix (must be GREEN) on every run, in a
# throwaway directory, and the tool REFUSES to report on the real population
# unless both fire. A control that cannot fail is decoration.
# ---------------------------------------------------------------------------
control() {
  local tmp rc_bad rc_good bad good
  tmp="$(mktemp -d)" || return 1
  bad="$tmp/planted_defect.sh"
  good="$tmp/planted_fix.sh"

  # The defect, in its three shapes: direct, multi-stage, and negated.
  {
    echo '#!/bin/bash'
    echo 'set -uo pipefail'
    echo "f() { printf '%s' \"\$1\" $PIPE grep -q PAT; }"
    echo "g() { printf '%s' \"\$1\" $PIPE tr a b $PIPE grep -qxF \"\$2\"; }"
    echo "h() { ! printf '%s' \"\$1\" $PIPE grep -qE '^x'; }"
    echo "# a comment describing printf piped into grep -q must NOT be a finding"
    echo "i() { [ -n \"\$1\" ] || grep -q PAT \"\$2\"; }"   # a double pipe is NOT a pipeline
  } > "$bad"

  # The same three predicates, fixed. Nothing here should be found.
  {
    echo '#!/bin/bash'
    echo 'set -uo pipefail'
    echo 'f() { grep -q PAT <<<"$1"; }'
    echo 'g() { grep -qxF "$2" <<<"$(tr a b <<<"$1")"; }'
    echo 'h() { ! grep -qE "^x" <<<"$1"; }'
  } > "$good"

  local n_bad n_good
  n_bad="$(scan_file "$bad" | wc -l | tr -d ' ')"
  n_good="$(scan_file "$good" | wc -l | tr -d ' ')"
  rm -rf "$tmp"

  CONTROL_BAD="$n_bad"
  CONTROL_GOOD="$n_good"
  # Three planted defects must be found. The comment must NOT be, and neither
  # must the `||` line -- both are false-positive families this tool has
  # actually shipped, so they are planted rather than merely avoided.
  [ "$n_bad" = "3" ] && [ "$n_good" = "0" ]
}

CONTROL_BAD=; CONTROL_GOOD=
if ! control; then
  echo "pipefail-sigpipe: CONTROL FAILED -- planted defect gave ${CONTROL_BAD:-?} findings (expected 3), planted fix gave ${CONTROL_GOOD:-?} (expected 0)." >&2
  echo "  The scanner is not doing what this tool reports it as doing, so no verdict is offered on the real population." >&2
  exit 2
fi

# ---------------------------------------------------------------------------
# The real scan.
# ---------------------------------------------------------------------------
findings=""
n_find=0
for f in "${files[@]}"; do
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    findings="${findings}${hit}
"
    n_find=$((n_find + 1))
  done < <(scan_file "$f")
done

echo "pipefail-sigpipe: control -- planted defect $CONTROL_BAD/3 found, planted fix $CONTROL_GOOD/0 found; both arms fired, so the verdict below means something."
echo "pipefail-sigpipe: EXAMINED ${#files[@]} instrument(s) across ${#POP_DIRS[@]} declared directory(ies)."

if [ "$n_find" -eq 0 ]; then
  echo "pipefail-sigpipe: no instrument ends a pipeline in grep -q."
  exit 0
fi

echo "pipefail-sigpipe: FINDING -- $n_find site(s) end a pipeline in an early-exiting reader:"
printf '%s' "$findings" | while IFS= read -r hit; do
  [ -n "$hit" ] || continue
  hpath="${hit%%:*}"
  rest="${hit#*:}"
  hline="${rest%%:*}"
  htext="${rest#*:}"
  if grep -q 'set -[a-z]*o\{0,1\}[[:space:]]*pipefail\|set -o pipefail' "$hpath" 2>/dev/null; then
    armed="pipefail SET in this file"
  else
    armed="pipefail not set in this file -- latent, and one line away from live"
  fi
  printf '    %s:%s  (%s)\n' "$hpath" "$hline" "$armed"
  printf '      %s\n' "$htext"
done

cat <<'REMEDY'

  Under set -o pipefail, grep -q exits on first match, its writer takes SIGPIPE
  and exits 141, and pipefail promotes 141 to the pipeline status -- so the test
  reads FALSE on an input that matched.

  Rewrite the final stage as a herestring. If anything sits BETWEEN the original
  writer and the grep, move that stage into a command substitution as well: a
  herestring on the first stage alone leaves the early-exiting reader downstream
  of a writer that can still be SIGPIPEd.

  Where the final stage is grep -qv, test emptiness explicitly: an empty capture
  becomes ONE EMPTY LINE under a herestring, which -v matches, and the answer
  flips.

  To DESCRIBE the idiom without tripping this check, put it in a comment.
REMEDY

exit 1
