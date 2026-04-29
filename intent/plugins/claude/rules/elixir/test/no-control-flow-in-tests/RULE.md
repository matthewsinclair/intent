---
id: IN-EX-TEST-005
language: elixir
category: test
severity: warning
title: No control flow in test bodies
summary: >
  Test bodies are straight-line: setup, action, assert. Never use `if`, `case`,
  `cond`, `||`, or `&&` to decide which branch the test is on. Each branch is a
  separate test. Helpers in `test/support/` may use normal Elixir; test bodies
  may not.
principles:
  - pattern-matching
  - test-determinism
applies_when:
  - 'Inside a `test "..." do ... end` block'
  - "Inside a `describe` block's setup where branching would select different assertions"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Helpers in `test/support/` — those are regular Elixir and use all language constructs"
  - "Pattern-matching assertions (`assert {:ok, _} = call()`) — that is a match, not control flow"
  - "Guards on `setup` callbacks or test tags — those select which tests run, not which branch runs within a test"
references:
  - IN-EX-CODE-001
related_rules:
  - IN-EX-TEST-001
aliases: []
tags:
  - elixir
  - testing
  - determinism
status: active
version: 1
---

# No control flow in test bodies

A test with an `if` inside it is two tests pretending to be one. The reader cannot tell which branch ran. When the test fails, the stack trace points at the assertion inside the branch; the assertion inside the _other_ branch is silently skipped. The solution is not to add a second `if`; it is to split the test.

## Problem

Four failure modes:

1. **Silent branch skip.** `if success?, do: assert_success(), else: assert_failure()` passes when `success?` is wrong — because the else branch quietly asserts the wrong contract. The test is green; the behaviour is broken.
2. **Unclear failure message.** When an `if` test fails, the stack trace says "line 47 of test_x.exs, in the `else` branch". You have to re-read the test to work out what branch was taken. A split pair of tests gives you the branch name in the test name.
3. **`case` for extracting a value is a smell.** `case call() do; {:ok, v} -> assert v == 42; _ -> flunk(...) end` is an assertion-with-fallback-error. Replace with `assert {:ok, 42} = call()` — the `=` fails with a clear diagnostic if the shape or value differs.
4. **`||` / `&&` defaults hide contract.** `result = call() || :default` — what was the test actually asserting? Was `:default` the expected failure path, or a safety net against a bug? A test should pin the answer, not choose between answers.

The discipline is: one test, one path, one set of assertions. If the function has two behaviours, write two tests.

## Detection

Signals:

- `if` / `unless` / `case` / `cond` inside a `test "..." do ... end` block.
- `||` or `&&` building a value that is then asserted on.
- `try/rescue` inside a test body (almost always hiding a missing `assert_raise/2`).
- `case result do; {:ok, v} -> assert v == x; _ -> flunk("expected ok") end` — should be `assert {:ok, ^x} = result`.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE '^[[:space:]]+(if|unless|case|cond) ' test/
```

The reliable structural signal is "is the test making a decision about what to assert?" If yes, split it.

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
test "creates order" do
  case Orders.create(params()) do
    {:ok, order} -> assert order.status == :pending
    {:error, _} -> flunk("should not fail")
  end
end

test "admin or user can view" do
  if admin?, do: assert allowed?(user), else: assert !allowed?(user)
end
```

The first test hides the assertion behind a `case`; a failure prints "flunk" instead of the real shape. The second packs two tests into one.

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
test "success: creates order with valid params" do
  assert {:ok, order} = Orders.create(valid_params())
  assert order.status == :pending
end

test "success: admin can view" do
  assert allowed?(admin_user())
end

test "success: non-admin cannot view" do
  refute allowed?(regular_user())
end
```

Each test has one path. Each failure points at a specific assertion. Adding a fourth case (`:editor` role) is a new test, not an extra `if` branch.

## When This Applies

- Any test body, in any file under `test/`.
- Any `setup` block that uses control flow to choose assertions (setup should _prepare_, not _assert_).

## When This Does Not Apply

- **Pattern-match assertions.** `assert {:ok, v} = call()` is a match, not a branch. It is the preferred form.
- **`test/support/` helpers.** A fixture helper or test utility may use `if`, `case`, `cond`, pipes, `with` — helpers are normal Elixir.
- **`ExUnit.Case` tag-based selection.** `@tag :integration` or `@moduletag :skip` is a _test selection_ mechanism, not an in-test branch.
- **Guarded `setup` clauses.** `setup %{role: :admin}` and `setup %{role: :user}` are routing by test metadata; each test body still runs straight-line.

A good test: "can a reader glance at this test body and tell exactly which assertions fire?" If the answer involves "it depends on...", split the test.

## Further Reading

- [Intent `IN-EX-CODE-001` pattern-match-over-conditionals](../../code/pattern-match-over-conditionals/RULE.md) — the production-code equivalent of this rule.
- [Intent `IN-EX-TEST-001` strong-assertions](../strong-assertions/RULE.md) — what to assert once you have split the test.
- [ExUnit docs — `assert_raise/2` and `assert_receive/3`](https://hexdocs.pm/ex_unit/ExUnit.Assertions.html) — the built-in assertions that replace `try/rescue` and timing-sleep patterns.
