#!/usr/bin/env bats
# Guard: no shell file in the Intent CLI corpus emits a capitalised `Error: `.
#
# THE PROPERTY, AND WHY IT OUTLIVES ITS OLD VEHICLE. Intent speaks one error
# voice -- lowercase `error: `, from `error()` -- and a single voice is only
# single if nothing reintroduces the other one, so the rule is grepped rather
# than read (issue 0011). This arm lived in `tests/unit/helpers.bats`, whose
# other sixteen arms all `source bin/intent_helpers`. That file is population A
# and the prune deletes it; this arm is the one occupant that never touched it,
# greps directories instead, and has a job after the delete. It is moved here
# rather than deleted with its neighbours -- vc's standard, adopted: prune the
# VEHICLE, carry the PROPERTY.
#
# THE CORPUS NEEDS NO RE-SCOPING AND THAT IS THE POINT. The three roots below
# are the same three the arm always named. `bin/` shrinks from 83 files to
# population B when the prune lands, and the plugin bin dirs are untouched, so
# the delete changes only what is WATCHED, never what is asserted.
#
# ==========================================================================
# WHAT THE MIGRATION FIXED, MEASURED RATHER THAN SUSPECTED
# ==========================================================================
#
# The arm as it stood was `run grep -rn ... ; [ "$status" -ne 0 ]`, and **that
# passes for two different reasons that mean opposite things**:
#
#   grep over the real corpus, no match   -> status 1
#   grep over a path that does not exist  -> status 2
#
# Both are `-ne 0`. So a typo in a root, a renamed directory, or a corpus that
# had gone empty would report the reassuring answer, and the arm would keep
# doing it for as long as nobody looked. **An emptiness assertion is worth
# exactly what its instrument is worth**, and this one could not fail.
#
# The three arms below separate what that one conflated: the corpus is REAL,
# the scan is CLEAN, and the detector FIRES. The last is driven against a
# planted fixture in a tempdir and never against the tracked tree -- a positive
# control that writes into the repository is a commit waiting to happen on a
# shared checkout.
#
# THE NEIGHBOUR HAS THE SAME DEFECT AND IS NOT FIXED HERE:
# `tests/unit/set_e_increment_guard.bats` branches on `[ "$status" -eq 0 ]`
# over `${INTENT_PROJECT_ROOT}/bin` and so also reads a broken instrument as a
# clean corpus. Named rather than silently repaired, because it is a different
# file with a different owner and its subject is not this one's.

load "../lib/test_helper.bash"

# The corpus: every shell entrypoint Intent ships, in the three roots that hold
# them. Kept as a function so all three arms drive the same instrument -- a
# positive control over a DIFFERENT invocation from the one it vouches for is
# decoration.
corpus_roots() {
  echo "${INTENT_PROJECT_ROOT}/bin"
  echo "${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin"
  echo "${INTENT_PROJECT_ROOT}/intent/plugins/agents/bin"
}

scan() {
  grep -rn '"Error: ' "$@"
}

@test "the corpus this guard scans is real, and its size is reported" {
  local roots=()
  while IFS= read -r r; do roots+=("$r"); done < <(corpus_roots)

  for root in "${roots[@]}"; do
    [ -d "$root" ] || fail "corpus root is missing, so every green below is a statement about the instrument: $root"
  done

  # The denominator, asserted rather than assumed. An empty corpus and a clean
  # corpus print the same thing downstream, and only this arm can tell them
  # apart.
  local n
  n=$(find "${roots[@]}" -type f | wc -l | tr -d ' ')
  [ "$n" -gt 0 ] || fail "the corpus is empty -- a scan of nothing cannot report a voice"
  echo "corpus: $n file(s) across ${#roots[@]} root(s)"
}

@test "no shell command emits a capitalised Error: prefix" {
  local roots=()
  while IFS= read -r r; do roots+=("$r"); done < <(corpus_roots)

  run scan "${roots[@]}"

  # THREE OUTCOMES, NOT TWO. Status 2 is the instrument failing to read the
  # corpus and it is a FAILURE here, which is the whole reason this file exists.
  case "$status" in
    0) fail "capitalised Error: prefix reintroduced:
$output" ;;
    1) : ;;
    *) fail "the scan could not read the corpus (grep exit $status) -- this is a
broken instrument, not a clean estate:
$output" ;;
  esac
}

@test "the scan fires on a planted Error: and not on the lowercase voice" {
  # Driven in a tempdir. A positive control that plants into the tracked tree is
  # one `git add -A` away from being committed by any node on this checkout.
  local fixture
  fixture=$(mktemp -d)

  printf '%s\n' 'echo "Error: something went wrong" >&2' > "$fixture/offender.sh"
  run scan "$fixture"
  [ "$status" -eq 0 ] || fail "the scan did not find a planted capitalised Error:, so the green above says nothing"
  [[ "$output" == *"offender.sh"* ]] || fail "the scan matched but did not name the file: $output"

  # And it does NOT fire on the voice the estate is supposed to speak, or a
  # guard that reports the fix as the defect gets switched off.
  rm -f "$fixture/offender.sh"
  printf '%s\n' 'error "something went wrong"' > "$fixture/correct.sh"
  printf '%s\n' 'echo "error: something went wrong" >&2' >> "$fixture/correct.sh"
  run scan "$fixture"
  [ "$status" -eq 1 ] || fail "the scan fired on the correct lowercase voice: $output"

  rm -rf "$fixture"
}
