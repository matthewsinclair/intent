#!/usr/bin/env bats
# Guards for two of the menubar app's dev verbs in `int macos`. Each arm runs the
# script's own functions, extracted from it, in a bash carrying the script's
# shell options, with the tools they call stubbed, so nothing here needs Xcode
# or rsvg-convert.
#
# - app-icons writes every logo derivative, the menu masthead's copy of the
#   512 px icon included (issue 0620). Nothing wrote the masthead copy, so a new
#   logo would refresh the app icon and leave the masthead behind.
# - app-test's summary is read from the run's result bundle (issue 0621).
#   xcodebuild's "Executed N tests" line counts XCTest alone, so a Swift Testing
#   suite's results never reached the summary a reader judges a run by.

load "../lib/test_helper.bash"

# INTENT_MACOS_SCRIPT redirects every test at another copy, as in
# macos_release_assets.bats, so the arms can be shown red against the script as
# it was before the fix.
MACOS="${INTENT_MACOS_SCRIPT:-${INTENT_HOME}/bin/.devbin/cmd/macos}"

# Where the app bundles the masthead logo from, relative to the project root.
MASTHEAD_LOGO="native/macos/Intent/Intent/Masthead/intent-logo.png"

@test "invariant: app-icons writes the masthead logo as a byte copy of the 512 px icon (0620)" {
  [ -f "$INTENT_HOME/$MASTHEAD_LOGO" ] || fail "the app no longer bundles $MASTHEAD_LOGO; point this arm at where it moved"
  local root="$TEST_TEMP_DIR/root" stubs="$TEST_TEMP_DIR/stubs" fns="$TEST_TEMP_DIR/icons.sh"
  mkdir -p "$root/docs/design" "$stubs"
  printf '<svg/>\n' >"$root/docs/design/intent-logo.svg"
  # rsvg-convert -w <px> -h <px> <svg> -o <out> writes a different file for each
  # size, so a copy of the wrong size fails the comparison below.
  printf '#!/bin/sh\nprintf "png %%s\\n" "$2" >"$7"\n' >"$stubs/rsvg-convert"
  chmod +x "$stubs/rsvg-convert"
  {
    printf 'ROOT=%q\n' "$root"
    sed -n '/^APP_DIR=/p;/^APP_ASSETS=/p;/^APP_MASTHEAD_LOGO=/p;/^LOGO_SVG=/p;/^app_png() {/p' "$MACOS"
    sed -n '/^cmd_app_icons() {/,/^}/p' "$MACOS"
  } >"$fns"
  # The two Contents.json writers are stubbed: each heredoc closes its JSON with
  # a bare "}" that would end a sed range early, and neither is under test.
  # shellcheck disable=SC2016 # the inner bash expands these, not this one
  run "$BASH" -c 'set -euo pipefail
    shopt -s inherit_errexit
    die() { printf "%s\n" "$*" >&2; exit 1; }
    app_appicon_contents() { printf "{}\n"; }
    app_menubar_contents() { printf "{}\n"; }
    PATH="$2:$PATH"
    . "$1"
    cmd_app_icons' _ "$fns" "$stubs"
  assert_success
  assert_file_exists "$root/$MASTHEAD_LOGO"
  cmp -s "$root/native/macos/Intent/Intent/Assets.xcassets/AppIcon.appiconset/icon_512.png" "$root/$MASTHEAD_LOGO" \
    || fail "the masthead logo is not the 512 px icon's bytes: [$(cat "$root/$MASTHEAD_LOGO")]"
}

# run_app_test_summary <bundle> -- app_test_summary, extracted from the script
# and run against <bundle>, with xcrun stubbed: `xcrun xcresulttool get
# test-results <summary|tests> --path <bundle> --compact` prints the bundle's
# <summary|tests>.json, which the arms write in the shape the tool's own
# --schema gives, and fails as the tool does when there is none.
run_app_test_summary() {
  local stubs="$TEST_TEMP_DIR/stubs" fns="$TEST_TEMP_DIR/summary.sh"
  mkdir -p "$stubs"
  printf '#!/bin/sh\ncat "$6/$4.json"\n' >"$stubs/xcrun"
  chmod +x "$stubs/xcrun"
  sed -n '/^app_test_summary() {/,/^}/p' "$MACOS" >"$fns"
  # shellcheck disable=SC2016 # the inner bash expands these, not this one
  run "$BASH" -c 'set -euo pipefail
    shopt -s inherit_errexit
    PATH="$2:$PATH"
    . "$1"
    app_test_summary "$3"' _ "$fns" "$stubs" "$1"
}

# write_bundle <dir> <summary json> <tests json>
write_bundle() {
  mkdir -p "$1"
  printf '%s\n' "$2" >"$1/summary.json"
  printf '%s\n' "$3" >"$1/tests.json"
}

@test "invariant: app-test's summary counts a Swift Testing suite and names each suite's verdict (0621)" {
  local bundle="$TEST_TEMP_DIR/green.xcresult"
  write_bundle "$bundle" \
    '{"title":"Test - Intent","environmentDescription":"Intent, My Mac","topInsights":[],"result":"Passed","totalTestCount":5,"passedTests":5,"failedTests":0,"skippedTests":0,"expectedFailures":0,"statistics":[],"devicesAndConfigurations":[],"testFailures":[]}' \
    '{"testPlanConfigurations":[],"devices":[],"testNodes":[{"nodeType":"Test Plan","name":"Intent","result":"Passed","children":[{"nodeType":"Unit test bundle","name":"IntentTests","result":"Passed","children":[{"nodeType":"Test Suite","name":"HealthTests","result":"Passed","children":[{"nodeType":"Test Case","name":"testGreen()","result":"Passed"},{"nodeType":"Test Case","name":"testRed()","result":"Passed"}]},{"nodeType":"Test Suite","name":"Masthead","result":"Passed","children":[{"nodeType":"Test Case","name":"showsVersion()","result":"Passed"},{"nodeType":"Test Case","name":"showsCommit()","result":"Passed"},{"nodeType":"Test Case","name":"showsKind()","result":"Passed"}]}]}]}]}'
  run_app_test_summary "$bundle"
  assert_success
  assert_output_contains "Passed: 5 tests, 5 passed, 0 failed, 0 skipped, 0 expected failures"
  assert_output_contains "Passed  HealthTests (2)"
  assert_output_contains "Passed  Masthead (3)"
  refute_output_contains "FAILED"
}

@test "invariant: app-test's summary names a failed Swift Testing test with its message (0621)" {
  local bundle="$TEST_TEMP_DIR/red.xcresult"
  write_bundle "$bundle" \
    '{"title":"Test - Intent","environmentDescription":"Intent, My Mac","topInsights":[],"result":"Failed","totalTestCount":5,"passedTests":4,"failedTests":1,"skippedTests":0,"expectedFailures":0,"statistics":[],"devicesAndConfigurations":[],"testFailures":[{"testName":"showsKind()","targetName":"IntentTests","failureText":"Expectation failed: kind == \"release\"","testIdentifier":1,"testIdentifierString":"Masthead/showsKind()"}]}' \
    '{"testPlanConfigurations":[],"devices":[],"testNodes":[{"nodeType":"Test Plan","name":"Intent","result":"Failed","children":[{"nodeType":"Unit test bundle","name":"IntentTests","result":"Failed","children":[{"nodeType":"Test Suite","name":"HealthTests","result":"Passed","children":[{"nodeType":"Test Case","name":"testGreen()","result":"Passed"},{"nodeType":"Test Case","name":"testRed()","result":"Passed"}]},{"nodeType":"Test Suite","name":"Masthead","result":"Failed","children":[{"nodeType":"Test Case","name":"showsVersion()","result":"Passed"},{"nodeType":"Test Case","name":"showsCommit()","result":"Passed"},{"nodeType":"Test Case","name":"showsKind()","result":"Failed"}]}]}]}]}'
  run_app_test_summary "$bundle"
  assert_success
  assert_output_contains "Failed: 5 tests, 4 passed, 1 failed, 0 skipped, 0 expected failures"
  assert_output_contains "Failed  Masthead (3)"
  assert_output_contains 'FAILED  Masthead/showsKind(): Expectation failed: kind == "release"'
}

@test "invariant: app-test's summary says so when the bundle holds no test results (0621)" {
  local bundle="$TEST_TEMP_DIR/empty.xcresult"
  mkdir -p "$bundle"
  run_app_test_summary "$bundle"
  assert_failure
  assert_output_contains "no test results in $bundle"
}
