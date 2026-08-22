#!/bin/bash
# provenance_fields_check.sh -- an artefact's durable record carries two primary
# fields, each naming the question it answers, and holds a third with a named
# release condition. Covers ST0056 AC-11.7 (AT-11.7).
#
# WHAT THIS CHECKS AND WHAT IT CANNOT.
#
# It checks that the FIELDS EXIST AND ARE LABELLED. It cannot check that a source
# commit is HONEST about the bytes -- that is the drift field's job and the drift
# field is HELD, which is why this tool reports the hold rather than omitting it.
# An omitted field reads as a closed partition; a held one reads as a work-list.
#
# WHY TWO PRIMARY FIELDS AND NOT ONE, WHICH IS THE WHOLE POINT OF THE ROW.
# The estate holds a MEASURED failure of each field alone, and neither subsumes
# the other:
#
#   HASH ALONE FAILS AT CURRENCY. `f2e4d1f9005d0334` matched its record exactly
#   for ten hours while refusing every project verb -- 158 commits behind, and
#   every hash in the pipeline agreed with every other hash the whole time.
#
#   SOURCE COMMIT ALONE FAILS AT IDENTITY. `dirty-18197aaf` was carried by two
#   structurally different artefacts in ONE build -- `intent` at 9,008,848 bytes
#   and `intentd` at 373,136 -- and by three distinct binaries in one day.
#
# So neither is secondary and demoting either reopens what the other was minted
# to close. dc and ic independently proposed demoting the hash; the convergence
# was NOT evidence, because both had the same blind spot.
#
# BOTH POSITIVE CONTROLS ARE REQUIRED BEFORE A CLEAN RESULT (--self-test).
# An instrument that catches only one missing field has been shown to work on one
# field. `--self-test` drives a record missing each field in turn and REFUSES to
# report itself sound unless it goes non-zero on both, independently.
#
# FIELD COUNT IS NOT MATURITY. `dist-provenance.txt` carried commit + verdict +
# reason where other records had one field, and three nodes cited it as the
# exemplar on that basis -- while it was describing a build 805 commits and two
# days old and saying nothing about it. Accurate, well-formed, and silent about
# the property the reader needed. This tool asks what a field ANSWERS, never how
# many there are.
#
# AND EVERYTHING ABOVE IS ABOUT ONE RECORD, WHICH IS WHY IT MISSED A PROPERTY OF
# THE SET (cc, 2026-08-21, on real bytes, while I was calling this tool sound).
# A per-record check asks whether a FIELD exists and is labelled. Two
# individually impeccable records -- or one record and two artefacts -- can each
# be well-formed and still describe NO SINGLE STATE of this repo. Nothing
# per-record will ever fire on that, because nothing per-record is malformed.
#
# MEASURED ON DISK THE DAY THIS ARM WAS WRITTEN:
#
#   dist-provenance.txt        commit: 26fe1aea...     subject: the CHECKOUT
#   target/release/intent      dirty-483e65e4...       subject: itself
#   target/release/intentd     dirty-5819417b...       subject: itself
#
# THREE disagreements: neither artefact came from the recorded commit, and the
# two came from different trees as each other.
#
# THE RECORD WAS NOT LYING, AND THAT IS THE INTERESTING PART. It said
# `checkout_clean: no` and warned that `target/release may hold bytes that match
# no commit`. It flags the RISK and structurally cannot name WHICH bytes, because
# it holds ONE `commit:` for a SET while each artefact answers only for itself.
# An honest, correct record that cannot carry the property. So the remedy is not
# a better record -- it is a check whose subject is the RELATIONSHIP.
#
# THIS IS NOT `int macos publish`'s CHECK AND MUST NOT BECOME A COPY OF IT.
# `artefact_commit_blockers` (bin/.devbin/cmd/macos) tests each artefact against
# the TAG at release time, so the set property falls out transitively from a
# common pivot -- and it is SOUND: two `dirty-` markers fail its dirty arm twice
# over, independently. This asks a different question against a different pivot
# at a different time: DOES THIS RECORD DESCRIBE THE BYTES BESIDE IT. A
# release-time check cannot answer that before a release, and most of a binary's
# life is before one.
#
# THE MARKER FORMAT IS A CONTRACT WITH TWO READERS: `marker_of` below, and
# `artefact_source_commit()` in `bin/.devbin/cmd/macos`. Change the spelling and
# BOTH must move. That is two readers of a documented format rather than two
# implementations of a decision, which is why it is tolerated -- and it is named
# here so a format change can find them both.

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
RECORD="${RECORD_OVERRIDE:-$ROOT/native/rust/target/dist-provenance.txt}"
ARTEFACT_DIR="${ARTEFACT_DIR_OVERRIDE:-$ROOT/native/rust/target/release}"
# The set this record is supposed to be about. Kept beside $BINARIES in
# `bin/.devbin/cmd/macos` rather than derived from it: a parity instrument that
# imported the CLI's list would go silent the moment the CLI could not be
# sourced, and a silent instrument is the failure this file exists to refuse.
ARTEFACTS="intent intentd"
SELFTEST=0

die() { printf 'provenance-fields: %s\n' "$1" >&2; exit 2; }

while [ $# -gt 0 ]; do
  case "$1" in
    --self-test) SELFTEST=1; shift ;;
    --record) RECORD="${2:?--record needs a path}"; shift 2 ;;
    -*) die "unknown option: $1" ;;
    *) die "unexpected argument: $1" ;;
  esac
done

# check_record <path> -- 0 if every required field is present and labelled.
# Prints one line per defect. Never mutates the record.
#
# THE LABEL IS THE POINT, NOT THE VALUE. A record carrying a bare `commit:` has
# the datum and not the claim, and the reader cannot tell which of the two
# questions it was meant to answer. That is the exact state `dist-provenance.txt`
# was in when three nodes read it as the estate's best record.
check_record() {
  local f="$1" bad=0 body
  [ -f "$f" ] || { printf '  MISSING RECORD -- %s does not exist\n' "$f"; return 1; }
  body="$(cat "$f")"

  # 1. IDENTITY -- the artefact hash, and a label saying it answers identity.
  if ! printf '%s' "$body" | grep -qE '^artefact_sha256:'; then
    printf '  MISSING FIELD artefact_sha256 -- nothing answers IDENTITY.\n'
    printf '    Without it, one source commit covers structurally different artefacts:\n'
    printf '    dirty-18197aaf was carried by intent at 9008848 bytes and intentd at 373136.\n'
    bad=1
  elif ! printf '%s' "$body" | grep -qE '^ *answers: IDENTITY'; then
    printf '  UNLABELLED artefact_sha256 -- present, but does not name the question it answers.\n'
    bad=1
  fi

  # 2. CURRENCY -- the source commit, and a label saying it answers currency.
  if ! printf '%s' "$body" | grep -qE '^(commit|source_commit):'; then
    printf '  MISSING FIELD source commit -- nothing answers CURRENCY.\n'
    printf '    Without it, a correct hash certifies a stale artefact:\n'
    printf '    f2e4d1f9005d0334 matched its record exactly for ten hours, 158 commits behind.\n'
    bad=1
  elif ! printf '%s' "$body" | grep -qE '^ *answers: CURRENCY'; then
    printf '  UNLABELLED source commit -- present, but does not name the question it answers.\n'
    bad=1
  fi

  # 3. DRIFT -- HELD, and the hold must name its release condition.
  #    A hold with a named condition is a covered property; a hold without one is
  #    a permanent exemption with no work-list. Omitting it entirely is worse than
  #    either, because a two-field record reads as a CLOSED partition.
  if ! printf '%s' "$body" | grep -qE '^drift:'; then
    printf '  MISSING FIELD drift -- the partition reads as CLOSED over two properties.\n'
    printf '    Neither primary expresses how much uncommitted code the artefact holds.\n'
    bad=1
  elif ! printf '%s' "$body" | grep -qiE '^drift: *HELD'; then
    printf '  drift is present but not declared HELD -- this tool cannot verify drift,\n'
    printf '    so a record CLAIMING it is asserting a property nothing here measured.\n'
    bad=1
  elif ! printf '%s' "$body" | grep -qE '^ *release condition:'; then
    printf '  drift is HELD with NO RELEASE CONDITION -- that is a permanent exemption.\n'
    bad=1
  fi

  return "$bad"
}

# marker_of <file> -- the source commit the BYTES name, read off the bytes.
# One of the two readers of the marker contract; the other is
# `artefact_source_commit()` in bin/.devbin/cmd/macos.
#
# `|| true` guards `pipefail`: `grep -o` exits 1 on no match and would otherwise
# make the assignment itself non-zero, so the absent-marker case would be
# indistinguishable from a broken `strings`.
# marker_of -- DELEGATES to the one extraction site as of 2026-08-22.
#
# THIS BODY WAS BYTE-FOR-BYTE `artefact_source_commit`, UNDER A DIFFERENT NAME,
# WRITTEN BY THE SAME NODE ON THE DAY BEFORE THAT NODE WROTE `artefact.lib`'s
# header claiming to be THE ONE EXTRACTION SITE. Four implementations of one
# contract existed; this was the starkest, because it was not even an inline
# fragment -- it was the whole function, renamed.
#
# NO FALLBACK (vc's ruling). An absent lib is a hard stop, not a quiet local copy.
_ART_LIB="$ROOT/bin/.devbin/cmd/shared/artefact.lib"
[ -f "$_ART_LIB" ] || {
  echo "${0##*/}: cannot read $_ART_LIB -- the source-commit marker parser has ONE home and this is not it." >&2
  exit 2
}
# shellcheck source=/dev/null
. "$_ART_LIB"
marker_of() { artefact_source_commit "$1"; }

# check_artefact_set <record> <dir> -- does this record describe the bytes
# beside it, and do those bytes agree with EACH OTHER?
#
# TWO PIVOTS, NOT ONE, because they fail independently. An artefact can disagree
# with the RECORD while the set agrees internally (a stale record beside a
# coherent build), and the set can disagree internally while every member
# matches nothing (two dirty trees). Reporting one would hide the other.
check_artefact_set() {
  local rec="$1" dir="$2" bad=0 present=0 first="" first_name="" name f m rec_commit warned=""
  rec_commit="$(sed -n -e 's/^commit: //p' -e 's/^source_commit: //p' "$rec" 2>/dev/null | head -1)" || true
  grep -qE '^checkout_clean: *no' "$rec" 2>/dev/null && warned="yes"

  for name in $ARTEFACTS; do
    f="$dir/$name"
    [ -f "$f" ] || continue
    present=$((present + 1))
    if ! m="$(marker_of "$f")"; then
      printf '  %s carries NO source-commit marker -- its bytes name no commit, so no record can be shown to be about them.\n' "$name"
      bad=1
      continue
    fi
    if [ -z "$first" ]; then
      first="$m"
      first_name="$name"
    elif [ "$m" != "$first" ]; then
      printf '  SET DISAGREES -- %s names %s; %s names %s.\n' "$first_name" "$first" "$name" "$m"
      printf '    They were built from DIFFERENT trees, so the set on disk corresponds to no single\n'
      printf '    state of this repo. Every record beside them can be well-formed and this stays true.\n'
      bad=1
    fi
    if [ -n "$rec_commit" ] && [ "$m" != "$rec_commit" ]; then
      printf '  %s names %s, but the record names %s -- the record is not about these bytes.\n' "$name" "$m" "$rec_commit"
      bad=1
    fi
  done

  # AN ARM THAT EXAMINED NOTHING READS EXACTLY LIKE ONE THAT FOUND NOTHING
  # WRONG, and this estate spent 2026-08-21 on that class. Absence is reported
  # as absence and never folded into the pass.
  if [ "$present" -eq 0 ]; then
    printf '  SET NOT EXAMINED -- no artefacts in %s.\n' "$dir"
    printf '    That is not a pass. Build them, or point ARTEFACT_DIR_OVERRIDE at them.\n'
    return 0
  fi

  if [ "$bad" -ne 0 ] && [ -n "$warned" ]; then
    printf '    NOTE: the record already says `checkout_clean: no`, so it FLAGGED this risk honestly.\n'
    printf '    What it cannot say is WHICH bytes, or that they disagree with one another -- it holds\n'
    printf '    ONE commit for a SET. The record is correct and structurally silent, which is the\n'
    printf '    whole reason this arm exists. Do not "fix" it by editing the record.\n'
  fi
  return "$bad"
}

# --- self-test ---------------------------------------------------------------
# Drives one synthetic record per primary field, each missing exactly that field.
# REFUSES to pass unless BOTH go non-zero, because an instrument that catches one
# has been shown to work on one.
if [ "$SELFTEST" -eq 1 ]; then
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT
  ok=1

  # Control 1 -- the CURRENCY failure: a correct hash, no source commit.
  cat > "$tmp/no_currency.txt" <<'EOF'
artefact_sha256: f2e4d1f9005d0334
  answers: IDENTITY -- which of several builds these bytes are
drift: HELD
  release condition: computed over the paths the claim is about
EOF
  if check_record "$tmp/no_currency.txt" >/dev/null 2>&1; then
    printf 'provenance-fields: SELF-TEST FAILED -- a record with NO SOURCE COMMIT passed.\n' >&2
    printf '  This is the f2e4d1f9005d0334 case: correct hash, 158 commits behind.\n' >&2
    ok=0
  else
    printf 'provenance-fields: control 1 (currency) -- a record with no source commit is REFUSED.\n'
  fi

  # Control 2 -- the IDENTITY failure: a source commit, no artefact hash.
  cat > "$tmp/no_identity.txt" <<'EOF'
source_commit: dirty-18197aaf
  answers: CURRENCY -- which commit the source came from
drift: HELD
  release condition: computed over the paths the claim is about
EOF
  if check_record "$tmp/no_identity.txt" >/dev/null 2>&1; then
    printf 'provenance-fields: SELF-TEST FAILED -- a record with NO ARTEFACT HASH passed.\n' >&2
    printf '  This is the dirty-18197aaf case: one marker, two artefacts of different sizes.\n' >&2
    ok=0
  else
    printf 'provenance-fields: control 2 (identity) -- a record with no artefact hash is REFUSED.\n'
  fi

  # Control 3 -- a fully compliant record must PASS, or the tool refuses everything
  # and its two refusals above prove nothing.
  cat > "$tmp/compliant.txt" <<'EOF'
artefact_sha256: 957aa2b2e9029f5b
  answers: IDENTITY -- which of several builds these bytes are
source_commit: 26fe1aea94f0ffa4d98998065d61daa0240ecc5f
  answers: CURRENCY -- which commit the source came from
drift: HELD
  release condition: computed over the paths the claim is about (AC-11.7)
EOF
  if check_record "$tmp/compliant.txt" >/dev/null 2>&1; then
    printf 'provenance-fields: control 3 (green) -- a compliant record is ACCEPTED.\n'
  else
    printf 'provenance-fields: SELF-TEST FAILED -- a compliant record was REFUSED.\n' >&2
    printf '  A tool that refuses everything has not been shown to discriminate.\n' >&2
    ok=0
  fi

  # --- the SET arm's controls -------------------------------------------------
  # SAME DISCIPLINE AS ABOVE, AND THE TWO PIVOTS ARE ISOLATED FROM EACH OTHER.
  # A set whose members disagree ALSO disagrees with any record naming one of
  # them, so a naive control would fire both arms and prove neither. Control 5
  # therefore uses a record with NO commit field, which switches the record arm
  # off and leaves only the set pivot able to speak.
  mkdir -p "$tmp/art"
  printf 'commit: aaaaaaaaaaaa\n' > "$tmp/rec_set.txt"
  printf 'checkout_clean: yes\n' > "$tmp/rec_nocommit.txt"

  mkset() {
    printf 'pad\n[intent-source-commit:%s]\npad\n' "$1" > "$tmp/art/intent"
    printf 'pad\n[intent-source-commit:%s]\npad\n' "$2" > "$tmp/art/intentd"
  }

  # Control 4 -- a coherent set matching its record must PASS, or the three
  # refusals below prove only that this arm refuses everything.
  mkset aaaaaaaaaaaa aaaaaaaaaaaa
  if check_artefact_set "$tmp/rec_set.txt" "$tmp/art" >/dev/null 2>&1; then
    printf 'provenance-fields: control 4 (set green) -- a coherent set matching its record is ACCEPTED.\n'
  else
    printf 'provenance-fields: SELF-TEST FAILED -- a coherent set matching its record was REFUSED.\n' >&2
    ok=0
  fi

  # Control 5 -- THE SET PIVOT, isolated: two artefacts from different trees,
  # against a record that names no commit at all.
  mkset aaaaaaaaaaaa bbbbbbbbbbbb
  if check_artefact_set "$tmp/rec_nocommit.txt" "$tmp/art" >/dev/null 2>&1; then
    printf 'provenance-fields: SELF-TEST FAILED -- a set whose members DISAGREE passed.\n' >&2
    printf '  This is the dirty-483e65e4 / dirty-5819417b case: one record, two trees, both well-formed.\n' >&2
    ok=0
  else
    printf 'provenance-fields: control 5 (set pivot) -- members built from DIFFERENT trees are REFUSED.\n'
  fi

  # Control 6 -- THE RECORD PIVOT, isolated: an internally coherent set that the
  # record is simply not about.
  mkset cccccccccccc cccccccccccc
  if check_artefact_set "$tmp/rec_set.txt" "$tmp/art" >/dev/null 2>&1; then
    printf 'provenance-fields: SELF-TEST FAILED -- a coherent set the record is NOT about passed.\n' >&2
    ok=0
  else
    printf 'provenance-fields: control 6 (record pivot) -- a coherent set the record is not about is REFUSED.\n'
  fi

  # Control 7 -- ABSENCE MUST ANNOUNCE ITSELF. An arm that examined nothing
  # reads exactly like one that found nothing wrong, so the empty case is
  # required to SAY it examined nothing rather than to pass quietly.
  rm -f "$tmp/art/intent" "$tmp/art/intentd"
  if check_artefact_set "$tmp/rec_set.txt" "$tmp/art" 2>/dev/null | grep -q 'SET NOT EXAMINED'; then
    printf 'provenance-fields: control 7 (absence) -- an empty set reports NOT EXAMINED rather than passing silently.\n'
  else
    printf 'provenance-fields: SELF-TEST FAILED -- an empty artefact set did not announce itself.\n' >&2
    ok=0
  fi

  [ "$ok" -eq 1 ] || exit 1
  printf 'provenance-fields: self-test PASSED -- both primary fields independently enforced,\n'
  printf '  and the SET arm shown able to fire on each pivot separately and to stay quiet when coherent.\n'
  exit 0
fi

# --- the real subject --------------------------------------------------------
printf 'provenance-fields: subject %s\n' "${RECORD#"$ROOT"/}"
printf 'provenance-fields: set     %s\n' "${ARTEFACT_DIR#"$ROOT"/}"

out="$(check_record "$RECORD")"
rc_fields=$?
set_out="$(check_artefact_set "$RECORD" "$ARTEFACT_DIR")"
rc_set=$?

# BOTH ARMS ALWAYS RUN AND ARE ALWAYS REPORTED SEPARATELY, because they have
# DIFFERENT SUBJECTS: the record's own shape, and the record's relationship to
# the bytes beside it. Folding them into one verdict would let a reader credit
# one with the other's coverage -- which is the compound-artefact mistake this
# row exists to refuse, and it is how a per-record check came to be read as
# covering a set for as long as it did.
if [ "$rc_fields" -eq 0 ]; then
  printf 'provenance-fields: FIELDS ok -- both primary fields present and labelled; drift HELD with a release condition.\n'
else
  printf 'provenance-fields: FIELDS -- the record does not carry the partition AC-11.7 requires:\n'
  printf '%s\n' "$out"
  printf '    Written by `int macos stage` -- see bin/.devbin/cmd/macos.\n'
fi

if [ -z "$set_out" ]; then
  printf 'provenance-fields: SET ok -- every staged artefact names the same commit, and it is the one the record names.\n'
elif [ "$rc_set" -eq 0 ]; then
  printf 'provenance-fields: SET -- NOT ESTABLISHED (this is not a pass):\n'
  printf '%s\n' "$set_out"
else
  printf 'provenance-fields: SET -- this record does not describe the bytes beside it:\n'
  printf '%s\n' "$set_out"
fi

printf 'provenance-fields: REACH -- FIELDS asks whether fields EXIST and are LABELLED. SET asks whether\n'
printf '    the record and the artefacts name ONE commit between them. NEITHER can check that an\n'
printf '    embedded marker is HONEST about the bytes carrying it -- that is the drift field, and\n'
printf '    drift is HELD. Release-time enforcement is `artefact_commit_blockers` in\n'
printf '    bin/.devbin/cmd/macos, which pivots on the TAG rather than on this record.\n'

if [ "$rc_fields" -ne 0 ] || [ "$rc_set" -ne 0 ]; then
  exit 1
fi
exit 0
