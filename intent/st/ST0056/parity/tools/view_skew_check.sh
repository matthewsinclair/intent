#!/usr/bin/env bash
# view_skew_check.sh -- a committed APPARATUS view must still match what its
# generator produces from committed canon.
#
# THE NOUN IS THE CONTRACT. This tool covers the parity apparatus views under
# intent/st/ST0056/parity/ and NOTHING ELSE. It does not cover the thread views
# -- the covers and acceptance contracts the thread model renders -- which are
# `thread_view_skew_check.sh`'s subject and which no gate runs. Saying
# "generated view" claims both, and a reader who knows the coverage figure
# still reads a general noun as the general category. UNCHECKED IS NOT EMPTY,
# and the output is the only place that distinction can travel.
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
# RETIRED 2026-11-25 (ST1c2db93 WP-5cc0, vc decision 134): `gen_register.sh`,
# `gen_pertest.sh`, `register.md`, `pertest.md`, the burn sweep and the TAP
# corpus behind them left with the v2 bats-estate inventory. The paragraphs
# above and the counts below are the measurement as it stood before that.
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
# Read by `read -d ''`, not `$(cat <<'EOF' ...)`: bash 3.2, which the commit
# gate may run, mis-parses a heredoc inside a command substitution whose text
# holds an apostrophe or a parenthesis, and refuses the whole file.
UNCHECKABLE=""
IFS= read -r -d '' UNCHECKABLE <<'EOF' || true
intent/st/ST0056/parity/README.md|gen_inventory.sh DOES redirect (via OUTDIR, not OUT -- the reported "missing OUT" was a naming mismatch, not a missing capability). NOT un-re-derivable any more, only un-CHEAPLY-checkable: the probe TSV is committed at parity/probes/toplevel.tsv and the generator is a formatter fixed point, so a fresh render reproduces the committed files exactly. What it still needs is $WT, a detached worktree at the measured revision, because the verb and flag extractors read the v2 SOURCE rather than the probe data -- seconds of setup per run, and a slow gate is one that gets bypassed with --no-verify. To check on demand: git worktree add --detach <dir> 69d42a7, then SP=<scratch> WT=<dir> OUTDIR=<out> gen_inventory.sh, then diff against parity/.
intent/st/ST0056/parity/cmd-*.md|same as README.md. 27 files, verified 27/27 BYTE-IDENTICAL to a fresh render on 2026-08-15 -- so these have a real content check now, just not one a pre-commit hook can afford. THIS ENTRY USED TO CLAIM committing the TSV would promote all 27 to CHECKABLE in one move. That was wrong twice over: the TSV was necessary and not sufficient (the worktree remains), and the generator ALSO emitted unaligned tables and a trailing blank line, so its output could never equal the committed file no matter what the input was. Both found by committing the TSV and actually running it. Recorded rather than quietly corrected, because a guard that names its own highest-leverage fix and is mistaken about it sends the next person the same way.
EOF

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
# Read by `read -d ''`, not `$(cat <<'EOF' ...)`: bash 3.2, which the commit
# gate may run, mis-parses a heredoc inside a command substitution whose text
# holds an apostrophe or a parenthesis, and refuses the whole file.
AUTHORED=""
IFS= read -r -d '' AUTHORED <<'EOF' || true
surface/agent-guide.spec.md|The AC-09.4 spec for the `intent llm` agent guide (ic). Authored, not generated: it describes what the guide contains and where each half comes from. There is no canon it derives from, so there is nothing to check it against and nothing missing.
intent/st/ST0056/parity/ac-09_6-mcp-facade-read.md|The AC-09.6 read (ic, 2026-08-30): the classified partition of every MCP-exposed row and the decision list for hv. Authored evidence, not generated -- its numbers came from a scratchpad instrument whose controls and defects it records, and the durable artefact is the JUDGEMENT, which no generator can re-produce. The 59 facade declarations it grounds live in surface/dispatch-table.json, where the skew pair above already checks them.
intent/st/ST0056/parity/ac-09_5-resources-design.md|The AC-09.5 resources design (ic, 2026-08-31): the reasoning behind vc's bound (A) -- resources serve entity models through the existing st_show/wp_show/issue_show doors, the URI grammar derives from nav.rs's kinds, and the one build decision is the render seam (one shared entity-to-text renderer so "contents match st show" is byte-identical). Authored design record, not generated: it carries a ruling's argument and the measurement that wip.md and the boards have no CLI read to match, and the durable artefact is the JUDGEMENT, which no generator can re-produce. Cites AT-09.5; the surface it designs lands in mcp.rs and its witness in mcp_resources.rs.
intent/st/ST0056/parity/ac-06_1-unstable-causes.md|The AC-06.1 read (cc, 2026-08-31): why 27 files in the BATS estate are UNSTABLE, resolved to four causes with 25 of them at one commit. Authored evidence, not generated -- it runs HALF of burn.sh deliberately (the default binding alone decides UNSTABLE; the second run exists to compute a delta nobody needs here) so that NO baseline is produced and vc ruling that the new baseline stays out is honoured literally rather than measured around. The durable artefact is the CAUSE ATTRIBUTION and the controls it survived, which no generator can re-produce; the per-file numbers it rests on are re-derivable by re-running the estate at the named revision.
EOF

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
  rel="${f#"$ROOT"/}"

  case "$CHECKABLE" in *"$rel|"*) continue ;; esac

  known=0
  while IFS='|' read -r pat _why; do
    [ -n "$pat" ] || continue
    # shellcheck disable=SC2254  # $pat is a deliberate glob (cmd-*.md)
    case "$rel" in $pat) known=1; break ;; esac
  done <<< "$UNCHECKABLE"

  if [ "$known" -eq 0 ]; then
    while IFS='|' read -r pat _what; do
      [ -n "$pat" ] || continue
      # shellcheck disable=SC2254  # deliberate glob, same as above
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

# THREAD VIEWS IN THE CHANGE ARE NAMED AS UNCHECKED, NEVER PASSED OVER IN
# SILENCE. Reporting "nothing to check" on a commit that carries one tells the
# reader the category was EMPTY when it was UNCOVERED -- different claims, one
# of them true, and the false one arrives at the moment an operator is looking
# for reassurance. This does not check them; it refuses to imply it did.
#
# WHETHER A PATH IS A THREAD VIEW IS THE ID DOOR'S ANSWER (ST0095 AC-12.1): a
# view lives at `intent/st/<thread>/info.md`, `acceptance.md` or
# `WP/<package>/info.md`, and whether `<thread>` is a thread is asked of the
# door, never matched by a pattern here, so a hashed thread's view is named as
# an ordinal's is. Every candidate directory goes to the door in one call,
# read-only through lib_read_only.sh, which refuses rather than migrates this
# checkout's store, and only when the change carries a candidate at all.
thread_views=""
unconfirmed=""
# Asked only on a run that is otherwise clean: the note below prints only then,
# and a door that cannot answer must not turn a skew finding into a refusal
# that hides it.
if [ "$TRIGGERED" -eq 1 ] && [ "$rc" -eq 0 ]; then
  # shellcheck source=lib_read_only.sh
  . "$HERE/lib_read_only.sh"
  view_paths=()
  for p in $CHANGED; do
    case "${p##*/}" in info.md | acceptance.md) ;; *) continue ;; esac
    asks="$(ro_st_tokens "$p")"
    [ -n "$asks" ] || continue
    file="${p#intent/st/"${asks%%$'\t'*}"/}"
    case "$asks" in
      *$'\t'wp) ;;
      *) case "$file" in info.md | acceptance.md) ;; *) continue ;; esac ;;
    esac
    view_paths+=("$p")
  done
  if [ "${#view_paths[@]}" -gt 0 ]; then
    RO_ROOT="$ROOT"
    RO_TAG="skew"
    RO_WORK="$(mktemp -d "${TMPDIR:-/tmp}/view-skew.XXXXXX")" ||
      ro_die "no temporary directory for the id door's run, so this commit is unchecked rather than clean"
    trap 'rm -rf "$RO_WORK"' EXIT
    ro_ids_for "${view_paths[@]}"
    for p in "${view_paths[@]}"; do
      confirmed=1
      while IFS=$'\t' read -r token kind; do
        ro_id_is "$token" "$kind" && continue
        confirmed=0
        unconfirmed="$unconfirmed  $p -- the door answers $(ro_id_answer "$token") for $kind $token"$'\n'
        break
      done <<<"$(ro_st_tokens "$p")"
      [ "$confirmed" -eq 1 ] && thread_views="$thread_views  $p"$'\n'
    done
  fi
fi

if [ "$rc" -eq 0 ]; then
  if [ "$TRIGGERED" -eq 1 ] && [ "$checked" -eq 0 ]; then
    echo "skew: no apparatus view was touched by this change -- nothing for THIS check to do."
  else
    echo "skew: $checked apparatus view(s) match their canon; $(printf '%s' "$UNCHECKABLE" | grep -c .) declared un-re-derivable; $(printf '%s' "$AUTHORED" | grep -c .) authored (nothing to check)."
  fi
  if [ -n "$thread_views" ]; then
    # THIS TOOL'S BOUNDARY IS THIS TOOL'S TO STATE; THE SIBLING'S GATE STATUS IS NOT.
    #
    # This paragraph used to end `and no gate runs it. This is a GAP`. That was
    # true when written and expired 2026-08-27 when thread_view_skew_check.sh was
    # admitted to the gate -- after which this printed "no gate runs it" a few
    # lines from the sibling's own output examining every view in the same run.
    # Its function inverted: written to stop a committer reading silence as a
    # green, it came to instruct them to distrust a property just verified, and
    # to re-run by hand a check that had already run (issue 0112).
    #
    # THE FIX IS NOT TO SAY `gated in this same run` INSTEAD. That is the same
    # defect one generation later -- a claim about ANOTHER tool's roster status,
    # transcribed here, expiring silently the day the roster changes. The roster
    # is the authority and it is greppable; a status copied out of it is a second
    # home for a fact that already has one.
    echo "skew: NOT CHECKED -- this change carries thread view(s), which this tool does not cover:"
    printf '%s' "$thread_views"
    echo "      Their check is thread_view_skew_check.sh. WHETHER IT RUNS AT COMMIT TIME IS THE"
    echo "      ROSTER'S ANSWER, NOT THIS TOOL'S -- runner_roster_check.sh is the authority."
    echo "      What this line records is the boundary of THIS check: the files above are"
    echo "      outside its population, so its result above says nothing about them."
  fi
  if [ -n "$unconfirmed" ]; then
    # Shaped like a thread view, and the door does not answer it as one, so it is
    # named here with the door's answer rather than dropped from the list above.
    echo "skew: NOT A THREAD VIEW by the id door's answer, so neither this tool nor thread_view_skew_check.sh covers it:"
    printf '%s' "$unconfirmed"
  fi
fi

exit $rc
