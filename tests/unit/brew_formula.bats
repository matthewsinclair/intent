#!/usr/bin/env bats
# Guards for Intent's Homebrew formula and cask, rendered by devbin's built-in
# `formula` in assets mode from the templates under packaging/homebrew/ (ST0091
# WP-03). The formula names macOS arm64's intent, intentd and support tree and
# the Linux x86_64 and aarch64 packages, the cask names Intent.app.zip, and
# every sha256 in either is read from the release's SHA256SUMS.
#
# NO ARM HERE REACHES THE NETWORK. Each renders in a scratch repository holding
# a copy of this checkout's bin/devbin, bin/.devbin and packaging/, committed
# and tagged as a release, with `--sums` naming a sums file the arm writes, so
# the built-in reads no release.
#
# ST0087 AT-04.1 cites this file for AC-04.1: the formula is rendered from the
# release's version and per-asset checksums, installs the release's assets,
# writes the keg's manifest with managed_by brew, links both binaries, runs the
# version line in test do and names intent bootstrap and intent use in its
# caveats, and an arm below pins each clause.
#
# ST0091 AT-03.1 cites this file for AC-03.1, with release_publish.bats.
# ST0091 AT-03.3 cites this file for AC-03.3.

load "../lib/test_helper.bash"

ASSETS="intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz intent-x86_64-unknown-linux-musl.tar.gz intent-aarch64-unknown-linux-musl.tar.gz Intent.app.zip"

# make_release -- $REPO, a scratch repository holding this checkout's devbin and
# templates, committed and tagged v3.3.0 annotated, as the cut tags; and
# $SUMS, a sums file giving each asset a distinct sha256 of its own.
make_release() {
  REPO="${TEST_TEMP_DIR}/repo"
  mkdir -p "$REPO/bin"
  cp "${INTENT_PROJECT_ROOT}/bin/devbin" "$REPO/bin/"
  cp -R "${INTENT_PROJECT_ROOT}/bin/.devbin" "$REPO/bin/"
  cp -R "${INTENT_PROJECT_ROOT}/packaging" "$REPO/"
  printf '3.3.0\n' >"$REPO/VERSION"
  git -C "$REPO" init -q
  git -C "$REPO" config user.name t
  git -C "$REPO" config user.email t@t
  git -C "$REPO" add -A
  git -C "$REPO" commit -q -m base
  git -C "$REPO" tag -a v3.3.0 -m v3.3.0
  SUMS="${TEST_TEMP_DIR}/SHA256SUMS"
  : >"$SUMS"
  local a i=0
  for a in $ASSETS; do
    i=$((i + 1))
    printf '%064d  %s\n' "$i" "$a" >>"$SUMS"
  done
}

# render -- the built-in's render of v3.3.0 into $FORMULA and $CASK.
render() {
  FORMULA="${TEST_TEMP_DIR}/intent.rb"
  CASK="${TEST_TEMP_DIR}/intent-app.rb"
  run bash -c 'cd "$1" && bin/devbin formula render v3.3.0 "$2" "$3" --sums "$4" 2>&1' _ "$REPO" "$FORMULA" "$CASK" "$SUMS"
}

sum_of() { awk -v f="$1" '$2 == f { print $1 }' "$SUMS"; }

@test "AT-03.1: the formula and the cask render from the templates, every asset's sha256 read from the release's sums" {
  make_release
  render
  assert_success
  local a
  for a in $ASSETS; do
    if [ "$a" = Intent.app.zip ]; then
      grep -qF "sha256 \"$(sum_of "$a")\"" "$CASK" || fail "the cask does not carry $a's sum: $(cat "$CASK")"
    else
      grep -qF "sha256 \"$(sum_of "$a")\"" "$FORMULA" || fail "the formula does not carry $a's sum"
      grep -qF "/v3.3.0/$a\"" "$FORMULA" || fail "the formula does not fetch $a from the release's own URL"
    fi
  done
  grep -qF 'RELEASE_VERSION = "3.3.0"' "$FORMULA" || fail "the formula does not carry the release's version"
  grep -qF "SOURCE_COMMIT = \"$(git -C "$REPO" rev-parse "v3.3.0^{commit}")\"" "$FORMULA" ||
    fail "the formula does not name the tag's commit"
  grep -qF 'version "3.3.0"' "$CASK" || fail "the cask does not carry the release's version"
  ! grep -qF '@@' "$FORMULA" "$CASK" || fail "a placeholder survived the render"
}

@test "AT-03.1: an asset the release's sums do not name is refused, and nothing is rendered" {
  make_release
  grep -v ' intent-support.tar.gz$' "$SUMS" >"$SUMS.less"
  mv "$SUMS.less" "$SUMS"
  render
  assert_failure
  assert_output_contains "intent-support.tar.gz"
  [ ! -e "$FORMULA" ] || fail "a formula was written without every asset's sum"
}

@test "AT-03.1: the install lays the assets into libexec, writes the manifest with managed_by brew and links both binaries" {
  make_release
  render
  assert_success
  grep -qF '(libexec/"bin").install Dir["intent-*"].first => "intent"' "$FORMULA" || fail "the macOS CLI is not installed into libexec/bin"
  grep -qF 'resource("support").stage do' "$FORMULA" || fail "the support tree is not staged"
  grep -qF '"# managed_by: brew",' "$FORMULA" || fail "the manifest does not say managed_by brew"
  grep -qF '(libexec/".intent-install").write' "$FORMULA" || fail "the manifest is not written under libexec"
  grep -qF 'bin.install_symlink libexec/"bin/intent"' "$FORMULA" || fail "bin/intent is not linked"
  grep -qF 'bin.install_symlink libexec/"bin/intentd"' "$FORMULA" || fail "bin/intentd is not linked"
  grep -qF 'chmod 0755, libexec/"bin/intentd"' "$FORMULA" || fail "intentd is not made executable"
}

@test "AT-03.1: test do runs the version line, and the caveats name intent bootstrap and intent use" {
  make_release
  render
  assert_success
  grep -qF 'assert_match version.to_s, shell_output("#{bin}/intent --version")' "$FORMULA" || fail "test do does not run the version line"
  grep -qF 'intent bootstrap' "$FORMULA" || fail "the caveats do not name intent bootstrap"
  grep -qF 'intent use' "$FORMULA" || fail "the caveats do not name intent use"
}

# AC-03.3, issue 0667 (hv, 2026-10-31): the files `intent daemon logs` reads.
@test "AT-03.3: the service block sends stdout to intentd.log and stderr to intentd.err.log under the user's state dir" {
  make_release
  render
  assert_success
  grep -qF 'log_path "#{Dir.home}/.local/state/intent/intentd.log"' "$FORMULA" || fail "stdout does not go to intentd.log: $(grep -n log_path "$FORMULA")"
  grep -qF 'error_log_path "#{Dir.home}/.local/state/intent/intentd.err.log"' "$FORMULA" || fail "stderr does not go to intentd.err.log: $(grep -n log_path "$FORMULA")"
  grep -qF 'intentd writes every line it logs to stdout or stderr' "$FORMULA" || fail "the comment does not say where intentd writes"
}

# AC-03.1: cmd/brew keeps the smoke and nothing else, and no render or publish
# code is left in it or in brew.lib.
@test "AT-03.1: bin/devbin brew offers the smoke alone, and neither it nor brew.lib carries a renderer or a tap push" {
  run bash -c 'cd "$1" && bin/devbin brew --help 2>&1' _ "$INTENT_PROJECT_ROOT"
  assert_success
  assert_output_contains "smoke"
  [[ "$output" != *"render <tag>"* ]] || fail "brew still offers render: $output"
  run bash -c 'cd "$1" && bin/devbin brew publish 2>&1' _ "$INTENT_PROJECT_ROOT"
  assert_failure
  local f
  for f in bin/.devbin/cmd/brew bin/.devbin/cmd/shared/brew.lib; do
    ! grep -qE 'brew_render_formula|brew_render_cask|brew_push_tap|cmd_render|cmd_publish' "${INTENT_PROJECT_ROOT}/$f" ||
      fail "$f still carries render or publish code"
  done
}
