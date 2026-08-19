#!/bin/bash
# intentfiles_reviewable.sh -- a realisation change is reviewable: the manifest
# line that caused it lands in the same working tree.
#
# COVERS ST0057 AC-02.4, AND IS CITED BY AT-02.4. Both ids are spelled out
# because the row and the file each look correct alone and only the PAIR can be
# wrong -- `intent at lint` refused this file for naming the criterion and not
# the row, which is the same defect vc hit on AT-03.18.
#
# The committed-manifest ruling rests on reviewability:
# `.intentfiles` is tracked so that when realisation changes, a reader of the
# diff sees the CAUSE beside the EFFECT. Files appearing or vanishing with no
# visible reason is the state the ruling exists to prevent -- and it is not a
# hypothetical, because dehydration removes committed files.
#
# THE PROPERTY HOLDS TODAY BY CONSTRUCTION, AND THAT IS EXACTLY WHY THIS TOOL
# EXISTS. `organize` decides realisation from the manifest's declared set alone
# and NEVER WRITES THE MANIFEST BACK -- there is no `intentfiles::render` call
# anywhere in `organize.rs`. So the declared set can only change when a human
# edits the file, which puts the cause in the working tree by definition.
# Arm 3 is what notices if that ever stops being true: the moment `organize`
# gains a generated-region write (which `intentfiles::render` exists to do, and
# which AC-02.3 is about), realisation can change with nobody having edited
# anything, and the reviewability question has to be answered again rather than
# inherited.
#
# A tool asserting only arm 2 would pass forever and notice nothing, because it
# would be testing the human's own edit.
#
# EXIT CODES. 0 pass, 1 a real violation, 2 CANNOT MEASURE. The third is not
# decoration: on this estate `organize --apply` currently changes nothing,
# because the ship gate refuses every removal while preconditions are unmet, so
# a run that observed no realisation change and reported success would be the
# vacuous pass -- right verb, right depth, a population that cannot contain the
# failure. Every arm prints what it examined.
#
# IT NEVER TOUCHES THE LIVE ESTATE. Arm 2 and arm 3 run `organize --apply`,
# which removes committed files, so they run in a DISPOSABLE CLONE made with
# `git clone --local`. The live tree is read for arm 1 and nothing else. Four
# nodes work in this checkout; an instrument that mutates it is a second writer
# with the worst possible timing.

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel)"
BIN="${INTENT_BIN:-$ROOT/native/rust/target/debug/intent}"
MANIFEST="intent/.intentfiles"
rc=0

say() { printf '%s\n' "$*"; }
fail() { printf 'FAIL: %s\n' "$*"; rc=1; }
unmeasured() { printf 'CANNOT MEASURE: %s\n' "$*"; [ "$rc" -eq 0 ] && rc=2; }

# The first thread id in canon, by GLOB rather than by parsing `ls`
# (IN-SH-CODE-002). The ids here cannot contain a space, so `ls` would have
# worked -- and that is the argument the rule exists to refuse, because the
# next directory it gets copied into is the one where it does not.
first_thread_id() {
  local f
  for f in "$1"/intent/.canon/st/*.json; do
    [ -e "$f" ] || return 1
    basename "$f" .json
    return 0
  done
  return 1
}

# ---------------------------------------------------------------------------
# Arm 1 -- the manifest is TRACKED. Half the criterion's literal text, and the
# cheapest thing here: an untracked manifest makes every other arm meaningless,
# because the cause would never appear in anybody's diff at all.
# ---------------------------------------------------------------------------
arm1_tracked() {
  if git -C "$ROOT" ls-files --error-unmatch "$MANIFEST" >/dev/null 2>&1; then
    say "arm1: $MANIFEST is tracked"
  else
    fail "arm1: $MANIFEST is NOT tracked -- a realisation change can never be reviewed, because the cause is invisible to git"
  fi
}

# A disposable clone at HEAD. `--local` is cheap and `--no-hardlinks` keeps the
# clone's object churn out of the live repository.
make_clone() {
  local dest="$1"
  git clone --quiet --local --no-hardlinks "$ROOT" "$dest" 2>/dev/null || return 1
  [ -x "$BIN" ] || return 1
  return 0
}

# Every path git reports as changed, one per line, porcelain-stable.
changed_paths() {
  git -C "$1" status --porcelain=v1 -- . | sed 's/^...//'
}

# ---------------------------------------------------------------------------
# Arm 2 -- a DECLARED-SET change lands together with its files.
#
# Constructed rather than observed, and the estate's own zero is printed below:
# the live manifest declares nothing and the gate refuses every removal, so
# there is no realisation change on this estate to watch.
# ---------------------------------------------------------------------------
arm2_cause_beside_effect() {
  local work; work="$(mktemp -d)"; trap 'rm -rf "$work"' RETURN
  local clone="$work/clone"
  if ! make_clone "$clone"; then
    unmeasured "arm2: could not make a clone or find the binary at $BIN"
    return
  fi

  local victim; victim="$(first_thread_id "$clone")" || victim=""
  if [ -z "$victim" ]; then
    unmeasured "arm2: the clone carries no threads, so no realisation can change"
    return
  fi

  # **THE DEHYDRATED STATE IS COMMITTED IN THE CLONE, and the first version of
  # this arm did not do that and measured nothing.** Deleting the file and
  # letting `organize` put it back is INVISIBLE to `git status`: the file
  # returns to its HEAD content, so the changed-path set is empty and the arm
  # reported "no realisation change" over a run that had hydrated correctly.
  # A restore-to-baseline is a real effect with no diff, and this arm's whole
  # observable is the diff.
  rm -f "$clone/intent/st/$victim/info.md"
  git -C "$clone" -c user.email=t@t -c user.name=t commit --quiet -a -m "dehydrated" 2>/dev/null

  # Now the CAUSE: declare it. The file reappearing is the EFFECT, and both
  # must land in one changed-path set for the diff to be reviewable.
  printf 'STEELTHREAD:%s  # pinned by intentfiles_reviewable.sh\n' "$victim" \
    | cat - "$clone/$MANIFEST" > "$clone/$MANIFEST.new" && mv "$clone/$MANIFEST.new" "$clone/$MANIFEST"

  # **THE VERB IS DRIVEN WITH ITS OUTPUT READ, NEVER SUPPRESSED.** The first
  # version piped it to /dev/null, which is the defect AC-03.15 was minted from
  # -- a write verb whose refusal nobody saw. Its summary line is reported
  # below whatever the verdict.
  local out; out="$( cd "$clone" && "$BIN" organize --apply 2>&1 )"
  say "arm2: organize said -- $(printf '%s\n' "$out" | head -1)"

  local changed; changed="$(changed_paths "$clone")"
  local n_files; n_files="$(printf '%s\n' "$changed" | grep -c "^intent/st/$victim/" || true)"
  local has_manifest; has_manifest="$(printf '%s\n' "$changed" | grep -c "^$MANIFEST\$" || true)"

  say "arm2: examined $victim -- $n_files changed path(s) under its directory, manifest in the set: $has_manifest"
  if [ "$n_files" -eq 0 ]; then
    unmeasured "arm2: no realisation change was produced, so nothing was reviewed -- this is not a pass"
    return
  fi
  if [ "$has_manifest" -eq 0 ]; then
    fail "arm2: $victim's files changed and $MANIFEST did not -- the effect is in the diff and the cause is not"
  fi
}

# ---------------------------------------------------------------------------
# Arm 3 -- THE ONE THAT WILL EVER FIRE. If `organize` changes the realised set
# WITHOUT the manifest changing, the property is broken for a reason no human
# edit explains.
#
# It runs organize with the manifest UNTOUCHED. Today the declared set cannot
# move on its own, so the realised set must not move either. If a future
# `organize` writes its generated region from status, this arm forces the
# manifest into the same changed set rather than letting realisation drift
# silently.
# ---------------------------------------------------------------------------
arm3_nothing_moves_realisation_but_the_manifest() {
  local work; work="$(mktemp -d)"; trap 'rm -rf "$work"' RETURN
  local clone="$work/clone"
  if ! make_clone "$clone"; then
    unmeasured "arm3: could not make a clone or find the binary at $BIN"
    return
  fi

  local before; before="$(find "$clone/intent/st" -name '*.md' | wc -l | tr -d ' ')"
  local out; out="$( cd "$clone" && "$BIN" organize --apply 2>&1 )"
  say "arm3: organize said -- $(printf '%s\n' "$out" | head -1)"
  local after; after="$(find "$clone/intent/st" -name '*.md' | wc -l | tr -d ' ')"

  local changed; changed="$(changed_paths "$clone")"
  local has_manifest; has_manifest="$(printf '%s\n' "$changed" | grep -c "^$MANIFEST\$" || true)"

  say "arm3: realised views $before -> $after with the manifest untouched, manifest in the changed set: $has_manifest"
  if [ "$before" -ne "$after" ] && [ "$has_manifest" -eq 0 ]; then
    fail "arm3: organize moved realisation from $before to $after and $MANIFEST did not change -- a reader of this diff sees files appear or vanish with nothing naming the cause"
  fi
  if [ "$before" -eq "$after" ]; then
    say "arm3: realisation did not move, which is the CORRECT answer while the declared set can only change by hand -- recorded, not counted as evidence of the guard firing"
  fi
}

# ---------------------------------------------------------------------------
# SELF-TEST -- the arms are driven against worlds that VIOLATE the criterion,
# and each must FAIL on its own.
#
# **A clean sweep is unbankable until it has been run against known positives.**
# Every arm here reports PASS on a healthy estate, and that is exactly the
# reading a broken arm produces. The two violations below are constructible;
# arm 3's is not, and that is stated rather than skipped.
# ---------------------------------------------------------------------------
selftest() {
  local work; work="$(mktemp -d)"; trap 'rm -rf "$work"' RETURN
  local failures=0 checked=0

  # Positive control 1: the manifest is UNTRACKED. Arm 1 must fail.
  local c1="$work/untracked"
  if make_clone "$c1"; then
    git -C "$c1" rm --quiet --cached "$MANIFEST" >/dev/null 2>&1
    checked=$((checked + 1))
    if ( ROOT="$c1"; rc=0; arm1_tracked >/dev/null 2>&1; [ "$rc" -eq 1 ] ); then
      say "selftest: arm1 FAILS on an untracked manifest -- correct"
    else
      say "selftest: ARM 1 DID NOT FIRE on an untracked manifest"
      failures=$((failures + 1))
    fi
  fi

  # Positive control 2: files change and the manifest does NOT. Arm 2's guard
  # is the `has_manifest` test, driven here directly against a world where the
  # effect is present and the cause is absent.
  local c2="$work/nocause"
  if make_clone "$c2"; then
    local v; v="$(first_thread_id "$c2")" || v=""
    printf '\nrealisation changed with nobody saying so\n' >> "$c2/intent/st/$v/info.md"
    checked=$((checked + 1))
    local changed; changed="$(changed_paths "$c2")"
    local n; n="$(printf '%s\n' "$changed" | grep -c "^intent/st/$v/" || true)"
    local m; m="$(printf '%s\n' "$changed" | grep -c "^$MANIFEST\$" || true)"
    if [ "$n" -gt 0 ] && [ "$m" -eq 0 ]; then
      say "selftest: the effect-without-cause world is constructible and arm2's guard reads it -- $n file(s) changed, manifest in the set: $m"
    else
      say "selftest: COULD NOT CONSTRUCT the effect-without-cause world (n=$n m=$m)"
      failures=$((failures + 1))
    fi
  fi

  # **ARM 3 HAS NO CONSTRUCTIBLE POSITIVE CONTROL AND IT IS SAID RATHER THAN
  # OMITTED.** Its violating world needs an `organize` that moves realisation
  # from something other than the declared set, and no such build exists --
  # which is the same fact that makes arm 3 pass today. So arm 3 is UNPROVEN by
  # this self-test, not proven by its silence, and the day somebody teaches
  # `organize` to write its generated region is the day this becomes testable
  # and must be revisited.
  say "selftest: arm3 has NO constructible positive control -- unproven, not passing"

  say "selftest: $checked control(s) driven, $failures did not fire"
  [ "$failures" -eq 0 ]
}

if [ "${1:-}" = "--selftest" ]; then
  if selftest; then say "intentfiles_reviewable: SELFTEST PASS"; exit 0; fi
  say "intentfiles_reviewable: SELFTEST FAIL"; exit 1
fi

arm1_tracked
arm2_cause_beside_effect
arm3_nothing_moves_realisation_but_the_manifest

case "$rc" in
  0) say "intentfiles_reviewable: PASS" ;;
  1) say "intentfiles_reviewable: FAIL" ;;
  2) say "intentfiles_reviewable: CANNOT MEASURE -- not a pass" ;;
esac
exit "$rc"
