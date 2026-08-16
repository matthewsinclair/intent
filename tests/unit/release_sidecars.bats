#!/usr/bin/env bats
# Guards for intent build release's sidecar contract.
#
# intent build release is maintainer-only tooling and cannot be exercised end to end in a
# unit test -- it tags, pushes to two remotes and publishes a GitHub release. So
# these are mechanical guards over the script's structure, which is exactly where
# the two defects they cover lived:
#
#   1. Every release tag was internally inconsistent. The script stamped VERSION
#      but not intent/.config/config.json or CLAUDE.md, and the wrap that fixed
#      them was a manual commit AFTER the tag -- so checking out v2.17.3 gave you
#      VERSION=2.17.3 alongside intent_version=2.17.2. Three releases shipped
#      that way before anyone wrote it down as a watch-out rather than a bug.
#
#   2. The detect step and the stage step read different lists. NEEDS_COMMIT was
#      set from a bare `git status --porcelain` (any dirty file at all) while the
#      commit staged only VERSION/CHANGELOG.md/AGENTS.md, so the script could
#      announce a commit it had not made.

load "../lib/test_helper.bash"

# INTENT_RELEASE_SCRIPT redirects every test in this file at another copy of the
# script. It exists so the guards below can be mutation-tested -- pointed at a
# deliberately broken copy to confirm they go red -- without editing the shipped
# one. Unset, which is every normal run, this is byte-identical to the plain path.
RELEASE="${INTENT_RELEASE_SCRIPT:-${INTENT_HOME}/bin/.devbin/cmd/build.d/release}"

@test "intent build release is syntactically valid" {
  run bash -n "$RELEASE"
  assert_success
}

@test "the sidecar list is declared once and covers every file a release stamps" {
  run grep -E '^SIDECAR_FILES=' "$RELEASE"
  assert_success
  assert_output_contains "VERSION"
  assert_output_contains "CHANGELOG.md"
  assert_output_contains "AGENTS.md"
  assert_output_contains "CLAUDE.md"
  assert_output_contains "CONFIG_REL"

  # Declared exactly once -- a second declaration is how the two lists drifted.
  run bash -c "grep -cE '^SIDECAR_FILES=' '$RELEASE'"
  assert_output "1"
}

@test "the commit stages the sidecar list rather than a hardcoded set" {
  # The staging line must read the list, not re-spell it.
  run grep -E 'git add -- \$SIDECAR_FILES' "$RELEASE"
  assert_success

  # The pre-fix hardcoded form must not survive anywhere.
  run grep -F 'git add VERSION CHANGELOG.md AGENTS.md' "$RELEASE"
  assert_failure
}

@test "the detect step is scoped to the same list the commit stages" {
  # The NEEDS_COMMIT probe must carry a pathspec. An unscoped
  # `git status --porcelain` here is the exact shape of defect 2 above.
  run grep -E 'git status --porcelain -- \$SIDECAR_FILES' "$RELEASE"
  assert_success
}

@test "config.json and CLAUDE.md are stamped before the commit, not after the tag" {
  local stamp_line claude_line commit_line tag_line
  stamp_line="$(grep -n 'stamp_project_version' "$RELEASE" | head -1 | cut -d: -f1)"
  claude_line="$(grep -n 'claude upgrade --apply' "$RELEASE" | head -1 | cut -d: -f1)"
  commit_line="$(grep -n 'git commit -m "release: v\$TARGET"' "$RELEASE" | head -1 | cut -d: -f1)"
  tag_line="$(grep -n 'log_step "tag"' "$RELEASE" | head -1 | cut -d: -f1)"

  [ -n "$stamp_line" ] || fail "intent build release never stamps intent_version"
  [ -n "$claude_line" ] || fail "intent build release never refreshes CLAUDE.md"

  # Ordering is the whole fix: a stamp after the tag is the manual wrap this
  # replaced, and it is what made every published tag self-inconsistent.
  [ "$stamp_line" -lt "$commit_line" ] || fail "intent_version is stamped after the release commit"
  [ "$claude_line" -lt "$commit_line" ] || fail "CLAUDE.md is refreshed after the release commit"
  [ "$commit_line" -lt "$tag_line" ] || fail "the release commit lands after the tag"
}

@test "intent build release delegates CLAUDE.md rather than rendering the template itself" {
  # The placeholder substitution already has three homes (intent_init,
  # intent_st_zero, intent_claude_upgrade). A fourth here would be a Highlander
  # violation, and the canon engine is CLAUDE.md's owner.
  # No placeholder rendering...
  run grep -F '[[INTENT_VERSION]]' "$RELEASE"
  assert_failure
  # ...and no direct edit of the generated file either. (Naming the template in
  # a comment is fine and is why this greps for the edit, not the mention.)
  run grep -E '(sed|perl|awk)[^|]*CLAUDE\.md' "$RELEASE"
  assert_failure
  # The refresh goes through the engine that owns the file.
  run grep -F 'claude upgrade --apply' "$RELEASE"
  assert_success
}

@test "intent build release refuses to tag a tree left dirty by the sidecar sync" {
  run grep -F 'refusing to tag a dirty tree' "$RELEASE"
  assert_success
}

@test "intent build release documents the in-progress CHANGELOG convention" {
  # A hand-typed date is correct only on the day it is typed; it goes stale at
  # midnight and aborts the pre-flight date gate. This bit v2.17.4.
  run grep -F 'in progress' "$RELEASE"
  assert_success
  run bash -c "sed -n '1,40p' '$RELEASE' | grep -F 'NOT with a hand-typed date'"
  assert_success
}

@test "intent_upgrade routes its stamp through the shared helper" {
  # The stamper existed only in intent_upgrade; intent build release growing a second
  # copy is what the shared helper prevents.
  run grep -F 'stamp_project_version' "${INTENT_HOME}/bin/intent_upgrade"
  assert_success
  run grep -F "jq --arg v \"\$TARGET_VERSION\" '.intent_version" "${INTENT_HOME}/bin/intent_upgrade"
  assert_failure
}

@test "intent build release pins INTENT_HOME to the checkout being released" {
  # bin/intent only derives INTENT_HOME when unset, so an exported INTENT_HOME
  # from the maintainer's shell silently wins and every sub-command reads THAT
  # tree's VERSION -- stamping the wrong version into AGENTS.md and CLAUDE.md
  # while VERSION and config.json carry the right one. Found by cutting a test
  # release from a clone with INTENT_HOME left pointing elsewhere.
  run grep -E '^export INTENT_HOME="\$PROJECT_ROOT"' "$RELEASE"
  assert_success
}

@test "the CLAUDE.md refresh skips the .claude/ stack" {
  # Without --skip-settings the canon engine also rewrites .claude/settings.json
  # and the hook scripts, which substitute [[INTENT_HOME]] and so differ between
  # checkouts. They are unrelated to the release and outside the sidecar list, so
  # they trip the dirty-tree check and abort the cut.
  run grep -E 'claude upgrade --apply --skip-settings' "$RELEASE"
  assert_success
}

# --------------------------------------------------------------------
# The native Cargo.lock check (ST0056 / WP-11).
# --------------------------------------------------------------------
#
# Unlike every guard above, these RUN the shipped functions against real
# lockfile fixtures. The guards above are structural because the release script
# cannot be exercised end to end -- it tags, pushes and publishes. That is true
# of the script and false of these three functions, which are pure over two
# files, and the difference matters: the check below had no test at all, and it
# reached the morning of a v3 cut accepting the one change it exists to refuse.
#
# The defect, for whoever reads this next: it asserted "every changed line is a
# version line". A dependency bump is a lone version line, identical in shape to
# a workspace member's, so it sailed through. The comment said what the code did
# and both were wrong about the invariant -- so diffing prose against mechanism
# proved nothing. Only running the refusal against a case it should refuse did.

# Source the three lock functions out of the script under test. Extracted rather
# than copied: a test carrying its own copy of the code under test passes
# forever regardless of what ships.
#
# Deliberately NOT a setup() -- test_helper.bash defines one, and a definition
# here would override it for every test in this file. They would all still pass,
# so nothing would signal that TEST_TEMP_DIR had stopped being created.
load_lock_fns() {
  local lockfns="${TEST_TEMP_DIR}/lockfns.sh"
  sed -n '/^lock_packages() {/,/^}/p;/^lock_members() {/,/^}/p;/^lock_nonmember_changes() {/,/^}/p' \
    "$RELEASE" > "$lockfns"
  # shellcheck disable=SC1090
  . "$lockfns"
}

# A minimal but structurally faithful lockfile: two path packages (no `source`)
# and one registry package (with `source`), which is exactly the distinction
# lock_members reads.
write_lock() {  # write_lock PATH MEMBER_VERSION DEP_VERSION [EXTRA]
  cat > "$1" <<EOF
version = 4

[[package]]
name = "intent-cli"
version = "$2"
dependencies = [
 "serde",
]

[[package]]
name = "intentsvcs"
version = "$2"

[[package]]
name = "serde"
version = "$3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "abc123"
EOF
  if [ -n "${4:-}" ]; then printf '%s\n' "$4" >> "$1"; fi
  return 0
}

@test "all three lock functions are present in the shipped script" {
  run grep -c '^lock_packages() {\|^lock_members() {\|^lock_nonmember_changes() {' "$RELEASE"
  assert_success
  assert_output "3"
}

@test "lock_packages reads a name and version out of every package block" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/a.lock" "3.0.0" "1.0.229"
  run lock_packages "${TEST_TEMP_DIR}/a.lock"
  assert_success
  assert_output "$(printf 'intent-cli\t3.0.0\nintentsvcs\t3.0.0\nserde\t1.0.229')"
}

@test "lock_members is exactly the packages carrying no source line" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/a.lock" "3.0.0" "1.0.229"
  run lock_members "${TEST_TEMP_DIR}/a.lock"
  assert_success
  assert_output "$(printf 'intent-cli\nintentsvcs')"
}

@test "a member-only version bump is accepted" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock"  "3.0.0"     "1.0.229"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output ""
}

# THE REGRESSION. This is the case the old check accepted, and it is the whole
# reason this block exists: a lone dependency version bump is identical in shape
# to a workspace member's.
@test "a dependency version bump riding alongside a member bump is REFUSED by name" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock"  "3.0.0"     "9.9.9"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "serde"
}

@test "a dependency bump with no member change at all is REFUSED by name" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock"  "3.0.0-dev" "9.9.9"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "serde"
}

@test "a newly resolved package is REFUSED by name" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock"  "3.0.0"     "1.0.229" \
'
[[package]]
name = "brand-new-crate"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"'
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "brand-new-crate"
}

# An empty parse yields an empty difference, which reads identically to "nothing
# changed". Without this guard the check would accept anything, confidently.
@test "an unreadable lockfile fails rather than reporting no offenders" {
  load_lock_fns
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  : > "${TEST_TEMP_DIR}/empty.lock"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/empty.lock"
  assert_failure
  assert_output ""
}

@test "the refusal is wired to lock_nonmember_changes, not compared inline" {
  # Anchored to the start of a line so a comment mentioning the function cannot
  # satisfy it -- an untestable inline pipeline is how the original defect
  # survived, and a check prose can satisfy is not a check.
  run grep -cE '^[[:space:]]*if ! unexpected="\$\(lock_nonmember_changes ' "$RELEASE"
  assert_success
  assert_output "1"

  # ...and the comparison itself has exactly ONE home. "The pre-fix form must not
  # survive" is the wrong predicate here and this test caught that on its first
  # run: the fix was an EXTRACTION, not a deletion, so the pipeline legitimately
  # still exists -- inside the function. What must not exist is a SECOND copy.
  #
  # The pair is what distinguishes the three states a grep otherwise cannot.
  # Re-inlined with the function gone: the assertion above drops to 0. Function
  # kept and a copy pasted back at the call site: this count goes to 2.
  run grep -c 'comm -3 "$pkgs_before"' "$RELEASE"
  assert_success
  assert_output "1"
}
