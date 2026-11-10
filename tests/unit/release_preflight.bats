#!/usr/bin/env bats
# The cut's read-only checks before anything is stamped, bin/.devbin/cut/preflight.
#
# ST0091 AT-01.3 cites this file for AC-01.3.
#
# devbin's release core will run these at its pre-flight, a dry run included,
# once the devbin release carrying the seam (devbin 0327) is swept in; until
# then they run by hand, and these arms drive each check as the seam will: in a
# scratch repository with PROJECT_ROOT pointed at it, a stub standing in for the
# release pair's CLI where the check asks it something. Each check writes
# nothing, refuses on stderr with the remedy, and passes with one `ok:` line.

load "../lib/test_helper.bash"

PREFLIGHT="${INTENT_PROJECT_ROOT}/bin/.devbin/cut/preflight"

# make_repo -- $REPO committed at 3.2.0, with a schema face carrying it, a
# sidecar and a tracked file that is not the release's, and the build target
# ignored, as this repository ignores it.
make_repo() {
  REPO="${TEST_TEMP_DIR}/repo"
  mkdir -p "$REPO/schema"
  git -C "$REPO" init -q
  git -C "$REPO" config user.name t
  git -C "$REPO" config user.email t@t
  printf '3.2.0\n' >"$REPO/VERSION"
  printf 'claude\n' >"$REPO/CLAUDE.md"
  printf 'usage rules\n' >"$REPO/usage-rules.md"
  printf '{"x-intent-ver": "3.2.0"}\n' >"$REPO/schema/thread.schema.json"
  printf -- '-- INTENT_VER: 3.2.0\n' >"$REPO/schema/ddl.sql"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m base
}

# write_release_cli -- the release pair's intent, answering the two things the
# checks ask: `daemon status --format json`, whose state is STUB_DAEMON (and
# whose exit is STUB_DAEMON_RC), and `claude upgrade`'s dry run, which names
# each of STUB_WOULD_WRITE's paths.
write_release_cli() {
  local bin="$REPO/native/rust/target/release/intent"
  mkdir -p "$(dirname "$bin")"
  cat >"$bin" <<'EOF'
#!/usr/bin/env bash
case "$1 ${2:-}" in
  "daemon status")
    printf '{"state":"%s"}\n' "${STUB_DAEMON:-stopped}"
    exit "${STUB_DAEMON_RC:-0}" ;;
  "claude upgrade")
    for p in ${STUB_WOULD_WRITE:-}; do printf 'would write: %s\n' "$p"; done ;;
  *) echo "stub intent: unexpected arguments: $*" >&2; exit 2 ;;
esac
EOF
  chmod +x "$bin"
}

# check <name> -- one check as the seam runs it. $output is its stdout alone;
# its stderr is in $ERR.
check() {
  ERR="${TEST_TEMP_DIR}/check.err"
  run env PROJECT_ROOT="$REPO" bash -c '"$1" "$2" 2>"$3"' _ "$PREFLIGHT" "$1" "$ERR"
}

err_has() {
  grep -qF -- "$1" "$ERR" || fail "expected the check's stderr to name '$1'; it was: $(cat "$ERR")"
}

@test "AT-01.3: clean: while no stamp has begun, a dirty and an untracked path are refused by name" {
  make_repo
  printf 'edited\n' >>"$REPO/usage-rules.md"
  printf 'new\n' >"$REPO/stray.txt"
  before="$(git -C "$REPO" status --porcelain)"
  check clean
  assert_failure
  err_has "usage-rules.md"
  err_has "stray.txt"
  [ "$(git -C "$REPO" status --porcelain)" = "$before" ] || fail "the check changed the tree it judged"
}

# vc decision 115: a cut stopped inside the stamp is continued by the same cut
# run again, so the stamp's own dirt must not refuse it.
@test "AT-01.3: clean: once VERSION differs from HEAD's, a stamp has begun and the check stands down for the core's resume" {
  make_repo
  printf '3.3.0\n' >"$REPO/VERSION"
  printf 'stamped\n' >>"$REPO/CLAUDE.md"
  check clean
  assert_success
  assert_output_contains "a stamp has begun"
}

@test "AT-01.3: clean: a clean tree passes" {
  make_repo
  check clean
  assert_success
  assert_output "ok: the tree is clean at 3.2.0"
}

# AC-01.3: a live intentd refuses before anything is stamped, and the remedy
# says how to stop it.
@test "AT-01.3: daemon: a live intentd is refused with the remedy that stops it, and a stopped one passes" {
  make_repo
  write_release_cli
  export STUB_DAEMON=live
  check daemon
  assert_failure
  err_has "intentd is running"
  err_has "intent daemon stop"
  export STUB_DAEMON=stopped
  check daemon
  assert_success
  assert_output "ok: no intentd is running"
}

@test "AT-01.3: daemon: a daemon status that fails is a refusal, never a pass" {
  make_repo
  write_release_cli
  export STUB_DAEMON_RC=1
  check daemon
  assert_failure
  err_has "intent daemon status failed"
}

@test "AT-01.3: daemon: no release pair is a refusal with the build that makes one" {
  make_repo
  check daemon
  assert_failure
  err_has "bin/devbin build all"
}

# The faces are read as HEAD has them, so the answer does not depend on how far
# a stamp has got.
@test "AT-01.3: faces: a face at HEAD that does not carry HEAD's version is refused by name, whatever the tree holds" {
  make_repo
  printf -- '-- INTENT_VER: 3.1.0\n' >"$REPO/schema/ddl.sql"
  git -C "$REPO" commit -q -am "a stale face"
  printf -- '-- INTENT_VER: 3.2.0\n' >"$REPO/schema/ddl.sql"
  check faces
  assert_failure
  err_has "schema/ddl.sql carries 3.1.0"
  git -C "$REPO" commit -q -am "the face regenerated"
  printf '3.3.0\n' >"$REPO/VERSION"
  check faces
  assert_success
  assert_output "ok: the schema faces carry 3.2.0"
}

@test "AT-01.3: spill: a tracked file the canon engine would rewrite outside the release's files is refused by name, and a sidecar or an untracked path is not" {
  make_repo
  write_release_cli
  export STUB_WOULD_WRITE="CLAUDE.md .mcp.json usage-rules.md"
  check spill
  assert_failure
  err_has "usage-rules.md"
  run grep -qF "CLAUDE.md" "$ERR"
  assert_failure
  run grep -qF ".mcp.json" "$ERR"
  assert_failure
  export STUB_WOULD_WRITE="CLAUDE.md .mcp.json"
  check spill
  assert_success
}

# The checks are wired as declared options, so the real dispatcher reaches one.
# `faces` asks this tree nothing it can change and needs no daemon or pair.
@test "AT-01.3: bin/devbin preflight faces runs the declared check against this tree" {
  run bash -c 'cd "$1" && bin/devbin preflight faces 2>&1' _ "$INTENT_PROJECT_ROOT"
  assert_success
  assert_output_contains "ok: the schema faces carry"
}
