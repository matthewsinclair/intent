---
id: IN-EX-TEST-007
language: elixir
category: test
severity: warning
title: Highlander Rule for tests — shared setup and fixtures
summary: >
  No duplicated setup or assertion patterns across tests. Common preconditions
  go in a `setup` block. Repeated fixture-creation expressions go in
  `test/support/` helpers with globally-unique identity attributes. If the
  same three lines appear in two tests, extract.
principles:
  - highlander
applies_when:
  - "Three or more tests duplicate the same setup lines verbatim"
  - "A fixture helper is being copy-pasted into multiple test files"
  - "A hard-coded identity (email, slug, token) is used across tests and will collide under `async: true`"
applies_to:
  - "test/**/*_test.exs"
  - "test/support/**/*.ex"
does_not_apply_when:
  - "One-off tests that genuinely share no setup with the rest of the suite"
  - "Tests whose setup is intentionally varied to exercise a specific branch"
  - "Trivial single-line assertions that are easier to read inline than extract"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-TEST-001
  - IN-EX-CODE-006
aliases: []
tags:
  - elixir
  - testing
  - fixtures
  - highlander
status: active
version: 1
---

# Highlander Rule for tests — shared setup and fixtures

"There can be only one" applies to tests too. A user-registration fixture, an authenticated-conn setup, an admin-role factory — each belongs in one place. Duplicating them across test files guarantees drift: someone adds a `:role` column, updates the fixture in `accounts_test.exs`, and three other test files are now using the outdated version.

## Problem

Three failure modes when test setup is duplicated:

1. **Fixture drift.** `user_fixture/1` is defined in `accounts_test.exs` and re-defined in `posts_test.exs` with slightly different defaults. A model change updates one; the other keeps returning the old shape. Tests pass; the integration path is untested.
2. **Hard-coded identity collisions.** `user_fixture/0` that always creates `%{email: "alice@test.com"}` deadlocks in `async: true` runs — two tests try to insert the same unique key; one fails. The fix is not "add `async: false`"; it is `System.unique_integer/1` in the fixture.
3. **Assertion-pattern repetition.** Every test asserts `assert html =~ "Dashboard"` after logging in an admin. Extract the login + the assertion pattern into a helper once; every test then reads "log in admin; assert dashboard". The focus of each test is the thing that varies, not the boilerplate.

The discipline is the same as for production code: one canonical home per concern. Fixtures go in `test/support/fixtures.ex` (or one file per context). Authenticated conns go in `test/support/conn_case.ex`. Named `setup` blocks share preconditions within a module; `ExUnit.CaseTemplate` shares them across modules.

## Detection

Signals:

- The same 3+ lines appearing in multiple tests (grep the exact string).
- Two `user_fixture/N` definitions in different test files.
- Hard-coded email/slug/token values that will collide under concurrent inserts.
- A test module with 5+ tests where the first 3 lines of each test are identical.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rn 'def user_fixture' test/
grep -rnE '"(alice|bob|admin)@[^"]*\.(com|test)"' test/ --include='*_test.exs'
```

The reliable structural signal is "if I changed `user_fixture` today, how many files would I have to edit?" The answer must be one.

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
test "success: admin can view dashboard" do
  user = Accounts.register_user!(%{email: "admin@test.com", role: :admin})
  conn = build_conn() |> log_in_user(user)
  conn = get(conn, ~p"/admin/dashboard")
  assert html_response(conn, 200)
end

test "success: admin can view users" do
  user = Accounts.register_user!(%{email: "admin@test.com", role: :admin})
  conn = build_conn() |> log_in_user(user)
  conn = get(conn, ~p"/admin/users")
  assert html_response(conn, 200)
end
```

Four lines duplicated; hard-coded `admin@test.com` collides in async; each test re-does the login before getting to the assertion of interest.

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
setup do
  user = user_fixture(role: :admin)
  conn = build_conn() |> log_in_user(user)
  %{conn: conn, user: user}
end

test "success: admin can view dashboard", %{conn: conn} do
  conn = get(conn, ~p"/admin/dashboard")
  assert html_response(conn, 200) =~ "Dashboard"
end

test "success: admin can view users", %{conn: conn} do
  conn = get(conn, ~p"/admin/users")
  assert html_response(conn, 200) =~ "Users"
end
```

The shared precondition runs once per test via `setup`. Each test body is the variation. `user_fixture/1` lives in `test/support/` and uses `System.unique_integer/1` for identity so async-safe concurrency is the default.

## When This Applies

- Any test module with 3+ tests sharing the same `Accounts.register_user!`, `build_conn`, `log_in_user` sequence.
- Any fixture helper defined twice across files.
- Any test file whose first 3 lines of each test are identical.

## When This Does Not Apply

- **One-off tests.** A single isolated test whose setup is genuinely bespoke. Extraction would hurt clarity.
- **Variation testing.** If the setup varies per test to exercise a specific branch (e.g. `setup do %{role: :admin} end` vs `setup do %{role: :user} end` across describes), the variance is the point — extract the _parts_ that stay the same, not the whole thing.
- **Trivial one-liners.** `assert html =~ "Dashboard"` once per test is not "duplication" in any meaningful sense. Extract only when the boilerplate exceeds the variation.

A good test: "if the shared setup changed, would I have to edit more than one place?" If yes, extract.

## Further Reading

- [Intent `IN-AG-HIGHLANDER-001`](../../../agnostic/highlander/RULE.md) — the cross-language rule.
- [Intent `IN-EX-CODE-006` module-highlander](../../code/module-highlander/RULE.md) — the production-code equivalent.
- [Intent `IN-EX-TEST-001` strong-assertions](../strong-assertions/RULE.md) — what each test asserts after the shared setup runs.
- [ExUnit.CaseTemplate docs](https://hexdocs.pm/ex_unit/ExUnit.CaseTemplate.html) — the primary mechanism for sharing setup across test modules.
