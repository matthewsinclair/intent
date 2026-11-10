#!/usr/bin/env bats
# The cut's stamp helpers in bin/.devbin/cmd/shared/releasestamp.lib.
#
# ST0091 AT-01.2 cites this file for AC-01.1.
#
# What the stamp's steps trust and cannot check for themselves: the lock's
# non-member check, which `stamp lock` refuses with; `footer_only`, which
# `stamp views` folds with; and the one test of membership in the sidecar list. The
# lock and footer arms moved here from release_sidecars.bats when the release
# driver retired. They source the library whole, which runs nothing, where
# they used to lift each function out of the driver with sed.

load "../lib/test_helper.bash"

# INTENT_RELEASESTAMP_LIB points every arm at another copy of the library, so
# the arms can be driven against a broken copy without editing the shipped one.
RELEASESTAMP="${INTENT_RELEASESTAMP_LIB:-${INTENT_HOME}/bin/.devbin/cmd/shared/releasestamp.lib}"

# A function this file tests that the library does not define must fail the
# arm: every refusal arm below would otherwise pass on a library without it.
load_lib() {
  # shellcheck disable=SC1090
  . "$RELEASESTAMP"
  declare -F lock_packages lock_members lock_nonmember_changes footer_only release_is_sidecar >/dev/null ||
    fail "$RELEASESTAMP does not define every function these arms test"
}

# A minimal but structurally faithful lockfile: two path packages (no `source`)
# and one registry package (with `source`), which is exactly the distinction
# lock_members reads.
write_lock() { # write_lock PATH MEMBER_VERSION DEP_VERSION [EXTRA]
  cat >"$1" <<EOF
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
  if [ -n "${4:-}" ]; then printf '%s\n' "$4" >>"$1"; fi
  return 0
}

@test "AT-01.2: lock_packages reads a name and version out of every package block" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/a.lock" "3.0.0" "1.0.229"
  run lock_packages "${TEST_TEMP_DIR}/a.lock"
  assert_success
  assert_output "$(printf 'intent-cli\t3.0.0\nintentsvcs\t3.0.0\nserde\t1.0.229')"
}

@test "AT-01.2: lock_members is exactly the packages carrying no source line" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/a.lock" "3.0.0" "1.0.229"
  run lock_members "${TEST_TEMP_DIR}/a.lock"
  assert_success
  assert_output "$(printf 'intent-cli\nintentsvcs')"
}

@test "AT-01.2: a member-only version bump is accepted" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock" "3.0.0" "1.0.229"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output ""
}

# THE REGRESSION the check exists for: a lone dependency version bump is
# identical in shape to a workspace member's.
@test "AT-01.2: a dependency version bump riding alongside a member bump is REFUSED by name" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock" "3.0.0" "9.9.9"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "serde"
}

@test "AT-01.2: a dependency bump with no member change at all is REFUSED by name" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock" "3.0.0-dev" "9.9.9"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "serde"
}

@test "AT-01.2: a newly resolved package is REFUSED by name" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  write_lock "${TEST_TEMP_DIR}/after.lock" "3.0.0" "1.0.229" \
    '
[[package]]
name = "brand-new-crate"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"'
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/after.lock"
  assert_success
  assert_output "brand-new-crate"
}

# An empty parse yields an empty difference, which reads identically to
# "nothing changed". Without this the check would accept anything, confidently.
@test "AT-01.2: an unreadable lockfile fails rather than reporting no offenders" {
  load_lib
  write_lock "${TEST_TEMP_DIR}/before.lock" "3.0.0-dev" "1.0.229"
  : >"${TEST_TEMP_DIR}/empty.lock"
  run lock_nonmember_changes "${TEST_TEMP_DIR}/before.lock" "${TEST_TEMP_DIR}/empty.lock"
  assert_failure
  assert_output ""
}

# --------------------------------------------------------------------
# footer_only: the view classifier the stamp's fold trusts (issue 0464)
# --------------------------------------------------------------------
#
# Each arm diffs a real file in a scratch repository, so the classifier reads
# what `git diff -U0` actually prints, file headers included, rather than a
# hand-typed approximation of it.

# view_diff <before> <after> -- print `git diff -U0` of one view going from
# <before> to <after>, in a scratch repository.
view_diff() {
  local repo="${TEST_TEMP_DIR}/views"
  rm -rf "$repo" && mkdir -p "$repo" && git -C "$repo" init -q
  printf '%s' "$1" >"$repo/info.md"
  git -C "$repo" add info.md && git -C "$repo" -c user.name=t -c user.email=t@t commit -q -m base
  printf '%s' "$2" >"$repo/info.md"
  git -C "$repo" diff -U0 -- info.md
}

@test "AT-01.2: footer_only: a view whose only change is its footer is footer-only" {
  load_lib
  run footer_only <<<"$(view_diff \
    $'# T\n\n- a\n\n_Generated by Intent v3.0.3 from x._\n' \
    $'# T\n\n- a\n\n_Generated by Intent v3.1.0 from x._\n')"
  assert_success
}

@test "AT-01.2: footer_only: the major-only footer, and the move onto it, are footer-only" {
  load_lib
  run footer_only <<<"$(view_diff \
    $'# T\n\n- a\n\n_Generated by Intent v3.2.1 from x._\n' \
    $'# T\n\n- a\n\n_Generated by Intent v3 from x._\n')"
  assert_success
  run footer_only <<<"$(view_diff \
    $'# T\n\n- a\n\n_Generated by Intent v3 from x._\n' \
    $'# T\n\n- a\n\n_Generated by Intent v4 from x._\n')"
  assert_success
}

@test "AT-01.2: footer_only: a changed markdown bullet is NOT footer-only (0464)" {
  load_lib
  diff_out="$(view_diff \
    $'# T\n\n- a\n\n_Generated by Intent v3.0.3 from x._\n' \
    $'# T\n\n- b\n\n_Generated by Intent v3.1.0 from x._\n')"
  run footer_only <<<"$diff_out"
  assert_failure
  # The control: the old selector reads the same diff as footer-only, which is
  # the defect. It drops `-- a` and `+- b` and keeps only the footer lines.
  run bash -c "grep -E '^[-+][^-+]' | grep -v -q -E '^[-+]_Generated by Intent v[0-9]+\.[0-9]+\.[0-9]+ '" <<<"$diff_out"
  assert_failure
}

@test "AT-01.2: footer_only: an added blank line is NOT footer-only" {
  load_lib
  run footer_only <<<"$(view_diff \
    $'# T\n\n- a\n\n_Generated by Intent v3.0.3 from x._\n' \
    $'# T\n\n\n- a\n\n_Generated by Intent v3.1.0 from x._\n')"
  assert_failure
}

@test "AT-01.2: footer_only: no change at all is NOT footer-only" {
  load_lib
  run footer_only <<<""
  assert_failure
}

# The sidecar list holds files and one directory entry, `schema`, whose tree is
# the release's too; a path that merely starts with an entry's letters is not.
@test "AT-01.2: release_is_sidecar: a listed file and a path under a listed directory are the release's, and nothing else is" {
  load_lib
  release_is_sidecar VERSION || fail "VERSION is in SIDECAR_FILES and was not read as the release's"
  release_is_sidecar schema/thread.schema.json || fail "a face under schema/ was not read as the release's"
  run release_is_sidecar schemata/x.json
  assert_failure
  run release_is_sidecar intent/st/ST0001/info.md
  assert_failure
}
