#!/usr/bin/env bats
# Tests for ST0039: strict `Greppable proxy` contract in critic_runner.sh.
#
# - critic_proxy_is_simple: predicate accepts only single grep invocations
#   with allowed flag clusters; rejects pipes, -L, -v, awk, xargs, etc.
# - critic_patterns_from_grep_block: walks a rule's proxy block, emits one
#   accepted pattern per simple line, emits a once-per-rule stderr
#   diagnostic for refused lines.
# - critic_apply_rule: union of accepted patterns; dedupe on (line, content).
# - Field-bug regressions: single-step `case` in lib/*.ex and the compliant
#   `use ExUnit.Case, async: true` form do not produce findings.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-proxies-XXXXXX)"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/intent/plugins/claude/lib/rules_lib.sh"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/intent/plugins/claude/lib/critic_runner.sh"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# --- critic_proxy_is_simple: positive cases --------------------------------

@test "proxy_is_simple: accepts plain grep -rnE 'pat' lib/" {
  critic_proxy_is_simple "grep -rnE 'pattern' lib/"
}

@test "proxy_is_simple: accepts grep with --include= flag" {
  critic_proxy_is_simple "grep --include=*.ex -rnE 'pattern' lib/"
}

@test "proxy_is_simple: accepts grep -rn (no -E)" {
  critic_proxy_is_simple "grep -rn 'literal' test/"
}

@test "proxy_is_simple: accepts pattern containing | (regex alternation)" {
  critic_proxy_is_simple "grep -rnE '(foo|bar)' lib/"
}

@test "proxy_is_simple: accepts leading whitespace" {
  critic_proxy_is_simple "    grep -rnE 'pattern' lib/"
}

# --- critic_proxy_is_simple: negative cases --------------------------------

@test "proxy_is_simple: rejects pipe to wc" {
  ! critic_proxy_is_simple "grep -rnE 'pattern' lib/ | wc -l"
}

@test "proxy_is_simple: rejects -L flag (inverse semantics)" {
  ! critic_proxy_is_simple "grep -rnL 'pattern' test/"
}

@test "proxy_is_simple: rejects -L inside flag cluster -rnL" {
  ! critic_proxy_is_simple "grep -rnL 'pattern' test/"
}

@test "proxy_is_simple: rejects -v negative filter" {
  ! critic_proxy_is_simple "grep -rnE 'p1' lib/ | grep -v 'filter'"
}

@test "proxy_is_simple: rejects xargs second-stage filter" {
  ! critic_proxy_is_simple "grep -rnE 'p1' lib/ | xargs grep -l 'p2'"
}

@test "proxy_is_simple: rejects awk" {
  ! critic_proxy_is_simple "awk '/pattern/ {print}' lib/foo.ex"
}

@test "proxy_is_simple: rejects -B context flag" {
  ! critic_proxy_is_simple "grep -rnE -B5 'pattern' lib/"
}

@test "proxy_is_simple: rejects comment line" {
  ! critic_proxy_is_simple "# this is a comment"
}

@test "proxy_is_simple: rejects empty line" {
  ! critic_proxy_is_simple ""
}

@test "proxy_is_simple: rejects non-grep first word" {
  ! critic_proxy_is_simple "echo 'pattern'"
}

# --- critic_patterns_from_grep_block: real rules --------------------------

@test "patterns_from_grep_block: IN-EX-CODE-005 emits both grep lines" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/no-silent-failures/RULE.md"
  local out
  out="$(critic_patterns_from_grep_block "$rule" "IN-EX-CODE-005")"
  [[ "$out" == *"rescue _ ->"* ]]
  [[ "$out" == *"_ = [a-z_]+"* ]]
}

@test "patterns_from_grep_block: IN-EX-TEST-002 emits its single pattern" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  local out
  out="$(critic_patterns_from_grep_block "$rule" "IN-EX-TEST-002")"
  [[ "$out" == *"Process\\.sleep\\("* ]]
}

@test "patterns_from_grep_block: IN-EX-CODE-004 keeps the error-forwarder detector" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/with-for-railway/RULE.md"
  local out
  out="$(critic_patterns_from_grep_block "$rule" "IN-EX-CODE-004")"
  [[ "$out" == *"error -> error"* ]]
  [[ "$out" != *"case.*do"* ]]
}

# Synthetic rule with a refused line: assert the runner emits the stderr
# diagnostic and still emits the kept patterns.
@test "patterns_from_grep_block: emits stderr note for refused lines and keeps simple ones" {
  local rule="${TEST_TEMP_DIR}/RULE.md"
  cat > "$rule" <<'EOF'
---
id: IN-XX-FAKE-001
language: shell
severity: warning
status: active
version: 1
---

# Synthetic rule

## Detection

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'good_pattern' lib/
grep -rnE 'bad_pattern' lib/ | wc -l
```
EOF
  local out
  out="$(critic_patterns_from_grep_block "$rule" "IN-XX-FAKE-001" 2>"${TEST_TEMP_DIR}/err")"
  local err
  err="$(cat "${TEST_TEMP_DIR}/err")"
  [[ "$out" == *"good_pattern"* ]]
  [[ "$out" != *"bad_pattern"* ]]
  [[ "$err" == *"note: skipping IN-XX-FAKE-001"* ]]
}

@test "patterns_from_grep_block: stderr diagnostic emitted only once per rule" {
  local rule="${TEST_TEMP_DIR}/RULE.md"
  cat > "$rule" <<'EOF'
---
id: IN-XX-FAKE-002
language: shell
severity: warning
status: active
version: 1
---

# Two refused lines

## Detection

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnL 'p1' test/
grep -rnE 'p2' lib/ | xargs grep -l 'p3'
```
EOF
  critic_patterns_from_grep_block "$rule" "IN-XX-FAKE-002" 2>"${TEST_TEMP_DIR}/err" >/dev/null
  local count
  count="$(grep -c 'note: skipping' "${TEST_TEMP_DIR}/err" || true)"
  [ "$count" -eq 1 ]
}

# --- ST0039 regression: stripped rules have no greppable proxy ------------

@test "IN-EX-TEST-003 has no greppable proxy (inverse semantics)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/async-by-default/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-CODE-003 has no greppable proxy (line continuation + filter)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/impl-true-on-callbacks/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-LV-001 has no greppable proxy (-B context + filter pipe)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/lv/two-phase-mount/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-LV-003 has no greppable proxy (awk state machine)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/lv/thin-liveviews/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-PHX-001 has no greppable proxy (awk state machine)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/phoenix/thin-controllers/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-ASH-001 has no greppable proxy (callsite scope cannot be inferred per-file)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/ash/code-interfaces-only/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-ASH-002 has no greppable proxy (proxy was inverted: fired on compliant)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/ash/actor-on-query/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

@test "IN-EX-TEST-004 has no greppable proxy (filter pipe required)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/start-supervised/RULE.md"
  [ -z "$(critic_extract_greppable_block "$rule")" ]
}

# --- Field-bug regressions: false-positive reproductions ------------------

@test "IN-EX-CODE-004 does NOT fire on single-step case (Conflab regression)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/with-for-railway/RULE.md"
  local fixture="${TEST_TEMP_DIR}/lib/foo.ex"
  mkdir -p "${TEST_TEMP_DIR}/lib"
  cat > "$fixture" <<'EOF'
defmodule Foo do
  def find_user(id) do
    case Repo.get(User, id) do
      nil -> {:error, :not_found}
      user -> {:ok, user}
    end
  end
end
EOF
  local out
  out="$(cd "${TEST_TEMP_DIR}" && critic_apply_rule "$rule" "lib/foo.ex" 2>/dev/null)"
  [ -z "$out" ]
}

@test "IN-EX-TEST-003 does NOT fire on compliant async test (Conflab regression)" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/async-by-default/RULE.md"
  local fixture="${TEST_TEMP_DIR}/test/foo_test.exs"
  mkdir -p "${TEST_TEMP_DIR}/test"
  cat > "$fixture" <<'EOF'
defmodule FooTest do
  use ExUnit.Case, async: true

  test "trivial" do
    assert 1 + 1 == 2
  end
end
EOF
  local out
  out="$(cd "${TEST_TEMP_DIR}" && critic_apply_rule "$rule" "test/foo_test.exs" 2>/dev/null)"
  [ -z "$out" ]
}

# --- Positive controls: kept rules still detect their target patterns -----

@test "IN-EX-TEST-002 still fires on Process.sleep in *_test.exs" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/no-process-sleep/RULE.md"
  local fixture="${TEST_TEMP_DIR}/test/bar_test.exs"
  mkdir -p "${TEST_TEMP_DIR}/test"
  cat > "$fixture" <<'EOF'
defmodule BarTest do
  use ExUnit.Case, async: true

  test "racy thing" do
    spawn(fn -> :ok end)
    Process.sleep(100)
    assert true
  end
end
EOF
  local out
  out="$(cd "${TEST_TEMP_DIR}" && critic_apply_rule "$rule" "test/bar_test.exs" 2>/dev/null)"
  [[ "$out" == *"Process.sleep"* ]]
  [[ "$out" == *"critical"* ]]
}

@test "IN-EX-CODE-004 still fires on error -> error forwarder antipattern" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/with-for-railway/RULE.md"
  local fixture="${TEST_TEMP_DIR}/lib/forwarder.ex"
  mkdir -p "${TEST_TEMP_DIR}/lib"
  cat > "$fixture" <<'EOF'
defmodule Forwarder do
  def chain(input) do
    case validate(input) do
      {:ok, x} ->
        case persist(x) do
          {:ok, y} -> {:ok, y}
          error -> error
        end
      error -> error
    end
  end
end
EOF
  local out
  out="$(cd "${TEST_TEMP_DIR}" && critic_apply_rule "$rule" "lib/forwarder.ex" 2>/dev/null)"
  [[ "$out" == *"error -> error"* ]]
}

@test "IN-EX-CODE-005 multi-pattern union: union dedupes (line, content) hits" {
  local rule="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/code/no-silent-failures/RULE.md"
  local fixture="${TEST_TEMP_DIR}/lib/silent.ex"
  mkdir -p "${TEST_TEMP_DIR}/lib"
  cat > "$fixture" <<'EOF'
defmodule Silent do
  def first do
    try do
      do_it()
    rescue _ -> :ok
    end
  end

  def second do
    _ = compute_thing()
    :ok
  end
end
EOF
  local out lines
  out="$(cd "${TEST_TEMP_DIR}" && critic_apply_rule "$rule" "lib/silent.ex" 2>/dev/null)"
  lines="$(printf '%s\n' "$out" | grep -c "IN-EX-CODE-005" || true)"
  [ "$lines" -eq 2 ]
}
