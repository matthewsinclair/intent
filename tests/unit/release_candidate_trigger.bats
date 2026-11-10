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
# THAT ONE RUN CARRIES EVERY LEG. tests.yml calls rust.yml for a candidate, and
# its test-macos job, its rust job, test-summary's every-leg rule and rust.yml's
# macOS matrix entry each key on the candidate's ref. A prefix that drifted
# from the one the cut pushes would skip the macOS leg AND excuse the skip, so
# the run would read green with a leg missing. The third test holds each of
# those keys to the prefix config names.

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

# The `if:` line of job <job> in workflow <file>.
job_if() {
  awk -v job="$2" '
    $0 ~ "^  " job ":[[:space:]]*$" { injob = 1; next }
    injob && /^  [^[:space:]#]/ { injob = 0 }
    injob && /^    if:/ { print; exit }
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

@test "AT-04.1: a candidate's run carries the macOS leg and rust.yml's job, each keyed on the prefix config names" {
  local tmpl key site
  tmpl="$(candidate_template)"
  [ -n "$tmpl" ]
  key="startsWith(github.ref, 'refs/heads/${tmpl%%\{tag\}*}')"
  echo "key: $key"
  grep -qx '    uses: ./.github/workflows/rust.yml' "$WF/tests.yml"
  grep -qx '  workflow_call:' "$WF/rust.yml"
  for site in "$(job_if "$WF/tests.yml" test-macos)" "$(job_if "$WF/tests.yml" rust)" \
    "$(grep -m1 '^ *every_leg=' "$WF/tests.yml")" "$(grep -m1 '^ *os: ' "$WF/rust.yml")"; do
    echo "site: $site"
    grep -qF -- "$key" <<<"$site"
  done
}
