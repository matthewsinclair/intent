#!/usr/bin/env bats
# `int prepush`, the push gates: a frozen remote, and the delivered pair's
# currency (issue 0518), asked of every push but a candidate's.
#
# ST0091 AT-02.2 cites this file for AC-02.2.
#
# WHAT LEFT (ST0091 WP-02). This file used to hold the range arms of the
# clone-and-build check and the arms of the Linux preflight. devbin's release
# core now runs Intent's gates on the release commit and on kovacs as legs of
# the cut, so the hook clones nothing, builds nothing and asks for no kovacs
# summary. The arms below hold what the hook still does, and that it does
# nothing more.
#
# THE PAIR IS STAMPED, NOT BUILT. The verdict reads each binary's
# `[intent-source-commit:<sha>]` marker through `strings`, which reads a plain
# file as readily as a binary, so two small files carrying the marker are a pair
# to it. They sit where the real pair does, and `setup` ignores
# `native/rust/target/` as this repository does. Every assertion carries the
# runner's own output (issue 0521).
#
# THE CANDIDATE PREFIX IS DECLARED IN THE FIXTURE'S CONFIG AS `rc/{tag}`, not the
# core's default `candidate/{tag}`, so an arm that passes a candidate push proves
# the prefix is read from config and not written into the hook.

load "../lib/test_helper.bash"

RUNNER="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/prepush"
ZERO="0000000000000000000000000000000000000000"

# THE TRIMMED PATH CARRIES ONE THING MORE THAN /usr/bin:/bin -- A BASH AT THE
# FLOOR. devbin refuses at load on any bash below 5 (helpers, issue 0076), and
# macOS ships /bin/bash 3.2, so a private directory holding one link, `bash`,
# joins the PATH, and `setup` asserts the floor.
stamp_pair() {
  local dir="$REPO/native/rust/target/release" b
  mkdir -p "$dir"
  for b in intent intentd; do
    printf '[intent-source-commit:%s]\n' "$1" >"$dir/$b"
  done
}

# runner <stdin> [<remote>] -- the runner as git's pre-push hook runs it: the
# ref pairs on stdin and the destination as argv. "" for stdin is the by-hand
# path. RUNNER_FILE, when set, is the runner to run.
runner() {
  local stdin_content="$1" remote="${2:-}"
  local -a argv=()
  [ -z "$remote" ] || argv=("$remote" "$(git -C "$REPO" config --get "remote.$remote.url")")
  printf '%s\n' "$stdin_content" | {
    if [ -z "$stdin_content" ]; then exec </dev/null; fi
    env -i HOME="$HOME" PATH="$TRIMMED_PATH" DEVBIN_NAME=int \
      DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
      PROJECT_ROOT="$REPO" ${ALLOW_FROZEN:+INTENT_ALLOW_FROZEN_PUSH=1} \
      bash "${RUNNER_FILE:-$RUNNER}" "${argv[@]}" 2>&1
  }
}

refpair() { printf 'refs/heads/main %s refs/heads/main %s' "$1" "$2"; }
at() { git -C "$REPO" rev-parse "$1"; }

expect_passed() {
  [ "$status" -eq 0 ] || fail "expected the push gates to pass, got status $status: $output"
  [[ "$output" == *"prepush: the push gates pass"* ]] ||
    fail "expected 'the push gates pass'; got status $status: $output"
}

expect_stale() {
  [ "$status" -ne 0 ] || fail "expected the stale pair to be REFUSED, got status 0: $output"
  [[ "$output" == *"BLOCKED: the delivered pair"* ]] ||
    fail "expected the currency refusal; got status $status: $output"
}

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/intent-prepush-XXXXXX")" && pwd)"
  REPO="$TEST_TEMP_DIR/repo"
  mkdir "$TEST_TEMP_DIR/shim"
  ln -s "$(command -v bash)" "$TEST_TEMP_DIR/shim/bash"
  TRIMMED_PATH="$TEST_TEMP_DIR/shim:/usr/bin:/bin"

  PATH="$TRIMMED_PATH" command -v git >/dev/null 2>&1 ||
    skip_other_system "git is not on the trimmed PATH -- this fixture cannot run"
  env -i PATH="$TRIMMED_PATH" bash -c '[ "${BASH_VERSINFO[0]}" -ge 5 ]' 2>/dev/null ||
    skip_other_system "the bash on the trimmed PATH is below devbin's floor of 5 -- the runner would refuse at load"

  # Two remotes, named as this project names them. The branch is named
  # explicitly on all three: bare `git init` takes it from the user's own
  # `init.defaultBranch`, which the CI runners do not set.
  git init -q -b main --bare "$TEST_TEMP_DIR/local.git"
  git init -q -b main --bare "$TEST_TEMP_DIR/upstream.git"
  git init -q -b main "$REPO"
  git -C "$REPO" config user.email t@t
  git -C "$REPO" config user.name t
  mkdir -p "$REPO/native/rust" "$REPO/intent/whiteboard" "$REPO/bin/.devbin"
  printf 'base\n' >"$REPO/README.md"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  printf 'release:\n  publish:\n    candidate: rc/{tag}\n' >"$REPO/bin/.devbin/config.yaml"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm base
  git -C "$REPO" remote add local "$TEST_TEMP_DIR/local.git"
  git -C "$REPO" remote add upstream "$TEST_TEMP_DIR/upstream.git"
  git -C "$REPO" push -q local main
  git -C "$REPO" push -q upstream main
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
  return 0
}

# native_change -- a commit under native/, so a pair stamped before it is
# behind HEAD.
native_change() {
  printf '[workspace]\n' >"$REPO/native/rust/Cargo.toml"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "native change"
}

@test "AT-02.2: a push over a current pair passes, and neither clones, builds nor asks for a Linux preflight" {
  native_change
  stamp_pair "$(at HEAD)"
  run runner "$(refpair "$(at HEAD)" "$(at upstream/main)")" upstream
  expect_passed
  [[ "$output" != *"clon"* ]] || fail "the hook still speaks of a clone: $output"
  [[ "$output" != *"preflight"* ]] || fail "the hook still asks for a Linux preflight: $output"
}

@test "AT-02.2: a stale pair is refused on any push, a board-only one included, and by hand" {
  # 0518's own property: the push's contents are not the question, whether the
  # pair describes what is being published is.
  local before
  before="$(at HEAD)"
  native_change
  printf 'note\n' >"$REPO/intent/whiteboard/note.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "board only"
  stamp_pair "$before"
  run runner "$(refpair "$(at HEAD)" "$(at HEAD~1)")" local
  expect_stale
  [[ "$output" == *"behind HEAD"* ]] || fail "expected the pair named as BEHIND; got: $output"
  [[ "$output" == *"remedy: bin/devbin build all"* ]] || fail "expected the rebuild remedy; got: $output"
  run runner "" ""
  expect_stale
}

# vc's ruling on dc's finding F2: a candidate push is not the release push.
@test "AT-02.2: a push of candidate refs alone passes over a stale pair, and main or a mixed push does not" {
  local before
  before="$(at HEAD)"
  native_change
  stamp_pair "$before"
  run runner "refs/heads/main $(at HEAD) refs/heads/rc/v3.2.1 $ZERO" upstream
  [ "$status" -eq 0 ] || fail "expected the candidate push to pass over a stale pair, got status $status: $output"
  [[ "$output" == *"only candidate refs"* ]] || fail "expected the candidate named; got: $output"
  run runner "$(refpair "$(at HEAD)" "$(at upstream/main)")" upstream
  expect_stale
  run runner "refs/heads/main $(at HEAD) refs/heads/rc/v3.2.1 $ZERO
refs/heads/main $(at HEAD) refs/heads/main $(at upstream/main)" upstream
  expect_stale
  # The core's default prefix is not the declared one, so it is not exempt.
  run runner "refs/heads/main $(at HEAD) refs/heads/candidate/v3.2.1 $ZERO" upstream
  expect_stale
}

@test "AT-02.2: a frozen remote refuses the push, a candidate's included, and the escape lets one through" {
  native_change
  stamp_pair "$(at HEAD)"
  # FROZEN_REMOTES is the hook's one interface for closing a remote: the list
  # in the runner. A copy of the runner with upstream on it is a frozen estate.
  # The copy sits in a tree laid out as this one is, its libraries and the
  # source_commit.rs they read the build-input scope from linked to the real ones.
  local frozen="$TEST_TEMP_DIR/frozen"
  mkdir -p "$frozen/bin/.devbin/cmd" "$frozen/native/rust"
  ln -s "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared" "$frozen/bin/.devbin/cmd/shared"
  ln -s "${INTENT_PROJECT_ROOT}/native/rust/build-support" "$frozen/native/rust/build-support"
  RUNNER_FILE="$frozen/bin/.devbin/cmd/prepush"
  sed 's/^FROZEN_REMOTES=""$/FROZEN_REMOTES="upstream"/' "$RUNNER" >"$RUNNER_FILE"
  grep -qx 'FROZEN_REMOTES="upstream"' "$RUNNER_FILE" || fail "the runner declares no empty FROZEN_REMOTES to set"
  run runner "refs/heads/main $(at HEAD) refs/heads/rc/v3.2.1 $ZERO" upstream
  [ "$status" -ne 0 ] || fail "expected the frozen remote to refuse the candidate push, got status 0: $output"
  [[ "$output" == *"BLOCKED: 'upstream' is a FROZEN remote"* ]] || fail "expected the frozen refusal; got: $output"
  run runner "$(refpair "$(at HEAD)" "$(at local/main)")" local
  expect_passed
  ALLOW_FROZEN=1 run runner "$(refpair "$(at HEAD)" "$(at upstream/main)")" upstream
  expect_passed
}

# A RELEASE PUSH, THE WAY THE CUT MAKES ONE (issue 0546).
#
# ST0091 AT-01.4 cites this file for AC-01.4.
#
# The release commit bumps the workspace version in native/rust/Cargo.toml,
# which is a build input, and the pair on disk was built before that commit. So
# the release's own push is refused by the verdict above unless something builds
# the pair at the release commit first, which is what `pair_at_head` does: the
# retired driver's `pair_at_tag`, moved into releasebuild.lib when devbin's core
# took the cut (ST0091). The arms below run THAT function, sourced as its
# callers source it, in a repository laid out as this one is: the currency
# libraries and releasebuild.lib at their real path, `source_commit.rs` beside
# the workspace so the libraries read the real build-input scope, and a
# `bin/devbin` that stands in for the dispatcher. `build all` in the stand-in
# stamps the pair at HEAD, which is what a promoted build names;
# DEVBIN_STUB_FAIL makes it fail, and DEVBIN_STUB_NOPROMOTE makes it exit 0
# without touching the pair, as the real one does on a dirty tree. The tag is
# annotated, as the core makes it (`git tag -a`), so the hook is handed the tag
# object's sha, as it is at a real cut.
cut_release() {
  local shared="$REPO/bin/.devbin/cmd/shared" lib
  mkdir -p "$shared" "$REPO/native/rust/build-support" "$REPO/bin"
  for lib in artefact sharedtarget currency releasebuild; do
    cp "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/$lib.lib" "$shared/"
  done
  cp "${INTENT_PROJECT_ROOT}/native/rust/build-support/source_commit.rs" "$REPO/native/rust/build-support/"
  printf '[workspace.package]\nversion = "3.2.0"\n' >"$REPO/native/rust/Cargo.toml"
  cat >"$REPO/bin/devbin" <<'STUB'
#!/usr/bin/env bash
[ "$1 $2" = "build all" ] || { echo "stub devbin: unexpected arguments: $*" >&2; exit 2; }
[ -z "${DEVBIN_STUB_FAIL:-}" ] || { echo "stub devbin: the build failed" >&2; exit 1; }
[ -z "${DEVBIN_STUB_NOPROMOTE:-}" ] || { echo "stub devbin: built into target/private/release" >&2; exit 0; }
sha="$(git rev-parse HEAD)"
for b in intent intentd; do
  printf '[intent-source-commit:%s]\n' "$sha" >"native/rust/target/release/$b"
done
STUB
  chmod +x "$REPO/bin/devbin"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "the workspace and the release tooling"
  git -C "$REPO" push -q local main
  git -C "$REPO" push -q upstream main
  stamp_pair "$(at HEAD)"

  # The release commit and its tag, as the core makes them: the commit is
  # "release: <tag>", and the tag is annotated.
  printf '[workspace.package]\nversion = "3.2.1"\n' >"$REPO/native/rust/Cargo.toml"
  git -C "$REPO" commit -qam "release: v3.2.1"
  git -C "$REPO" tag -a v3.2.1 -m v3.2.1
  [ "$(git -C "$REPO" cat-file -t v3.2.1)" = tag ] || fail "v3.2.1 is not an annotated tag, as the core makes one"
}

# The two ref pairs git hands the hook for `git push local main v3.2.1`. For an
# annotated tag the local sha is the tag object's, as git hands it.
release_refs() {
  printf 'refs/heads/main %s refs/heads/main %s\nrefs/tags/v3.2.1 %s refs/tags/v3.2.1 %s' \
    "$(at HEAD)" "$(at local/main)" "$(at v3.2.1)" "$ZERO"
}

# Run pair_at_head as its callers do, the libraries it requires sourced first,
# with only the stand-in's switches added to a clean environment. A library
# that does not define it fails the arm rather than passing it.
pair_at_head_run() {
  env -i HOME="$HOME" PATH="$TRIMMED_PATH" PROJECT_ROOT="$REPO" "$@" \
    bash -c 'for lib in artefact sharedtarget currency releasebuild; do . "$1/$lib.lib" || exit 2; done
      declare -F pair_at_head >/dev/null || { echo "releasebuild.lib defines no pair_at_head"; exit 2; }
      pair_at_head' _ "$REPO/bin/.devbin/cmd/shared" 2>&1
}

@test "AT-01.4: a release push passes the pre-push check once the cut has built the pair at the release commit" {
  cut_release

  # The defect, as vc measured it: over the pair built before the release
  # commit, the release's own push is refused.
  run runner "$(release_refs)"
  [ "$status" -ne 0 ] || fail "expected the release push to be REFUSED over the pre-release pair, got status 0: $output"
  [[ "$output" == *"BLOCKED: the delivered pair"* ]] ||
    fail "expected the currency refusal; got status $status: $output"

  run pair_at_head_run
  [ "$status" -eq 0 ] || fail "expected pair_at_head to pass the pair it built, got status $status: $output"
  [ "$output" = "ok" ] || fail "expected the verdict 'ok'; got: $output"

  # The same push over the pair built at the release commit passes.
  run runner "$(release_refs)"
  expect_passed
}

@test "AT-01.4: a pair build that fails refuses before the push, naming the build" {
  cut_release
  run pair_at_head_run DEVBIN_STUB_FAIL=1
  [ "$status" -eq 1 ] || fail "expected pair_at_head to refuse a failed build, got status $status: $output"
  [[ "$output" == *"refuse:bin/devbin build all failed"* ]] ||
    fail "expected the refusal to name the build; got: $output"
}

@test "AT-01.4: a pair build that promotes nothing refuses before the push, with the hook's own verdict" {
  # `build all` exits 0 into target/private/release on a dirty tree, so its exit
  # code is not the answer; the verdict the hook takes is.
  cut_release
  run pair_at_head_run DEVBIN_STUB_NOPROMOTE=1
  [ "$status" -eq 1 ] || fail "expected pair_at_head to refuse an unpromoted pair, got status $status: $output"
  [[ "$output" == *"refuse:"*"behind HEAD"* ]] ||
    fail "expected the currency verdict naming the pair as behind; got: $output"
}


# ST0091 AT-02.1 cites this file for AC-02.1: the cut's leg on kovacs is
# declared, and the Linux preflight it replaces is gone.
@test "AT-02.1: release check names kovacs as the cut's leg, and the Linux preflight's two files are gone" {
  local row
  row="$(cd "$INTENT_PROJECT_ROOT" && bin/devbin release check 2>/dev/null | awk '$1 == "legs"')"
  [[ "$row" == *kovacs* ]] || fail "release check's legs row does not name kovacs: $row"
  for f in bin/.devbin/preflight-linux.sh bin/.devbin/cmd/shared/preflight.lib; do
    [ ! -e "${INTENT_PROJECT_ROOT}/$f" ] || fail "$f is back, and the Linux run is the cut's leg"
  done
}
