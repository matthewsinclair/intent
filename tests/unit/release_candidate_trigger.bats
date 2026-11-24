#!/usr/bin/env bats
# A release's candidate starts the workflows a cut reads, and a tag starts none
# of them (ST0091 WP-04, AC-04.1; devbin's D64).
#
# ST0091 AT-04.1 cites this file for AC-04.1.
#
# THE CUT READS ONE WORKFLOW'S RUN, ON ITS CANDIDATE. devbin's release core
# pushes the release commit to its candidate (release.publish.candidate) and
# reads release.ci.workflow's run there, tests.yml's, before it tags anything.
# Its pre-flight asks `bin/devbin ci-state --fires` whether that push starts the
# workflow, and refuses the cut when it does not. The first test asks the same
# reader of HEAD, so a trigger that stops admitting the candidate goes red at
# the commit that breaks it rather than at the next cut.
#
# THAT ONE RUN CARRIES THE RUST LEG. tests.yml calls rust.yml for a candidate,
# and its rust job and test-summary's rust-leg rule each key on the candidate's
# ref. A prefix that drifted from the one the cut pushes would skip the rust leg
# AND excuse the skip, so the run would read green with a leg missing. The test
# "a candidate's run carries rust.yml's job" holds each of those keys to the
# prefix config names.
#
# THE CANDIDATE'S RUN HAS NO macOS LEG (hv decision 97). The cut's local macos
# leg on rhadamanth is its macOS proof. tests.yml's test-macos job,
# test-summary's macOS-leg rule and rust.yml's macOS matrix entry key on a run
# by hand alone. The test "the macOS leg runs by hand alone" holds each of
# those to that key, and holds test-summary's gate line to pass each leg's rule
# with that leg's result.

load "../lib/test_helper.bash"

WF="${INTENT_PROJECT_ROOT}/.github/workflows"

# The candidate's ref template, as `release check` prints it in its
# declaration: the one config names, declared or devbin's default. Asked at the
# project root, since the helper's setup() stands each test in a directory
# outside the project.
candidate_template() {
  (cd "${INTENT_PROJECT_ROOT}" && bin/devbin release check 2>/dev/null) |
    awk '$1 == "publish.candidate" { print $2; exit }'
}

# The keys directly under `on:` -> `push:` in workflow <file>, one a line.
push_trigger_keys() {
  awk '
    /^on:[[:space:]]*$/ { inon = 1; next }
    inon && /^[^[:space:]#]/ { inon = 0; inpush = 0 }
    inon && /^  push:[[:space:]]*$/ { inpush = 1; next }
    inon && /^  [^[:space:]#]/ { inpush = 0 }
    inpush && /^    [A-Za-z_-]+:/ { k = $1; sub(/:.*$/, "", k); print k }
  ' "$1"
}

# The `<key>:` line of job <job> in workflow <file>.
job_line() {
  awk -v job="$2" -v key="$3" '
    $0 ~ "^  " job ":[[:space:]]*$" { injob = 1; next }
    injob && /^  [^[:space:]#]/ { injob = 0 }
    injob && $0 ~ "^    " key ":" { print; exit }
  ' "$1"
}

@test "AT-04.1: a push of the cut's candidate starts release.ci.workflow as committed at HEAD" {
  local tmpl ref line rc=0
  tmpl="$(candidate_template)"
  echo "publish.candidate: ${tmpl:-(not read)}"
  [ -n "$tmpl" ]
  ref="${tmpl//\{tag\}/v0.0.0}"
  cd "${INTENT_PROJECT_ROOT}"
  line="$(bin/devbin ci-state --fires "$ref" "$(git rev-parse HEAD)" VERSION 2>/dev/null)" || rc=$?
  echo "ci-state --fires $ref: rc=$rc: $line"
  [ "$rc" -eq 0 ]
  [ "${line%%$'\t'*}" = "fires" ]
}

@test "AT-04.1: no tag push starts tests.yml, rust.yml or linux-release.yml" {
  local f keys
  for f in tests.yml rust.yml linux-release.yml; do
    keys="$(push_trigger_keys "$WF/$f")"
    echo "$f: push keys: ${keys//$'\n'/, }"
    grep -qx branches <<<"$keys"
    [ -z "$(grep -xE 'tags|tags-ignore' <<<"$keys" || true)" ]
  done
}

@test "AT-04.1: a candidate's run carries rust.yml's job, keyed on the prefix config names" {
  local tmpl key dispatch="github.event_name == 'workflow_dispatch'" uses rust_if rust_leg
  tmpl="$(candidate_template)"
  [ -n "$tmpl" ]
  key="startsWith(github.ref, 'refs/heads/${tmpl%%\{tag\}*}')"
  uses="$(job_line "$WF/tests.yml" rust uses)"
  rust_if="$(job_line "$WF/tests.yml" rust if)"
  rust_leg="$(grep -m1 '^ *rust_leg=' "$WF/tests.yml")"
  echo "key: $key"
  echo "uses: $uses"
  echo "rust if: $rust_if"
  echo "rust_leg: $rust_leg"
  grep -qx '  workflow_call:' "$WF/rust.yml"
  [ "$uses" = "    uses: ./.github/workflows/rust.yml" ]
  [ "$rust_if" = "    if: \${{ $key || $dispatch }}" ]
  [ "$rust_leg" = "          rust_leg=\"\${{ $key || $dispatch }}\"" ]
}

@test "AT-04.1: the macOS leg runs by hand alone, never on a candidate" {
  local dispatch="github.event_name == 'workflow_dispatch'" macos_if macos_leg os gate
  macos_if="$(job_line "$WF/tests.yml" test-macos if)"
  macos_leg="$(grep -m1 '^ *macos_leg=' "$WF/tests.yml")"
  os="$(grep -m1 '^ *os: ' "$WF/rust.yml")"
  gate="$(grep -m1 'needs.test-linux.result' "$WF/tests.yml")"
  echo "test-macos if: $macos_if"
  echo "macos_leg: $macos_leg"
  echo "os: $os"
  echo "gate: $gate"
  [ "$macos_if" = "    if: \${{ $dispatch }}" ]
  [ "$macos_leg" = "          macos_leg=\"\${{ $dispatch }}\"" ]
  [ "$os" = "        os: \${{ fromJSON($dispatch && '[\"macos-latest\", \"ubuntu-latest\"]' || '[\"ubuntu-latest\"]') }}" ]
  [ "$gate" = '          if [ "${{ needs.test-linux.result }}" == "success" ] && leg_ok "${{ needs.test-macos.result }}" "$macos_leg" && leg_ok "${{ needs.rust.result }}" "$rust_leg"; then' ]
}
