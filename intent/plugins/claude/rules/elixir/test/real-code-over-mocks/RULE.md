---
id: IN-EX-TEST-006
language: elixir
category: test
severity: warning
title: Real code over mocks
summary: >
  Never mock when the real module can be tested directly. Use real domain
  calls and real fixtures. Mock only at true external boundaries — HTTP APIs,
  email delivery, payment gateways, SMS, push notifications. Mocking internal
  modules couples tests to implementation and masks integration bugs.
principles:
  - integration-over-isolation
applies_when:
  - "Writing a test for a module in the same app that calls another module in the same app"
  - "Introducing Mox to stub a function that could be called directly"
  - "Using `Mox.expect/3` on a module whose behaviour is owned by the same app"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Stubbing a true external boundary (HTTP client, mailer, payment gateway, SMS, push)"
  - "Stubbing time (`DateTime.utc_now/0`) or randomness (`:rand.uniform/1`) for deterministic assertions"
  - "Testing a protocol or behaviour implementation where the mock *is* the contract under test"
references: []
related_rules:
  - IN-EX-TEST-001
aliases: []
tags:
  - elixir
  - testing
  - mocks
  - integration
status: active
version: 1
---

# Real code over mocks

A mock is a stand-in for something you cannot call in tests. Mocking your own modules treats your own code as "something you cannot call", which is a confession that the design makes integration testing hard — not a reason to skip integration testing. Real code over mocks; mocks only at the edges where the world is external, slow, or non-deterministic.

## Problem

Four failure modes when mocks are used internally:

1. **Tests pass, production fails.** A test mocks `MyApp.Accounts.get_user!/1` to return a fake user. The test passes. Production breaks because the real `get_user!/1` raises on missing users and the caller did not handle the raise. The mock hid the contract.
2. **Refactor-hostile.** Renaming `Accounts.get_user!/1` to `Accounts.fetch_user!/1` breaks every mock. The tests have to be updated in lockstep with the production code. The mock duplicates the contract; both must change, and the test can drift.
3. **Over-specified tests.** `expect(MockAccounts, :get_user!, fn _ -> fake_user() end)` asserts that `get_user!/1` was called exactly once, with this argument. Now the test fails when a refactor coalesces two calls into one — even though the behaviour is unchanged.
4. **Behaviour-free mocks.** When the mocked module has no `@behaviour`, Mox cannot verify that the mock matches the real contract. The mock can return shapes the real module never would, and the tests are green for the wrong reasons.

Use the real module. Use real fixtures. Exercise the actual integration path.

## Detection

Signals:

- `Mox.expect/3` or `Mox.stub/2` on a module defined in the same app (`MyApp.*`) that does not hit a network/disk/hardware boundary.
- A `Mock*` module in `test/support/` shadowing an internal module.
- `with_mock/3` (from the `mock` library) around an internal function call.
- A test named "creates user" that mocks `get_user!/1`, `register_user/1`, or similar — the thing it is testing.
- `MyApp.Accounts.Mock` being used in tests that are _not_ testing the `@behaviour MyApp.Accounts` contract.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE '(expect|stub)\(Mock[A-Z]' test/ --include='*_test.exs'
grep -rnE 'with_mock\(' test/ --include='*_test.exs'
```

The reliable structural signal is "is the mocked boundary genuinely external — network, disk, time, randomness, hardware — or is it just another module in this app?"

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
test "success: sends welcome email" do
  # Antipattern: mocking internal Accounts.get_user!/1
  expect(MockAccounts, :get_user!, fn _id -> %User{id: 1, email: "a@test.com"} end)
  assert :ok = Notifications.send_welcome(1)
end
```

The test verifies that `Notifications.send_welcome/1` called a mocked function. It does not verify that `Accounts.get_user!/1` actually works, or that `send_welcome/1` passes the right fields to the mailer.

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
test "success: sends welcome email with user's address" do
  # Real user via real fixture — no mock on Accounts.
  user = user_fixture(email: "alice@test.com")

  # Mock ONLY the true external boundary — the mailer.
  expect(MockMailer, :deliver, fn email ->
    assert email.to == user.email
    assert email.subject =~ "Welcome"
    {:ok, %{id: "msg-1"}}
  end)

  assert :ok = Notifications.send_welcome(user.id)
end
```

The real `Accounts` is exercised. The real `Notifications` is exercised. Only the mailer — the external boundary — is stubbed.

## When This Applies

- Unit and integration tests for modules that call other modules in the same app.
- Tests that reach across the public surface of a context (Accounts, Content, Billing).
- Tests for coordinators (LiveView, controllers, Oban workers) that orchestrate internal services.

## When This Does Not Apply

- **True external boundaries.** HTTP clients (`Finch`, `Req`, `HTTPoison`), mailers (`Swoosh.Adapters.Test` is fine too), payment gateways, SMS/push providers, cloud storage SDKs — anything talking to a service Intent/your app does not own.
- **Deterministic clocks and RNG.** Injecting a `Clock` or `Random` behaviour to pin time or randomness is the correct way to make tests deterministic.
- **Testing a behaviour.** If your test _is_ asserting that `MyApp.Accounts.Impl` correctly implements `@behaviour MyApp.Accounts`, the mock _is_ the contract under test — Mox's `verify_on_exit!/0` is then doing the right thing.

A good test: "does the mocked module do I/O my test environment cannot do?" If no — use it for real.

## Further Reading

- [Intent `IN-EX-TEST-001` strong-assertions](../strong-assertions/RULE.md) — what your real-code tests should assert, once the mocks are gone.
- [José Valim — "Mocks and explicit contracts"](https://dashbit.co/blog/mocks-and-explicit-contracts) — the canonical Elixir-community article on this rule.
- [Mox docs](https://hexdocs.pm/mox/Mox.html) — Mox is not the villain; over-use of Mox on internal modules is. Mox itself at an external boundary with a `@behaviour` is exemplary.
