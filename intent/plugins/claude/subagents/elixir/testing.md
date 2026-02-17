# Elixir Testing Patterns Reference

Comprehensive reference for testing in Phoenix/Ash projects. Covers test case modules, LiveView testing, Mox, Ash-specific patterns, and common pitfalls.

## Test Organization

### Directory Structure

```
test/
├── test_helper.exs
├── my_app/                      # Domain tests (use DataCase)
│   ├── accounts/
│   │   └── accounts_test.exs
│   └── content/
│       └── content_test.exs
├── my_app_web/                  # Web tests
│   ├── controllers/             # Controller tests (use ConnCase)
│   │   └── page_controller_test.exs
│   └── live/                    # LiveView tests (use ConnCase)
│       └── post_live_test.exs
└── support/
    ├── conn_case.ex             # ConnCase module
    ├── data_case.ex             # DataCase module
    ├── fixtures/                # Test data factories
    │   ├── accounts_fixtures.ex
    │   └── content_fixtures.ex
    └── mocks/
        └── mocks.ex             # Mox mock definitions
```

### Naming Conventions

```elixir
describe "create_user/2" do
  test "success: creates user with valid attributes" do
    # happy path
  end

  test "success: creates user with optional fields omitted" do
    # edge case happy path
  end

  test "failure: returns error for duplicate email" do
    # expected failure
  end

  test "failure: returns error when required field missing" do
    # validation failure
  end
end
```

- Prefix test names with `success:` or `failure:`
- Use `describe` blocks per function under test
- One assertion focus per test

## DataCase — Domain Tests

DataCase sets up the Ecto sandbox for isolated database access in tests.

```elixir
defmodule MyApp.DataCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      alias MyApp.Repo

      import Ecto
      import Ecto.Changeset
      import Ecto.Query
      import MyApp.DataCase
    end
  end

  setup tags do
    MyApp.DataCase.setup_sandbox(tags)
    :ok
  end

  def setup_sandbox(tags) do
    pid = Ecto.Adapters.SQL.Sandbox.start_owner!(MyApp.Repo, shared: not tags[:async])
    on_exit(fn -> Ecto.Adapters.SQL.Sandbox.stop_owner(pid) end)
  end
end
```

### Using DataCase

```elixir
defmodule MyApp.AccountsTest do
  use MyApp.DataCase, async: true

  alias MyApp.Accounts

  describe "create_user/2" do
    test "success: creates user with valid attributes" do
      user = Accounts.create_user!("alice@example.com", "Alice", authorize?: false)

      assert user.email == "alice@example.com"
      assert user.name == "Alice"
    end

    test "failure: returns error for duplicate email" do
      Accounts.create_user!("alice@example.com", "Alice", authorize?: false)

      assert {:error, _changeset} =
               Accounts.create_user("alice@example.com", "Bob", authorize?: false)
    end
  end
end
```

### Key DataCase Patterns

- Always use `async: true` when tests don't share state
- Use `authorize?: false` when authorization is not the test focus
- Prefer `!` functions for setup (clearer errors on failure)
- Use non-raising functions when testing error cases

## ConnCase — Controller and LiveView Tests

ConnCase provides a connection for testing HTTP endpoints and LiveViews.

```elixir
defmodule MyAppWeb.ConnCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint MyAppWeb.Endpoint

      use MyAppWeb, :verified_routes

      import Plug.Conn
      import Phoenix.ConnTest
      import Phoenix.LiveViewTest
      import MyAppWeb.ConnCase
    end
  end

  setup tags do
    MyApp.DataCase.setup_sandbox(tags)
    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end
end
```

### Controller Tests

```elixir
defmodule MyAppWeb.PageControllerTest do
  use MyAppWeb.ConnCase, async: true

  describe "GET /" do
    test "success: renders home page", %{conn: conn} do
      conn = get(conn, ~p"/")
      assert html_response(conn, 200) =~ "Welcome"
    end
  end
end
```

### Authenticated Controller Tests

```elixir
defmodule MyAppWeb.PostControllerTest do
  use MyAppWeb.ConnCase, async: true

  import MyApp.Fixtures.AccountsFixtures

  setup %{conn: conn} do
    user = user_fixture()
    conn = log_in_user(conn, user)
    %{conn: conn, user: user}
  end

  describe "GET /posts" do
    test "success: lists posts for authenticated user", %{conn: conn} do
      conn = get(conn, ~p"/posts")
      assert html_response(conn, 200) =~ "Posts"
    end
  end
end
```

### JSON API Tests

```elixir
describe "POST /api/posts" do
  test "success: creates post and returns 201", %{conn: conn} do
    payload = %{post: %{title: "Test", body: "Content"}}

    conn =
      conn
      |> put_req_header("content-type", "application/json")
      |> post(~p"/api/posts", payload)

    assert %{"id" => _id, "title" => "Test"} = json_response(conn, 201)
  end
end
```

## LiveView Tests

LiveView tests use `ConnCase` with `Phoenix.LiveViewTest` helpers.

### Basic LiveView Test

```elixir
defmodule MyAppWeb.PostLive.IndexTest do
  use MyAppWeb.ConnCase, async: true

  import MyApp.Fixtures.ContentFixtures
  import Phoenix.LiveViewTest

  setup %{conn: conn} do
    user = user_fixture()
    conn = log_in_user(conn, user)
    %{conn: conn, user: user}
  end

  describe "Index" do
    test "success: lists all posts", %{conn: conn} do
      post = post_fixture()

      {:ok, _view, html} = live(conn, ~p"/posts")

      assert html =~ post.title
    end

    test "success: creates new post via form", %{conn: conn} do
      {:ok, view, _html} = live(conn, ~p"/posts")

      view
      |> form("#post-form", post: %{title: "New Post", body: "Content"})
      |> render_submit()

      assert has_element?(view, "#posts", "New Post")
    end
  end
end
```

### LiveView Navigation Tests

```elixir
test "success: navigates to post detail", %{conn: conn} do
  post = post_fixture()

  {:ok, view, _html} = live(conn, ~p"/posts")

  {:ok, _detail_view, html} =
    view
    |> element("#post-#{post.id}")
    |> render_click()
    |> follow_redirect(conn)

  assert html =~ post.title
end
```

### LiveView Event Tests

```elixir
test "success: deletes post on click", %{conn: conn} do
  post = post_fixture()

  {:ok, view, _html} = live(conn, ~p"/posts")

  view
  |> element("#delete-#{post.id}")
  |> render_click()

  refute has_element?(view, "#post-#{post.id}")
end
```

### LiveView Component Tests

```elixir
test "success: renders form component" do
  post = post_fixture()

  html =
    render_component(MyAppWeb.PostLive.FormComponent,
      id: "post-form",
      post: post,
      action: :edit,
      current_user: user_fixture()
    )

  assert html =~ post.title
  assert html =~ "Save"
end
```

### LiveView Flash Tests

```elixir
test "success: shows flash message after action", %{conn: conn} do
  {:ok, view, _html} = live(conn, ~p"/posts")

  view
  |> form("#post-form", post: %{title: "New Post"})
  |> render_submit()

  assert render(view) =~ "Post created successfully"
end
```

## Mox — Behaviour-Based Mocking

Mox enforces behaviour contracts for mocks, preventing tests from drifting from real implementations.

### Setup

```elixir
# test/support/mocks/mocks.ex
Mox.defmock(MyApp.MockMailer, for: MyApp.Mailer.Behaviour)
Mox.defmock(MyApp.MockHTTPClient, for: MyApp.HTTPClient.Behaviour)
```

```elixir
# config/test.exs
config :my_app, mailer: MyApp.MockMailer
config :my_app, http_client: MyApp.MockHTTPClient
```

### Using Mox in Tests

```elixir
defmodule MyApp.NotificationsTest do
  use MyApp.DataCase, async: true

  import Mox

  setup :verify_on_exit!

  describe "send_welcome/1" do
    test "success: sends welcome email" do
      user = user_fixture()

      expect(MyApp.MockMailer, :deliver, fn email ->
        assert email.to == user.email
        assert email.subject =~ "Welcome"
        {:ok, %{id: "msg-123"}}
      end)

      assert :ok = MyApp.Notifications.send_welcome(user)
    end
  end
end
```

### Mox Patterns

- **`expect/3`** — function must be called exactly N times (default 1)
- **`stub/3`** — function can be called any number of times
- **`verify_on_exit!/1`** — ensure all expectations were met (use in `setup`)
- **`allow/3`** — share mock expectations with another process (for async)

```elixir
# Expect exactly 2 calls
expect(MyApp.MockMailer, :deliver, 2, fn _email -> {:ok, %{}} end)

# Stub for any number of calls
stub(MyApp.MockMailer, :deliver, fn _email -> {:ok, %{}} end)

# Allow another process to use this mock
allow(MyApp.MockMailer, self(), pid)
```

## Ash Testing Patterns

### Testing Domain Actions

```elixir
defmodule MyApp.AccountsTest do
  use MyApp.DataCase, async: true

  describe "register_user/2" do
    test "success: creates user with hashed password" do
      user =
        MyApp.Accounts.register_user!("alice@example.com", "password123",
          authorize?: false
        )

      assert user.email == "alice@example.com"
      assert user.hashed_password != "password123"
    end

    test "failure: rejects invalid email" do
      assert {:error, changeset} =
               MyApp.Accounts.register_user("invalid", "password123",
                 authorize?: false
               )

      assert errors_on(changeset)[:email]
    end
  end
end
```

### Testing Ash Policies

```elixir
describe "authorization" do
  test "success: admin can delete any post" do
    admin = user_fixture(role: :admin)
    post = post_fixture()

    assert {:ok, _} = MyApp.Content.delete_post(post.id, actor: admin)
  end

  test "failure: regular user cannot delete others' posts" do
    user = user_fixture()
    other_user = user_fixture()
    post = post_fixture(author: other_user)

    assert {:error, %Ash.Error.Forbidden{}} =
             MyApp.Content.delete_post(post.id, actor: user)
  end

  test "success: user can check permissions with can?" do
    user = user_fixture()
    post = post_fixture(author: user)

    assert Ash.can?({MyApp.Content.Post, :delete}, user)
  end
end
```

### Test Data with Ash.Generator

```elixir
# In test fixtures
defmodule MyApp.Fixtures.AccountsFixtures do
  def user_fixture(attrs \\ %{}) do
    defaults = %{
      email: "user-#{System.unique_integer([:positive])}@example.com",
      name: "Test User"
    }

    attrs = Map.merge(defaults, Map.new(attrs))
    MyApp.Accounts.create_user!(attrs.email, attrs.name, authorize?: false)
  end
end
```

### Globally Unique Values

Use `System.unique_integer/1` for identity attributes to prevent deadlocks in concurrent tests:

```elixir
# GOOD — unique per test run, safe for async
def user_fixture(attrs \\ %{}) do
  unique = System.unique_integer([:positive])

  defaults = %{
    email: "user-#{unique}@example.com",
    username: "user_#{unique}"
  }

  Map.merge(defaults, Map.new(attrs))
  |> then(&MyApp.Accounts.create_user!(&1.email, &1[:name] || "User", authorize?: false))
end

# BAD — hardcoded, will deadlock in async tests
def user_fixture(_attrs \\ %{}) do
  MyApp.Accounts.create_user!("alice@example.com", "Alice", authorize?: false)
end
```

## File Upload Testing

```elixir
describe "avatar upload" do
  test "success: uploads and saves avatar", %{conn: conn} do
    {:ok, view, _html} = live(conn, ~p"/settings")

    avatar =
      file_input(view, "#avatar-form", :avatar, [
        %{
          name: "photo.jpg",
          content: File.read!("test/support/fixtures/photo.jpg"),
          type: "image/jpeg"
        }
      ])

    assert render_upload(avatar, "photo.jpg") =~ "photo.jpg"

    view
    |> form("#avatar-form")
    |> render_submit()

    assert render(view) =~ "Avatar updated"
  end

  test "failure: rejects oversized file", %{conn: conn} do
    {:ok, view, _html} = live(conn, ~p"/settings")

    large_content = :crypto.strong_rand_bytes(10_000_000)

    avatar =
      file_input(view, "#avatar-form", :avatar, [
        %{
          name: "huge.jpg",
          content: large_content,
          type: "image/jpeg"
        }
      ])

    assert render_upload(avatar, "huge.jpg") =~ "Too large"
  end
end
```

## Async Test Considerations

### When to Use `async: true`

- Tests that only read/write their own data through the Ecto sandbox
- Tests that don't rely on shared process state
- Most DataCase and ConnCase tests

### When to Use `async: false`

- Tests that modify application environment (`Application.put_env`)
- Tests that interact with shared GenServers or ETS tables
- Tests that need exclusive access to external services

### Sandbox Ownership

```elixir
# Manual ownership for spawned processes
setup do
  :ok = Ecto.Adapters.SQL.Sandbox.checkout(MyApp.Repo)
  Ecto.Adapters.SQL.Sandbox.mode(MyApp.Repo, {:shared, self()})
end
```

For Mox with async tests, explicitly allow the process:

```elixir
setup do
  # Allow the test process to share mock expectations
  Mox.allow(MyApp.MockMailer, self(), fn -> self() end)
  :ok
end
```

## Common Testing Mistakes

1. **Hardcoded identity attributes** — use `System.unique_integer/1` for concurrent safety
2. **Testing private functions** — test through the public API
3. **Business logic assertions in LiveView tests** — test logic in domain tests; LiveView tests verify wiring
4. **Missing `verify_on_exit!`** — always set up in Mox tests
5. **`{:ok, result} = Domain.create()`** — use `Domain.create!()` for clearer error messages
6. **Testing implementation details** — test behaviour, not internal state
7. **Shared test state** — each test should set up its own data
8. **Missing `async: true`** — use it by default, opt out only when necessary
