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

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
RECORD="${RECORD_OVERRIDE:-$ROOT/native/rust/target/dist-provenance.txt}"
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

  [ "$ok" -eq 1 ] || exit 1
  printf 'provenance-fields: self-test PASSED -- both primary fields are independently enforced.\n'
  exit 0
fi

# --- the real subject --------------------------------------------------------
printf 'provenance-fields: subject %s\n' "${RECORD#"$ROOT"/}"
out="$(check_record "$RECORD")"
rc=$?
if [ "$rc" -eq 0 ]; then
  printf 'provenance-fields: both primary fields present and labelled; drift HELD with a release condition.\n'
  printf 'provenance-fields: REACH -- this checks that the fields EXIST and are LABELLED.\n'
  printf '    It cannot check that a source commit is HONEST about the bytes. That is the\n'
  printf '    drift field, and drift is HELD.\n'
  exit 0
fi

printf '%s\n' "$out"
printf 'provenance-fields: the record does not carry the partition AC-11.7 requires.\n'
printf '    Written by `int macos stage` -- see bin/.devbin/cmd/macos.\n'
exit 1
