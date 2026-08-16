#!/usr/bin/env bats
# The four `int` rust gates reach the cargo workspace, and do not drift from CI.
#
# WHAT WENT WRONG, AND WHY NOTHING NOTICED FOR A DAY. `a1a949c` moved all native
# code to `native/rust/`, deleting the root `Cargo.toml`. Every catalogue-derived
# cargo line in devbin -- `test rust`, `check clippy`, `fmt rust`, and the
# `check format` builtin's rust arm -- runs in a subshell at PROJECT_ROOT, which
# is the right default (a declared `run:` is a project command). All four died
# with `could not find Cargo.toml`, and stayed dead until matts ran one by hand.
#
# **CI WAS GREEN THE WHOLE TIME.** `.github/workflows/rust.yml` sets
# `working-directory: native/rust` and was updated by the move; devbin was not.
# Same three checks, two homes, one of them followed the tree. That is the defect
# this file exists to catch -- not the missing manifest, which is a symptom, but
# **the two homes disagreeing**, which is the cause and which nothing observed.
#
# So the load-bearing test here is `every cargo check CI runs has a devbin twin`.
# It would have gone red at the moment `working-directory:` was added, a day
# before anyone ran the gate. The other tests are cheaper facts that hold the
# premises that one rests on.
#
# WHY NOT JUST PUT A Cargo.toml BACK AT THE ROOT. It is what the move deleted,
# and `bin/.devbin/cmd/prepush` now refuses a push that reintroduces a second
# workspace manifest outside `crates/`. Test 1 pins the same invariant here, so
# the two agree rather than one silently undoing the other.

load "../lib/test_helper.bash"

ROOT="${INTENT_PROJECT_ROOT}"
CONFIG="${ROOT}/bin/.devbin/config.yaml"
WORKFLOW="${ROOT}/.github/workflows/rust.yml"

# The cargo command lines devbin will actually run, one per line.
devbin_cargo_lines() {
  sed -n 's/^ *run: \(cargo .*\)$/\1/p' "$CONFIG"
}

# The cargo command lines CI runs. `rust.yml` is the only workflow that stands
# in the workspace, so it is the only one whose bare `cargo` lines are comparable.
ci_cargo_lines() {
  sed -n 's/^ *run: \(cargo .*\)$/\1/p' "$WORKFLOW"
}

# Strip the two compensators that exist ONLY because devbin does not stand in
# the workspace, so what remains is the check itself and can be compared.
#
#   --manifest-path native/rust/Cargo.toml   cargo's answer to "not here"
#   --all on a fmt line                      required because that manifest is
#                                            VIRTUAL: without it cargo reports
#                                            `Failed to find targets` and exits
#                                            1, which reads exactly like a
#                                            formatting finding
#
# Anything else that differs is real drift and must fail.
normalise() {
  sed -e 's| --manifest-path native/rust/Cargo\.toml||' \
    -e 's|^cargo fmt --all|cargo fmt|'
}

@test "there is no cargo workspace manifest at the project root" {
  # The premise of every --manifest-path below, and the invariant cmd/prepush
  # enforces at push time. If this fails, the tree moved back and the gates
  # need rereading rather than the test relaxing.
  [ ! -f "${ROOT}/Cargo.toml" ]
}

@test "the workspace manifest is where the gates say it is" {
  [ -f "${ROOT}/native/rust/Cargo.toml" ]
}

@test "every cargo line devbin runs names the workspace manifest" {
  local line found=0
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    found=$((found + 1))
    if [[ "$line" != *"--manifest-path native/rust/Cargo.toml"* ]]; then
      echo "devbin cargo line does not reach the workspace: $line" >&2
      return 1
    fi
  done < <(devbin_cargo_lines)
  # A config that declares NO cargo line is the state this whole file was
  # written about -- the gates falling back to the catalogue's root-relative
  # defaults. Passing on zero lines would be the exact false green.
  [ "$found" -ge 3 ]
}

@test "every fmt line devbin runs carries --all" {
  local line
  while IFS= read -r line; do
    case "$line" in
      cargo\ fmt*)
        if [[ "$line" != *"--all"* ]]; then
          echo "cargo fmt without --all on a virtual manifest exits 1 as 'Failed to find targets': $line" >&2
          return 1
        fi
        ;;
    esac
  done < <(devbin_cargo_lines)
}

@test "every cargo check CI runs has a devbin twin with the same flags" {
  local ci_line normalised twins
  twins="$(devbin_cargo_lines | normalise)"
  [ -n "$twins" ] || {
    echo "devbin declares no cargo lines at all -- the gates are on catalogue defaults" >&2
    return 1
  }
  while IFS= read -r ci_line; do
    [ -n "$ci_line" ] || continue
    normalised="$(printf '%s\n' "$ci_line" | normalise)"
    if ! printf '%s\n' "$twins" | grep -qxF "$normalised"; then
      echo "CI runs a check devbin does not, or runs it with different flags:" >&2
      echo "  CI:     $ci_line" >&2
      echo "  devbin: $twins" >&2
      return 1
    fi
  done < <(ci_cargo_lines)
}

@test "check format reaches the workspace" {
  # Deliberately asserts the INVOCATION, not the verdict. Four sessions share
  # this tree, so a peer's mid-edit unformatted file would make a verdict
  # assertion fail for a reason that has nothing to do with this gate. Both a
  # green and a red run pass here; only the broken-invocation signatures fail.
  #
  # `fmt rust` is NOT exercised: it MUTATES, and a test that reformats a peer's
  # in-flight files is worse than the defect it guards. Its command line is
  # covered structurally above.
  run bash -c "cd '$ROOT' && bin/int check format 2>&1"
  [[ "$output" != *"could not find"*"Cargo.toml"* ]]
  [[ "$output" != *"Failed to find targets"* ]]
}

@test "the set of disabled gates is pinned" {
  # `enabled: false` removes a gate from `int check all` AND from help, so a
  # disabled gate is invisible rather than merely off. That is the right lever --
  # `check toolchain` demands a `.tool-versions` this project deliberately does
  # not have, and a permanently-red aggregate trains everyone to ignore the one
  # command that would tell them something is wrong -- but it is a lever that
  # silently shrinks what "all" means.
  #
  # So the set is pinned rather than the mechanism forbidden: turning off a
  # second gate becomes a deliberate act with a test to update, instead of a
  # config line nobody reviews. Each entry here should carry its reason in
  # config.yaml beside the flag.
  local disabled
  # `[a-z0-9_-]` and not `[a-z-]`: the first mutation of this test disabled a gate
  # called `clippy2` and the extractor SILENTLY DID NOT SEE IT, so the test passed
  # over exactly the change it exists to catch. A needle that skips the names
  # nobody has invented yet is the allowlist-versus-needle-list lesson, one file
  # over, in a test written the same day.
  disabled="$(grep -B1 '^ *enabled: false' "$CONFIG" | sed -n 's/^ *\([a-z][a-z0-9_-]*\):$/\1/p' | sort | tr '\n' ' ')"
  [ "$disabled" = "toolchain " ]
}

@test "the declared language set is pinned" {
  # NOT tidiness. `commands.check.options.format.run` replaces devbin's
  # multi-language format builtin with ONE command line, because devbin refuses
  # to derive a check for a language whose `fmt` arm the config overrides. That
  # is equivalent while the languages are shell (no formatter) and rust -- and a
  # THIRD language's format check would then silently not happen. This is the
  # tripwire that makes adding one a decision rather than an accident.
  local langs
  langs="$(sed -n '/^  languages:/,/^[^ ]/p' "$CONFIG" | sed -n 's/^ *- \(.*\)$/\1/p' | sort | tr '\n' ' ')"
  [ "$langs" = "rust shell " ]
}
