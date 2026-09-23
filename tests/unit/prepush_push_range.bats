#!/usr/bin/env bats
# `int prepush` decides from THIS push's range, not from the tracked remote.
#
# THE DEFECT, measured by cc 2026-08-15. The gate printed `no native/ or
# build-manifest change in this push` on a push whose diff carried files under
# `native/` and `schema/`. The grep was right; the RANGE was wrong. It computed
# `git diff --name-only @{upstream}...HEAD`, and `@{upstream}` names exactly ONE
# remote -- while the hook fires for a push to EITHER, and this project's
# standing instruction is to push both.
#
# So after `git push upstream main` succeeds, `upstream/main` IS HEAD, and the
# `git push local main` that follows carries every commit while computing an
# EMPTY range. **The second push of the standard two-push sequence was ungated
# by construction**, and it failed in the direction nobody watches: quietly,
# exiting 0, with a confident message that said a true thing about the wrong
# question.
#
# HOW THE ASSERTION AVOIDS THE 16s BUILD. The gate's expensive half clones HEAD
# and cold-builds it. What is under test here is the DECISION, not the build, so
# every case runs with `cargo` off PATH. That splits the two outcomes cleanly and
# instantly:
#
#   skipped -> exit 0, prints `no native/ or build-manifest change`
#   engaged -> exit non-zero, dies `cargo not on PATH`
#
# `setup` asserts that lever actually took. A fixture whose lever silently fails
# reports "nothing failed", which is indistinguishable from a test that does not
# check -- so if cargo is reachable on the trimmed PATH these SKIP loudly rather
# than quietly cloning and passing for the wrong reason.
#
# MUTATION-PROVEN, and the run CORRECTED this file rather than confirming it.
# The mutation is the runner as it stood at HEAD, in a sacrificial worktree,
# against this suite. Some tests discriminate; the rest pass under the defect:
#
#   1 gated at the remote that is behind      FAILS -- the empty range
#   2 answer follows each remote's position   FAILS -- only after the rewrite below
#   5 new ref on the remote                   FAILS -- stdin ignored entirely
#   7 by hand, furthest-behind remote         FAILS -- the by-hand path had it too
#   3, 4, 6, 8                                pass: correct for reasons the defect
#                                             happens to share
#
# **Test 2 was first written as cc phrased the ask -- "a push carrying native/ is
# gated regardless of which remote it goes to" -- and PASSED ON THE BROKEN
# RUNNER.** With both remotes behind, the old range is non-empty and the gate
# engages for the wrong reason; and since the defect never reads stdin, "the same
# answer whichever remote" is a property the BUG satisfies perfectly. It is now
# one HEAD against two remotes in different states, which the old computation
# cannot express. **A property stated in the words of the report is not
# necessarily a property that catches the defect being reported.**
#
# THE CASES THAT MUST STILL SKIP CARRY EQUAL WEIGHT, even though they do not
# discriminate. A gate that fires on every push gets `--no-verify`'d, which is
# cry-wolf arriving through a different door, so the board-only and already-level
# cases are as load-bearing as the regression -- they are what keeps the fix from
# being "make it always check", which would pass tests 1, 2, 5 and be useless.
#
# AND SINCE 0518 EVERY ARM RUNS OVER A DELIVERED PAIR, WHICH THIS FIXTURE STAMPS.
# The runner takes `artefact_currency_verdict` BEFORE the path trigger, because a
# whiteboard-only push over a stale pair is exactly as stale -- 0518's judged
# placement, kept by hv's ruling on issue 0521. A fixture repo holding no pair
# is therefore refused before the range is read. Measured as 0521: every arm red
# on both CI legs and on this host, each printing `no intent binary at
# <repo>/native/rust/target/release/intent` -- and CI showed none of that,
# because no assertion here printed the runner's output. So `gate` stamps a pair
# the verdict passes before every run, every assertion carries `$output`, and
# the last arm holds the placement itself.

load "../lib/test_helper.bash"

RUNNER="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/prepush"
ZERO="0000000000000000000000000000000000000000"

# THE TRIMMED PATH CARRIES ONE THING MORE THAN /usr/bin:/bin -- A BASH AT THE
# FLOOR. devbin 0.1.2 refuses at load on any bash below 5 (helpers, issue 0076),
# and macOS ships /bin/bash 3.2 permanently, so a PATH of /usr/bin:/bin alone
# resolves `bash` to a runner that refuses before it reads a ref pair. Measured
# in the 3.2.0 cut's dry run, 2026-09-21: every arm red, each with `int: bash >=
# 5 is required; this is bash 3.2.57(1)-release (/bin/bash)`. The bash this
# suite runs on clears the floor, but on this host it lives in the same
# directory as cargo, so that directory cannot join the PATH without handing
# back the lever. A private directory holding ONE link, `bash`, is what joins,
# and `setup` asserts the floor the same way it asserts the lever.

# THE PAIR, AS THE CURRENCY VERDICT READS IT. The verdict takes each binary's
# `[intent-source-commit:<sha>]` marker through `strings`, which reads a plain
# file as readily as a binary, so two small files carrying the marker are a pair
# to it and no build is needed. They sit where the real pair does, and `setup`
# ignores `native/rust/target/` as this repository does, so no `git add -A` in
# an arm commits them into a range.
stamp_pair() {
  local dir="$REPO/native/rust/target/release" b
  mkdir -p "$dir"
  for b in intent intentd; do
    printf '[intent-source-commit:%s]\n' "$1" >"$dir/$b"
  done
}

# Run the runner the way git's pre-push hook does: ref pairs on stdin, no argv.
# Pass "" for stdin to exercise the by-hand path. `runner` runs over whatever
# pair is stamped; `gate` stamps one at HEAD first, which is the state every
# range arm describes.
gate() {
  stamp_pair "$(at HEAD)"
  runner "$1"
}

runner() {
  local stdin_content="$1"
  if [ -z "$stdin_content" ]; then
    env -i HOME="$HOME" PATH="$TRIMMED_PATH" DEVBIN_NAME=int \
      DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
      PROJECT_ROOT="$REPO" \
      bash "$RUNNER" </dev/null 2>&1
  else
    printf '%s\n' "$stdin_content" |
      env -i HOME="$HOME" PATH="$TRIMMED_PATH" DEVBIN_NAME=int \
        DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
        PROJECT_ROOT="$REPO" \
        bash "$RUNNER" 2>&1
  fi
}

refpair() { printf 'refs/heads/main %s refs/heads/main %s' "$1" "$2"; }
at() { git -C "$REPO" rev-parse "$1"; }

# EVERY ASSERTION CARRIES THE RUNNER'S OWN WORDS. 0521 went red on CI with none
# of them, and the one line that named the cause had to be fetched by running
# the file again here. Each outcome is asserted in one place, so no arm can
# state it without printing what the runner said.
expect_engaged() {
  [ "$status" -ne 0 ] || fail "expected the gate to ENGAGE, got status 0: $output"
  [[ "$output" == *"cargo not on PATH"* ]] ||
    fail "expected the build half, dying 'cargo not on PATH'; got status $status: $output"
}

expect_skipped() {
  [ "$status" -eq 0 ] || fail "expected the gate to SKIP, got status $status: $output"
  [[ "$output" == *"no native/ or build-manifest change"* ]] ||
    fail "expected 'no native/ or build-manifest change'; got status $status: $output"
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
    skip_other_system "the bash on the trimmed PATH is below devbin's floor of 5 -- the runner would refuse at load, before it reads a ref pair, and every arm would fail for that reason alone"
  PATH="$TRIMMED_PATH" command -v cargo >/dev/null 2>&1 &&
    skip_other_system "cargo IS on the trimmed PATH -- the lever that separates skipped from engaged does not take here, and a run that cannot distinguish them would pass for the wrong reason"

  # TWO remotes, which is the whole point: one bare repo per remote, named the
  # way this project names them, with `upstream` tracked exactly as here.
  # Name the branch EXPLICITLY on all three, because the default is not ours to
  # assume: bare `git init` takes its branch name from `init.defaultBranch` in the
  # user's own `~/.gitconfig`. matts has that set to `main`, so this fixture built
  # `main` here and passed; the CI runners do not set it, so they built `master`
  # and every test in this file died in `setup` with `src refspec main does not
  # match any` -- 29 consecutive red runs of the `Intent Tests` leg while the
  # `rust` leg stayed green. The test was passing on this machine because of a
  # line in a personal config file, not because the gate under it was correct.
  # `release_script.bats:34` already writes it this way.
  git init -q -b main --bare "$TEST_TEMP_DIR/local.git"
  git init -q -b main --bare "$TEST_TEMP_DIR/upstream.git"
  git init -q -b main "$REPO"
  git -C "$REPO" config user.email t@t
  git -C "$REPO" config user.name t
  mkdir -p "$REPO/native/rust" "$REPO/intent/whiteboard"
  printf 'base\n' >"$REPO/README.md"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm base
  git -C "$REPO" remote add local "$TEST_TEMP_DIR/local.git"
  git -C "$REPO" remote add upstream "$TEST_TEMP_DIR/upstream.git"
  git -C "$REPO" push -q local main
  git -C "$REPO" push -q upstream main
  git -C "$REPO" branch --set-upstream-to=upstream/main main >/dev/null 2>&1
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
  return 0
}

# The state cc measured: a native/ change already pushed to `upstream`, so the
# tracked remote is level with HEAD while `local` is still behind. Asserts the
# rig reached that state before any test trusts it.
seed_pushed_to_upstream_only() {
  printf '[workspace]\n' >"$REPO/native/rust/Cargo.toml"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "native change"
  git -C "$REPO" push -q upstream main
  git -C "$REPO" fetch -q --all
  [ "$(at HEAD)" = "$(at upstream/main)" ] || {
    echo "rig did not reach the measured state: upstream is not level with HEAD" >&2
    return 1
  }
  [ -z "$(git -C "$REPO" diff --name-only upstream/main...HEAD)" ] || {
    echo "rig did not reach the measured state: the old range is not empty" >&2
    return 1
  }
}

@test "a push carrying native/ is gated at the remote that is behind" {
  # THE REGRESSION. The old computation saw an empty range here and exited 0.
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at local/main)")"
  expect_engaged
}

@test "the answer follows each remote's own position, not the tracked remote's" {
  # cc asked for "a push carrying native/ is gated regardless of which remote it
  # goes to". Written literally -- both remotes behind, assert both gate -- that
  # PASSES ON THE BROKEN RUNNER, twice over: the old range is non-empty when
  # both are behind, and the defect ignores stdin entirely, so "the same answer
  # whichever remote" is trivially satisfied by the bug. Measured, not reasoned:
  # the literal form was written first and survived the mutation.
  #
  # The property with teeth is the one below. ONE HEAD, two remotes in DIFFERENT
  # states, two different answers -- each correct for its own remote. A runner
  # that consults `@{upstream}` cannot produce two answers here at all.
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at local/main)")"
  expect_engaged
  run gate "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  expect_skipped
}

@test "a push to a remote already level is skipped" {
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  expect_skipped
}

@test "a board-only push is skipped even when the remote is behind" {
  # No false positives: the range is genuinely non-empty here, and the path
  # trigger is what must decide. A gate that fires on every push gets bypassed.
  local prev
  prev="$(at HEAD)"
  printf 'note\n' >"$REPO/intent/whiteboard/note.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "board only"
  [ -n "$(git -C "$REPO" diff --name-only "$prev"...HEAD)" ] ||
    fail "rig did not reach the measured state: the board-only range is empty"
  run gate "$(refpair "$(at HEAD)" "$prev")"
  expect_skipped
}

@test "a push creating a new ref on the remote is gated" {
  # Every commit is arriving and there is no bounded range to compute, so the
  # gate pays for the check rather than guessing a narrow one.
  run gate "$(refpair "$(at HEAD)" "$ZERO")"
  expect_engaged
}

@test "deleting a ref is skipped -- no tree is pushed" {
  run gate "$(refpair "$ZERO" "$(at local/main)")"
  expect_skipped
}

@test "run by hand it answers for the remote that is furthest behind" {
  # No stdin, so there are no ref pairs and no single right answer -- "what
  # would go if I pushed now" differs per remote. The union is what keeps the
  # by-hand path from reintroducing the same under-report wearing another name.
  seed_pushed_to_upstream_only
  run gate ""
  expect_engaged
}

@test "run by hand with no remotes at all, it checks rather than guesses" {
  git -C "$REPO" remote remove local
  git -C "$REPO" remote remove upstream
  run gate ""
  expect_engaged
}

@test "a stale pair is refused before the path trigger, on a push that would skip" {
  # 0518's own property, and the placement hv's ruling on 0521 keeps. ONE push,
  # two pairs: over a current pair this board-only push skips, and over a pair
  # built before the native change it is refused -- so the pair is the only thing
  # that moved the answer. Put the verdict after the path trigger, or take it
  # out, and the second run exits 0 like the first.
  seed_pushed_to_upstream_only
  local before_native
  before_native="$(at HEAD~1)"
  printf 'note\n' >"$REPO/intent/whiteboard/note.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "board only"

  run gate "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  expect_skipped

  stamp_pair "$before_native"
  run runner "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  [ "$status" -ne 0 ] || fail "expected the stale pair to be REFUSED, got status 0: $output"
  [[ "$output" == *"BLOCKED: the delivered pair"* ]] ||
    fail "expected the currency refusal; got status $status: $output"
  [[ "$output" == *"behind HEAD"* ]] ||
    fail "expected the pair named as BEHIND, not missing or unread; got status $status: $output"
  [[ "$output" == *"remedy: bin/devbin build all"* ]] ||
    fail "expected the rebuild remedy; got status $status: $output"
}

# A RELEASE PUSH, THE WAY `bin/devbin build release` MAKES ONE (issue 0546). The
# release commit bumps the workspace version in native/rust/Cargo.toml, which is
# a build input, and tags it; the pair on disk was built before that commit. So
# the release's own push is refused by the verdict above unless something builds
# the pair at the tag first, which is what the release step's `pair_at_tag` now
# does. The arms below run THAT function, lifted from the shipped script, in a
# repository laid out as this one is: the three currency libraries at their real
# path, `source_commit.rs` beside the workspace so the libraries read the real
# build-input scope, and a `bin/devbin` that stands in for the dispatcher.
# `build all` in the stand-in stamps the pair at HEAD, which is what a promoted
# build names; DEVBIN_STUB_FAIL makes it fail, and DEVBIN_STUB_NOPROMOTE makes
# it exit 0 without touching the pair, as the real one does on a dirty tree.
RELEASE_SCRIPT="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/build.d/release"

cut_release() {
  local shared="$REPO/bin/.devbin/cmd/shared" lib
  mkdir -p "$shared" "$REPO/native/rust/build-support" "$REPO/bin"
  for lib in artefact sharedtarget currency; do
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

  # The release commit and its tag, as the release step makes them.
  printf '[workspace.package]\nversion = "3.2.1"\n' >"$REPO/native/rust/Cargo.toml"
  git -C "$REPO" commit -qam "release: v3.2.1"
  git -C "$REPO" tag v3.2.1

  sed -n '/^pair_at_tag() {/,/^}/p' "$RELEASE_SCRIPT" >"$TEST_TEMP_DIR/pair_at_tag.sh"
  grep -q '^pair_at_tag() {' "$TEST_TEMP_DIR/pair_at_tag.sh" ||
    fail "no pair_at_tag() in $RELEASE_SCRIPT -- the release step no longer builds the pair at the tag"
}

# The two ref pairs git hands the hook for `git push local main v3.2.1`.
release_refs() {
  printf 'refs/heads/main %s refs/heads/main %s\nrefs/tags/v3.2.1 %s refs/tags/v3.2.1 %s' \
    "$(at HEAD)" "$(at local/main)" "$(at v3.2.1)" "$ZERO"
}

# Run pair_at_tag as the release step does, with only the stand-in's switches
# added to a clean environment.
pair_at_tag_run() {
  env -i HOME="$HOME" PATH="$TRIMMED_PATH" PROJECT_ROOT="$REPO" "$@" \
    bash -c '. "$1" && pair_at_tag' _ "$TEST_TEMP_DIR/pair_at_tag.sh" 2>&1
}

@test "a release push passes the pre-push check once the release step has built the pair at the tag" {
  cut_release

  # The defect, as vc measured it: over the pair built before the release
  # commit, the release's own push is refused.
  run runner "$(release_refs)"
  [ "$status" -ne 0 ] || fail "expected the release push to be REFUSED over the pre-release pair, got status 0: $output"
  [[ "$output" == *"BLOCKED: the delivered pair"* ]] ||
    fail "expected the currency refusal; got status $status: $output"

  run pair_at_tag_run
  [ "$status" -eq 0 ] || fail "expected pair_at_tag to pass the pair it built, got status $status: $output"
  [ "$output" = "ok" ] || fail "expected the verdict 'ok'; got: $output"

  # The same push over the pair built at the tag reaches the range decision,
  # and the release commit carries native/, so the gate engages.
  run runner "$(release_refs)"
  expect_engaged
}

@test "a build at the tag that fails refuses before the push, naming the build" {
  cut_release
  run pair_at_tag_run DEVBIN_STUB_FAIL=1
  [ "$status" -eq 1 ] || fail "expected pair_at_tag to refuse a failed build, got status $status: $output"
  [[ "$output" == *"refuse:bin/devbin build all failed"* ]] ||
    fail "expected the refusal to name the build; got: $output"
}

@test "a build at the tag that promotes nothing refuses before the push, with the hook's own verdict" {
  # `build all` exits 0 into target/private/release on a dirty tree, so its exit
  # code is not the answer; the verdict the hook takes is.
  cut_release
  run pair_at_tag_run DEVBIN_STUB_NOPROMOTE=1
  [ "$status" -eq 1 ] || fail "expected pair_at_tag to refuse an unpromoted pair, got status $status: $output"
  [[ "$output" == *"refuse:"*"behind HEAD"* ]] ||
    fail "expected the currency verdict naming the pair as behind; got: $output"
}
