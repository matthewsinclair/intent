---
id: IN-EX-TEST-001
language: elixir
category: test
severity: critical
title: Strong assertions against concrete values
summary: >
  Shape assertions such as `assert is_struct/2`, `assert is_map/1`, and
  `refute is_nil/1` pass for any value of the right type. They do not prove
  the function did what the test claims. Assert on concrete field values and
  return shapes — usually by pattern matching.
principles:
  - honest-data
  - public-interface
applies_when:
  - "Any ExUnit test asserting on a return value of a fallible function"
  - "Assertions on struct fields, map values, or list contents"
  - "Assertions that a non-nil return was produced (without checking what it is)"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Property-based tests asserting invariants rather than specific values"
  - "Tests where the shape itself is the contract (GenServer callback return, `{:ok, socket}` etc.)"
  - "Contract/type tests whose sole purpose is to verify a return type"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-TEST-002
tags:
  - elixir
  - exunit
  - assertions
aliases: []
status: active
version: 1
---

# Strong assertions against concrete values

Tests that assert shape without asserting value let real bugs through while appearing to pass. Assert what the function is supposed to produce, not merely that it produced something of the right type.

## Problem

A passing test should be evidence that the function under test behaves as specified. Shape assertions like `assert is_struct(result, User)` or `refute is_nil(result)` degrade that evidence to "the function returned something that looks roughly like a User." Any bug that leaves the shape intact but corrupts the content slips through.

Three failure modes recur:

1. **Silent regressions.** A change swaps `role: :admin` for `role: :viewer` in a default. Every shape-based test still passes. The bug ships.
2. **False coverage.** The test suite size grows, but the mean information-per-test falls. Reviewers glance at the assertion, see `assert is_struct/2`, and assume the test is doing work.
3. **Misleading failures.** When the shape assertion finally does fail (because the function now returns `nil` or an `{:error, _}` tuple), the failure message tells the reviewer nothing about which field is wrong — because the test never cared about fields.

The root cause is testing _"what type came back"_ instead of _"what value came back."_ Pattern matching against a literal expected shape flips the emphasis back to value.

## Detection

Scan test files for shape-only assertions that are the **last assertion in a test**. If a shape assertion is followed by field assertions, it is usually a harmless preamble; if it is the only assertion, it is the rule violation.

Signals:

- `assert is_struct(x)` or `assert is_struct(x, Mod)` with no subsequent field assertions.
- `assert is_map(x)` followed only by `Map.has_key?/2` checks (key presence without value checks).
- `assert is_list(x)` without any `assert Enum.at/length/any?` on the contents.
- `refute is_nil(x)` as the sole assertion on a function result.
- `assert match?(%Mod{}, x)` with `_` field patterns or no field patterns at all — the wildcard version is equivalent to `is_struct/2`.

Greppable proxy (not authoritative; Critic must confirm by reading surrounding context):

```bash
grep -rnE 'assert is_struct|assert is_map|assert is_list|refute is_nil' test/
```

A Critic subagent confirms by reading the test body and checking whether any subsequent line constrains the actual value.

## Bad

See `bad_test.exs` for the runnable form. Inline snippet:

```elixir
test "fetch/1 returns a user" do
  user = UserService.fetch(42)
  assert is_struct(user, UserService)
  assert is_integer(user.id)
  refute is_nil(user.role)
end
```

All three assertions pass when `role: :banned` replaces the expected `:viewer`. The test name promises "returns a user," and the assertions only verify "returned a struct of the right module."

## Good

See `good_test.exs` for the runnable form. Inline snippet:

```elixir
test "fetch/1 returns the user with the requested id and default viewer role" do
  assert %UserService{id: 42, name: "Default", role: :viewer} = UserService.fetch(42)
end
```

One assertion, via pattern match, pinning every field the function is contracted to produce. Any regression in `id`, `name`, or `role` fails the test at the binding site with a diagnostic that shows the actual value.

## When This Applies

- Unit tests for functions that return structs, maps, or lists populated from arguments or fixtures.
- Integration tests that round-trip a value through a service and assert on the result.
- Any test whose name implies a behavioural claim ("returns the correct X") rather than a type claim ("returns an X").

In most ExUnit codebases this covers the large majority of tests.

## When This Does Not Apply

- **Property-based tests.** `StreamData` generators produce random inputs; the test asserts invariants, not specific values. `assert is_integer/1` on a generator result is appropriate because the generator covers the value space elsewhere.
- **Shape-is-contract tests.** Some APIs specify the shape as the contract. A GenServer `handle_call/3` that is documented to return `{:reply, _, socket}` is correctly tested with `assert match?({:reply, _, %Socket{}}, result)`. The shape _is_ the public interface.
- **Type-contract tests.** A test whose only job is to verify that `fetch/1` returns a `User` struct (not a map, not a tuple) is legitimate; usually a single such test guards the type contract alongside many value-level tests.
- **Negative tests that assert a specific error tuple.** `assert {:error, :not_found} = lookup(unknown_id)` is a pattern-match assertion on a concrete value and already complies with this rule.

If a reviewer is unsure whether a test is an exception, the default is to write the strong assertion — the burden falls on the shape-only form to justify itself.

## Further Reading

- [Intent `IN-AG-HIGHLANDER-001`](../../../../rules/agnostic/highlander/RULE.md) — duplicate assertion paths tend to drift; one value-level assertion is better than three shape-level ones.
- [ExUnit.Assertions — `assert/1` and pattern matching](https://hexdocs.pm/ex_unit/ExUnit.Assertions.html#assert/1) — how pattern-match assertions generate precise failure messages.
- [José Valim — "Mocks and explicit contracts"](https://dashbit.co/blog/mocks-and-explicit-contracts) — context on testing against behaviour not shape.
- [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) — adjacent upstream rules on test strength (`rules/core/assert-receive-vs-received`, `rules/telemetry/test-shape-not-values`) live in the pinned commit; see `_attribution/elixir-test-critic.md`.
