#!/bin/bash
# sparse_tree_equals_manifest.sh -- after `organize`, the tree holds only what
# `.intentfiles` names.
#
# COVERS ST0057 AC-04.6, AND IS CITED BY AT-04.6. Both ids are spelled out
# because the row and the file each look correct alone and only the PAIR can be
# wrong -- the same reason `intentfiles_reviewable.sh` states both.
#
# WHY THE ROW EXISTS AT ALL: EVERY ONE OF AC-04.1 THROUGH AC-04.5 IS SATISFIED
# BY AN `organize` THAT REMOVES NOTHING. Report-never-remove, refuse-on-hand-
# edit, never-resolve-a-divergence, run-twice-changes-nothing, refuse-on-digest-
# change -- all five are BRAKES. Nothing among them asserts the verb ever
# reconciles anything, so this is the one row that measures the outcome rather
# than the safety.
#
# AN EQUALITY, NEVER A CONTAINMENT. A subset check passes on a tree that removed
# nothing; a superset check passes on an empty one. Both differences are printed,
# in both directions, NAMED -- present-but-undeclared and declared-but-absent are
# different defects with different fixes, and a single "3 mismatches" line sends
# the reader to work out which they have.
#
# TWO INSTRUMENTS, NEVER ONE SET COMPARED TO ITSELF. Disk is derived by LISTING
# THE FILESYSTEM; declared is derived by PARSING `.intentfiles`. **The parse
# below is a second spelling of a grammar Rust already owns, and that is the
# point rather than an oversight**: an instrument that asked `intentsvcs` what
# the manifest declares would be asking the subject to grade itself, and would
# agree with it by construction on exactly the day the parser was wrong. The
# cost is real and is paid deliberately -- so this parser REFUSES a line it
# cannot read (exit 2) rather than skipping it, because a skipped line silently
# drops an artefact from the declared set and produces a tree that looks
# reconciled.
#
# THE UNIT IS THE ARTEFACT, BECAUSE THE MANIFEST NAMES ARTEFACTS AND NEVER
# FILES. `.intentfiles`'s own header says so. So both sets are sets of THREAD
# IDS, and a thread counts as on disk when its directory holds at least one
# file -- an empty directory is a different state from a realised one and is
# reported separately rather than folded into either side.
#
# DENOMINATOR CAUTION, INHERITED FROM ic 2026-08-18 AND STATED RATHER THAN
# ASSUMED: a `find | wc -l` over a tree counts directory nodes as well as files
# unless filtered, and filtering with `-type f` BLINDS the count to a deletion,
# because the directory node is the only input whose mtime records one. This
# tool counts neither raw total: it counts ARTEFACTS, by name, and prints the
# names. A count nobody can expand into a list is the one that goes wrong
# quietly.
#
# THE EXEMPT SET IS ASSERTED, NOT SKIPPED. `intent/st/steel_threads.md` is an
# index view: renderer-produced, naming no single artefact, so no manifest entry
# can imply it. It is printed as an EXEMPTION WITH ITS REASON on every run, so
# an exemption silently growing is visible -- the shrunken-roster failure that
# took ST0056 AT-03.15 five versions to close. Anything else directly under
# `intent/st/` that is not a thread directory is a FINDING, not a new exemption.
#
# THE RED IS SYNTHETIC AND THE POPULATION IS LIVE, AND NEITHER HALF SUBSTITUTES
# FOR THE OTHER (vc's rewrite of this row, 2026-08-20, after the original
# red-first clause named a baseline `organize --apply` had already consumed at
# `e7f00e65` -- 57 thread directories and no manifest, gone by the time anyone
# could run this). `--self-test` plants one undeclared file and requires red,
# removes it and requires green, then declares an absent thread and requires red
# the other way. That discrimination is a property of THIS SCRIPT and must not
# borrow a defect from the live estate, or the estate is not free to fix it.
# **And every ordinary run reports the live population**, because a synthetic
# green over an unstated denominator is the vacuous pass in the other direction:
# it proves the tool can see one planted file and says nothing about whether it
# looked at the estate at all.
#
# EXIT CODES. 0 the two sets are equal, 1 they differ, 2 CANNOT MEASURE -- an
# unreadable manifest line, a missing `intent/st`, or a zero denominator. The
# third is not decoration: a run over an estate with no threads would report
# equality of two empty sets and mean nothing.
#
# IT NEVER WRITES TO THE LIVE ESTATE. The ordinary run is read-only. The
# self-test builds its own tree in a temp directory and removes it. Four nodes
# work in this checkout; an instrument that mutates it is a second writer with
# the worst possible timing.

set -uo pipefail

say() { printf '%s\n' "$*"; }
die() {
  printf 'sparse_tree_equals_manifest: cannot measure -- %s\n' "$1" >&2
  exit 2
}

# The one path that is renderer-produced and names no artefact, with the reason
# carried beside it rather than in a comment -- it is printed on every run.
EXEMPT_NAME="steel_threads.md"
EXEMPT_WHY="an index view: renderer-produced, but names no single artefact, so no manifest entry can imply it"

# --------------------------------------------------------------------------
# INSTRUMENT ONE: the filesystem.
# --------------------------------------------------------------------------
# Thread ids that have a realised form on disk, one per line. A directory with
# no file in it is NOT realised and is reported by the caller as its own state.
disk_realised() {
  local st_dir="$1" d id
  for d in "$st_dir"/*/; do
    [ -d "$d" ] || continue
    id="$(basename "$d")"
    # `find -type f` is correct HERE and only here: the question is whether the
    # directory holds any file at all, not how many, so a deletion changing the
    # count is not something this line has to notice.
    if [ -n "$(find "$d" -type f -print -quit 2>/dev/null)" ]; then
      printf '%s\n' "$id"
    fi
  done
}

# Directories under `intent/st/` that exist and hold no file. Neither realised
# nor absent, and folding them into either side would misreport a half-removed
# thread as a clean one.
disk_empty_dirs() {
  local st_dir="$1" d
  for d in "$st_dir"/*/; do
    [ -d "$d" ] || continue
    if [ -z "$(find "$d" -type f -print -quit 2>/dev/null)" ]; then
      printf '%s\n' "$(basename "$d")"
    fi
  done
}

# Files directly under `intent/st/` that are not thread directories. Exactly one
# is exempt; anything else is a finding.
disk_strays() {
  local st_dir="$1" f n
  for f in "$st_dir"/*; do
    [ -f "$f" ] || continue
    n="$(basename "$f")"
    [ "$n" = "$EXEMPT_NAME" ] && continue
    printf '%s\n' "$n"
  done
}

# --------------------------------------------------------------------------
# INSTRUMENT TWO: the manifest's grammar.
# --------------------------------------------------------------------------
# `<SIGIL>:<ID>` with an optional trailing `# comment`; blank lines, whole-line
# comments and BEGIN/END markers are structure. ANYTHING ELSE STOPS THE RUN with
# its line number -- the manifest's own header promises that, and a skipped line
# is an artefact silently dropped from the declared set.
declared_from_manifest() {
  local file="$1" n=0 line body id
  while IFS= read -r line || [ -n "$line" ]; do
    n=$((n + 1))
    body="${line%%#*}"
    body="$(printf '%s' "$body" | tr -d '[:space:]')"
    [ -z "$body" ] && continue
    case "$body" in
      STEELTHREAD:*)
        id="${body#STEELTHREAD:}"
        case "$id" in
          ST[0-9][0-9][0-9][0-9]) printf '%s\n' "$id" ;;
          *) die "line $n of $file: \`$id\` is not a steel-thread id (ST followed by four digits)" ;;
        esac
        ;;
      *) die "line $n of $file: \`$line\` is not \`<SIGIL>:<ID>\`" ;;
    esac
  done <"$file"
}

# --------------------------------------------------------------------------
# THE MEASUREMENT.
# --------------------------------------------------------------------------
# Takes a project root so the self-test can drive the same code over a tree it
# built. A self-test that reimplemented this would be measuring itself.
measure() {
  # SPLIT RATHER THAN CHAINED. `local a=$1 b=$a` does not see `a` under bash's
  # `local`, and under `set -u` that is an unbound-variable abort rather than an
  # empty string -- which is the good failure, and is how this was caught on the
  # first self-test run instead of by a path that silently resolved to `/`.
  local root="$1"
  local st_dir="$root/intent/st"
  local manifest="$root/intent/.intentfiles"
  [ -d "$st_dir" ] || die "no $st_dir -- there is no realised estate to measure"

  local on_disk declared empty strays
  on_disk="$(disk_realised "$st_dir" | sort)"
  empty="$(disk_empty_dirs "$st_dir" | sort)"
  strays="$(disk_strays "$st_dir" | sort)"

  if [ -f "$manifest" ]; then
    declared="$(declared_from_manifest "$manifest" | sort -u)" || exit 2
  else
    # **ABSENT IS NOT EMPTY** (hv). Nobody has said, so everything realised is
    # declared -- and the equality is trivially true rather than measurable.
    # Reported as cannot-measure rather than as a pass, because a green here
    # would be this tool's own vacuous-denominator failure.
    say "sparse_tree_equals_manifest: no $manifest."
    say "  ABSENT IS NOT EMPTY: nobody has said, so everything on disk is declared and"
    say "  the equality holds by definition. There is nothing here to measure."
    exit 2
  fi

  local n_disk n_declared
  n_disk="$(printf '%s' "$on_disk" | grep -c . || true)"
  n_declared="$(printf '%s' "$declared" | grep -c . || true)"

  say "sparse_tree_equals_manifest: on_disk=$n_disk declared=$n_declared  (root: $root)"
  say "  exempt: $EXEMPT_NAME -- $EXEMPT_WHY"

  # THE LIVE POPULATION, PRINTED. A synthetic green over an unstated
  # denominator says nothing about whether the estate was looked at.
  say "  realised on disk: $(printf '%s' "$on_disk" | tr '\n' ' ')"
  say "  declared: $(printf '%s' "$declared" | tr '\n' ' ')"

  if [ "$n_disk" -eq 0 ] && [ "$n_declared" -eq 0 ]; then
    die "both sets are empty -- an equality of two empty sets is not evidence"
  fi

  local undeclared absent rc=0
  undeclared="$(comm -23 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$declared") | grep -c . || true)"
  absent="$(comm -13 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$declared") | grep -c . || true)"

  if [ "$undeclared" -gt 0 ]; then
    rc=1
    say "  PRESENT BUT UNDECLARED ($undeclared) -- \`organize --apply\` has not removed these, or the manifest lost a line:"
    comm -23 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$declared") | sed 's/^/    /'
  fi
  if [ "$absent" -gt 0 ]; then
    rc=1
    say "  DECLARED BUT ABSENT ($absent) -- the manifest names artefacts that have no realised form; \`organize --apply\` would hydrate them:"
    comm -13 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$declared") | sed 's/^/    /'
  fi
  if [ -n "$empty" ]; then
    rc=1
    say "  DIRECTORY PRESENT AND EMPTY -- neither realised nor removed, so a half-finished dehydration:"
    printf '%s\n' "$empty" | sed 's/^/    /'
  fi
  if [ -n "$strays" ]; then
    rc=1
    say "  UNDER intent/st/ AND NOT A THREAD -- a new exemption must be DECLARED in this tool, never inferred:"
    printf '%s\n' "$strays" | sed 's/^/    /'
  fi

  [ "$rc" -eq 0 ] && say "  the two sets are EQUAL."
  return "$rc"
}

# --------------------------------------------------------------------------
# SELF-TEST -- synthetic, because the discrimination is a property of this
# script and must never borrow a defect from the live estate.
# --------------------------------------------------------------------------
fixture() {
  local root="$1" ids="$2" declared="$3" id
  mkdir -p "$root/intent/st"
  for id in $ids; do
    mkdir -p "$root/intent/st/$id"
    printf '# %s\n' "$id" >"$root/intent/st/$id/info.md"
  done
  printf '# a fixture manifest\n' >"$root/intent/.intentfiles"
  for id in $declared; do
    printf 'STEELTHREAD:%s\n' "$id" >>"$root/intent/.intentfiles"
  done
}

selftest() {
  local fails=0
  # NOT `local`: the EXIT trap fires after this function has returned, when a
  # local is already out of scope -- so the cleanup would run against an unbound
  # name and, without `set -u`, against an empty one. `rm -rf ""` is the version
  # of this bug that does nothing; `rm -rf "$tmp"/` with tmp empty is the version
  # that does not.
  SELFTEST_TMP="$(mktemp -d)" || die "cannot make a temp directory"
  trap 'rm -rf "${SELFTEST_TMP:?}"' EXIT
  local tmp="$SELFTEST_TMP"

  # 1. EQUAL -> green.
  fixture "$tmp/a" "ST0001 ST0002" "ST0001 ST0002"
  if measure "$tmp/a" >/dev/null; then say "  selftest: equal sets -> green   OK"; else
    say "  selftest: equal sets -> green   FAIL"; fails=1; fi

  # 2. PLANT ONE UNDECLARED FILE -> red. The synthetic red this row requires.
  fixture "$tmp/b" "ST0001 ST0002 ST0003" "ST0001 ST0002"
  if measure "$tmp/b" >/dev/null; then
    say "  selftest: planted undeclared thread -> red   FAIL (it passed)"; fails=1
  else say "  selftest: planted undeclared thread -> red   OK"; fi

  # 3. REMOVE IT -> green again. Without this the tool could be one that always
  #    goes red, which arm 2 alone cannot tell apart from a working one.
  rm -rf "$tmp/b/intent/st/ST0003"
  if measure "$tmp/b" >/dev/null; then say "  selftest: undeclared removed -> green   OK"; else
    say "  selftest: undeclared removed -> green   FAIL"; fails=1; fi

  # 4. THE OTHER DIRECTION: declared and absent. A tool that only saw surplus
  #    would pass on an estate that had deleted half of what it declares.
  fixture "$tmp/c" "ST0001" "ST0001 ST0004"
  if measure "$tmp/c" >/dev/null; then
    say "  selftest: declared but absent -> red   FAIL (it passed)"; fails=1
  else say "  selftest: declared but absent -> red   OK"; fi

  # 5. AN UNREADABLE LINE STOPS THE RUN rather than dropping an artefact.
  fixture "$tmp/d" "ST0001" "ST0001"
  printf 'NONSENSE\n' >>"$tmp/d/intent/.intentfiles"
  # **A SUBSHELL, BECAUSE `die` CALLS `exit` AND A REDIRECTION DOES NOT CONTAIN
  # ONE.** The first version wrote `measure ... >/dev/null 2>&1` and the exit
  # took the whole self-test with it -- four arms had already printed OK, so it
  # read as a pass with a stray rc rather than as an arm that never ran.
  local rc=0
  ( measure "$tmp/d" ) >/dev/null 2>&1 || rc=$?
  if [ "$rc" -eq 2 ]; then say "  selftest: unreadable manifest line -> exit 2   OK"; else
    say "  selftest: unreadable manifest line -> exit 2   FAIL (rc=$rc)"; fails=1; fi

  return "$fails"
}

main() {
  if [ "${1:-}" = "--self-test" ]; then
    say "sparse_tree_equals_manifest: SELF-TEST"
    if selftest; then say "sparse_tree_equals_manifest: SELFTEST PASS"; exit 0; fi
    say "sparse_tree_equals_manifest: SELFTEST FAIL"; exit 1
  fi
  local root="${1:-$(git rev-parse --show-toplevel 2>/dev/null)}"
  [ -n "$root" ] || die "no root given and not inside a git repository"
  measure "$root"
}

main "$@"
