#!/bin/bash
# estate_corpus.sh -- capture the v2 estate the migrator is run against, and make it answer for itself.
#
# AC-10.5. The migrator (WP-10) has to be exercised against a real v2 estate, and
# the three conservation checks have to be computed against the SAME estate the
# migrator saw. This is where that estate comes from.
#
# THE SPLIT THIS SERVES, BECAUSE IT IS WHAT MAKES THE CHECK NON-VACUOUS. cc's
# migrator enumerates what it PRODUCED; the census beside this file enumerates
# what the estate CONTAINED. Neither side is completable from the other, and the
# two failure directions do not overlap: the corpus side goes wrong by being too
# SMALL (an artefact the estate never held, so a migrator branch ships
# unexercised), the migrator side by being too NARROW (one it held and the
# migrator did not convert). Neither error can hide in the other's green. The
# corpus author therefore cannot mark the corpus correct -- if this
# misrepresents a real v2 estate, the migrator passes here and fails on the real
# thing, and the migrator's author is who finds out.
#
# WHY A NAMED COMMIT AND NEVER A WORKTREE. Four sessions write this repository
# concurrently and `intent/whiteboard/` changes every few minutes, so a corpus
# read from the worktree is a subject that moves under the instrument measuring
# it. Worse, it moves in ways nothing reports: on 2026-08-17 the issue estate
# went from 23 OPEN to 0 in eleven minutes (`9a9c7799`, then `fef54072`), so two
# runs an hour apart would have measured two different populations and agreed
# about nothing except the number.
#
# WHY THE CANARY IS PINNED AT `42fb5269` AND NOT AT HEAD. Stated as a criterion
# rather than as a date, because "the newest revision" is not a selection: it is
# THE LAST COMMITTED REVISION AT WHICH `intent/issues/` POPULATES BOTH ARMS --
# 23 OPEN and 38 CLOSED. At HEAD the estate holds 0 OPEN and 40 CLOSED, and the
# migrator's design routes issue findings to BLOCK in live work and CARRY in
# closed. A corpus captured at HEAD offers the BLOCK arm no input at all, so
# that arm ships unexercised while every count reconciles perfectly against
# zero -- the same wrong-zero that let `legacy.rs` never read the issue estate
# without a single number disagreeing. Measured by dc, 2026-08-17.
#
# WHAT THE CAPTURE FILE IS AND IS NOT. `CAPTURE` records the claim -- which
# member, which revision, and the exact command that produced the directory. It
# is NOT the thing `verify` trusts. Verify recomputes `git hash-object` over
# every captured file and compares it to `git ls-tree` at the pinned revision,
# so the corpus's identity is derived from its own bytes and checked against
# git's independent, immutable record of the same tree. There is no stored
# expected-hash list anywhere, because a record written by the same act that
# writes the files agrees with them for reasons that have nothing to do with
# being right (`self_provenance_check.sh`, and the manifest that cost a day).
#
# CAPTURE VERIFIES BEFORE IT RETURNS. A guard positioned after the write can
# only catch the subset of fabrications that are also implausible, so this one
# runs inside the act: a capture that cannot verify is not reported as a capture
# that needs checking, it fails.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

die() {
  echo "estate-corpus: $*" >&2
  exit 2
}

# The fleet AC-10.5 names, in full. The three that cannot be captured from this
# checkout are DECLARED and marked, never omitted: a table holding one member
# reads exactly like a table holding the set, and scope belongs in a
# denominator rather than in an adjective.
#
# id | repo | revision | capturable | why this revision, or why not
members() {
  cat <<'EOF'
canary|.|42fb5269|yes|the last committed revision at which intent/issues/ populates BOTH arms (23 OPEN + 38 CLOSED); at HEAD the OPEN arm is empty and the migrator's BLOCK path would ship unexercised
lamplight|-|-|no|AC-10.5 names it at a "post-sweep revision"; the sweep program was ruled off, no revision was ever named, and the repository is not reachable from this checkout
utilz|-|-|no|as lamplight: no revision named, not reachable from this checkout
baize|-|-|no|as lamplight: no revision named, not reachable from this checkout
EOF
}

member_row() {
  members | awk -F'|' -v id="$1" '$1 == id { print; found = 1 } END { exit !found }'
}

cmd_list() {
  printf '%-10s %-10s %-12s %s\n' MEMBER CAPTURABLE REVISION CRITERION
  members | while IFS='|' read -r id repo rev capturable why; do
    printf '%-10s %-10s %-12s %s\n' "$id" "$capturable" "$rev" "$why"
  done
}

# The estate is the whole of `intent/`. Not a curated subset: the awkward files
# are the point. ST0056's own directory carries generated parity artefacts
# (.tap, .tsv, .sh) that are not v2 canon at all, `intent/whiteboard/` is 98
# files no v3 model has a place for, and both must be NAMED as out-of-model
# rather than quietly absent. A corpus trimmed to what converts cleanly would
# test the migrator against the half of the estate that was never in doubt.
SUBTREE="intent/"

cmd_capture() {
  local id="${1:-}" dest="${2:-}"
  [ -n "$id" ] || die "usage: estate_corpus.sh capture <member> [dest]"

  local row repo rev capturable why
  row="$(member_row "$id")" || die "unknown member: $id (try: estate_corpus.sh list)"
  IFS='|' read -r _ repo rev capturable why <<EOF
$row
EOF

  [ "$capturable" = yes ] || die "member $id is declared and NOT capturable from here -- $why"

  local src
  src="$(cd "$ROOT/$repo" 2>/dev/null && pwd)" || die "member $id names repo $repo, which does not resolve from $ROOT"
  git -C "$src" rev-parse --git-dir >/dev/null 2>&1 || die "$src is not a git repository"

  local full
  full="$(git -C "$src" rev-parse --verify "$rev^{commit}" 2>/dev/null)" ||
    die "revision $rev is not in $src -- a pinned corpus revision that has left history is not a corpus"

  dest="${dest:-$ROOT/tmp/corpus/$id}"

  # Never remove a directory this tool did not create. The ownership marker is
  # the CAPTURE file, so a mistyped path fails loudly instead of deleting
  # someone's work.
  if [ -e "$dest" ]; then
    if [ -f "$dest/CAPTURE" ]; then
      rm -rf "$dest" || die "cannot replace prior capture at $dest"
    else
      die "$dest exists and carries no CAPTURE file -- refusing to remove a directory this tool did not create"
    fi
  fi
  mkdir -p "$dest" || die "cannot create $dest"

  # `git archive` rather than a checkout: it reads the object store directly, so
  # nothing in the working tree of $src can reach the captured bytes.
  git -C "$src" archive --format=tar "$full" "$SUBTREE" | (cd "$dest" && tar -xf -) ||
    die "git archive failed for $full $SUBTREE"

  local now
  now="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
  {
    echo "# Corpus capture -- the CLAIM under test, not the answer."
    echo "#"
    echo "# \`verify\` does not trust any line in this file except the revision, and"
    echo "# it checks that revision by recomputing every captured file's hash against"
    echo "# git's own record of the same tree. Editing a line here changes what is"
    echo "# claimed; it cannot change what the bytes are."
    echo
    echo "member: $id"
    echo "repo: $repo"
    echo "revision: $full"
    echo "subtree: $SUBTREE"
    echo "captured_at: $now"
    echo "criterion: $why"
    echo
    echo "# Reproduce exactly this directory:"
    echo "#   $(basename "$0") capture $id <dest>"
    echo "# Or without this tool:"
    echo "#   git -C <repo> archive --format=tar $full $SUBTREE | (cd <dest> && tar -xf -)"
  } >"$dest/CAPTURE" || die "cannot write $dest/CAPTURE"

  # The guard runs inside the act, not after it.
  cmd_verify "$dest" || die "capture wrote $dest and it does NOT verify -- the directory is left in place for inspection"

  echo "captured: $id at $full -> $dest"
}

cmd_verify() {
  local dir="${1:-}" rev="${2:-}"
  [ -n "$dir" ] || die "usage: estate_corpus.sh verify <dir> [revision]"
  [ -d "$dir" ] || die "no such directory: $dir"

  local capture="$dir/CAPTURE"
  local repo
  if [ -z "$rev" ]; then
    [ -f "$capture" ] || die "$dir carries no CAPTURE file and no revision was given -- a corpus that cannot name its revision cannot be verified"
    rev="$(awk '$1 == "revision:" { print $2 }' "$capture")"
    [ -n "$rev" ] || die "$capture carries no revision: line"
  fi
  repo="$(awk '$1 == "repo:" { print $2 }' "$capture" 2>/dev/null)"
  repo="${repo:-.}"

  local src
  src="$(cd "$ROOT/$repo" 2>/dev/null && pwd)" || die "CAPTURE names repo $repo, which does not resolve from $ROOT"
  git -C "$src" rev-parse --verify "$rev^{commit}" >/dev/null 2>&1 ||
    die "revision $rev is not in $src"

  # The referent: git's record of that tree. Nothing derived from $dir.
  local listing
  listing="$(git -C "$src" ls-tree -r "$rev" "$SUBTREE")" ||
    die "cannot read tree $rev in $src"
  [ -n "$listing" ] || die "tree $rev holds nothing under $SUBTREE -- an empty referent would pass an empty corpus"

  local declared=0 matched=0 missing=0 diverged=0 nonblob=0
  local mode type sha path actual
  local tmp
  tmp="$(mktemp)" || die "cannot create a scratch file"
  printf '%s\n' "$listing" >"$tmp"

  while IFS=$'\t' read -r meta path; do
    read -r mode type sha <<EOF
$meta
EOF
    declared=$((declared + 1))
    # A non-blob entry (submodule, or a symlink whose target this tool does not
    # reproduce byte-for-byte) is REFUSED rather than skipped. A silently
    # skipped entry is precisely the disappearance this whole check exists to
    # make impossible.
    if [ "$type" != blob ]; then
      echo "NONBLOB $path ($type) -- not reproducible by this capture"
      nonblob=$((nonblob + 1))
      continue
    fi
    if [ ! -f "$dir/$path" ]; then
      echo "MISSING $path"
      missing=$((missing + 1))
      continue
    fi
    actual="$(git -C "$src" hash-object -- "$dir/$path" 2>/dev/null)"
    if [ "$actual" = "$sha" ]; then
      matched=$((matched + 1))
    else
      echo "DIVERGED $path (tree $sha, file ${actual:-unhashable})"
      diverged=$((diverged + 1))
    fi
  done <"$tmp"
  rm -f "$tmp"

  # The other direction. A file present in the capture and absent from the tree
  # is an ADDITION, and an addition to a corpus is how a fixture stops being the
  # estate it claims to be. CAPTURE itself is this tool's own artefact and is
  # the one expected extra.
  #
  # SET COMPARISON RATHER THAN A GREP PER FILE, AND THE REASON IS A DEFECT THIS
  # TOOL SHIPPED FOR ONE RUN. The first draft asked `printf "$listing" | grep -qF
  # <path>` once per captured file. Under `set -o pipefail` that pipeline
  # reports the WRITER's death, not the reader's answer: `grep -q` exits the
  # instant it matches, `printf` takes SIGPIPE (141), pipefail promotes 141 to
  # the pipeline's status, and a FOUND path is reported as EXTRA. It fired on
  # 395 of 1077 files -- every one of them an early match, because a late match
  # lets printf finish first -- while the same run reported 1077/1077 matched in
  # the other direction. A green and a false alarm from one loop over one set.
  local extra=0 dirlist treelist
  dirlist="$(mktemp)" && treelist="$(mktemp)" || die "cannot create scratch files"
  (cd "$dir" && find . -type f) | sed 's|^\./||' | grep -v '^CAPTURE$' | sort >"$dirlist"
  printf '%s\n' "$listing" | cut -f2- | sort >"$treelist"
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    echo "EXTRA $f"
    extra=$((extra + 1))
  done < <(comm -23 "$dirlist" "$treelist")
  rm -f "$dirlist" "$treelist"

  # The matched count is reported on every run, pass or fail. A check that only
  # speaks when it fails cannot be told from a check that never ran.
  echo "corpus: $matched/$declared match at $rev (missing $missing, diverged $diverged, extra $extra, non-blob $nonblob)"

  # A run that declared files and matched NONE is the instrument failing, not a
  # corpus wrong in every file. Exit 2 says so, because a wrong maximum is more
  # persuasive than a wrong zero -- it looks like diligence rewarded.
  if [ "$declared" -gt 0 ] && [ "$matched" -eq 0 ]; then
    die "declared $declared files and matched none -- this is the check failing, not the corpus"
  fi
  [ $((missing + diverged + extra + nonblob)) -eq 0 ]
}

case "${1:-}" in
  list) shift; cmd_list "$@" ;;
  capture) shift; cmd_capture "$@" ;;
  verify) shift; cmd_verify "$@" ;;
  *) die "usage: estate_corpus.sh {list|capture <member> [dest]|verify <dir> [revision]}" ;;
esac
