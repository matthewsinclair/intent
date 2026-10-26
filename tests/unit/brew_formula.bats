#!/usr/bin/env bats
# Guards for Intent's Homebrew formula, rendered by `bin/int brew render` and by
# `bin/int macos publish` through one renderer, `cmd/shared/brew.lib` (ST0081
# WP-06). The formula names macOS arm64's intent, intentd and support tree and
# the Linux x86_64 and aarch64 packages, and every sha256 in it is read from the
# bytes at a base URL: the release's own, or a file:// directory for a dev tap.
#
# NO ARM HERE REACHES THE NETWORK OR THE CHECKOUT'S STAGING. The verb is driven
# against file:// fixtures in TEST_TEMP_DIR, and the arms that need the release's
# own base or `publish` run a copy of bin/ in a sandbox, whose STAGE_DIR is the
# sandbox's, with gh, curl and rustc stood in on PATH.

load "../lib/test_helper.bash"

# INTENT_BREW_LIB redirects every lib-level arm at another copy, for mutation
# testing without editing the shipped file, as INTENT_MACOS_SCRIPT does.
BREW_LIB="${INTENT_BREW_LIB:-${INTENT_HOME}/bin/.devbin/cmd/shared/brew.lib}"
FIVE="intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz intent-x86_64-unknown-linux-musl.tar.gz intent-aarch64-unknown-linux-musl.tar.gz"
# The commit render_formula hands the renderer, as brew_source_commit would.
PLANTED_COMMIT="0123456789abcdef0123456789abcdef01234567"

# assets_in <dir> <asset>... -- a directory holding a small stand-in for each.
assets_in() {
  local dir="$1" a
  shift
  mkdir -p "$dir"
  for a in "$@"; do printf 'stand-in for %s\n' "$a" >"$dir/$a"; done
}

# brew_render_verb <arg>... -- the real `brew render`, from the project root,
# which the dispatcher requires.
brew_render_verb() {
  cd "$INTENT_PROJECT_ROOT" || return 1
  run bin/int brew render "$@"
}

# render_formula -- the release's shape through the lib's own renderer, from a
# planted sums file naming all five assets. die is the only helper it calls, and
# nothing leaves TEST_TEMP_DIR.
render_formula() {
  local sums="${TEST_TEMP_DIR}/fetched.sums" t
  # shellcheck disable=SC2329 # the sourced renderer calls it
  die() {
    printf 'die: %s\n' "$*" >&2
    return 1
  }
  # shellcheck disable=SC2034 # brew.lib reads it as it is sourced
  ROOT="$TEST_TEMP_DIR"
  # shellcheck disable=SC1090
  . "$BREW_LIB"
  {
    printf 'aaa  intent-%s\nbbb  intentd-%s\nccc  %s\n' "$MACOS_TRIPLE" "$MACOS_TRIPLE" "$SUPPORT_ASSET"
    for t in $LINUX_TRIPLES; do printf 'ddd-%s  %s\n' "$t" "$(brew_linux_asset "$t")"; done
  } >"$sums"
  brew_render_formula 9.9.9 0 release "$sums" "$PLANTED_COMMIT"
}

# brew_sandbox -- SB, a copy of this checkout's bin/ whose STAGE_DIR is its own,
# and SHIMS, a directory for stand-ins to put first on PATH: a rustc that names
# aarch64-apple-darwin, a gh that answers `release view` from GH_ASSETS (one
# asset name per line, counted when the query asks for a length), and a curl
# that serves CURL_FIXTURES for the v9.9.9 release's download URLs only.
brew_sandbox() {
  SB="${TEST_TEMP_DIR}/sb"
  SHIMS="${TEST_TEMP_DIR}/shims"
  mkdir -p "$SB/native/rust/target/dist" "$SHIMS"
  cp -R "$INTENT_PROJECT_ROOT/bin" "$SB/"
  printf '#!/bin/sh\nprintf "rustc 1.99.0\\nhost: aarch64-apple-darwin\\n"\n' >"$SHIMS/rustc"
  cat >"$SHIMS/gh" <<'EOF'
#!/bin/sh
for a in "$@"; do last="$a"; done
case "$*" in
  *--json*)
    case "$last" in
      *length*) grep -c . "$GH_ASSETS" ;;
      *) cat "$GH_ASSETS" ;;
    esac
    ;;
esac
exit 0
EOF
  cat >"$SHIMS/curl" <<'EOF'
#!/bin/sh
out="" url=""
while [ $# -gt 0 ]; do
  case "$1" in
    -o) out="$2"; shift 2 ;;
    -*) shift ;;
    *) url="$1"; shift ;;
  esac
done
case "$url" in
  https://github.com/matthewsinclair/intent/releases/download/v9.9.9/*) ;;
  *) echo "curl: (6) the stand-in serves only v9.9.9's release: $url" >&2; exit 6 ;;
esac
[ -f "$CURL_FIXTURES/${url##*/}" ] || { echo "curl: (22) The requested URL returned error: 404" >&2; exit 22; }
cp "$CURL_FIXTURES/${url##*/}" "$out"
EOF
  chmod +x "$SHIMS/rustc" "$SHIMS/gh" "$SHIMS/curl"
}

# sandbox_tag -- make SB a repository whose one commit carries the annotated tag
# v9.9.9, so a release formula has a commit for the keg's manifest to name, and
# print that commit. The sandbox is untagged by default because `macos publish`
# refuses there for want of the tag, and its arm asserts that refusal.
sandbox_tag() {
  local g=(git -C "$SB" -c user.name=t -c user.email=t@t.invalid -c core.hooksPath=/dev/null -c commit.gpgsign=false -c tag.gpgsign=false)
  "${g[@]}" init -q && "${g[@]}" commit -q --allow-empty -m sandbox && "${g[@]}" tag -a v9.9.9 -m v9.9.9 &&
    git -C "$SB" rev-parse 'v9.9.9^{commit}'
}


# hash_of <file> -- its sha256, from whichever tool this host has.
hash_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d' ' -f1
  else
    shasum -a 256 "$1" | cut -d' ' -f1
  fi
}

# stage_release -- the sandbox's staging as `macos prepare` leaves it: a CLI that
# reports 9.9.9, the daemon, the support tree, the app zip, and SHA256SUMS.txt
# with each one's real hash.
stage_release() {
  local d="$SB/native/rust/target/dist" a
  printf '#!/bin/sh\necho "intent 9.9.9 (0000000)"\n' >"$d/intent-aarch64-apple-darwin"
  chmod +x "$d/intent-aarch64-apple-darwin"
  for a in intentd-aarch64-apple-darwin intent-support.tar.gz Intent.app.zip; do printf 'staged %s\n' "$a" >"$d/$a"; done
  for a in intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz Intent.app.zip; do
    printf '%s  %s\n' "$(hash_of "$d/$a")" "$a"
  done >"$d/SHA256SUMS.txt"
}

@test "success: a release formula carries the macOS branch and both Linux architectures" {
  run render_formula
  assert_success
  assert_output_contains 'url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-aarch64-apple-darwin"'
  assert_output_contains 'sha256 "aaa"'
  assert_output_contains "on_macos do"
  assert_output_contains "depends_on arch: :arm64"
  assert_output_contains 'url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intentd-aarch64-apple-darwin"'
  assert_output_contains 'url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-support.tar.gz"'
  assert_output_contains "on_linux do"
  assert_output_contains 'on_intel do
      url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-x86_64-unknown-linux-musl.tar.gz"
      sha256 "ddd-x86_64-unknown-linux-musl"'
  assert_output_contains 'on_arm do
      url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-aarch64-unknown-linux-musl.tar.gz"
      sha256 "ddd-aarch64-unknown-linux-musl"'
  # The release's URLs carry /v<version>/, so a version line would be the
  # redundant one `brew audit --strict` refuses, and nothing limits a platform.
  refute_output_contains '  version "'
  refute_output_contains "depends_on :macos"
  refute_output_contains "depends_on :linux"
}

@test "the rendered caveat names the upgrade case and stays conditional (issue 0527)" {
  run render_formula
  assert_success
  # The upgrade case: a pointer naming any keg, whether or not it is still on
  # disk (issue 0540), and the door that says which case a machine is in.
  assert_output_contains "if it names an Intent under Cellar/intent/, whether"
  assert_output_contains "or not that version is still installed"
  assert_output_contains '`intent bootstrap --check` says where it points'
  refute_output_contains "no longer installed"
  # Still conditional, so a pointer naming a source checkout is never moved.
  assert_output_contains "if that file does not exist yet"
  assert_output_contains "such as a source checkout, needs nothing"
  # Backticks render as text: a heredoc that ran them would have dropped them.
  assert_output_contains 'only `intent bootstrap` writes'
  assert_output_contains '`brew upgrade` removes'
}

@test "the formula's install block is agnostic about what the archive carries" {
  # Rooted at the install root, so a shipped set that grows -- `intent critic` and
  # `intent claude rules` are unimplemented today and will need the rule library,
  # which lives outside the marker directory -- is a content change rather than a
  # formula change. On Linux the package is that root, so its line is the same.
  run grep -c -F 'libexec.install Dir["*"]' "$BREW_LIB"
  assert_success
  [ "$output" -eq 2 ] || fail "expected the support tree's line and the Linux package's, found $output"
}

@test "invariant: a release formula refuses each missing asset by name with its URL, and prints no formula" {
  local a others base
  for a in $FIVE; do
    base="${TEST_TEMP_DIR}/without-${a}"
    # shellcheck disable=SC2086 # FIVE is space-free names, split on purpose
    others="$(printf '%s\n' $FIVE | grep -vxF "$a" | tr '\n' ' ')"
    # shellcheck disable=SC2086
    assets_in "$base" $others
    brew_render_verb v9.9.9 - --from "file://$base"
    [ "$status" -ne 0 ] || fail "a base without $a rendered: $output"
    [[ "$output" == *"missing: $a from file://$base/$a"* ]] || fail "the refusal did not name $a with its URL: $output"
    [[ "$output" != *"class Intent"* ]] || fail "a partial formula was printed without $a: $output"
  done
}

@test "success: --dev-tap with only the Linux aarch64 package renders that branch with a version line and names the rest" {
  local base="${TEST_TEMP_DIR}/arm"
  assets_in "$base" intent-aarch64-unknown-linux-musl.tar.gz
  brew_render_verb v9.9.9 - --from "file://$base" --dev-tap
  assert_success
  # No tag v9.9.9 in this checkout, so the dev tap's manifest names HEAD, and says so.
  assert_output_contains "no tag v9.9.9 here, so the dev tap's manifest names HEAD"
  assert_output_contains "NOT THE RELEASE FORMULA"
  assert_output_contains '  version "9.9.9"'
  assert_output_contains "  depends_on :linux"
  assert_output_contains "    depends_on arch: :arm64"
  assert_output_contains "on_arm do
      url \"file://$base/intent-aarch64-unknown-linux-musl.tar.gz\""
  # At their own indentation: the url comment names `on_macos do` in its text.
  refute_output_contains "
  on_macos do
"
  refute_output_contains "
    on_intel do
"
  assert_output_contains "This formula renders no macOS branch"
  local a
  for a in intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz intent-x86_64-unknown-linux-musl.tar.gz; do
    assert_output_contains "not in this dev tap: $a from file://$base/$a"
  done
}

@test "invariant: brew publish refuses without a terminal, before fetching anything" {
  cd "$INTENT_PROJECT_ROOT" || return 1
  run bin/int brew publish </dev/null
  [ "$status" -eq 1 ] || fail "publish without a terminal exited $status: $output"
  assert_output_contains "nobody to ask"
  assert_output_contains "Nothing was fetched and nothing was pushed"
  refute_output_contains "missing:"
}

@test "invariant: with the release's base, a macOS hash SHA256SUMS.txt does not record refuses" {
  local a
  brew_sandbox
  stage_release
  local commit
  commit="$(sandbox_tag)" || fail "the sandbox could not be tagged"
  CURL_FIXTURES="${TEST_TEMP_DIR}/served"
  mkdir -p "$CURL_FIXTURES"
  for a in intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz; do
    cp "$SB/native/rust/target/dist/$a" "$CURL_FIXTURES/$a"
  done
  assets_in "$CURL_FIXTURES" intent-x86_64-unknown-linux-musl.tar.gz intent-aarch64-unknown-linux-musl.tar.gz
  # THE CONTROL FIRST: served bytes equal to the staged ones render, from the
  # release's own URLs.
  run env PATH="$SHIMS:$PATH" CURL_FIXTURES="$CURL_FIXTURES" bash -c 'cd "$1" && bin/int brew render v9.9.9 -' _ "$SB"
  [ "$status" -eq 0 ] || fail "the control did not render: $output"
  assert_output_contains 'url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-aarch64-apple-darwin"'
  # The manifest names the tag's commit, read from git, never typed.
  assert_output_contains "  SOURCE_COMMIT = \"$commit\".freeze"
  # Then the release serves a daemon the proof did not cover.
  printf 'other bytes\n' >"$CURL_FIXTURES/intentd-aarch64-apple-darwin"
  run env PATH="$SHIMS:$PATH" CURL_FIXTURES="$CURL_FIXTURES" bash -c 'cd "$1" && bin/int brew render v9.9.9 -' _ "$SB"
  [ "$status" -ne 0 ] || fail "a served daemon the sums do not record rendered: $output"
  assert_output_contains "intentd-aarch64-apple-darwin at https://github.com/matthewsinclair/intent/releases/download/v9.9.9 hashes to"
  assert_output_contains "SHA256SUMS.txt records"
  refute_output_contains "class Intent"
}

@test "invariant: publish leaves a release's other assets alone, and refuses one of its own by name" {
  brew_sandbox
  stage_release
  GH_ASSETS="${TEST_TEMP_DIR}/on-the-release"
  # A release carrying the Linux packages only: publish must pass the asset check
  # and go on to its next refusal, the tag this sandbox has no repository for.
  printf '%s\n' intent-x86_64-unknown-linux-musl.tar.gz intent-aarch64-unknown-linux-musl.tar.gz >"$GH_ASSETS"
  run env PATH="$SHIMS:$PATH" GH_ASSETS="$GH_ASSETS" bash -c 'cd "$1" && bin/int macos publish --dry-run' _ "$SB"
  [ "$status" -ne 0 ] || fail "publish --dry-run passed in a sandbox with no repository: $output"
  assert_output_contains "no tag v9.9.9 in this repository"
  refute_output_contains "already carries"
  refute_output_contains "carrying 2 asset(s)"
  # The same release carrying the CLI publish would upload: refused, by name.
  printf '%s\n' intent-aarch64-apple-darwin >>"$GH_ASSETS"
  run env PATH="$SHIMS:$PATH" GH_ASSETS="$GH_ASSETS" bash -c 'cd "$1" && bin/int macos publish --dry-run' _ "$SB"
  [ "$status" -ne 0 ] || fail "publish accepted a release already carrying its CLI: $output"
  assert_output_contains "already carries what this would upload:
       intent-aarch64-apple-darwin"
  refute_output_contains "no tag v9.9.9"
}

@test "the formula installs into libexec and symlinks bin" {
  # `lib` is a LINKED directory: brew symlinks keg lib subdirectories into the
  # shared prefix, so installing the support tree there would publish a directory
  # called `templates` into the global lib alongside every other formula's. The
  # binary must still sit beside the marker, so bin gets a symlink.
  run render_formula
  assert_success
  assert_output_contains 'bin.install_symlink libexec/"bin/intent"'
  assert_output_contains 'bin.install_symlink libexec/"bin/intentd"'
  # The pre-fix form put the binary straight into bin, where the walk finds no
  # marker above it.
  refute_output_contains 'bin.install Dir["intent-*"].first => "intent"'
}

@test "the support resource is declared inside on_macos with its own hash" {
  # At top level from 3.0.1 to 3.2.3, beside the url that had to leave `on_macos
  # do on_arm do` so `brew tap` could validate every simulated platform. The url
  # stays there. The resources moved into `on_macos` with ST0081 WP-06, because
  # the Linux package carries both binaries and the tree, and `brew readall --os
  # all --arch all` passed the rendered formula on 2026-10-14, with the 3.0.0
  # shape, url nested in `on_macos do on_arm do`, as the control that refused.
  run render_formula
  assert_success
  assert_output_contains '  on_macos do
    depends_on arch: :arm64

    resource "intentd" do'
  assert_output_contains '    resource "support" do
      url "https://github.com/matthewsinclair/intent/releases/download/v#{RELEASE_VERSION}/intent-support.tar.gz"
      sha256 "ccc"
    end'
}

@test "invariant: with no sha256 tool on PATH, brew render refuses and says why, where it exited in silence" {
  # brew_fetch runs with its stderr in a file that brew_formula shows only on its
  # own way out, so a refusal that ended the subshell from inside the fetch was
  # written into that file and never shown, and the temp directory stayed (vc,
  # 2026-10-14). The PATH is built: what the dispatcher runs before the handler,
  # curl and mktemp, and no sha256sum or shasum.
  local base="${TEST_TEMP_DIR}/arm" bin="${TEST_TEMP_DIR}/no-hash-bin" tmp="${TEST_TEMP_DIR}/tmp" t p
  assets_in "$base" intent-aarch64-unknown-linux-musl.tar.gz
  mkdir -p "$bin" "$tmp"
  for t in bash dirname readlink awk date tr sed rm cat grep head curl mktemp; do
    p="$(command -v "$t")" || fail "the arm's PATH needs $t, and this host has none"
    ln -s "$p" "$bin/$t"
  done
  cd "$INTENT_PROJECT_ROOT" || return 1
  run env PATH="$bin" TMPDIR="$tmp" bin/int brew render v9.9.9 - --from "file://$base" --dev-tap
  [ "$status" -ne 0 ] || fail "brew render rendered with no sha256 tool on PATH: $output"
  assert_output_contains "neither sha256sum nor shasum is on PATH"
  refute_output_contains "class Intent"
  [ -z "$(ls -A "$tmp")" ] || fail "the fetch directory was left behind: $(ls -A "$tmp")"
}

# THE KEG'S MANIFEST (ST0087 T3): the formula writes `.intent-install` over what
# it laid out, because the keg has no checkout to run `intent install` in.

@test "the formula writes the keg's manifest in T3's form, with managed_by brew" {
  run render_formula
  assert_success
  assert_output_contains "  SOURCE_COMMIT = \"$PLANTED_COMMIT\".freeze"
  assert_output_contains '    manifest = libexec/".intent-install"'
  # In `install`: Homebrew 7's `brew style` refuses a post_install.
  refute_output_contains "def post_install"
  assert_output_contains '      "# tool: intent",
      "# version: #{RELEASE_VERSION}",
      "# source_commit: #{SOURCE_COMMIT}",
      "# managed_by: brew",'
}

@test "the install block's manifest lists every file under libexec as shasum -c input" {
  command -v ruby >/dev/null || fail "this arm runs the formula's own Ruby, and this host has no ruby"
  local root="${TEST_TEMP_DIR}/keg/libexec" def="${TEST_TEMP_DIR}/manifest.rb" harness="${TEST_TEMP_DIR}/harness.rb" sum f
  run render_formula
  assert_success
  # The manifest's lines as rendered, from its first statement to the write.
  printf '%s\n' "$output" | awk '/^    manifest = libexec\/"\.intent-install"$/ {on=1} on {print} on && /^    manifest\.write/ {exit}' >"$def"
  grep -q '^    manifest\.write' "$def" || fail "no manifest write in the rendered formula"
  # The keg's libexec, stood in: its files are listed and summed, never run.
  for f in bin/intent bin/intentd lib/templates/a.md intent/plugins/claude/.keep; do
    mkdir -p "$root/${f%/*}"
    printf '%s\n' "$f" >"$root/$f"
  done
  # Homebrew's own Pathname#sha256 and the formula's two constants, stood in.
  cat >"$harness" <<'RUBY'
require "pathname"
require "digest"
class Pathname
  def sha256
    Digest::SHA256.file(self).hexdigest
  end
end
RELEASE_VERSION = "9.9.9".freeze
SOURCE_COMMIT = "0123456789abcdef0123456789abcdef01234567".freeze
def libexec
  Pathname.new(ARGV[0])
end
eval(File.read(ARGV[1]))
RUBY
  run ruby "$harness" "$root" "$def"
  assert_success
  run cat "$root/.intent-install"
  assert_output_contains "# tool: intent
# version: 9.9.9
# source_commit: $PLANTED_COMMIT
# managed_by: brew"
  assert_output_contains "  intent/plugins/claude/.keep"
  refute_output_contains ".intent-install"
  if command -v shasum >/dev/null 2>&1; then sum="shasum -a 256 -c"; else sum="sha256sum -c"; fi
  run bash -c "cd \"\$1\" && grep -v '^#' .intent-install | $sum" _ "$root"
  assert_success
  [ "$(printf '%s\n' "$output" | grep -c ': OK$')" -eq "$(cd "$root" && find . -type f ! -name .intent-install | grep -c .)" ] ||
    fail "the manifest does not list every file under libexec: $output"
  # And the check can fail: a file changed after the manifest is a mismatch.
  printf 'changed\n' >"$root/lib/templates/a.md"
  run bash -c "cd \"\$1\" && grep -v '^#' .intent-install | $sum" _ "$root"
  [ "$status" -ne 0 ] || fail "a changed file still matched the manifest: $output"
}

@test "the rendered caveat names intent use for a maintainer with a dev or opt tree" {
  run render_formula
  assert_success
  assert_output_contains '`intent` with `intent use`, and `intent use brew` hands it back to this keg.'
}

@test "the formula's test do runs the version line" {
  # `brew install` never runs this block and smoke drives the keg's binary
  # directly, so the rendered text is the one place the clause is pinned.
  run render_formula
  assert_success
  assert_output_contains '  test do
    assert_match version.to_s, shell_output("#{bin}/intent --version")
  end'
}

@test "invariant: brew render takes a release tag, and refuses a bare version" {
  brew_render_verb 9.9.9 -
  [ "$status" -ne 0 ] || fail "a bare version rendered: $output"
  assert_output_contains "brew render takes a release tag, v<version>, got '9.9.9'"
  brew_render_verb v9.9.9
  [ "$status" -ne 0 ] || fail "render with no <out> ran: $output"
  assert_output_contains "usage: bin/devbin brew render <tag> <out>"
}

@test "invariant: a release formula in a checkout without its tag refuses by name and renders nothing" {
  local a
  brew_sandbox
  stage_release
  CURL_FIXTURES="${TEST_TEMP_DIR}/served"
  mkdir -p "$CURL_FIXTURES"
  for a in intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz; do
    cp "$SB/native/rust/target/dist/$a" "$CURL_FIXTURES/$a"
  done
  assets_in "$CURL_FIXTURES" intent-x86_64-unknown-linux-musl.tar.gz intent-aarch64-unknown-linux-musl.tar.gz
  run env PATH="$SHIMS:$PATH" CURL_FIXTURES="$CURL_FIXTURES" bash -c 'cd "$1" && bin/int brew render v9.9.9 -' _ "$SB"
  [ "$status" -ne 0 ] || fail "a release formula rendered with no tag to name: $output"
  assert_output_contains "no tag v9.9.9 in"
  refute_output_contains "class Intent"
}

@test "success: brew render writes <out> and names it" {
  local base="${TEST_TEMP_DIR}/arm" out="${TEST_TEMP_DIR}/intent.rb"
  assets_in "$base" intent-aarch64-unknown-linux-musl.tar.gz
  brew_render_verb v9.9.9 "$out" --from "file://$base" --dev-tap
  assert_success
  assert_output_contains "wrote: $out"
  refute_output_contains "class Intent"
  grep -q '^class Intent < Formula$' "$out" || fail "$out does not hold the formula"
}

# THE CASK (issue 0634): Casks/intent-app.rb, rendered from the bytes of
# Intent.app.zip at a base, its url always the release's, and pushed beside the
# formula in the tap's one commit.

@test "success: --cask renders the app's cask from the zip's own bytes, at the release's URL" {
  local base="${TEST_TEMP_DIR}/app"
  assets_in "$base" Intent.app.zip
  brew_render_verb v9.9.9 - --cask --from "file://$base"
  assert_success
  assert_output_contains 'cask "intent-app" do'
  assert_output_contains '  version "9.9.9"'
  assert_output_contains "  sha256 \"$(hash_of "$base/Intent.app.zip")\""
  assert_output_contains '  url "https://github.com/matthewsinclair/intent/releases/download/v#{version}/Intent.app.zip"'
  assert_output_contains '  depends_on formula: "matthewsinclair/intent/intent"'
  assert_output_contains '  app "Intent.app"'
  refute_output_contains "url \"file://"
  refute_output_contains "class Intent"
}

@test "invariant: --cask with no app zip at the base refuses by name with its URL, and prints no cask" {
  local base="${TEST_TEMP_DIR}/no-app"
  # shellcheck disable=SC2086 # FIVE is space-free names, split on purpose
  assets_in "$base" $FIVE
  brew_render_verb v9.9.9 - --cask --from "file://$base"
  [ "$status" -ne 0 ] || fail "a base with no app zip rendered a cask: $output"
  assert_output_contains "missing: Intent.app.zip from file://$base/Intent.app.zip"
  refute_output_contains 'cask "intent-app"'
}

@test "invariant: with the release's base, a cask whose zip SHA256SUMS.txt does not record refuses" {
  brew_sandbox
  stage_release
  CURL_FIXTURES="${TEST_TEMP_DIR}/served"
  mkdir -p "$CURL_FIXTURES"
  cp "$SB/native/rust/target/dist/Intent.app.zip" "$CURL_FIXTURES/Intent.app.zip"
  # THE CONTROL FIRST: the staged zip, served, renders.
  run env PATH="$SHIMS:$PATH" CURL_FIXTURES="$CURL_FIXTURES" bash -c 'cd "$1" && bin/int brew render v9.9.9 - --cask' _ "$SB"
  [ "$status" -eq 0 ] || fail "the control did not render: $output"
  assert_output_contains "  sha256 \"$(hash_of "$CURL_FIXTURES/Intent.app.zip")\""
  # Then the release serves a zip the proof did not cover.
  printf 'other bytes\n' >"$CURL_FIXTURES/Intent.app.zip"
  run env PATH="$SHIMS:$PATH" CURL_FIXTURES="$CURL_FIXTURES" bash -c 'cd "$1" && bin/int brew render v9.9.9 - --cask' _ "$SB"
  [ "$status" -ne 0 ] || fail "a served zip the sums do not record rendered: $output"
  assert_output_contains "Intent.app.zip at https://github.com/matthewsinclair/intent/releases/download/v9.9.9 hashes to"
  assert_output_contains "SHA256SUMS.txt records"
  refute_output_contains 'cask "intent-app"'
}

@test "invariant: the tap push lands the formula and the cask in one commit" {
  local bare="${TEST_TEMP_DIR}/tap.git" shims="${TEST_TEMP_DIR}/push-shims" seed="${TEST_TEMP_DIR}/seed"
  git init -q --bare "$bare"
  git init -q "$seed"
  git -C "$seed" -c user.name=t -c user.email=t@t.invalid commit -q --allow-empty -m seed
  git -C "$seed" push -q "$bare" HEAD:refs/heads/main
  git -C "$bare" symbolic-ref HEAD refs/heads/main
  mkdir -p "$shims"
  # gh, stood in: `repo view` answers, and `repo clone <tap> <dir>` clones the bare tap.
  cat >"$shims/gh" <<'SHIM'
#!/bin/sh
[ "$1 $2" = "repo view" ] && exit 0
[ "$1 $2" = "repo clone" ] && exec git clone -q "$TAP_BARE" "$4"
exit 1
SHIM
  chmod +x "$shims/gh"
  run env PATH="$shims:$PATH" TAP_BARE="$bare" GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@t.invalid GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@t.invalid \
    bash -c 'die() { printf "die: %s\n" "$*" >&2; exit 1; }; require_tool() { command -v "$1" >/dev/null; }; ROOT="$2"; . "$1"; brew_push_tap "the formula" "the cask" 9.9.9 "made by a test"' _ "$BREW_LIB" "$TEST_TEMP_DIR"
  [ "$status" -eq 0 ] || fail "the push failed: $output"
  [ "$(git -C "$bare" rev-list --count main)" = 2 ] || fail "the push did not land as one commit on the seed: $(git -C "$bare" log --oneline main)"
  [ "$(git -C "$bare" diff-tree --no-commit-id --name-only -r main | sort | tr '\n' ' ')" = "Casks/intent-app.rb Formula/intent.rb " ] ||
    fail "the one commit did not carry exactly the formula and the cask: $(git -C "$bare" diff-tree --no-commit-id --name-only -r main)"
  [ "$(git -C "$bare" show main:Casks/intent-app.rb)" = "the cask" ] || fail "the cask's bytes are not what was pushed"
  [ "$(git -C "$bare" show main:Formula/intent.rb)" = "the formula" ] || fail "the formula's bytes are not what was pushed"
}
