#!/usr/bin/env bash
# Test helper functions and setup for Intent tests

# Set up project-specific paths
# Use absolute paths to ensure tests work from any directory
INTENT_PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INTENT_BIN_DIR="${INTENT_PROJECT_ROOT}/bin"
INTENT_TEST_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures"
INTENT_TEMP_DIR="${INTENT_PROJECT_ROOT}/tests/tmp"

# The CLI under test: the v3 binary this repo builds, since hv's decision 6 at
# the 3.0.1 cut (the v2 shell implementation goes with the v2 trunk). Build it
# with `cargo build --release` under native/rust; set INTENT_BIN in the
# environment to run the estate against another binary without editing a
# single test.
INTENT_BIN="${INTENT_BIN:-${INTENT_PROJECT_ROOT}/native/rust/target/release/intent}"
export INTENT_BIN

# Export INTENT_HOME for tests
export INTENT_HOME="${INTENT_PROJECT_ROOT}"

# Create temporary test directory
setup_file() {
  mkdir -p "${INTENT_TEMP_DIR}"
}

# Clean up test directory after all tests in file
teardown_file() {
  if [ -d "${INTENT_TEMP_DIR}" ]; then
    rm -rf "${INTENT_TEMP_DIR}"
  fi
}

# Create a temporary test directory for each test
setup() {
  # Create temp dir outside of Intent project to test "outside project" scenarios
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
}

# Clean up temporary test directory after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Redirect HOME into a per-test sandbox so no test can write to the real
# ~/.claude or ~/.config (ST0042 F-TEST-1/F-TEST-9). Call after TEST_TEMP_DIR
# exists -- from an overridden setup(), or inline in a test that manages its
# own temp dir. Pair with teardown_fake_home in teardown() (or before the
# test's own cleanup).
setup_fake_home() {
  REAL_HOME="$HOME"
  export HOME="${TEST_TEMP_DIR}/fakehome"
  mkdir -p "$HOME/.claude/skills" "$HOME/.claude/agents" "$HOME/.config"
}

teardown_fake_home() {
  if [ -n "${REAL_HOME:-}" ]; then
    export HOME="$REAL_HOME"
    unset REAL_HOME
  fi
}

# Helper function to create a test Intent project
#
# The declared version follows the BINARY UNDER TEST, defaulting to the version
# the conformance harness actually points at. It said "2.10.0" for years, which
# was harmless while only v2 ever read it and stopped being harmless the moment
# v3 learned to detect an unmigrated project (AC-10.7): 2.10.0 is below the
# v2.19.0 migration floor, so the shared builder for 38 .bats files produced a
# project the v3 binary correctly REFUSES at fixture construction, before any
# assertion runs -- and the refusal lands in the same place in the output as
# "family not wired yet", on files already expected to be red. Found by ic,
# who measured it rather than inferring it from my claim that the fixtures
# were fine; they were fine in Rust and not here.
#
# An env var rather than a hardcoded version because the same files run against
# both binaries -- that is what parity means -- and a v2 baseline is entitled
# to a fixture declaring a v2 version.
#
# THE DEFAULT READS `VERSION`, THE SAME FILE `tests/run_tests.sh:61` READS, SO
# THE TWO AGREE BY CONSTRUCTION RATHER THAN BY BEING REMEMBERED AT A RELEASE.
# It used to be a literal `3.0.0`, and the runner defended itself while this
# did not: `run_tests.sh` has exported this from VERSION since `e474b419`, so
# only a DIRECT single-file `bats` run reached the literal -- the invocation
# our own guidance prefers. That built a v3 fixture, drove the v2 binary at it,
# and every test died on the version guard, which reads as "family not wired
# yet" on files already expected to be red.
#
# Re-derived 2026-08-21 before changing it: 41 files call create_test_project
# and NONE of them drive the v3 binary; the 6 files that touch it never call
# this. Nothing anywhere reads INTENT_FIXTURE_VERSION except the runner that
# sets it. So the literal was wrong in 41 of the 41 cases where it could fire.
#
# NOT `get_intent_version()` below, deliberately: that answers a different
# question (the INSTALLATION's version, INTENT_HOME first) and falls back to a
# hardcoded 2.2.1, which is a silent wrong answer -- the class this removes.
create_test_project() {
  local project_name="${1:-Test Project}"
  local dir="${2:-$TEST_TEMP_DIR/test-project}"
  local version="${INTENT_FIXTURE_VERSION:-$(cat "${INTENT_PROJECT_ROOT}/VERSION")}"

  mkdir -p "$dir/intent/.config"
  cat > "$dir/intent/.config/config.json" << EOF
{
  "intent_version": "$version",
  "project_name": "$project_name",
  "author": "test_user",
  "created_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF
  
  # Create standard directories
  mkdir -p "$dir/intent/st/COMPLETED"
  mkdir -p "$dir/intent/st/NOT-STARTED"
  mkdir -p "$dir/intent/st/CANCELLED"
  mkdir -p "$dir/intent/eng/tpd"
  mkdir -p "$dir/intent/ref"
  mkdir -p "$dir/intent/llm"

  echo "$dir"
}

# Write an acceptance.md that the close-gate treats as exempt, into an ST dir.
# ST0048's gate refuses an empty or missing contract; a fixture that exercises
# ST/WP mechanics (not the acceptance contract itself) declares the sanctioned
# escape so the close path is reached.
write_exempt_acceptance() {
  local st_dir="$1"
  cat > "$st_dir/acceptance.md" << 'ACCEPTANCE_EOF'
---
acceptance: exempt
---
# Acceptance (exempt -- fixture exercises ST/WP mechanics, not the gate)
ACCEPTANCE_EOF
}

# Helper function to run intent command
run_intent() {
  "$INTENT_BIN" "$@"
}

# Helper to check if command output contains expected text
assert_output_contains() {
  local expected="$1"
  if [[ "$output" != *"$expected"* ]]; then
    echo "Expected output to contain: $expected"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to check if command succeeded
assert_success() {
  if [ "$status" -ne 0 ]; then
    echo "Expected command to succeed, but it failed with status $status"
    echo "Output: $output"
    return 1
  fi
}

# Helper to check if command failed
assert_failure() {
  if [ "$status" -eq 0 ]; then
    echo "Expected command to fail, but it succeeded"
    echo "Output: $output"
    return 1
  fi
}

# Helper for test failures
fail() {
  echo "$1"
  return 1
}

# Helper to check if file exists
assert_file_exists() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "Expected file to exist: $file"
    return 1
  fi
}

# Helper to check if directory exists
assert_directory_exists() {
  local dir="$1"
  if [ ! -d "$dir" ]; then
    echo "Expected directory to exist: $dir"
    return 1
  fi
}

# Helper to check if file contains text
assert_file_contains() {
  local file="$1"
  local text="$2"
  if ! grep -qF "$text" "$file"; then
    echo "Expected file $file to contain: $text"
    echo "File contents:"
    cat "$file"
    return 1
  fi
}

# Helper to check if file does not exist
assert_file_not_exists() {
  local file="$1"
  if [ -f "$file" ]; then
    echo "Expected file to not exist: $file"
    return 1
  fi
}

# Helper to check if output does not contain text
refute_output_contains() {
  local text="$1"
  if [[ "$output" == *"$text"* ]]; then
    echo "Expected output to NOT contain: $text"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to check exact output match
assert_output() {
  local expected="$1"
  if [[ "$output" != "$expected" ]]; then
    echo "Expected output: $expected"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to get Intent version from VERSION file
get_intent_version() {
  # First try the VERSION file in the Intent installation
  if [ -f "${INTENT_HOME}/VERSION" ]; then
    cat "${INTENT_HOME}/VERSION"
  elif [ -f "${INTENT_PROJECT_ROOT}/VERSION" ]; then
    cat "${INTENT_PROJECT_ROOT}/VERSION"
  elif [ -f "${INTENT_PROJECT_ROOT}/intent/.config/config.json" ]; then
    # Fallback to config.json for compatibility
    jq -r '.version // .intent_version // "2.2.1"' "${INTENT_PROJECT_ROOT}/intent/.config/config.json"
  else
    echo "2.2.1"
  fi
}

# Load bats libraries if available
# Note: bats libraries can be installed globally or added to tests/lib/
# For now, we rely on the basic assert functions defined above
# ============================================================================
# ARMED TOOLS -- one home for "this arm needs a tool that may not be here"
# (issue 0512)
# ============================================================================
#
# THIS REPOSITORY HAD THREE BEHAVIOURS FOR ONE SITUATION and the worst was the
# default. `require_prettier()` in
# `native/rust/crates/intent-cli/tests/view_single_writer.rs` panics; an arm in
# `devbin_fmt_md.bats` skipped and reported `ok`; and the guard arms asserted
# `[ "$status" -eq 1 ]` and failed saying nothing but that, from which a reader
# cannot tell a broken guard from an absent tool. Three homes, disagreeing
# about whether a missing tool is fatal, invisible, or a test failure.
#
# `require_tool` is the bats side of the PANIC form. It cannot be the same CODE
# as the Rust one -- different language -- so the two are bound by VOCABULARY
# instead: both honour `INTENT_ALLOW_MISSING_<TOOL>`, so a contributor working
# without prettier sets `INTENT_ALLOW_MISSING_PRETTIER=1` once and both suites
# read it. That is the part that would silently diverge, so that is the part
# made identical.
#
# **THE WAIVER IS AN ENV VAR RATHER THAN A SILENT PROBE BECAUSE SOMEONE HAS TO
# HAVE DECIDED**, which is the Rust file's own reasoning and is not restated
# further here.

# Fail the calling arm unless `$1` is on PATH, naming the tool and what went
# unmeasured. `$2` is that subject, in the arm's own words.
#
# A bare `require_tool prettier "..."` line is enough to fail the arm: measured
# under Bats 1.14.0, a helper returning 1 fails the test and its output is
# shown. It is NOT `|| fail`, because the whole point is that the arm stops
# before asserting anything about an instrument that is not there.
require_tool() {
  local tool="$1" subject="$2" waiver
  command -v "$tool" >/dev/null 2>&1 && return 0

  waiver="INTENT_ALLOW_MISSING_$(printf '%s' "$tool" | tr '[:lower:]-' '[:upper:]_')"
  if [ -n "${!waiver:-}" ]; then
    echo "WAIVED: $tool is absent and $waiver is set, so ${subject} was NOT"
    echo "measured on this machine. This is a waiver, not a pass."
    return 0
  fi

  echo "$tool is not on PATH, so ${subject} cannot be measured."
  echo "This arm FAILS rather than skipping: a skip here is a green that means"
  echo "nothing, on an arm whose subject is an instrument's own verdict."
  echo "Install $tool, or set $waiver=1 to waive it deliberately."
  return 1
}

# The tools this repository ARMS, discovered rather than listed.
#
# Two surfaces declare arming and both are read, because a list written here
# would go stale the first time either moved:
#   * the rule library's `critic_tool:` frontmatter -- the critic fails CLOSED
#     when an armed rule's tool is missing;
#   * `staged-format-guard.sh`'s own `UNENFORCED <lang>(<tool>)` lines, which
#     are the guard naming the formatters it enforces with.
armed_tools() {
  {
    find "${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules" -name RULE.md \
      -exec sed -n 's/^critic_tool: *//p' {} +
    sed -n 's/.*UNENFORCED [a-z]*(\([a-z]*\)).*/\1/p' \
      "${INTENT_PROJECT_ROOT}/lib/templates/hooks/staged-format-guard.sh"
  } | sort -u
}

# Build a PATH in `$1` that resolves everything on the current PATH EXCEPT the
# tool named in `$2`, and verify it both ways before returning.
#
# MOVED HERE FROM `critic_arming_census.bats` (issue 0512), which is where it
# was written and where its reasoning was earned. Generalising it past one
# hardcoded tool name was cheaper than writing a second one, and a second one
# would have been the very shape this change exists to remove.
#
# (That sentence does not open a line with the linter's own name: a comment
# beginning `# shellcheck ...` is parsed as a DIRECTIVE, and SC1072/SC1073 fired
# on the first draft of it.)
#
# ITS ORIGINAL COMMENT, KEPT BECAUSE IT IS THE REASON THE FUNCTION LOOKS LIKE
# THIS. The previous form was a hardcoded list whose own comment named the
# assumption it rested on: `/opt/homebrew/bin` is where shellcheck lives *on
# this machine*. That is a macOS-with-Homebrew fact, and the list it left
# behind -- `/usr/bin:/bin:/usr/sbin:/sbin` -- is precisely where shellcheck
# lives on Linux, so on `ubuntu-latest` the absent-tool arms ran with the tool
# PRESENT and asserted an absence that never happened. It was green on exactly
# one machine: the one the constant was written for. Dropping whichever
# directory the tool lives in does not generalise either: on Linux `/bin` is a
# symlink to `/usr/bin`, and removing both takes `sed`, `grep` and `awk` with
# it -- and a critic that cannot run proves nothing about arming.
#
# **BOTH DIRECTIONS ARE CHECKED HERE RATHER THAN BY THE CALLER, BECAUSE EACH
# FAILS SILENTLY ON ITS OWN.** A farm that still resolves the tool turns an
# absent-tool arm into a second copy of the present-tool arm -- a test
# asserting nothing, reporting green. A farm that lost the coreutils makes the
# subject fail for a reason that has nothing to do with the tool. The first is
# the bug this replaces; the second is the bug the obvious fix introduces.
build_no_tool_path() {
  local farm="$1" absent="$2" dir entry base saved_ifs tool
  mkdir -p "$farm"
  saved_ifs="$IFS"
  IFS=:
  # shellcheck disable=SC2086 # deliberate splitting of PATH on IFS=:, which is
  # the only way to walk its entries in bash 3.2 without an array; quoting it
  # would make the whole PATH one positional and the loop below would look for
  # a single directory named "/usr/bin:/bin:...".
  set -- $PATH
  IFS="$saved_ifs"
  for dir in "$@"; do
    [ -d "$dir" ] || continue
    for entry in "$dir"/*; do
      [ -f "$entry" ] && [ -x "$entry" ] || continue
      base="${entry##*/}"
      [ "$base" = "$absent" ] && continue
      [ -e "$farm/$base" ] && continue
      ln -s "$entry" "$farm/$base" 2>/dev/null || true
    done
  done

  if ( PATH="$farm"; command -v "$absent" >/dev/null 2>&1 ); then
    printf 'the constructed PATH still resolves `%s`: %s\n' "$absent" "$farm" >&2
    return 1
  fi
  for tool in sed grep awk git; do
    [ "$tool" = "$absent" ] && continue
    if ! ( PATH="$farm"; command -v "$tool" >/dev/null 2>&1 ); then
      printf 'the constructed PATH lost `%s` -- the subject cannot run under it\n' "$tool" >&2
      return 1
    fi
  done
}

# ---------------------------------------------------------------------------
# THE OTHER TWO CLASSES OF SKIP, TAGGED SO THEY ARE TELLABLE APART (issue 0512)
# ---------------------------------------------------------------------------
#
# A CENSUS OF THE GREEN RUN 35743714571 FOUND ELEVEN SKIPS PER LEG AND THEY ARE
# NOT ONE PROBLEM. They are three, and every one of them printed `# skip` and
# read as `ok`, which is why the first framing of 0512 saw only the one class
# it had an instance of:
#
#   ARMED TOOL ABSENT   `require_tool` -- FAILS, names the tool. Not a skip at
#                       all, because the arm's subject is an instrument's own
#                       verdict and there is nothing to report.
#   NO WITNESS          the corpus holds no member the claim could be shown on,
#                       so the arm cannot fail whatever the code does. Vacuous,
#                       and the honest answers are to plant a witness or delete
#                       the arm -- a tag is the marker, not the fix.
#   OTHER SYSTEM        the arm describes a system this machine is not (a 3.2
#                       `/bin/bash` on a 5.x host; a fixture PATH the host
#                       cannot supply). A skip here can be correct.
#
# THE TAG IS THE WHOLE POINT. `# skip` on its own tells a reader nothing about
# which of the three they are looking at, so the one that is a defect hides
# behind the two that are not.

# The corpus holds nothing this arm could be witnessed on. `$1` says what is
# missing from the population, not what the arm wanted.
skip_no_witness() {
  skip "NO WITNESS: $1"
}

# This machine is not the system the arm describes. `$1` names the system.
skip_other_system() {
  skip "OTHER SYSTEM: $1"
}
