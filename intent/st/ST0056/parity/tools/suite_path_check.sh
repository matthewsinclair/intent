#!/bin/bash
# suite_path_check.sh -- does every test module a suite.rs registers exist IN THE COMMIT?
#
# Issue 0348. Each crate compiles its integration tests as ONE binary: `suite.rs`
# names every test file with `#[path = "x.rs"] mod x;`. A commit that registers a
# path whose file is not in the commit builds on the machine that has the file and
# nowhere else -- d4d7aec2b carried such a registration, and HEAD could not build
# its test target until 5746e02e.
#
# THE INDEX IS WHAT IS READ, NOT THE WORKING TREE. The working tree of the node
# that wrote the registration HAS the file; that is the whole defect. Both the
# `suite.rs` being committed and the files it names are read from the index, so
# the question answered is the one the commit poses.
#
# EXIT CODES: 0 every registered path is in the index; 1 a registered path is not
# (each one named); 2 the tool could not read a verdict.
#
# THE CONTROLS RUN FIRST, IN A SCRATCH REPOSITORY, AND A CONTROL THAT DOES NOT
# FIRE REFUSES THE RUN at 2: a check whose red arm cannot go red would pass every
# commit for the wrong reason.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
G="env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE -u GIT_OBJECT_DIRECTORY -u GIT_COMMON_DIR git"

# missing_registrations <repo> -- prints `<suite.rs>: <path>` for every #[path]
# registration whose file the index does not hold. Reads only the index.
missing_registrations() {
  local repo="$1" suite dir rel line
  while IFS= read -r suite; do
    [ -n "$suite" ] || continue
    dir="${suite%/suite.rs}"
    while IFS= read -r line; do
      case "$line" in
        *'#[path = "'*'"]'*)
          rel="${line#*#\[path = \"}"
          rel="${rel%%\"*}"
          if ! $G -C "$repo" cat-file -e ":$dir/$rel" 2>/dev/null; then
            printf '%s: %s\n' "$suite" "$rel"
          fi
          ;;
      esac
    done < <($G -C "$repo" show ":$suite" 2>/dev/null)
  done < <($G -C "$repo" ls-files -- 'native/rust/crates/*/tests/suite.rs')
}

# ---- controls ---------------------------------------------------------------
TMP="$(mktemp -d)" || { echo "suite-path-check: cannot make a scratch directory" >&2; exit 2; }
trap 'rm -rf "$TMP"' EXIT
fixture() {
  local d="$1"
  mkdir -p "$d/native/rust/crates/thing/tests"
  $G -C "$d" init -q 2>/dev/null || return 1
  printf '#[path = "present.rs"]\nmod present;\n' > "$d/native/rust/crates/thing/tests/suite.rs"
  printf '#[test]\nfn t() {}\n' > "$d/native/rust/crates/thing/tests/present.rs"
  $G -C "$d" add -A >/dev/null 2>&1 || return 1
}
fixture "$TMP/green" || { echo "suite-path-check: the green control could not build its fixture" >&2; exit 2; }
if [ -n "$(missing_registrations "$TMP/green")" ]; then
  echo "suite-path-check: CONTROL FAILED -- a suite whose every registration is staged was reported" >&2
  exit 2
fi
fixture "$TMP/red" || { echo "suite-path-check: the red control could not build its fixture" >&2; exit 2; }
printf '#[path = "present.rs"]\nmod present;\n#[path = "absent.rs"]\nmod absent;\n' > "$TMP/red/native/rust/crates/thing/tests/suite.rs"
# The file exists on disk and is NOT staged: the exact shape of the defect.
printf '#[test]\nfn t() {}\n' > "$TMP/red/native/rust/crates/thing/tests/absent.rs"
$G -C "$TMP/red" add native/rust/crates/thing/tests/suite.rs >/dev/null 2>&1
case "$(missing_registrations "$TMP/red")" in
  *"suite.rs: absent.rs"*) ;;
  *)
    echo "suite-path-check: CONTROL FAILED -- a registration whose file is on disk but not staged went unreported" >&2
    exit 2
    ;;
esac

# ---- the verdict ------------------------------------------------------------
missing="$(missing_registrations "$ROOT")"
if [ -n "$missing" ]; then
  echo "suite-path-check: a suite.rs registers a test module this commit does not carry:" >&2
  printf '%s\n' "$missing" | sed 's/^/  /' >&2
  echo "  remedy: stage the named file with the registration, or drop the registration until the file lands" >&2
  exit 1
fi
echo "suite-path-check: ok -- every #[path] a staged suite.rs registers is in the commit (both controls fired)"
exit 0
