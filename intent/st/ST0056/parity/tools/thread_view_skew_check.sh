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
# it. This parses one verdict; it does not form one.
#
# IT PARSES THE DENOMINATOR AND REFUSES IF IT CANNOT, WHICH IS THE WHOLE
# DIFFERENCE BETWEEN THIS AND A GREP. A text-scraping gate whose pattern stops
# matching does not fail: it goes GREEN, forever, and nothing anywhere says so.
# That is `pre-commit.sh:98` -- *a guard nothing dispatches is indistinguishable
# from a guard that passes* -- one layer out, and this estate found three
# instances of the family in one morning. So the summary line is read for its
# `N view(s)` FIRST, and a summary this script cannot parse is exit 2 and a
# refusal. **A zero is not a result until the instrument has proven it can
# produce a non-zero**, and the denominator is that proof carried on every run.
#
# FIELD-ANCHORED, NEVER A SUBSTRING. The needle is `^residue: <path> -- view-skew
# -- `, because a path or a message that MENTIONS the class is not a finding OF
# it -- the same distinction that had a broadcast match `_(empty)_` inside its
# own quoted prose and silently drop a node from two sends.
#
# THIS PARSE HAS A SHELF LIFE AND THE EXPIRY IS NAMED SO IT DOES NOT OUTLIVE ITS
# REASON. `render.rs:1668` records that `doctor` has no machine face yet -- *it
# stays inline until `doctor` has a machine face to carry it, which needs a
# surface row and is not mine to add*. `Finding` already derives `Serialize` and
# `FindingClass` is already kebab-case on the wire, so `doctor --json` is a
# `Serialize` on `Report` plus a surface row. **WHEN THAT LANDS, DELETE THE
# PARSING IN THIS SCRIPT AND CONSUME THE STRUCTURED OUTPUT -- do not keep both.**
# A text reader left beside a structured one is a second opinion about what
# `doctor` said, which is the same shape as the roster that agreed with neither
# dispatcher beneath it and the verifier that carried its own copy of the
# classifier's rules. The refusal above is a WORKAROUND for a missing surface,
# not a preference, and a workaround nobody dated is how the estate acquires
# permanent ones.
#
# `--changed <paths...>` BLOCKS ONLY ON WHAT THIS COMMIT TOUCHES and reports the
# rest, which is the clock guard's ratified inherited-breakage rule: a guard that
# must be bypassed to work is a guard nobody keeps, and a repository that
# arrived already skewed would otherwise be unable to commit the fix.
#
# Exit codes: 0 clean, not applicable, or inherited-only; 1 this commit touches
# a skewed view; 2 the instrument cannot answer (no verdict, unreadable summary).

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

# THE BINARY IS RESOLVED BY ABSOLUTE PATH BECAUSE v3 IS OFF PATH BY STANDING
# RULING, and `intent` on PATH is v2, which knows nothing about any of this.
BIN="$ROOT/native/rust/target/release/intent"
if [ ! -x "$BIN" ]; then
  # NAMED, NOT SILENT, and exit 0. A machine with no v3 build owes nothing, but
  # a reader must never mistake "no build here" for "no skew here" -- those are
  # the two states this whole script exists to keep apart.
  echo "thread-view-skew: no v3 binary at $BIN -- generated thread views are UNCHECKED this commit." >&2
  exit 0
fi

OUT="$("$BIN" doctor 2>&1)"
RC=$?
# `doctor` exits 1 for findings of ANY class, so its exit code is not this
# instrument's verdict and must not be read as one. Anything above 1 is the
# command failing rather than reporting, and a failure to run is not a pass.
[ "$RC" -le 1 ] || die "\`$BIN doctor\` exited $RC -- it did not produce a verdict, so this commit is unchecked rather than clean. Output follows:
$OUT"

SUMMARY="$(printf '%s\n' "$OUT" | grep -E '^doctor: [0-9]+ finding\(s\) across ' | tail -1)"
[ -n "$SUMMARY" ] || die "\`doctor\` printed no summary line this script can read. Its output shape has changed and the needles below are no longer known to match -- which is the one failure a text-reading gate cannot detect from its own silence, so it refuses instead. Re-derive the needles against the current output before trusting any answer from this script."

VIEWS="$(printf '%s\n' "$SUMMARY" | sed -n 's/.*, \([0-9][0-9]*\) view(s).*/\1/p')"
[ -n "$VIEWS" ] || die "the summary line parsed but carries no view denominator: $SUMMARY"
[ "$VIEWS" -gt 0 ] 2>/dev/null || die "\`doctor\` reports $VIEWS views rendered. An empty population and a clean estate compare equal, so this is a refusal and not a pass."

SKEWED="$(printf '%s\n' "$OUT" | sed -n 's/^residue: \(.*\) -- view-skew -- .*$/\1/p')"

if [ -z "$SKEWED" ]; then
  echo "thread-view-skew: $VIEWS generated view(s) match the model."
  exit 0
fi

BLOCKING=""
INHERITED=""
while IFS= read -r path; do
  [ -n "$path" ] || continue
  if [ "$TRIGGERED" -eq 1 ] && [ "${CHANGED#* $path }" = "$CHANGED" ]; then
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
