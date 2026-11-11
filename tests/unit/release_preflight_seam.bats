#!/usr/bin/env bats
# The cut's pre-flight seam: devbin's release core runs Intent's own read-only
# checks, `release.preflight`, at its pre-flight, a dry run's included, and a
# check that refuses stops the cut before anything is stamped (devbin 0327).
#
# ST0091 AT-01.6 cites this file for AC-01.3.
#
# Each arm drives the core, `bin/devbin release`, in a scratch repository
# holding this checkout's bin/devbin and bin/.devbin, so the list run is the one
# Intent declares, with a stub standing in for the release pair's CLI. The
# scratch repository has no remote, so the core's own pre-flight refuses too
# and the cut stops there whatever the checks say: no arm can commit, tag or
# push. The arms read what the seam ran and what it refused, beside the core's.

load "../lib/test_helper.bash"

# make_repo -- $REPO committed at 3.2.0 with a 3.3.0 changelog entry, Intent's
# devbin and its config, a schema face carrying 3.2.0 for `preflight faces`,
# and the build target ignored, as this repository ignores it.
make_repo() {
  REPO="${TEST_TEMP_DIR}/repo"
  mkdir -p "$REPO/bin" "$REPO/schema"
  cp "${INTENT_PROJECT_ROOT}/bin/devbin" "$REPO/bin/"
  cp -R "${INTENT_PROJECT_ROOT}/bin/.devbin" "$REPO/bin/"
  printf '3.2.0\n' >"$REPO/VERSION"
  printf '# Changelog\n\n## [3.3.0] - 2026-11-11\n\n- a change\n' >"$REPO/CHANGELOG.md"
  printf '{"x-intent-ver": "3.2.0"}\n' >"$REPO/schema/thread.schema.json"
  printf -- '-- INTENT_VER: 3.2.0\n' >"$REPO/schema/ddl.sql"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  git -C "$REPO" init -q
  git -C "$REPO" config user.name t
  git -C "$REPO" config user.email t@t
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m base
  write_release_cli
  HEAD_BEFORE="$(git -C "$REPO" rev-parse HEAD)"
}

# write_release_cli -- the release pair's intent, answering what the checks
# ask: `daemon status --format json`, whose state is STUB_DAEMON, and `claude
# upgrade`'s dry run, which would write nothing.
write_release_cli() {
  local bin="$REPO/native/rust/target/release/intent"
  mkdir -p "$(dirname "$bin")"
  cat >"$bin" <<'EOF'
#!/usr/bin/env bash
case "$1 ${2:-}" in
  "daemon status") printf '{"state":"%s"}\n' "${STUB_DAEMON:-stopped}" ;;
  "claude upgrade") ;;
  *) echo "stub intent: unexpected arguments: $*" >&2; exit 2 ;;
esac
EOF
  chmod +x "$bin"
}

# cut [<flag>] -- the core's cut of v3.3.0 in $REPO, stdin closed, as the seam
# asks nobody; STUB_DAEMON reaches the stub through the environment.
cut() {
  run bash -c 'cd "$1" && shift && bin/devbin release v3.3.0 "$@" </dev/null 2>&1' _ "$REPO" "$@"
}

# declared -- release.preflight's lines as Intent's config declares them, read
# through the core's own `release check`, one a line.
declared() {
  (cd "$REPO" && bin/devbin release check 2>/dev/null) |
    awk '$1 == "preflight" { $1 = ""; $NF = ""; print }' | tr ',' '\n' | sed 's/^ *//; s/ *$//' | grep .
}

# nothing_stamped -- VERSION and HEAD as they were, and no tag.
nothing_stamped() {
  [ "$(tr -d '[:space:]' <"$REPO/VERSION")" = 3.2.0 ] || fail "VERSION was stamped: $(cat "$REPO/VERSION")"
  [ "$(git -C "$REPO" rev-parse HEAD)" = "$HEAD_BEFORE" ] || fail "HEAD moved"
  [ -z "$(git -C "$REPO" tag -l v3.3.0)" ] || fail "v3.3.0 was tagged"
}

@test "AT-01.6: the dry run runs every check release.preflight declares through the core's seam" {
  make_repo
  local cmds cmd
  cmds="$(declared)"
  [ -n "$cmds" ] || fail "release check names no release.preflight"
  STUB_DAEMON=stopped cut --dry-run
  assert_failure
  while IFS= read -r cmd; do
    assert_output_contains "release: pre-flight: bin/devbin $cmd"
  done <<<"$cmds"
  assert_output_contains "command(s) of release.preflight passed, and none wrote to the tree"
  assert_output_contains "ok: no intentd is running"
  nothing_stamped
}

@test "AT-01.6: a live intentd refuses the dry run at pre-flight, beside the core's refusals, with the remedy that stops it" {
  make_repo
  STUB_DAEMON=live cut --dry-run
  assert_failure
  assert_output_contains "error: preflight daemon: intentd is running"
  assert_output_contains 'run `intent daemon stop`'
  assert_output_contains "the pre-flight command 'preflight daemon' failed"
  assert_output_contains "stopped at the pre-flight step"
  nothing_stamped
}

@test "AT-01.6: a live intentd refuses the cut itself at pre-flight, before the confirm and before anything is stamped" {
  make_repo
  STUB_DAEMON=live cut
  assert_failure
  assert_output_contains "error: preflight daemon: intentd is running"
  assert_output_contains "the pre-flight command 'preflight daemon' failed"
  assert_output_contains "stopped at the pre-flight step"
  [[ "$output" != *"DRY RUN"* ]] || fail "the cut ran as a dry run: $output"
  nothing_stamped
}
