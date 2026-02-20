---
description: "Elixir testing rules: strong assertions, no control flow in tests, real code over mocks, spec-driven tests"
---

# Elixir Testing Essentials

Mandatory rules for writing ExUnit tests. These prevent shape tests, stub coupling, and hidden failures.

**NEVER worry about test counts.** Do not count tests, do not report coverage percentages, do not set numeric targets. Quality over quantity, always. One strong test that proves a domain invariant is worth more than twenty shape tests that pass for any implementation.

## Rules

### 1. No control flow in test bodies

NEVER use `if/case/cond/||/&&` in the test body itself. Test bodies are straight-line: setup, action, assert. Test HELPERS in `test/support/` may use normal Elixir -- pattern matching, pipes, `with` blocks.

```elixir
# BAD -- control flow hides the failure path
test "success: creates order" do
  case Orders.create(valid_params()) do
    {:ok, order} -> assert order.status == :pending
    {:error, _} -> flunk("should not fail")
  end
end

# GOOD -- straight-line: setup, action, assert
test "success: creates order" do
  assert {:ok, order} = Orders.create(valid_params())
  assert order.status == :pending
end
```

### 2. Strong assertions against concrete values

NEVER assert on shape alone (`is_struct`, `is_map`, `is_list`, `refute is_nil`). Assert specific field values, specific error messages, specific counts. If you cannot state the expected value, you do not understand the outcome.

```elixir
# BAD -- shape test (passes for ANY user struct)
test "success: creates user" do
  {:ok, user} = Accounts.register_user(params)
  assert is_struct(user, User)
  refute is_nil(user.email)
end

# GOOD -- concrete values prove correctness
test "success: creates user" do
  {:ok, user} = Accounts.register_user(%{email: "alice@test.com", name: "Alice"})
  assert user.email == "alice@test.com"
  assert user.name == "Alice"
  assert String.starts_with?(user.hashed_password, "$2b$")
end
```

### 3. One assertion focus per test

Each test verifies one outcome. Name with `success:`, `failure:`, or `invariant:` prefix. If a test needs two `describe` blocks, it is two tests.

```elixir
# BAD -- testing two unrelated outcomes
test "creates and validates user" do
  {:ok, user} = Accounts.register_user(valid_params())
  assert user.email == "alice@test.com"
  {:error, changeset} = Accounts.register_user(%{email: ""})
  assert errors_on(changeset).email
end

# GOOD -- separate tests, clear names
test "success: creates user with valid email" do
  {:ok, user} = Accounts.register_user(valid_params())
  assert user.email == valid_params().email
end

test "failure: rejects empty email" do
  assert {:error, changeset} = Accounts.register_user(%{email: "", name: "X"})
  assert "can't be blank" in errors_on(changeset).email
end
```

### 4. Test the domain contract, not the implementation

Test through public API (code interfaces in Ash). Never test private functions. Never assert on internal state that could change without affecting behavior.

```elixir
# BAD -- testing internal implementation detail
test "success: hashes password with bcrypt" do
  changeset = User.changeset(%User{}, %{password: "secret"})
  assert Bcrypt.verify_pass("secret", changeset.changes.hashed_password)
end

# GOOD -- test through the public domain API
test "success: password is hashed on registration" do
  {:ok, user} = Accounts.register_user(%{email: "a@test.com", password: "secret"})
  assert user.hashed_password != "secret"
end
```

### 5. Real code over mocks

NEVER mock when the real module can be tested directly. Use real domain calls and Ash generators for test data. Mock ONLY at true external boundaries (HTTP APIs, email delivery, payment gateways). Mocking internal modules is forbidden.

```elixir
# BAD -- mocking an internal module
test "success: sends welcome email" do
  expect(MockAccounts, :get_user!, fn id -> %User{id: id, email: "a@test.com"} end)
  assert :ok = Notifications.send_welcome(123)
end

# GOOD -- use real data, mock only the external mailer
test "success: sends welcome email" do
  user = user_fixture()
  expect(MockMailer, :deliver, fn email ->
    assert email.to == user.email
    assert email.subject =~ "Welcome"
    {:ok, %{id: "msg-1"}}
  end)
  assert :ok = Notifications.send_welcome(user.id)
end
```

### 6. The Highlander Rule for tests

No duplicated setup or assertion patterns. Use shared `setup` blocks for common preconditions. Extract repeated patterns into helpers in `test/support/`. If the same 3 lines appear in multiple tests, refactor into a named helper.

```elixir
# BAD -- duplicated setup in every test
test "success: admin can view dashboard" do
  user = Accounts.register_user!(%{email: "a-#{unique()}@test.com", role: :admin})
  conn = build_conn() |> log_in_user(user)
  conn = get(conn, ~p"/admin/dashboard")
  assert html_response(conn, 200)
end

test "success: admin can view users" do
  user = Accounts.register_user!(%{email: "b-#{unique()}@test.com", role: :admin})
  conn = build_conn() |> log_in_user(user)
  conn = get(conn, ~p"/admin/users")
  assert html_response(conn, 200)
end

# GOOD -- shared setup, focused assertions
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

### 7. Globally unique test data

Use `System.unique_integer([:positive])` for identity attributes. Never hardcode values that could collide in async tests. Fixture helpers must generate unique values by default.

```elixir
# BAD -- hardcoded, will deadlock in async tests
def user_fixture(attrs \\ %{}) do
  Accounts.register_user!(%{email: "alice@test.com", name: "Alice"})
end

# GOOD -- unique per test run
def user_fixture(attrs \\ %{}) do
  unique = System.unique_integer([:positive])
  defaults = %{email: "user-#{unique}@test.com", name: "User #{unique}"}
  params = Map.merge(defaults, Map.new(attrs))
  Accounts.register_user!(params, authorize?: false)
end
```

### 8. Spec-driven tests when spec exists

When a `*_test.spec.md` file exists, every test must correspond to a spec assertion. Test names must match spec assertion text exactly. No tests without a spec counterpart, no spec assertions without a test.

```elixir
# Given spec assertion:
#   - success: creates user with valid email and password
#     - assert user.email == input_email
#     - assert user.hashed_password != input_password

# BAD -- name does not match spec
test "it should create a user" do
  # ...
end

# GOOD -- name matches spec exactly
test "success: creates user with valid email and password" do
  params = %{email: "alice@test.com", password: "secret123"}
  assert {:ok, user} = Accounts.register_user(params)
  assert user.email == params.email
  assert user.hashed_password != params.password
end
```
