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

RELEASE="${INTENT_HOME}/bin/.devbin/cmd/build.d/release"

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
