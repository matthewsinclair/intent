#!/usr/bin/env bash
# id_pattern_check.sh -- no commit adds a pattern for an id (ST0095 AC-12.1, the
# guard half of WP-12).
#
# WHY. ST0095 moves thread, work package and issue ids from ordinals to hashes,
# and one module of the binary owns an id. A script that spells an id's shape
# for itself -- a regex, glob, `case` arm or awk, sed or grep expression that
# matches or extracts one -- is a second owner, and it reads a hashed id as no
# id at all, silently. The sites that carry one today are this guard's CENSUS,
# parity/data/id_patterns.tsv, each to be routed through the binary's id door
# (WP-10) by the other half of WP-12. This guard is the ratchet: no new one.
#
# IT JUDGES THE WHOLE INDEX AGAINST HEAD, NOT THE DIFF. A ratchet over added
# lines alone can be walked around: a census row dropped while its site stays,
# a site copied to another file under a moved row, or one removed row standing
# for several new sites all add nothing it would read. So it reads every line
# of the scripts, hooks, templates, plugins, parity tools and workflows (ROOTS
# below) that the DETECTOR matches, in the index and at HEAD, each against the
# census of its own side, and finds:
#   - NEW: a detected line, not a comment, holding no needle (a fixed string)
#     that a census row gives for its file;
#   - STALE: a row whose needle is on no detected line of its file -- a site
#     routed away takes its row with it;
#   - GROWN: a needle on more lines of the files whose rows list it than at
#     HEAD, counted the same way there against HEAD's census. A rename keeps the
#     count; a copy does not;
#   - ADDED: a row HEAD's census lacks, unless it is a move: a HEAD row with
#     the same needle whose row and site have both gone. So THE CENSUS ONLY
#     SHRINKS, by rows as well as by counts.
# A comment line, one whose first non-blank character is `#`, is not code. With
# no census at HEAD (the commit that brings it) there is nothing to grow from or
# add to, and it says so; NEW and STALE still hold.
#
# THE CONTROL runs first, in a scratch repository and never in this one, and
# refuses to report unless each arm fires: one planted new pattern for every form
# the detector knows, each found on its own line, and one under the glob root; a
# planted stale row; a planted copy of a listed site, found as growth; and the
# listed site and the non-patterns, found as nothing. Its scratch git runs with
# the hook's GIT_* variables unset and the user's signing and hooks off, so it
# never touches the index being judged.
#
# REACH -- what this cannot see:
#   - It reads LINES. A pattern split across a continuation, or built at run
#     time from pieces (as this file builds its own fixtures), is invisible.
#   - The DETECTOR knows these forms, each at a word boundary: ST followed by a
#     character class (not a subscript `[@` or `[$`), `\d`, a glob (`*`, `??`),
#     `{`, a group or a zero-padded printf format; WP- or WP/ (either case,
#     for the `/wp/NN` path) followed by a class, `\d`, a glob, `?`, a group or
#     a padded format; AC- or AT- followed by a class, `\d`, `*`, a group or a
#     padded format; ISSUE: followed by a class, `*` or a padded format; and
#     issues/ followed by a class or a padded format. And a four-digit class --
#     `[0-9]{4}`, `[0-9]{3,4}`, `[0-9]{4,}`, four `[0-9]`, `[[:digit:]]{4}`,
#     `\d{4}` or `\d\d\d\d` -- that no `-`, `[` or `]` borders, which is how it
#     tells an issue id from a date.
#   - NOT SEEN, since each also spells things that are no id or hides the
#     prefix: a bare `printf '%04d'` (realise_plan.sh mints an issue id that
#     way; stub files are numbered so), a BRE interval `[0-9]\{4\}` (a Rust
#     edition's year), `ST.{4}`, a fixed-width cut (`cut -c`, `${x:0:N}`), a
#     prefix in a class or an alternation (`S[T]`, `(ST|AT)`, `[Ss][Tt]`, a
#     case-blind `grep -i`), an extglob `ST+([0-9])`, `issue-NNNN`, a bare
#     two-digit package number, and the hashed shape ST0095 brings.
#   - A needle is a fixed string, so a line already holding a listed needle may
#     carry a second, different pattern unseen.
#   - Files outside ROOTS (eg tests/, native/rust/, docs/), including one a root
#     sources or links to, are not its subject.
#   - A line the detector reads that is no id (a year, a port) refuses; the
#     route is a reviewed change to the detector, whose control covers each form.
#
# Exit 0 clean, 1 findings (this gates), 2 not evaluable.

# NO `-e`, ON PURPOSE: its 1 is a finding and gates, and under `-e` a failing
# command would end it with its own status, which guard.lib says a gating guard
# cannot tell from a finding. So it reads the status of each step its verdict
# rests on, and refuses through die(), exit 2, when one fails.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" || {
  echo "id-pattern: error: cannot find the directory this tool is in" >&2
  exit 2
}
REPO_ROOT="$(cd "$HERE/../../../../.." && pwd)" || {
  echo "id-pattern: error: cannot reach the repository root from $HERE" >&2
  exit 2
}

die() {
  echo "id-pattern: error: $1" >&2
  exit 2
}

cd "$REPO_ROOT" || die "cannot reach the repository root"

ROOTS=(bin .githooks lib/templates intent/plugins ':(glob)intent/st/*/parity/tools/**' .github)
CENSUS=intent/st/ST0056/parity/data/id_patterns.tsv

# THE DETECTOR, one ERE, which git grep and awk both read, awk from the
# environment so its backslashes reach the regex as written. The thread prefix
# is written `S[T]` so this line holds no form it detects.
export ID_PATTERN_RE='(^|[^A-Za-z0-9_])(S[T](\[[^@$]|\\d|\*|\{|\(|\?\?|%0[0-9]*d)|[Ww][Pp][-/](\[|\\d|\*|\(|\?|%0[0-9]*d)|A[CT]-(\[|\\d|\(|\*|%0[0-9]*d)|ISSUE:(\[|\*|%0[0-9]*d)|issues/(\[|%0[0-9]*d))|(^|[^]0-9-])(\[0-9\](\{4\}|\{3,4\}|\{4,\}|\[0-9\]\[0-9\]\[0-9\])|\[\[:digit:\]\]\{4\}|\\d(\{4\}|\\d\\d\\d))([^-[]|$)'

# The repository git reads: this one, under the hook's own GIT_* variables, or
# the control's scratch repository, with them unset and the user's signing and
# hooks off. (A fourth copy of this list; one shared helper is a follow-up.)
SCRATCH=""
gitx() {
  if [ -n "$SCRATCH" ]; then
    env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE -u GIT_OBJECT_DIRECTORY \
      -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR -u GIT_NAMESPACE -u GIT_PREFIX \
      git -C "$SCRATCH" -c maintenance.auto=false -c commit.gpgsign=false -c core.hooksPath=/dev/null "$@"
  else
    git "$@"
  fi
}

# hits <tree> -- prints "<file>:<line>:<text>" for each line of <tree> (empty
# for the index) under ROOTS that the detector matches. Every file is read as
# text (`-a`), so no attribute or stray byte can hide one, and colour and path
# quoting are pinned off. git grep exits 1 for no line, which is an answer;
# anything else is no verdict, so its status is read.
hits() {
  local out rc=0
  if [ -z "$1" ]; then
    out="$(gitx -c core.quotePath=false grep --no-color --cached -n -a -E -e "$ID_PATTERN_RE" -- "${ROOTS[@]}")" || rc=$?
  else
    out="$(gitx -c core.quotePath=false grep --no-color -n -a -E -e "$ID_PATTERN_RE" "$1" -- "${ROOTS[@]}")" || rc=$?
    out="${out//$'\n'"$1":/$'\n'}"
    out="${out#"$1":}"
  fi
  case "$rc" in
    0) printf '%s\n' "$out" ;;
    1) ;;
    *) return 2 ;;
  esac
}

# verdict <census> <index hits> <HEAD census, or NOCENSUS> <HEAD hits> --
# prints the findings, one per line, as NEW, STALE, GROWN, ADDED or UNREAD with
# tab-separated fields. UNREAD is a line git grep matched that awk does not, a
# disagreement between the two readers, and no verdict. Pure: it reads only its
# arguments.
verdict() {
  CENSUS_TEXT="$1" INDEX_HITS="$2" HEAD_CENSUS="$3" HEAD_HITS="$4" awk '
    function comment(line,   lead) {
      lead = line
      sub(/^[[:space:]]+/, "", lead)
      return substr(lead, 1, 1) == "#"
    }
    function parse(rec) {
      p = index(rec, ":"); hf = substr(rec, 1, p - 1); rest = substr(rec, p + 1)
      q = index(rest, ":"); hl = substr(rest, 1, q - 1); ht = substr(rest, q + 1)
    }
    function load(text, file, needle, keys,   n, i, f, k) {
      n = split(text, lines, "\n"); k = 0
      for (i = 1; i <= n; i++) {
        if (lines[i] ~ /^#/ || lines[i] == "") continue
        split(lines[i], f, "\t"); k++
        file[k] = f[1]; needle[k] = f[2]; keys[f[1] "\t" f[2]] = 1
      }
      return k
    }
    # count <hits> <file-needles map> <counts> <seen> -- tallies each hit that
    # holds a needle its own census gives for its file. Returns the hits read.
    function count(text, bf, tally, seen, side,   n, i, m, j, nl, held) {
      n = split(text, hl_, "\n")
      for (i = 1; i <= n; i++) {
        if (hl_[i] == "") continue
        parse(hl_[i])
        if (comment(ht)) continue
        if (ht !~ re) { printf "UNREAD\t%s:%s:%s\n", hf, hl, ht; continue }
        m = split(bf[hf], nl, "\n"); held = 0
        for (j = 1; j <= m; j++) if (nl[j] != "" && index(ht, nl[j])) {
          held = 1; seen[hf "\t" nl[j]] = 1; tally[nl[j]]++
        }
        if (!held && side == "index") printf "NEW\t%s:%s:%s\n", hf, hl, ht
      }
    }
    BEGIN {
      re = ENVIRON["ID_PATTERN_RE"]
      nr = load(ENVIRON["CENSUS_TEXT"], rf, rn, rk)
      for (i = 1; i <= nr; i++) byfile[rf[i]] = byfile[rf[i]] "\n" rn[i]
      count(ENVIRON["INDEX_HITS"], byfile, now, seen, "index")
      for (i = 1; i <= nr; i++)
        if (!((rf[i] "\t" rn[i]) in seen)) printf "STALE\t%s\t%s\n", rf[i], rn[i]
      if (ENVIRON["HEAD_CENSUS"] == "NOCENSUS") exit
      hr = load(ENVIRON["HEAD_CENSUS"], hf_, hn_, hk)
      for (i = 1; i <= hr; i++) headbyfile[hf_[i]] = headbyfile[hf_[i]] "\n" hn_[i]
      count(ENVIRON["HEAD_HITS"], headbyfile, before, headseen, "head")
      for (n in now) if (now[n] > before[n] + 0) printf "GROWN\t%s\t%d\t%d\n", n, before[n] + 0, now[n]
      for (i = 1; i <= nr; i++) {
        if ((rf[i] "\t" rn[i]) in hk) continue
        moved = 0
        for (j = 1; j <= hr; j++)
          if (hn_[j] == rn[i] && !((hf_[j] "\t" hn_[j]) in rk) && !((hf_[j] "\t" hn_[j]) in seen)) moved = 1
        if (!moved) printf "ADDED\t%s\t%s\n", rf[i], rn[i]
      }
    }
  '
}

# judge -- reads the index and HEAD of the repository gitx names, and prints
# the findings. Its status is 0 when it could read everything it rests on,
# whatever it found, and 2 otherwise: it runs in a command substitution, so a
# die() inside ends only the substitution, and its caller reads the status. An
# empty answer is never taken for a clean one.
judge() {
  local census index head="" headcensus=NOCENSUS listing out
  census="$(gitx show ":$CENSUS")" || die "the census $CENSUS is not in the index, so no line can be judged"
  awk -F'\t' '!/^#/ && NF && (NF != 4 || $1 == "" || $2 == "" || $3 == "" || $4 == "") { bad = 1; print "id-pattern: error: census row " NR " needs four non-empty tab-separated fields: " $0 > "/dev/stderr" } END { exit bad }' <<<"$census" ||
    return 2
  index="$(hits "")" || die "git grep could not read the index"
  if gitx rev-parse -q --verify HEAD >/dev/null; then
    listing="$(gitx ls-tree HEAD -- "$CENSUS")" || die "git could not list HEAD's tree"
    if [ -n "$listing" ]; then
      headcensus="$(gitx show "HEAD:$CENSUS")" || die "git could not read the census at HEAD"
      head="$(hits HEAD)" || die "git grep could not read HEAD"
    else
      echo "id-pattern: the census is new in this commit, so there is nothing to grow from or add to" >&2
    fi
  elif gitx symbolic-ref -q HEAD >/dev/null; then
    echo "id-pattern: no commit yet, so there is nothing to grow from or add to" >&2
  else
    die "git could not read HEAD"
  fi
  out="$(verdict "$census" "$index" "$headcensus" "$head")" || return 2
  if grep -q '^UNREAD' <<<"$out"; then
    grep '^UNREAD' <<<"$out" | sed 's/^UNREAD./id-pattern: error: git grep and awk read this line differently: /' >&2
    return 2
  fi
  [ -z "$out" ] || printf '%s\n' "$out"
  return 0
}

# ---------------------------------------------------------------------------
# THE CONTROL. The fixtures are built from pieces, so no line of this file
# spells a pattern it would find. Every expectation is derived from what is
# planted: the new patterns' lines from the file's length before they land.
# ---------------------------------------------------------------------------
control() {
  local d='[0-9]' bs="\\" st="S""T" wp="W""P" wpl="w""p" ac="A""C" at="A""T" iss="ISS""UE:" isl="iss""ues/"
  local file=bin/planted.sh deep="intent/st/ST0001/parity/tools/deep.sh" out err base i want got
  local listed="grep -c '^${st}${d}+' list" listed_needle="^${st}${d}+" stale_needle="gone-from-its-file"
  # One planted line for each form the REACH says the detector knows.
  local -a fresh=(
    "grep -E '${st}${d}+' f" "grep -P '${st}${bs}d+' f" "for t in ${st}*; do :; done"
    "grep -E '${st}{1}' f" "sed -E 's/(${st}(${d}+))/x/' f" "case \$x in ${st}????) ;; esac"
    "printf '${st}%04d' 1"
    "grep -E '${wp}-${d}' f" "grep -E '${wpl}/${d}' f" "grep -P '${wp}/${bs}d' f" "ls ${wp}/*"
    "grep -E '${wp}/(${d})' f" "case \$x in ${wp}-?) ;; esac" "printf '${wp}-%02d' 1"
    "grep -E '${ac}-${d}' f" "grep -P '${at}-${bs}d' f" "grep -E '${ac}-(${d})' f"
    "case \$x in ${at}-*) ;; esac" "printf '${at}-%02d' 1"
    "grep -E '${iss}${d}' f" "case \$x in ${iss}*) ;; esac" "printf '${iss}%04d' 1"
    "ls ${isl}${d}" "printf '${isl}%04d' 1"
    "grep -E '^${d}{4} ' f" "grep -E '^${d}{3,4}' f" "grep -E '^${d}{4,}' f" "grep '^${d}${d}${d}${d} ' f"
    "grep -E '^[[:digit:]]""{4} ' f" "grep -P '^${bs}d{4} ' f" "grep -P '^${bs}d${bs}d${bs}d${bs}d ' f"
  )
  local -a plain=("echo 'see ST0056 and issue 0731'" "date +%Y-%m-%d" "printf '%04d' 7" "echo \"\${${st}[@]}\"")
  SCRATCH="$(mktemp -d)" || die "control: no temporary directory for its scratch repository"
  trap 'rm -rf "$SCRATCH"' EXIT
  err="$(
    {
      gitx init -q &&
        gitx config user.name control && gitx config user.email control@localhost &&
        mkdir -p "$SCRATCH/bin" "$SCRATCH/$(dirname "$CENSUS")" "$SCRATCH/$(dirname "$deep")" &&
        printf '#!/usr/bin/env bash\n%s\n' "$listed" >"$SCRATCH/$file" &&
        printf '%s\t%s\tcontrol\tcontrol\n%s\t%s\tcontrol\tcontrol\n' \
          "$file" "$listed_needle" "$file" "$stale_needle" >"$SCRATCH/$CENSUS" &&
        gitx add -A && gitx commit -q -m base
    } 2>&1
  )" || die "control: could not build its scratch repository: $err"
  base="$(wc -l <"$SCRATCH/$file" | tr -d ' ')"
  printf '%s\n' "${fresh[@]}" "${plain[@]}" "$listed" >>"$SCRATCH/$file"
  printf '#!/usr/bin/env bash\n%s\n' "${fresh[0]}" >"$SCRATCH/$deep"
  err="$(gitx add -A 2>&1)" || die "control: could not stage its planted lines: $err"
  out="$(judge)" || die "control: the judgement of its scratch repository could not read what it rests on"
  rm -rf "$SCRATCH"
  trap - EXIT
  SCRATCH=""
  want=""
  for ((i = base + 1; i <= base + ${#fresh[@]}; i++)); do want="${want}${want:+ }${file}:$i"; done
  want="${want} ${deep}:$(($(printf '#!/usr/bin/env bash\n' | wc -l) + 1))"
  got="$(awk -F'\t' '$1 == "NEW" { split($2, p, ":"); printf "%s%s:%s", s, p[1], p[2]; s = " " }' <<<"$out")"
  CONTROL_WANT="$want" CONTROL_GOT="$got"
  CONTROL_STALE="$(awk -F'\t' '$1 == "STALE" { print $3 }' <<<"$out")"
  CONTROL_GROWN="$(awk -F'\t' '$1 == "GROWN" { print $2 }' <<<"$out")"
  [ "$got" = "$want" ] && [ "$CONTROL_STALE" = "$stale_needle" ] && [ "$CONTROL_GROWN" = "$listed_needle" ]
}

CONTROL_WANT=""
CONTROL_GOT=""
CONTROL_STALE=""
CONTROL_GROWN=""
if ! control; then
  echo "id-pattern: CONTROL FAILED -- the planted new patterns stand at ${CONTROL_WANT:-?} and were found at ${CONTROL_GOT:-none}; the planted stale row was found as: ${CONTROL_STALE:-nothing}; the planted copy grew: ${CONTROL_GROWN:-nothing}. No verdict is offered on this repository." >&2
  exit 2
fi
echo "id-pattern: control -- a planted new pattern for each form the detector knows was found where it stands, and one under the glob root; the planted stale row and the planted copy were found; the listed site and the non-patterns were not. Every arm fired."

# ---------------------------------------------------------------------------
# The real verdict, on this repository's index and HEAD.
# ---------------------------------------------------------------------------
findings="$(judge)" || die "the judgement of this repository could not read what it rests on; its message is above"

if [ -z "$findings" ]; then
  echo "id-pattern: every line that spells an id's shape is a census site, every census row still names its site, and the census did not grow since HEAD."
  exit 0
fi

echo "id-pattern: FINDING --"
awk -F'\t' '
  $1 == "NEW" { sub(/^NEW\t/, ""); print "    " $0 "\n      spells the shape of an id, and the census names no needle on it" }
  $1 == "STALE" { print "    census row " $2 " :: " $3 "\n      its needle is on no line of its file: remove the row, the census only shrinks" }
  $1 == "GROWN" { print "    needle " $2 " is on " $4 " line(s) of its rows, where HEAD had " $3 "\n      a copy or a new site: the census only shrinks, and a moved site keeps its count" }
  $1 == "ADDED" { print "    census row added: " $2 " :: " $3 "\n      the census only shrinks: a row is added only as a move, its HEAD row and site gone" }
' <<<"$findings"
cat <<'REMEDY'

  Ask the binary's id door (ST0095 WP-10) instead of spelling an id's shape.
  Until the build the gate runs carries that door, say so where the step asks
  it, and never read its absence as an answer. Moving an existing site moves its
  census row in the same commit.
REMEDY
exit 1
