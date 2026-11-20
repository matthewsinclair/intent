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
#   - It sees `-q` anywhere among grep's flags (issue 0727): alone or in a
#     cluster (with a numeric option after it too, eg `-qm1`), before or after
#     the pattern, or spelt `--quiet` or `--silent`.
#     It reads from `grep` to the end of that pipeline stage, the next `|`, `;`,
#     `&` or `)`, so a quoted pattern holding one of those ends the stage early,
#     and a quoted pattern holding ` -q` reads as the flag.
#   - It says nothing about whether a flagged predicate is otherwise correct.
#   - It reads two populations, each by its own rule (issue 0723): the `.sh`
#     files directly under POP_DIRS, and every file under BASH_DIRS whose first
#     line declares bash, by a shebang or by shellcheck's `shell=bash`
#     directive. A bash file anywhere else, or one whose first line declares
#     neither, is outside it.
#
# Exit 0 clean, 1 findings (this gates), 2 not evaluable.

# NO `-e`, ON PURPOSE (issue 0727): its 1 is a finding and gates, and under `-e`
# a failing command would end it with its own status, which guard.lib says a
# gating guard cannot tell from a finding. So it reads the status of each step
# its verdict rests on, and refuses through die(), exit 2, when one fails.
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
# grep with -q anywhere among its flags, to the end of its stage (REACH says how).
GREPQ="grep[^$PIPE;&)]*[[:space:]](-[A-Za-z]*q[A-Za-z0-9]*|--quiet|--silent)([[:space:]]|\$)"
# `||` IS NOT A PIPE, and the first version of this tool read it as one. That
# gave two false positives immediately -- `[ ... ] || grep -q PAT file` reads a
# FILE with no pipeline anywhere, and `[ ... ] || grep -q PAT <<<"$x"` is the
# FIXED form being reported as the defect. A guard whose findings include the
# remedy it recommends teaches the reader to stop believing it, so the pipe must
# be a SINGLE one: preceded by the start of the line or by a non-pipe character.
PAT="(^|[^$PIPE])[$PIPE][[:space:]]*$GREPQ"

# THE POPULATION IS DECLARED AND ITS SIZE IS ASSERTED, FOR EACH SOURCE. A grep
# over an absent root exits 2 and prints nothing, which reads as a clean estate;
# the counts below are what make "no findings" mean something. Every directory
# holds shell this repository owns and commits.
#
# TWO SOURCES, BECAUSE NO ONE RULE NAMES BOTH (issue 0723). The instruments are
# the `.sh` files directly under POP_DIRS. The gates' own shell under BASH_DIRS
# carries no extension (`cut/gate`, `cmd/precommit`) or a `.lib` one, so a file
# there is taken by what its first line declares: a bash shebang, or, for a
# library the gates source, shellcheck's `shell=bash` directive. Each source
# that yields nothing refuses on its own, and so does each BASH_DIRS directory,
# which is committed tooling and never dehydrated as a thread's tools can be: a
# rule that stopped matching, or a directory that moved, cannot hide behind
# another's files.
POP_DIRS=(intent/st/ST0056/parity/tools intent/st/ST0057/parity/tools lib/templates/hooks)
BASH_DIRS=(bin/.devbin/cut bin/.devbin/cmd)
BASH_SHEBANG='^#!.*[/[:space:]]bash([[:space:]]|$)'
BASH_DIRECTIVE='^#[[:space:]]*shellcheck[[:space:]](.*[[:space:]])?shell=bash([[:space:]]|$)'

# declares_bash <path> -- true when the file's first line is a bash shebang or
# the `shell=bash` directive that shellcheck reads. One `read` per file, and no
# process.
declares_bash() {
  local first=""
  IFS= read -r first <"$1"
  [[ $first =~ $BASH_SHEBANG || $first =~ $BASH_DIRECTIVE ]]
}

files=()
n_sh=0
for d in "${POP_DIRS[@]}"; do
  # A thread's tools can be dehydrated, so an absent POP_DIRS directory is
  # skipped, and only the source as a whole must yield files. find's status
  # is read, so a partial listing refuses rather than shrinks the population.
  [ -d "$d" ] || continue
  list="$(find "$d" -maxdepth 1 -type f -name '*.sh' | sort)" || die "cannot list the .sh files under $d"
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    files+=("$f")
    n_sh=$((n_sh + 1))
  done <<<"$list"
done
[ "$n_sh" -gt 0 ] || die "the declared .sh population is EMPTY -- ${POP_DIRS[*]} held no .sh files, so a clean verdict here would be an artefact of finding nothing to read"

n_bash=0
for d in "${BASH_DIRS[@]}"; do
  [ -d "$d" ] || die "the declared bash directory $d is absent -- it is committed tooling, so its absence is a moved tree and not a clean one"
  list="$(find "$d" -type f | sort)" || die "cannot list the files under $d"
  n_dir=0
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    [ -r "$f" ] || die "cannot read $f, so whether it declares bash is unknown"
    declares_bash "$f" || continue
    files+=("$f")
    n_dir=$((n_dir + 1))
  done <<<"$list"
  [ "$n_dir" -gt 0 ] || die "the declared bash population is EMPTY under $d -- no file there declares bash on its first line, so a clean verdict on it would be an artefact of finding nothing to read"
  n_bash=$((n_bash + n_dir))
done

# ---------------------------------------------------------------------------
# scan_file <path> -- prints "<path>:<lineno>:<line>" per finding.
#
# A comment line is one whose first non-blank character is `#`. That is a
# FIRST-CHARACTER test and not a search for `#` anywhere, because a `#` inside a
# live command is not a comment and excluding those lines would blind the tool
# to real code.
#
# **ONE `awk` PER FILE, NOT ONE `grep` PER LINE.** This read each line in bash
# and forked `grep -Eq` for it: 91 instruments are some 28,000 lines, so some
# 28,000 processes, 28 s of the pre-commit gate's 46 on every commit of this
# repository whatever was staged. `awk` applies the same ERE to every line in
# one process; the comment test, the pattern and the output are unchanged, and
# the two-sided control below drives THIS function, so a scanner that stopped
# finding the planted defects would still refuse to report.
# ---------------------------------------------------------------------------
scan_file() {
  awk -v path="$1" -v pat="$PAT" '
    {
      stripped = $0
      sub(/^[[:space:]]+/, "", stripped)
      if (substr(stripped, 1, 1) == "#") next
      if ($0 ~ pat) printf "%s:%d:%s\n", path, NR, stripped
    }
  ' "$1"
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
  local tmp bad good found_bad found_good i
  # Each defect shape, one per line: direct, multi-stage, negated, -q later
  # among the flags, -q spelt long, and -q before a numeric option (issue 0727).
  local -a header=('#!/bin/bash' 'set -uo pipefail')
  local -a defects=(
    "f() { printf '%s' \"\$1\" $PIPE grep -q PAT; }"
    "g() { printf '%s' \"\$1\" $PIPE tr a b $PIPE grep -qxF \"\$2\"; }"
    "h() { ! printf '%s' \"\$1\" $PIPE grep -qE '^x'; }"
    "j() { printf '%s' \"\$1\" $PIPE grep -E -q '^x'; }"
    "k() { printf '%s' \"\$1\" $PIPE grep --quiet PAT; }"
    "m() { printf '%s' \"\$1\" $PIPE grep -qm1 PAT; }"
  )
  # The two false-positive families this tool has actually shipped, planted
  # rather than merely avoided: a comment describing the idiom, and a double
  # pipe, which is NOT a pipeline.
  local -a decoys=(
    "# a comment describing printf piped into grep -q must NOT be a finding"
    "i() { [ -n \"\$1\" ] || grep -q PAT \"\$2\"; }"
  )
  # The same predicates, fixed. Nothing here should be found.
  local -a fixes=(
    'f() { grep -q PAT <<<"$1"; }'
    'g() { grep -qxF "$2" <<<"$(tr a b <<<"$1")"; }'
    'h() { ! grep -qE "^x" <<<"$1"; }'
    'j() { grep -E -q "^x" <<<"$1"; }'
    'k() { grep --quiet PAT <<<"$1"; }'
    'm() { grep -qm1 PAT <<<"$1"; }'
  )

  tmp="$(mktemp -d)" || die "control: no temporary directory for its planted files"
  bad="$tmp/planted_defect.sh"
  good="$tmp/planted_fix.sh"
  printf '%s\n' "${header[@]}" "${defects[@]}" "${decoys[@]}" >"$bad" || { rm -rf "$tmp"; die "control: could not write its planted defects to $bad"; }
  printf '%s\n' "${header[@]}" "${fixes[@]}" >"$good" || { rm -rf "$tmp"; die "control: could not write its planted fixes to $good"; }
  found_bad="$(scan_file "$bad")" || { rm -rf "$tmp"; die "control: the scanner could not read its planted defects"; }
  found_good="$(scan_file "$good")" || { rm -rf "$tmp"; die "control: the scanner could not read its planted fixes"; }
  rm -rf "$tmp"

  # WHICH LINES, NOT HOW MANY, and both derived from what was written: the
  # defects stand on the lines after the header, and the scanner must find
  # each of those lines and no other, so a scanner that missed a defect and
  # found a decoy instead cannot balance a count.
  CONTROL_WANT=""
  for ((i = ${#header[@]} + 1; i <= ${#header[@]} + ${#defects[@]}; i++)); do
    CONTROL_WANT="${CONTROL_WANT}${CONTROL_WANT:+ }$i"
  done
  CONTROL_GOT="$(awk -F: 'NF { printf "%s%s", sep, $2; sep = " " }' <<<"$found_bad")"
  CONTROL_GOOD="$found_good"
  [ "$CONTROL_GOT" = "$CONTROL_WANT" ] && [ -z "$CONTROL_GOOD" ]
}

CONTROL_WANT=; CONTROL_GOT=; CONTROL_GOOD=
if ! control; then
  echo "pipefail-sigpipe: CONTROL FAILED -- the planted defects stand on line(s) ${CONTROL_WANT:-?} and the scanner found line(s) ${CONTROL_GOT:-none}; the planted fixes, where nothing is a finding, gave: ${CONTROL_GOOD:-nothing}." >&2
  echo "  The scanner is not doing what this tool reports it as doing, so no verdict is offered on the real population -- $n_sh .sh instrument(s) across ${#POP_DIRS[@]} declared directory(ies) and $n_bash bash file(s) under ${#BASH_DIRS[@]}, every one of them UNEXAMINED. A refusal that does not name its scope is indistinguishable from a refusal over nothing." >&2
  exit 2
fi

# ---------------------------------------------------------------------------
# The real scan.
# ---------------------------------------------------------------------------
findings=""
n_find=0
for f in "${files[@]}"; do
  # Its own status is read (issue 0727): a file awk cannot read has no verdict,
  # and adding no finding for it would pass it unread.
  hits="$(scan_file "$f")" || die "could not read $f, so whether it ends a pipeline in grep -q is unknown"
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    findings="${findings}${hit}
"
    n_find=$((n_find + 1))
  done <<<"$hits"
done

echo "pipefail-sigpipe: control -- every planted defect was found on its own line (${CONTROL_WANT}), and no decoy or planted fix was; both arms fired, so the verdict below means something."
echo "pipefail-sigpipe: EXAMINED ${#files[@]} file(s): $n_sh .sh instrument(s) across ${#POP_DIRS[@]} declared directory(ies), and $n_bash bash file(s) under ${#BASH_DIRS[@]}, by a shebang or shellcheck's shell=bash on their first line."

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
