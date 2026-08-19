#!/usr/bin/env bats
# The shipped hooks must run under the bash macOS actually has: 3.2.57.
#
# WHY THIS EXISTS. cc reached for `mapfile` while building
# `lib/templates/hooks/canon-ignore-guard.sh` and caught it themselves. `env
# bash` on a developer machine here resolves to homebrew 5.3, so a bash-4
# construct passes every local test and fails only on a default shell -- and
# `lib/templates/` is the one tree every fleet project inherits verbatim on its
# next `intent upgrade`.
#
# **THE TWO CLASSES FAIL IN OPPOSITE WAYS AND ONLY ONE OF THEM IS SAFE.**
# Measured against /bin/bash 3.2.57(1)-release on this machine:
#
#   bash-4 SYNTAX     `echo "${v^^}"`
#     -> `bad substitution`, **rc=1**. Loud. Fails at the point of use.
#
#   bash-4 BUILTIN    `mapfile -t a < f` in a script with no `set -e`
#     -> `mapfile: command not found` on stderr, **rc=0**, and the following
#        `echo "${#a[@]}"` prints **0**. **The script completes successfully
#        having done nothing, and hands its caller a plausible number.**
#
# **AND NOT ONE OF THE SEVEN SHIPPED HOOKS SETS `-e`**, so the silent form is
# the one this estate would actually get. A guard that silently does nothing
# reports success having checked nothing, which is the exact shape the critic
# gate spent two days removing one layer up.
#
# **`bash -n` IS BLIND TO BOTH, AND THE CONTROL IS WHY THIS TEST IS A GREP.**
# Driven before relying on it: `/bin/bash -n` on a file containing `${v^^}`
# returns **rc=0**, and on one containing `mapfile` returns **rc=0**. Parameter
# expansion operators and builtin names are not validated at parse time, so the
# obvious instrument -- parse every template under the old bash -- produces a
# clean sweep that means nothing. **A zero is not a result until the check has
# produced a non-zero, and that one could not.**
#
# So the instrument is a construct grep, and its REACH is stated rather than
# left to be inferred: it names the constructs known to bite, and a bash-4
# feature not on the list is invisible to it. The list is the finding, not the
# ceiling.

load "../lib/test_helper.bash"

TEMPLATES="${INTENT_PROJECT_ROOT}/lib/templates"

# Each entry is `<label>|<ERE>`. Builtins first, because they are the silent
# ones; syntax second, because it is loud but still ships broken.
BASH4_CONSTRUCTS='mapfile|(^|[^[:alnum:]_])mapfile[[:space:]]
readarray|(^|[^[:alnum:]_])readarray[[:space:]]
associative array|(declare|local|typeset)[[:space:]]+-[A-Za-z]*A
case-modifying expansion|\$\{[A-Za-z_][A-Za-z0-9_]*(\[[^]]*\])?(\^\^?|,,?)\}
coproc|(^|[^[:alnum:]_])coproc[[:space:]]
append-both-redirect|&>>'

template_shell_files() {
  find "$TEMPLATES" -type f \( -name '*.sh' -o -name '*.bash' \) | sort
}

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-bash32-XXXXXX)"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "templates: the population is non-empty, so a clean sweep is not a vacuous one" {
  n="$(template_shell_files | wc -l | tr -d ' ')"
  [ "$n" -ge 5 ]
}

@test "templates: no shipped hook uses a bash-4 construct" {
  # A code comment that NAMES the construct in order to explain why it was
  # avoided is not a use, and canon-ignore-guard.sh carries exactly such a
  # comment. Comment lines are dropped before matching -- otherwise the first
  # person to document this class would trip the guard against it.
  violations=""
  while IFS= read -r file; do
    [ -n "$file" ] || continue
    body="$(grep -vE '^[[:space:]]*#' "$file")"
    while IFS= read -r entry; do
      [ -n "$entry" ] || continue
      label="${entry%%|*}"
      re="${entry#*|}"
      if printf '%s\n' "$body" | grep -qE "$re"; then
        violations="$violations
${file#"${INTENT_PROJECT_ROOT}/"}: $label"
      fi
    done <<< "$BASH4_CONSTRUCTS"
  done <<< "$(template_shell_files)"
  [ -z "$violations" ] || {
    echo "bash-4 construct(s) in a tree every fleet project inherits:$violations"
    false
  }
}

@test "templates: every shipped hook still parses under /bin/bash 3.2" {
  # NECESSARY AND FAR FROM SUFFICIENT, and it is here labelled as such rather
  # than left to look like coverage: this arm cannot see either bash-4 class
  # above (both parse clean at 3.2). It catches ordinary syntax errors only.
  refused=""
  while IFS= read -r file; do
    [ -n "$file" ] || continue
    /bin/bash -n "$file" 2>/dev/null || refused="$refused ${file#"${INTENT_PROJECT_ROOT}/"}"
  done <<< "$(template_shell_files)"
  [ -z "$refused" ] || {
    echo "refused by /bin/bash -n:$refused"
    false
  }
}

# --- THE CONTROLS. Without these the sweep above is an unproven zero. -------

@test "control: the construct list detects a bash-4 builtin when one is present" {
  printf '#!/bin/bash\nmapfile -t a < /etc/hosts\n' > "${TEST_TEMP_DIR}/mutant.sh"
  body="$(grep -vE '^[[:space:]]*#' "${TEST_TEMP_DIR}/mutant.sh")"
  run bash -c "printf '%s\n' \"\$1\" | grep -qE '(^|[^[:alnum:]_])mapfile[[:space:]]'" _ "$body"
  [ "$status" -eq 0 ]
}

@test "control: the construct list detects a case-modifying expansion" {
  printf '#!/bin/bash\nv=abc\necho "${v^^}"\n' > "${TEST_TEMP_DIR}/mutant.sh"
  body="$(grep -vE '^[[:space:]]*#' "${TEST_TEMP_DIR}/mutant.sh")"
  run bash -c "printf '%s\n' \"\$1\" | grep -qE '\\\$\{[A-Za-z_][A-Za-z0-9_]*(\[[^]]*\])?(\^\^?|,,?)\}'" _ "$body"
  [ "$status" -eq 0 ]
}

@test "control: a comment naming the construct is NOT a violation" {
  printf '#!/bin/bash\n# mapfile is bash 4+ so this reads with while/read instead\nwhile IFS= read -r l; do :; done < /etc/hosts\n' \
    > "${TEST_TEMP_DIR}/documented.sh"
  body="$(grep -vE '^[[:space:]]*#' "${TEST_TEMP_DIR}/documented.sh")"
  run bash -c "printf '%s\n' \"\$1\" | grep -qE '(^|[^[:alnum:]_])mapfile[[:space:]]'" _ "$body"
  [ "$status" -ne 0 ]
}

@test "control: /bin/bash -n really is blind to both classes, which is why the grep exists" {
  # This asserts the WEAKNESS of the parse arm, deliberately. If a future bash
  # on this machine starts rejecting these at parse time, this control goes red
  # and the reasoning above needs revisiting -- which is the point.
  printf '#!/bin/bash\nv=abc\necho "${v^^}"\n' > "${TEST_TEMP_DIR}/syn.sh"
  printf '#!/bin/bash\nmapfile -t a < /etc/hosts\n' > "${TEST_TEMP_DIR}/blt.sh"
  /bin/bash -n "${TEST_TEMP_DIR}/syn.sh" 2>/dev/null
  [ "$?" -eq 0 ]
  /bin/bash -n "${TEST_TEMP_DIR}/blt.sh" 2>/dev/null
  [ "$?" -eq 0 ]
}

@test "control: the silent failure is real -- a bash-4 builtin without set -e exits 0" {
  # The reason the builtin class outranks the syntax class. Recorded as a
  # driven fact rather than an assertion about bash, because it is the whole
  # argument for why this file exists.
  printf '#!/bin/bash\nmapfile -t a < /etc/hosts\necho "${#a[@]}"\n' > "${TEST_TEMP_DIR}/silent.sh"
  /bin/bash "${TEST_TEMP_DIR}/silent.sh" > "${TEST_TEMP_DIR}/out" 2>/dev/null
  [ "$?" -eq 0 ]
  [ "$(cat "${TEST_TEMP_DIR}/out")" = "0" ]
}
