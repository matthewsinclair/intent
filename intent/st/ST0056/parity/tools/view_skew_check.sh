#!/bin/bash
# view_skew_check.sh -- a committed generated view must still match what its
# generator produces from committed canon.
#
# THE INCIDENT. `surface/dispatch-table.md` was stale against its own JSON canon
# from f0d6e64 until someone happened to regenerate it. The cost was twenty
# minutes chasing a phantom md5, and cc reports it was the second stale-committed
# -view cost that week. Nothing detected it either time. The failure is not that
# a view goes stale in a working tree -- it is that a stale view LANDS, and a
# report only helps if someone runs it and reads it.
#
# WHY THIS IS A SIBLING OF provenance_check.sh AND NOT PART OF IT. That merge was
# the natural-looking move and vc ruled against it, correctly: they are different
# invariants. Provenance checks that STAMPS AGREE within a measurement group --
# metadata. Skew checks that CONTENT MATCHES CANON. Merging them gives one script
# two unrelated reasons to fail behind one exit code, which is precisely the
# `intent critic` exit-2 overload already sitting in hv's queue as a defect. Do
# not reproduce a known defect in new apparatus.
#
# WHAT IS ACTUALLY CHECKABLE IS SMALLER THAN IT LOOKS, and the measurement below
# corrects the table this guard was commissioned from.
#
# The stated precondition was "the generator honours OUT", so a check can
# regenerate to a temp path and diff without mutating the thing it is checking.
# That is necessary and it is NOT sufficient, and the difference is a live trap:
# `gen_register.sh` declares OUT and still cannot be round-tripped, because it
# also requires SP (a directory holding the RAW `burn.tsv`) and WT (a detached
# worktree at the measured revision). Neither is committed -- `burn.tsv` is
# tracked nowhere and is not even on disk. Checking for an OUT variable passes;
# actually redirecting OUT dies at `SP: parameter null or not set`.
#
# So `register.md` is in the SAME class as `pertest.md`, not a class above it:
# NOT re-derivable from committed state, at any price short of a full re-sweep.
#
# AND THE SAME QUESTION, ASKED OF THE OTHER 27, GAVE THE SAME ANSWER. The
# reported blocker on `cmd-*.md` was "gen_inventory.sh does not honour OUT".
# It does -- via OUTDIR. That was a naming mismatch, not a missing capability,
# and had it been treated as a one-line fix the 27 files would have been
# promoted to CHECKABLE on a false premise. The real blocker is that
# gen_inventory.sh renders from `$SP/probes/toplevel.tsv`, which is NOT TRACKED.
#
# So the honest count is ONE OF THIRTY. `dispatch-table.md` is the only
# apparatus view re-derivable from committed state; the other 29 rest on their
# stamp alone.
#
# That is not a reason to weaken this guard, it is the argument for the other
# one. `provenance_check.sh` groups exactly those 29 -- the burn pair, the 26
# cmd files, and the table -- so the two checks partition the apparatus rather
# than overlapping it, and the stamp check is the ONLY guard 29 artefacts have.
# It is also, per vc, still unwired.
#
# THE HIGHEST-LEVERAGE CHANGE AVAILABLE HERE IS NOT IN THIS FILE: committing
# `probes/toplevel.tsv` would move 27 artefacts from stamp-only to content-
# checked in one move. Recorded rather than done, because whether a measurement
# input belongs in the repo is a judgement about the apparatus, not a tidy-up.
#
# THE BACKSTOP ENUMERATES; IT DOES NOT SNIFF A BANNER. The obvious design is to
# look for the "GENERATED VIEW" banner and check everything carrying one. That
# was measured and rejected: of the 30 apparatus views, exactly ONE carries a
# banner. `register.md`, `pertest.md` and all 26 `cmd-*.md` have none, so a
# banner needle would have covered a single file and reported full coverage --
# a needle that stops matching without saying so, which is the class this
# toolchain has already been bitten by three times. Enumerating the directory
# and demanding every view be classified cannot fail that way: a new view is
# unregistered until someone registers it.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
P="$ROOT/intent/st/ST0056/parity"

rc=0
checked=0
skipped=0

# ---------------------------------------------------------------------------
# CHECKABLE -- view | generator | canon input(s), space-separated
# A view belongs here only if the generator has been PROVEN to round-trip to a
# temp OUT, not merely observed to mention OUT.
# ---------------------------------------------------------------------------
CHECKABLE="surface/dispatch-table.md|gen_dispatch_table.sh|surface/dispatch-table.json"

# ---------------------------------------------------------------------------
# DECLARED UNCHECKABLE -- view-or-glob | why
# Stating the reason is the point. An artefact nobody can check is a real
# finding about the apparatus, and burying it as an omission is how a guard
# quietly stops covering what it claims to.
# ---------------------------------------------------------------------------
UNCHECKABLE="$(cat <<'EOF'
intent/st/ST0056/parity/register.md|WT ONLY -- a detached worktree at the measured revision, which is not a file and cannot be committed. CORRECTED 2026-08-16: this line used to say the generator "needs SP (raw burn.tsv, untracked and absent)". The burn input is committed at tools/burn-baseline.tsv and gen_register.sh now DEFAULTS to it, so SP is an override rather than a precondition. Verified today: byte-identical to the committed artefact with no SP and no BURN.
intent/st/ST0056/parity/pertest.md|WT ONLY, same as register.md. CORRECTED 2026-08-16: this line used to say the generator needs "the ephemeral TAP ... which is not committed". The TAP corpus IS committed at tools/tap-baseline/ (196 files) and TAP_DIR defaults to it; BURN now defaults to the committed twin too. Verified today: byte-identical with no SP, no BURN and no TAP_DIR.
intent/st/ST0056/parity/README.md|gen_inventory.sh DOES redirect (via OUTDIR, not OUT -- the reported "missing OUT" was a naming mismatch, not a missing capability). NOT un-re-derivable any more, only un-CHEAPLY-checkable: the probe TSV is committed at parity/probes/toplevel.tsv and the generator is a formatter fixed point, so a fresh render reproduces the committed files exactly. What it still needs is $WT, a detached worktree at the measured revision, because the verb and flag extractors read the v2 SOURCE rather than the probe data -- seconds of setup per run, and a slow gate is one that gets --no-verify'd. To check on demand: git worktree add --detach <dir> 69d42a7, then SP=<scratch> WT=<dir> OUTDIR=<out> gen_inventory.sh, then diff against parity/.
intent/st/ST0056/parity/cmd-*.md|same as README.md. 27 files, verified 27/27 BYTE-IDENTICAL to a fresh render on 2026-08-15 -- so these have a real content check now, just not one a pre-commit hook can afford. THIS ENTRY USED TO CLAIM committing the TSV would promote all 27 to CHECKABLE in one move. That was wrong twice over: the TSV was necessary and not sufficient (the worktree remains), and the generator ALSO emitted unaligned tables and a trailing blank line, so its output could never equal the committed file no matter what the input was. Both found by committing the TSV and actually running it. Recorded rather than quietly corrected, because a guard that names its own highest-leverage fix and is mistaken about it sends the next person the same way.
EOF
)"

# ---------------------------------------------------------------------------
# AUTHORED -- view-or-glob | what it is
# A THIRD CATEGORY, because two were not enough and the second was lying. The
# backstop below demands every apparatus .md be checkable or declared, and the
# summary line calls the declared ones "un-re-derivable" -- which is true of a
# generated view whose input is gone, and false of a file nobody generates.
# Filing an authored document under un-re-derivable would inflate the count of
# artefacts this apparatus cannot check with one it was never supposed to.
# Found the moment the first authored file landed in surface/ and the guard
# refused it, which is the backstop working.
# ---------------------------------------------------------------------------
AUTHORED="$(cat <<'EOF'
surface/agent-guide.spec.md|The AC-09.4 spec for the `intent llm` agent guide (ic). Authored, not generated: it describes what the guide contains and where each half comes from. There is no canon it derives from, so there is nothing to check it against and nothing missing.
EOF
)"

# ---------------------------------------------------------------------------
# PATH TRIGGERING. A full check costs ~4s wall, and a slow gate gets --no-verify'd,
# which is the cry-wolf family arriving through a different door. With no
# arguments every triple is checked. With --changed, a triple is checked only if
# its view, its generator or its canon is among the named paths.
#
# This is sound rather than a fudge: gen_dispatch_table.sh reads only $IN, so
# the view cannot go stale unless the canon, the generator or the view itself
# changes. The backstop always runs -- it costs a directory listing, and its
# whole job is to notice things nobody declared.
# ---------------------------------------------------------------------------
CHANGED=""
TRIGGERED=0
if [ "${1:-}" = "--changed" ]; then
  shift
  CHANGED=" $* "
  TRIGGERED=1
fi

triggered_by() {
  [ "$TRIGGERED" -eq 0 ] && return 0
  local p
  for p in $1; do
    case "$CHANGED" in *" $p "*) return 0 ;; esac
  done
  return 1
}

# ---------------------------------------------------------------------------
while IFS='|' read -r view gen canon; do
  [ -n "$view" ] || continue

  if ! triggered_by "$view $canon intent/st/ST0056/parity/tools/$gen"; then
    skipped=$((skipped + 1))
    continue
  fi

  if [ ! -f "$ROOT/$view" ]; then
    echo "skew: $view is registered as generated but is not on disk" >&2
    rc=1; continue
  fi

  tmp="$(mktemp)" || { echo "skew: cannot create a temp file" >&2; exit 2; }
  if ! OUT="$tmp" bash "$HERE/$gen" >/dev/null 2>"$tmp.err"; then
    echo "skew: $gen refused to render -- cannot judge $view" >&2
    sed 's/^/  /' "$tmp.err" >&2
    rm -f "$tmp" "$tmp.err"
    rc=1; continue
  fi

  # A generator that exits 0 and writes nothing would otherwise read as
  # "identical to an empty file was not requested" -- refuse instead.
  if [ ! -s "$tmp" ]; then
    echo "skew: $gen exited 0 but wrote nothing to OUT -- it does not honour OUT, and $view cannot be checked this way" >&2
    rm -f "$tmp" "$tmp.err"
    rc=1; continue
  fi

  if diff -q "$ROOT/$view" "$tmp" >/dev/null 2>&1; then
    printf 'ok: %-32s matches %s\n' "${view##*/}" "${canon##*/}"
    checked=$((checked + 1))
  else
    echo "skew: $view DOES NOT match what $gen produces from $canon" >&2
    echo "  the committed view is stale, or a row was hand-edited. Regenerate; never hand-edit the view to match." >&2
    diff "$ROOT/$view" "$tmp" 2>&1 | sed -n '1,12p' | sed 's/^/  /' >&2
    rc=1
  fi
  rm -f "$tmp" "$tmp.err"
done <<< "$CHECKABLE"

# ---------------------------------------------------------------------------
# THE BACKSTOP. Every .md in the apparatus must be classified -- checkable, or
# declared uncheckable with a reason. An unregistered view is REPORTED, because
# a new generator landing is exactly when this guard needs to grow, and the only
# moment anyone will notice is now.
# ---------------------------------------------------------------------------
unregistered=""
for f in "$ROOT/surface"/*.md "$P"/*.md; do
  [ -f "$f" ] || continue
  rel="${f#$ROOT/}"

  case "$CHECKABLE" in *"$rel|"*) continue ;; esac

  known=0
  while IFS='|' read -r pat _why; do
    [ -n "$pat" ] || continue
    # shellcheck disable=SC2254 -- $pat is a deliberate glob (cmd-*.md)
    case "$rel" in $pat) known=1; break ;; esac
  done <<< "$UNCHECKABLE"

  if [ "$known" -eq 0 ]; then
    while IFS='|' read -r pat _what; do
      [ -n "$pat" ] || continue
      # shellcheck disable=SC2254 -- deliberate glob, same as above
      case "$rel" in $pat) known=1; break ;; esac
    done <<< "$AUTHORED"
  fi

  [ "$known" -eq 1 ] || unregistered="$unregistered  $rel"$'\n'
done

if [ -n "$unregistered" ]; then
  echo "skew: apparatus view(s) with no registered generator -- classify them or state why they stand alone:" >&2
  printf '%s' "$unregistered" >&2
  rc=1
fi

if [ "$rc" -eq 0 ]; then
  if [ "$TRIGGERED" -eq 1 ] && [ "$checked" -eq 0 ]; then
    echo "skew: no generated view was touched by this change -- nothing to check."
  else
    echo "skew: $checked generated view(s) match their canon; $(printf '%s' "$UNCHECKABLE" | grep -c .) declared un-re-derivable; $(printf '%s' "$AUTHORED" | grep -c .) authored (nothing to check)."
  fi
fi

exit $rc
