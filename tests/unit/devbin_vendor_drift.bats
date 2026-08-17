#!/usr/bin/env bats
# `int vendor` reports which devbin-owned files this project has locally patched.
#
# WHY IT EXISTS (issue 0048). `bin/.devbin/manifest.sha256` names the files devbin
# OWNS and its header states the contract: an edit is DETECTED, not overwritten.
# So a local patch is a SUPPORTED state. What was missing is that the detector the
# header refers to lives in the Devbin SOURCE repository, so from inside the
# project carrying the patches there was nothing to run -- the divergence had a
# size nobody knew and members nobody had enumerated. Measured 2026-08-17: the
# count named in conversation was three, the truth was four, and two of the four
# predated the discussion and were news to everyone in it.
#
# THE CONTROL THAT MATTERS HERE IS THE MATCHES COUNT, not any of the refusals.
# The first hand-run of this measurement reported ALL 27 entries diverged, because
# under zsh `path` is a special variable tied to `PATH` and `while read -r want
# path` destroys it on the first iteration -- `shasum` then cannot be found, every
# comparison fails, and every file reports as drifted. **A broken instrument
# reporting maximum alarm, which reads exactly like a catastrophic finding**, and
# far more persuasive than a wrong zero because it looks like diligence rewarded.
# So there is a test below asserting that zero matches is reported as a BROKEN RUN
# rather than as a wholly-diverged tree.

load "../lib/test_helper.bash"

VENDOR="${INTENT_VENDOR_SCRIPT:-${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/vendor}"
REAL_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib"

# A vendored tree that matches its own manifest exactly.
vendor_fixture() {
  VROOT="${TEST_TEMP_DIR}/vendor"
  rm -rf "$VROOT"
  mkdir -p "$VROOT/bin/.devbin/lib/cmd"
  printf 'echo alpha\n' >"$VROOT/bin/.devbin/lib/alpha"
  printf 'echo beta\n' >"$VROOT/bin/.devbin/lib/beta"
  printf 'echo gamma\n' >"$VROOT/bin/.devbin/lib/cmd/gamma"
  {
    printf '# devbin manifest -- written by: devbin install.\n'
    printf '# devbin_version: 9.9.9\n'
    printf '# source: /nowhere/Devbin\n'
    (cd "$VROOT" && shasum -a 256 bin/.devbin/lib/alpha bin/.devbin/lib/beta bin/.devbin/lib/cmd/gamma)
  } >"$VROOT/bin/.devbin/manifest.sha256"
}

# Drive the REAL command against the fixture, the way the dispatcher would.
run_vendor() {
  run env PROJECT_ROOT="$VROOT" DEVBIN_LIB="$REAL_LIB" bash "$VENDOR"
}

@test "int vendor is syntactically valid" {
  run bash -n "$VENDOR"
  assert_success
}

# --------------------------------------------------------------------
# Baselines first: the steady state is "stock", and it must be legible
# --------------------------------------------------------------------

@test "BASELINE: a tree matching its manifest reports stock, and says so" {
  vendor_fixture
  run_vendor
  assert_success
  assert_output_contains "3 file(s) MATCH"
  assert_output_contains "running stock devbin"
  refute_output_contains "DIVERGED"
}

@test "BASELINE: the manifest header is read, not restated" {
  # The version and source come OUT of the manifest. A report that hardcoded them
  # would keep saying 0.1.0 after an upgrade and nobody would notice.
  vendor_fixture
  run_vendor
  assert_output_contains "9.9.9"
  assert_output_contains "/nowhere/Devbin"
}

# --------------------------------------------------------------------
# The three states, each reported as itself
# --------------------------------------------------------------------

@test "a locally patched file is DIVERGED and is named" {
  vendor_fixture
  printf 'echo alpha modified\n' >>"$VROOT/bin/.devbin/lib/alpha"

  run_vendor
  assert_output_contains "DIVERGED"
  assert_output_contains "bin/.devbin/lib/alpha"
  # The other two must NOT be swept in with it -- a report that names everything
  # names nothing.
  refute_output_contains "bin/.devbin/lib/beta"
  assert_output_contains "2 file(s) MATCH"
}

@test "DIVERGED alone REPORTS rather than refuses" {
  # The deliberate call, same as `int hooks`'s bare form. A divergence nobody has
  # ruled on is not a reason to block every session in the repository, and the
  # carry-vs-fork decision is hv's -- a tool that made it by refusing would be
  # taking it.
  vendor_fixture
  printf 'echo alpha modified\n' >>"$VROOT/bin/.devbin/lib/alpha"

  run_vendor
  assert_success
  assert_output_contains "Supported, not damage"
}

@test "a manifest entry with no file is MISSING, and that DOES fail" {
  # Different from a patch in kind: the manifest says devbin put a file here and
  # it is gone, which is a broken install rather than a local decision.
  vendor_fixture
  rm "$VROOT/bin/.devbin/lib/beta"

  run_vendor
  assert_failure
  assert_output_contains "MISSING"
  assert_output_contains "bin/.devbin/lib/beta"
}

@test "a file the manifest never listed is UNLISTED, not DIVERGED" {
  # An edit to a known file is a patch the vendoring contract anticipates. A NEW
  # file under the owned tree is something it has no opinion about at all, and it
  # is the one that surprises an upgrade rather than merely conflicting with it.
  vendor_fixture
  printf 'echo intruder\n' >"$VROOT/bin/.devbin/lib/delta"

  run_vendor
  assert_output_contains "UNLISTED"
  assert_output_contains "bin/.devbin/lib/delta"
  refute_output_contains "DIVERGED"
}

@test "cmd/ and help/ are project territory and are never reported" {
  # The manifest header declares them so. Reporting a project's own handlers as
  # vendored drift would make the report cry wolf on every commit -- and this very
  # command lives under cmd/, so it would report itself.
  vendor_fixture
  mkdir -p "$VROOT/bin/.devbin/cmd" "$VROOT/bin/.devbin/help"
  printf 'echo mine\n' >"$VROOT/bin/.devbin/cmd/mine"
  printf 'help text\n' >"$VROOT/bin/.devbin/help/mine.md"

  run_vendor
  assert_success
  refute_output_contains "UNLISTED"
  assert_output_contains "running stock devbin"
}

# --------------------------------------------------------------------
# The instrument control: zero matches is a broken RUN, not a broken TREE
# --------------------------------------------------------------------

@test "ZERO matches is reported as a broken run rather than a wholly diverged tree" {
  # THE LESSON THIS FILE WAS BUILT AROUND. A tree where every single vendored file
  # differs is not a patch set, it is a fork -- and the far likelier explanation is
  # that the measuring tool broke, which is exactly what happened by hand. A report
  # that said "27 DIVERGED" and stopped would have been believed.
  vendor_fixture
  for f in alpha beta cmd/gamma; do
    printf 'all changed\n' >>"$VROOT/bin/.devbin/lib/$f"
  done

  run_vendor
  assert_failure
  assert_output_contains "BROKEN RUN"
  assert_output_contains "0 file(s) MATCH"
  # And it must name the actual trap, not merely say "something is wrong".
  assert_output_contains "path"
}

# --------------------------------------------------------------------
# What it must never do
# --------------------------------------------------------------------

@test "it NEVER rewrites the manifest" {
  # Restamping would make this report clean while changing nothing about the tree,
  # erasing the only signal that the divergence exists. The manifest is evidence
  # here, not a lockfile to be refreshed.
  vendor_fixture
  printf 'echo alpha modified\n' >>"$VROOT/bin/.devbin/lib/alpha"
  local before
  before="$(shasum -a 256 "$VROOT/bin/.devbin/manifest.sha256" | cut -d' ' -f1)"

  run_vendor
  local after
  after="$(shasum -a 256 "$VROOT/bin/.devbin/manifest.sha256" | cut -d' ' -f1)"
  [ "$before" = "$after" ]

  run grep -cE 'manifest_write|shasum .* >.*manifest' "$VENDOR"
  assert_failure
}

@test "no loop variable is named path, which zsh ties to PATH" {
  # Not a style check. `while read -r want path` destroyed PATH on the first
  # iteration when this was run by hand, so shasum vanished and every file
  # reported as diverged. The test above catches the SYMPTOM; this catches the
  # cause returning.
  run grep -cE '^[[:space:]]*while read -r [a-z]+ path' "$VENDOR"
  assert_failure
}
