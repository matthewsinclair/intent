#!/usr/bin/env bats
# Guards for the macOS release ASSET SET -- what ships, and what gets proven.
#
# `int macos` cannot be exercised end to end in a unit test: it signs with a
# Developer ID from the login Keychain, waits on a multi-minute notarisation round
# trip to Apple, and uploads to a public release. So the refusals are extracted as
# functions and driven directly, which is the only way they get exercised at all
# -- inline, their first real run would have been a release.
#
# WHAT THEY EXIST FOR. A v3 formula that installs only the binaries produces an
# install that cannot find itself. `intent` locates its own tree by walking up
# from its symlink-resolved location to the directory containing `lib/templates/`,
# and two shipped consumers exec out of it: `intent claude hook <name>` runs
# `lib/templates/.claude/scripts/<name>.sh`, and the pre-commit gate runs
# `lib/templates/hooks/<guard>.sh` at a path parsed back out of `intent info`.
# Measured against a reproduction of exactly what the formula builds, both fail
# and the gate's guard path comes back unresolvable -- so every consumer project
# would silently lose its session hooks and both whiteboard guards.
#
# The support tree therefore ships as a third artefact, which breaks an identity
# the pipeline used to rely on: "staged artefact" and "must be proven signed and
# notarised" stopped being the same set. These guards cover the two places that
# identity used to be assumed.

load "../lib/test_helper.bash"

# INTENT_MACOS_SCRIPT redirects every test at another copy, so these guards can be
# mutation-tested against a deliberately broken one without editing the shipped
# script. Unset -- every normal run -- this is the plain path.
MACOS="${INTENT_MACOS_SCRIPT:-${INTENT_HOME}/bin/.devbin/cmd/macos}"

# Source the classification helpers and the constants they read. The constants
# come OUT OF THE SCRIPT rather than being restated here: a test that spells
# BINARIES itself asserts against its own copy and goes green while the script
# says something else.
load_macos_fns() {
  local fns="${TEST_TEMP_DIR}/macosfns.sh"
  sed -n '/^BINARIES=/p;/^SUPPORT_ASSET=/p;/^SUPPORT_PATHS=/p' "$MACOS" >"$fns"
  sed -n '/^is_binary_artefact() {/,/^}/p;/^staged_artefacts() {/,/^}/p;/^unclassified_artefacts() {/,/^}/p;/^support_tree_drift() {/,/^}/p;/^provenance_blockers() {/,/^}/p' \
    "$MACOS" >>"$fns"
  # shellcheck disable=SC1090
  . "$fns"
}

# A staging directory holding whatever names it is given.
stage_with() {
  STAGE_DIR="${TEST_TEMP_DIR}/dist"
  rm -rf "$STAGE_DIR"
  mkdir -p "$STAGE_DIR"
  local f
  for f in "$@"; do printf 'x' >"$STAGE_DIR/$f"; done
}

# A source tree shaped like the part of the install that ships, plus a real
# tarball built from it the same way cmd_stage builds one.
build_support_fixture() {
  ROOT="${TEST_TEMP_DIR}/root"
  STAGE_DIR="${TEST_TEMP_DIR}/dist"
  rm -rf "$ROOT" "$STAGE_DIR"
  # THE FIXTURE'S TREES ARE DERIVED FROM $SUPPORT_PATHS, NEVER LISTED AGAIN
  # HERE. When the copy list grew from one tree to three on 2026-08-26 this
  # helper still mkdir'd `lib/templates` alone, `tar` failed with `Cannot stat`
  # on the other two, and four arms went red. The arms were right and the
  # fixture was the stale copy -- which is the same one-list-two-homes shape
  # the copy list itself had just been fixed for. Empty dirs are enough: the
  # drift check compares `find -type f`, so a directory with no files in it
  # contributes nothing to either side.
  #
  # AND THE SHAPE IS DERIVED TOO, NOT ASSUMED TO BE A DIRECTORY. The list may name
  # a FILE: `intent/plugins/claude/bin/intent_claude_cwi` is one, entered at file
  # level on 2026-08-29 because its directory holds six v2 scripts that must not
  # ship. `mkdir -p` on a file entry silently builds a DIRECTORY of that name --
  # which passes every arm here, since an empty directory contributes nothing to
  # `find -type f`, while meaning the drift arms never exercise a file entry at
  # all. That is the same shape as the defect above it, one turn quieter: the
  # fixture agrees with the list and stops agreeing with the tree. Shape is read
  # from the same tree the list is read from, so it cannot drift in either respect.
  local _p _src="${INTENT_HOME:-${MACOS%/bin/.devbin/cmd/macos}}"
  for _p in $SUPPORT_PATHS; do
    if [ -f "$_src/$_p" ]; then
      mkdir -p "$ROOT/$(dirname "$_p")"
      printf '#!/usr/bin/env bash\n' >"$ROOT/$_p"
    else
      mkdir -p "$ROOT/$_p"
    fi
  done
  mkdir -p "$ROOT/lib/templates/hooks" "$ROOT/lib/templates/.claude/scripts" "$STAGE_DIR"
  printf '#!/usr/bin/env bash\n' >"$ROOT/lib/templates/hooks/whiteboard-clock-guard.sh"
  printf '#!/usr/bin/env bash\n' >"$ROOT/lib/templates/hooks/pre-commit.sh"
  printf '#!/usr/bin/env bash\n' >"$ROOT/lib/templates/.claude/scripts/require-in-session.sh"
  tar -czf "$STAGE_DIR/$SUPPORT_ASSET" -C "$ROOT" --exclude '.DS_Store' $SUPPORT_PATHS
}

@test "int macos is syntactically valid" {
  run bash -n "$MACOS"
  assert_success
}

# --------------------------------------------------------------------
# Classification: every staged artefact is a decision
# --------------------------------------------------------------------

@test "is_binary_artefact is derived from BINARIES, not from a name pattern" {
  load_macos_fns
  run is_binary_artefact "intent-aarch64-apple-darwin" "aarch64-apple-darwin"
  assert_success
  run is_binary_artefact "intentd-aarch64-apple-darwin" "aarch64-apple-darwin"
  assert_success

  # The support asset is NOT a Mach-O, however it is spelled. This is the
  # distinction the old `*-$triple` glob could not make.
  run is_binary_artefact "intent-support.tar.gz" "aarch64-apple-darwin"
  assert_failure
  # A binary name carrying the wrong triple is not this host's artefact.
  run is_binary_artefact "intent-x86_64-apple-darwin" "aarch64-apple-darwin"
  assert_failure
}

@test "staged_artefacts lists the artefacts and never the claim about them" {
  load_macos_fns
  stage_with intent-aarch64-apple-darwin intentd-aarch64-apple-darwin \
    intent-support.tar.gz SHA256SUMS.txt

  run staged_artefacts
  assert_success
  assert_output_contains "intent-support.tar.gz"
  # SHA256SUMS.txt describes the others; hashing it into itself is meaningless.
  run bash -c "cd '$STAGE_DIR' && ls -1 | wc -l | tr -d ' '"
  assert_output "4"
  run bash -c "$(declare -f staged_artefacts); STAGE_DIR='$STAGE_DIR'; staged_artefacts | wc -l | tr -d ' '"
  assert_output "3"
}

@test "BASELINE: the shipping set classifies clean" {
  load_macos_fns
  stage_with intent-aarch64-apple-darwin intentd-aarch64-apple-darwin intent-support.tar.gz
  run unclassified_artefacts "aarch64-apple-darwin"
  assert_success
  assert_output ""
}

@test "an artefact that is neither a proven binary nor declared data is REFUSED" {
  # The control that closes the class. Without it, the next artefact added to the
  # staging directory either ships as unproven bytes under a published hash or is
  # silently left out of SHA256SUMS.txt -- and neither failure names itself.
  load_macos_fns
  stage_with intent-aarch64-apple-darwin intentd-aarch64-apple-darwin \
    intent-support.tar.gz intent-completions.zip

  run unclassified_artefacts "aarch64-apple-darwin"
  assert_output_contains "intent-completions.zip"
}

@test "the refusal names the artefact rather than only counting it" {
  load_macos_fns
  stage_with intent-aarch64-apple-darwin intentd-aarch64-apple-darwin \
    intent-support.tar.gz alpha.bin beta.bin

  run unclassified_artefacts "aarch64-apple-darwin"
  assert_output_contains "alpha.bin"
  assert_output_contains "beta.bin"
}

# --------------------------------------------------------------------
# The support tree matches its source, in both directions
# --------------------------------------------------------------------

@test "BASELINE: an archive built from the tree reports no drift" {
  load_macos_fns
  build_support_fixture
  run support_tree_drift
  assert_success
  assert_output ""
}

@test "a file in the tree and missing from the archive is REFUSED" {
  # This is the failure the whole change exists to prevent, one level in: an
  # archive that built cleanly and ships an install missing a guard.
  load_macos_fns
  build_support_fixture
  printf '#!/usr/bin/env bash\n' >"$ROOT/lib/templates/hooks/whiteboard-header-guard.sh"

  run support_tree_drift
  assert_output_contains "whiteboard-header-guard.sh"
  assert_output_contains "<"
}

@test "a file in the archive and missing from the tree is REFUSED" {
  # The other direction, which fails differently: a stale archive, or one reaching
  # somewhere it should not.
  load_macos_fns
  build_support_fixture
  rm "$ROOT/lib/templates/hooks/pre-commit.sh"

  run support_tree_drift
  assert_output_contains "pre-commit.sh"
  assert_output_contains ">"
}

@test ".DS_Store is excluded on both sides, so its presence is not drift" {
  # Excluded when the archive is built; comparing an exclusion against a
  # non-exclusion would report drift that is not drift, on any tree the Finder
  # has looked at.
  load_macos_fns
  build_support_fixture
  printf 'x' >"$ROOT/lib/templates/.DS_Store"

  run support_tree_drift
  assert_success
  assert_output ""
}

# --------------------------------------------------------------------
# Structural: the pipeline reads the classified set, not a glob
# --------------------------------------------------------------------

@test "the checksum step hashes the classified set rather than a triple glob" {
  # `shasum ./*-$triple` silently omits any artefact not spelled that way, and an
  # asset missing from SHA256SUMS.txt is one `formula` cannot name and `publish`
  # cannot round-trip. It ships unverified or not at all.
  run grep -F 'shasum -a 256 ./*-"$triple"' "$MACOS"
  assert_failure

  run grep -cE 'staged_artefacts \| tr .* xargs -0 shasum' "$MACOS"
  assert_success
  assert_output "1"
}

@test "publish uploads what was hashed, derived from the same one function" {
  # Restating the list here is how "what gets published" and "what has a published
  # hash" drift apart -- it would have shipped two of the three artefacts and left
  # the formula pointing at a resource that 404s.
  run grep -F 'for b in $BINARIES; do assets=' "$MACOS"
  assert_failure

  run grep -cE 'for a in \$\(staged_artefacts\); do assets=' "$MACOS"
  assert_success
  assert_output "1"
}

@test "the formula installs into libexec and symlinks bin" {
  # `lib` is a LINKED directory: brew symlinks keg lib subdirectories into the
  # shared prefix, so installing the support tree there would publish a directory
  # called `templates` into the global lib alongside every other formula's. The
  # binary must still sit beside the marker, so bin gets a symlink.
  run grep -F 'bin.install_symlink libexec/"bin/intent"' "$MACOS"
  assert_success

  # The pre-fix form put the binary straight into bin, where the walk finds no
  # marker above it.
  run grep -F 'bin.install Dir["intent-*"].first => "intent"' "$MACOS"
  assert_failure
}

@test "the support resource is declared in the formula with its own hash" {
  run grep -cE '^      resource "support" do' "$MACOS"
  assert_success
  assert_output "1"

  run grep -F 'sha256 "$sha_support"' "$MACOS"
  assert_success
}

# --------------------------------------------------------------------
# Provenance: the bytes must come from the tag they are published under
# --------------------------------------------------------------------
#
# EVERY OTHER CHECK IN THIS PIPELINE ASKS WHETHER THE BYTES AGREE WITH EACH
# OTHER. `publish` re-downloads what it uploaded and hashes THAT; `formula` reads
# the sums file; `checksum` refuses an unclassified artefact. All of it is
# internal consistency -- and a set of bytes built from a peer's uncommitted work
# is perfectly self-consistent. Nothing was asking whether these are the bytes the
# tag names.
#
# Two ways they might not be, both reachable today rather than theoretical.
# `stage` COPIES out of native/rust/target/release instead of building, and that
# directory is shared mutable state in a clone several sessions write to. And the
# support tarball is archived with `tar -C "$ROOT"` from the WORKING TREE, so
# uncommitted shell ships verbatim -- that one arrived with the support asset
# itself and needs no peer and no stale cache to fire.

# A checkout shaped like the part of the tree that ships, plus one file outside it.
git_fixture() {
  PROV_ROOT="${TEST_TEMP_DIR}/prov"
  rm -rf "$PROV_ROOT"
  mkdir -p "$PROV_ROOT/lib/templates/hooks" "$PROV_ROOT/src"
  printf '#!/usr/bin/env bash\n' >"$PROV_ROOT/lib/templates/hooks/guard.sh"
  printf 'fn main() {}\n' >"$PROV_ROOT/src/thing.rs"
  git -C "$PROV_ROOT" init -q
  git -C "$PROV_ROOT" config user.email t@t
  git -C "$PROV_ROOT" config user.name t
  git -C "$PROV_ROOT" add -A
  git -C "$PROV_ROOT" commit -qm init
}

@test "BASELINE: a clean checkout has NO provenance blockers" {
  # The negative control, and it is the one that makes every refusal below mean
  # something. A blocker function that fires on everything reads exactly like a
  # blocker function that works -- and it would refuse every release forever,
  # which is the direction that gets a guard deleted rather than fixed.
  load_macos_fns
  git_fixture

  run provenance_blockers "$PROV_ROOT"
  assert_success
  assert_output ""
}

@test "a dirty SUPPORT TREE is refused, and named, because it ships verbatim" {
  # The certain one: these paths are archived out of the working tree, so dirt
  # here is not a risk of shipping uncommitted bytes, it IS shipping them.
  load_macos_fns
  git_fixture
  printf '# edited but never committed\n' >>"$PROV_ROOT/lib/templates/hooks/guard.sh"

  run provenance_blockers "$PROV_ROOT"
  assert_output_contains "archived VERBATIM"
  assert_output_contains "guard.sh"
}

@test "a dirty tree OUTSIDE the support paths is still refused, for the weaker reason" {
  # A source edit does not ship through the tarball, but it can be compiled into
  # the binaries `stage` copies. Different evidence, different sentence -- one
  # message for two causes is how a refusal gets skimmed.
  load_macos_fns
  git_fixture
  printf 'fn extra() {}\n' >>"$PROV_ROOT/src/thing.rs"

  run provenance_blockers "$PROV_ROOT"
  assert_output_contains "may hold bytes that match no commit"
  # It must NOT claim the support tree would ship, because it would not.
  refute_output_contains "archived VERBATIM"
}

@test "a directory that is not a checkout at all is refused" {
  load_macos_fns
  mkdir -p "${TEST_TEMP_DIR}/notgit"

  run provenance_blockers "${TEST_TEMP_DIR}/notgit"
  assert_output_contains "not a git checkout"
}

@test "publish refuses on the provenance record, not merely on the tag existing" {
  # `publish` already established that a tag EXISTS and then published bytes with
  # no evidence they came from it. Three refusals, because the record can be
  # absent, present-and-untraceable, or present-and-naming-another-commit, and
  # they send the reader somewhere different each time.
  run grep -cF 'no provenance record at $PROVENANCE_FILE' "$MACOS"
  assert_success
  assert_output "1"

  # THE FIELD IS NAMED FOR ITS OWN SUBJECT, and that is what this arm asserts
  # rather than an incidental spelling. It used to be `traceable` -- a word about
  # the ARTEFACTS -- while the thing that writes it is `git status`. Driven on
  # real repositories, three cases: with ONE unrelated file dirty and both
  # binaries built at HEAD, the refusal beneath this field claimed the artefacts
  # could not name their commit, which the artefact check further down
  # contradicts, and its remedy sent the operator to rebuild binaries that were
  # never the problem.
  run grep -cF '[ "$prov_clean" = "yes" ] ||' "$MACOS"
  assert_success
  assert_output "1"

  # The refusal states the subject it actually measured.
  run grep -cF 'die "the working tree was not clean when these artefacts were staged' "$MACOS"
  assert_success
  assert_output "1"

  # AND THE REGRESSION GUARD, which is the half that would have caught the defect
  # rather than described it: this refusal must claim NOTHING about what the bytes
  # can name. That is a different question with its own check, on the bytes.
  #
  # Matched on `die "` and not on the sentence alone, so a comment ABOUT the old
  # wording is not mistaken for the old wording being emitted -- mention is not
  # invocation, and this file's subject now discusses its own history.
  run grep -cF 'die "the staged artefacts cannot name' "$MACOS"
  assert_output "0"

  # Compared against the TAG's commit, never HEAD: publishing may legitimately
  # happen after the branch has moved on.
  run grep -cE 'tag_commit="\$\(git -C "\$ROOT" rev-list -n 1 "\$tag"' "$MACOS"
  assert_success
  assert_output "1"
  run grep -cF '[ "$prov_commit" = "$tag_commit" ] ||' "$MACOS"
  assert_success
  assert_output "1"
}

@test "the provenance record lives OUTSIDE the staging directory" {
  # Inside it, `unclassified_artefacts` would refuse it -- correctly, since it is
  # not something that ships -- and `stage`'s own `rm -rf` would give it the same
  # lifetime as the thing it describes.
  run grep -cE '^PROVENANCE_FILE=.*/dist-provenance\.txt"$' "$MACOS"
  assert_success
  assert_output "1"

  load_macos_fns
  stage_with intent-aarch64-apple-darwin intentd-aarch64-apple-darwin \
    intent-support.tar.gz dist-provenance.txt
  run unclassified_artefacts "aarch64-apple-darwin"
  assert_output_contains "dist-provenance.txt"
}

@test "the formula's install block is agnostic about what the archive carries" {
  # Rooted at the install root, so a shipped set that grows -- `intent critic` and
  # `intent claude rules` are unimplemented today and will need the rule library,
  # which lives outside the marker directory -- is a content change rather than a
  # formula change.
  run grep -F 'libexec.install Dir["*"]' "$MACOS"
  assert_success
}
