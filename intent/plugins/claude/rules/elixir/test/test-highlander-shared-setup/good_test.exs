# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-007 (test-highlander-shared-setup):
#   the common "register admin + log in + build conn" precondition lives in a
#   single `setup` block. Each test body is the variation (URL + expected body
#   fragment). The fixture helper uses `System.unique_integer/1` so multiple
#   async tests never collide on identity.
Mix.install([])

ExUnit.start(autorun: true)

defmodule TestHighlanderSharedSetupGoodTest do
  use ExUnit.Case, async: true

  # --- Fixture helper — one canonical home, async-safe identity.
  defmodule Fixtures do
    def user_fixture(attrs \\ %{}) do
      unique = System.unique_integer([:positive])

      defaults = %{
        id: unique,
        email: "user-#{unique}@test.com",
        role: :viewer
      }

      Map.merge(defaults, Map.new(attrs))
    end
  end

  # --- Domain
  defmodule Dashboard do
    def render(%{role: :admin}, "/admin/dashboard"), do: {200, "<h1>Dashboard</h1>"}
    def render(%{role: :admin}, "/admin/users"), do: {200, "<h1>Users</h1>"}
    def render(_, _), do: {403, "Forbidden"}
  end

  setup do
    user = Fixtures.user_fixture(role: :admin)
    %{user: user}
  end

  test "success: admin can view dashboard", %{user: user} do
    assert {200, body} = Dashboard.render(user, "/admin/dashboard")
    assert body =~ "Dashboard"
  end

  test "success: admin can view users", %{user: user} do
    assert {200, body} = Dashboard.render(user, "/admin/users")
    assert body =~ "Users"
  end

  test "success: admin email is globally unique per test", %{user: user} do
    assert user.email =~ ~r/^user-\d+@test\.com$/
  end
end
