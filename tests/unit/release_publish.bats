#!/usr/bin/env bats
# The cut's publication, bin/.devbin/cut/publish, the smoke's tap check and the
# stage's refusal of a pair that cannot ship (ST0091 WP-03, vc decision 118).
#
# ST0091 AT-03.1 cites this file for AC-03.1, with brew_formula.bats.
# ST0091 AT-03.2 cites this file for AC-03.2.
#
# devbin's release core runs `bin/devbin cut publish` and then `bin/devbin brew
# smoke --at-tag` as its after: commands, both under the cut's yes. The arms
# drive each as the core does, in a scratch repository tagged as a release, with
# PROJECT_ROOT pointed at it and stand-ins where the subject is what the step
# does with an answer: a `bin/devbin` that logs each step it is asked for, a
# `gh` that names the assets the release holds and the version the tap carries,
# and a pair whose markers are written as a build embeds them. The build, the
# signing and notarisation, the upload and the tap push are the stand-ins'.

load "../lib/test_helper.bash"

PUBLISH="${INTENT_PROJECT_ROOT}/bin/.devbin/cut/publish"
MAC_ASSETS="intent-aarch64-apple-darwin.tar.gz Intent.app.zip"

# make_repo -- $REPO committed and tagged v3.3.0 annotated, its stand-in
# `bin/devbin`, and the stand-in `gh` first on PATH. The stand-in devbin logs
# every call to $CALLS and fails the step STUB_FAIL names; its `build all`
# writes a pair that is a release build of HEAD unless STUB_BUILD_DEV is set.
# The stand-in gh answers `release view` with the names in $HELD, one per line,
# and the tap's formula with RELEASE_VERSION $STUB_TAP_VERSION.
make_repo() {
  REPO="${TEST_TEMP_DIR}/repo"
  mkdir -p "$REPO/bin" "$REPO/native/rust/target/release"
  printf 'native/rust/target/\n' >"$REPO/.gitignore"
  cat >"$REPO/bin/devbin" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$CALLS"
[ "$*" != "${STUB_FAIL:-}" ] || { echo "stub devbin: $* failed" >&2; exit 1; }
if [ "$*" = "build all" ]; then
  kind=release
  [ -z "${STUB_BUILD_DEV:-}" ] || kind=dev
  for b in intent intentd; do
    printf '[intent-source-commit:%s][intent-source-kind:%s]\n' "$(git rev-parse HEAD)" "$kind" >"native/rust/target/release/$b"
  done
fi
exit 0
EOF
  chmod +x "$REPO/bin/devbin"
  git -C "$REPO" init -q
  git -C "$REPO" config user.name t
  git -C "$REPO" config user.email t@t
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m base
  git -C "$REPO" tag -a v3.3.0 -m v3.3.0

  STUBS="${TEST_TEMP_DIR}/stubs"
  mkdir -p "$STUBS"
  cat >"$STUBS/gh" <<'EOF'
#!/usr/bin/env bash
case "$1 $2" in
  "release view") cat "$HELD" ;;
  "api repos/"*) printf 'class Intent < Formula\n  RELEASE_VERSION = "%s".freeze\n' "${STUB_TAP_VERSION:-3.2.8}" ;;
  *) echo "stub gh: unexpected arguments: $*" >&2; exit 2 ;;
esac
EOF
  chmod +x "$STUBS/gh"
  CALLS="${TEST_TEMP_DIR}/calls"
  HELD="${TEST_TEMP_DIR}/held"
  : >"$CALLS"
  : >"$HELD"
  export CALLS HELD
}

# write_pair <kind> <commit> -- the pair as a build embeds its markers.
write_pair() {
  local b
  for b in intent intentd; do
    printf '[intent-source-commit:%s][intent-source-kind:%s]\n' "$2" "$1" >"$REPO/native/rust/target/release/$b"
  done
}

hold_all() { printf '%s\n' $MAC_ASSETS >"$HELD"; }

publish() {
  ERR="${TEST_TEMP_DIR}/publish.err"
  run env PROJECT_ROOT="$REPO" PATH="$STUBS:$PATH" ${CONFIRMED:+DEVBIN_RELEASE_CONFIRMED=$CONFIRMED} \
    bash -c '"$1" 2>"$2"' _ "$PUBLISH" "$ERR"
}

err_has() {
  grep -qF -- "$1" "$ERR" || fail "expected stderr to name '$1'; it was: $(cat "$ERR")"
}

calls_are() {
  local want
  want="$(printf '%s\n' "$@")"
  [ "$(cat "$CALLS")" = "$want" ] || fail "expected the steps:
$want
they were:
$(cat "$CALLS")"
}

@test "AT-03.1: a first run builds the pair at the tag, stages and publishes the macOS assets, writes the sums and pushes the tap, in that order" {
  make_repo
  write_pair dev "$(git -C "$REPO" rev-parse HEAD)"
  publish
  assert_success
  calls_are "build all" "macos prepare" "macos publish" "release attach v3.3.0" "formula publish --pull"
}

@test "AT-03.1: a run after a full success rebuilds nothing and uploads nothing, and the sums and the tap steps find their work done" {
  make_repo
  write_pair release "$(git -C "$REPO" rev-parse HEAD)"
  hold_all
  publish
  assert_success
  calls_are "release attach v3.3.0" "formula publish --pull"
  assert_output_contains "already a release build"
  assert_output_contains "already holds every macOS asset"
}

# The reason the four steps are one command: a tap ahead of its assets breaks
# every `brew upgrade`.
@test "AT-03.1: a failed macOS publish stops the run before the sums and the tap, and the remedy is the same run again" {
  make_repo
  write_pair release "$(git -C "$REPO" rev-parse HEAD)"
  export STUB_FAIL="macos publish"
  publish
  assert_failure
  calls_are "macos prepare" "macos publish"
  err_has "the tap is as it was"
  err_has "run the same cut again"
}

@test "AT-03.1: a pair the build leaves as a dev build is refused before anything is staged" {
  make_repo
  export STUB_BUILD_DEV=1
  publish
  assert_failure
  calls_are "build all"
  err_has "is not a release build of"
}

@test "AT-03.1: a release holding some macOS assets and not others is refused by name, and nothing is uploaded" {
  make_repo
  write_pair release "$(git -C "$REPO" rev-parse HEAD)"
  printf 'intent-aarch64-apple-darwin.tar.gz\n' >"$HELD"
  publish
  assert_failure
  [ ! -s "$CALLS" ] || fail "a step ran over a half-published release: $(cat "$CALLS")"
  err_has "held intent-aarch64-apple-darwin.tar.gz"
  err_has "missing Intent.app.zip"
  # The remedy names the one command for each held asset, to run as printed.
  err_has "gh release delete-asset v3.3.0 intent-aarch64-apple-darwin.tar.gz --repo matthewsinclair/intent --yes"
  [ "$(grep -c 'gh release delete-asset' "$ERR")" -eq "$(grep -c . "$HELD")" ] ||
    fail "expected one delete command per held asset: $(cat "$ERR")"
  err_has "then run the same cut again"
}

@test "AT-03.1: HEAD without a release tag, or a tag the cut did not confirm, is refused before any step" {
  make_repo
  CONFIRMED=v3.3.1 publish
  assert_failure
  err_has "the cut confirmed v3.3.1"
  git -C "$REPO" tag -d v3.3.0 >/dev/null
  publish
  assert_failure
  err_has "no release tag is at HEAD"
  [ ! -s "$CALLS" ] || fail "a step ran with no release to publish: $(cat "$CALLS")"
}

# vc's ruling: the core runs the smoke even when publish failed before it, and
# a smoke of the previous release must not read green over that.
@test "AT-03.1: brew smoke --at-tag says the publish did not finish when the tap is not at the tag's version, and smokes nothing" {
  make_repo
  export STUB_TAP_VERSION=3.2.8
  run env DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" PROJECT_ROOT="$REPO" PATH="$STUBS:$PATH" \
    bash "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/brew" smoke --at-tag
  assert_failure
  assert_output_contains "the tap is not at 3.3.0: the publish step did not finish"
  assert_output_contains "Nothing was smoked"
}

# AC-03.2, vc's ruling on dc's finding F1.
@test "AT-03.2: macos prepare refuses a pair that is a dev build, or a release build of another commit, naming the rebuild" {
  make_repo
  local head
  head="$(git -C "$REPO" rev-parse HEAD)"
  write_pair dev "$head"
  run env DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" PROJECT_ROOT="$REPO" \
    bash "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/macos" prepare
  assert_failure
  assert_output_contains "a dev build, not a release build"
  assert_output_contains "bin/devbin build all"
  write_pair release 0123456789abcdef0123456789abcdef01234567
  run env DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" PROJECT_ROOT="$REPO" \
    bash "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/macos" prepare
  assert_failure
  assert_output_contains "built at 0123456789abcdef0123456789abcdef01234567, not at HEAD $head"
}

# AC-03.2: the asset names 3.2.8 shipped, read from the one place that names
# them for the stage, are the formula's.
@test "AT-03.2: the formula's macOS assets are the ones the stage names" {
  local a assets
  assets="$(awk '/^formula:/ { f = 1; next } f && /^[^ #]/ { f = 0 } f && /^  assets:/ { a = 1; next } a && /^    - / { sub(/^    - /, ""); print; next } a { a = 0 }' "${INTENT_PROJECT_ROOT}/bin/.devbin/config.yaml")"
  [ -n "$assets" ] || fail "config.yaml's formula: block declares no assets"
  for a in $MAC_ASSETS; do
    grep -qx -- "$a" <<<"$assets" || fail "the formula does not name the stage's asset $a: $assets"
  done
  run bash -c 'ROOT=/x; . "$1/bin/.devbin/cmd/shared/brew.lib"; brew_macos_assets; printf "%s\n" "$APP_ASSET"' _ "$INTENT_PROJECT_ROOT"
  assert_success
  [ "$(printf '%s\n' $MAC_ASSETS)" = "$output" ] || fail "brew.lib names other macOS assets: $output"
}
