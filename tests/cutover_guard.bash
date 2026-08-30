#!/usr/bin/env bash
#
# cutover_guard.bash -- AT-12.1, covering AC-12.1.
#
# THE PROPERTY, WHICH IS THE SUBJECT: no file surviving the prune depends on
# `bin/` -- neither by SOURCING `bin/intent_helpers` nor by CALLING a symbol
# only `bin/` defines. This file is an instrument for that property and is not
# the property. AT-12.1 names the property deliberately, because nobody
# re-derives a claim once there is a tool to run.
#
# TWO EDGES, AND THE SECOND IS INVISIBLE TO ANY SWEEP SPELLED `source`.
# `claude_plugin_helpers.sh` calls four symbols that only `bin/intent_helpers`
# defines and sources none of them -- it rides on its sourcers having sourced
# the v2 helpers first. A green on the source edge alone says nothing about it.
#
# THE SIGNAL IS STDERR -- NEVER rc, NEVER STDOUT. Measured on
# `intent_claude_cwi` 2026-08-27: with `bin/` absent the pre-port form returns
# rc=0 with stdout byte-identical to the ported form, differing only on stderr.
# The missing symbol is `error()` ITSELF, so the No-Silent-Errors path is the
# one that goes silent and execution continues past every guard it protected.
# Asserting on rc or stdout yields a permanently-green test.
#
# BOTH ARMS, BECAUSE NEITHER ALONE DECIDES IT. The surviving arm passes for a
# file that never had the dependency, so it cannot tell PORTED from
# NEVER-COUPLED. The control arm re-creates the pre-port form of each survivor
# by deleting the primitives it carries, and requires that to BREAK.
#
# THE STATIC CALL-EDGE ARM EXPIRES WITH ITS SUBJECT, AND SAYS SO RATHER THAN
# GOING QUIETLY GREEN. It needs `bin/intent_helpers` present to know what
# symbols `bin/` alone defines. After the prune that file is gone and the arm
# reports `n/a -- bin/ already pruned` instead of a pass: a check that passes
# because its subject vanished is a claim that was true when made and false
# when used. The driven arms do not expire -- a missing symbol still says
# `command not found` on stderr.
#
# Usage: bash tests/cutover_guard.bash [-v]

set -uo pipefail

VERBOSE=0
[ "${1:-}" = "-v" ] && VERBOSE=1

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT" || exit 1

RED=0
note() { printf 'FINDING  %s\n' "$1"; RED=$((RED + 1)); }
say()  { [ "$VERBOSE" -eq 1 ] && printf '         %s\n' "$1"; return 0; }
die()  { printf 'error: %s\n' "$1" >&2; exit 2; }

# ---- population, derived from the tree ----
#
# The two trees the install SHIPS and RUNS. `bin/` is deliberately absent from
# this list: it is the thing being pruned, not a survivor. A file that should
# have been pruned and was not is IN this population and fails the arms below,
# which is how this guard checks the prune without being told the prune list.
POP=()
for d in intent/plugins/claude/bin intent/plugins/claude/lib \
         lib/templates/hooks lib/templates/.claude/scripts; do
  [ -d "$d" ] || continue
  while IFS= read -r f; do
    case "$f" in *.json) continue ;; esac
    [ -f "$f" ] && POP+=("$f")
  done <<< "$(find "$d" -maxdepth 1 -type f | sort)"
done
[ "${#POP[@]}" -gt 0 ] || die "the population is empty -- this instrument measured nothing"

# ---- the symbol roster of the thing being pruned ----
if [ -f bin/intent_helpers ]; then
  SYMS="$(grep -oE '^[a-z_]+\(\)' bin/intent_helpers | sed 's/()//' | sort -u)"
  [ -n "$SYMS" ] || die "bin/intent_helpers defines no functions -- the roster is empty"
  CALL_ARM=live
else
  SYMS=""
  CALL_ARM=expired
fi

# Does $1 reference symbol $2 at a COMMAND POSITION, comments stripped?
#
# A provenance citation in a comment is not an edge -- the criterion is explicit
# that deleting those citations destroys the record. Neither is a bare word that
# merely looks like the name: `warning)` opening a `case` arm, `--severity-min
# warning` as a flag value, and `intent info` as a CLI verb all matched a looser
# form of this predicate and produced three false findings in its first run. A
# command position is the start of a command or the inside of a substitution.
calls() {
  local file="$1" sym="$2" code
  grep -qE "^${sym}\(\)" "$file" && return 1     # its own definition, not a call
  # comments out, and `case` ARMS out. `critical|warning|recommendation|style)`
  # is an alternation of PATTERNS, and its `|` is indistinguishable from a
  # pipeline separator to the command-position regex below. Case arms supplied
  # every false finding this instrument produced -- five across two passes.
  # The PATTERN is cut off each `case` arm, and the arm's BODY is kept. Deleting
  # the whole line would hide a real call in `foo) error "x" ;;` -- a blind spot
  # traded for a false positive, which is the worse half of that trade.
  code="$(grep -vE '^[[:space:]]*#' "$file" | sed -E 's/^[[:space:]]*[A-Za-z0-9_*?|.-]+\)//')"
  # start of a command, and NOT followed by `)` -- that trailing paren is what
  # makes a `case` pattern look exactly like a call
  grep -qE "(^|[;&|{}]|&&|\|\|)[[:space:]]*${sym}([[:space:]]|;|\||&|$)" <<< "$code" && return 0
  # inside a command substitution
  grep -qE "\\\$\([[:space:]]*${sym}([[:space:]]|\))" <<< "$code" && return 0
  return 1
}

# ANY spelling of the directory. The criterion measured three across seven files
# -- `$INTENT_ROOT/bin/`, `$INTENT_HOME/bin/`, and a two-step `INTENT_BIN=` --
# so a grep for one literal reads as a complete answer to a narrower question.
#
# NO PIPELINE HERE, AND THAT IS NOT STYLE. Under `set -o pipefail` a `grep -q`
# that exits on its first match SIGPIPEs the `grep -v` feeding it, the pipeline
# returns 141, and the predicate answers "no". It fires only while the upstream
# grep is still writing -- so the first version of this answered correctly for
# every short file and wrongly for `intent_claude_subagents` and
# `intent_claude_upgrade`, the two longest in the population. A silent wrong
# answer whose probability rises with file size.
sources_helpers() {
  local code
  code="$(grep -vE '^[[:space:]]*#' "$1")"
  grep -qE '^[[:space:]]*(source|\.)[[:space:]].*intent_helpers' <<< "$code"
}

# ---- ARM 1: static, both edges, over the whole population ----
printf '== static edge census (%d files) ==\n' "${#POP[@]}"
SRC_EDGES=0
CALL_EDGES=0
for f in "${POP[@]}"; do
  if sources_helpers "$f"; then
    note "SOURCE EDGE  $f sources bin/intent_helpers"
    SRC_EDGES=$((SRC_EDGES + 1))
    continue
  fi
  [ "$CALL_ARM" = live ] || continue
  hits=""
  while IFS= read -r s; do
    [ -n "$s" ] || continue
    calls "$f" "$s" && hits="$hits $s"
  done <<< "$SYMS"
  if [ -n "$hits" ]; then
    note "CALL EDGE    $f calls$hits -- defined only in bin/intent_helpers, sourced nowhere"
    CALL_EDGES=$((CALL_EDGES + 1))
  fi
done
if [ "$CALL_ARM" = expired ]; then
  printf 'call edge: n/a -- bin/intent_helpers is already pruned, so what it alone defined\n'
  printf '           is no longer derivable. NOT a pass. The driven arms do not expire.\n'
fi

# ---- fixture: the shipped trees, with bin/ ABSENT ----
#
# Only what is driven is copied. Copying `intent/` wholesale pulls in the store,
# the parity corpus and every board history, which took longer than the whole
# rest of this run.
FIX="$(mktemp -d)"
cleanup() { rm -rf "$FIX"; }
trap cleanup EXIT
mkdir -p "$FIX/install/intent" "$FIX/install/lib"
cp -R intent/plugins "$FIX/install/intent/plugins" || die "fixture: cannot copy intent/plugins"
cp -R intent/.config "$FIX/install/intent/.config" || die "fixture: cannot copy intent/.config"
cp -R lib/templates "$FIX/install/lib/templates" || die "fixture: cannot copy lib/templates"
[ -e "$FIX/install/bin" ] && die "the fixture has a bin/ -- the arms below would prove nothing"

# ---- ARM 2 + 3: driven, in that fixture ----
#
# Driven over the plugin trees only. The `lib/templates` hook bodies are in the
# static population above but are NOT driven here: a git guard run outside a
# repository emits git's own errors on stderr, and an arm that reds on that is
# measuring the fixture. They are declared in the partition rather than dropped.
printf '\n== driven, in a tree with bin/ absent ==\n'
DRIVEN=0; CONTROLLED=0; UNCONTROLLED=0; STATIC_ONLY=0

# Driven from INSIDE the fixture project, not beside it: `intent_claude_cwi`
# resolves a project root and refuses without one, so driving from `$FIX` reds
# every arm on the fixture's shape rather than on the dependency under test.
# stdin is closed, so a script that reads a prompt cannot hang the run.
drive() {  # $1 = absolute script path; prints what it wrote to stderr
  ( cd "$FIX/install" && bash "$1" </dev/null ) 2>&1 >/dev/null
}

for f in "${POP[@]}"; do
  case "$f" in
    intent/plugins/claude/bin/*) ;;
    *) STATIC_ONLY=$((STATIC_ONLY + 1)); continue ;;
  esac

  target="$FIX/install/$f"
  [ -f "$target" ] || die "fixture is missing $f"

  # SURVIVING ARM: correct answer, EMPTY stderr.
  err="$(drive "$target")"
  DRIVEN=$((DRIVEN + 1))
  if [ -n "$err" ]; then
    note "SURVIVING    $f writes to stderr with bin/ absent: $(printf '%s' "$err" | head -1)"
  else
    say "surviving ok: $f"
  fi

  # CONTROL ARM: the pre-port form is this file with the primitives it CARRIES
  # deleted. A file carrying none is not controllable -- and that is the honest
  # verdict, not a pass: it means this arm cannot tell whether it was ported.
  carried=()
  if [ "$CALL_ARM" = live ]; then
    while IFS= read -r s; do
      [ -n "$s" ] || continue
      grep -qE "^${s}\(\)" "$f" && carried+=("$s")
    done <<< "$SYMS"
  fi
  if [ "${#carried[@]}" -eq 0 ]; then
    UNCONTROLLED=$((UNCONTROLLED + 1))
    say "uncontrolled: $f carries no bin/ primitive of its own"
    continue
  fi

  pre="$FIX/preport_$(basename "$f")"
  cp "$target" "$pre"
  for s in "${carried[@]}"; do
    awk -v fn="$s" 'BEGIN{d=0} $0 ~ "^"fn"\\(\\)" {d=1; next} d && /^}/ {d=0; next} !d' \
      "$pre" > "$pre.new" && mv "$pre.new" "$pre"
  done
  cmp -s "$target" "$pre" && die "the pre-port form of $f is identical to the ported form -- the control edits nothing"
  err="$(drive "$pre")"
  CONTROLLED=$((CONTROLLED + 1))
  if [ -z "$err" ]; then
    note "CONTROL      $f pre-port form (minus ${carried[*]}) is SILENT with bin/ absent -- this arm cannot fail, so its green means nothing"
  else
    say "control ok: $f pre-port form breaks on stderr as it must ($(printf '%s' "$err" | head -1))"
  fi
done

# ---- partition + vacuity ----
TOTAL="${#POP[@]}"
SUM=$((DRIVEN + STATIC_ONLY))
printf '\npartition -- %d driven + %d static-only = %d, of %d files\n' \
  "$DRIVEN" "$STATIC_ONLY" "$SUM" "$TOTAL"
[ "$SUM" -eq "$TOTAL" ] || die "the partition does not close: $SUM accounted, $TOTAL in the population"
printf 'driven     -- %d controlled + %d uncontrolled = %d\n' \
  "$CONTROLLED" "$UNCONTROLLED" "$DRIVEN"
[ $((CONTROLLED + UNCONTROLLED)) -eq "$DRIVEN" ] || die "the driven partition does not close"

if [ "$CONTROLLED" -eq 0 ]; then
  die "NO FILE WAS CONTROLLED. The surviving arm alone cannot tell PORTED from NEVER-COUPLED, so a green here would be the permanently-green test AT-12.1 names. Either the population is wrong or no survivor carries a primitive of its own."
fi

printf '\n'
if [ "$RED" -eq 0 ]; then
  printf 'ok: %d file(s) surviving the prune, none depending on bin/ by either edge\n' "$TOTAL"
  exit 0
fi
printf 'FAIL: %d finding(s) -- %d source edge(s), %d call edge(s)\n' "$RED" "$SRC_EDGES" "$CALL_EDGES"
exit 1
