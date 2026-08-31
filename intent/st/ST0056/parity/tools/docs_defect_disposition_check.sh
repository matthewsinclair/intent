#!/usr/bin/env bash
# AC-02.3 -- every defect a v3.0.0 reader can hit carries a disposition.
#
# WHAT THIS CHECKS, AND WHAT IT REFUSES TO TAKE ON TRUST
#
# The population is DERIVED here, never transcribed, because AC-02.3's own point
# is being auditable rather than asserted -- and the figure moves: it was 103 of
# 164 on the morning of 2026-08-31 and 105 of 166 by the afternoon.
#
# A member is EXCLUDED only when BOTH hold:
#   (1) it was closed in the register AT THE CUT, and
#   (2) its body carries no language suggesting the closure was not a fix.
# (2) exists because the register has exactly TWO status values, so nothing in
# the record distinguishes a fix from a wontfix / by-design / superseded. A
# member excluded by the RULE is invisible; one excluded by a DISPOSITION is on
# the record. So the rule errs toward the population and every judgement is
# written down. (vc's catch, 2026-08-31: 12 of 43 closures carry non-fix prose.)
#
# A `stated` disposition is VERIFIED AGAINST BYTES, never against a promise: the
# page it names must exist and must contain the quoted string. Docs are the
# published pages PLUS the binary's own `--help`, because both are reached by
# following the docs correctly.
#
# Exit 0 = every member dispositioned and every `stated` quote found.
# Exit 1 = a member is undispositioned, a row is stale, or a quote is absent.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)" || exit 2

CUT="${CUT:-v3.0.0}"
REG=intent/.canon/issues
MANIFEST="${1:-intent/st/ST0056/parity/data/ac0203-dispositions.tsv}"
NONFIX='wontfix|won.t fix|not a defect|not a bug|duplicate of|superseded by|by design|as designed|invalid|no longer applies|closed without|recorded not fixed'

fail=0
note() { printf '%s\n' "$*"; }

# ---- 0. the exclusion pattern is controlled BEFORE any zero is trusted ------
tmp=$(mktemp -d); printf '%s\n' 'this is a duplicate of 0012' 'closing wontfix' > "$tmp/p"
ctl=$(grep -icE "$NONFIX" "$tmp/p"); rm -rf "$tmp"
if [ "$ctl" -lt 2 ]; then
  note "docs-disposition: FAIL -- the non-fix pattern scored $ctl of 2 on planted prose."
  note "  Every exclusion below would be unearned. Refusing to report."
  exit 1
fi

# ---- 1. derive the population ----------------------------------------------
pop_file=$(mktemp); excl=0; all=0
for f in "$REG"/*.json; do
  all=$((all+1)); id=$(basename "$f" .json)
  if git cat-file -e "$CUT:$f" 2>/dev/null \
     && [ "$(git show "$CUT:$f" | jq -r '.status')" = "closed" ] \
     && ! git show "$CUT:$f" | jq -r '.body // ""' | grep -qiE "$NONFIX"; then
    excl=$((excl+1)); continue
  fi
  echo "$id" >> "$pop_file"
done
pop=$(grep -c . "$pop_file")
if [ $((excl + pop)) -ne "$all" ]; then
  note "docs-disposition: FAIL -- denominator broken: $excl + $pop != $all"; exit 1
fi

note "docs-disposition: population derived at $CUT ($(git rev-parse --short "$CUT"))"
note "  register files: $all    excluded (closed at cut, closure reads as a fix): $excl"
note "  POPULATION: $pop    asserted: $excl + $pop = $((excl+pop)) of $all"

# ---- 2. the criterion's own named members are the control ------------------
miss=0
for id in 0086 0139 0121 0122 0149; do
  grep -qx "$id" "$pop_file" || { note "  CONTROL FAIL: $id, named in AC-02.3's prose, is not in the population"; miss=$((miss+1)); }
done
if [ "$miss" -gt 0 ]; then
  note "docs-disposition: FAIL -- the derivation is refuted by the criterion's own members."
  rm -f "$pop_file"; exit 1
fi
note "  control: AC-02.3's five named members, 0 of 5 dropped"

# ---- 3. the manifest --------------------------------------------------------
if [ ! -f "$MANIFEST" ]; then
  note "docs-disposition: FAIL -- no manifest at $MANIFEST; all $pop members are undispositioned."
  rm -f "$pop_file"; exit 1
fi

undisposed=0; stale=0; stated=0; notreach=0; badquote=0
while IFS= read -r id; do
  row=$(awk -F'\t' -v i="$id" '$1==i' "$MANIFEST" | head -1)
  if [ -z "$row" ]; then
    undisposed=$((undisposed+1)); [ "$undisposed" -le 12 ] && note "  UNDISPOSITIONED: $id  $(jq -r '.title' "$REG/$id.json" | cut -c1-64)"
    continue
  fi
  disp=$(printf '%s' "$row" | cut -f2)
  where=$(printf '%s' "$row" | cut -f3)
  quote=$(printf '%s' "$row" | cut -f4)
  case "$disp" in
    stated)
      if [ -z "$quote" ]; then
        note "  NO QUOTE: $id claims stated in '$where' with nothing to verify"; badquote=$((badquote+1)); continue
      fi
      if [ "${where#--help:}" != "$where" ]; then
        # A verb may be multi-word ("at lint"), so it must reach argv as TWO
        # tokens. An array says that; a bare $verb would only do it by accident.
        read -r -a verb_parts <<< "${where#--help:}"
        if ! intent "${verb_parts[@]}" --help 2>&1 | grep -qF -- "$quote"; then
          note "  QUOTE ABSENT: $id -- '$quote' is not in \`intent ${where#--help:} --help\`"; badquote=$((badquote+1)); continue
        fi
      elif [ ! -f "$where" ]; then
        note "  PAGE ABSENT: $id names '$where', which does not exist"; badquote=$((badquote+1)); continue
      elif ! grep -qF -- "$quote" "$where"; then
        note "  QUOTE ABSENT: $id -- '$quote' is not in the bytes of $where"; badquote=$((badquote+1)); continue
      fi
      stated=$((stated+1)) ;;
    not-reader-reachable)
      if [ -z "$where" ]; then
        note "  NO REASON: $id is not-reader-reachable with no reason recorded"; badquote=$((badquote+1)); continue
      fi
      notreach=$((notreach+1)) ;;
    *)
      note "  BAD DISPOSITION: $id carries '$disp'"; badquote=$((badquote+1)) ;;
  esac
done < "$pop_file"

while IFS=$'\t' read -r id _rest; do
  [ -z "$id" ] && continue
  case "$id" in \#*) continue ;; esac
  grep -qx "$id" "$pop_file" || { note "  STALE ROW: $id is in the manifest and not in the population"; stale=$((stale+1)); }
done < "$MANIFEST"

[ "$undisposed" -gt 12 ] && note "  ... and $((undisposed-12)) more undispositioned"
rm -f "$pop_file"

note ""
note "docs-disposition: COUNTS"
note "  stated in the docs where a reader would meet it: $stated"
note "  recorded not reader-reachable with a reason:     $notreach"
note "  sum: $((stated+notreach)) of $pop"
[ "$undisposed" -gt 0 ] && { note "  UNDISPOSITIONED: $undisposed"; fail=1; }
[ "$stale" -gt 0 ]      && { note "  STALE ROWS: $stale"; fail=1; }
[ "$badquote" -gt 0 ]   && { note "  UNVERIFIED CLAIMS: $badquote"; fail=1; }
[ $((stated+notreach)) -ne "$pop" ] && fail=1

if [ "$fail" -eq 0 ]; then note "docs-disposition: ok -- $pop of $pop dispositioned, every stated claim verified against bytes"; else note "docs-disposition: FAIL"; fi
exit "$fail"
