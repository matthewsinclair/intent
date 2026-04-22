# EXPECTED: passes
# BAD PRACTICE: duplicated setup across multiple tests + hard-coded identity.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — two or more tests where the first N lines are identical fixture
#   creation, with hard-coded emails that would collide under `async: true`.
#   ExUnit passes fine in isolation; the antipattern is that a fixture-shape
#   change means editing every test, and the hard-coded emails would deadlock
#   under concurrent DB inserts in a real Mix project.
Mix.install([])

ExUnit.start(autorun: true)

defmodule TestHighlanderSharedSetupBadTest do
  use ExUnit.Case, async: true

  defmodule Dashboard do
    def render(%{role: :admin}, "/admin/dashboard"), do: {200, "<h1>Dashboard</h1>"}
    def render(%{role: :admin}, "/admin/users"), do: {200, "<h1>Users</h1>"}
    def render(_, _), do: {403, "Forbidden"}
  end

  # No shared `setup`. No fixture helper. Identity is hard-coded.
  test "success: admin can view dashboard" do
    user = %{id: 1, email: "admin@test.com", role: :admin}
    assert {200, body} = Dashboard.render(user, "/admin/dashboard")
    assert body =~ "Dashboard"
  end

  test "success: admin can view users" do
    user = %{id: 1, email: "admin@test.com", role: :admin}
    assert {200, body} = Dashboard.render(user, "/admin/users")
    assert body =~ "Users"
  end
end
